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
        data_slot,
        onclick: click_handler,
        disabled,
    }: EuvButtonProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let content: VirtualNode = match children {
        VirtualNode::Empty => VirtualNode::Text(TextNode::new(Cow::Owned(label.to_string()), None)),
        other => other,
    };
    let size_attr: &str = match size {
        EuvButtonSize::Default => BUTTON_SIZE_DEFAULT,
        EuvButtonSize::Small => BUTTON_SIZE_SMALL,
        EuvButtonSize::Large => BUTTON_SIZE_LARGE,
        EuvButtonSize::Icon => BUTTON_SIZE_ICON,
        EuvButtonSize::ExtraSmall => BUTTON_SIZE_XS,
        EuvButtonSize::IconExtraSmall => BUTTON_SIZE_ICON_XS,
        EuvButtonSize::IconSmall => BUTTON_SIZE_ICON_SM,
        EuvButtonSize::IconLarge => BUTTON_SIZE_ICON_LG,
    };
    let data_slot: &str = data_slot.unwrap_or(BUTTON_SLOT);
    html! {
        button {
            type: "button"
            class: button_class(variant, size)
            "data-slot": data_slot
            "data-variant": match variant {
                EuvButtonVariant::Primary => BUTTON_VARIANT_DEFAULT,
                EuvButtonVariant::Default => BUTTON_VARIANT_DEFAULT,
                EuvButtonVariant::Secondary => BUTTON_VARIANT_SECONDARY,
                EuvButtonVariant::Destructive => BUTTON_VARIANT_DESTRUCTIVE,
                EuvButtonVariant::Outline => BUTTON_VARIANT_OUTLINE,
                EuvButtonVariant::Ghost => BUTTON_VARIANT_GHOST,
                EuvButtonVariant::Link => BUTTON_VARIANT_LINK,
            }
            "data-size": size_attr
            "aria-label": label
            disabled: disabled
            onclick: click_handler
            content
        }
    }
}

