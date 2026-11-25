use super::*;

/// Expands `WatchInput` into a span-preserving `::euv::Effect::new` call.
///
/// # Arguments
///
/// - `WatchInput` - Parsed `watch!` input.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - Generated effect construction expression.
pub(crate) fn expand_watch(input: WatchInput) -> proc_macro2::TokenStream {
    let reads: Vec<proc_macro2::TokenStream> = input
        .signals
        .iter()
        .map(|sig: &Expr| quote_spanned! {sig.span()=> (#sig).get() })
        .collect();
    let callback: Expr = input.callback;
    let cb_span: Span = callback.span();
    quote_spanned! {cb_span=> {
        let mut __euv_cb = #callback;
        ::euv::Effect::new(move || {
            __euv_cb(#(#reads),*);
        })
    }}
}

/// Expands `ComputedInput` into a span-preserving typed `::euv::Memo::<RetType>::new` call.
///
/// # Arguments
///
/// - `ComputedInput` - Parsed `computed!` input.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - Generated memo construction expression.
pub(crate) fn expand_computed(input: ComputedInput) -> proc_macro2::TokenStream {
    let reads: Vec<proc_macro2::TokenStream> = input
        .signals
        .iter()
        .map(|sig: &Expr| quote_spanned! {sig.span()=> (#sig).get() })
        .collect();
    let ret_ty: Type = input.return_type;
    let callback: Expr = input.callback;
    let cb_span: Span = callback.span();
    quote_spanned! {cb_span=> {
        let mut __euv_cb = #callback;
        ::euv::Memo::<#ret_ty>::new(move || -> #ret_ty {
            __euv_cb(#(#reads),*)
        })
    }}
}
