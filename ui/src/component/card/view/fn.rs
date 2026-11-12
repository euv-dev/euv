use super::*;

/// A card component that wraps children with a styled container.
///
/// The card is the surface the reference publishes: a bordered panel
/// with a header, a body and a closing row. The `title` prop still
/// works — it fills the header, and the header carries the title.
///
/// # Arguments
///
/// - `VirtualNode<EuvCardProps>` - The props node containing title.
///
/// # Returns
///
/// - `VirtualNode` - A styled card element.
#[component]
pub fn euv_card(node: VirtualNode<EuvCardProps>) -> VirtualNode {
    let EuvCardProps { title }: EuvCardProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    #[cfg(feature = "theme-shadcn")]
    let card_class: String = format!("{} {}", c_card().get_name(), c_card_shadcn().get_name());
    #[cfg(not(feature = "theme-shadcn"))]
    let card_class: String = c_card().get_name().to_string();
    html! {
        div {
            class: card_class
            "data-slot": "card"
            euv_card_header {
                euv_card_title {
                    text: title
                }
            }
            children
        }
    }
}

/// The head of a card.
///
/// The header holds the title, the description and the action, so the
/// three sit together and the reader finds them in one place.
///
/// # Arguments
///
/// - `VirtualNode<EuvCardHeaderProps>` - The props node carrying the
///   component settings.
///
/// # Returns
///
/// - `VirtualNode` - The card header virtual DOM tree.
#[component]
pub fn euv_card_header(node: VirtualNode<EuvCardHeaderProps>) -> VirtualNode {
    let EuvCardHeaderProps {}: EuvCardHeaderProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            class: c_euv_card_header()
            "data-slot": "card-header"
            children
        }
    }
}

/// The name of a card.
///
/// The title falls back to its own text when the caller gives it no
/// children, so a card can be titled either way.
///
/// # Arguments
///
/// - `VirtualNode<EuvCardTitleProps>` - The props node carrying the
///   component settings.
///
/// # Returns
///
/// - `VirtualNode` - The card title virtual DOM tree.
#[component]
pub fn euv_card_title(node: VirtualNode<EuvCardTitleProps>) -> VirtualNode {
    let EuvCardTitleProps { text }: EuvCardTitleProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let has_children: bool = !matches!(&children, VirtualNode::Empty);
    #[cfg(feature = "theme-shadcn")]
    let title_class: String = format!(
        "{} {}",
        c_card_title().get_name(),
        c_card_title_shadcn().get_name()
    );
    #[cfg(not(feature = "theme-shadcn"))]
    let title_class: String = c_card_title().get_name().to_string();
    html! {
        h3 {
            class: title_class
            "data-slot": "card-title"
            if has_children {
                children
            } else {
                text
            }
        }
    }
}

/// The explanation under the name of a card.
///
/// The description falls back to its own text when the caller gives it
/// no children, so a card can be described either way.
///
/// # Arguments
///
/// - `VirtualNode<EuvCardDescriptionProps>` - The props node carrying the
///   component settings.
///
/// # Returns
///
/// - `VirtualNode` - The card description virtual DOM tree.
#[component]
pub fn euv_card_description(node: VirtualNode<EuvCardDescriptionProps>) -> VirtualNode {
    let EuvCardDescriptionProps { text }: EuvCardDescriptionProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let has_children: bool = !matches!(&children, VirtualNode::Empty);
    html! {
        div {
            class: c_euv_card_description()
            "data-slot": "card-description"
            if has_children {
                children
            } else {
                text
            }
        }
    }
}

/// The body of a card.
///
/// The content holds whatever the card is about, so it takes no settings
/// of its own.
///
/// # Arguments
///
/// - `VirtualNode<EuvCardContentProps>` - The props node carrying the
///   component settings.
///
/// # Returns
///
/// - `VirtualNode` - The card content virtual DOM tree.
#[component]
pub fn euv_card_content(node: VirtualNode<EuvCardContentProps>) -> VirtualNode {
    let EuvCardContentProps {}: EuvCardContentProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            class: c_euv_card_content()
            "data-slot": "card-content"
            children
        }
    }
}

/// The closing row of a card.
///
/// The footer holds the actions that end the story of the card, so it
/// takes no settings of its own.
///
/// # Arguments
///
/// - `VirtualNode<EuvCardFooterProps>` - The props node carrying the
///   component settings.
///
/// # Returns
///
/// - `VirtualNode` - The card footer virtual DOM tree.
#[component]
pub fn euv_card_footer(node: VirtualNode<EuvCardFooterProps>) -> VirtualNode {
    let EuvCardFooterProps {}: EuvCardFooterProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            class: c_euv_card_footer()
            "data-slot": "card-footer"
            children
        }
    }
}

/// The control that acts on a card.
///
/// The action sits at the end of the header, so it takes no settings of
/// its own.
///
/// # Arguments
///
/// - `VirtualNode<EuvCardActionProps>` - The props node carrying the
///   component settings.
///
/// # Returns
///
/// - `VirtualNode` - The card action virtual DOM tree.
#[component]
pub fn euv_card_action(node: VirtualNode<EuvCardActionProps>) -> VirtualNode {
    let EuvCardActionProps {}: EuvCardActionProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            class: c_euv_card_action()
            "data-slot": "card-action"
            children
        }
    }
}
