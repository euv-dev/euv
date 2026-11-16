use super::*;

/// Props for the `euv_card` component.
///
/// Defines the strongly-typed interface for the card wrapper.
#[derive(Clone, Data, Debug, Default, New)]
pub struct EuvCardProps {
    /// The card title displayed in the header.
    pub title: ::euv::CowText,
    /// The heading level the title is rendered at.
    ///
    /// The level is a setting of the component because the library cannot
    /// know the outline of the page the card sits in: the same card is the
    /// section of one page and the whole panel of another.
    #[get(type(copy))]
    pub level: Option<EuvHeadingLevel>,
}

/// Props for the `euv_card_header` component.
///
/// The header groups the title, the description and the action, so it
/// takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvCardHeaderProps {}

/// Props for the `euv_card_title` component.
///
/// The title names the card; the text is the fallback used when the
/// title is given no children.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvCardTitleProps {
    /// The title text.
    pub text: ::euv::CowText,
    /// The heading level the title is rendered at.
    ///
    /// The level is a setting of the component because the library cannot
    /// know the outline of the page the card sits in: the same card is the
    /// section of one page and the whole panel of another.
    #[get(type(copy))]
    pub level: Option<EuvHeadingLevel>,
}

/// Props for the `euv_card_description` component.
///
/// The description explains the card; the text is the fallback used when
/// the description is given no children.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvCardDescriptionProps {
    /// The description text.
    pub text: ::euv::CowText,
}

/// Props for the `euv_card_content` component.
///
/// The content holds the body of the card, so it takes no settings of
/// its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvCardContentProps {}

/// Props for the `euv_card_footer` component.
///
/// The footer holds the closing row of the card, so it takes no settings
/// of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvCardFooterProps {}

/// Props for the `euv_card_action` component.
///
/// The action holds the control that acts on the card, so it takes no
/// settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvCardActionProps {}
