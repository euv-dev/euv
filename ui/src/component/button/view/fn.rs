use super::*;

/// A versatile button component.
///
/// The button renders a real `<button>` element carrying `type="button"`,
/// so a button placed inside a form submits nothing by accident. Its
/// appearance is composed of three classes — the shared control class, one
/// variant class and one size class — and its keyboard focus is drawn as
/// a ring around the control rather than as a browser outline.
///
/// # Arguments
///
/// - `VirtualNode<EuvButtonProps>` - The props node containing variant,
///   size, label, onclick and disabled.
///
/// # Returns
///
/// - `VirtualNode` - A styled button element.
#[component]
pub fn euv_button(node: VirtualNode<EuvButtonProps>) -> VirtualNode {
    let EuvButtonProps {
        variant,
        size,
        label,
        onclick: click_handler,
        disabled,
    }: EuvButtonProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let content: VirtualNode = match children {
        VirtualNode::Empty => VirtualNode::Text(TextNode::new(Cow::Owned(label.to_string()), None)),
        other => other,
    };
    html! {
        button {
            type: "button"
            class: button_class(variant, size)
            disabled: disabled
            onclick: click_handler
            content
        }
    }
}

/// Builds the class list of a button.
///
/// The registry splits the control into a base class, a variant class and
/// a size class, so the list is assembled by hand rather than through
/// three `class:` attributes — one attribute would repeat the base name.
///
/// # Arguments
///
/// - `EuvButtonVariant` - The visual variant.
/// - `EuvButtonSize` - The control size.
///
/// # Returns
///
/// - `String` - The class list.
pub(crate) fn button_class(variant: EuvButtonVariant, size: EuvButtonSize) -> String {
    let variant_class: &str = match variant {
        EuvButtonVariant::Primary => c_euv_button_primary().get_name(),
        EuvButtonVariant::Default => c_euv_button_default().get_name(),
        EuvButtonVariant::Secondary => c_euv_button_secondary().get_name(),
        EuvButtonVariant::Destructive => c_euv_button_destructive().get_name(),
        EuvButtonVariant::Outline => c_euv_button_outline().get_name(),
        EuvButtonVariant::Ghost => c_euv_button_ghost().get_name(),
        EuvButtonVariant::Link => c_euv_button_link().get_name(),
    };
    let size_class: &str = match size {
        EuvButtonSize::Default => c_euv_button_size_default().get_name(),
        EuvButtonSize::Small => c_euv_button_size_small().get_name(),
        EuvButtonSize::Large => c_euv_button_size_large().get_name(),
        EuvButtonSize::Icon => c_euv_button_size_icon().get_name(),
    };
    let mut class_name: String = String::with_capacity(96);
    class_name.push_str(c_euv_button().get_name());
    class_name.push(' ');
    class_name.push_str(variant_class);
    class_name.push(' ');
    class_name.push_str(size_class);
    class_name
}
