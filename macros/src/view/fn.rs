use super::*;

/// Expands a parsed `ViewMacroInput` into zero-VDOM arena node construction tokens.
///
/// # Arguments
///
/// - `ViewMacroInput` - Parsed `view!` macro AST.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - Generated Rust expression evaluating to `::euv::NodeId`.
pub(crate) fn expand_view(input: ViewMacroInput) -> proc_macro2::TokenStream {
    let body: proc_macro2::TokenStream = expand_node(&input.root);
    quote! {{
        let mut __euv_ctx_owned: ::euv::BuildContext = ::euv::current_build_context();
        let __euv_ctx: &mut ::euv::BuildContext = &mut __euv_ctx_owned;
        #body
    }}
}

/// Returns a helpful native migration error if `tag` is a legacy Web/HTML element name.
///
/// # Arguments
///
/// - `&Ident` - Element tag identifier.
///
/// # Returns
///
/// - `Option<proc_macro2::TokenStream>` - Compile error tokens if `tag` is an HTML tag, else `None`.
fn legacy_html_tag_error(tag: &Ident) -> Option<proc_macro2::TokenStream> {
    let name: String = tag.to_string();
    let replacement: &str = match name.as_str() {
        "div" | "section" | "main" | "header" | "footer" | "nav" | "article" | "aside" | "ul"
        | "ol" | "li" | "form" => "`column`, `row`, `view`, or `grid`",
        "span" | "p" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "label" | "strong" | "code" => {
            "`text`"
        }
        "button" | "a" => "`{ EuvButton::new(...) }` or `view { on_click: ... }`",
        "img" => "`image { source: ... }`",
        _ => return None,
    };
    let msg: String =
        format!("HTML tag `{name}` is not part of native euv `view!` — use {replacement} instead");
    Some(Error::new(tag.span(), msg).to_compile_error())
}

