use super::*;

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
    let EuvIconProps { name, label, size }: EuvIconProps = node.try_get_props().unwrap_or_default();
    let px: i32 = size.px();
    let style: String = format!("font-size: {px}px; width: {px}px; height: {px}px;");
    let style_ref: &str = style.as_str();
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
