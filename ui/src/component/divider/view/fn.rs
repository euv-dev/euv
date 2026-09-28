use super::*;

/// A generic section rule aligned with common component libraries.
///
/// Renders a single dashed rule in the requested orientation. A non-empty
/// `label` switches to the labelled form, where the text is centred
/// between two rule segments — the same monochrome "dashed line, text,
/// dashed line" motif the design system uses for section breaks.
///
/// # Arguments
///
/// - `VirtualNode<EuvDividerProps>` - The props node containing orientation
///   and label.
///
/// # Returns
///
/// - `VirtualNode` - The divider virtual DOM tree.
#[component]
pub fn euv_divider(node: VirtualNode<EuvDividerProps>) -> VirtualNode {
    let EuvDividerProps { orientation, label }: EuvDividerProps =
        node.try_get_props().unwrap_or_default();
    let orientation_class: fn() -> &'static Css = match orientation {
        EuvDividerOrientation::Horizontal => c_euv_divider_horizontal,
        EuvDividerOrientation::Vertical => c_euv_divider_vertical,
    };
    html! {
        div {
            class: c_euv_divider()
            class: {
                orientation_class()
            }
            if { !label.is_empty() } {
                div {
                    class: c_euv_divider_labeled()
                    span {
                        class: c_euv_divider_label_text()
                        {
                            label
                        }
                    }
                }
            }
        }
    }
}
