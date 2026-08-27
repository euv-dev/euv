use super::*;

/// Parses the `class!` macro input and generates `Css` function definitions.
///
/// # Arguments
///
/// - `TokenStream` - The raw token stream representing class definitions.
///
/// # Returns
///
/// - `TokenStream` - The generated token stream constructing `Css` functions.
pub(crate) fn parse_class(input: TokenStream) -> TokenStream {
    let tokens: proc_macro2::TokenStream = match parse::<ClassInput>(input) {
        Ok(class_input) => class_input.into_token_stream(),
        Err(error) => return error.to_compile_error().into(),
    };
    TokenStream::from(tokens)
}

/// Parses a CSS property key from the token stream.
///
/// Supports two forms:
/// - Static: A kebab-case identifier or string literal (e.g., `font_size`, `"background"`).
/// - Dynamic: A braced expression (e.g., `{key_var}`) that evaluates to a string at runtime.
///
/// # Arguments
///
/// - `ParseStream` - The syn parse stream to read from.
///
/// # Returns
///
/// - `syn::Result<ClassPropKey>` - The parsed property key.
pub(crate) fn parse_class_prop_key(input: ParseStream) -> syn::Result<ClassPropKey> {
    if input.peek(Brace) {
        let content: ParseBuffer<'_>;
        braced!(content in input);
        let expr: Expr = content.parse()?;
        Ok(ClassPropKey::Dynamic(expr.to_token_stream()))
    } else {
        let mut tokens: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
        while !input.peek(Token![:]) && !input.is_empty() {
            let token: proc_macro2::TokenTree = input.parse()?;
            tokens.extend(Some(token));
        }
        Ok(ClassPropKey::Static(tokens))
    }
}

