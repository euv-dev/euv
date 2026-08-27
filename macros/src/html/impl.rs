use super::*;

/// Implementation of `Parse` for `HtmlRoot`, parsing zero or more HTML nodes.
impl Parse for HtmlRoot {
    /// Parses the root of an `html!` macro invocation.
    ///
    /// # Arguments
    ///
    /// - `ParseStream` - The syn parse stream to read from.
    ///
    /// # Returns
    ///
    /// - `syn::Result<Self>` - The parsed `HtmlRoot`, or a syntax error.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let children: Vec<HtmlNode> = parse_html_children(input)?;
        Ok(Self { children })
    }
}

/// Implementation of `ToTokens` for `HtmlRoot`, converting root HTML nodes into virtual node tokens.
///
/// - 0 children → `VirtualNode::Empty`
/// - 1 child → the child's token stream (no Fragment wrapper)
/// - N children → `VirtualNode::Fragment(vec![...])`
impl ToTokens for HtmlRoot {
    /// Converts the root HTML nodes into a single virtual node token stream.
    ///
    /// # Arguments
    ///
    /// - `&mut proc_macro2::TokenStream` - The target token stream to append to.
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(children_to_node_tokens(self.get_children()));
    }
}

/// Implementation of `Parse` for `HtmlNode`, parsing HTML input into a node.
impl Parse for HtmlNode {
    /// Parses a single HTML node from the token stream.
    ///
    /// # Arguments
    ///
    /// - `ParseStream` - The syn parse stream to read from.
    ///
    /// # Returns
    ///
    /// - `syn::Result<Self>` - The parsed `HtmlNode`, or a syntax error.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(LitStr) && input.peek2(Brace) {
            let element: HtmlElement = input.parse()?;
            return Ok(HtmlNode::Element(element));
        }
        if input.peek(LitStr) {
            let literal_string: LitStr = input.parse()?;
            return Ok(HtmlNode::Text(literal_string.value()));
        }
        if input.peek(Token![if]) {
            let html_if: HtmlIf = input.parse()?;
            return Ok(HtmlNode::If(html_if));
        }
        if input.peek(Token![match]) {
            let html_match: HtmlMatch = input.parse()?;
            return Ok(HtmlNode::Match(html_match));
        }
        if input.peek(Token![for]) {
            let html_for: HtmlFor = input.parse()?;
            return Ok(HtmlNode::For(html_for));
        }
        if input.peek(Brace) && input.peek2(Brace) {
            let forked: ParseBuffer<'_> = input.fork();
            let _first_brace: ParseBuffer<'_>;
            braced!(_first_brace in forked);
            let second_brace: ParseBuffer<'_>;
            braced!(second_brace in forked);
            if is_dynamic_tag_pattern(&second_brace, input) {
                let tag_content: ParseBuffer<'_>;
                braced!(tag_content in input);
                let tag_expr: Expr = tag_content.parse()?;
                let body_content: ParseBuffer<'_>;
                braced!(body_content in input);
                let (attributes, children): (HtmlAttrs, Vec<HtmlNode>) =
                    parse_dynamic_component_children(&body_content)?;
                return Ok(HtmlNode::DynamicTag(HtmlDynamicTag::new(
                    tag_expr, attributes, children,
                )));
            }
        }
        if input.peek(Brace) {
            let content: ParseBuffer<'_>;
            braced!(content in input);
            let expr: Expr = content.parse()?;
            return Ok(HtmlNode::Dynamic(expr));
        }
        if input.peek(Ident) {
            if input.peek2(Brace) {
                let element: HtmlElement = input.parse()?;
                return Ok(HtmlNode::Element(element));
            }
            let expr: Expr = input.parse()?;
            return Ok(HtmlNode::Expr(expr));
        }
        Err(input.error(ERR_EXPECTED_ELEMENT))
    }
}

/// Implementation of `Parse` for `HtmlIf`, parsing reactive and inline `if` conditionals.
impl Parse for HtmlIf {
    /// Parses a conditional into an `HtmlIf` AST.
    ///
    /// Each branch condition is independently parsed as either reactive (braced)
    /// or inline (plain expression). The overall `is_reactive` flag is set to
    /// `true` if any branch has a braced condition, causing the entire if-chain
    /// to be wrapped in a `DynamicNode` for reactive re-rendering.
    ///
    /// Supported syntaxes per branch:
    /// - Reactive: `{expr}` — the braced expression is treated as a signal.
    /// - Inline: `condition` — a plain Rust boolean expression.
    ///
    /// Any combination is valid, e.g.:
    /// - `if {a} {} else if {b} {}` — all reactive
    /// - `if a {} else if b {}` — all inline
    /// - `if {a} {} else if b {}` — mixed (first reactive, second inline)
    /// - `if a {} else if {b} {}` — mixed (first inline, second reactive)
    ///
    /// # Arguments
    ///
    /// - `ParseStream` - The syn parse stream to read from.
    ///
    /// # Returns
    ///
    /// - `syn::Result<Self>` - The parsed `HtmlIf`, or a syntax error.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut branches: Vec<(Option<Expr>, Vec<HtmlNode>, bool)> = Vec::new();
        let mut is_reactive: bool = false;
        input.parse::<Token![if]>()?;
        let branch_reactive: bool = input.peek(Brace);
        is_reactive = is_reactive || branch_reactive;
        let condition: Expr = if branch_reactive {
            let cond_content: ParseBuffer<'_>;
            braced!(cond_content in input);
            cond_content.parse()?
        } else {
            parse_expr_until_brace(input)?
        };
        let body_content: ParseBuffer<'_>;
        braced!(body_content in input);
        let body: Vec<HtmlNode> = parse_html_children(&body_content)?;
        branches.push((Some(condition), body, branch_reactive));
        while input.peek(Token![else]) {
            input.parse::<Token![else]>()?;
            if input.peek(Token![if]) {
                input.parse::<Token![if]>()?;
                let branch_reactive: bool = input.peek(Brace);
                is_reactive = is_reactive || branch_reactive;
                let condition: Expr = if branch_reactive {
                    let cond_content: ParseBuffer<'_>;
                    braced!(cond_content in input);
                    cond_content.parse()?
                } else {
                    parse_expr_until_brace(input)?
                };
                let body_content: ParseBuffer<'_>;
                braced!(body_content in input);
                let body: Vec<HtmlNode> = parse_html_children(&body_content)?;
                branches.push((Some(condition), body, branch_reactive));
            } else {
                let body_content: ParseBuffer<'_>;
                braced!(body_content in input);
                let body: Vec<HtmlNode> = parse_html_children(&body_content)?;
                branches.push((None, body, false));
                break;
            }
        }
        Ok(Self {
            is_reactive,
            branches,
        })
    }
}

