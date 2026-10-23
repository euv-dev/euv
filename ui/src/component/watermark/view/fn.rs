use super::*;

/// A watermark stamped over the content it wraps.
///
/// Renders a `c_euv_watermark` box holding the caller's content, with a
/// `c_euv_watermark_layer` overlay carrying the `c_euv_watermark_text`
/// stamp on top of it.
///
/// The overlay takes no pointer events, so the content underneath stays
/// usable — a watermark is a mark on the page, not a lid on it.
///
/// # Arguments
///
/// - `VirtualNode<EuvWatermarkProps>` - The props node containing the
///   watermark text.
///
/// # Returns
///
/// - `VirtualNode` - The watermarked virtual DOM tree.
#[component]
pub fn euv_watermark(node: VirtualNode<EuvWatermarkProps>) -> VirtualNode {
    let EuvWatermarkProps { text }: EuvWatermarkProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            class: c_euv_watermark()
            children
            if !text.is_empty() {
                div {
                    class: c_euv_watermark_layer()
                    div {
                        class: c_euv_watermark_text()
                        text
                    }
                }
            }
        }
    }
}
