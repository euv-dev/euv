use super::*;

/// A user or entity avatar rendered as a square image or an initials fallback.
///
/// Renders a `<span>` carrying the base `c_euv_avatar` class plus one of the
/// `c_euv_avatar_small` / `c_euv_avatar_medium` / `c_euv_avatar_large` size
/// classes, and `c_euv_avatar_square` when the `square` signal is true. When
/// `src` is non-empty an `<img>` is rendered, otherwise the `initials` text.
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
    let size_class: &str = match size {
        EuvAvatarSize::Small => c_euv_avatar_small().get_name(),
        EuvAvatarSize::Medium => c_euv_avatar_medium().get_name(),
        EuvAvatarSize::Large => c_euv_avatar_large().get_name(),
    };
    let square_class: &str = c_euv_avatar_square().get_name();
    let mut class_name: String = String::with_capacity(
        c_euv_avatar().get_name().len() + 1 + size_class.len() + 1 + square_class.len(),
    );
    class_name.push_str(c_euv_avatar().get_name());
    class_name.push(' ');
    class_name.push_str(size_class);
    if is_square.get() {
        class_name.push(' ');
        class_name.push_str(c_euv_avatar_square().get_name());
    }
    if src.is_empty() {
        html! {
            span {
                class: class_name
                initials
            }
        }
    } else {
        html! {
            span {
                class: class_name
                img {
                    src: src
                    alt: alt
                }
            }
        }
    }
}