/// Implementation of `Parse` for `HtmlMatch`, parsing reactive and inline match expressions.
///
/// Each arm body can be any valid HTML content (elements, expressions, if, etc.)
/// without requiring outer braces. Bodies are terminated by `,` or end of the
/// match block.
impl Parse for HtmlMatch {
    /// Parses a `match` expression into an `HtmlMatch` AST.
    ///
    /// Supports two syntaxes:
    /// - Reactive: `match {expr} { pattern => { children } ... }`
    ///   Detected when `match` is immediately followed by `{`.
    ///   The scrutinee expression in braces is treated as a signal that triggers re-rendering.
    /// - Inline: `match expr { pattern => { children } ... }`
    ///   Detected when `match` is followed by a non-`{` token.
    ///   The scrutinee is a plain Rust expression, evaluated once at render time.
    ///
    /// # Arguments
    ///
    /// - `ParseStream` - The syn parse stream to read from.
    ///
    /// # Returns
    ///
    /// - `syn::Result<Self>` - The parsed `HtmlMatch`, or a syntax error.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![match]>()?;
        let is_reactive: bool = input.peek(Brace);
        let scrutinee: Expr = if is_reactive {
            let scrutinee_content: ParseBuffer<'_>;
            braced!(scrutinee_content in input);
            scrutinee_content.parse()?
        } else {
            parse_expr_until_brace(input)?
        };
        let arms_content: ParseBuffer<'_>;
        braced!(arms_content in input);
        let mut arms: Vec<(proc_macro2::TokenStream, Vec<HtmlNode>)> = Vec::new();
        while !arms_content.is_empty() {
            let mut pattern_tokens: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
            while !arms_content.peek(Token![=>]) {
                let token_tree: proc_macro2::TokenTree = arms_content.parse()?;
                pattern_tokens.extend([token_tree]);
            }
            arms_content.parse::<Token![=>]>()?;
            let body: Vec<HtmlNode> = parse_match_arm_body(&arms_content)?;
            arms.push((pattern_tokens, body));
            if arms_content.peek(Token![,]) {
                arms_content.parse::<Token![,]>()?;
            }
        }
        Ok(Self {
            is_reactive,
            scrutinee,
            arms,
        })
    }
}

/// Implementation of `Parse` for `HtmlFor`, parsing reactive and inline for loops.
impl Parse for HtmlFor {
    /// Parses a `for` loop into an `HtmlFor` AST.
    ///
    /// Supports two syntaxes:
    /// - Reactive: `for pattern in {expr} { children }`
    ///   Detected when `in` is immediately followed by `{`.
    ///   The iterable expression in braces is treated as a signal that triggers re-rendering.
    /// - Inline: `for pattern in expr { children }`
    ///   Detected when `in` is followed by a non-`{` token.
    ///   The iterable is a plain Rust expression, evaluated once at render time.
    ///
    /// # Arguments
    ///
    /// - `ParseStream` - The syn parse stream to read from.
    ///
    /// # Returns
    ///
    /// - `syn::Result<Self>` - The parsed `HtmlFor`, or a syntax error.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![for]>()?;
        let mut pattern_tokens: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
        while !input.peek(Token![in]) {
            let token_tree: proc_macro2::TokenTree = input.parse()?;
            pattern_tokens.extend([token_tree]);
        }
        input.parse::<Token![in]>()?;
        let is_reactive: bool = input.peek(Brace);
        let iterable: Expr = if is_reactive {
            let iter_content: ParseBuffer<'_>;
            braced!(iter_content in input);
            iter_content.parse()?
        } else {
            parse_expr_until_brace(input)?
        };
        let body_content: ParseBuffer<'_>;
        braced!(body_content in input);
        let body: Vec<HtmlNode> = parse_html_children(&body_content)?;
        Ok(Self {
            is_reactive,
            pattern: pattern_tokens,
            iterable,
            body,
        })
    }
}

