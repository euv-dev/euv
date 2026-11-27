use super::*;

/// Attempts to parse a named style function header without consuming tokens on failure.
///
/// # Arguments
///
/// - `ParseStream<'_>` - Token input stream to inspect.
///
/// # Returns
///
/// - `Result<bool>` - `Ok(true)` when the stream starts with a named style function header.
fn parse_named_style_header_probe(input: ParseStream<'_>) -> Result<bool> {
    let fork: ParseBuffer<'_> = input.fork();
    let _: Visibility = fork.parse()?;
    let _: Option<Token![const]> = fork.parse()?;
    let _: Ident = fork.parse()?;
    if fork.peek(Paren) {
        let param_buf: ParseBuffer<'_>;
        parenthesized!(param_buf in fork);
        let _: Punctuated<FnArg, Token![,]> =
            param_buf.parse_terminated(FnArg::parse, Token![,])?;
    }
    Ok(fork.peek(Brace))
}

/// Speculatively checks whether `input` begins with a named `vis [const] ident [(params)] { ... }` definition.
///
/// # Arguments
///
/// - `ParseStream<'_>` - Token input stream to inspect without consuming.
///
/// # Returns
///
/// - `bool` - `true` when `input` starts with a named style function definition.
pub(crate) fn is_named_style_def(input: ParseStream<'_>) -> bool {
    parse_named_style_header_probe(input).unwrap_or(false)
}

/// Parses zero or more newline-separated `StyleStmt` entries (`key: value` or `expr`), rejecting semicolons and multiple rules on the same line.
///
/// # Arguments
///
/// - `ParseStream<'_>` - Token stream inside a `style!` block or function body.
///
/// # Returns
///
/// - `Result<Vec<StyleStmt>>` - Parsed statements or compile error on `;` or same-line rules.
pub(crate) fn parse_style_stmts(body: ParseStream<'_>) -> Result<Vec<StyleStmt>> {
    let mut stmts: Vec<StyleStmt> = Vec::new();
    let mut prev_end_line: Option<usize> = None;
    while !body.is_empty() {
        if body.peek(Token![;]) {
            let semi: Token![;] = body.parse()?;
            return Err(Error::new(semi.span(), ERR_STYLE_UNEXPECTED_SEMI));
        }
        let (stmt, start_line, start_span, end_line): (StyleStmt, usize, Span, usize) =
            if body.peek(Ident) && body.peek2(Token![:]) && !body.peek2(Token![::]) {
                let key: Ident = body.parse()?;
                let kspan: Span = key.span();
                let _: Token![:] = body.parse()?;
                let value: Expr = body.parse()?;
                let s_line: usize = kspan.start().line;
                let e_line: usize = value.span().end().line;
                (StyleStmt::KeyValue { key, value }, s_line, kspan, e_line)
            } else {
                let expr: Expr = body.parse()?;
                let espan: Span = expr.span();
                let s_line: usize = espan.start().line;
                let e_line: usize = espan.end().line;
                (StyleStmt::PropExpr(expr), s_line, espan, e_line)
            };
        if let Some(prev_line) = prev_end_line
            && start_line <= prev_line
        {
            return Err(Error::new(start_span, ERR_STYLE_NEWLINE_REQUIRED));
        }
        if body.peek(Token![;]) {
            let semi: Token![;] = body.parse()?;
            return Err(Error::new(semi.span(), ERR_STYLE_UNEXPECTED_SEMI));
        }
        prev_end_line = Some(end_line);
        stmts.push(stmt);
    }
    Ok(stmts)
}

