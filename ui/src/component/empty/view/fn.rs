use super::*;

/// An empty state, for a region that has nothing to show.
///
/// Renders a `c_euv_empty` block holding an optional `c_euv_empty_icon`
/// glyph, the `c_euv_empty_text` headline, an optional
/// `c_euv_empty_desc` explanation, and an optional action.
///
/// The action is a real `euv_button` in the `Primary` variant — the
/// project standard forbids hand-rolled buttons — and it only appears
/// when `action_label` is not empty.
///
/// # Arguments
///
/// - `VirtualNode<EuvEmptyProps>` - The props node containing the empty
///   state configuration.
///
/// # Returns
///
/// - `VirtualNode` - The empty state virtual DOM tree.
#[component]
pub fn euv_empty(node: VirtualNode<EuvEmptyProps>) -> VirtualNode {
    let EuvEmptyProps {
        icon,
        title,
        desc,
        action_label,
        on_action,
    }: EuvEmptyProps = node.try_get_props().unwrap_or_default();
    let has_action: bool = !action_label.is_empty();
    let base_class: &Css = c_euv_empty();
    let shadcn_class: &Css = if cfg!(feature = "theme-shadcn") {
        c_euv_empty_shadcn()
    } else {
        &EMPTY_CLASS
    };
    html! {
            div {
                "data-slot": "empty"
                class: base_class
                class: shadcn_class
                if !icon.is_empty() {
                    div {
                        "data-slot": "empty-icon"
                        class: c_euv_empty_icon()
                        icon
                    }
                }
                div {
                    class: c_euv_empty_text()
                    title
                }
                if !desc.is_empty() {
                    div {
                        class: c_euv_empty_desc()
                        desc
                    }
                }
                if has_action {
                    div {
                        class: c_euv_empty_action()
                        euv_button {
                            variant: EuvButtonVariant::Primary
                            label: action_label
                            onclick: on_action
                        }
                    }
                }
            }
    }
}

/// Holds the head of an empty state.
///
/// The header stacks the picture, the headline and the explanation and
/// centres them, so the reader reads them as one block.
///
/// # Arguments
///
/// - `VirtualNode<EuvEmptyHeaderProps>` - The props node carrying the
///   settings of the header.
///
/// # Returns
///
/// - `VirtualNode` - The euv empty header virtual DOM tree.
#[component]
pub fn euv_empty_header(node: VirtualNode<EuvEmptyHeaderProps>) -> VirtualNode {
    let EuvEmptyHeaderProps {}: EuvEmptyHeaderProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
            div {
                "data-slot": "empty-header"
                class: c_euv_empty_header()
                children
            }
    }
}

/// Holds the picture that marks an empty state.
///
/// The picture is either the glyph itself or the glyph sitting in a
/// rounded box, so the reader recognises the state before the headline
/// tells them what is missing.
///
/// # Arguments
///
/// - `VirtualNode<EuvEmptyMediaProps>` - The props node carrying the
///   size the picture is drawn at.
///
/// # Returns
///
/// - `VirtualNode` - The euv empty media virtual DOM tree.
#[component]
pub fn euv_empty_media(node: VirtualNode<EuvEmptyMediaProps>) -> VirtualNode {
    let EuvEmptyMediaProps { variant }: EuvEmptyMediaProps =
        node.try_get_props().unwrap_or_default();
    let variant_attr: &str = match variant {
        EuvEmptyMediaVariant::Default => EMPTY_MEDIA_VARIANT_DEFAULT,
        EuvEmptyMediaVariant::Icon => EMPTY_MEDIA_VARIANT_ICON,
    };
    let variant_class: fn() -> &'static Css = match variant {
        EuvEmptyMediaVariant::Default => c_euv_empty_media_default,
        EuvEmptyMediaVariant::Icon => c_euv_empty_media_icon,
    };
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "empty-media"
            "data-variant": variant_attr
            class: c_euv_empty_media()
            class: variant_class()
            children
        }
    }
}

/// Names the state of an empty region.
///
/// The title is the headline of the state, so it is written at the size
/// of a headline and it says what is missing.
///
/// # Arguments
///
/// - `VirtualNode<EuvEmptyTitleProps>` - The props node carrying the
///   settings of the title.
///
/// # Returns
///
/// - `VirtualNode` - The euv empty title virtual DOM tree.
#[component]
pub fn euv_empty_title(node: VirtualNode<EuvEmptyTitleProps>) -> VirtualNode {
    let EuvEmptyTitleProps {}: EuvEmptyTitleProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "empty-title"
            class: c_euv_empty_title()
            children
        }
    }
}

/// Explains the state of an empty region.
///
/// The description says why the region is empty, in the quiet voice the
/// reference uses for text that is read rather than acted on.
///
/// # Arguments
///
/// - `VirtualNode<EuvEmptyDescriptionProps>` - The props node carrying
///   the settings of the description.
///
/// # Returns
///
/// - `VirtualNode` - The euv empty description virtual DOM tree.
#[component]
pub fn euv_empty_description(node: VirtualNode<EuvEmptyDescriptionProps>) -> VirtualNode {
    let EuvEmptyDescriptionProps {}: EuvEmptyDescriptionProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "empty-description"
            class: c_euv_empty_description()
            children
        }
    }
}

/// Holds the actions of an empty region.
///
/// The content holds whatever the reader can do about the empty state,
/// so the actions sit below the explanation they belong to.
///
/// # Arguments
///
/// - `VirtualNode<EuvEmptyContentProps>` - The props node carrying the
///   settings of the content.
///
/// # Returns
///
/// - `VirtualNode` - The euv empty content virtual DOM tree.
#[component]
pub fn euv_empty_content(node: VirtualNode<EuvEmptyContentProps>) -> VirtualNode {
    let EuvEmptyContentProps {}: EuvEmptyContentProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "empty-content"
            class: c_euv_empty_content()
            children
        }
    }
}
