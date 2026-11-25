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

/// Expands a single `ViewNode` into a `::euv::NodeId` expression.
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
            quote! {
                __euv_ctx.text(::std::string::String::from(#lit), ::euv::Style::empty())
            }
        }
        ViewNode::Expr(expr) => match expr {
            Expr::Closure(_) => {
                quote! {
                    __euv_ctx.reactive_text(::euv::Style::empty(), #expr)
                }
            }
            other => {
                quote! {
                    ::euv::Mountable::mount(#other, __euv_ctx)
                }
            }
        },
        ViewNode::Element(elem) => expand_element(elem),
    }
}

/// Expands a `ViewElement` into direct `BuildContext` calls with zero `use` statements.
///
/// # Arguments
///
/// - `&ViewElement` - Element AST to expand.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - Expanded block returning `::euv::NodeId`.
fn expand_element(elem: &ViewElement) -> proc_macro2::TokenStream {
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
    for prop in elem.props.iter() {
        let pname: String = prop.name.to_string();
        let val: &Expr = &prop.value;
        match pname.as_str() {
            PROP_STYLE => {
                base_style = quote! { (#val) };
            }
            PROP_WIDTH => {
                style_mutations.push(quote! { __euv_style = __euv_style.width(#val); });
            }
            PROP_HEIGHT => {
                style_mutations.push(quote! { __euv_style = __euv_style.height(#val); });
            }
            PROP_PADDING => {
                style_mutations.push(quote! { __euv_style = __euv_style.padding(#val); });
            }
            PROP_MARGIN => {
                style_mutations.push(quote! { __euv_style = __euv_style.margin(#val); });
            }
            PROP_GAP => {
                style_mutations.push(quote! { __euv_style = __euv_style.gap(#val); });
            }
            PROP_BACKGROUND => {
                style_mutations.push(quote! { __euv_style = __euv_style.background(#val); });
            }
            PROP_COLOR => {
                style_mutations.push(quote! { __euv_style = __euv_style.color(#val); });
            }
            PROP_RADIUS => {
                style_mutations.push(quote! { __euv_style = __euv_style.radius(#val); });
            }
            PROP_BORDER => {
                style_mutations.push(quote! {
                    let (__euv_bw, __euv_bc): (f32, ::euv::Color) = #val;
                    __euv_style = __euv_style.border(__euv_bw, __euv_bc);
                });
            }
            PROP_ALIGN => {
                style_mutations.push(quote! { __euv_style = __euv_style.align(#val); });
            }
            PROP_JUSTIFY => {
                style_mutations.push(quote! { __euv_style = __euv_style.justify(#val); });
            }
            PROP_FLEX_GROW => {
                style_mutations.push(quote! { __euv_style = __euv_style.flex_grow(#val); });
            }
            PROP_OVERFLOW => {
                style_mutations.push(quote! { __euv_style = __euv_style.overflow(#val); });
            }
            PROP_FONT_SIZE => {
                style_mutations.push(quote! { __euv_style = __euv_style.font_size(#val); });
            }
            PROP_FONT_WEIGHT => {
                style_mutations.push(quote! { __euv_style = __euv_style.font_weight(#val); });
            }
            PROP_Z_INDEX => {
                style_mutations.push(quote! { __euv_style = __euv_style.z_index(#val); });
            }
            PROP_OPACITY => {
                style_mutations.push(quote! { __euv_style = __euv_style.opacity(#val); });
            }
            PROP_COLUMNS => {
                style_mutations.push(quote! { __euv_style = __euv_style.grid(#val); });
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
                input_value = Some(val);
            }
            PROP_PLACEHOLDER => {
                input_placeholder = Some(val);
            }
            PROP_SOURCE => {
                image_source = Some(val);
            }
            PROP_PAINTER => {
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
            _ => {
                let err: proc_macro2::TokenStream =
                    Error::new(prop.name.span(), ERR_UNKNOWN_VIEW_PROP).to_compile_error();
                return err;
            }
        }
    }
    let direction_setup: proc_macro2::TokenStream = match tag_name.as_str() {
        TAG_ROW => quote! { __euv_style = __euv_style.row(); },
        TAG_COLUMN | TAG_COL => quote! { __euv_style = __euv_style.column(); },
        TAG_GRID => quote! { __euv_style = __euv_style.grid(2); },
        _ => quote! {},
    };
    let node_creation: proc_macro2::TokenStream = match tag_name.as_str() {
        TAG_VIEW | TAG_ROW | TAG_COLUMN | TAG_COL | TAG_GRID => {
            quote! { __euv_ctx.view(__euv_style) }
        }
        TAG_SCROLL => {
            quote! { __euv_ctx.scroll_view(__euv_style) }
        }
        TAG_PORTAL => {
            quote! { __euv_ctx.portal(__euv_style) }
        }
        TAG_TEXT => {
            if let Some(first) = elem.children.first() {
                match first {
                    ViewNode::StaticText(lit) => {
                        quote! { __euv_ctx.text(::std::string::String::from(#lit), __euv_style) }
                    }
                    ViewNode::Expr(Expr::Closure(closure)) => {
                        quote! { __euv_ctx.reactive_text(__euv_style, #closure) }
                    }
                    ViewNode::Expr(expr) => {
                        quote! { __euv_ctx.text(::std::string::String::from(#expr), __euv_style) }
                    }
                    ViewNode::Element(_) => {
                        quote! { __euv_ctx.text(::std::string::String::new(), __euv_style) }
                    }
                }
            } else {
                quote! { __euv_ctx.text(::std::string::String::new(), __euv_style) }
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
            quote! {
                __euv_ctx.input(
                    ::euv::InputNodeState::new(#val_expr, #ph_expr, 0, false, false, false),
                    __euv_style,
                )
            }
        }
        TAG_IMAGE => {
            let Some(src) = image_source else {
                return Error::new(elem.tag.span(), ERR_IMAGE_MISSING_SOURCE).to_compile_error();
            };
            quote! { __euv_ctx.image(#src, __euv_style) }
        }
        TAG_CANVAS => {
            let Some(painter) = canvas_painter else {
                return Error::new(elem.tag.span(), ERR_CANVAS_MISSING_PAINTER).to_compile_error();
            };
            quote! { __euv_ctx.canvas(__euv_style, #painter) }
        }
        _ => {
            let comp_ident: &Ident = &elem.tag;
            quote! { #comp_ident() }
        }
    };
    let click_stmt: proc_macro2::TokenStream = match on_click {
        Some(cb) => quote! { __euv_ctx.on_click(__euv_node, #cb); },
        None => quote! {},
    };
    let change_stmt: proc_macro2::TokenStream = match on_change {
        Some(cb) => quote! { __euv_ctx.on_change(__euv_node, #cb); },
        None => quote! {},
    };
    let scroll_stmt: proc_macro2::TokenStream = match on_scroll {
        Some(cb) => quote! { __euv_ctx.on_scroll(__euv_node, #cb); },
        None => quote! {},
    };
    let a11y_stmt: proc_macro2::TokenStream = if a11y_role.is_some()
        || a11y_label.is_some()
        || a11y_hint.is_some()
    {
        let role_expr: proc_macro2::TokenStream = match a11y_role {
            Some(r) => quote! { #r },
            None => quote! { ::euv::A11yRole::Group },
        };
        let label_expr: proc_macro2::TokenStream = match a11y_label {
            Some(l) => quote! { ::std::string::String::from(#l) },
            None => quote! { ::std::string::String::new() },
        };
        let hint_stmt: proc_macro2::TokenStream = match a11y_hint {
            Some(h) => quote! { __euv_a11y.set_hint(::std::string::String::from(#h)); },
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
    let child_stmts: Vec<proc_macro2::TokenStream> = if tag_name == TAG_TEXT {
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
