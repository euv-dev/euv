use super::*;

/// The rule that divides one part of a layout from the next.
///
/// The rule is a single hairline in the border colour. A decorative rule
/// carries no role, and a rule that means something carries
/// `role="separator"` together with the direction it runs in.
///
/// # Arguments
///
/// - `VirtualNode<EuvSeparatorProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv separator virtual DOM tree.
#[component]
pub fn euv_separator(node: VirtualNode<EuvSeparatorProps>) -> VirtualNode {
    let EuvSeparatorProps {
        orientation,
        decorative,
    }: EuvSeparatorProps = node.try_get_props().unwrap_or_default();
    let orientation_class: fn() -> &'static Css = match orientation {
        EuvSeparatorOrientation::Horizontal => c_euv_separator_horizontal,
        EuvSeparatorOrientation::Vertical => c_euv_separator_vertical,
    };
    let orientation_attr: &str = match orientation {
        EuvSeparatorOrientation::Horizontal => "horizontal",
        EuvSeparatorOrientation::Vertical => "vertical",
    };
    let role: &str = if decorative { "none" } else { "separator" };
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            role: role
            "aria-orientation": orientation_attr
            class: c_euv_separator()
            class: orientation_class()
            children
        }
    }
}
