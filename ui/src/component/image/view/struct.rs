use super::*;

/// Props for the [`euv_image`] component.
///
/// Defines the strongly-typed interface for a picture: where it comes
/// from, what it says, the box it should occupy, and the text shown
/// while it loads or if it fails.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvImageProps {
    /// The image source URL.
    pub src: &'static str,
    /// The alternative text, read by screen readers.
    pub alt: &'static str,
    /// The text shown while the picture loads and when it fails.
    ///
    /// An empty string renders the picture with no placeholder.
    pub placeholder: &'static str,
    /// The explicit width in pixels, when the caller wants one.
    pub width: Option<u32>,
    /// The explicit height in pixels, when the caller wants one.
    pub height: Option<u32>,
}
