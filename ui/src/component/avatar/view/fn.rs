use super::*;

/// A user or entity avatar rendered as a circular image or an initials fallback.
///
/// Renders a `<span>` carrying the base `c_euv_avatar` class plus one of the
/// `c_euv_avatar_small` / `c_euv_avatar_medium` / `c_euv_avatar_large` size
/// classes, and `c_euv_avatar_square` when the `square` signal is true. When
/// `src` is non-empty an `<img>` is rendered; if it fails, the initials appear.
///
/// The frame is the surface the reference publishes, so it carries its own
/// slot and the initials it falls back to are a part of their own.
///
/// # Arguments
///
/// - `VirtualNode<EuvAvatarProps>` - The props node carrying the component configuration.
///
/// # Returns
///
/// - `VirtualNode` - The component virtual DOM tree.
#[component]
pub fn euv_avatar(node: VirtualNode<EuvAvatarProps>) -> VirtualNode {
    let EuvAvatarProps {
        src,
        alt,
        initials,
        size,
        square: is_square,
    }: EuvAvatarProps = node.try_get_props().unwrap_or_default();
    let is_square: bool = is_square.get();
    let size_class: &str = match size {
        EuvAvatarSize::Small => c_euv_avatar_small().get_name(),
        EuvAvatarSize::Medium => c_euv_avatar_medium().get_name(),
        EuvAvatarSize::Large => c_euv_avatar_large().get_name(),
    };
    let data_size: &str = match size {
        EuvAvatarSize::Small => AVATAR_DATA_SIZE_SMALL,
        EuvAvatarSize::Medium => AVATAR_DATA_SIZE_DEFAULT,
        EuvAvatarSize::Large => AVATAR_DATA_SIZE_LARGE,
    };
    let square_class: &str = c_euv_avatar_square().get_name();
    let failed_source: Signal<String> = HookContext::use_hook(|| Signal::create(String::new()));
    let failed_source_value: String = failed_source.get();
    let show_fallback: bool = src.is_empty() || failed_source_value == src;
    let on_error: Option<Rc<dyn Fn(Event)>> = if src.is_empty() {
        None
    } else {
        let source: String = src.to_string();
        Some(Rc::new(move |_: Event| {
            failed_source.set(source.clone());
        }))
    };
    let mut class_name: String = String::with_capacity(
        c_euv_avatar().get_name().len() + 1 + size_class.len() + 1 + square_class.len(),
    );
    class_name.push_str(c_euv_avatar().get_name());
    class_name.push(' ');
    class_name.push_str(size_class);
    if is_square {
        class_name.push(' ');
        class_name.push_str(c_euv_avatar_square().get_name());
    }
    html! {
        span {
            class: class_name
            "data-slot": "avatar"
            "data-size": data_size
            if show_fallback {
                euv_avatar_fallback {
                    text: initials
                }
            }
            else {
                img {
                    src: src
                    alt: alt
                    "data-slot": "avatar-image"
                    class: c_euv_avatar_image()
                    onerror: on_error
                }
            }
        }
    }
}

/// The initials shown when an avatar has no picture or its image fails.
///
/// The fallback takes the size and the colour of the frame it sits in,
/// so the initials read as the avatar itself rather than as a label
/// under it.
///
/// # Arguments
///
/// - `VirtualNode<EuvAvatarFallbackProps>` - The props node carrying the
///   component configuration.
///
/// # Returns
///
/// - `VirtualNode` - The avatar fallback virtual DOM tree.
#[component]
pub fn euv_avatar_fallback(node: VirtualNode<EuvAvatarFallbackProps>) -> VirtualNode {
    let EuvAvatarFallbackProps { text }: EuvAvatarFallbackProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let content: VirtualNode = match children {
        VirtualNode::Empty => VirtualNode::Text(TextNode::new(Cow::Owned(text.to_string()), None)),
        other => other,
    };
    let shadcn_class: &Css = if cfg!(feature = "theme-shadcn") {
        c_euv_avatar_fallback_shadcn()
    } else {
        &EMPTY_CLASS
    };
    html! {
        span {
            class: c_euv_avatar_fallback()
            class: shadcn_class
            "data-slot": "avatar-fallback"
            content
        }
    }
}

/// The mark that rides on the edge of an avatar.
///
/// The badge is a small frame at the far end of the avatar, so the
/// reader sees the mark without the mark taking the room of the
/// picture.
///
/// # Arguments
///
/// - `VirtualNode<EuvAvatarBadgeProps>` - The props node carrying the
///   component configuration.
///
/// # Returns
///
/// - `VirtualNode` - The avatar badge virtual DOM tree.
#[component]
pub fn euv_avatar_badge(node: VirtualNode<EuvAvatarBadgeProps>) -> VirtualNode {
    let EuvAvatarBadgeProps { text }: EuvAvatarBadgeProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let content: VirtualNode = match children {
        VirtualNode::Empty => VirtualNode::Text(TextNode::new(Cow::Owned(text.to_string()), None)),
        other => other,
    };
    html! {
        span {
            class: c_euv_avatar_badge()
            "data-slot": "avatar-badge"
            content
        }
    }
}

/// The row of avatars that belong together.
///
/// The group overlaps its members, so a long list of people still reads
/// as one cluster rather than as a strip of separate pictures.
///
/// # Arguments
///
/// - `VirtualNode<EuvAvatarGroupProps>` - The props node carrying the
///   component configuration.
///
/// # Returns
///
/// - `VirtualNode` - The avatar group virtual DOM tree.
#[component]
pub fn euv_avatar_group(node: VirtualNode<EuvAvatarGroupProps>) -> VirtualNode {
    let EuvAvatarGroupProps {}: EuvAvatarGroupProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            class: c_euv_avatar_group()
            "data-slot": "avatar-group"
            children
        }
    }
}

/// The count of the members a group does not show.
///
/// The count sits where the last avatar would be, so the reader learns
/// how many more there are without the group growing.
///
/// # Arguments
///
/// - `VirtualNode<EuvAvatarGroupCountProps>` - The props node carrying the
///   component configuration.
///
/// # Returns
///
/// - `VirtualNode` - The avatar group count virtual DOM tree.
#[component]
pub fn euv_avatar_group_count(node: VirtualNode<EuvAvatarGroupCountProps>) -> VirtualNode {
    let EuvAvatarGroupCountProps { text }: EuvAvatarGroupCountProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let content: VirtualNode = match children {
        VirtualNode::Empty => VirtualNode::Text(TextNode::new(Cow::Owned(text.to_string()), None)),
        other => other,
    };
    html! {
        div {
            class: c_euv_avatar_group_count()
            "data-slot": "avatar-group-count"
            content
        }
    }
}
