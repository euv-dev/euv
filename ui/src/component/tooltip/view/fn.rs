use super::*;

/// A hover / focus hint anchored to its trigger element.
///
/// The trigger element is supplied as children and wrapped in a
/// `c_euv_tooltip` inline-flex box. The bubble always stays mounted and is
/// revealed purely by CSS, so no JavaScript positioning runs on hover and the
/// hint costs nothing until the pointer or keyboard focus reaches the trigger.
/// The bubble carries the shared `c_euv_tooltip_bubble` styling plus the
/// `c_euv_tooltip_bubble_top` / `_bottom` / `_left` / `_right` class that
/// anchors it to the requested side of the trigger.
///
/// # Arguments
///
/// - `VirtualNode<EuvTooltipProps>` - The props node containing the hint text
///   and the bubble placement.
///
/// # Returns
///
/// - `VirtualNode` - The tooltip wrapper, trigger children and hint bubble.
#[component]
pub fn euv_tooltip(node: VirtualNode<EuvTooltipProps>) -> VirtualNode {
    let EuvTooltipProps { text, placement }: EuvTooltipProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let bubble_class: fn() -> &'static Css = match placement {
        EuvTooltipPlacement::Top => c_euv_tooltip_bubble_top,
        EuvTooltipPlacement::Bottom => c_euv_tooltip_bubble_bottom,
        EuvTooltipPlacement::Left => c_euv_tooltip_bubble_left,
        EuvTooltipPlacement::Right => c_euv_tooltip_bubble_right,
    };
    html! {
        span {
            class: c_euv_tooltip()
            children
            div {
                class: c_euv_tooltip_bubble()
                class: bubble_class()
                {
                    text
                }
            }
        }
    }
}
