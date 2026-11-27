use super::*;

/// Attempts to inline a direct `Expr::Closure` callback for `watch!` or `computed!` into one outer `move ||` closure.
///
/// # Arguments
///
/// - `&[Expr]` - Tracked signal expressions.
/// - `&Expr` - Candidate callback expression.
/// - `Option<&Type>` - Optional explicit return type (`Some` for `computed!`, `None` for `watch!`).
/// - `bool` - `true` when discarding the body's value (`watch!`), `false` when returning it (`computed!`).
///
/// # Returns
///
/// - `Option<proc_macro2::TokenStream>` - Single-closure expansion when `callback` can be safely inlined.
fn try_inline_direct_closure(
    signals: &[Expr],
    callback: &Expr,
    explicit_ret: Option<&Type>,
    discard_result: bool,
) -> Option<proc_macro2::TokenStream> {
    let Expr::Closure(closure) = callback else {
        return None;
    };
    if !closure.attrs.is_empty()
        || closure.lifetimes.is_some()
        || closure.asyncness.is_some()
        || closure.inputs.len() != signals.len()
    {
        return None;
    }
    if discard_result {
        let is_unit_ret: bool = match &closure.output {
            ReturnType::Default => true,
            ReturnType::Type(_, ret_ty) => matches!(
                ret_ty.as_ref(),
                Type::Tuple(tup) if tup.elems.is_empty()
            ),
        };
        if !is_unit_ret {
            return None;
        }
    }
    let reads: Vec<proc_macro2::TokenStream> = signals
        .iter()
        .map(|sig: &Expr| quote_spanned! {sig.span()=> (#sig).get() })
        .collect();
    let bindings: proc_macro2::TokenStream = if signals.len() == 1 {
        let pat: &Pat = &closure.inputs[0];
        let read: &proc_macro2::TokenStream = &reads[0];
        quote! { let #pat = #read; }
    } else {
        let tmp_idents: Vec<Ident> = (0..signals.len())
            .map(|idx: usize| Ident::new(&format!("__euv_arg_{idx}"), Span::call_site()))
            .collect();
        let pats: Vec<&Pat> = closure.inputs.iter().collect();
        quote! {
            let (#(#tmp_idents),*) = (#(#reads),*);
            #(let #pats = #tmp_idents;)*
        }
    };
    let body: &Expr = closure.body.as_ref();
    let body_tokens: proc_macro2::TokenStream = match body {
        Expr::Block(expr_block) if expr_block.attrs.is_empty() && expr_block.label.is_none() => {
            let stmts: &[Stmt] = &expr_block.block.stmts;
            if discard_result {
                if let Some(Stmt::Expr(last_expr, None)) = stmts.last() {
                    let prefix: &[Stmt] = &stmts[..stmts.len() - 1];
                    quote! {
                        #(#prefix)*
                        #last_expr;
                    }
                } else {
                    quote! { #(#stmts)* }
                }
            } else {
                quote! { #(#stmts)* }
            }
        }
        other if discard_result => quote! { #other; },
        other => quote! { #other },
    };
    let cb_span: Span = callback.span();
    match explicit_ret {
        Some(ret_ty) => Some(quote_spanned! {cb_span=>
            move || -> #ret_ty {
                #bindings
                #body_tokens
            }
        }),
        None => match &closure.output {
            ReturnType::Default => Some(quote_spanned! {cb_span=>
                move || {
                    #bindings
                    #body_tokens
                }
            }),
            ReturnType::Type(_, ret_ty) => Some(quote_spanned! {cb_span=>
                move || -> #ret_ty {
                    #bindings
                    #body_tokens
                }
            }),
        },
    }
}

/// Returns whether `expr` is a qualified function path (`module::handler`) that can be invoked directly without a `mut` local binding.
///
/// # Arguments
///
/// - `&Expr` - Candidate callback expression.
///
/// # Returns
///
/// - `bool` - `true` when `expr` is a multi-segment path without attributes.
fn is_qualified_fn_path(expr: &Expr) -> bool {
    let Expr::Path(expr_path) = expr else {
        return false;
    };
    expr_path.attrs.is_empty() && expr_path.path.segments.len() >= 2
}

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
    if let Some(inlined) = try_inline_direct_closure(&input.signals, &callback, None, true) {
        return quote_spanned! {cb_span=>
            ::euv::Effect::new(#inlined)
        };
    }
    let reads: Vec<proc_macro2::TokenStream> = input
        .signals
        .iter()
        .map(|sig: &Expr| quote_spanned! {sig.span()=> (#sig).get() })
        .collect();
    if is_qualified_fn_path(&callback) {
        return quote_spanned! {cb_span=>
            ::euv::Effect::new(move || {
                (#callback)(#(#reads),*);
            })
        };
    }
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
    if let Some(inlined) =
        try_inline_direct_closure(&input.signals, &callback, input.return_type.as_ref(), false)
    {
        return match input.return_type {
            Some(ret_ty) => quote_spanned! {cb_span=>
                ::euv::Memo::<#ret_ty>::new(#inlined)
            },
            None => quote_spanned! {cb_span=>
                ::euv::Memo::new(#inlined)
            },
        };
    }
    let reads: Vec<proc_macro2::TokenStream> = input
        .signals
        .iter()
        .map(|sig: &Expr| quote_spanned! {sig.span()=> (#sig).get() })
        .collect();
    if is_qualified_fn_path(&callback) {
        return match input.return_type {
            Some(ret_ty) => quote_spanned! {cb_span=>
                ::euv::Memo::<#ret_ty>::new(move || -> #ret_ty {
                    (#callback)(#(#reads),*)
                })
            },
            None => quote_spanned! {cb_span=>
                ::euv::Memo::new(move || (#callback)(#(#reads),*))
            },
        };
    }
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
