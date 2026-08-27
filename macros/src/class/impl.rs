use super::*;

/// Implementation of `Parse` for `ClassInput`, parsing the `class!` macro input.
impl Parse for ClassInput {
    /// Parses the `class!` macro input into a `ClassInput` AST.
    ///
    /// Supports CSS-native syntax:
    /// - Pseudo-classes: `:hover { ... }`, `:focus-visible { ... }`, `:nth-child(2n) { ... }`
    /// - Pseudo-elements: `::before { ... }`, `::-webkit-scrollbar { ... }`
    /// - At-rules: `@media (max-width: 767px) { ... }`, `@keyframes fade { ... }`
    ///
    /// # Arguments
    ///
    /// - `ParseStream` - The syn parse stream to read from.
    ///
    /// # Returns
    ///
    /// - `syn::Result<Self>` - The parsed `ClassInput`, or a syntax error.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut classes: Vec<ClassDef> = Vec::new();
        while !input.is_empty() {
            let visibility: Visibility = input.parse()?;
            let name: Ident = input.parse()?;
            let mut generics: Generics = if input.peek(Token![<]) {
                input.parse()?
            } else {
                Generics::default()
            };
            let params: Option<Vec<ClassParam>> = if input.peek(Paren) {
                let param_content: ParseBuffer<'_>;
                parenthesized!(param_content in input);
                let mut param_list: Vec<ClassParam> = Vec::new();
                while !param_content.is_empty() {
                    let param_name: Ident = param_content.parse()?;
                    param_content.parse::<Token![:]>()?;
                    let param_type: Type = param_content.parse()?;
                    param_list.push(ClassParam {
                        name: param_name,
                        param_type,
                    });
                    if param_content.peek(Token![,]) {
                        param_content.parse::<Token![,]>()?;
                    }
                }
                if param_list.is_empty() {
                    None
                } else {
                    Some(param_list)
                }
            } else {
                None
            };
            if input.peek(Token![where]) {
                let where_clause: WhereClause = input.parse()?;
                generics.where_clause = Some(where_clause);
            }
            let content: ParseBuffer<'_>;
            braced!(content in input);
            let mut extends: Vec<ClassExtend> = Vec::new();
            let mut properties: Vec<(ClassPropKey, ClassPropValue)> = Vec::new();
            let mut selector_blocks: Vec<SelectorBlock> = Vec::new();
            let mut at_rule_blocks: Vec<AtRuleBlock> = Vec::new();
            while !content.is_empty() {
                if content.peek(Token![::]) {
                    content.parse::<Token![::]>()?;
                    let selector: String = parse_selector(&content, 2)?;
                    let block_content: ParseBuffer<'_>;
                    braced!(block_content in content);
                    let inner: BlockContent = parse_block_content(&block_content)?;
                    selector_blocks.push(SelectorBlock::new(
                        selector,
                        inner.properties,
                        inner.selector_blocks,
                    ));
                    continue;
                }
                if content.peek(Token![:]) && !content.peek2(Brace) {
                    content.parse::<Token![:]>()?;
                    let selector: String = parse_selector(&content, 1)?;
                    let block_content: ParseBuffer<'_>;
                    braced!(block_content in content);
                    let inner: BlockContent = parse_block_content(&block_content)?;
                    selector_blocks.push(SelectorBlock::new(
                        selector,
                        inner.properties,
                        inner.selector_blocks,
                    ));
                    continue;
                }
                if peek_at_rule(&content) {
                    let at_rule: AtRuleBlock = parse_at_rule(&content)?;
                    at_rule_blocks.push(at_rule);
                    continue;
                }
                if content.peek(Ident) {
                    let forked: ParseBuffer<'_> = content.fork();
                    let keyword: Ident = forked.parse()?;
                    let keyword_str: String = keyword.to_string();
                    let is_extends: bool =
                        forked.peek(Paren) && !keyword_str.starts_with(CHAR_AT) && {
                            let forked_extends_buffer: ParseBuffer<'_> = content.fork();
                            let _: Result<Ident, syn::Error> =
                                forked_extends_buffer.parse::<Ident>();
                            if forked_extends_buffer.peek(Paren) {
                                let _paren_content: ParseBuffer<'_>;
                                parenthesized!(_paren_content in forked_extends_buffer);
                                forked_extends_buffer.peek(Semi) || forked_extends_buffer.is_empty()
                            } else {
                                false
                            }
                        };
                    if is_extends {
                        content.parse::<Ident>()?;
                        let paren_content: ParseBuffer<'_>;
                        parenthesized!(paren_content in content);
                        let mut args: Vec<proc_macro2::TokenStream> = Vec::new();
                        while !paren_content.is_empty() {
                            let arg_tokens: proc_macro2::TokenStream = paren_content.parse()?;
                            args.push(arg_tokens);
                            if paren_content.peek(Token![,]) {
                                paren_content.parse::<Token![,]>()?;
                            } else {
                                break;
                            }
                        }
                        extends.push(ClassExtend {
                            name: keyword,
                            args,
                        });
                        if content.peek(Semi) {
                            content.parse::<Semi>()?;
                        }
                        continue;
                    }
                    // Check if this is an element selector block (e.g. `h1 { ... }`, `input, button { ... }`).
                    if is_element_selector_block(&content) {
                        let selector: String = parse_element_selector(&content)?;
                        let block_content: ParseBuffer<'_>;
                        braced!(block_content in content);
                        let inner: BlockContent = parse_block_content(&block_content)?;
                        selector_blocks.push(SelectorBlock::new(
                            selector,
                            inner.properties,
                            inner.selector_blocks,
                        ));
                        continue;
                    }
                }
                // Check for `*` or other element selector blocks not starting with Ident.
                if is_element_selector_block(&content) {
                    let selector: String = parse_element_selector(&content)?;
                    let block_content: ParseBuffer<'_>;
                    braced!(block_content in content);
                    let inner: BlockContent = parse_block_content(&block_content)?;
                    selector_blocks.push(SelectorBlock::new(
                        selector,
                        inner.properties,
                        inner.selector_blocks,
                    ));
                    continue;
                }
                let css_key: ClassPropKey = parse_class_prop_key(&content)?;
                content.parse::<Token![:]>()?;
                let expr: Expr = content.parse()?;
                let expanded: proc_macro2::TokenStream = expand_var_macros(&expr);
                let prop_value: ClassPropValue = ClassPropValue::Expr(expanded);
                properties.push((css_key, prop_value));
                if content.peek(Semi) {
                    content.parse::<Semi>()?;
                }
            }
            classes.push(ClassDef {
                visibility,
                name,
                generics,
                params,
                extends,
                properties,
                selector_blocks,
                at_rule_blocks,
            });
        }
        Ok(Self { classes })
    }
}

