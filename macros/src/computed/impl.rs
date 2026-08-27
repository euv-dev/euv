use super::*;

/// Implementation of `Parse` for `ComputedInput`, parsing the `computed!` macro input.
///
/// Syntax: `computed!(signal1, signal2, ..., |param1: Type1, _, _: Type2, ...| -> RetType { body })`
///
/// The expressions before the closure are signal expressions.
/// The closure parameters correspond to `.get()` values of the respective signals.
/// Parameter types are optional and parsed after a colon if present.
/// Anonymous parameters use `_` (with or without a type annotation).
/// The return type after `->` specifies the type of the computed signal value.
impl Parse for ComputedInput {
    /// Parses the `computed!` macro input into a `ComputedInput` AST.
    ///
    /// # Arguments
    ///
    /// - `ParseStream` - The syn parse stream to read from.
    ///
    /// # Returns
    ///
    /// - `syn::Result<Self>` - The parsed `ComputedInput`, or a syntax error.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut signals: Vec<Expr> = Vec::new();
        while !input.peek(Token![|]) {
            let expr: Expr = input.parse()?;
            signals.push(expr);
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }
        input.parse::<Token![|]>()?;
        let mut param_names: Vec<Option<Ident>> = Vec::new();
        let mut param_types: Vec<Option<Type>> = Vec::new();
        while !input.peek(Token![|]) {
            let param_name: Option<Ident> = if input.peek(Token![_]) {
                input.parse::<Token![_]>()?;
                None
            } else {
                Some(input.parse::<Ident>()?)
            };
            param_names.push(param_name);
            let param_type: Option<Type> = if input.peek(Token![:]) {
                input.parse::<Token![:]>()?;
                Some(input.parse::<Type>()?)
            } else {
                None
            };
            param_types.push(param_type);
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }
        input.parse::<Token![|]>()?;
        input.parse::<Token![->]>()?;
        let return_type: Type = input.parse::<Type>()?;
        let body_content: ParseBuffer<'_>;
        braced!(body_content in input);
        // `Block::parse_within` returns the parsed
        // statements, allowing the trailing
        // expression-statement to omit the semicolon.
        // Using `Stmt::parse` directly here would reject
        // `x * 2` (no trailing `;`) with "unexpected end
        // of input, expected semicolon", forcing callers
        // into a `return x * 2;` workaround that still
        // hits a type-inference edge case in the
        // generated `use_signal` closure body.
        let body: Vec<Stmt> = body_content.call(Block::parse_within)?;
        if signals.len() != param_names.len() {
            return Err(input.error(ERR_SIGNAL_PARAM_MISMATCH));
        }
        Ok(Self {
            signals,
            param_names,
            param_types,
            return_type,
            body,
        })
    }
}

/// Implementation of `ToTokens` for `ComputedInput`, converting computed input into
/// a reactive signal derivation.
///
/// Generated code:
/// 1. Creates a result signal via `use_signal` with an initial value computed
///    from the closure body using the current signal values.
/// 2. Sets up a `watch!`-style subscription on all input signals.
/// 3. When any input signal changes, re-executes the closure body and updates
///    the result signal via `set()` to mark its dependents dirty precisely.
/// 4. Returns the result signal handle.
///
/// Uses the same `Box::leak` raw pointer pattern as `watch!` for fire closure
/// management, which is safe in single-threaded WASM contexts.
impl ToTokens for ComputedInput {
    /// Converts this computed input into a reactive computed signal token stream.
    ///
    /// # Arguments
    ///
    /// - `&mut proc_macro2::TokenStream` - The target token stream to append to.
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let signals: Vec<Ident> = (0..self.get_signals().len())
            .map(|signal_index: usize| {
                Ident::new(
                    &format!("{COMPUTED_SIGNAL_PREFIX}{signal_index}"),
                    Span::call_site(),
                )
            })
            .collect();
        let result_ident: Ident = Ident::new(COMPUTED_RESULT_PREFIX, Span::call_site());
        let signal_exprs: &Vec<Expr> = self.get_signals();
        let param_names: &Vec<Option<Ident>> = self.get_param_names();
        let param_types: &Vec<Option<Type>> = self.get_param_types();
        let return_type: &Type = self.get_return_type();
        let body: &Vec<Stmt> = self.get_body();
        let all_gets: Vec<proc_macro2::TokenStream> = signals
            .iter()
            .zip(param_names.iter())
            .zip(param_types.iter())
            .map(
                |((signal, param), param_type): ((&Ident, &Option<Ident>), &Option<Type>)| match (
                    param, param_type,
                ) {
                    (Some(name), Some(ty)) => quote! { let #name: #ty = #signal.get(); },
                    (Some(name), None) => quote! { let #name = #signal.get(); },
                    (None, Some(ty)) => quote! { let _: #ty = #signal.get(); },
                    (None, None) => quote! { let _ = #signal.get(); },
                },
            )
            .collect();
        let subscribe_calls: Vec<proc_macro2::TokenStream> = signals
            .iter()
            .map(|signal: &Ident| {
                quote! {
                    {
                        #signal.subscribe(move || {
                            ::euv_core::App::batch(|| {
                                unsafe { ::euv_core::FireHandle::fire_at(__euv_computed_fire_addr) }
                            });
                        });
                    }
                }
            })
            .collect();
        tokens.extend(quote! {{
                    #(let #signals: ::euv_core::Signal<_> = #signal_exprs;)*
                    let #result_ident: ::euv_core::Signal<#return_type> = ::euv_core::App::use_signal::<#return_type, _>(|| -> #return_type {
                        #(#all_gets)*
                        { #(#body)* }
                    });
            let __euv_computed_subscribed: ::euv_core::Signal<bool> = ::euv_core::App::use_signal(|| false);
            if !__euv_computed_subscribed.get() {
                let __euv_computed_fire_addr: usize = ::euv_core::FireHandle::new(move || {
                    #(#all_gets)*
                    #result_ident.set({ #(#body)* });
                })
                .into();
                ::euv_core::App::batch(|| {
                    #(#subscribe_calls)*
                    __euv_computed_subscribed.set(true);
                });
            }
            #result_ident
        }});
    }
}