/// Implementation of `Parse` for `HtmlElement`, parsing HTML element syntax.
impl Parse for HtmlElement {
    /// Parses an HTML element with its attributes and children.
    ///
    /// # Arguments
    ///
    /// - `ParseStream` - The syn parse stream to read from.
    ///
    /// # Returns
    ///
    /// - `syn::Result<Self>` - The parsed `HtmlElement`, or a syntax error.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let (tag, tag_name, is_ident_tag): (Ident, String, bool) = if input.peek(LitStr) {
            let literal: LitStr = input.parse()?;
            let tag_name: String = literal.value();
            let tag: Ident = Ident::new(&tag_name, literal.span());
            (tag, tag_name, false)
        } else {
            let tag: Ident = input.parse()?;
            let tag_str: String = tag.to_string();
            (tag, tag_str, true)
        };
        let content: ParseBuffer<'_>;
        braced!(content in input);
        let mut attributes: HtmlAttrs = Vec::new();
        let mut children: Vec<HtmlNode> = Vec::new();
        while !content.is_empty() {
            if is_attr_key_pattern(&content) && !is_double_colon(&content) {
                let key_string: String = parse_ident_name(&content)?;
                let key_literal: LitStr = LitStr::new(&key_string, content.span());
                content.parse::<Colon>()?;
                let key_str: String = key_string
                    .strip_prefix(RAW_IDENT_PREFIX)
                    .unwrap_or(&key_string)
                    .to_string();
                let value: HtmlAttrValue = parse_attr_value(&content, &key_str)?;
                attributes.push((key_literal.to_token_stream(), value))
            } else if content.peek(Token![if]) {
                let html_if: HtmlIf = content.parse()?;
                children.push(HtmlNode::If(html_if));
            } else if content.peek(Token![match]) {
                let html_match: HtmlMatch = content.parse()?;
                children.push(HtmlNode::Match(html_match));
            } else if content.peek(Token![for]) {
                let html_for: HtmlFor = content.parse()?;
                children.push(HtmlNode::For(html_for));
            } else if content.peek(Brace) && content.peek2(Brace) {
                let forked: ParseBuffer<'_> = content.fork();
                let _first_brace: ParseBuffer<'_>;
                braced!(_first_brace in forked);
                let second_brace: ParseBuffer<'_>;
                braced!(second_brace in forked);
                if is_dynamic_tag_pattern(&second_brace, &content) {
                    let tag_content: ParseBuffer<'_>;
                    braced!(tag_content in content);
                    let tag_expr: Expr = tag_content.parse()?;
                    let body_content: ParseBuffer<'_>;
                    braced!(body_content in content);
                    let (dynamic_attrs, dynamic_children): (HtmlAttrs, Vec<HtmlNode>) =
                        parse_dynamic_component_children(&body_content)?;
                    children.push(HtmlNode::DynamicTag(HtmlDynamicTag::new(
                        tag_expr,
                        dynamic_attrs,
                        dynamic_children,
                    )));
                } else {
                    let child_content: ParseBuffer<'_>;
                    braced!(child_content in content);
                    let expr: Expr = child_content.parse()?;
                    children.push(HtmlNode::Dynamic(expr));
                }
            } else if content.peek(Brace) && content.peek2(Colon) {
                let key_content: ParseBuffer<'_>;
                braced!(key_content in content);
                let key_expr: Expr = key_content.parse()?;
                content.parse::<Colon>()?;
                let value: HtmlAttrValue = parse_attr_value(&content, STR_EMPTY)?;
                attributes.push((key_expr.to_token_stream(), value));
            } else if content.peek(Brace) {
                let child_content: ParseBuffer<'_>;
                braced!(child_content in content);
                let expr: Expr = child_content.parse()?;
                children.push(HtmlNode::Dynamic(expr));
            } else if content.peek(LitStr) && content.peek2(Brace) {
                let element: HtmlElement = content.parse()?;
                children.push(HtmlNode::Element(element));
            } else if content.peek(LitStr) && content.peek2(Colon) {
                let key_literal: LitStr = content.parse()?;
                let key_str: String = key_literal.value();
                content.parse::<Colon>()?;
                let value: HtmlAttrValue = parse_attr_value(&content, &key_str)?;
                attributes.push((key_literal.to_token_stream(), value));
            } else if content.peek(LitStr) {
                let literal_string: LitStr = content.parse()?;
                children.push(HtmlNode::Text(literal_string.value()));
            } else if content.peek(Ident) {
                if content.peek2(Brace) {
                    let element: HtmlElement = content.parse()?;
                    children.push(HtmlNode::Element(element));
                } else if is_ident_tag
                    && is_user_fn(&tag_name)
                    && !content.peek2(Paren)
                    && get_user_fn_props_fields(&tag_name).is_some_and(|fields: &Vec<String>| {
                        let forked: ParseBuffer<'_> = content.fork();
                        forked
                            .parse::<Ident>()
                            .map(|ident: Ident| fields.contains(&ident.to_string()))
                            .unwrap_or_default()
                    })
                {
                    let key: Ident = content.parse()?;
                    let value_expr: Expr = syn::parse_quote!(#key);
                    attributes.push((key.to_token_stream(), HtmlAttrValue::Expr(value_expr)));
                } else {
                    let expr: Expr = content.parse()?;
                    children.push(HtmlNode::Expr(expr));
                }
            } else {
                return Err(content.error(ERR_UNEXPECTED_TOKEN_IN_ELEMENT));
            }
        }
        let merged_attributes: HtmlAttrs = merge_same_key_attributes(attributes);
        Ok(Self {
            tag,
            tag_name,
            is_ident_tag,
            attributes: merged_attributes,
            children,
        })
    }
}

