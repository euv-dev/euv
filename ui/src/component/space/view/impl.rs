use super::*;

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