/// Expands a single `ViewNode` into a span-preserving `::euv::NodeId` expression.
///
/// # Arguments
///
/// - `&ViewNode` - Node AST to expand.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - Expanded node expression.
fn expand_node(node: &ViewNode) -> proc_macro2::TokenStream {
    match node {
        ViewNode::StaticText(lit) => {
            quote_spanned! {lit.span()=>
                __euv_ctx.text(::std::string::String::from(#lit), ::euv::Style::empty())
            }
        }
        ViewNode::Expr(expr) => match expr {
            Expr::Closure(_) => {
                quote_spanned! {expr.span()=>
                    __euv_ctx.reactive_text(::euv::Style::empty(), #expr)
                }
            }
            other => {
                quote_spanned! {other.span()=>
                    ::euv::Mountable::mount(#other, __euv_ctx)
                }
            }
        },
        ViewNode::Element(elem) => expand_element(elem),
    }
}

/// Expands a `ViewElement` into span-preserving `BuildContext` and `Style` method calls.
///
/// # Arguments
///
/// - `&ViewElement` - Element AST to expand.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - Expanded block returning `::euv::NodeId`.
fn expand_element(elem: &ViewElement) -> proc_macro2::TokenStream {
    if let Some(html_err) = legacy_html_tag_error(&elem.tag) {
        return html_err;
    }
    let tag_name: String = elem.tag.to_string();
    let mut base_style: proc_macro2::TokenStream = quote! { ::euv::Style::empty() };
    let mut style_mutations: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut on_click: Option<&Expr> = None;
    let mut on_change: Option<&Expr> = None;
    let mut on_scroll: Option<&Expr> = None;
    let mut a11y_role: Option<&Expr> = None;
    let mut a11y_label: Option<&Expr> = None;
    let mut a11y_hint: Option<&Expr> = None;
    let mut input_value: Option<&Expr> = None;
    let mut input_placeholder: Option<&Expr> = None;
    let mut image_source: Option<&Expr> = None;
    let mut canvas_painter: Option<&Expr> = None;
    let mut slot_id: Option<&Expr> = None;
    let mut slot_route: Option<&Expr> = None;
    let mut slot_hub: Option<&Expr> = None;
    let mut slot_fallback: Option<&Expr> = None;
    for prop in elem.props.iter() {
        let pname: String = prop.name.to_string();
        let val: &Expr = &prop.value;
        let pspan: Span = prop.name.span();
        match pname.as_str() {
            PROP_STYLE => {
                base_style = quote_spanned! {val.span()=> #val };
            }
            PROP_WIDTH => {
                style_mutations
                    .push(quote_spanned! {pspan=> __euv_style = __euv_style.width(#val); });
            }
            PROP_HEIGHT => {
                style_mutations
                    .push(quote_spanned! {pspan=> __euv_style = __euv_style.height(#val); });
            }
            PROP_PADDING => {
                style_mutations
                    .push(quote_spanned! {pspan=> __euv_style = __euv_style.padding(#val); });
            }
            PROP_MARGIN => {
                style_mutations
                    .push(quote_spanned! {pspan=> __euv_style = __euv_style.margin(#val); });
            }
            PROP_GAP => {
                style_mutations
                    .push(quote_spanned! {pspan=> __euv_style = __euv_style.gap(#val); });
            }
            PROP_BACKGROUND => {
                style_mutations
                    .push(quote_spanned! {pspan=> __euv_style = __euv_style.background(#val); });
            }
            PROP_COLOR => {
                style_mutations
                    .push(quote_spanned! {pspan=> __euv_style = __euv_style.color(#val); });
            }
            PROP_RADIUS => {
                style_mutations
                    .push(quote_spanned! {pspan=> __euv_style = __euv_style.radius(#val); });
            }
            PROP_BORDER => {
                style_mutations.push(quote_spanned! {val.span()=>
                    let (__euv_bw, __euv_bc): (f32, ::euv::Color) = #val;
                    __euv_style = __euv_style.border(__euv_bw, __euv_bc);
                });
            }
            PROP_ALIGN => {
                style_mutations
                    .push(quote_spanned! {pspan=> __euv_style = __euv_style.align(#val); });
            }
            PROP_JUSTIFY => {
                style_mutations
                    .push(quote_spanned! {pspan=> __euv_style = __euv_style.justify(#val); });
            }
            PROP_FLEX_GROW => {
                style_mutations
                    .push(quote_spanned! {pspan=> __euv_style = __euv_style.flex_grow(#val); });
            }
            PROP_OVERFLOW => {
                style_mutations
                    .push(quote_spanned! {pspan=> __euv_style = __euv_style.overflow(#val); });
            }
            PROP_FONT_SIZE => {
                style_mutations
                    .push(quote_spanned! {pspan=> __euv_style = __euv_style.font_size(#val); });
            }
            PROP_FONT_WEIGHT => {
                style_mutations
                    .push(quote_spanned! {pspan=> __euv_style = __euv_style.font_weight(#val); });
            }
            PROP_Z_INDEX => {
                style_mutations
                    .push(quote_spanned! {pspan=> __euv_style = __euv_style.z_index(#val); });
            }
            PROP_OPACITY => {
                style_mutations
                    .push(quote_spanned! {pspan=> __euv_style = __euv_style.opacity(#val); });
            }
            PROP_COLUMNS => {
                style_mutations
                    .push(quote_spanned! {pspan=> __euv_style = __euv_style.grid(#val); });
            }
            PROP_ON_CLICK => {
                on_click = Some(val);
            }
            PROP_ON_CHANGE => {
                on_change = Some(val);
            }
            PROP_ON_SCROLL => {
                on_scroll = Some(val);
            }
            PROP_VALUE => {
                if tag_name != TAG_INPUT {
                    return Error::new(pspan, ERR_INPUT_PROP_ONLY_ON_INPUT).to_compile_error();
                }
                input_value = Some(val);
            }
            PROP_PLACEHOLDER => {
                if tag_name != TAG_INPUT {
                    return Error::new(pspan, ERR_INPUT_PROP_ONLY_ON_INPUT).to_compile_error();
                }
                input_placeholder = Some(val);
            }
            PROP_SOURCE => {
                if tag_name != TAG_IMAGE {
                    return Error::new(pspan, ERR_SOURCE_ONLY_ON_IMAGE).to_compile_error();
                }
                image_source = Some(val);
            }
            PROP_PAINTER => {
                if tag_name != TAG_CANVAS {
                    return Error::new(pspan, ERR_PAINTER_ONLY_ON_CANVAS).to_compile_error();
                }
                canvas_painter = Some(val);
            }
            PROP_ROLE => {
                a11y_role = Some(val);
            }
            PROP_A11Y_LABEL => {
                a11y_label = Some(val);
            }
            PROP_A11Y_HINT => {
                a11y_hint = Some(val);
            }
            PROP_ID => {
                if tag_name != TAG_SLOT {
                    return Error::new(pspan, ERR_ID_ONLY_ON_SLOT).to_compile_error();
                }
                slot_id = Some(val);
            }
            PROP_ROUTE => {
                if tag_name != TAG_PAGE_SLOT {
                    return Error::new(pspan, ERR_ROUTE_ONLY_ON_PAGE_SLOT).to_compile_error();
                }
                slot_route = Some(val);
            }
            PROP_HUB => {
                if tag_name != TAG_SLOT && tag_name != TAG_PAGE_SLOT {
                    return Error::new(pspan, ERR_HUB_ONLY_ON_DYNAMIC_SLOT).to_compile_error();
                }
                slot_hub = Some(val);
            }
            PROP_FALLBACK => {
                if tag_name != TAG_SLOT && tag_name != TAG_PAGE_SLOT {
                    return Error::new(pspan, ERR_HUB_ONLY_ON_DYNAMIC_SLOT).to_compile_error();
                }
                slot_fallback = Some(val);
            }
            _ => {
                let scope: String = format!("view! element `{tag_name}`");
                return unknown_prop_error(&prop.name, &scope, KNOWN_VIEW_PROPS).to_compile_error();
            }
        }
    }
    let tag_span: Span = elem.tag.span();
    let direction_setup: proc_macro2::TokenStream = match tag_name.as_str() {
        TAG_ROW => quote_spanned! {tag_span=> __euv_style = __euv_style.row(); },
        TAG_COLUMN | TAG_COL => quote_spanned! {tag_span=> __euv_style = __euv_style.column(); },
        TAG_GRID => quote_spanned! {tag_span=> __euv_style = __euv_style.grid(2); },
        _ => quote! {},
    };
    let node_creation: proc_macro2::TokenStream = match tag_name.as_str() {
        TAG_VIEW | TAG_ROW | TAG_COLUMN | TAG_COL | TAG_GRID => {
            quote_spanned! {tag_span=> __euv_ctx.view(__euv_style) }
        }
        TAG_SCROLL => {
            quote_spanned! {tag_span=> __euv_ctx.scroll_view(__euv_style) }
        }
        TAG_PORTAL => {
            quote_spanned! {tag_span=> __euv_ctx.portal(__euv_style) }
        }
        TAG_TEXT => {
            if let Some(first) = elem.children.first() {
                match first {
                    ViewNode::StaticText(lit) => {
                        quote_spanned! {lit.span()=> __euv_ctx.text(::std::string::String::from(#lit), __euv_style) }
                    }
                    ViewNode::Expr(Expr::Closure(closure)) => {
                        quote_spanned! {closure.span()=> __euv_ctx.reactive_text(__euv_style, #closure) }
                    }
                    ViewNode::Expr(expr) => {
                        quote! { __euv_ctx.text(::std::string::String::from(#expr), __euv_style) }
                    }
                    ViewNode::Element(_) => {
                        quote_spanned! {tag_span=> __euv_ctx.text(::std::string::String::new(), __euv_style) }
                    }
                }
            } else {
                quote_spanned! {tag_span=> __euv_ctx.text(::std::string::String::new(), __euv_style) }
            }
        }
        TAG_INPUT => {
            let val_expr: proc_macro2::TokenStream = match input_value {
                Some(v) => quote! { ::std::string::String::from(#v) },
                None => quote! { ::std::string::String::new() },
            };
            let ph_expr: proc_macro2::TokenStream = match input_placeholder {
                Some(p) => quote! { ::std::string::String::from(#p) },
                None => quote! { ::std::string::String::new() },
            };
            quote_spanned! {tag_span=>
                __euv_ctx.input(
                    ::euv::InputNodeState::new(#val_expr, #ph_expr, 0, false, false, false),
                    __euv_style,
                )
            }
        }
        TAG_IMAGE => {
            let Some(src) = image_source else {
                return Error::new(tag_span, ERR_IMAGE_MISSING_SOURCE).to_compile_error();
            };
            quote_spanned! {src.span()=> __euv_ctx.image(#src, __euv_style) }
        }
        TAG_CANVAS => {
            let Some(painter) = canvas_painter else {
                return Error::new(tag_span, ERR_CANVAS_MISSING_PAINTER).to_compile_error();
            };
            quote_spanned! {painter.span()=> __euv_ctx.canvas(__euv_style, #painter) }
        }
        TAG_SLOT => {
            let Some(id_expr) = slot_id else {
                return Error::new(tag_span, ERR_SLOT_MISSING_ID).to_compile_error();
            };
            let fb_expr: proc_macro2::TokenStream =
                build_slot_fallback(slot_fallback, &elem.children);
            let hub_stmt: proc_macro2::TokenStream = match slot_hub {
                Some(h) => {
                    quote_spanned! {h.span()=> __euv_slot.set_hub(::core::option::Option::Some(#h)); }
                }
                None => quote! {},
            };
            quote_spanned! {tag_span=> {
                let mut __euv_slot: ::euv::DynamicComponentSlot = ::euv::DynamicComponentSlot::new(
                    ::std::string::String::from(#id_expr),
                    #fb_expr,
                );
                __euv_slot.set_style(__euv_style);
                #hub_stmt
                ::euv::Mountable::mount(__euv_slot, __euv_ctx)
            }}
        }
        TAG_PAGE_SLOT => {
            let Some(route_expr) = slot_route else {
                return Error::new(tag_span, ERR_PAGE_SLOT_MISSING_ROUTE).to_compile_error();
            };
            let fb_expr: proc_macro2::TokenStream =
                build_slot_fallback(slot_fallback, &elem.children);
            let hub_stmt: proc_macro2::TokenStream = match slot_hub {
                Some(h) => {
                    quote_spanned! {h.span()=> __euv_slot.set_hub(::core::option::Option::Some(#h)); }
                }
                None => quote! {},
            };
            quote_spanned! {tag_span=> {
                let mut __euv_slot: ::euv::DynamicPageSlot = ::euv::DynamicPageSlot::new(
                    ::std::string::String::from(#route_expr),
                    #fb_expr,
                );
                __euv_slot.set_style(__euv_style);
                #hub_stmt
                ::euv::Mountable::mount(__euv_slot, __euv_ctx)
            }}
        }
        _ => {
            let comp_ident: &Ident = &elem.tag;
            quote_spanned! {tag_span=> #comp_ident() }
        }
    };
    let click_stmt: proc_macro2::TokenStream = match on_click {
        Some(cb) => quote_spanned! {cb.span()=> __euv_ctx.on_click(__euv_node, #cb); },
        None => quote! {},
    };
    let change_stmt: proc_macro2::TokenStream = match on_change {
        Some(cb) => quote_spanned! {cb.span()=> __euv_ctx.on_change(__euv_node, #cb); },
        None => quote! {},
    };
    let scroll_stmt: proc_macro2::TokenStream = match on_scroll {
        Some(cb) => quote_spanned! {cb.span()=> __euv_ctx.on_scroll(__euv_node, #cb); },
        None => quote! {},
    };
    let a11y_stmt: proc_macro2::TokenStream = if a11y_role.is_some()
        || a11y_label.is_some()
        || a11y_hint.is_some()
    {
        let role_expr: proc_macro2::TokenStream = match a11y_role {
            Some(r) => quote_spanned! {r.span()=> #r },
            None => quote! { ::euv::A11yRole::Group },
        };
        let label_expr: proc_macro2::TokenStream = match a11y_label {
            Some(l) => quote! { ::std::string::String::from(#l) },
            None => quote! { ::std::string::String::new() },
        };
        let hint_stmt: proc_macro2::TokenStream = match a11y_hint {
            Some(h) => {
                quote! { __euv_a11y.set_hint(::std::string::String::from(#h)); }
            }
            None => quote! {},
        };
        quote! {
            let mut __euv_a11y: ::euv::A11yProps = ::euv::A11yProps::new(#role_expr, #label_expr);
            #hint_stmt
            __euv_ctx.set_a11y(__euv_node, __euv_a11y);
        }
    } else {
        quote! {}
    };
    let child_stmts: Vec<proc_macro2::TokenStream> =
        if tag_name == TAG_TEXT || tag_name == TAG_SLOT || tag_name == TAG_PAGE_SLOT {
            Vec::new()
        } else {
            elem.children
                .iter()
                .map(|child: &ViewNode| {
                    let child_expr: proc_macro2::TokenStream = expand_node(child);
                    quote! {
                        let __euv_child: ::euv::NodeId = #child_expr;
                        __euv_ctx.append(__euv_node, __euv_child);
                    }
                })
                .collect()
        };
    quote! {{
        let mut __euv_style: ::euv::Style = #base_style;
        #direction_setup
        #(#style_mutations)*
        let __euv_node: ::euv::NodeId = #node_creation;
        #click_stmt
        #change_stmt
        #scroll_stmt
        #a11y_stmt
        #(#child_stmts)*
        __euv_node
    }}
}

/// Builds the `::euv::DynamicBuilderFn` fallback closure for `slot` and `page_slot` elements.
///
/// # Arguments
///
/// - `Option<&Expr>` - Optional explicit `fallback:` closure expression.
/// - `&[ViewNode]` - Inline fallback children declared inside the slot block.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - Expanded `Rc::new(...)` builder closure.
fn build_slot_fallback(
    explicit_fallback: Option<&Expr>,
    children: &[ViewNode],
) -> proc_macro2::TokenStream {
    if let Some(fb) = explicit_fallback {
        return quote_spanned! {fb.span()=> ::std::rc::Rc::new(#fb) };
    }
    if children.is_empty() {
        return quote! {
            ::std::rc::Rc::new(move || -> ::euv::NodeId {
                let mut __euv_ctx_owned: ::euv::BuildContext = ::euv::current_build_context();
                __euv_ctx_owned.view(::euv::Style::empty())
            })
        };
    }
    if children.len() == 1 {
        let single: proc_macro2::TokenStream = expand_node(&children[0]);
        return quote! {
            ::std::rc::Rc::new(move || -> ::euv::NodeId {
                let mut __euv_ctx_owned: ::euv::BuildContext = ::euv::current_build_context();
                let __euv_ctx: &mut ::euv::BuildContext = &mut __euv_ctx_owned;
                #single
            })
        };
    }
    let child_stmts: Vec<proc_macro2::TokenStream> = children
        .iter()
        .map(|child: &ViewNode| {
            let child_expr: proc_macro2::TokenStream = expand_node(child);
            quote! {
                let __euv_child: ::euv::NodeId = #child_expr;
                __euv_ctx.append(__euv_fb_root, __euv_child);
            }
        })
        .collect();
    quote! {
        ::std::rc::Rc::new(move || -> ::euv::NodeId {
            let mut __euv_ctx_owned: ::euv::BuildContext = ::euv::current_build_context();
            let __euv_ctx: &mut ::euv::BuildContext = &mut __euv_ctx_owned;
            let __euv_fb_root: ::euv::NodeId =
                __euv_ctx.view(::euv::Style::empty().column().width(::euv::Length::Percent(100.0)));
            #(#child_stmts)*
            __euv_fb_root
        })
    }
}
