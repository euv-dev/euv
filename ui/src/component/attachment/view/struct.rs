use super::*;

/// Props for the [`euv_attachment`] component.
///
/// The chip carries the state of the file, its size and the direction the
/// parts are laid out in. The media, the text and the actions are passed
/// as children.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvAttachmentProps {
    /// How far along the file is.
    pub state: EuvAttachmentState,
    /// The size of the chip.
    pub size: EuvAttachmentSize,
    /// The direction the parts are laid out in.
    pub orientation: EuvAttachmentOrientation,
}

/// Props for the [`euv_attachment_media`] component.
///
/// The media carries the kind of media and the size of the thumbnail.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvAttachmentMediaProps {
    /// The kind of media the thumbnail holds.
    pub variant: EuvAttachmentMediaVariant,
    /// The size of the thumbnail.
    pub size: EuvAttachmentSize,
}

/// Props for the [`euv_attachment_content`] component.
///
/// The content holds the text of the chip and takes no settings.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvAttachmentContentProps {}

/// Props for the [`euv_attachment_title`] component.
///
/// The title holds the name of the file and takes no settings.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvAttachmentTitleProps {}

/// Props for the [`euv_attachment_description`] component.
///
/// The description holds the line under the name and takes no settings.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvAttachmentDescriptionProps {}

/// Props for the [`euv_attachment_actions`] component.
///
/// The actions row takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvAttachmentActionsProps {}

/// Props for the [`euv_attachment_action`] component.
///
/// The action is a plain button and takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvAttachmentActionProps {}

/// Props for the [`euv_attachment_trigger`] component.
///
/// The trigger covers the chip and takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvAttachmentTriggerProps {}

/// Props for the [`euv_attachment_group`] component.
///
/// The group takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvAttachmentGroupProps {}
