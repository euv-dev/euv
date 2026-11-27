use super::*;

/// Android NDK `ANativeWindow_Buffer` ABI struct.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub(crate) struct ANativeWindowBuffer {
    /// Pixel width of locked buffer.
    pub(crate) width: i32,
    /// Pixel height of locked buffer.
    pub(crate) height: i32,
    /// Row stride in pixels (may exceed `width`).
    pub(crate) stride: i32,
    /// Pixel format discriminant (`ANDROID_WINDOW_FORMAT_RGBA_8888`).
    pub(crate) format: i32,
    /// Mutable pointer to locked pixel memory.
    pub(crate) bits: *mut c_void,
    /// NDK reserved padding words.
    pub(crate) reserved: [u32; 6],
}

/// Native Android NDK window record bound to an `ANativeWindow*`.
#[derive(Clone, Data, Debug)]
pub struct AndroidWindowRecord {
    /// Declarative window configuration.
    pub(crate) config: WindowConfig,
    /// Live window state and cutout safe-area metrics.
    #[get(type(copy))]
    pub(crate) state: WindowState,
    /// Raw `ANativeWindow*` pointer provided by `NativeActivity` / `SurfaceView`.
    #[get(type(copy))]
    pub(crate) native_window: usize,
}

/// Direct Android NDK (`ANativeWindow` / `AInput` / Vulkan / OpenGL ES) platform backend.
#[derive(Clone, Data, Debug)]
pub struct AndroidPlatform {
    /// Monotonic window handle generator.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) next_id: u64,
    /// Attached `ANativeWindow*` pointer supplied by the Android NDK lifecycle callback.
    #[get(type(copy))]
    pub(crate) attached_native_window: usize,
    /// Detected display density scale factor (`dpi / 160.0`).
    #[get(type(copy))]
    pub(crate) density_scale: f32,
    /// Active Android surface window records.
    pub(crate) windows: HashMap<WindowId, AndroidWindowRecord>,
    /// Queued touch, key, IME, and lifecycle events.
    pub(crate) pending_events: Vec<WindowEvent>,
    /// Clipboard cache synchronized through JNI bridge.
    #[get(skip)]
    pub(crate) clipboard_cache: Option<String>,
    /// Whether the Android Choreographer / render pump is paused for background freeze.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    pub(crate) pump_paused: bool,
}
