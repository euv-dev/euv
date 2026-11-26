use super::*;

/// Unified window control interface implemented by all platform backends.
pub trait WindowController {
    /// Applies a single `WindowCommand` to the underlying native or web window.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable window controller.
    /// - `WindowCommand` - Command to apply.
    ///
    /// # Returns
    ///
    /// - `Result<(), PlatformError>` - Success or platform error.
    fn apply_command(&mut self, command: WindowCommand) -> Result<(), PlatformError>;
}
