use super::*;

/// Builds the vector element and its accessible drawing attributes.
///
/// # Arguments
///
/// - `&str` - The inner glyph markup.
/// - `i32` - The square edge of the glyph in pixels.
///
/// # Returns
///
/// - `String` - The vector element markup.
pub(crate) fn svg_glyph_markup(markup: &str, px: i32) -> String {
    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{px}\" height=\"{px}\" \
         viewBox=\"{SVG_GLYPH_VIEW_BOX}\" fill=\"none\" stroke=\"currentColor\" \
         stroke-width=\"{SVG_GLYPH_STROKE_WIDTH}\" stroke-linecap=\"round\" \
         stroke-linejoin=\"round\" aria-hidden=\"true\" focusable=\"false\">{markup}</svg>"
    )
}

/// A monochrome glyph wrapper rendering an inline emoji or text symbol.
///
/// Renders a `<span>` carrying the base `c_euv_icon` class plus an inline
/// style pinning `font-size`, `width`, and `height` to the size in pixels so
/// the glyph occupies a stable box regardless of the surrounding type scale.
/// When `label` is non-empty a `c_euv_icon_label` span is rendered next to the
/// glyph.
///
/// # Arguments
///
/// - `VirtualNode<EuvIconProps>` - The props node carrying the component configuration.
///
/// # Returns
///
/// - `VirtualNode` - The component virtual DOM tree.
#[component]
pub fn euv_icon(node: VirtualNode<EuvIconProps>) -> VirtualNode {
    let EuvIconProps {
        name,
        label,
        size,
        svg,
    }: EuvIconProps = node.try_get_props().unwrap_or_default();
    let px: i32 = size.px();
    let style: String = format!("font-size: {px}px; width: {px}px; height: {px}px;");
    let style_ref: &str = style.as_str();
    match svg {
        Some(markup) => {
            let drawing: String = svg_glyph_markup(markup, px);
            if label.is_empty() {
                html! {
                    span {
                        class: c_euv_icon()
                        style: style_ref
                        inner_html: drawing
                    }
                }
            } else {
                html! {
                    span {
                        class: c_euv_icon()
                        style: style_ref
                        inner_html: drawing
                        span {
                            class: c_euv_icon_label()
                            label
                        }
                    }
                }
            }
        }
        None => {
            if label.is_empty() {
                html! {
                    span {
                        class: c_euv_icon()
                        style: style_ref
                        name
                    }
                }
            } else {
                html! {
                    span {
                        class: c_euv_icon()
                        style: style_ref
                        name
                        span {
                            class: c_euv_icon_label()
                            label
                        }
                    }
                }
            }
        }
    }
}
