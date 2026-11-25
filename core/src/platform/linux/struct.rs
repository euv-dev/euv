use super::*;

/// Native Linux X11 / Wayland window record.
#[derive(Clone, Data, Debug)]
pub struct LinuxWindowRecord {
    /// Window configuration.
    pub(crate) config: WindowConfig,
    /// Live window state.
    #[get(type(copy))]
    pub(crate) state: WindowState,
    /// True when backed by Wayland; false when backed by X11 Xlib.
    #[get(type(copy))]
    pub(crate) is_wayland: bool,
    /// Raw display handle (`Display*` or `wl_display*`).
    #[get(type(copy))]
    pub(crate) display: usize,
    /// Raw window or surface identifier (`Window` or `wl_surface*`).
    #[get(type(copy))]
    pub(crate) window: u64,
}

/// Direct Linux X11 / Wayland (`dlopen`-loaded) + Vulkan platform backend.
#[derive(Clone, Data, Debug)]
pub struct LinuxPlatform {
    /// Monotonic window handle generator.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) next_id: u64,
    /// Dynamically loaded `libX11` handle if present.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) x11_lib: usize,
    /// Active native Linux windows.
    pub(crate) windows: HashMap<WindowId, LinuxWindowRecord>,
    /// Queued window and input events.
    pub(crate) pending_events: Vec<WindowEvent>,
    /// Clipboard text cache.
    #[get(skip)]
    pub(crate) clipboard_cache: Option<String>,
}
