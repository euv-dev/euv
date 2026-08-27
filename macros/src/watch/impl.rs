use super::*;

/// Implementation of `Parse` for `WatchInput`, parsing the `watch!` macro input.
///
/// Syntax: `watch!(signal1, signal2, ..., |param1: Type1, _, _: Type2, ...| { body })`
///
/// The expressions before the closure are signal expressions.
/// The closure parameters correspond to `.get()` values of the respective signals.
/// Parameter types are optional and parsed after a colon if present.
/// Anonymous parameters use `_` (with or without a type annotation).
impl Parse for WatchInput {
    /// Parses the `watch!` macro input into a `WatchInput` AST.
    ///
    /// # Arguments
    ///
    /// - `ParseStream` - The syn parse stream to read from.
    ///
    /// # Returns
    ///
    /// - `syn::Result<Self>` - The parsed `WatchInput`, or a syntax error.
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
        let body_content: ParseBuffer<'_>;
        braced!(body_content in input);
        let mut body: Vec<Stmt> = Vec::new();
        while !body_content.is_empty() {
            let stmt: Stmt = body_content.parse()?;
            body.push(stmt);
        }
        if signals.len() != param_names.len() {
            return Err(input.error(ERR_SIGNAL_PARAM_MISMATCH));
        }
        Ok(Self {
            signals,
            param_names,
            param_types,
            body,
        })
    }
}

/// Implementation of `ToTokens` for `WatchInput`, converting watch input into reactive subscription code.
///
/// Generated code:
/// 1. Uses a `use_signal(|| false)` guard to ensure subscriptions and
///    initial body execution only happen once per DynamicNode lifecycle,
///    preventing duplicate subscriptions and infinite re-render loops.
/// 2. Clones each signal into a local binding.
/// 3. On first execution, the entire initialisation (subscribe registration
///    and body execution) is wrapped in `batch` so that
///    any `set()` calls inside the body mark their dependents dirty
///    precisely but do not trigger premature microtask dispatches.
/// 4. Subsequent render_fn invocations skip the block entirely — the body
///    only fires via the `subscribe` callbacks when a watched signal
///    actually changes.
///
/// Uses `Box::leak` raw pointer pattern instead of `Rc<RefCell<>>` to
/// avoid interior mutability. The fire closure is double-boxed
/// (`Box<Box<dyn FnMut()>>`) so that the outer `Box` is sized and has a
/// thin pointer that can be safely cast to `usize`. The address is captured
/// in each subscribe callback and cast back for invocation. This is safe in
/// single-threaded WASM contexts and eliminates `RefCell` borrow conflicts
/// that occur when watch callbacks trigger cascading signal updates.
impl ToTokens for WatchInput {
    /// Converts this watch input into reactive subscription token stream.
    ///
    /// # Arguments
    ///
    /// - `&mut proc_macro2::TokenStream` - The target token stream to append to.
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let signals: Vec<Ident> = (0..self.get_signals().len())
            .map(|signal_index: usize| {
                Ident::new(
                    &format!("{WATCH_SIGNAL_PREFIX}{signal_index}"),
                    Span::call_site(),
                )
            })
            .collect();
        let signal_exprs: &Vec<Expr> = self.get_signals();
        let param_names: &Vec<Option<Ident>> = self.get_param_names();
        let param_types: &Vec<Option<Type>> = self.get_param_types();
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
                                unsafe { ::euv_core::FireHandle::fire_at(__euv_watch_fire_addr) }
                            });
                        });
                    }
                }
            })
            .collect();
        tokens.extend(quote! {{
            #(let #signals: ::euv_core::Signal<_> = #signal_exprs;)*
            let __euv_watch_subscribed: ::euv_core::Signal<bool> = ::euv_core::App::use_signal(|| false);
            if !__euv_watch_subscribed.get() {
                let __euv_watch_fire_addr: usize = ::euv_core::FireHandle::new(move || {
                    #(#all_gets)*
                    { #(#body)* }
                })
                .into();
                ::euv_core::App::batch(|| {
                    #(#subscribe_calls)*
                    {
                        #(#all_gets)*
                        { #(#body)* }
                    }
                    __euv_watch_subscribed.set(true);
                });
            }
        }});
    }
}
