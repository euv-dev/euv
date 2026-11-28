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

/// Hardware display mode descriptor reported by Android `Display.Mode` and desktop monitors.
#[derive(Clone, Copy, Debug, Default, Getter, New, PartialEq)]
pub struct DisplayModeInfo {
    /// OS display mode identifier (e.g. Android `Display.Mode.getModeId()`).
    #[get(pub(crate), type(copy))]
    pub(crate) mode_id: i32,
    /// Physical mode width in pixels.
    #[get(pub(crate), type(copy))]
    pub(crate) physical_width: u32,
    /// Physical mode height in pixels.
    #[get(pub(crate), type(copy))]
    pub(crate) physical_height: u32,
    /// Nominal hardware refresh rate in Hertz (e.g. `60.0`, `90.0`, `120.0`, `144.0`, `240.0`).
    #[get(pub(crate), type(copy))]
    pub(crate) refresh_rate_hz: f32,
}

/// Resolved VSync frame pacing parameters for high-refresh displays (`CADisplayLink`, `Choreographer`, desktop timers).
#[derive(Clone, Copy, Debug, Default, Getter, New, PartialEq)]
pub struct FramePacingConfig {
    /// Minimum adaptive refresh rate in Hertz (`CAFrameRateRange.minimum`).
    #[get(pub(crate), type(copy))]
    pub(crate) min_hz: f32,
    /// Maximum adaptive refresh rate in Hertz (`CAFrameRateRange.maximum`).
    #[get(pub(crate), type(copy))]
    pub(crate) max_hz: f32,
    /// Preferred target refresh rate in Hertz (`CAFrameRateRange.preferred` / `Surface.setFrameRate`).
    #[get(pub(crate), type(copy))]
    pub(crate) preferred_hz: f32,
    /// Nominal frame duration in seconds (`1.0 / preferred_hz`).
    #[get(pub(crate), type(copy))]
    pub(crate) frame_interval_secs: f32,
    /// Nominal frame duration in nanoseconds (`round(1e9 / preferred_hz)`).
    #[get(pub(crate), type(copy))]
    pub(crate) frame_interval_nanos: u64,
}

/// Platform-side active touch contact tracker normalizing Android `MotionEvent` and iOS `UITouch` cancellation lifecycles.
#[derive(Clone, Data, Debug, Default, PartialEq)]
pub struct TouchPointerTracker {
    /// Ordered set of currently live mapped pointer contact identifiers (`>= 1`).
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) active_pointers: Vec<u64>,
}

/// Generation-stamped mobile host session lifecycle coordinator guarding same-process exit and re-entry races.
#[derive(Clone, Data, Debug, Default, PartialEq)]
pub struct MobileSessionLifecycle {
    /// Monotonic process-wide session generation incremented on every fresh activity or scene launch.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    pub(crate) session_generation: u64,
    /// Identifier of the currently active host activity or scene owning native bridge callbacks.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    pub(crate) active_owner_id: u64,
    /// Monotonic frame-loop generation preventing duplicate `Choreographer` or `CADisplayLink` callbacks.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    pub(crate) loop_generation: u64,
    /// Whether the active session currently has an attached native rendering surface.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    pub(crate) surface_attached: bool,
    /// Whether the active session's VSync render loop is running.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    pub(crate) loop_running: bool,
    /// Whether the active session's runtime `App` is still open and interactive.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    pub(crate) app_running: bool,
    /// Whether the active session is currently background-frozen.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    pub(crate) background_frozen: bool,
    /// Whether the active session received a terminal exit or `Terminate` signal.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    pub(crate) terminated: bool,
    /// Active touch contact tracker cleared automatically on pause, focus loss, and surface teardown.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) touch_tracker: TouchPointerTracker,
}
