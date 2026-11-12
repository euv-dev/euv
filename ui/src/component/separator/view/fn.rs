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
    let orientation_class: &Css = match orientation {
        EuvSeparatorOrientation::Horizontal => c_euv_separator_horizontal(),
        EuvSeparatorOrientation::Vertical => c_euv_separator_vertical(),
    };
    let orientation_attr: &str = match orientation {
        EuvSeparatorOrientation::Horizontal => SEPARATOR_ORIENTATION_HORIZONTAL,
        EuvSeparatorOrientation::Vertical => SEPARATOR_ORIENTATION_VERTICAL,
    };
    let role: &str = if decorative {
        SEPARATOR_ROLE_DECORATIVE
    } else {
        SEPARATOR_ROLE_SEPARATOR
    };
    let children: VirtualNode = node.get_children().into();
    let base_class: &Css = c_euv_separator();
    let shadcn_class: &Css = if cfg!(feature = "theme-shadcn") {
        c_euv_separator_shadcn()
    } else {
        &EMPTY_CLASS
    };
    html! {
        div {
            "data-slot": "separator"
            role: role
            "aria-orientation": orientation_attr
            "data-orientation": orientation_attr
            class: base_class
            class: orientation_class
            class: shadcn_class
            children
        }
    }
}
