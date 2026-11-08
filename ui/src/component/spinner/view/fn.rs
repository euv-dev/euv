use super::*;

/// A busy indicator.
///
/// The glyph is a ring drawn with a CSS border whose leading edge is
/// highlighted and then rotated, so the spinner needs no image and no
/// inline SVG: it is one element and one animation.
///
/// The element carries `role="status"` and an `aria-label`, so assistive
/// technology reads the busy state instead of announcing an empty box.
///
/// # Arguments
///
/// - `VirtualNode<EuvSpinnerProps>` - The props node carrying the label.
///
/// # Returns
///
/// - `VirtualNode` - The spinner virtual DOM tree.
#[component]
pub fn euv_spinner(node: VirtualNode<EuvSpinnerProps>) -> VirtualNode {
    let EuvSpinnerProps { label, show_label }: EuvSpinnerProps =
        node.try_get_props().unwrap_or_default();
    let label_text: &'static str = if label.is_empty() { "Loading" } else { label };
    html! {
        span {
            class: c_euv_spinner_root()
            span {
                class: c_euv_spinner()
                role: "status"
                "aria-label": label_text
            }
            span {
                class: if { show_label.get() } {
                    c_euv_spinner_label()
                } else {
                    c_euv_spinner_label_hidden()
                }
                label_text
            }
        }
    }
}