/// A button that is joined to the buttons beside it.
///
/// The reference documents the group of buttons on the button page, so
/// the joined button is a part of the button family: the frame around it
/// joins it to its neighbours, so the border between them is dropped and
/// only the outer frame is drawn.
///
/// # Arguments
///
/// - `VirtualNode<EuvButtonGroupedProps>` - The props node containing
///   variant, size, label, onclick and disabled.
///
/// # Returns
///
/// - `VirtualNode` - The joined button virtual DOM tree.
#[component]
pub fn euv_button_grouped(node: VirtualNode<EuvButtonGroupedProps>) -> VirtualNode {
    let EuvButtonGroupedProps {
        variant,
        size,
        label,
        onclick: click_handler,
        disabled,
    }: EuvButtonGroupedProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let content: VirtualNode = match children {
        VirtualNode::Empty => VirtualNode::Text(TextNode::new(Cow::Owned(label.to_string()), None)),
        other => other,
    };
    let size_attr: &str = match size {
        EuvButtonSize::Default => BUTTON_SIZE_DEFAULT,
        EuvButtonSize::Small => BUTTON_SIZE_SMALL,
        EuvButtonSize::Large => BUTTON_SIZE_LARGE,
        EuvButtonSize::Icon => BUTTON_SIZE_ICON,
        EuvButtonSize::ExtraSmall => BUTTON_SIZE_XS,
        EuvButtonSize::IconExtraSmall => BUTTON_SIZE_ICON_XS,
        EuvButtonSize::IconSmall => BUTTON_SIZE_ICON_SM,
        EuvButtonSize::IconLarge => BUTTON_SIZE_ICON_LG,
    };
    html! {
        div {
            class: c_euv_button_group_horizontal()
            "data-slot": "button-group"
            button {
                type: "button"
                class: button_class(variant, size)
                "data-slot": BUTTON_SLOT
                "data-variant": match variant {
                    EuvButtonVariant::Primary | EuvButtonVariant::Default => BUTTON_VARIANT_DEFAULT,
                    EuvButtonVariant::Secondary => BUTTON_VARIANT_SECONDARY,
                    EuvButtonVariant::Destructive => BUTTON_VARIANT_DESTRUCTIVE,
                    EuvButtonVariant::Outline => BUTTON_VARIANT_OUTLINE,
                    EuvButtonVariant::Ghost => BUTTON_VARIANT_GHOST,
                    EuvButtonVariant::Link => BUTTON_VARIANT_LINK,
                }
                "data-size": size_attr
                "aria-label": label
                disabled: disabled
                onclick: click_handler
                content
            }
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
    let base_class: &str = c_euv_button().get_name();
    let shadcn_class: &str = if cfg!(feature = "theme-shadcn") {
        c_euv_button_shadcn().get_name()
    } else {
        ""
    };
    let variant_class: &str = match variant {
        EuvButtonVariant::Primary => c_euv_button_primary().get_name(),
        EuvButtonVariant::Default => c_euv_button_default().get_name(),
        EuvButtonVariant::Secondary => c_euv_button_secondary().get_name(),
        EuvButtonVariant::Destructive => c_euv_button_destructive().get_name(),
        EuvButtonVariant::Outline => c_euv_button_outline().get_name(),
        EuvButtonVariant::Ghost => c_euv_button_ghost().get_name(),
        EuvButtonVariant::Link => c_euv_button_link().get_name(),
    };
    let shadcn_variant_class: &str = if cfg!(feature = "theme-shadcn") {
        match variant {
            EuvButtonVariant::Primary | EuvButtonVariant::Default => {
                c_euv_button_default_shadcn().get_name()
            }
            EuvButtonVariant::Secondary => c_euv_button_secondary_shadcn().get_name(),
            EuvButtonVariant::Destructive => c_euv_button_destructive_shadcn().get_name(),
            EuvButtonVariant::Outline => c_euv_button_outline_shadcn().get_name(),
            EuvButtonVariant::Ghost => c_euv_button_ghost_shadcn().get_name(),
            EuvButtonVariant::Link => "",
        }
    } else {
        ""
    };
    let size_class: &str = match size {
        EuvButtonSize::Default => c_euv_button_size_default().get_name(),
        EuvButtonSize::Small => c_euv_button_size_small().get_name(),
        EuvButtonSize::Large => c_euv_button_size_large().get_name(),
        EuvButtonSize::Icon => c_euv_button_size_icon().get_name(),
        EuvButtonSize::ExtraSmall => c_euv_button_size_xs().get_name(),
        EuvButtonSize::IconExtraSmall => c_euv_button_size_icon_xs().get_name(),
        EuvButtonSize::IconSmall => c_euv_button_size_icon_sm().get_name(),
        EuvButtonSize::IconLarge => c_euv_button_size_icon_lg().get_name(),
    };
    let shadcn_size_class: &str = if cfg!(feature = "theme-shadcn") {
        match size {
            EuvButtonSize::Default => c_euv_button_size_default_shadcn().get_name(),
            EuvButtonSize::Small => c_euv_button_size_small_shadcn().get_name(),
            EuvButtonSize::Large => c_euv_button_size_large_shadcn().get_name(),
            EuvButtonSize::Icon => c_euv_button_size_icon_shadcn().get_name(),
            EuvButtonSize::ExtraSmall => c_euv_button_size_xs_shadcn().get_name(),
            EuvButtonSize::IconExtraSmall => c_euv_button_size_icon_xs_shadcn().get_name(),
            EuvButtonSize::IconSmall => c_euv_button_size_icon_sm_shadcn().get_name(),
            EuvButtonSize::IconLarge => c_euv_button_size_icon_lg_shadcn().get_name(),
        }
    } else {
        ""
    };
    let mut class_name: String = String::with_capacity(160);
    class_name.push_str(base_class);
    class_name.push(' ');
    class_name.push_str(variant_class);
    class_name.push(' ');
    class_name.push_str(size_class);
    if !shadcn_class.is_empty() {
        class_name.push(' ');
        class_name.push_str(shadcn_class);
        if !shadcn_variant_class.is_empty() {
            class_name.push(' ');
            class_name.push_str(shadcn_variant_class);
        }
        class_name.push(' ');
        class_name.push_str(shadcn_size_class);
    }
    class_name
}
