/// Win32 window class name registered for euv native windows.
pub(crate) const WIN32_CLASS_NAME: &str = "EuvNativeWindowClass";

/// Win32 `user32.dll` module name for dynamic Per-Monitor DPI awareness lookup.
pub(crate) const WIN32_DLL_USER32: &str = "user32.dll";

/// Win32 dynamic symbol name for `SetProcessDpiAwarenessContext`.
pub(crate) const WIN32_SYM_SET_DPI_CONTEXT: &str = "SetProcessDpiAwarenessContext\0";

/// Win32 `DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2` sentinel (`(HANDLE)-4`).
pub(crate) const DPI_AWARENESS_CONTEXT_PER_MONITOR_V2: isize = -4;

/// Win32 standard overlapped window style (`WS_OVERLAPPEDWINDOW`).
pub(crate) const WS_OVERLAPPEDWINDOW: u32 = 0x00CF_0000;

/// Win32 popup borderless window style (`WS_POPUP`).
pub(crate) const WS_POPUP: u32 = 0x8000_0000;

/// Win32 `SW_SHOW` command constant.
pub(crate) const SW_SHOW: i32 = 5;

/// Win32 `SW_HIDE` command constant.
pub(crate) const SW_HIDE: i32 = 0;

/// Win32 `HWND_TOPMOST` sentinel.
pub(crate) const HWND_TOPMOST: isize = -1;

/// Win32 `HWND_NOTOPMOST` sentinel.
pub(crate) const HWND_NOTOPMOST: isize = -2;

/// Win32 `SWP_NOMOVE | SWP_NOSIZE` flags.
pub(crate) const SWP_NOMOVE_NOSIZE: u32 = 0x0003;

/// Win32 `SWP_NOMOVE | SWP_NOZORDER` flags.
pub(crate) const SWP_NOMOVE_NOZORDER: u32 = 0x0006;

/// Win32 `BI_RGB` uncompressed bitmap compression constant.
pub(crate) const BI_RGB: u32 = 0;

/// Win32 `DIB_RGB_COLORS` color table usage constant.
pub(crate) const DIB_RGB_COLORS: u32 = 0;

/// Win32 `SRCCOPY` raster operation code.
pub(crate) const SRCCOPY: u32 = 0x00CC_0020;

/// Win32 `HALFTONE` high-quality stretch blit mode for GDI `SetStretchBltMode`.
pub(crate) const STRETCH_HALFTONE: i32 = 4;

/// Win32 GDI `GetDeviceCaps` index for vertical display refresh rate in Hertz (`VREFRESH`).
pub(crate) const GDI_VREFRESH: i32 = 116;

/// Win32 `PM_REMOVE` flag for `PeekMessageW`.
pub(crate) const PM_REMOVE: u32 = 0x0001;

/// Win32 `WM_DESTROY` message constant.
pub(crate) const WM_DESTROY: u32 = 0x0002;

/// Win32 `WM_SIZE` message constant.
pub(crate) const WM_SIZE: u32 = 0x0005;

/// Win32 `SIZE_MINIMIZED` `w_param` value for `WM_SIZE`.
pub(crate) const SIZE_MINIMIZED: usize = 1;

/// Win32 `WM_SETFOCUS` message constant.
pub(crate) const WM_SETFOCUS: u32 = 0x0007;

/// Win32 `WM_KILLFOCUS` message constant.
pub(crate) const WM_KILLFOCUS: u32 = 0x0008;

/// Win32 `WM_PAINT` message constant.
pub(crate) const WM_PAINT: u32 = 0x000F;

/// Win32 `WM_CLOSE` message constant.
pub(crate) const WM_CLOSE: u32 = 0x0010;

/// Win32 `WM_SETTINGCHANGE` message constant.
pub(crate) const WM_SETTINGCHANGE: u32 = 0x001A;

/// Win32 `WM_ACTIVATEAPP` message constant.
pub(crate) const WM_ACTIVATEAPP: u32 = 0x001C;

/// Win32 `WM_POWERBROADCAST` message constant.
pub(crate) const WM_POWERBROADCAST: u32 = 0x0218;

/// Win32 `PBT_POWERSTATUSCHANGE` `w_param` value for `WM_POWERBROADCAST`.
pub(crate) const PBT_POWERSTATUSCHANGE: usize = 0x000A;

/// Win32 `WM_THEMECHANGED` message constant.
pub(crate) const WM_THEMECHANGED: u32 = 0x031A;

/// Win32 `WM_CLIPBOARDUPDATE` message constant.
pub(crate) const WM_CLIPBOARDUPDATE: u32 = 0x031D;

/// Win32 `WM_KEYDOWN` message constant.
pub(crate) const WM_KEYDOWN: u32 = 0x0100;

/// Win32 `WM_KEYUP` message constant.
pub(crate) const WM_KEYUP: u32 = 0x0101;

/// Win32 `WM_CHAR` message constant.
pub(crate) const WM_CHAR: u32 = 0x0102;

/// Win32 `WM_MOUSEMOVE` message constant.
pub(crate) const WM_MOUSEMOVE: u32 = 0x0200;

/// Win32 `WM_LBUTTONDOWN` message constant.
pub(crate) const WM_LBUTTONDOWN: u32 = 0x0201;

/// Win32 `WM_LBUTTONUP` message constant.
pub(crate) const WM_LBUTTONUP: u32 = 0x0202;

/// Win32 `WM_RBUTTONDOWN` message constant.
pub(crate) const WM_RBUTTONDOWN: u32 = 0x0204;

/// Win32 `WM_RBUTTONUP` message constant.
pub(crate) const WM_RBUTTONUP: u32 = 0x0205;

/// Win32 `WM_MBUTTONDOWN` message constant.
pub(crate) const WM_MBUTTONDOWN: u32 = 0x0207;

/// Win32 `WM_MBUTTONUP` message constant.
pub(crate) const WM_MBUTTONUP: u32 = 0x0208;

/// Win32 `WM_MOUSEWHEEL` message constant.
pub(crate) const WM_MOUSEWHEEL: u32 = 0x020A;

/// Win32 `WM_DPICHANGED` message constant.
pub(crate) const WM_DPICHANGED: u32 = 0x02E0;

/// Win32 `VK_SHIFT` virtual key code.
pub(crate) const VK_SHIFT: i32 = 0x10;

/// Win32 `VK_CONTROL` virtual key code.
pub(crate) const VK_CONTROL: i32 = 0x11;

/// Win32 `VK_MENU` (Alt) virtual key code.
pub(crate) const VK_MENU: i32 = 0x12;

/// Win32 `VK_LWIN` virtual key code.
pub(crate) const VK_LWIN: i32 = 0x5B;

/// Win32 `VK_RWIN` virtual key code.
pub(crate) const VK_RWIN: i32 = 0x5C;

/// Win32 default 96 DPI baseline.
pub(crate) const WIN32_DEFAULT_DPI: f32 = 96.0;

/// Win32 `CW_USEDEFAULT` position sentinel.
pub(crate) const CW_USEDEFAULT: i32 = -2_147_483_648;

/// Error message when `CreateWindowExW` returns a null `HWND`.
pub(crate) const ERR_WIN32_CREATE_WINDOW: &str = "CreateWindowExW returned null HWND";
