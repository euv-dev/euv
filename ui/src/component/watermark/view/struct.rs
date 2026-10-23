use super::*;

/// Props for the [`euv_watermark`] component.
///
/// Defines the strongly-typed interface for a watermark: the text
/// stamped over the content it wraps.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvWatermarkProps {
    /// The text stamped over the content.
    ///
    /// An empty string renders the content without a watermark.
    pub text: &'static str,
}
