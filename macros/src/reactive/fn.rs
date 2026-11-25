use super::*;

/// Expands `WatchInput` into a `::euv::Effect::new` call with absolute paths.
///
/// # Arguments
///
/// - `WatchInput` - Parsed `watch!` input.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - Generated effect construction expression.
pub(crate) fn expand_watch(input: WatchInput) -> proc_macro2::TokenStream {
    let signals: Vec<Expr> = input.signals;
    let callback: Expr = input.callback;
    quote! {{
        let mut __euv_cb = #callback;
        ::euv::Effect::new(move || {
            __euv_cb(#((#signals).get()),*);
        })
    }}
}

/// Expands `ComputedInput` into a typed `::euv::Memo::<RetType>::new` call.
///
/// # Arguments
///
/// - `ComputedInput` - Parsed `computed!` input.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - Generated memo construction expression.
pub(crate) fn expand_computed(input: ComputedInput) -> proc_macro2::TokenStream {
    let signals: Vec<Expr> = input.signals;
    let ret_ty: Type = input.return_type;
    let callback: Expr = input.callback;
    quote! {{
        let mut __euv_cb = #callback;
        ::euv::Memo::<#ret_ty>::new(move || -> #ret_ty {
            __euv_cb(#((#signals).get()),*)
        })
    }}
}
