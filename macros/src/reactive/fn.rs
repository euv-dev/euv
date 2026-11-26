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
    let callback: Expr = input.callback;
    let cb_span: Span = callback.span();
    if let Some(ty) = input.global_type {
        return quote_spanned! {cb_span=>
            ::euv::GlobalState::watch::<#ty, _>(#callback)
        };
    }
    if input.signals.is_empty() {
        return quote_spanned! {cb_span=>
            ::euv::Effect::new(#callback)
        };
    }
    let reads: Vec<proc_macro2::TokenStream> = input
        .signals
        .iter()
        .map(|sig: &Expr| quote_spanned! {sig.span()=> (#sig).get() })
        .collect();
    quote_spanned! {cb_span=> {
        let mut __euv_cb = #callback;
        ::euv::Effect::new(move || {
            __euv_cb(#(#reads),*);
        })
    }}
}

/// Expands `ComputedInput` into a span-preserving `::euv::Memo::new` call.
///
/// # Arguments
///
/// - `ComputedInput` - Parsed `computed!` input.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - Generated memo construction expression.
pub(crate) fn expand_computed(input: ComputedInput) -> proc_macro2::TokenStream {
    let callback: Expr = input.callback;
    let cb_span: Span = callback.span();
    if input.signals.is_empty() {
        return match input.return_type {
            Some(ret_ty) => quote_spanned! {cb_span=>
                ::euv::Memo::<#ret_ty>::new(#callback)
            },
            None => quote_spanned! {cb_span=>
                ::euv::Memo::new(#callback)
            },
        };
    }
    let reads: Vec<proc_macro2::TokenStream> = input
        .signals
        .iter()
        .map(|sig: &Expr| quote_spanned! {sig.span()=> (#sig).get() })
        .collect();
    match input.return_type {
        Some(ret_ty) => quote_spanned! {cb_span=> {
            let mut __euv_cb = #callback;
            ::euv::Memo::<#ret_ty>::new(move || -> #ret_ty {
                __euv_cb(#(#reads),*)
            })
        }},
        None => quote_spanned! {cb_span=> {
            let mut __euv_cb = #callback;
            ::euv::Memo::new(move || __euv_cb(#(#reads),*))
        }},
    }
}
