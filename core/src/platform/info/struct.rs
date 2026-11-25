use super::*;

/// Normalized hardware and OS capability flags reported by every platform backend.
#[derive(Clone, Copy, Debug, Default, Eq, Getter, Hash, PartialEq)]
pub struct PlatformCapabilities {
    /// Supports opening and managing multiple independent OS windows simultaneously.
    #[get(type(copy))]
    pub(crate) multi_window: bool,
    /// Supports hover pointer tracking (mouse, trackpad, Apple Pencil hover).
    #[get(type(copy))]
    pub(crate) hover_pointer: bool,
    /// Supports multi-touch gestures.
    #[get(type(copy))]
    pub(crate) touch: bool,
    /// Supports stylus / Apple Pencil pressure input.
    #[get(type(copy))]
    pub(crate) stylus: bool,
    /// Supports physical hardware keyboard input.
    #[get(type(copy))]
    pub(crate) hardware_keyboard: bool,
    /// Reports non-trivial OS safe-area cutouts or gesture bars.
    #[get(type(copy))]
    pub(crate) safe_area_insets: bool,
    /// Supports native GPU hardware acceleration (Metal / Vulkan / D3D / WebGPU / WebGL2).
    #[get(type(copy))]
    pub(crate) hardware_gpu: bool,
    /// Supports direct CPU framebuffer presentation.
    #[get(type(copy))]
    pub(crate) software_blit: bool,
    /// Supports native system clipboard read/write.
    #[get(type(copy))]
    pub(crate) clipboard: bool,
    /// Supports native IME / soft keyboard composition.
    #[get(type(copy))]
    pub(crate) ime: bool,
    /// Supports Apple Dynamic Island hardware cutout presentation (`iOS`).
    #[get(type(copy))]
    pub(crate) dynamic_island: bool,
    /// Supports native Live Activities / ongoing live status surfaces (`iOS`, `iPadOS`, `Android`).
    #[get(type(copy))]
    pub(crate) live_activities: bool,
    /// Supports desktop system tray or menu bar status icons (`macOS`, `Windows`, `Linux`).
    #[get(type(copy))]
    pub(crate) system_tray: bool,
    /// Supports OS biometric authentication (Face ID, Touch ID, BiometricPrompt, Windows Hello, Passkeys).
    #[get(type(copy))]
    pub(crate) biometrics: bool,
    /// Supports tactile haptic feedback pulses.
    #[get(type(copy))]
    pub(crate) haptics: bool,
    /// Supports custom URL protocol / deep link handler registration.
    #[get(type(copy))]
    pub(crate) protocol_handler: bool,
    /// Supports native OS accessibility bridge (NSAccessibility, UIAccessibility, UIA, AT-SPI2, Android, ARIA).
    #[get(type(copy))]
    pub(crate) accessibility_bridge: bool,
}
