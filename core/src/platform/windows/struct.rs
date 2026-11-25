use super::*;

/// Win32 `WNDCLASSEXW` ABI struct.
#[repr(C)]
pub(crate) struct WndClassExW {
    /// Structure byte size.
    pub(crate) cb_size: u32,
    /// Class style flags.
    pub(crate) style: u32,
    /// Window procedure function pointer.
    pub(crate) lpfn_wnd_proc: unsafe extern "system" fn(isize, u32, usize, isize) -> isize,
    /// Extra class bytes.
    pub(crate) cb_cls_extra: i32,
    /// Extra window bytes.
    pub(crate) cb_wnd_extra: i32,
    /// Module instance handle.
    pub(crate) h_instance: isize,
    /// Class icon handle.
    pub(crate) h_icon: isize,
    /// Class cursor handle.
    pub(crate) h_cursor: isize,
    /// Background brush handle.
    pub(crate) hbr_background: isize,
    /// Optional menu name.
    pub(crate) lpsz_menu_name: *const u16,
    /// Wide class name pointer.
    pub(crate) lpsz_class_name: *const u16,
    /// Small icon handle.
    pub(crate) h_icon_sm: isize,
}

/// Win32 `BITMAPINFOHEADER` ABI struct for `StretchDIBits`.
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub(crate) struct BitmapInfoHeader {
    /// Header size in bytes (`40`).
    pub(crate) bi_size: u32,
    /// Image width in pixels.
    pub(crate) bi_width: i32,
    /// Negative height indicates top-down scanline order.
    pub(crate) bi_height: i32,
    /// Color planes (`1`).
    pub(crate) bi_planes: u16,
    /// Bits per pixel (`32`).
    pub(crate) bi_bit_count: u16,
    /// Compression format (`BI_RGB`).
    pub(crate) bi_compression: u32,
    /// Image size in bytes.
    pub(crate) bi_size_image: u32,
    /// Horizontal pixels per meter.
    pub(crate) bi_x_pels_per_meter: i32,
    /// Vertical pixels per meter.
    pub(crate) bi_y_pels_per_meter: i32,
    /// Used palette colors.
    pub(crate) bi_clr_used: u32,
    /// Important palette colors.
    pub(crate) bi_clr_important: u32,
}

/// Win32 `BITMAPINFO` wrapper struct.
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub(crate) struct BitmapInfo {
    /// Header specification.
    pub(crate) header: BitmapInfoHeader,
    /// Single color entry placeholder.
    pub(crate) colors: [u32; 1],
}

/// Win32 `MSG` ABI struct.
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub(crate) struct Win32Msg {
    /// Target window handle.
    pub(crate) hwnd: isize,
    /// Message identifier.
    pub(crate) message: u32,
    /// Word parameter.
    pub(crate) w_param: usize,
    /// Long parameter.
    pub(crate) l_param: isize,
    /// Timestamp.
    pub(crate) time: u32,
    /// Cursor X.
    pub(crate) pt_x: i32,
    /// Cursor Y.
    pub(crate) pt_y: i32,
}

/// Native Win32 window record storing `HWND` and `HINSTANCE`.
#[derive(Clone, Data, Debug)]
pub struct WindowsWindowRecord {
    /// Window configuration.
    pub(crate) config: WindowConfig,
    /// Live window state and DPI metrics.
    #[get(type(copy))]
    pub(crate) state: WindowState,
    /// Win32 `HWND`.
    #[get(type(copy))]
    pub(crate) hwnd: usize,
    /// Win32 `HINSTANCE`.
    #[get(type(copy))]
    pub(crate) hinstance: usize,
}

/// Direct Windows Win32 / GDI32 / Direct3D platform backend.
#[derive(Clone, Data, Debug)]
pub struct WindowsPlatform {
    /// Monotonic window handle generator.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) next_id: u64,
    /// Whether the Win32 window class has been registered.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) class_registered: bool,
    /// Active Win32 windows.
    pub(crate) windows: HashMap<WindowId, WindowsWindowRecord>,
    /// Queued window and input events.
    pub(crate) pending_events: Vec<WindowEvent>,
    /// Clipboard cache.
    #[get(skip)]
    pub(crate) clipboard_cache: Option<String>,
}
