use super::*;

/// Expands `StyleBlockInput` into span-preserving, zero-allocation `Style` builder functions.
///
/// # Arguments
///
/// - `StyleBlockInput` - Parsed style definitions.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - Generated Rust function items.
pub(crate) fn expand_style(input: StyleBlockInput) -> proc_macro2::TokenStream {
    let mut fns: Vec<proc_macro2::TokenStream> = Vec::with_capacity(input.defs.len());
    for def in input.defs.iter() {
        let vis: &Visibility = &def.vis;
        let name: &Ident = &def.name;
        let mut stmts: Vec<proc_macro2::TokenStream> = Vec::with_capacity(def.rules.len());
        for rule in def.rules.iter() {
            let key: String = rule.key.to_string();
            let val: &Expr = &rule.value;
            let kspan: Span = rule.key.span();
            match key.as_str() {
                STYLE_KEY_DIRECTION => {
                    let dir_str: String = quote!(#val).to_string();
                    let trimmed: &str = dir_str.trim();
                    if trimmed == STYLE_VAL_ROW {
                        stmts.push(quote_spanned! {kspan=> __s = __s.row(); });
                    } else if trimmed == STYLE_VAL_COLUMN {
                        stmts.push(quote_spanned! {kspan=> __s = __s.column(); });
                    } else {
                        let msg: String = format!(
                            "invalid direction `{trimmed}` in style! `{name}` — expected `row` or `column`"
                        );
                        return Error::new(val.span(), msg).to_compile_error();
                    }
                }
                STYLE_KEY_DISPLAY => stmts.push(quote_spanned! {kspan=> __s = __s.display(#val); }),
                STYLE_KEY_POSITION => {
                    stmts.push(quote_spanned! {kspan=> __s = __s.position(#val); });
                }
                STYLE_KEY_WIDTH => stmts.push(quote_spanned! {kspan=> __s = __s.width(#val); }),
                STYLE_KEY_HEIGHT => stmts.push(quote_spanned! {kspan=> __s = __s.height(#val); }),
                STYLE_KEY_MIN_WIDTH => {
                    stmts.push(quote_spanned! {kspan=> __s = __s.min_width(#val); });
                }
                STYLE_KEY_MIN_HEIGHT => {
                    stmts.push(quote_spanned! {kspan=> __s = __s.min_height(#val); });
                }
                STYLE_KEY_MAX_WIDTH => {
                    stmts.push(quote_spanned! {kspan=> __s = __s.max_width(#val); });
                }
                STYLE_KEY_MAX_HEIGHT => {
                    stmts.push(quote_spanned! {kspan=> __s = __s.max_height(#val); });
                }
                STYLE_KEY_ASPECT_RATIO => {
                    stmts.push(quote_spanned! {kspan=> __s = __s.aspect_ratio(#val); });
                }
                STYLE_KEY_PADDING => stmts.push(quote_spanned! {kspan=> __s = __s.padding(#val); }),
                STYLE_KEY_MARGIN => stmts.push(quote_spanned! {kspan=> __s = __s.margin(#val); }),
                STYLE_KEY_GAP => stmts.push(quote_spanned! {kspan=> __s = __s.gap(#val); }),
                STYLE_KEY_BACKGROUND => {
                    stmts.push(quote_spanned! {kspan=> __s = __s.background(#val); });
                }
                STYLE_KEY_GRADIENT => {
                    stmts.push(quote_spanned! {kspan=> __s = __s.gradient(#val); });
                }
                STYLE_KEY_SHADOW => stmts.push(quote_spanned! {kspan=> __s = __s.shadow(#val); }),
                STYLE_KEY_FOCUS_RING => {
                    stmts.push(quote_spanned! {kspan=> __s = __s.focus_ring(#val); });
                }
                STYLE_KEY_CURSOR => stmts.push(quote_spanned! {kspan=> __s = __s.cursor(#val); }),
                STYLE_KEY_COLOR => stmts.push(quote_spanned! {kspan=> __s = __s.color(#val); }),
                STYLE_KEY_RADIUS => stmts.push(quote_spanned! {kspan=> __s = __s.radius(#val); }),
                STYLE_KEY_ALIGN => stmts.push(quote_spanned! {kspan=> __s = __s.align(#val); }),
                STYLE_KEY_JUSTIFY => stmts.push(quote_spanned! {kspan=> __s = __s.justify(#val); }),
                STYLE_KEY_FLEX_GROW => {
                    stmts.push(quote_spanned! {kspan=> __s = __s.flex_grow(#val); });
                }
                STYLE_KEY_FLEX_SHRINK => {
                    stmts.push(quote_spanned! {kspan=> __s = __s.flex_shrink(#val); });
                }
                STYLE_KEY_FLEX_WRAP => {
                    stmts.push(quote_spanned! {kspan=> __s = __s.wrap(#val); });
                }
                STYLE_KEY_OVERFLOW => {
                    stmts.push(quote_spanned! {kspan=> __s = __s.overflow(#val); });
                }
                STYLE_KEY_FONT_SIZE => {
                    stmts.push(quote_spanned! {kspan=> __s = __s.font_size(#val); });
                }
                STYLE_KEY_FONT_WEIGHT => {
                    stmts.push(quote_spanned! {kspan=> __s = __s.font_weight(#val); });
                }
                STYLE_KEY_FONT_FAMILY => {
                    stmts.push(quote_spanned! {kspan=> __s = __s.font_family(#val); });
                }
                STYLE_KEY_TEXT_ALIGN => {
                    stmts.push(quote_spanned! {kspan=> __s = __s.text_align(#val); });
                }
                STYLE_KEY_TEXT_OVERFLOW => {
                    stmts.push(quote_spanned! {kspan=> __s = __s.text_overflow(#val); });
                }
                STYLE_KEY_OPACITY => stmts.push(quote_spanned! {kspan=> __s = __s.opacity(#val); }),
                STYLE_KEY_Z_INDEX => stmts.push(quote_spanned! {kspan=> __s = __s.z_index(#val); }),
                STYLE_KEY_GRID | STYLE_KEY_COLUMNS => {
                    stmts.push(quote_spanned! {kspan=> __s = __s.grid(#val); });
                }
                STYLE_KEY_BORDER => {
                    stmts.push(quote_spanned! {val.span()=>
                        let (__bw, __bc): (f32, ::euv::Color) = #val;
                        __s = __s.border(__bw, __bc);
                    });
                }
                _ => {
                    let scope: String = format!("style! `{name}`");
                    return unknown_prop_error(&rule.key, &scope, KNOWN_STYLE_PROPS)
                        .to_compile_error();
                }
            }
        }
        fns.push(quote_spanned! {name.span()=>
            #[must_use]
            #vis fn #name() -> ::euv::Style {
                let mut __s: ::euv::Style = ::euv::Style::empty();
                #(#stmts)*
                __s
            }
        });
    }
    quote! { #(#fns)* }
}

/// Expands `TokenBlockInput` into span-preserving typed `const` items.
///
/// # Arguments
///
/// - `TokenBlockInput` - Parsed design token entries.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - Generated constant declarations.
pub(crate) fn expand_tokens(input: TokenBlockInput) -> proc_macro2::TokenStream {
    let items: Vec<proc_macro2::TokenStream> = input
        .entries
        .iter()
        .map(|entry: &TokenEntry| {
            let vis: &Visibility = &entry.vis;
            let name: &Ident = &entry.name;
            let ty: &Type = &entry.ty;
            let value: &Expr = &entry.value;
            quote_spanned! {name.span()=>
                #vis const #name: #ty = #value;
            }
        })
        .collect();
    quote! { #(#items)* }
}
