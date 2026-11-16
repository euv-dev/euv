use super::*;

/// A field with the pieces that belong to it.
///
/// The group is the frame: one border, one surface and one focus ring
/// around the control and its addons. The control itself carries no
/// border of its own, so the frame never doubles up.
///
/// # Arguments
///
/// - `VirtualNode<EuvInputGroupProps>` - The props node carrying the
///   disabled state.
///
/// # Returns
///
/// - `VirtualNode` - The input-group virtual DOM tree.
#[component]
pub fn euv_input_group(node: VirtualNode<EuvInputGroupProps>) -> VirtualNode {
    let EuvInputGroupProps { disabled }: EuvInputGroupProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let base_class: &Css = c_euv_input_group();
    html! {
        div {
            class: if { disabled.get() } {
                c_euv_input_group_disabled()
            } else {
                base_class
            }
            role: "group"
            "data-slot": "input-group"
            "data-disabled": if { disabled.get() } {
                "true"
            } else {
                "false"
            }
            children
        }
    }
}

/// An icon, a prefix or a suffix that belongs to the field.
///
/// The addon sits where the alignment prop puts it: beside the control
/// on the inline positions, on its own line above or below it on the
/// block positions.
///
/// # Arguments
///
/// - `VirtualNode<EuvInputGroupAddonProps>` - The props node carrying the
///   alignment.
///
/// # Returns
///
/// - `VirtualNode` - The addon virtual DOM tree.
#[component]
pub fn euv_input_group_addon(node: VirtualNode<EuvInputGroupAddonProps>) -> VirtualNode {
    let EuvInputGroupAddonProps { align }: EuvInputGroupAddonProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let align_value: &'static str = match align {
        EuvInputGroupAlign::InlineStart => "inline-start",
        EuvInputGroupAlign::InlineEnd => "inline-end",
        EuvInputGroupAlign::BlockStart => "block-start",
        EuvInputGroupAlign::BlockEnd => "block-end",
    };
    html! {
        div {
            class: match align {
                EuvInputGroupAlign::InlineStart => c_euv_input_group_addon_inline_start(),
                EuvInputGroupAlign::InlineEnd => c_euv_input_group_addon_inline_end(),
                EuvInputGroupAlign::BlockStart => c_euv_input_group_addon_block_start(),
                EuvInputGroupAlign::BlockEnd => c_euv_input_group_addon_block_end(),
            }
            role: "group"
            "data-slot": "input-group-addon"
            "data-align": align_value
            children
        }
    }
}

/// A button that acts on the field.
///
/// # Arguments
///
/// - `VirtualNode<EuvInputGroupButtonProps>` - The props node carrying the
///   size, the label and the click handler.
///
/// # Returns
///
/// - `VirtualNode` - The button virtual DOM tree.
#[component]
pub fn euv_input_group_button(node: VirtualNode<EuvInputGroupButtonProps>) -> VirtualNode {
    let EuvInputGroupButtonProps {
        size,
        label,
        onclick,
    }: EuvInputGroupButtonProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let has_children: bool = !matches!(&children, VirtualNode::Empty);
    html! {
        button {
            type: "button"
            class: match size {
                EuvInputGroupButtonSize::Xs => c_euv_input_group_button_xs(),
                EuvInputGroupButtonSize::Sm => c_euv_input_group_button_sm(),
                EuvInputGroupButtonSize::IconXs => c_euv_input_group_button_icon_xs(),
                EuvInputGroupButtonSize::IconSm => c_euv_input_group_button_icon_sm(),
            }
            "data-slot": "input-group-button"
            "data-size": match size {
                EuvInputGroupButtonSize::Xs => "xs",
                EuvInputGroupButtonSize::Sm => "sm",
                EuvInputGroupButtonSize::IconXs => "icon-xs",
                EuvInputGroupButtonSize::IconSm => "icon-sm",
            }
            onclick: onclick
            if has_children {
                children
            } else {
                label
            }
        }
    }
}

/// The quiet caption of a field.
///
/// # Arguments
///
/// - `VirtualNode<EuvInputGroupTextProps>` - The props node carrying the
///   text.
///
/// # Returns
///
/// - `VirtualNode` - The caption virtual DOM tree.
#[component]
pub fn euv_input_group_text(node: VirtualNode<EuvInputGroupTextProps>) -> VirtualNode {
    let EuvInputGroupTextProps { text }: EuvInputGroupTextProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let has_children: bool = !matches!(&children, VirtualNode::Empty);
    html! {
        span {
            class: c_euv_input_group_text()
            "data-slot": "input-group-text"
            if has_children {
                children
            } else {
                text
            }
        }
    }
}

/// The single-line control of a group.
///
/// # Arguments
///
/// - `VirtualNode<EuvInputGroupInputProps>` - The props node carrying the
///   value signal.
///
/// # Returns
///
/// - `VirtualNode` - The control virtual DOM tree.
#[component]
pub fn euv_input_group_input(node: VirtualNode<EuvInputGroupInputProps>) -> VirtualNode {
    let EuvInputGroupInputProps {
        id,
        placeholder,
        value,
        disabled,
        oninput,
    }: EuvInputGroupInputProps = node.try_get_props().unwrap_or_default();
    let is_disabled: bool = disabled.get();
    html! {
        input {
            id: id
            type: "text"
            placeholder: placeholder
            value: value
            class: c_euv_input_group_input()
            "data-slot": "input-group-input"
            disabled: is_disabled
            oninput: oninput
        }
    }
}

/// The multi-line control of a group.
///
/// # Arguments
///
/// - `VirtualNode<EuvInputGroupTextareaProps>` - The props node carrying
///   the value signal.
///
/// # Returns
///
/// - `VirtualNode` - The control virtual DOM tree.
#[component]
pub fn euv_input_group_textarea(node: VirtualNode<EuvInputGroupTextareaProps>) -> VirtualNode {
    let EuvInputGroupTextareaProps {
        id,
        placeholder,
        value,
        rows,
        disabled,
        oninput,
    }: EuvInputGroupTextareaProps = node.try_get_props().unwrap_or_default();
    let effective_rows: i32 = if rows > 0 { rows } else { 3 };
    let is_disabled: bool = disabled.get();
    html! {
        textarea {
            id: id
            rows: effective_rows
            placeholder: placeholder
            value: value
            class: c_euv_input_group_textarea()
            "data-slot": "input-group-control"
            disabled: is_disabled
            oninput: oninput
        }
    }
}
