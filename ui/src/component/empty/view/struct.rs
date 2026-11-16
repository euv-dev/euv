use super::*;

/// Props for the [`euv_empty`] component.
///
/// Defines the strongly-typed interface for an empty state: the glyph
/// that marks the emptiness, the headline, the explanation, and an
/// optional action that offers the reader a way out of it.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvEmptyProps {
    /// The optional glyph shown above the headline (skipped when empty).
    pub icon: ::euv::CowText,
    /// The headline naming what is missing.
    pub title: ::euv::CowText,
    /// The optional explanation under the headline (skipped when empty).
    pub desc: ::euv::CowText,
    /// The optional action label (skipped when empty).
    pub action_label: ::euv::CowText,
    /// The callback fired when the action is taken.
    #[debug(skip)]
    pub on_action: Option<Rc<dyn Fn(Event)>>,
}

/// Props for the [`euv_empty_header`] component.
///
/// The header takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvEmptyHeaderProps {}

/// Props for the [`euv_empty_media`] component.
///
/// The media carries the picture that marks the emptiness and the size
/// the picture is drawn at.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvEmptyMediaProps {
    /// The size the picture is drawn at.
    pub variant: EuvEmptyMediaVariant,
}

/// Props for the [`euv_empty_title`] component.
///
/// The title takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvEmptyTitleProps {}

/// Props for the [`euv_empty_description`] component.
///
/// The description takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvEmptyDescriptionProps {}

/// Props for the [`euv_empty_content`] component.
///
/// The content takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvEmptyContentProps {}
