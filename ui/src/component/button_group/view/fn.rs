use super::*;

/// A row of joined buttons.
///
/// The group carries `role="group"` and the orientation it was given, so
/// the direction is readable from the DOM and not only from the layout.
///
/// # Arguments
///
/// - `VirtualNode<EuvButtonGroupProps>` - The props node carrying the
///   orientation.
///
/// # Returns
///
/// - `VirtualNode` - The button-group virtual DOM tree.
#[component]
pub fn euv_button_group(node: VirtualNode<EuvButtonGroupProps>) -> VirtualNode {
    let EuvButtonGroupProps { orientation }: EuvButtonGroupProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let orientation_value: &'static str = match orientation {
        EuvButtonGroupOrientation::Horizontal => ORIENTATION_HORIZONTAL,
        EuvButtonGroupOrientation::Vertical => ORIENTATION_VERTICAL,
    };
    html! {
        div {
            class: if { orientation == EuvButtonGroupOrientation::Vertical } {
                c_euv_button_group_vertical()
            } else {
                c_euv_button_group_horizontal()
            }
            role: "group"
            "data-slot": "button-group"
            "data-orientation": orientation_value
            children
        }
    }
}

/// A button that belongs to a button group.
///
/// The reference composes a group out of buttons, so the button inside a
/// group is a part of the group: the group owns the joined frame and the
/// button carries the control.
///
/// # Arguments
///
/// - `VirtualNode<EuvButtonGroupButtonProps>` - The props node carrying the
///   variant, the size and the label of the button.
///
/// # Returns
///
/// - `VirtualNode` - The grouped button virtual DOM tree.
#[component]
pub fn euv_button_group_button(node: VirtualNode<EuvButtonGroupButtonProps>) -> VirtualNode {
    let EuvButtonGroupButtonProps {
        variant,
        size,
        label,
        onclick: click_handler,
        disabled,
    }: EuvButtonGroupButtonProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let has_children: bool = !matches!(&children, VirtualNode::Empty);
    html! {
        button {
            type: "button"
            class: crate::component::button::button_class(variant, size)
            "data-slot": "button"
            disabled: disabled
            onclick: click_handler
            if has_children {
                children
            } else {
                label
            }
        }
    }
}

/// A caption that sits inside a button group.
///
/// The caption takes the muted surface so the eye reads it as a label
/// rather than as another action.
///
/// # Arguments
///
/// - `VirtualNode<EuvButtonGroupTextProps>` - The props node carrying the
///   text.
///
/// # Returns
///
/// - `VirtualNode` - The caption virtual DOM tree.
#[component]
pub fn euv_button_group_text(node: VirtualNode<EuvButtonGroupTextProps>) -> VirtualNode {
    let EuvButtonGroupTextProps { text }: EuvButtonGroupTextProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let has_children: bool = !matches!(&children, VirtualNode::Empty);
    html! {
        div {
            class: c_euv_button_group_text()
            "data-slot": "button-group-text"
            if has_children {
                children
            } else {
                text
            }
        }
    }
}

/// A rule between two buttons of a group.
///
/// # Arguments
///
/// - `VirtualNode<EuvButtonGroupSeparatorProps>` - The props node
///   carrying the orientation.
///
/// # Returns
///
/// - `VirtualNode` - The separator virtual DOM tree.
#[component]
pub fn euv_button_group_separator(node: VirtualNode<EuvButtonGroupSeparatorProps>) -> VirtualNode {
    let EuvButtonGroupSeparatorProps { orientation }: EuvButtonGroupSeparatorProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let orientation_value: &'static str = match orientation {
        EuvButtonGroupSeparatorOrientation::Vertical => ORIENTATION_VERTICAL,
        EuvButtonGroupSeparatorOrientation::Horizontal => ORIENTATION_HORIZONTAL,
    };
    html! {
        div {
            class: if {
                orientation == EuvButtonGroupSeparatorOrientation::Horizontal
            } {
                c_euv_button_group_separator_horizontal()
            } else {
                c_euv_button_group_separator_vertical()
            }
            role: "separator"
            "data-slot": "button-group-separator"
            "data-orientation": orientation_value
            children
        }
    }
}
