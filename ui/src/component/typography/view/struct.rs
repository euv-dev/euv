use super::*;

/// Props for the [`euv_text`] component.
///
/// Defines the strongly-typed interface for a run of text: the string
/// itself and the typographic role it plays.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvTextProps {
    /// The text to render.
    pub text: &'static str,
    /// The typographic role the text plays.
    pub variant: EuvTextVariant,
}