/// Implementation of `ToTokens` for `HtmlNode`, converting HTML nodes into virtual node tokens.
impl ToTokens for HtmlNode {
    /// Converts this HTML node into its corresponding virtual node token stream.
    ///
    /// # Arguments
    ///
    /// - `&mut proc_macro2::TokenStream` - The target token stream to append to.
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            HtmlNode::Element(element) => element.to_tokens(tokens),
            HtmlNode::Text(text) => {
                tokens.extend(quote! {
                    ::euv_core::VirtualNode::Text(::euv_core::TextNode::new(#text.into(), None))
                });
            }
            HtmlNode::Expr(expr) => {
                tokens.extend(quote! {
                    (#expr).into()
                });
            }
            HtmlNode::Dynamic(expr) => {
                tokens.extend(quote! {
                    ::euv_core::VirtualNode::create_dynamic(move |_: &mut ::euv_core::HookContext| (#expr).into())
                });
            }
            HtmlNode::If(html_if) => {
                if html_if.get_is_reactive() {
                    let if_chain: proc_macro2::TokenStream =
                        build_html_if_chain(html_if.get_branches());
                    tokens.extend(quote! {
                    ::euv_core::VirtualNode::create_dynamic(move |_: &mut ::euv_core::HookContext| { #if_chain })
                });
                } else {
                    let if_chain: proc_macro2::TokenStream =
                        build_html_if_chain(html_if.get_branches());
                    tokens.extend(quote! {
                        { #if_chain }
                    });
                }
            }
            HtmlNode::Match(html_match) => {
                let scrutinee_expr: &Expr = strip_braces_from_expr(html_match.get_scrutinee());
                let scrutinee_tokens: proc_macro2::TokenStream =
                    auto_get_expr_tokens(scrutinee_expr, html_match.get_is_reactive());
                if html_match.get_is_reactive() {
                    let arm_tokens: Vec<proc_macro2::TokenStream> = html_match
                        .get_arms()
                        .iter()
                        .enumerate()
                        .map(
                            |(arm_index, (pattern, body)): (
                                usize,
                                &(proc_macro2::TokenStream, Vec<HtmlNode>),
                            )| {
                                let body_expr: proc_macro2::TokenStream =
                                    children_to_node_tokens(body);
                                quote! {
                                    #pattern => {
                                        __euv_hook_context.switch_arm(#arm_index);
                                        #body_expr
                                    }
                                }
                            },
                        )
                        .collect();
                    tokens.extend(quote! {
                        ::euv_core::VirtualNode::create_dynamic(move |__euv_hook_context: &mut ::euv_core::HookContext| {
                            match #scrutinee_tokens {
                                #(#arm_tokens)*
                            }
                        })
                    });
                } else {
                    let arm_tokens: Vec<proc_macro2::TokenStream> = html_match
                        .get_arms()
                        .iter()
                        .map(
                            |(pattern, body): &(proc_macro2::TokenStream, Vec<HtmlNode>)| {
                                let body_expr: proc_macro2::TokenStream =
                                    children_to_node_tokens(body);
                                quote! {
                                    #pattern => #body_expr
                                }
                            },
                        )
                        .collect();
                    tokens.extend(quote! {
                        {
                            match #scrutinee_tokens {
                                #(#arm_tokens)*
                            }
                        }
                    });
                }
            }
            HtmlNode::For(html_for) => {
                let pattern: &proc_macro2::TokenStream = html_for.get_pattern();
                let iterable: &Expr = html_for.get_iterable();
                let iterable_tokens: proc_macro2::TokenStream =
                    auto_get_expr_tokens(iterable, html_for.get_is_reactive());
                let body_tokens: proc_macro2::TokenStream = children_to_tokens(html_for.get_body());
                let for_tokens: proc_macro2::TokenStream = quote! {
                    let mut __euv_nodes: Vec<::euv_core::VirtualNode> = Vec::new();
                    for #pattern in #iterable_tokens {
                        __euv_nodes.extend(#body_tokens);
                    }
                    ::euv_core::VirtualNode::Fragment(__euv_nodes)
                };
                if html_for.get_is_reactive() {
                    tokens.extend(quote! {
                        ::euv_core::VirtualNode::create_dynamic(move |_: &mut ::euv_core::HookContext| {
                            #for_tokens
                        })
                    });
                } else {
                    tokens.extend(for_tokens);
                }
            }
            HtmlNode::DynamicTag(dynamic_tag) => {
                dynamic_tag.to_tokens(tokens);
            }
        }
    }
}

/// Implementation of `ToTokens` for `HtmlStylePropValue`, converting style property values into tokens.
impl ToTokens for HtmlStylePropValue {
    /// Converts this style property value into its token stream representation.
    ///
    /// # Arguments
    ///
    /// - `&mut proc_macro2::TokenStream` - The target token stream to append to.
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            HtmlStylePropValue::Literal(literal_value) => literal_value.to_tokens(tokens),
            HtmlStylePropValue::Expr(expr) => expr.to_tokens(tokens),
            HtmlStylePropValue::If(html_attr_if) => {
                let mode: AttrIfMode = if html_attr_if.get_is_inline() {
                    AttrIfMode::Raw
                } else {
                    AttrIfMode::Reactive
                };
                let ctx: AttrIfContext<'_> =
                    AttrIfContext::new(html_attr_if, html_attr_if.get_else_default(), mode);
                let if_chain: proc_macro2::TokenStream = attr_if_to_tokens(&ctx);
                if_chain.to_tokens(tokens);
            }
            HtmlStylePropValue::Match(html_attr_match) => {
                let mode: AttrIfMode = if html_attr_match.get_is_inline() {
                    AttrIfMode::Raw
                } else {
                    AttrIfMode::Reactive
                };
                let match_expr: proc_macro2::TokenStream =
                    attr_match_to_tokens(html_attr_match, mode);
                match_expr.to_tokens(tokens);
            }
        }
    }
}

/// Implementation of `ToTokens` for `HtmlAttrValue`, converting attribute values into tokens.
///
/// For `HtmlAttrValue::If` and `HtmlAttrValue::Match`, generates either a reactive signal
/// or an inline expression depending on whether the conditional is reactive or inline.
/// For `HtmlAttrValue::Style` containing conditionals, generates a reactive signal.
/// For static values (`Expr` and `Style` without conditionals), the value is emitted directly.
impl ToTokens for HtmlAttrValue {
    /// Converts this attribute value into its token stream representation.
    ///
    /// # Arguments
    ///
    /// - `&mut proc_macro2::TokenStream` - The target token stream to append to.
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            HtmlAttrValue::Expr(expr) => expr.to_tokens(tokens),
            HtmlAttrValue::If(html_attr_if) => {
                if html_attr_if.get_is_inline() {
                    let ctx: AttrIfContext<'_> = AttrIfContext::new(
                        html_attr_if,
                        html_attr_if.get_else_default(),
                        AttrIfMode::Raw,
                    );
                    let if_chain: proc_macro2::TokenStream = attr_if_to_tokens(&ctx);
                    tokens.extend(quote! {
                        ::euv_core::AttrValueAdapter::new(#if_chain).into()
                    });
                } else {
                    let ctx: AttrIfContext<'_> = AttrIfContext::new(
                        html_attr_if,
                        html_attr_if.get_else_default(),
                        AttrIfMode::Reactive,
                    );
                    let if_chain: proc_macro2::TokenStream = attr_if_to_tokens(&ctx);
                    tokens.extend(quote! {
                        ::euv_core::AttributeValue::reactive(move || #if_chain)
                    });
                }
            }
            HtmlAttrValue::Match(html_attr_match) => {
                if html_attr_match.get_is_inline() {
                    let match_expr: proc_macro2::TokenStream =
                        attr_match_to_tokens(html_attr_match, AttrIfMode::Raw);
                    tokens.extend(quote! {
                        ::euv_core::AttrValueAdapter::new(#match_expr).into()
                    });
                } else {
                    let match_expr: proc_macro2::TokenStream =
                        attr_match_to_tokens(html_attr_match, AttrIfMode::Reactive);
                    tokens.extend(quote! {
                        ::euv_core::AttributeValue::reactive(move || #match_expr)
                    });
                }
            }
            HtmlAttrValue::Style(props) => {
                let has_conditional: bool = is_style_props_conditional(props);
                let has_inline: bool = is_attr_value_inline(self);
                let all_literal: bool =
                    props
                        .iter()
                        .all(|(_, value): &(String, HtmlStylePropValue)| {
                            matches!(value, HtmlStylePropValue::Literal(_))
                        });
                if has_inline {
                    let prop_tokens: Vec<proc_macro2::TokenStream> = props
                        .iter()
                        .map(|(key, value): &(String, HtmlStylePropValue)| {
                            let value_tokens: proc_macro2::TokenStream = match value {
                                HtmlStylePropValue::If(html_attr_if)
                                    if html_attr_if.get_is_inline() =>
                                {
                                    let ctx: AttrIfContext<'_> = AttrIfContext::new(
                                        html_attr_if,
                                        html_attr_if.get_else_default(),
                                        AttrIfMode::Raw,
                                    );
                                    let if_chain: proc_macro2::TokenStream =
                                        attr_if_to_tokens(&ctx);
                                    quote! { #if_chain }
                                }
                                HtmlStylePropValue::Match(html_attr_match)
                                    if html_attr_match.get_is_inline() =>
                                {
                                    let match_expr: proc_macro2::TokenStream =
                                        attr_match_to_tokens(html_attr_match, AttrIfMode::Raw);
                                    quote! { #match_expr }
                                }
                                _ => quote! { #value },
                            };
                            quote! { (#key, #value_tokens) }
                        })
                        .collect();
                    tokens.extend(quote! {
                        ::euv_core::Css::style_string(&[#(#prop_tokens), *])
                    });
                } else if has_conditional {
                    let prop_tokens: Vec<proc_macro2::TokenStream> = props
                        .iter()
                        .map(|(key, value): &(String, HtmlStylePropValue)| {
                            quote! { (#key.to_string(), (#value).into()) }
                        })
                        .collect();
                    tokens.extend(quote! {
                        ::euv_core::AttributeValue::reactive(move || ::euv_core::Css::style_string_owned(&[#(#prop_tokens), *]))
                    });
                } else if all_literal {
                    let mut css_string: String = String::new();
                    for (key, value) in props {
                        if !css_string.is_empty() {
                            css_string.push(CHAR_SPACE);
                        }
                        css_string.push_str(key);
                        css_string.push_str(CSS_PROP_SEPARATOR);
                        if let HtmlStylePropValue::Literal(literal_value) = value {
                            css_string.push_str(literal_value);
                        }
                        css_string.push(CHAR_CSS_DECL_TERMINATOR);
                    }
                    tokens.extend(quote! {
                        #css_string.to_string()
                    });
                } else {
                    let key_value_tokens: Vec<proc_macro2::TokenStream> = props
                        .iter()
                        .map(|(key, value): &(String, HtmlStylePropValue)| {
                            quote! { (#key, #value) }
                        })
                        .collect();
                    tokens.extend(quote! {
                        ::euv_core::Css::style_string(&[#(#key_value_tokens), *])
                    });
                }
            }
            HtmlAttrValue::Classes(values) => {
                let value_tokens: Vec<proc_macro2::TokenStream> = values
                    .iter()
                    .map(|value: &HtmlAttrValue| {
                        let ctx: AttrValueContext<'_> =
                            AttrValueContext::new(value, ATTR_KEY_CLASS, false);
                        attr_value_to_attribute_value_tokens(&ctx)
                    })
                    .collect();
                tokens.extend(quote! {
                    ::euv_core::AttributeValue::merge_class(&[#(#value_tokens), *])
                });
            }
            HtmlAttrValue::Styles(values) => {
                let value_tokens: Vec<proc_macro2::TokenStream> = values
                    .iter()
                    .map(style_value_to_attribute_value_tokens)
                    .collect();
                tokens.extend(quote! {
                    ::euv_core::AttributeValue::merge_style(&[#(#value_tokens), *])
                });
            }
        }
    }
}

/// Implementation of `ToTokens` for `HtmlDynamicTag`, converting dynamic tags
/// into a reactive `DynamicNode` that re-renders when the tag expression changes.
///
/// The generated code wraps the tag expression in a `VirtualNode::create_dynamic`
/// closure. Inside the closure, the expression is evaluated to a tag name string.
/// If the tag name matches a registered user component, the component function
/// is called with default props and attributes/children are injected. Otherwise,
/// a native HTML element is created.
impl ToTokens for HtmlDynamicTag {
    /// Converts this dynamic tag into its virtual node token stream.
    ///
    /// # Arguments
    ///
    /// - `&mut proc_macro2::TokenStream` - The target token stream to append to.
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let tag_expr: &Expr = self.get_tag_expr();
        let children: &[HtmlNode] = self.get_children();
        let attr_tokens: Vec<proc_macro2::TokenStream> = self.attribute_entry_tokens();
        let child_tokens: Vec<proc_macro2::TokenStream> = nodes_to_token_vec(children);
        let component_registry: HashMap<String, ComponentInfo> = get_loaded_component_registry();
        let component_match_arms: Vec<proc_macro2::TokenStream> = component_registry
            .iter()
            .map(|(fn_name, component_info): (&String, &ComponentInfo)| {
                self.component_match_arm_tokens(
                    fn_name,
                    component_info,
                    &attr_tokens,
                    &child_tokens,
                )
            })
            .collect();
        tokens.extend(quote! {
            ::euv_core::VirtualNode::create_dynamic(move |_: &mut ::euv_core::HookContext| {
                let __euv_tag_name: String = (#tag_expr).to_string();
                match __euv_tag_name.as_str() {
                    #(#component_match_arms)*
                    _ => {
                        ::euv_core::VirtualNode::Element {
                            tag: ::euv_core::Tag::Element(::std::borrow::Cow::Owned(__euv_tag_name)),
                            attributes: vec![#(#attr_tokens), *],
                            children: vec![#(#child_tokens), *],
                            key: None,
                            props: None,
                        }
                    }
                }
            })
        });
    }
}

/// Inherent implementation of [`HtmlDynamicTag`].
impl HtmlDynamicTag {
    /// Builds `AttributeEntry::new(...)` tokens for each attribute of the
    /// dynamic tag. The result is shared between the native-element fallback
    /// arm and each component arm (which may splice its non-prop entries
    /// back into the component's element).
    ///
    /// # Returns
    ///
    /// - `Vec<proc_macro2::TokenStream>` - A `Vec<proc_macro2::TokenStream>` value.
    fn attribute_entry_tokens(&self) -> Vec<proc_macro2::TokenStream> {
        self.get_attributes()
            .iter()
            .map(|(key, value): &(proc_macro2::TokenStream, HtmlAttrValue)| {
                let key_string: String = extract_attr_key_string(key);
                // OPT 2: literal attribute keys become `Cow::Borrowed`,
                // matching the static-element fast path.
                let attr_name_token: proc_macro2::TokenStream =
                    quote! { ::std::borrow::Cow::Borrowed(#key_string) };
                let ctx: AttrEntryContext<'_> = AttrEntryContext::new(value, &key_string);
                let value_tokens: proc_macro2::TokenStream = attr_value_to_entry_value_tokens(&ctx);
                quote! {
                    ::euv_core::AttributeEntry::new(#attr_name_token, #value_tokens)
                }
            })
            .collect()
    }

    /// Emits the `fn_name_str => { ... }` arm for one registered component.
    ///
    /// Calls the component function with a Props literal built from the
    /// attributes that map to props fields, then splices any remaining
    /// entries (class / style / event handlers) into the returned element's
    /// attribute list.
    ///
    /// # Arguments
    ///
    /// - `&str` - Shared reference to a `str`.
    /// - `&ComponentInfo` - Shared reference to a `ComponentInfo`.
    /// - `&[proc_macro2::TokenStream]` - Shared reference to a `[proc_macro2::TokenStream]`.
    /// - `&[proc_macro2::TokenStream]` - Shared reference to a `[proc_macro2::TokenStream]`.
    ///
    /// # Returns
    ///
    /// - `proc_macro2::TokenStream` - A `proc_macro2::TokenStream` value.
    fn component_match_arm_tokens(
        &self,
        fn_name: &str,
        component_info: &ComponentInfo,
        attr_tokens: &[proc_macro2::TokenStream],
        dyn_child_tokens: &[proc_macro2::TokenStream],
    ) -> proc_macro2::TokenStream {
        let fn_ident: Ident = Ident::new(fn_name, proc_macro2::Span::call_site());
        let props_ident: Ident = Ident::new(
            component_info.get_props_type(),
            proc_macro2::Span::call_site(),
        );
        let fn_name_str: String = fn_name.to_string();
        let props_fields: &Vec<String> = component_info.get_props_fields();
        let props_field_types: &HashMap<String, String> = component_info.get_props_field_types();
        let prop_field_tokens: Vec<proc_macro2::TokenStream> =
            self.dyn_prop_field_tokens(props_fields, props_field_types);
        let non_prop_attr_tokens: Vec<proc_macro2::TokenStream> =
            self.dyn_non_prop_attr_tokens(attr_tokens);
        let props_init_tokens: proc_macro2::TokenStream = if prop_field_tokens.is_empty() {
            quote! { #props_ident::default() }
        } else if prop_field_tokens.len() == props_fields.len() {
            quote! { #props_ident { #(#prop_field_tokens), * } }
        } else {
            quote! { #props_ident { #(#prop_field_tokens), *, ..Default::default() } }
        };
        let has_children_field: bool = props_fields.contains(&ATTR_KEY_CHILDREN.to_string());
        let component_call_tokens: proc_macro2::TokenStream = if has_children_field {
            let children_token: proc_macro2::TokenStream =
                children_to_node_tokens(self.get_children());
            quote! { #fn_ident(::euv_core::VirtualNode::Element {
                tag: ::euv_core::Tag::Component(::std::borrow::Cow::Borrowed(#fn_name_str)),
                attributes: Vec::new(),
                children: vec![#(#dyn_child_tokens), *],
                key: None,
                props: Some(Box::new(#props_ident { children: #children_token, ..#props_init_tokens })),
            }) }
        } else {
            quote! { #fn_ident(::euv_core::VirtualNode::Element {
                tag: ::euv_core::Tag::Component(::std::borrow::Cow::Borrowed(#fn_name_str)),
                attributes: Vec::new(),
                children: vec![#(#dyn_child_tokens), *],
                key: None,
                props: Some(Box::new(#props_init_tokens)),
            }) }
        };
        quote! {
            #fn_name_str => {
                #component_call_tokens.extend_attributes([#(#non_prop_attr_tokens), *])
            }
        }
    }

    /// Builds the `field: expr` tokens for the props literal of a single
    /// dynamic-tag component, filtering attributes down to the props fields
    /// and converting each via the standard per-variant helpers.
    ///
    /// # Arguments
    ///
    /// - `&[String]` - Shared reference to a `[String]`.
    /// - `&HashMap<String, String>` - Shared reference to a `HashMap<String, String>`.
    ///
    /// # Returns
    ///
    /// - `Vec<proc_macro2::TokenStream>` - A `Vec<proc_macro2::TokenStream>` value.
    fn dyn_prop_field_tokens(
        &self,
        props_fields: &[String],
        props_field_types: &HashMap<String, String>,
    ) -> Vec<proc_macro2::TokenStream> {
        self.get_attributes()
            .iter()
            .filter_map(|(key, value): &(proc_macro2::TokenStream, HtmlAttrValue)| {
                let key_string: String = extract_attr_key_string(key);
                if !props_fields.contains(&key_string) {
                    return None;
                }
                let field_ident: Ident = Ident::new(&key_string, proc_macro2::Span::call_site());
                Some(prop_field_token(
                    &field_ident,
                    &key_string,
                    value,
                    props_field_types,
                ))
            })
            .collect()
    }

    /// Selects the non-prop attribute entries (class, style, event handlers)
    /// by indexing into the shared `attr_tokens` vec, so the emitted code
    /// reuses the already-built `AttributeEntry` literals.
    ///
    /// # Arguments
    ///
    /// - `&[proc_macro2::TokenStream]` - Shared reference to a `[proc_macro2::TokenStream]`.
    ///
    /// # Returns
    ///
    /// - `Vec<proc_macro2::TokenStream>` - A `Vec<proc_macro2::TokenStream>` value.
    fn dyn_non_prop_attr_tokens(
        &self,
        attr_tokens: &[proc_macro2::TokenStream],
    ) -> Vec<proc_macro2::TokenStream> {
        self.get_attributes()
            .iter()
            .enumerate()
            .filter(
                |(_, (key, _)): &(usize, &(proc_macro2::TokenStream, HtmlAttrValue))| {
                    let key_string: String = extract_attr_key_string(key);
                    key_string == ATTR_KEY_CLASS
                        || key_string == ATTR_KEY_STYLE
                        || key_string.starts_with(EVENT_ATTR_PREFIX)
                },
            )
            .map(
                |(index, _): (usize, &(proc_macro2::TokenStream, HtmlAttrValue))| {
                    attr_tokens[index].clone()
                },
            )
            .collect()
    }
}

/// Implementation of `ToTokens` for `HtmlElement`, converting HTML elements into virtual element tokens.
///
/// For identifier tags, the macro checks whether the tag name corresponds to a
/// user-defined function in the project source. If it does, the tag is treated
/// as a component function call with typed Props struct initialization and
/// children passed as a separate `VirtualNode` argument.
///
/// For non-component tags (HTML elements), the existing `VirtualNode::Element`
/// construction with `Tag::Element` is preserved.
///
/// String literal tags always produce `Tag::Element`, supporting custom
/// HTML5 elements (Web Components).
impl ToTokens for HtmlElement {
    /// Converts this HTML element into its virtual element token stream.
    ///
    /// # Arguments
    ///
    /// - `&mut proc_macro2::TokenStream` - The target token stream to append to.
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let tag_name: String = self.get_tag_name().clone();
        let tag_ident: Ident = self.get_tag().clone();
        let tag_span: Span = self.get_tag().span();
        // OPT 2: emit `Cow::Borrowed("div")` directly from the
        // compile-time-known tag name string. (Previously this was
        // `#tag_name.to_string()`, which materialised a fresh `String`
        // per element at runtime.) The token stream below contains
        // the literal `div` (not `"div".to_string()`), wrapped in
        // `Cow::Borrowed(...)` at the call site.
        let tag_literal: proc_macro2::TokenStream = quote_spanned!(tag_span=> #tag_name);
        // `portal { target: "#root" } children` is a special
        // pseudo-element handled at the macro level. It lowers to
        // `Tag::Portal(target_string)` with the children spliced
        // in directly, so the renderer can recognise it as a
        // portal without going through a runtime dispatch path.
        let is_portal: bool = tag_name == "portal";
        let is_component: bool = !is_portal && self.get_is_ident_tag() && is_user_fn(&tag_name);
        if is_portal {
            tokens.extend(self.portal_element_tokens());
        } else if is_component {
            tokens.extend(self.component_call_tokens(&tag_name, &tag_ident, tag_span));
        } else {
            tokens.extend(self.native_element_tokens(&tag_literal));
        }
    }
}

/// Inherent implementation of [`HtmlElement`].
impl HtmlElement {
    /// Emits the tokens for a `portal { target: "..." } children` element.
    ///
    /// Produces a `VirtualNode::Element { tag: Tag::Portal(target), ... }`
    /// literal. The `target` attribute is required and must evaluate to
    /// `&str` / `String` / `&String`. Anything else is a compile-time
    /// error via the `&str` constraint enforced by `to_string()`.
    ///
    /// Children are spliced verbatim — the renderer takes care of
    /// appending each child node to the resolved target element
    /// (rather than to the placeholder marker that lives in the
    /// declared position).
    ///
    /// # Returns
    ///
    /// - `proc_macro2::TokenStream` - A `proc_macro2::TokenStream` value.
    fn portal_element_tokens(&self) -> proc_macro2::TokenStream {
        // Find the `target:` attribute. We refuse to silently fall
        // back to `"body"` here because that would mask wiring
        // errors at the call site (the wrong target would still
        // "work" by appending to document.body). Better to surface
        // a clear "missing target attribute" panic at runtime than
        // a subtle off-by-one target.
        //
        // `String::from(...)` is the conversion path because
        // `Tag::Portal(String)` owns its payload and the macro
        // cannot call `.to_string()` on user expressions that do
        // not implement `Display` (e.g. `Signal<String>`, which
        // exposes a `.get()` accessor instead). `String::from`
        // accepts `&str`, `&String`, and any `Into<String>` source
        // — so `target: "#root"` and `target: signal.clone()` (a
        // `Signal<String>` is not `Into<String>`) require the user
        // to write `target: signal.get()`. That mirrors what the
        // user would have written for a manual
        // `Tag::Portal(...)` construction.
        let target_expr: proc_macro2::TokenStream = self
            .get_attributes()
            .iter()
            .find_map(|(key, value): &(proc_macro2::TokenStream, HtmlAttrValue)| {
                let key_string: String = extract_attr_key_string(key);
                if key_string != "target" {
                    return None;
                }
                if let HtmlAttrValue::Expr(expr) = value {
                    // OPT 2: portal targets are runtime expressions
                    // (Signal<String> via `.get()`, `String::from(s)`,
                    // plain string literal). Wrap the result in
                    // `Cow::Owned(String::from(expr))` so the rendered
                    // DOM still owns its selector when it really needs
                    // to. For the literal-string case
                    // (`target: "#root"`), `String::from` will heap
                    // allocate once; the common dynamic-tag fast path
                    // (in `native_element_tokens`) skips that.
                    Some(quote! {
                        ::std::borrow::Cow::Owned(::std::string::String::from(#expr))
                    })
                } else {
                    None
                }
            })
            .unwrap_or_else(|| {
                quote! { compile_error!("portal element requires a `target:` attribute") }
            });
        let attr_tokens: Vec<proc_macro2::TokenStream> = self
            .get_attributes()
            .iter()
            .filter_map(|(key, value): &(proc_macro2::TokenStream, HtmlAttrValue)| {
                let key_string: String = extract_attr_key_string(key);
                if key_string == "target" {
                    // Already consumed above as the portal target.
                    return None;
                }
                let value_tokens: proc_macro2::TokenStream =
                    attr_value_to_entry_value_tokens(&AttrEntryContext::new(value, &key_string));
                Some(quote! { ::euv_core::AttributeEntry::new(::std::borrow::Cow::Borrowed(#key_string), #value_tokens) })
            })
            .collect();
        let children_tokens: proc_macro2::TokenStream =
            children_to_flattened_tokens(self.get_children());
        quote! {
            ::euv_core::VirtualNode::Element {
                tag: ::euv_core::Tag::Portal(#target_expr),
                attributes: vec![#(#attr_tokens), *],
                children: #children_tokens,
                key: None,
                props: None,
            }
        }
    }

    /// Emits the tokens for a component element (e.g. `euv_button { ... }`).
    ///
    /// Produces a `<component-name>(VirtualNode::Element { ... })` invocation
    /// where the element wraps the props struct initialization and the
    /// element's own children.
    ///
    /// # Arguments
    ///
    /// - `&str` - Shared reference to a `str`.
    /// - `&Ident` - Shared reference to a `Ident`.
    /// - `Span` - A `Span` parameter.
    ///
    /// # Returns
    ///
    /// - `proc_macro2::TokenStream` - A `proc_macro2::TokenStream` value.
    fn component_call_tokens(
        &self,
        tag_name: &str,
        tag_ident: &Ident,
        tag_span: Span,
    ) -> proc_macro2::TokenStream {
        let props_type_name: &str = get_user_fn_props_type(tag_name).unwrap_or(STR_EMPTY);
        let props_type_ident: Ident = Ident::new(props_type_name, tag_span);
        let props_field_types: HashMap<String, String> = get_user_fn_props_field_types(tag_name)
            .cloned()
            .unwrap_or_default();
        let prop_field_tokens: Vec<proc_macro2::TokenStream> =
            self.prop_field_tokens(&props_field_types);
        let props_init_tokens: proc_macro2::TokenStream = if prop_field_tokens.is_empty() {
            quote! { #props_type_ident::default() }
        } else if prop_field_tokens.len() == props_field_types.len() {
            quote! { #props_type_ident { #(#prop_field_tokens), * } }
        } else {
            quote! { #props_type_ident { #(#prop_field_tokens), *, ..Default::default() } }
        };
        let child_tokens: Vec<proc_macro2::TokenStream> = nodes_to_token_vec(self.get_children());
        quote! {
            #tag_ident(::euv_core::VirtualNode::Element {
                tag: ::euv_core::Tag::Component(::std::borrow::Cow::Borrowed(#tag_name)),
                attributes: Vec::new(),
                children: vec![#(#child_tokens), *],
                key: None,
                props: Some(Box::new(#props_init_tokens)),
            })
        }
    }

    /// Builds the `field: value` pairs used inside the Props struct literal
    /// for a component element. Each attribute value is converted via the
    /// appropriate helper so the resulting literal is well-typed.
    ///
    /// # Arguments
    ///
    /// - `&HashMap<String, String>` - Shared reference to a `HashMap<String, String>`.
    ///
    /// # Returns
    ///
    /// - `Vec<proc_macro2::TokenStream>` - A `Vec<proc_macro2::TokenStream>` value.
    fn prop_field_tokens(
        &self,
        props_field_types: &HashMap<String, String>,
    ) -> Vec<proc_macro2::TokenStream> {
        self.get_attributes()
            .iter()
            .map(|(key, value): &(proc_macro2::TokenStream, HtmlAttrValue)| {
                let key_string: String = extract_attr_key_string(key);
                let field_ident: Ident = Ident::new(&key_string, proc_macro2::Span::call_site());
                prop_field_token(&field_ident, &key_string, value, props_field_types)
            })
            .collect()
    }

    /// Emits the tokens for a native HTML element (e.g. `div { ... }`).
    ///
    /// Produces a `VirtualNode::Element { ... }` literal with the tag name,
    /// collected attribute entries, flattened children, and optional `key`.
    ///
    /// # Arguments
    ///
    /// - `&proc_macro2::TokenStream` - Shared reference to a `proc_macro2::TokenStream`.
    ///
    /// # Returns
    ///
    /// - `proc_macro2::TokenStream` - A `proc_macro2::TokenStream` value.
    fn native_element_tokens(
        &self,
        tag_literal: &proc_macro2::TokenStream,
    ) -> proc_macro2::TokenStream {
        let mut key_expr: Option<proc_macro2::TokenStream> = None;
        let attr_tokens: Vec<proc_macro2::TokenStream> = self
            .get_attributes()
            .iter()
            .filter_map(|(key, value): &(proc_macro2::TokenStream, HtmlAttrValue)| {
                let key_string: String = extract_attr_key_string(key);
                // OPT 2: emit `Cow::Borrowed("class")` for literal
                // attribute keys so the entire DOM tree shares a single
                // static-string slice per attribute name. The `Cow`
                // widening keeps the door open for runtime-built keys
                // (none currently exist in the framework, but the
                // `html!` macro is the only place that constructs
                // `AttributeEntry` so this is a safe extension point).
                let attr_name_token: proc_macro2::TokenStream =
                    quote! { ::std::borrow::Cow::Borrowed(#key_string) };
                if key_string == ATTR_KEY_KEY {
                    if let HtmlAttrValue::Expr(expr) = value {
                        key_expr = Some(quote! { Some((#expr).into()) });
                    }
                    return None;
                }
                let ctx: AttrEntryContext<'_> = AttrEntryContext::new(value, &key_string);
                let value_tokens: proc_macro2::TokenStream = attr_value_to_entry_value_tokens(&ctx);
                Some(quote! {
                    ::euv_core::AttributeEntry::new(#attr_name_token, #value_tokens)
                })
            })
            .collect();
        let key_token: proc_macro2::TokenStream = key_expr.unwrap_or_else(|| quote! { None });
        let children_tokens: proc_macro2::TokenStream =
            children_to_flattened_tokens(self.get_children());
        // OPT 2: tag literal becomes `Cow::Borrowed("div")` instead of
        // a fresh `String` allocation.
        quote! {
            ::euv_core::VirtualNode::Element {
                tag: ::euv_core::Tag::Element(::std::borrow::Cow::Borrowed(#tag_literal)),
                attributes: vec![#(#attr_tokens), *],
                children: #children_tokens,
                key: #key_token,
                props: None,
            }
        }
    }
}
