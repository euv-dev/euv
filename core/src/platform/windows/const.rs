/// Win32 window class name registered for euv native windows.
pub(crate) const WIN32_CLASS_NAME: &str = "EuvNativeWindowClass";

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

/// Win32 `PM_REMOVE` flag for `PeekMessageW`.
pub(crate) const PM_REMOVE: u32 = 0x0001;

/// Win32 `WM_CLOSE` message constant.
pub(crate) const WM_CLOSE: u32 = 0x0010;

/// Win32 default 96 DPI baseline.
pub(crate) const WIN32_DEFAULT_DPI: f32 = 96.0;

/// Win32 `CW_USEDEFAULT` position sentinel.
pub(crate) const CW_USEDEFAULT: i32 = -2_147_483_648;

/// Error message when `CreateWindowExW` returns a null `HWND`.
pub(crate) const ERR_WIN32_CREATE_WINDOW: &str = "CreateWindowExW returned null HWND";
