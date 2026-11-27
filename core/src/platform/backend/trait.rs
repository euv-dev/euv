use super::*;

/// Unified system abstraction implemented directly over each OS native API (Web, Android, iOS, iPadOS, macOS, Windows, Linux, Headless).
pub trait PlatformBackend: WindowController {
    /// Returns the concrete `PlatformKind` served by this backend.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Backend reference.
    ///
    /// # Returns
    ///
    /// - `PlatformKind` - Active platform kind.
    fn kind(&self) -> PlatformKind;

    /// Returns the normalized capability matrix for this backend.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Backend reference.
    ///
    /// # Returns
    ///
    /// - `PlatformCapabilities` - Hardware and OS capability flags.
    fn capabilities(&self) -> PlatformCapabilities;

    /// Creates a native or web window configured by `config`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable platform backend.
    /// - `&WindowConfig` - Declarative window configuration.
    ///
    /// # Returns
    ///
    /// - `Result<WindowState, PlatformError>` - Initial live window state on success.
    fn create_window(&mut self, config: &WindowConfig) -> Result<WindowState, PlatformError>;

    /// Returns the latest `WindowState` for `id`, if alive.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Backend reference.
    /// - `WindowId` - Window identifier.
    ///
    /// # Returns
    ///
    /// - `Option<WindowState>` - Live window state.
    fn window_state(&self, id: WindowId) -> Option<WindowState>;

    /// Returns the ABI-stable `RawWindowHandle` for hardware GPU surface binding.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Backend reference.
    /// - `WindowId` - Window identifier.
    ///
    /// # Returns
    ///
    /// - `RawWindowHandle` - Native OS surface pointer bundle.
    fn raw_window_handle(&self, id: WindowId) -> RawWindowHandle;

    /// Drains all pending OS window, input, and lifecycle events since the last poll.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable platform backend.
    ///
    /// # Returns
    ///
    /// - `Vec<WindowEvent>` - Ordered pending events.
    fn poll_events(&mut self) -> Vec<WindowEvent>;

    /// Presents a CPU-rendered RGBA8 `Framebuffer` directly onto the native window surface.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable platform backend.
    /// - `WindowId` - Target window handle.
    /// - `&Framebuffer` - Rendered RGBA8 framebuffer.
    ///
    /// # Returns
    ///
    /// - `Result<(), PlatformError>` - Presentation status.
    fn present_framebuffer(
        &mut self,
        id: WindowId,
        framebuffer: &Framebuffer,
    ) -> Result<(), PlatformError>;

    /// Presents a damaged sub-region of a CPU-rendered RGBA8 `Framebuffer` when supported by the backend.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable platform backend.
    /// - `WindowId` - Target window handle.
    /// - `&Framebuffer` - Rendered RGBA8 framebuffer.
    /// - `DamageRegion` - Damaged pixel region updated this frame.
    ///
    /// # Returns
    ///
    /// - `Result<(), PlatformError>` - Presentation status.
    fn present_framebuffer_region(
        &mut self,
        id: WindowId,
        framebuffer: &Framebuffer,
        _damage: DamageRegion,
    ) -> Result<(), PlatformError> {
        self.present_framebuffer(id, framebuffer)
    }

    /// Reads the current system clipboard text if available.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Backend reference.
    ///
    /// # Returns
    ///
    /// - `Option<String>` - Clipboard UTF-8 contents.
    fn read_clipboard(&self) -> Option<String>;

    /// Writes `text` into the system clipboard.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable platform backend.
    /// - `&str` - UTF-8 text to store.
    ///
    /// # Returns
    ///
    /// - `Result<(), PlatformError>` - Clipboard write status.
    fn write_clipboard(&mut self, text: &str) -> Result<(), PlatformError>;

    /// Queries the current operating system theme mode (`Light` / `Dark`).
    ///
    /// # Arguments
    ///
    /// - `&Self` - Backend reference.
    ///
    /// # Returns
    ///
    /// - `ThemeMode` - Detected OS appearance mode.
    fn system_theme(&self) -> ThemeMode;

    /// Queries the active display's native hardware refresh rate in Hertz (`60.0` fallback).
    ///
    /// # Arguments
    ///
    /// - `&Self` - Backend reference.
    ///
    /// # Returns
    ///
    /// - `f32` - Native display refresh rate in Hertz (e.g. `120.0`, `144.0`, `180.0`).
    fn display_refresh_rate_hz(&self) -> f32 {
        60.0
    }
}