/// Builds the expanded `::euv::Style` expression for a sequence of `StyleStmt` entries.
///
/// # Arguments
///
/// - `&[StyleStmt]` - Ordered style statements.
/// - `&str` - Diagnostic context label (function name or `"inline"`).
///
/// # Returns
///
/// - `Result<proc_macro2::TokenStream>` - Expanded zero-allocation `Style` builder expression.
fn expand_style_body(stmts: &[StyleStmt], scope_label: &str) -> Result<proc_macro2::TokenStream> {
    let mut seen: HashSet<String> = HashSet::new();
    let mut ops: Vec<proc_macro2::TokenStream> = Vec::with_capacity(stmts.len());
    let mut needs_mut_binding: bool = false;
    for stmt in stmts.iter() {
        match stmt {
            StyleStmt::PropExpr(expr) => {
                let espan: Span = expr.span();
                ops.push(quote_spanned! {espan=>
                    __s = ::euv::StyleApply::apply_to(#expr, __s);
                });
                needs_mut_binding = true;
            }
            StyleStmt::KeyValue { key, value } => {
                let key_str: String = key.to_string();
                let kspan: Span = key.span();
                if !seen.insert(key_str.clone()) {
                    let msg: String =
                        format!("{ERR_STYLE_DUPLICATE_KEY} `{key_str}` in style! `{scope_label}`");
                    return Err(Error::new(kspan, msg));
                }
                let prop_op: proc_macro2::TokenStream = match key_str.as_str() {
                    STYLE_KEY_DIRECTION => {
                        let dir_str: String = quote!(#value).to_string();
                        let trimmed: &str = dir_str.trim();
                        if trimmed == STYLE_VAL_ROW {
                            quote! { __s = __s.row(); }
                        } else if trimmed == STYLE_VAL_COLUMN {
                            quote! { __s = __s.column(); }
                        } else {
                            quote! { __s = __s.direction(#value); }
                        }
                    }
                    STYLE_KEY_DISPLAY => quote! { __s = __s.display(#value); },
                    STYLE_KEY_POSITION => quote! { __s = __s.position(#value); },
                    STYLE_KEY_INSET => {
                        needs_mut_binding = true;
                        quote_spanned! {value.span()=>
                            let (__it, __ir, __ib, __il): (
                                ::core::option::Option<f32>,
                                ::core::option::Option<f32>,
                                ::core::option::Option<f32>,
                                ::core::option::Option<f32>,
                            ) = #value;
                            __s = __s.inset(__it, __ir, __ib, __il);
                        }
                    }
                    STYLE_KEY_WIDTH => quote! { __s = __s.width(#value); },
                    STYLE_KEY_HEIGHT => quote! { __s = __s.height(#value); },
                    STYLE_KEY_MIN_WIDTH => quote! { __s = __s.min_width(#value); },
                    STYLE_KEY_MIN_HEIGHT => quote! { __s = __s.min_height(#value); },
                    STYLE_KEY_MAX_WIDTH => quote! { __s = __s.max_width(#value); },
                    STYLE_KEY_MAX_HEIGHT => quote! { __s = __s.max_height(#value); },
                    STYLE_KEY_ASPECT_RATIO => quote! { __s = __s.aspect_ratio(#value); },
                    STYLE_KEY_PADDING => quote! { __s = __s.padding(#value); },
                    STYLE_KEY_MARGIN => quote! { __s = __s.margin(#value); },
                    STYLE_KEY_GAP => quote! { __s = __s.gap(#value); },
                    STYLE_KEY_BACKGROUND => quote! { __s = __s.background(#value); },
                    STYLE_KEY_GRADIENT => quote! { __s = __s.gradient(#value); },
                    STYLE_KEY_SHADOW => quote! { __s = __s.shadow(#value); },
                    STYLE_KEY_FOCUS_RING => quote! { __s = __s.focus_ring(#value); },
                    STYLE_KEY_CURSOR => quote! { __s = __s.cursor(#value); },
                    STYLE_KEY_POINTER_EVENTS => quote! { __s = __s.pointer_events(#value); },
                    STYLE_KEY_STOP_SCROLL_PROPAGATION => {
                        quote! { __s = __s.stop_scroll_propagation(#value); }
                    }
                    STYLE_KEY_STOP_EVENT_PROPAGATION => {
                        quote! { __s = __s.stop_event_propagation(#value); }
                    }
                    STYLE_KEY_COLOR => quote! { __s = __s.color(#value); },
                    STYLE_KEY_RADIUS => quote! { __s = __s.radius(#value); },
                    STYLE_KEY_ALIGN => quote! { __s = __s.align(#value); },
                    STYLE_KEY_JUSTIFY => quote! { __s = __s.justify(#value); },
                    STYLE_KEY_FLEX_GROW => quote! { __s = __s.flex_grow(#value); },
                    STYLE_KEY_FLEX_SHRINK => quote! { __s = __s.flex_shrink(#value); },
                    STYLE_KEY_FLEX_WRAP => quote! { __s = __s.wrap(#value); },
                    STYLE_KEY_OVERFLOW => quote! { __s = __s.overflow(#value); },
                    STYLE_KEY_FONT_SIZE => quote! { __s = __s.font_size(#value); },
                    STYLE_KEY_FONT_WEIGHT => quote! { __s = __s.font_weight(#value); },
                    STYLE_KEY_FONT_FAMILY => quote! { __s = __s.font_family(#value); },
                    STYLE_KEY_ITALIC => quote! { __s = __s.italic(#value); },
                    STYLE_KEY_TEXT_ALIGN => quote! { __s = __s.text_align(#value); },
                    STYLE_KEY_TEXT_OVERFLOW => quote! { __s = __s.text_overflow(#value); },
                    STYLE_KEY_OPACITY => quote! { __s = __s.opacity(#value); },
                    STYLE_KEY_Z_INDEX => quote! { __s = __s.z_index(#value); },
                    STYLE_KEY_GRID | STYLE_KEY_COLUMNS => quote! { __s = __s.grid(#value); },
                    STYLE_KEY_BORDER => {
                        needs_mut_binding = true;
                        quote_spanned! {value.span()=>
                            let (__bw, __bc): (f32, ::euv::Color) = #value;
                            __s = __s.border(__bw, __bc);
                        }
                    }
                    _ => {
                        let scope: String = format!("style! `{scope_label}`");
                        return Err(unknown_prop_error(key, &scope, KNOWN_STYLE_PROPS));
                    }
                };
                ops.push(quote! {
                    const _: fn() = ::euv::style_props::#key;
                    #prop_op
                });
            }
        }
    }
    let _: bool = needs_mut_binding;
    if ops.is_empty() {
        Ok(quote! { ::euv::Style::default() })
    } else {
        Ok(quote! {
            {
                let mut __s: ::euv::Style = ::euv::Style::default();
                #(#ops)*
                __s
            }
        })
    }
}

/// Expands `StyleBlockInput` into either named `Style` constructor functions or an inline `Style` expression.
///
/// # Arguments
///
/// - `StyleBlockInput` - Parsed style definitions or inline statements.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - Generated Rust tokens.
pub(crate) fn expand_style(input: StyleBlockInput) -> proc_macro2::TokenStream {
    match input {
        StyleBlockInput::Inline(stmts) => match expand_style_body(&stmts, "inline") {
            Ok(ts) => ts,
            Err(err) => err.to_compile_error(),
        },
        StyleBlockInput::Defs(defs) => {
            let mut fns: Vec<proc_macro2::TokenStream> = Vec::with_capacity(defs.len());
            for def in defs.iter() {
                let vis: &Visibility = &def.vis;
                let name: &Ident = &def.name;
                let params: &[FnArg] = def.params.as_slice();
                let name_str: String = name.to_string();
                let body: proc_macro2::TokenStream = match expand_style_body(&def.stmts, &name_str)
                {
                    Ok(ts) => ts,
                    Err(err) => return err.to_compile_error(),
                };
                fns.push(quote_spanned! {name.span()=>
                    #[must_use]
                    #vis fn #name(#(#params),*) -> ::euv::Style {
                        #body
                    }
                });
            }
            quote! { #(#fns)* }
        }
    }
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
