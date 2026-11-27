use super::*;

/// Expands `#[event]` on a struct, implementing `::euv::CustomEvent` plus `.emit()` and `::on(...)` helpers.
///
/// # Arguments
///
/// - `ItemStruct` - Annotated event payload struct.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - Original struct plus `CustomEvent` impl and ergonomic dispatch helpers.
pub(crate) fn expand_event_attr(item: ItemStruct) -> proc_macro2::TokenStream {
    let ident: &Ident = &item.ident;
    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();
    let has_clone: bool = struct_has_derive(&item.attrs, "Clone");
    let extra_derive: proc_macro2::TokenStream = if has_clone {
        quote! {}
    } else {
        quote! { #[derive(Clone)] }
    };
    quote! {
            #extra_derive
            #item

    /// Implements `impl #impl_generics ::euv::CustomEvent for #ident #ty_generics #where_clause {}`.
            impl #impl_generics ::euv::CustomEvent for #ident #ty_generics #where_clause {}

    /// Implements `impl #impl_generics #ident #ty_generics #where_clause`.
            impl #impl_generics #ident #ty_generics #where_clause {
                /// Emits this typed cross-component custom event to all active listeners on `EventBus`.
                #[cfg_attr(not(target_arch = "wasm32"), inline)]
                pub fn emit(self) {
                    ::euv::EventBus::emit(self);
                }

                /// Subscribes a scope-owned listener invoked whenever this custom event is emitted.
    ///
    /// # Arguments
    ///
    /// - `__EuvEvF` - A `__EuvEvF` parameter.
    ///
    /// # Returns
    ///
    /// - `::euv::Effect` - A `::euv::Effect` value.
                #[must_use]
                #[cfg_attr(not(target_arch = "wasm32"), inline)]
                pub fn on<__EuvEvF>(handler: __EuvEvF) -> ::euv::Effect
                where
                    __EuvEvF: FnMut(&Self) + 'static,
                {
                    ::euv::EventBus::on::<Self, __EuvEvF>(handler)
                }
            }
        }
}

/// Checks whether a struct's `#[derive(...)]` attributes already contain `trait_name`.
///
/// # Arguments
///
/// - `&[Attribute]` - Item attribute slice.
/// - `&str` - Trait identifier to probe.
///
/// # Returns
///
/// - `bool` - `true` when already derived.
pub(crate) fn struct_has_derive(attrs: &[Attribute], trait_name: &str) -> bool {
    for attr in attrs.iter() {
        if attr.path().is_ident("derive")
            && let Meta::List(list) = &attr.meta
        {
            let txt: String = list.tokens.to_string();
            if txt
                .split(|c: char| !c.is_alphanumeric() && c != '_')
                .any(|tok: &str| tok == trait_name)
            {
                return true;
            }
        }
    }
    false
}

/// Expands `emit!(event_expr)` into `::euv::EventBus::emit(event_expr)`.
///
/// # Arguments
///
/// - `Expr` - Event payload expression.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - Expanded event dispatch expression.
pub(crate) fn expand_emit(expr: Expr) -> proc_macro2::TokenStream {
    let span: Span = expr.span();
    quote_spanned! {span=>
        ::euv::EventBus::emit(#expr)
    }
}

/// Expands `on_event!(EventType => handler)` into `::euv::EventBus::on::<EventType, _>(handler)`.
///
/// # Arguments
///
/// - `OnEventInput` - Parsed event type and callback closure.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - Expanded scoped event subscription returning `::euv::Effect`.
pub(crate) fn expand_on_event(input: OnEventInput) -> proc_macro2::TokenStream {
    let ty: Type = input.event_type;
    let handler: Expr = input.handler;
    let span: Span = handler.span();
    quote_spanned! {span=>
        ::euv::EventBus::on::<#ty, _>(#handler)
    }
}
