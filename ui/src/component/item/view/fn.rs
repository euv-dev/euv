use super::*;

/// A list of items.
///
/// The group carries `role="list"`, so the rows inside it are announced
/// as a list even when they are not list elements themselves.
///
/// # Arguments
///
/// - `VirtualNode<EuvItemProps>` - The props node.
///
/// # Returns
///
/// - `VirtualNode` - The group virtual DOM tree.
#[component]
pub fn euv_item_group(node: VirtualNode<EuvItemProps>) -> VirtualNode {
    let base_class: &Css = c_euv_item_group();
    let shadcn_class: &Css = if cfg!(feature = "theme-shadcn") {
        c_euv_item_group_shadcn()
    } else {
        &EMPTY_CLASS
    };
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            class: base_class
            class: shadcn_class
            role: "list"
            "data-slot": "item-group"
            children
        }
    }
}

/// One row of a list.
///
/// The row carries its variant and its density as data attributes, so
/// the choice is readable from the DOM and not only from the layout.
///
/// # Arguments
///
/// - `VirtualNode<EuvItemProps>` - The props node carrying the variant
///   and the size.
///
/// # Returns
///
/// - `VirtualNode` - The item virtual DOM tree.
#[component]
pub fn euv_item(node: VirtualNode<EuvItemProps>) -> VirtualNode {
    let EuvItemProps { variant, size }: EuvItemProps = node.try_get_props().unwrap_or_default();
    let base_variant_class: &Css = match variant {
        EuvItemVariant::Default => c_euv_item_default(),
        EuvItemVariant::Outline => c_euv_item_outline(),
        EuvItemVariant::Muted => c_euv_item_muted(),
    };
    let shadcn_variant_class: &Css = if cfg!(feature = "theme-shadcn") {
        match variant {
            EuvItemVariant::Default => c_euv_item_default_shadcn(),
            EuvItemVariant::Outline => c_euv_item_outline_shadcn(),
            EuvItemVariant::Muted => c_euv_item_muted_shadcn(),
        }
    } else {
        &EMPTY_CLASS
    };
    let base_size_class: &Css = match size {
        EuvItemSize::Default => c_euv_item_size_default(),
        EuvItemSize::Sm => c_euv_item_size_sm(),
        EuvItemSize::Xs => c_euv_item_size_xs(),
    };
    let shadcn_size_class: &Css = if cfg!(feature = "theme-shadcn") {
        match size {
            EuvItemSize::Default => c_euv_item_size_default_shadcn(),
            EuvItemSize::Sm => c_euv_item_size_sm_shadcn(),
            EuvItemSize::Xs => c_euv_item_size_xs_shadcn(),
        }
    } else {
        &EMPTY_CLASS
    };
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            class: base_variant_class
            class: shadcn_variant_class
            class: base_size_class
            class: shadcn_size_class
            "data-slot": "item"
            "data-variant": match variant {
                EuvItemVariant::Default => "default",
                EuvItemVariant::Outline => "outline",
                EuvItemVariant::Muted => "muted",
            }
            "data-size": match size {
                EuvItemSize::Default => ITEM_SIZE_DEFAULT,
                EuvItemSize::Sm => ITEM_SIZE_SM,
                EuvItemSize::Xs => ITEM_SIZE_XS,
            }
            children
        }
    }
}

/// The media beside an item.
///
/// The media never shrinks, so the text column beside it keeps the same
/// width whether the row carries an icon or a picture.
///
/// # Arguments
///
/// - `VirtualNode<EuvItemMediaProps>` - The props node carrying the
///   media variant.
///
/// # Returns
///
/// - `VirtualNode` - The media virtual DOM tree.
#[component]
pub fn euv_item_media(node: VirtualNode<EuvItemMediaProps>) -> VirtualNode {
    let EuvItemMediaProps { variant }: EuvItemMediaProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            class: match variant {
                EuvItemMediaVariant::Default => c_euv_item_media_default(),
                EuvItemMediaVariant::Icon => c_euv_item_media_icon(),
                EuvItemMediaVariant::Image => c_euv_item_media_image(),
            }
            "data-slot": "item-media"
            "data-variant": match variant {
                EuvItemMediaVariant::Default => "default",
                EuvItemMediaVariant::Icon => "icon",
                EuvItemMediaVariant::Image => "image",
            }
            children
        }
    }
}

/// The text column of an item.
///
/// The column takes the room that is left, so the actions of the row keep
/// their place at the far end.
///
/// # Arguments
///
/// - `VirtualNode<EuvItemProps>` - The props node.
///
/// # Returns
///
/// - `VirtualNode` - The content virtual DOM tree.
#[component]
pub fn euv_item_content(node: VirtualNode<EuvItemProps>) -> VirtualNode {
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            class: c_euv_item_content()
            "data-slot": "item-content"
            children
        }
    }
}

/// The title of an item.
///
/// # Arguments
///
/// - `VirtualNode<EuvItemTitleProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv item title virtual DOM tree.
#[component]
pub fn euv_item_title(node: VirtualNode<EuvItemTitleProps>) -> VirtualNode {
    let EuvItemTitleProps { text }: EuvItemTitleProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let has_children: bool = !matches!(&children, VirtualNode::Empty);
    html! {
        div {
            class: c_euv_item_title()
            "data-slot": "item-title"
            if has_children {
                children
            } else {
                text
            }
        }
    }
}

/// The description of an item.
///
/// # Arguments
///
/// - `VirtualNode<EuvItemDescriptionProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv item description virtual DOM tree.
#[component]
pub fn euv_item_description(node: VirtualNode<EuvItemDescriptionProps>) -> VirtualNode {
    let EuvItemDescriptionProps { text }: EuvItemDescriptionProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let has_children: bool = !matches!(&children, VirtualNode::Empty);
    html! {
        p {
            class: c_euv_item_description()
            "data-slot": "item-description"
            if has_children {
                children
            } else {
                text
            }
        }
    }
}

/// The actions of an item.
///
/// # Arguments
///
/// - `VirtualNode<EuvItemProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv item actions virtual DOM tree.
#[component]
pub fn euv_item_actions(node: VirtualNode<EuvItemProps>) -> VirtualNode {
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            class: c_euv_item_actions()
            "data-slot": "item-actions"
            children
        }
    }
}

/// The header of an item.
///
/// # Arguments
///
/// - `VirtualNode<EuvItemProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv item header virtual DOM tree.
#[component]
pub fn euv_item_header(node: VirtualNode<EuvItemProps>) -> VirtualNode {
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            class: c_euv_item_header()
            "data-slot": "item-header"
            children
        }
    }
}

/// The footer of an item.
///
/// # Arguments
///
/// - `VirtualNode<EuvItemProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv item footer virtual DOM tree.
#[component]
pub fn euv_item_footer(node: VirtualNode<EuvItemProps>) -> VirtualNode {
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            class: c_euv_item_footer()
            "data-slot": "item-footer"
            children
        }
    }
}

/// A rule between two items of a group.
///
/// # Arguments
///
/// - `VirtualNode<EuvItemProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv item separator virtual DOM tree.
#[component]
pub fn euv_item_separator(node: VirtualNode<EuvItemProps>) -> VirtualNode {
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            class: c_euv_item_separator()
            role: "separator"
            "data-slot": "item-separator"
            children
        }
    }
}
