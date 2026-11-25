use super::*;

/// Native iOS / iPadOS UIKit window record supporting iPhone safe areas and iPadOS Stage Manager multi-window.
#[derive(Clone, Data, Debug)]
pub struct IosWindowRecord {
    /// Declarative window configuration.
    pub(crate) config: WindowConfig,
    /// Live window state and safe-area metrics.
    #[get(type(copy))]
    pub(crate) state: WindowState,
    /// Raw `UIWindow*` pointer.
    #[get(type(copy))]
    pub(crate) ui_window: usize,
    /// Raw `UIView*` pointer.
    #[get(type(copy))]
    pub(crate) ui_view: usize,
    /// Raw `CAMetalLayer*` pointer.
    #[get(type(copy))]
    pub(crate) metal_layer: usize,
    /// True when executing on iPadOS (`UIUserInterfaceIdiomPad`).
    #[get(type(copy))]
    pub(crate) is_ipad: bool,
}

/// Direct iOS & iPadOS UIKit / CoreGraphics / Metal platform backend.
#[derive(Clone, Data, Debug)]
pub struct IosPlatform {
    /// Dynamically resolved platform kind (`PlatformKind::Ios` or `PlatformKind::IpadOs`).
    #[get(type(copy))]
    pub(crate) resolved_kind: PlatformKind,
    /// Monotonic window ID counter.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) next_id: u64,
    /// Active UIKit windows (supports multiple scenes on iPadOS Stage Manager / Split View).
    pub(crate) windows: HashMap<WindowId, IosWindowRecord>,
    /// Queued UIKit touch, Apple Pencil, hardware keyboard, and safe-area events.
    pub(crate) pending_events: Vec<WindowEvent>,
    /// Clipboard cache synchronized with `UIPasteboard`.
    #[get(skip)]
    pub(crate) clipboard_cache: Option<String>,
}
