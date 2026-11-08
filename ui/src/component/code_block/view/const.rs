/// How long the copy button says it copied the snippet, in
/// milliseconds.
///
/// The reference keeps the confirmation on screen for two seconds,
/// which is long enough to read and short enough that the button does
/// not look stuck.
pub(crate) const COPY_FEEDBACK_MILLIS: u32 = 2000;

/// The name of the default code block variant.
pub(crate) const CODE_BLOCK_VARIANT_DEFAULT: &str = "default";

/// The name of the default code block size.
pub(crate) const CODE_BLOCK_SIZE_DEFAULT: &str = "default";

/// The name of the default code block line variant.
pub(crate) const CODE_BLOCK_LINE_VARIANT_DEFAULT: &str = "default";
