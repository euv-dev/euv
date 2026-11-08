use super::*;

/// How far along an attachment is.
///
/// The reference publishes five: the empty slot that waits for a file,
/// the upload that is running, the processing that follows it, the
/// failure, and the finished file.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum EuvAttachmentState {
    /// The slot is waiting for a file.
    Idle,
    /// The file is being uploaded.
    Uploading,
    /// The file is being processed.
    Processing,
    /// The upload failed.
    Error,
    /// The file is ready.
    #[default]
    Done,
}

/// The size of an attachment.
///
/// The reference publishes three: the default chip, the small one and the
/// extra small one, whose thumbnail is one step under the small one.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum EuvAttachmentSize {
    /// The chip stands at the default size.
    #[default]
    Default,
    /// The chip stands at the small size.
    Sm,
    /// The chip stands at the extra small size.
    Xs,
}

/// The direction the parts of an attachment are laid out in.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum EuvAttachmentOrientation {
    /// The thumbnail sits beside the text.
    #[default]
    Horizontal,
    /// The thumbnail sits above the text.
    Vertical,
}

/// The kind of media an attachment leads with.
///
/// The reference publishes two: the icon that stands for a file, and the
/// thumbnail of an image.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum EuvAttachmentMediaVariant {
    /// The media is an icon standing for the file.
    #[default]
    Icon,
    /// The media is the thumbnail of an image.
    Image,
}
