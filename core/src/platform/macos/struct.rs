use super::*;

/// CoreGraphics 2D point ABI layout matching `CGPoint`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct CgPoint {
    /// Horizontal coordinate.
    pub(crate) x: f64,
    /// Vertical coordinate.
    pub(crate) y: f64,
}

/// CoreGraphics 2D size ABI layout matching `CGSize`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct CgSize {
    /// Width in points.
    pub(crate) width: f64,
    /// Height in points.
    pub(crate) height: f64,
}

/// CoreGraphics rectangle ABI layout matching `CGRect`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct CgRect {
    /// Rectangle origin.
    pub(crate) origin: CgPoint,
    /// Rectangle size.
    pub(crate) size: CgSize,
}

/// CoreFoundation timer context ABI layout matching `CFRunLoopTimerContext`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub(crate) struct CfRunLoopTimerContext {
    /// Structure version (`0`).
    pub(crate) version: isize,
    /// Opaque user info pointer forwarded to timer callout.
    pub(crate) info: *mut c_void,
    /// Optional retain callback (`null`).
    pub(crate) retain: *const c_void,
    /// Optional release callback (`null`).
    pub(crate) release: *const c_void,
    /// Optional copyDescription callback (`null`).
    pub(crate) copy_description: *const c_void,
}

/// Native macOS window record storing direct Cocoa/AppKit/QuartzCore pointers.
#[derive(Clone, Data, Debug)]
pub struct MacOsWindowRecord {
    /// Active window configuration.
    pub(crate) config: WindowConfig,
    /// Live window state and metrics.
    #[get(type(copy))]
    pub(crate) state: WindowState,
    /// Raw `NSWindow*` pointer.
    #[get(type(copy))]
    pub(crate) ns_window: usize,
    /// Raw `NSView*` content view pointer.
    #[get(type(copy))]
    pub(crate) ns_view: usize,
    /// Raw `CAMetalLayer*` pointer attached to `ns_view`.
    #[get(type(copy))]
    pub(crate) metal_layer: usize,
}

/// Direct macOS Cocoa / AppKit / CoreGraphics / Metal platform backend.
#[derive(Clone, Data, Debug)]
pub struct MacOsPlatform {
    /// Monotonic window handle generator.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) next_id: u64,
    /// Open native macOS windows.
    pub(crate) windows: HashMap<WindowId, MacOsWindowRecord>,
    /// Queued window and input events.
    pub(crate) pending_events: Vec<WindowEvent>,
    /// Local clipboard cache synchronized with pasteboard operations.
    #[get(skip)]
    pub(crate) clipboard_cache: Option<String>,
    /// Cached `CGColorSpaceRef` reused across all frame presentations.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    pub(crate) cached_color_space: usize,
}
