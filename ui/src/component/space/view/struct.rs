use super::*;

/// The spacing step of the `euv_space` component.
///
/// Each step maps to a design token rather than a hardcoded pixel value, so
/// the gap follows the theme's spacing scale.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EuvSpaceSize {
    /// The tightest step, for separating an icon from its label.
    Xs,
    /// A small step, for list items and inline groups.
    Sm,
    /// The default step, for sibling blocks inside a section.
    #[default]
    Md,
    /// A large step, for separating section groups.
    Lg,
    /// The largest step, for separating top-level page regions.
    Xl,
}

impl EuvSpaceSize {
    /// Returns the name of the design token backing this step.
    ///
    /// The returned name is the token key without the `--` prefix, ready to be
    /// passed to [`var!`]; the caller resolves it to a `var(--token)`
    /// reference rather than hardcoding a pixel value.
    ///
    /// # Returns
    ///
    /// - `&'static str` - The design token name: `space-xs`, `space-sm`,
    ///   `space-md`, `space-lg`, or `space-xl`.
    pub fn token(&self) -> &'static str {
        match self {
            EuvSpaceSize::Xs => "space-xs",
            EuvSpaceSize::Sm => "space-sm",
            EuvSpaceSize::Md => "space-md",
            EuvSpaceSize::Lg => "space-lg",
            EuvSpaceSize::Xl => "space-xl",
        }
    }
}

/// Props for the `euv_space` component.
///
/// Defines the strongly-typed interface for the layout gap primitive.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvSpaceProps {
    /// The spacing step taken from the design token scale.
    pub size: EuvSpaceSize,
    /// Whether the gap is applied vertically (`margin-top`) instead of
    /// horizontally (`margin-left`).
    pub vertical: Signal<bool>,
}