/// Implementation of `ToTokens` for `ClassDef`, converting a class definition into `Css` function tokens.
impl ToTokens for ClassDef {
    /// Converts this class definition into token stream constructing a `Css`.
    ///
    /// # Arguments
    ///
    /// - `&mut proc_macro2::TokenStream` - The target token stream to append to.
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let visibility: &Visibility = self.get_visibility();
        let name: &Ident = self.get_name();
        let generics: &Generics = self.get_generics();
        let class_name_str: String = name.to_string();
        let has_generics: bool = !generics.params.is_empty();
        let where_clause: Option<&WhereClause> = generics.where_clause.as_ref();
        let has_extra: bool =
            !self.get_selector_blocks().is_empty() || !self.get_at_rule_blocks().is_empty();
        let has_extends: bool = !self.get_extends().is_empty();
        let selector_expr: proc_macro2::TokenStream = if has_extends {
            let parent_pseudo_refs: Vec<proc_macro2::TokenStream> = self
                .get_extends()
                .iter()
                .map(|parent: &ClassExtend| {
                    let parent_name: &Ident = parent.get_name();
                    let parent_args: &Vec<proc_macro2::TokenStream> = parent.get_args();
                    if parent_args.is_empty() {
                        quote! { #parent_name().get_pseudo_rules().iter().cloned() }
                    } else {
                        quote! { #parent_name(#(#parent_args), *).get_pseudo_rules().iter().cloned() }
                    }
                })
                .collect();
            let self_pseudo: proc_macro2::TokenStream =
                selector_blocks_to_tokens(self.get_selector_blocks())
                    .unwrap_or_else(|| quote! { Vec::new() });
            quote! {
                {
                    let mut all_pseudo: Vec<::euv_core::PseudoRule> = Vec::new();
                    #(all_pseudo.extend(#parent_pseudo_refs);)*
                    all_pseudo.extend(#self_pseudo);
                    all_pseudo
                }
            }
        } else if !self.get_selector_blocks().is_empty() {
            selector_blocks_to_tokens(self.get_selector_blocks())
                .unwrap_or_else(|| quote! { Vec::new() })
        } else {
            quote! { Vec::new() }
        };
        let at_rule_expr: proc_macro2::TokenStream = if has_extends {
            let parent_media_refs: Vec<proc_macro2::TokenStream> = self
                .get_extends()
                .iter()
                .map(|parent: &ClassExtend| {
                    let parent_name: &Ident = parent.get_name();
                    let parent_args: &Vec<proc_macro2::TokenStream> = parent.get_args();
                    if parent_args.is_empty() {
                        quote! { #parent_name().get_media_rules().iter().cloned() }
                    } else {
                        quote! { #parent_name(#(#parent_args), *).get_media_rules().iter().cloned() }
                    }
                })
                .collect();
            let self_media: proc_macro2::TokenStream =
                at_rule_blocks_to_media_tokens(self.get_at_rule_blocks())
                    .unwrap_or_else(|| quote! { Vec::new() });
            quote! {
                {
                    let mut all_media: Vec<::euv_core::MediaRule> = Vec::new();
                    #(all_media.extend(#parent_media_refs);)*
                    all_media.extend(#self_media);
                    all_media
                }
            }
        } else if !self.get_at_rule_blocks().is_empty() {
            at_rule_blocks_to_media_tokens(self.get_at_rule_blocks())
                .unwrap_or_else(|| quote! { Vec::new() })
        } else {
            quote! { Vec::new() }
        };
        match self.try_get_params() {
            Some(params) => {
                let param_defs: Vec<proc_macro2::TokenStream> = params
                    .iter()
                    .map(|param: &ClassParam| {
                        let param_name: &Ident = param.get_name();
                        let param_type: &Type = param.get_param_type();
                        quote! { #param_name: #param_type }
                    })
                    .collect();
                let dynamic_param_names: Vec<String> = collect_dynamic_param_names(self);
                let param_name_parts: Vec<proc_macro2::TokenStream> = params
                    .iter()
                    .map(|param: &ClassParam| {
                        let param_name: &Ident = param.get_name();
                        if dynamic_param_names.contains(&param_name.to_string()) {
                            quote! { ::euv_core::Css::param_class_name(&(#param_name).to_string()) }
                        } else {
                            quote! { std::any::type_name_of_val(&#param_name).to_string() }
                        }
                    })
                    .collect();
                let mut all_css_parts: Vec<proc_macro2::TokenStream> = self
                    .get_extends()
                    .iter()
                    .map(|parent: &ClassExtend| {
                        let parent_name: &Ident = parent.get_name();
                        let parent_args: &Vec<proc_macro2::TokenStream> = parent.get_args();
                        if parent_args.is_empty() {
                            quote! { #parent_name().get_style().to_string() + #STR_SPACE }
                        } else {
                            quote! { #parent_name(#(#parent_args), *).get_style().to_string() + #STR_SPACE }
                        }
                    })
                    .collect();
                for (key, value) in self.get_properties() {
                    let ClassPropValue::Expr(expr) = value;
                    match key {
                        ClassPropKey::Static(static_key) => {
                            let key_str: String = reconstruct_ident_from_tokens(static_key);
                            if is_static_string_expr(expr) {
                                let value_str: String = expr_to_string(expr);
                                let prop_str: String = format!(
                                    "{key_str}{CSS_PROP_SEPARATOR}{value_str}{CSS_DECL_TERMINATOR}"
                                );
                                all_css_parts.push(quote! { #prop_str.to_string() });
                            } else {
                                let key_sep: String = format!("{key_str}{CSS_PROP_SEPARATOR}");
                                all_css_parts.push(
                                    quote! { #key_sep.to_string() + &(#expr).to_string() + #CSS_DECL_TERMINATOR },
                                );
                            }
                        }
                        ClassPropKey::Dynamic(_) => {
                            let key_token: proc_macro2::TokenStream = class_prop_key_to_tokens(key);
                            all_css_parts.push(quote! { #key_token + #CSS_PROP_SEPARATOR + &(#expr).to_string() + #CSS_DECL_TERMINATOR });
                        }
                    }
                }
                let unique_name_expr: proc_macro2::TokenStream = if param_name_parts.is_empty() {
                    quote! { #class_name_str.to_string() }
                } else {
                    let name_format: String = format!("{{}}{STR_HYPHEN}{{}}");
                    quote! { format!(#name_format, #class_name_str, [#(#param_name_parts), *].join(#STR_HYPHEN)) }
                };
                let style_expr: proc_macro2::TokenStream = if all_css_parts.is_empty() {
                    quote! { #STR_EMPTY.to_string() }
                } else {
                    quote! { [#(#all_css_parts), *].concat() }
                };
                tokens.extend(quote! {
                    #visibility fn #name #generics(#(#param_defs), *) -> ::euv_core::Css #where_clause {
                        ::euv_core::Css::new(#unique_name_expr, #style_expr, #selector_expr, #at_rule_expr)
                    }
                });
            }
            None => {
                let name_span: Span = name.span();
                let const_name: Ident = Ident::new(&class_name_str.to_uppercase(), name.span());
                let const_name_token: proc_macro2::TokenStream =
                    quote_spanned!(name_span=> #const_name);
                let fn_name_token: proc_macro2::TokenStream = quote_spanned!(name_span=> #name);
                if has_generics {
                    let mut all_css_parts: Vec<proc_macro2::TokenStream> = self
                        .get_extends()
                        .iter()
                        .map(|parent: &ClassExtend| {
                            let parent_name: &Ident = parent.get_name();
                            let parent_args: &Vec<proc_macro2::TokenStream> = parent.get_args();
                            if parent_args.is_empty() {
                                quote! { #parent_name().get_style().to_string() + #STR_SPACE }
                            } else {
                                quote! { #parent_name(#(#parent_args), *).get_style().to_string() + #STR_SPACE }
                            }
                        })
                        .collect();
                    for (key, value) in self.get_properties() {
                        let ClassPropValue::Expr(expr) = value;
                        match key {
                            ClassPropKey::Static(static_key) => {
                                let key_str: String = reconstruct_ident_from_tokens(static_key);
                                if is_static_string_expr(expr) {
                                    let value_str: String = expr_to_string(expr);
                                    let prop_str: String = format!(
                                        "{key_str}{CSS_PROP_SEPARATOR}{value_str}{CSS_DECL_TERMINATOR}"
                                    );
                                    all_css_parts.push(quote! { #prop_str.to_string() });
                                } else {
                                    let key_sep: String = format!("{key_str}{CSS_PROP_SEPARATOR}");
                                    all_css_parts.push(quote! { #key_sep.to_string() + &(#expr).to_string() + #CSS_DECL_TERMINATOR });
                                }
                            }
                            ClassPropKey::Dynamic(_) => {
                                let key_token: proc_macro2::TokenStream =
                                    class_prop_key_to_tokens(key);
                                all_css_parts.push(quote! { #key_token + #CSS_PROP_SEPARATOR + &(#expr).to_string() + #CSS_DECL_TERMINATOR });
                            }
                        }
                    }
                    let style_expr: proc_macro2::TokenStream = if all_css_parts.is_empty() {
                        quote! { #STR_EMPTY.to_string() }
                    } else {
                        quote! { [#(#all_css_parts), *].concat() }
                    };
                    tokens.extend(quote! {
                        #visibility fn #name #generics() -> ::euv_core::Css #where_clause {
                            let css: ::euv_core::Css = ::euv_core::Css::new(#class_name_str.to_string(), #style_expr, #selector_expr, #at_rule_expr);
                            css.inject_style();
                            css
                        }
                    });
                } else {
                    let all_static: bool = !has_extends
                        && self.get_properties().iter().all(
                            |(key, value): &(ClassPropKey, ClassPropValue)| {
                                let ClassPropKey::Static(_) = key else {
                                    return false;
                                };
                                let ClassPropValue::Expr(expr) = value;
                                is_static_string_expr(expr)
                            },
                        )
                        && is_selector_blocks_static(self.get_selector_blocks())
                        && is_at_rule_blocks_static(self.get_at_rule_blocks());
                    if all_static {
                        let mut css_string: String = String::new();
                        for (key, value) in self.get_properties() {
                            let ClassPropValue::Expr(expr) = value;
                            let ClassPropKey::Static(key_tokens) = key else {
                                continue;
                            };
                            let key_str: String = reconstruct_ident_from_tokens(key_tokens);
                            css_string.push_str(&key_str);
                            css_string.push_str(CSS_PROP_SEPARATOR);
                            css_string.push_str(&expr_to_string(expr));
                            css_string.push_str(CSS_DECL_TERMINATOR);
                        }
                        if has_extra {
                            let selector_static: String =
                                selector_blocks_to_static_string(self.get_selector_blocks());
                            let at_rule_static: String =
                                at_rule_blocks_to_static_string(self.get_at_rule_blocks());
                            emit_once_lock_fn(
                                tokens,
                                OnceLockParams {
                                    visibility,
                                    fn_name_token: &fn_name_token,
                                    const_name_token: &const_name_token,
                                    class_name_str: &class_name_str,
                                    style_expr: &quote! { #css_string.to_string() },
                                    selector_expr: &quote! { ::euv_core::Css::parse_pseudo_rules(#selector_static) },
                                    at_rule_expr: &quote! { ::euv_core::Css::parse_media_rules(#at_rule_static) },
                                },
                            );
                        } else {
                            emit_once_lock_fn(
                                tokens,
                                OnceLockParams {
                                    visibility,
                                    fn_name_token: &fn_name_token,
                                    const_name_token: &const_name_token,
                                    class_name_str: &class_name_str,
                                    style_expr: &quote! { #css_string.to_string() },
                                    selector_expr: &selector_expr,
                                    at_rule_expr: &at_rule_expr,
                                },
                            );
                        }
                    } else {
                        let mut all_css_parts: Vec<proc_macro2::TokenStream> = self
                            .get_extends()
                            .iter()
                            .map(|parent: &ClassExtend| {
                                let parent_name: &Ident = parent.get_name();
                                let parent_args: &Vec<proc_macro2::TokenStream> = parent.get_args();
                                if parent_args.is_empty() {
                                    quote! { #parent_name().get_style().to_string() + #STR_SPACE }
                                } else {
                                    quote! { #parent_name(#(#parent_args), *).get_style().to_string() + #STR_SPACE }
                                }
                            })
                            .collect();
                        for (key, value) in self.get_properties() {
                            let ClassPropValue::Expr(expr) = value;
                            match key {
                                ClassPropKey::Static(static_key) => {
                                    let key_str: String = reconstruct_ident_from_tokens(static_key);
                                    if is_static_string_expr(expr) {
                                        let value_str: String = expr_to_string(expr);
                                        let prop_str: String = format!(
                                            "{key_str}{CSS_PROP_SEPARATOR}{value_str}{CSS_DECL_TERMINATOR}"
                                        );
                                        all_css_parts.push(quote! { #prop_str.to_string() });
                                    } else {
                                        let key_sep: String =
                                            format!("{key_str}{CSS_PROP_SEPARATOR}");
                                        all_css_parts.push(quote! { #key_sep.to_string() + &(#expr).to_string() + #CSS_DECL_TERMINATOR });
                                    }
                                }
                                ClassPropKey::Dynamic(_) => {
                                    let key_token: proc_macro2::TokenStream =
                                        class_prop_key_to_tokens(key);
                                    all_css_parts.push(quote! { #key_token + #CSS_PROP_SEPARATOR + &(#expr).to_string() + #CSS_DECL_TERMINATOR });
                                }
                            }
                        }
                        let style_expr: proc_macro2::TokenStream = if all_css_parts.is_empty() {
                            quote! { #STR_EMPTY.to_string() }
                        } else {
                            quote! { [#(#all_css_parts), *].concat() }
                        };
                        emit_once_lock_fn(
                            tokens,
                            OnceLockParams {
                                visibility,
                                fn_name_token: &fn_name_token,
                                const_name_token: &const_name_token,
                                class_name_str: &class_name_str,
                                style_expr: &style_expr,
                                selector_expr: &selector_expr,
                                at_rule_expr: &at_rule_expr,
                            },
                        );
                    }
                }
            }
        }
    }
}

/// Implementation of `ToTokens` for `ClassInput`, converting class definitions into token streams.
impl ToTokens for ClassInput {
    /// Converts all class definitions into token streams.
    ///
    /// # Arguments
    ///
    /// - `&mut proc_macro2::TokenStream` - The target token stream to append to.
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.get_classes()
            .iter()
            .for_each(|class_def: &ClassDef| class_def.to_tokens(tokens));
    }
}
