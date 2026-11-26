use super::*;

/// Expands `#[global_state]` on a struct, generating ergonomic global singleton registration, read, write, and watch helpers.
///
/// # Arguments
///
/// - `ItemStruct` - Annotated global state struct.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - Original struct plus global state methods.
pub(crate) fn expand_global_state_attr(item: ItemStruct) -> proc_macro2::TokenStream {
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

    /// Implements `impl #impl_generics #ident #ty_generics #where_clause`.
            impl #impl_generics #ident #ty_generics #where_clause {
                /// Registers or replaces this value in `GlobalState`.
    ///
    /// # Returns
    ///
    /// - `::euv::Signal<Self>` - A `::euv::Signal<Self>` value.
                pub fn provide(self) -> ::euv::Signal<Self> {
                    ::euv::GlobalState::provide(self)
                }

                /// Looks up the registered global `Signal<Self>` if present.
    ///
    /// # Returns
    ///
    /// - `::core::option::Option<::euv::Signal<Self>>` - A `::core::option::Option<::euv::Signal<Self>>` value.
                #[must_use]
                pub fn use_global() -> ::core::option::Option<::euv::Signal<Self>> {
                    ::euv::GlobalState::use_state::<Self>()
                }

                /// Reactively reads a clone of this global state if registered.
    ///
    /// # Returns
    ///
    /// - `::core::option::Option<Self>` - A `::core::option::Option<Self>` value.
                #[must_use]
                pub fn get_global() -> ::core::option::Option<Self> {
                    ::euv::GlobalState::get::<Self>()
                }

                /// Replaces the registered global state value and notifies active subscribers.
    ///
    /// # Arguments
    ///
    /// - `Self` - A `Self` parameter.
    ///
    /// # Returns
    ///
    /// - `bool` - A boolean.
                pub fn set_global(value: Self) -> bool {
                    ::euv::GlobalState::set(value)
                }

                /// Mutates the registered global state value in place via `updater`.
    ///
    /// # Arguments
    ///
    /// - `__EuvStF` - A `__EuvStF` parameter.
    ///
    /// # Returns
    ///
    /// - `bool` - A boolean.
                pub fn update_global<__EuvStF>(updater: __EuvStF) -> bool
                where
                    __EuvStF: FnOnce(&mut Self),
                {
                    ::euv::GlobalState::update::<Self, __EuvStF>(updater)
                }

                /// Subscribes a scope-owned watcher receiving `(&prev, &next)` whenever this global state changes.
    ///
    /// # Arguments
    ///
    /// - `__EuvStW` - A `__EuvStW` parameter.
    ///
    /// # Returns
    ///
    /// - `::core::option::Option<::euv::Effect>` - A `::core::option::Option<::euv::Effect>` value.
                #[must_use]
                pub fn watch_global<__EuvStW>(handler: __EuvStW) -> ::core::option::Option<::euv::Effect>
                where
                    __EuvStW: FnMut(&Self, &Self) + 'static,
                {
                    ::euv::GlobalState::watch::<Self, __EuvStW>(handler)
                }
            }
        }
}

/// Expands `local_state!(init)` into `::euv::LocalState::new(init)`.
///
/// # Arguments
///
/// - `Expr` - Initial value expression.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - Expanded local state creation expression.
pub(crate) fn expand_local_state(expr: Expr) -> proc_macro2::TokenStream {
    let span: Span = expr.span();
    quote_spanned! {span=>
        ::euv::LocalState::new(#expr)
    }
}

/// Expands `global_state!(Type)` or `global_state!(Type = init)` into `GlobalState` calls.
///
/// # Arguments
///
/// - `GlobalStateInput` - Parsed global state form.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - Expanded `GlobalState` expression.
pub(crate) fn expand_global_state(input: GlobalStateInput) -> proc_macro2::TokenStream {
    match input {
        GlobalStateInput::Lookup(ty) => {
            let span: Span = ty.span();
            quote_spanned! {span=>
                ::euv::GlobalState::use_state::<#ty>()
            }
        }
        GlobalStateInput::Init { ty, init } => {
            let span: Span = init.span();
            quote_spanned! {span=>
                match ::euv::GlobalState::use_state::<#ty>() {
                    ::core::option::Option::Some(__euv_sig) => __euv_sig,
                    ::core::option::Option::None => ::euv::GlobalState::provide::<#ty>(#init),
                }
            }
        }
    }
}
