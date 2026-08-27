use super::*;

/// Implementation of `Parse` for `VarsInput`, parsing the `vars!` macro input.
impl Parse for VarsInput {
    /// Parses the `vars!` macro input into a `VarsInput` AST.
    ///
    /// # Arguments
    ///
    /// - `ParseStream` - The syn parse stream to read from.
    ///
    /// # Returns
    ///
    /// - `syn::Result<Self>` - The parsed `VarsInput`, or a syntax error.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut defs: Vec<VarsDef> = Vec::new();
        while !input.is_empty() {
            let visibility: Visibility = input.parse()?;
            let name: Ident = input.parse()?;
            let params: Option<Vec<VarsParam>> = if input.peek(Paren) {
                let param_content: ParseBuffer<'_>;
                syn::parenthesized!(param_content in input);
                let mut param_list: Vec<VarsParam> = Vec::new();
                while !param_content.is_empty() {
                    let param_name: Ident = param_content.parse()?;
                    param_content.parse::<Token![:]>()?;
                    let param_type: Type = param_content.parse()?;
                    param_list.push(VarsParam {
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
            let content: ParseBuffer<'_>;
            braced!(content in input);
            let mut vars: Vec<(String, VarsValue)> = Vec::new();
            while !content.is_empty() {
                let var_name: String = parse_ident_name(&content)?;
                let css_key: String = format!("{CSS_CUSTOM_PROPERTY_PREFIX}{var_name}");
                content.parse::<Token![:]>()?;
                let var_value: VarsValue = {
                    let expr: Expr = content.parse()?;
                    let expanded: proc_macro2::TokenStream = expand_var_macros(&expr);
                    VarsValue::Expr(expanded)
                };
                vars.push((css_key, var_value));
                if content.peek(Semi) {
                    content.parse::<Semi>()?;
                }
            }
            defs.push(VarsDef {
                visibility,
                name,
                params,
                vars,
            });
        }
        Ok(Self { defs })
    }
}

/// Implementation of `ToTokens` for `VarsDef`, converting a vars block into `Css` function tokens.
///
/// Each CSS variable block becomes a `Css` function that, when called, injects
/// the CSS custom properties into the DOM and returns a reference to the class.
/// The CSS key names are prefixed with `--`.
impl ToTokens for VarsDef {
    /// Converts this CSS variable definition into token stream constructing a `Css`.
    ///
    /// # Arguments
    ///
    /// - `&mut proc_macro2::TokenStream` - The target token stream to append to.
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let visibility: &Visibility = self.get_visibility();
        let name: &Ident = self.get_name();
        let class_name_str: String = name.to_string();
        match self.try_get_params() {
            Some(params) => {
                let param_defs: Vec<proc_macro2::TokenStream> = params
                    .iter()
                    .map(|param: &VarsParam| {
                        let param_name: &Ident = param.get_name();
                        let param_type: &Type = param.get_param_type();
                        quote! { #param_name: #param_type }
                    })
                    .collect();
                let param_names: Vec<&Ident> = params
                    .iter()
                    .map(|param: &VarsParam| param.get_name())
                    .collect();
                let css_string_parts: Vec<proc_macro2::TokenStream> = self
                    .get_vars()
                    .iter()
                    .map(|(key, value): &(String, VarsValue)| match value {
                        VarsValue::Expr(expr) => {
                            if is_static_string_expr(expr) {
                                let value_str: String = expr_to_string(expr);
                                let prop_str: String =
                                    format!("{key}{CSS_PROP_SEPARATOR}{value_str}{CSS_DECL_TERMINATOR}");
                                quote! { #prop_str.to_string() }
                            } else {
                                let key_sep: String = format!("{key}{CSS_PROP_SEPARATOR}");
                                quote! { #key_sep.to_string() + &(#expr).to_string() + #CSS_DECL_TERMINATOR }
                            }
                        }
                    })
                    .collect();
                let name_format: String = format!("{{}}{STR_HYPHEN}{{}}");
                tokens.extend(quote! {
                        #visibility fn #name(#(#param_defs), *) -> ::euv_core::Css {
                        let css: ::euv_core::Css = ::euv_core::Css::new(format!(#name_format, #class_name_str, [#(format!("{:?}", #param_names)), *].join(#STR_HYPHEN)), [#(#css_string_parts), *].concat(), Vec::new(), Vec::new());
                        css.inject_style();
                        css
                    }
                });
            }
            None => {
                let name_span: Span = name.span();
                let const_name: Ident = Ident::new(&class_name_str.to_uppercase(), name.span());
                let const_name_token: proc_macro2::TokenStream =
                    quote_spanned!(name_span=> #const_name);
                let fn_name_token: proc_macro2::TokenStream = quote_spanned!(name_span=> #name);
                let all_static: bool =
                    self.get_vars()
                        .iter()
                        .all(|(_, value): &(String, VarsValue)| {
                            let VarsValue::Expr(expr) = value;
                            is_static_string_expr(expr)
                        });
                let style_expr: proc_macro2::TokenStream = if all_static {
                    let mut css_string: String = String::new();
                    for (key, value) in self.get_vars() {
                        let VarsValue::Expr(expr) = value;
                        css_string.push_str(key);
                        css_string.push_str(CSS_PROP_SEPARATOR);
                        css_string.push_str(&expr_to_string(expr));
                        css_string.push_str(CSS_DECL_TERMINATOR);
                    }
                    quote! { #css_string.to_string() }
                } else {
                    let css_string_parts: Vec<proc_macro2::TokenStream> = self
                        .get_vars()
                        .iter()
                        .map(|(key, value): &(String, VarsValue)| match value {
                            VarsValue::Expr(expr) => {
                                if is_static_string_expr(expr) {
                                    let value_str: String = expr_to_string(expr);
                                    let prop_str: String = format!("{key}{CSS_PROP_SEPARATOR}{value_str}{CSS_DECL_TERMINATOR}");
                                    quote! { #prop_str.to_string() }
                                } else {
                                    let key_sep: String = format!("{key}{CSS_PROP_SEPARATOR}");
                                    quote! { #key_sep.to_string() + &(#expr).to_string() + #CSS_DECL_TERMINATOR }
                                }
                            }
                        })
                        .collect();
                    quote! { [#(#css_string_parts), *].concat() }
                };
                emit_once_lock_fn(
                    tokens,
                    OnceLockParams {
                        visibility,
                        fn_name_token: &fn_name_token,
                        const_name_token: &const_name_token,
                        class_name_str: &class_name_str,
                        style_expr: &style_expr,
                        selector_expr: &quote! { Vec::new() },
                        at_rule_expr: &quote! { Vec::new() },
                    },
                );
            }
        }
    }
}

/// Implementation of `ToTokens` for `VarsInput`, converting vars definitions into token streams.
impl ToTokens for VarsInput {
    /// Converts all vars definitions into token streams.
    ///
    /// # Arguments
    ///
    /// - `&mut proc_macro2::TokenStream` - The target token stream to append to.
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.get_defs()
            .iter()
            .for_each(|vars_def: &VarsDef| vars_def.to_tokens(tokens));
    }
}