/// Converts a `ClassPropKey` into a token stream that evaluates to a `String`.
///
/// # Arguments
///
/// - `&ClassPropKey` - The property key to convert.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - Token stream that evaluates to a `String`.
pub(crate) fn class_prop_key_to_tokens(key: &ClassPropKey) -> proc_macro2::TokenStream {
    match key {
        ClassPropKey::Static(static_key) => {
            let key_str: String = reconstruct_ident_from_tokens(static_key);
            quote! { #key_str.to_string() }
        }
        ClassPropKey::Dynamic(expr) => {
            quote! { (#expr).to_string() }
        }
    }
}

/// Recursively expands `var!(name)` macro calls within an expression tree
/// into the corresponding CSS `var()` string literal.
///
/// # Arguments
///
/// - `&Expr` - The expression to expand.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - The expanded token stream with `var!()` calls replaced
///   by `"var(--xxx-yyy)"` string literals.
pub(crate) fn expand_var_macros(expr: &Expr) -> proc_macro2::TokenStream {
    match expr {
        Expr::Macro(expr_macro) => {
            if expr_macro.mac.path.is_ident(VAR) {
                let body_tokens: &proc_macro2::TokenStream = &expr_macro.mac.tokens;
                let body_str: String = reconstruct_ident_from_tokens(body_tokens);
                let css_name: String = format!("{CSS_VAR_PREFIX}{body_str}{CSS_VAR_SUFFIX}");
                quote! { #css_name }
            } else if expr_macro.mac.path.is_ident(FORMAT_MACRO) {
                let mac_tokens: &proc_macro2::TokenStream = &expr_macro.mac.tokens;
                let expanded: proc_macro2::TokenStream = expand_var_macros_in_tokens(mac_tokens);
                let path: &Path = &expr_macro.mac.path;
                quote! { #path!(#expanded) }
            } else {
                expr.into_token_stream()
            }
        }
        _ => expr.into_token_stream(),
    }
}

/// Scans a token stream for `var!(name)` patterns and expands them
/// to the corresponding CSS `var()` string literals.
///
/// This is used to handle `var!()` calls nested inside `format!()` and
/// other macro invocations where syn does not provide structured parsing.
///
/// # Arguments
///
/// - `&proc_macro2::TokenStream` - The token stream to scan.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - The token stream with `var!(name)` patterns replaced
///   by `"var(--xxx-yyy)"` string literals.
pub(crate) fn expand_var_macros_in_tokens(
    tokens: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let mut result: Vec<proc_macro2::TokenTree> = Vec::new();
    let mut iter: Peekable<proc_macro2::token_stream::IntoIter> =
        tokens.clone().into_iter().peekable();
    while let Some(token) = iter.next() {
        match &token {
            proc_macro2::TokenTree::Ident(ident)
                if *ident == VAR
                    && iter.peek().is_some_and(
                        |token: &proc_macro2::TokenTree| matches!(token, proc_macro2::TokenTree::Punct(punct) if punct.as_char() == '!'),
                    ) =>
            {
                iter.next();
                if iter
                    .peek()
                    .is_some_and(|token: &proc_macro2::TokenTree| matches!(token, proc_macro2::TokenTree::Group(_)))
                {
                    if let Some(proc_macro2::TokenTree::Group(group)) = iter.next() {
                        let inner: proc_macro2::TokenStream = group.stream();
                        let var_name: String = reconstruct_ident_from_tokens(&inner);
                        let css_name: String = format!("{CSS_VAR_PREFIX}{var_name}{CSS_VAR_SUFFIX}");
                        let expanded: proc_macro2::TokenStream = quote! { #css_name };
                        result.extend(expanded);
                    }
                } else {
                    result.push(proc_macro2::TokenTree::Ident(ident.clone()));
                    result.push(proc_macro2::TokenTree::Punct(proc_macro2::Punct::new(
                        '!',
                        proc_macro2::Spacing::Alone,
                    )));
                }
            }
            proc_macro2::TokenTree::Group(group) => {
                let expanded_inner: proc_macro2::TokenStream =
                    expand_var_macros_in_tokens(&group.stream());
                let new_group: proc_macro2::Group =
                    proc_macro2::Group::new(group.delimiter(), expanded_inner);
                result.push(proc_macro2::TokenTree::Group(new_group));
            }
            _ => {
                result.push(token);
            }
        }
    }
    result.into_iter().collect()
}

/// Collects parameter names that are wrapped in `{}` anywhere in a class definition.
///
/// A wrapped parameter is treated as a dynamic class parameter: its value contributes
/// to the generated class name, allowing every distinct value to inject its own CSS rule.
/// Bare parameters keep the existing type-based class-name behavior.
///
/// # Arguments
///
/// - `&ClassDef` - The class definition to scan.
///
/// # Returns
///
/// - `Vec<String>` - Unique names of parameters wrapped in `{}`.
pub(crate) fn collect_dynamic_param_names(class_def: &ClassDef) -> Vec<String> {
    let mut dynamic_param_names: Vec<String> = Vec::new();
    collect_dynamic_param_names_from_properties(
        class_def.get_properties(),
        &mut dynamic_param_names,
    );
    collect_dynamic_param_names_from_selector_blocks(
        class_def.get_selector_blocks(),
        &mut dynamic_param_names,
    );
    collect_dynamic_param_names_from_at_rule_blocks(
        class_def.get_at_rule_blocks(),
        &mut dynamic_param_names,
    );
    for parent in class_def.get_extends() {
        for arg in parent.get_args() {
            collect_braced_idents(arg, &mut dynamic_param_names);
        }
    }
    dynamic_param_names
}

/// Collects dynamic parameter identifiers from the supplied `properties` slice.
/// Helper body of the `collect_dynamic_param_names_from_properties` free function.
///
/// # Arguments
///
/// - `&[(ClassPropKey, ClassPropValue)]` - Shared reference to a `[(ClassPropKey, ClassPropValue)]`.
/// - `&mut Vec<String>` - Mutable reference to a `Vec<String>` (mutated in place).
fn collect_dynamic_param_names_from_properties(
    properties: &[(ClassPropKey, ClassPropValue)],
    dynamic_param_names: &mut Vec<String>,
) {
    for (key, value) in properties {
        if let ClassPropKey::Dynamic(tokens) = key {
            collect_braced_idents(tokens, dynamic_param_names);
        }
        let ClassPropValue::Expr(tokens) = value;
        collect_braced_idents(tokens, dynamic_param_names);
    }
}

/// Collects dynamic parameter identifiers from the supplied `selector_blocks` slice.
/// Helper body of the `collect_dynamic_param_names_from_selector_blocks` free function.
///
/// # Arguments
///
/// - `&[SelectorBlock]` - Shared reference to a `[SelectorBlock]`.
/// - `&mut Vec<String>` - Mutable reference to a `Vec<String>` (mutated in place).
fn collect_dynamic_param_names_from_selector_blocks(
    selector_blocks: &[SelectorBlock],
    dynamic_param_names: &mut Vec<String>,
) {
    for selector_block in selector_blocks {
        collect_dynamic_param_names_from_properties(
            selector_block.get_properties(),
            dynamic_param_names,
        );
        collect_dynamic_param_names_from_selector_blocks(
            selector_block.get_selector_blocks(),
            dynamic_param_names,
        );
    }
}

/// Collects dynamic parameter identifiers from the supplied `at_rule_blocks` slice.
/// Helper body of the `collect_dynamic_param_names_from_at_rule_blocks` free function.
///
/// # Arguments
///
/// - `&[AtRuleBlock]` - Shared reference to a `[AtRuleBlock]`.
/// - `&mut Vec<String>` - Mutable reference to a `Vec<String>` (mutated in place).
fn collect_dynamic_param_names_from_at_rule_blocks(
    at_rule_blocks: &[AtRuleBlock],
    dynamic_param_names: &mut Vec<String>,
) {
    for at_rule_block in at_rule_blocks {
        collect_dynamic_param_names_from_properties(
            at_rule_block.get_properties(),
            dynamic_param_names,
        );
        collect_dynamic_param_names_from_selector_blocks(
            at_rule_block.get_selector_blocks(),
            dynamic_param_names,
        );
        collect_dynamic_param_names_from_at_rule_blocks(
            at_rule_block.get_at_rule_blocks(),
            dynamic_param_names,
        );
    }
}

/// Recursively descends into every brace-delimited group of the token stream.
/// Helper body of the `collect_braced_idents` free function.
///
/// # Arguments
///
/// - `&proc_macro2::TokenStream` - Shared reference to a `proc_macro2::TokenStream`.
/// - `&mut Vec<String>` - Mutable reference to a `Vec<String>` (mutated in place).
fn collect_braced_idents(tokens: &proc_macro2::TokenStream, dynamic_param_names: &mut Vec<String>) {
    for token in tokens.clone() {
        if let TokenTree::Group(group) = token {
            if group.delimiter() == proc_macro2::Delimiter::Brace {
                collect_all_idents(&group.stream(), dynamic_param_names);
            }
            collect_braced_idents(&group.stream(), dynamic_param_names);
        }
    }
}

/// Collects every identifier-shaped token (recursively into groups).
/// Helper body of the `collect_all_idents` free function.
///
/// # Arguments
///
/// - `&proc_macro2::TokenStream` - Shared reference to a `proc_macro2::TokenStream`.
/// - `&mut Vec<String>` - Mutable reference to a `Vec<String>` (mutated in place).
fn collect_all_idents(tokens: &proc_macro2::TokenStream, dynamic_param_names: &mut Vec<String>) {
    for token in tokens.clone() {
        match token {
            TokenTree::Ident(ident) => {
                push_unique_param_name(dynamic_param_names, ident.to_string())
            }
            TokenTree::Group(group) => collect_all_idents(&group.stream(), dynamic_param_names),
            _ => {}
        }
    }
}

/// Appends `param_name` only when it is not already present.
/// Helper body of the `push_unique_param_name` free function.
///
/// # Arguments
///
/// - `&mut Vec<String>` - Mutable reference to a `Vec<String>` (mutated in place).
/// - `String` - A `String` parameter.
fn push_unique_param_name(dynamic_param_names: &mut Vec<String>, param_name: String) {
    if !dynamic_param_names.contains(&param_name) {
        dynamic_param_names.push(param_name);
    }
}

/// Checks whether a `proc_macro2::TokenStream` consists entirely of string literals,
/// meaning its value can be computed at compile time.
///
/// # Arguments
///
/// - `&proc_macro2::TokenStream` - The token stream to check.
///
/// # Returns
///
/// - `bool` - `true` if all tokens are string literals.
pub(crate) fn is_static_string_expr(tokens: &proc_macro2::TokenStream) -> bool {
    for token in tokens.clone() {
        match token {
            proc_macro2::TokenTree::Literal(_) => continue,
            _ => return false,
        }
    }
    true
}

/// Extracts the string value from a token stream that consists entirely of
/// string literals, concatenating them.
///
/// # Arguments
///
/// - `&proc_macro2::TokenStream` - The token stream consisting of string literals only.
///
/// # Returns
///
/// - `String` - The concatenated string value.
pub(crate) fn expr_to_string(tokens: &proc_macro2::TokenStream) -> String {
    let mut result: String = String::new();
    for token in tokens.clone() {
        if let proc_macro2::TokenTree::Literal(literal) = token {
            let literal_token_stream: proc_macro2::TokenStream =
                proc_macro2::TokenTree::Literal(literal).into();
            if let Ok(literal_string) = parse2::<LitStr>(literal_token_stream) {
                result.push_str(&literal_string.value());
            }
        }
    }
    result
}

/// Parses a CSS selector from the token stream starting with `::` or `:`.
///
/// This handles the full CSS pseudo-class and pseudo-element syntax:
/// - `::before`
/// - `::-webkit-scrollbar-thumb:hover`
/// - `:hover`
/// - `:focus-visible`
/// - `:nth-child(2n+1)`
/// - `:not(.class)`
/// - `:where(div, span)`
/// - `:has(> .child)`
/// - `:is(.a, .b)`
///
/// The selector is reconstructed verbatim from the parsed tokens,
/// preserving the original CSS syntax without modification.
///
/// # Arguments
///
/// - `ParseStream` - The syn parse stream to read from.
/// - `usize` - The number of leading colons already consumed (1 for `:`, 2 for `::`).
///
/// # Returns
///
/// - `syn::Result<String>` - The reconstructed CSS selector string.
pub(crate) fn parse_selector(input: ParseStream, initial_colons: usize) -> syn::Result<String> {
    let mut selector: String = String::new();
    for _ in 0..initial_colons {
        selector.push(CHAR_COLON);
    }
    selector.push_str(&parse_ident_name(input)?);
    while input.peek(Token![:]) || input.peek(Token![-]) || input.peek(Paren) {
        if input.peek(Paren) {
            let paren_content: ParseBuffer<'_>;
            parenthesized!(paren_content in input);
            let paren_tokens: proc_macro2::TokenStream = paren_content.parse()?;
            let paren_str: String = reconstruct_media_query(&paren_tokens);
            selector.push(CHAR_LEFT_PAREN);
            selector.push_str(&paren_str);
            selector.push(CHAR_RIGHT_PAREN);
            continue;
        }
        if input.peek(Token![-]) {
            input.parse::<Token![-]>()?;
            if input.peek(Token![-]) {
                input.parse::<Token![-]>()?;
                selector.push_str(STR_HYPHEN);
                selector.push_str(STR_HYPHEN);
                selector.push_str(&parse_ident_name(input)?);
                continue;
            }
            selector.push(CHAR_HYPHEN);
            selector.push_str(&parse_ident_name(input)?);
            continue;
        }
        if input.peek(Token![:]) && !input.peek2(Brace) {
            input.parse::<Token![:]>()?;
            if input.peek(Token![:]) {
                input.parse::<Token![:]>()?;
                selector.push(CHAR_COLON);
                selector.push(CHAR_COLON);
                selector.push_str(&parse_ident_name(input)?);
            } else {
                selector.push(CHAR_COLON);
                selector.push_str(&parse_ident_name(input)?);
            }
        } else {
            break;
        }
    }
    Ok(selector)
}

/// Checks whether the current position starts an element selector block
/// (e.g. `h1 { ... }`, `* { ... }`, `input, button { ... }`).
///
/// An element selector block starts with `*` or `Ident` and is followed by `{`
/// before a property-separating `:` (not `::`) or `;`.
///
/// # Arguments
///
/// - `&ParseStream` - The parse stream to check.
///
/// # Returns
///
/// - `bool` - `true` if the current position starts an element selector block.
pub(crate) fn is_element_selector_block(input: ParseStream) -> bool {
    if input.peek(Token![*]) {
        return true;
    }
    if !input.peek(Ident) {
        return false;
    }
    let forked: ParseBuffer<'_> = input.fork();
    while !forked.is_empty() {
        if forked.peek(Brace) {
            return true;
        }
        if forked.peek(Semi) {
            return false;
        }
        if forked.peek(Token![:]) && !forked.peek2(Token![:]) {
            let _: Result<Token![:], syn::Error> = forked.parse::<Token![:]>();
            if forked.peek(Brace) {
                return false;
            }
            while !forked.is_empty() {
                if forked.peek(Semi) || forked.peek(Brace) {
                    break;
                }
                let _: Result<TokenTree, syn::Error> = forked.parse::<TokenTree>();
            }
            if forked.peek(Brace) {
                return true;
            }
            return false;
        }
        let _: Result<TokenTree, syn::Error> = forked.parse::<TokenTree>();
    }
    false
}

/// Parses an element selector string from the token stream.
///
/// Collects all tokens until `{` and reconstructs a valid CSS selector string.
///
/// # Arguments
///
/// - `ParseStream` - The syn parse stream to read from.
///
/// # Returns
///
/// - `syn::Result<String>` - The reconstructed CSS selector string.
pub(crate) fn parse_element_selector(input: ParseStream) -> syn::Result<String> {
    let mut tokens: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
    while !input.peek(Brace) && !input.is_empty() {
        let token: TokenTree = input.parse()?;
        tokens.extend(Some(token));
    }
    Ok(reconstruct_selector_from_tokens(&tokens))
}

/// Reconstructs a CSS selector string from a raw `proc_macro2::TokenStream`.
///
/// Similar to `reconstruct_media_query` but optimized for CSS selectors:
/// - Commas are followed by a space
/// - Identifiers, punctuation, groups, and literals are preserved as-is
///
/// # Arguments
///
/// - `&proc_macro2::TokenStream` - The raw token stream to reconstruct.
///
/// # Returns
///
/// - `String` - The reconstructed CSS selector string.
pub(crate) fn reconstruct_selector_from_tokens(tokens: &proc_macro2::TokenStream) -> String {
    let mut result: String = String::new();
    for token in tokens.clone() {
        match &token {
            proc_macro2::TokenTree::Ident(ident) => {
                let raw_name: String = ident.to_string();
                let clean_name: String = raw_name
                    .strip_prefix(RAW_IDENT_PREFIX)
                    .unwrap_or(&raw_name)
                    .to_string();
                result.push_str(&clean_name);
            }
            proc_macro2::TokenTree::Punct(punct) => {
                let ch: char = punct.as_char();
                if ch == CHAR_COMMA {
                    result.push(CHAR_COMMA);
                    result.push(CHAR_SPACE);
                } else {
                    result.push(ch);
                }
            }
            proc_macro2::TokenTree::Group(group) => match group.delimiter() {
                proc_macro2::Delimiter::Parenthesis => {
                    result.push(CHAR_LEFT_PAREN);
                    let inner: String = reconstruct_selector_from_tokens(&group.stream());
                    result.push_str(&inner);
                    result.push(CHAR_RIGHT_PAREN);
                }
                proc_macro2::Delimiter::Bracket => {
                    result.push(CHAR_LEFT_BRACKET);
                    let inner: String = reconstruct_selector_from_tokens(&group.stream());
                    result.push_str(&inner);
                    result.push(CHAR_RIGHT_BRACKET);
                }
                _ => {
                    let inner: String = reconstruct_selector_from_tokens(&group.stream());
                    result.push_str(&inner);
                }
            },
            proc_macro2::TokenTree::Literal(literal) => {
                let literal_token_stream: proc_macro2::TokenStream =
                    proc_macro2::TokenTree::Literal(literal.clone()).into();
                if let Ok(literal_string) = parse2::<LitStr>(literal_token_stream) {
                    result.push_str(&literal_string.value());
                } else {
                    let literal_text: String = literal.to_string();
                    if literal_text.starts_with(CHAR_DOUBLE_QUOTE) {
                        if let Some(stripped) = literal_text
                            .strip_prefix(CHAR_DOUBLE_QUOTE)
                            .and_then(|text: &str| text.strip_suffix(CHAR_DOUBLE_QUOTE))
                        {
                            result.push_str(stripped);
                        } else {
                            result.push_str(&literal_text);
                        }
                    } else {
                        result.push_str(&literal_text);
                    }
                }
            }
        }
    }
    result
}

/// Parses the content of a selector or at-rule block, handling nested
/// selector blocks, at-rule blocks, and CSS properties.
///
/// # Arguments
///
/// - `ParseStream` - The syn parse stream to read from.
///
/// # Returns
///
/// - `syn::Result<BlockContent>` - The parsed block content.
pub(crate) fn parse_block_content(input: ParseStream) -> syn::Result<BlockContent> {
    let mut properties: Vec<(ClassPropKey, ClassPropValue)> = Vec::new();
    let mut selector_blocks: Vec<SelectorBlock> = Vec::new();
    let mut at_rule_blocks: Vec<AtRuleBlock> = Vec::new();
    while !input.is_empty() {
        if input.peek(Token![::]) {
            input.parse::<Token![::]>()?;
            let selector: String = parse_selector(input, 2)?;
            let block_content: ParseBuffer<'_>;
            braced!(block_content in input);
            let inner: BlockContent = parse_block_content(&block_content)?;
            selector_blocks.push(SelectorBlock::new(
                selector,
                inner.properties,
                inner.selector_blocks,
            ));
            continue;
        }
        if input.peek(Token![:]) && !input.peek2(Brace) {
            input.parse::<Token![:]>()?;
            let selector: String = parse_selector(input, 1)?;
            let block_content: ParseBuffer<'_>;
            braced!(block_content in input);
            let inner: BlockContent = parse_block_content(&block_content)?;
            selector_blocks.push(SelectorBlock::new(
                selector,
                inner.properties,
                inner.selector_blocks,
            ));
            continue;
        }
        if peek_at_rule(input) {
            let at_rule: AtRuleBlock = parse_at_rule(input)?;
            at_rule_blocks.push(at_rule);
            continue;
        }
        if is_element_selector_block(input) {
            let selector: String = parse_element_selector(input)?;
            let block_content: ParseBuffer<'_>;
            braced!(block_content in input);
            let inner: BlockContent = parse_block_content(&block_content)?;
            selector_blocks.push(SelectorBlock::new(
                selector,
                inner.properties,
                inner.selector_blocks,
            ));
            continue;
        }
        let css_key: ClassPropKey = parse_class_prop_key(input)?;
        input.parse::<Token![:]>()?;
        let expr: Expr = input.parse()?;
        let expanded: proc_macro2::TokenStream = expand_var_macros(&expr);
        let prop_value: ClassPropValue = ClassPropValue::Expr(expanded);
        properties.push((css_key, prop_value));
        if input.peek(Semi) {
            input.parse::<Semi>()?;
        }
    }
    Ok(BlockContent {
        properties,
        selector_blocks,
        at_rule_blocks,
    })
}

/// Checks whether the current position in the parse stream starts an at-rule.
///
/// An at-rule starts with `@` followed by an identifier and then
/// either a block `{...}` or a semicolon `;`.
///
/// # Arguments
///
/// - `&ParseStream` - The parse stream to check.
///
/// # Returns
///
/// - `bool` - `true` if the current position starts an at-rule.
pub(crate) fn peek_at_rule(input: ParseStream) -> bool {
    if !input.peek(Token![@]) {
        return false;
    }
    let forked: ParseBuffer<'_> = input.fork();
    let _: Result<Token![@], syn::Error> = forked.parse::<Token![@]>();
    if let Ok(ident) = forked.parse::<Ident>() {
        let ident_str: String = ident.to_string();
        return lookup_at_rule_kind(&ident_str).is_some();
    }
    if let Ok(token_tree) = forked.parse::<proc_macro2::TokenTree>()
        && let proc_macro2::TokenTree::Ident(ident) = token_tree
    {
        let raw_name: String = ident.to_string();
        let clean_name: &str = raw_name.strip_prefix(RAW_IDENT_PREFIX).unwrap_or(&raw_name);
        return lookup_at_rule_kind(clean_name).is_some();
    }
    false
}

/// Looks up an `AtRuleKind` by keyword name.
///
/// # Arguments
///
/// - `&str` - The at-rule keyword (e.g., "media", "keyframes", "supports").
///
/// # Returns
///
/// - `Option<AtRuleKind>` - The matching at-rule kind, or `None`.
pub(crate) fn lookup_at_rule_kind(keyword: &str) -> Option<AtRuleKind> {
    match keyword {
        AT_MEDIA => Some(AtRuleKind::Media),
        AT_KEYFRAMES => Some(AtRuleKind::Keyframes),
        AT_SUPPORTS => Some(AtRuleKind::Supports),
        AT_LAYER => Some(AtRuleKind::Layer),
        AT_CONTAINER => Some(AtRuleKind::Container),
        AT_PROPERTY => Some(AtRuleKind::Property),
        AT_SCOPE => Some(AtRuleKind::Scope),
        AT_FONT_FACE => Some(AtRuleKind::FontFace),
        AT_CHARSET => Some(AtRuleKind::Charset),
        AT_IMPORT => Some(AtRuleKind::Import),
        AT_NAMESPACE => Some(AtRuleKind::Namespace),
        AT_PAGE => Some(AtRuleKind::Page),
        AT_COLOR_PROFILE => Some(AtRuleKind::ColorProfile),
        AT_COUNTER_STYLE => Some(AtRuleKind::CounterStyle),
        AT_FONT_FEATURE_VALUES => Some(AtRuleKind::FontFeatureValues),
        AT_FONT_PALETTE_VALUES => Some(AtRuleKind::FontPaletteValues),
        AT_DOCUMENT => Some(AtRuleKind::Document),
        AT_STARTING_STYLE => Some(AtRuleKind::StartingStyle),
        AT_VIEW_TRANSITION => Some(AtRuleKind::ViewTransition),
        AT_POSITION_TRY => Some(AtRuleKind::PositionTry),
        AT_CUSTOM_MEDIA => Some(AtRuleKind::CustomMedia),
        AT_FUNCTION => Some(AtRuleKind::Function),
        _ => None,
    }
}

/// Parses an at-rule from the token stream.
///
/// Handles both block at-rules (`@media (...) { ... }`) and statement
/// at-rules (`@charset "UTF-8";`, `@import url("style.css");`).
///
/// # Arguments
///
/// - `ParseStream` - The syn parse stream to read from.
///
/// # Returns
///
/// - `syn::Result<AtRuleBlock>` - The parsed at-rule block.
pub(crate) fn parse_at_rule(input: ParseStream) -> syn::Result<AtRuleBlock> {
    input.parse::<Token![@]>()?;
    let keyword: String = parse_ident_name(input)?;
    let kind: AtRuleKind = lookup_at_rule_kind(&keyword)
        .ok_or_else(|| input.error(format!("unknown at-rule: @{keyword}")))?;
    let is_statement_rule: bool = matches!(
        kind,
        AtRuleKind::Charset | AtRuleKind::Import | AtRuleKind::Namespace
    );
    if is_statement_rule {
        let prelude_tokens: proc_macro2::TokenStream = input.parse()?;
        let prelude: String = reconstruct_media_query(&prelude_tokens);
        if input.peek(Semi) {
            input.parse::<Semi>()?;
        }
        return Ok(AtRuleBlock::new(
            kind,
            prelude,
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ));
    }
    let prelude: String = if input.peek(Paren) {
        let paren_content: ParseBuffer<'_>;
        parenthesized!(paren_content in input);
        let query_tokens: proc_macro2::TokenStream = paren_content.parse()?;
        let query_str: String = reconstruct_media_query(&query_tokens);
        format!("{CHAR_LEFT_PAREN}{query_str}{CHAR_RIGHT_PAREN}")
    } else if !input.peek(Brace) {
        let prelude_tokens: proc_macro2::TokenStream = input.parse()?;
        reconstruct_media_query(&prelude_tokens)
    } else {
        STR_EMPTY.to_string()
    };
    let block_content: ParseBuffer<'_>;
    braced!(block_content in input);
    let inner: BlockContent = parse_block_content(&block_content)?;
    Ok(AtRuleBlock::new(
        kind,
        prelude,
        inner.properties,
        inner.selector_blocks,
        inner.at_rule_blocks,
    ))
}

/// Generates style property token streams from a list of properties.
///
/// # Arguments
///
/// - `&[(ClassPropKey, ClassPropValue)]` - The properties to convert.
///
/// # Returns
///
/// - `Vec<proc_macro2::TokenStream>` - The generated token streams for each property.
pub(crate) fn properties_to_tokens(
    properties: &[(ClassPropKey, ClassPropValue)],
) -> Vec<proc_macro2::TokenStream> {
    properties
        .iter()
        .map(|(key, value): &(ClassPropKey, ClassPropValue)| match value {
            ClassPropValue::Expr(expr) => match key {
                ClassPropKey::Static(static_key) => {
                    let key_str: String = reconstruct_ident_from_tokens(static_key);
                    if is_static_string_expr(expr) {
                        let value_str: String = expr_to_string(expr);
                        let prop_str: String =
                            format!("{key_str}{CSS_PROP_SEPARATOR}{value_str}{CSS_DECL_TERMINATOR}");
                        quote! { #prop_str.to_string() }
                    } else {
                        let key_sep: String = format!("{key_str}{CSS_PROP_SEPARATOR}");
                        quote! { #key_sep.to_string() + &(#expr).to_string() + #CSS_DECL_TERMINATOR }
                    }
                }
                ClassPropKey::Dynamic(_) => {
                    let key_token: proc_macro2::TokenStream = class_prop_key_to_tokens(key);
                    quote! { #key_token + #CSS_PROP_SEPARATOR + &(#expr).to_string() + #CSS_DECL_TERMINATOR }
                }
            },
        })
        .collect()
}

/// Generates a `Vec<::euv_core::PseudoRule>` expression from a list of selector blocks.
///
/// # Arguments
///
/// - `&[SelectorBlock]` - The selector blocks to convert.
///
/// # Returns
///
/// - `Option<proc_macro2::TokenStream>` - The generated token stream, or `None` if empty.
pub(crate) fn selector_blocks_to_tokens(
    selector_blocks: &[SelectorBlock],
) -> Option<proc_macro2::TokenStream> {
    if selector_blocks.is_empty() {
        return None;
    }
    let parts: Vec<proc_macro2::TokenStream> = flatten_selector_blocks(selector_blocks, "");
    Some(quote! { vec![#(#parts), *] })
}

/// Recursively flattens nested selector blocks into a single-level list of
/// `PseudoRule` token streams, combining parent and child selectors.
///
/// For example, `::placeholder { :hover { ... } }` produces a single
/// `PseudoRule` with selector `::placeholder:hover`.
///
/// # Arguments
///
/// - `&[SelectorBlock]` - The selector blocks to flatten.
/// - `&str` - The accumulated parent selector prefix.
///
/// # Returns
///
/// - `Vec<proc_macro2::TokenStream>` - The flattened `PseudoRule` token streams.
fn flatten_selector_blocks(
    selector_blocks: &[SelectorBlock],
    parent_selector: &str,
) -> Vec<proc_macro2::TokenStream> {
    let mut result: Vec<proc_macro2::TokenStream> = Vec::new();
    for block in selector_blocks {
        let selector: &str = block.get_selector();
        let combined_selector: String = format!("{parent_selector}{selector}");
        let style_parts: Vec<proc_macro2::TokenStream> =
            properties_to_tokens(block.get_properties());
        if !style_parts.is_empty() {
            let style_expr: proc_macro2::TokenStream = quote! { [#(#style_parts), *].concat() };
            result.push(quote! {
                ::euv_core::PseudoRule::new(#combined_selector.to_string(), #style_expr)
            });
        }
        let nested: Vec<proc_macro2::TokenStream> =
            flatten_selector_blocks(block.get_selector_blocks(), &combined_selector);
        result.extend(nested);
    }
    result
}

/// Generates a `Vec<::euv_core::MediaRule>` expression from a list of at-rule blocks.
///
/// Only `AtRuleKind::Media` blocks are converted to `MediaRule`.
/// Other at-rule kinds are converted to their respective CSS output.
///
/// # Arguments
///
/// - `&[AtRuleBlock]` - The at-rule blocks to convert.
///
/// # Returns
///
/// - `Option<proc_macro2::TokenStream>` - The generated token stream, or `None` if empty.
pub(crate) fn at_rule_blocks_to_media_tokens(
    at_rule_blocks: &[AtRuleBlock],
) -> Option<proc_macro2::TokenStream> {
    let media_blocks: Vec<&AtRuleBlock> = at_rule_blocks
        .iter()
        .filter(|block: &&AtRuleBlock| matches!(block.get_kind(), AtRuleKind::Media))
        .collect();
    if media_blocks.is_empty() {
        return None;
    }
    let parts: Vec<proc_macro2::TokenStream> = media_blocks
        .iter()
        .map(|block: &&AtRuleBlock| {
            let query: &str = block.get_prelude();
            let style_parts: Vec<proc_macro2::TokenStream> =
                properties_to_tokens(block.get_properties());
            let pseudo_expr: proc_macro2::TokenStream =
                selector_blocks_to_tokens(block.get_selector_blocks())
                    .unwrap_or_else(|| quote! { Vec::new() });
            let style_expr: proc_macro2::TokenStream = if style_parts.is_empty() {
                quote! { #STR_EMPTY.to_string() }
            } else {
                quote! { [#(#style_parts), *].concat() }
            };
            quote! {
                ::euv_core::MediaRule::new(
                    #query.to_string(),
                    #style_expr,
                    #pseudo_expr
                )
            }
        })
        .collect();
    Some(quote! { vec![#(#parts), *] })
}

/// Generates static selector block string for compile-time evaluation.
///
/// # Arguments
///
/// - `&[SelectorBlock]` - The selector blocks to serialize.
///
/// # Returns
///
/// - `String` - The serialized selector rules string.
pub(crate) fn selector_blocks_to_static_string(selector_blocks: &[SelectorBlock]) -> String {
    let mut result: String = String::new();
    for block in selector_blocks {
        result.push_str(&selector_block_to_static_string(block));
    }
    result
}

/// Generates static at-rule block string for compile-time evaluation.
///
/// # Arguments
///
/// - `&[AtRuleBlock]` - The at-rule blocks to serialize.
///
/// # Returns
///
/// - `String` - The serialized at-rule rules string.
pub(crate) fn at_rule_blocks_to_static_string(at_rule_blocks: &[AtRuleBlock]) -> String {
    let mut result: String = String::new();
    for block in at_rule_blocks {
        let prefix: &str = at_rule_kind_to_css_prefix(block.get_kind());
        result.push_str(prefix);
        if !block.get_prelude().is_empty() {
            result.push_str(block.get_prelude());
        }
        result.push_str(CSS_RULE_OPEN);
        for (key, value) in block.get_properties() {
            let ClassPropValue::Expr(expr) = value;
            let ClassPropKey::Static(key_tokens) = key else {
                continue;
            };
            let key_str: String = reconstruct_ident_from_tokens(key_tokens);
            result.push_str(&key_str);
            result.push_str(CSS_PROP_SEPARATOR);
            result.push_str(&expr_to_string(expr));
            result.push_str(CSS_DECL_TERMINATOR);
        }
        for selector_block in block.get_selector_blocks() {
            result.push_str(&selector_block_to_static_string(selector_block));
        }
        for nested_at_rule in block.get_at_rule_blocks() {
            result.push_str(&at_rule_block_to_static_string(nested_at_rule));
        }
        result.push(CHAR_CSS_RULE_CLOSE);
    }
    result
}

/// Generates a static string for a single selector block.
///
/// # Arguments
///
/// - `&SelectorBlock` - The selector block to serialize.
///
/// # Returns
///
/// - `String` - The serialized selector rule string.
pub(crate) fn selector_block_to_static_string(block: &SelectorBlock) -> String {
    let mut result: String = String::new();
    result.push_str(block.get_selector());
    result.push_str(CSS_RULE_OPEN);
    for (key, value) in block.get_properties() {
        let ClassPropValue::Expr(expr) = value;
        let ClassPropKey::Static(key_tokens) = key else {
            continue;
        };
        let key_str: String = reconstruct_ident_from_tokens(key_tokens);
        result.push_str(&key_str);
        result.push_str(CSS_PROP_SEPARATOR);
        result.push_str(&expr_to_string(expr));
        result.push_str(CSS_DECL_TERMINATOR);
    }
    for nested in block.get_selector_blocks() {
        result.push_str(&selector_block_to_static_string(nested));
    }
    result.push(CHAR_CSS_RULE_CLOSE);
    result
}

/// Generates a static string for a single at-rule block.
///
/// # Arguments
///
/// - `&AtRuleBlock` - The at-rule block to serialize.
///
/// # Returns
///
/// - `String` - The serialized at-rule string.
pub(crate) fn at_rule_block_to_static_string(block: &AtRuleBlock) -> String {
    let mut result: String = String::new();
    let prefix: &str = at_rule_kind_to_css_prefix(block.get_kind());
    result.push_str(prefix);
    if !block.get_prelude().is_empty() {
        result.push_str(block.get_prelude());
    }
    result.push_str(CSS_RULE_OPEN);
    for (key, value) in block.get_properties() {
        let ClassPropValue::Expr(expr) = value;
        let ClassPropKey::Static(key_tokens) = key else {
            continue;
        };
        let key_str: String = reconstruct_ident_from_tokens(key_tokens);
        result.push_str(&key_str);
        result.push_str(CSS_PROP_SEPARATOR);
        result.push_str(&expr_to_string(expr));
        result.push_str(CSS_DECL_TERMINATOR);
    }
    for selector_block in block.get_selector_blocks() {
        result.push_str(&selector_block_to_static_string(selector_block));
    }
    for nested_at_rule in block.get_at_rule_blocks() {
        result.push_str(&at_rule_block_to_static_string(nested_at_rule));
    }
    result.push(CHAR_CSS_RULE_CLOSE);
    result
}

/// Returns the CSS at-rule prefix string for a given `AtRuleKind`.
///
/// # Arguments
///
/// - `&AtRuleKind` - The at-rule kind.
///
/// # Returns
///
/// - `&str` - The CSS at-rule prefix string (e.g., "@media ", "@keyframes ").
pub(crate) fn at_rule_kind_to_css_prefix(kind: &AtRuleKind) -> &'static str {
    match kind {
        AtRuleKind::Media => CSS_MEDIA_PREFIX,
        AtRuleKind::Keyframes => CSS_KEYFRAMES_PREFIX,
        AtRuleKind::Supports => CSS_SUPPORTS_PREFIX,
        AtRuleKind::Layer => CSS_LAYER_PREFIX,
        AtRuleKind::Container => CSS_CONTAINER_PREFIX,
        AtRuleKind::Property => CSS_PROPERTY_PREFIX,
        AtRuleKind::Scope => CSS_SCOPE_PREFIX,
        AtRuleKind::FontFace => CSS_FONT_FACE_PREFIX,
        AtRuleKind::Charset => CSS_CHARSET_PREFIX,
        AtRuleKind::Import => CSS_IMPORT_PREFIX,
        AtRuleKind::Namespace => CSS_NAMESPACE_PREFIX,
        AtRuleKind::Page => CSS_PAGE_PREFIX,
        AtRuleKind::ColorProfile => CSS_COLOR_PROFILE_PREFIX,
        AtRuleKind::CounterStyle => CSS_COUNTER_STYLE_PREFIX,
        AtRuleKind::FontFeatureValues => CSS_FONT_FEATURE_VALUES_PREFIX,
        AtRuleKind::FontPaletteValues => CSS_FONT_PALETTE_VALUES_PREFIX,
        AtRuleKind::Document => CSS_DOCUMENT_PREFIX,
        AtRuleKind::StartingStyle => CSS_STARTING_STYLE_PREFIX,
        AtRuleKind::ViewTransition => "@view-transition ",
        AtRuleKind::PositionTry => "@position-try ",
        AtRuleKind::CustomMedia => "@custom-media ",
        AtRuleKind::Function => "@function ",
    }
}

/// Generates the `OnceLock`-based static function body for a no-param class.
///
/// Shared by both the all-static and dynamic paths in `ClassDef::to_tokens`.
///
/// # Arguments
///
/// - `&mut proc_macro2::TokenStream` - The target token stream to append to.
/// - `OnceLockParams` - The parameters for the OnceLock function generation.
pub(crate) fn emit_once_lock_fn(
    tokens: &mut proc_macro2::TokenStream,
    once_lock_params: OnceLockParams<'_>,
) {
    let OnceLockParams {
        visibility,
        fn_name_token,
        const_name_token,
        class_name_str,
        style_expr,
        selector_expr,
        at_rule_expr,
    } = once_lock_params;
    tokens.extend(quote! {
        #visibility fn #fn_name_token() -> &'static ::euv_core::Css {
            static #const_name_token: ::std::sync::OnceLock<::euv_core::Css> = ::std::sync::OnceLock::new();
            #const_name_token.get_or_init(|| {
                let css: ::euv_core::Css = ::euv_core::Css::new(#class_name_str.to_string(), #style_expr, #selector_expr, #at_rule_expr);
                css.inject_style();
                css
            })
        }
    });
}

/// Reconstructs a CSS media query string from a raw `proc_macro2::TokenStream`.
///
/// Unlike `reconstruct_ident_from_tokens` which only handles identifiers and
/// hyphens, this function preserves all punctuation characters (colons, commas,
/// etc.) and parenthesis groups, producing a valid CSS media query string
/// such as `(max-width: 767px)` from the token stream `(max-width: 767px)`.
///
/// # Arguments
///
/// - `&proc_macro2::TokenStream` - The raw token stream to reconstruct.
///
/// # Returns
///
/// - `String` - The reconstructed media query string.
pub(crate) fn reconstruct_media_query(tokens: &proc_macro2::TokenStream) -> String {
    let mut result: String = String::new();
    let mut prev_needs_space: bool = false;
    for token in tokens.clone() {
        match &token {
            proc_macro2::TokenTree::Ident(ident) => {
                if prev_needs_space {
                    result.push(CHAR_SPACE);
                }
                let raw_name: String = ident.to_string();
                let clean_name: String = raw_name
                    .strip_prefix(RAW_IDENT_PREFIX)
                    .unwrap_or(&raw_name)
                    .to_string();
                result.push_str(&clean_name);
                prev_needs_space = true;
            }
            proc_macro2::TokenTree::Punct(punct) => {
                let ch: char = punct.as_char();
                if ch == CHAR_HYPHEN {
                    result.push(CHAR_HYPHEN);
                    prev_needs_space = false;
                } else if ch == CHAR_COLON {
                    result.push(CHAR_COLON);
                    result.push(CHAR_SPACE);
                    prev_needs_space = false;
                } else if ch == CHAR_COMMA {
                    result.push(CHAR_COMMA);
                    result.push(CHAR_SPACE);
                    prev_needs_space = false;
                } else {
                    result.push(ch);
                    prev_needs_space = false;
                }
            }
            proc_macro2::TokenTree::Group(group) => {
                match group.delimiter() {
                    proc_macro2::Delimiter::Parenthesis => {
                        result.push(CHAR_LEFT_PAREN);
                        let inner: String = reconstruct_media_query(&group.stream());
                        result.push_str(&inner);
                        result.push(CHAR_RIGHT_PAREN);
                    }
                    proc_macro2::Delimiter::Brace => {
                        let inner: String = reconstruct_media_query(&group.stream());
                        result.push_str(&inner);
                    }
                    proc_macro2::Delimiter::Bracket => {
                        result.push(CHAR_LEFT_BRACKET);
                        let inner: String = reconstruct_media_query(&group.stream());
                        result.push_str(&inner);
                        result.push(CHAR_RIGHT_BRACKET);
                    }
                    proc_macro2::Delimiter::None => {
                        let inner: String = reconstruct_media_query(&group.stream());
                        result.push_str(&inner);
                    }
                }
                prev_needs_space = false;
            }
            proc_macro2::TokenTree::Literal(literal) => {
                if prev_needs_space {
                    result.push(CHAR_SPACE);
                }
                let literal_token_stream: proc_macro2::TokenStream =
                    proc_macro2::TokenTree::Literal(literal.clone()).into();
                if let Ok(literal_string) = parse2::<LitStr>(literal_token_stream) {
                    result.push_str(&literal_string.value());
                } else {
                    let literal_text: String = literal.to_string();
                    if literal_text.starts_with(CHAR_DOUBLE_QUOTE) {
                        if let Some(stripped) = literal_text
                            .strip_prefix(CHAR_DOUBLE_QUOTE)
                            .and_then(|text: &str| text.strip_suffix(CHAR_DOUBLE_QUOTE))
                        {
                            result.push_str(stripped);
                        } else {
                            result.push_str(&literal_text);
                        }
                    } else {
                        result.push_str(&literal_text);
                    }
                }
                prev_needs_space = true;
            }
        }
    }
    result
}

/// Checks whether all properties and nested blocks are fully static
/// (compile-time evaluable) for a list of selector blocks.
///
/// # Arguments
///
/// - `&[SelectorBlock]` - The selector blocks to check.
///
/// # Returns
///
/// - `bool` - `true` if all content is static.
pub(crate) fn is_selector_blocks_static(selector_blocks: &[SelectorBlock]) -> bool {
    selector_blocks.iter().all(|block: &SelectorBlock| {
        block
            .get_properties()
            .iter()
            .all(|(key, value): &(ClassPropKey, ClassPropValue)| {
                let ClassPropKey::Static(_) = key else {
                    return false;
                };
                let ClassPropValue::Expr(expr) = value;
                is_static_string_expr(expr)
            })
            && is_selector_blocks_static(block.get_selector_blocks())
    })
}

/// Checks whether all properties and nested blocks are fully static
/// (compile-time evaluable) for a list of at-rule blocks.
///
/// # Arguments
///
/// - `&[AtRuleBlock]` - The at-rule blocks to check.
///
/// # Returns
///
/// - `bool` - `true` if all content is static.
pub(crate) fn is_at_rule_blocks_static(at_rule_blocks: &[AtRuleBlock]) -> bool {
    at_rule_blocks.iter().all(|block: &AtRuleBlock| {
        block
            .get_properties()
            .iter()
            .all(|(key, value): &(ClassPropKey, ClassPropValue)| {
                let ClassPropKey::Static(_) = key else {
                    return false;
                };
                let ClassPropValue::Expr(expr) = value;
                is_static_string_expr(expr)
            })
            && is_selector_blocks_static(block.get_selector_blocks())
            && is_at_rule_blocks_static(block.get_at_rule_blocks())
    })
}
