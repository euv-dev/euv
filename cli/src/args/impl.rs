use super::*;

/// Conversion helpers between CLI arguments and `euv-core` types.
impl RenderArg {
    /// Converts a `RenderArg` into the core `RenderMode` enum.
    ///
    /// # Arguments
    ///
    /// - `Self` - CLI render mode argument.
    ///
    /// # Returns
    ///
    /// - `RenderMode` - Core rendering pipeline mode.
    #[must_use]
    pub const fn to_render_mode(self) -> RenderMode {
        match self {
            Self::Auto => RenderMode::Auto,
            Self::Software => RenderMode::Software,
            Self::Hardware => RenderMode::Hardware,
            Self::Hybrid => RenderMode::Hybrid,
        }
    }

    /// Returns the CLI flag string forwarded to `euv` applications.
    ///
    /// # Arguments
    ///
    /// - `Self` - CLI render mode argument.
    ///
    /// # Returns
    ///
    /// - `&'static str` - Render flag token.
    #[must_use]
    pub const fn as_flag(self) -> &'static str {
        match self {
            Self::Auto => RENDER_FLAG_AUTO,
            Self::Software => RENDER_FLAG_SOFTWARE,
            Self::Hardware => RENDER_FLAG_HARDWARE,
            Self::Hybrid => RENDER_FLAG_HYBRID,
        }
    }
}
