use super::*;

/// Win32 `RECT` ABI struct for `AdjustWindowRectEx` and `GetClientRect`.
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub(crate) struct Win32Rect {
    /// Left X coordinate.
    pub(crate) left: i32,
    /// Top Y coordinate.
    pub(crate) top: i32,
    /// Right X coordinate.
    pub(crate) right: i32,
    /// Bottom Y coordinate.
    pub(crate) bottom: i32,
}

/// Queued Win32 window event tagging its source `HWND`.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Win32QueuedEvent {
    /// Originating Win32 `HWND`.
    pub(crate) hwnd: usize,
    /// Translated event payload.
    pub(crate) payload: Win32EventPayload,
}

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

/// Win32 `SYSTEM_POWER_STATUS` ABI struct for `GetSystemPowerStatus`.
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub(crate) struct Win32SystemPowerStatus {
    /// AC power status (`0` = offline, `1` = online, `255` = unknown).
    pub(crate) ac_line_status: u8,
    /// Battery charge flag bitmask.
    pub(crate) battery_flag: u8,
    /// Percentage of full battery charge remaining (`0..=100`, `255` = unknown).
    pub(crate) battery_life_percent: u8,
    /// Battery saver status (`1` = Battery Saver active on Windows 10+).
    pub(crate) system_status_flag: u8,
    /// Remaining battery lifetime in seconds (`u32::MAX` = unknown).
    pub(crate) battery_life_time: u32,
    /// Full battery lifetime in seconds (`u32::MAX` = unknown).
    pub(crate) battery_full_life_time: u32,
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
    /// Whether the Win32 frame pump is paused for background freeze.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    pub(crate) pump_paused: bool,
}
