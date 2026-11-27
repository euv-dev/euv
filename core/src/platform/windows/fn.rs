use super::*;

#[link(name = "kernel32")]
unsafe extern "system" {
    /// Retrieves a module handle for the specified loaded module.
    ///
    /// # Arguments
    ///
    /// - `*const u16` - Optional UTF-16 module name.
    ///
    /// # Returns
    ///
    /// - `isize` - Module `HMODULE` handle.
    fn GetModuleHandleW(module_name: *const u16) -> isize;
    /// Resolves an exported function pointer from a loaded DLL module.
    ///
    /// # Arguments
    ///
    /// - `*mut c_void` - Module `HMODULE` handle.
    /// - `*const u8` - Null-terminated ASCII export name.
    ///
    /// # Returns
    ///
    /// - `*mut c_void` - Function pointer or null.
    fn GetProcAddress(h_module: *mut c_void, lp_proc_name: *const u8) -> *mut c_void;
    /// Queries Windows AC/battery power and Battery Saver status.
    ///
    /// # Arguments
    ///
    /// - `*mut Win32SystemPowerStatus` - Output power status struct.
    ///
    /// # Returns
    ///
    /// - `i32` - Non-zero on success.
    fn GetSystemPowerStatus(status: *mut Win32SystemPowerStatus) -> i32;
}

#[link(name = "user32")]
unsafe extern "system" {
    /// Registers a Win32 window class.
    ///
    /// # Arguments
    ///
    /// - `*const WndClassExW` - Window class descriptor.
    ///
    /// # Returns
    ///
    /// - `u16` - Class atom.
    fn RegisterClassExW(wc: *const WndClassExW) -> u16;
    /// Computes outer window dimensions required for a requested client rectangle.
    ///
    /// # Arguments
    ///
    /// - `*mut Win32Rect` - In-out client/window rectangle.
    /// - `u32` - Window style flags.
    /// - `i32` - Non-zero when window has a menu bar.
    /// - `u32` - Extended window style flags.
    ///
    /// # Returns
    ///
    /// - `i32` - Non-zero on success.
    fn AdjustWindowRectEx(rect: *mut Win32Rect, style: u32, menu: i32, ex_style: u32) -> i32;
    /// Retrieves a window's client-area bounding rectangle.
    ///
    /// # Arguments
    ///
    /// - `isize` - Window `HWND`.
    /// - `*mut Win32Rect` - Output client rectangle.
    ///
    /// # Returns
    ///
    /// - `i32` - Non-zero on success.
    fn GetClientRect(hwnd: isize, rect: *mut Win32Rect) -> i32;
    /// Retrieves the style flags of a Win32 window.
    ///
    /// # Arguments
    ///
    /// - `isize` - Window `HWND`.
    /// - `i32` - Index (`-16` for `GWL_STYLE`).
    ///
    /// # Returns
    ///
    /// - `i32` - Style flags.
    fn GetWindowLongW(hwnd: isize, index: i32) -> i32;
    /// Marks a window's client rectangle valid after handling `WM_PAINT`.
    ///
    /// # Arguments
    ///
    /// - `isize` - Window `HWND`.
    /// - `*const Win32Rect` - Optional rectangle or null for entire client area.
    ///
    /// # Returns
    ///
    /// - `i32` - Non-zero on success.
    fn ValidateRect(hwnd: isize, rect: *const Win32Rect) -> i32;
    /// Queries the instantaneous pressed state of a virtual key.
    ///
    /// # Arguments
    ///
    /// - `i32` - Win32 `VK_*` key code.
    ///
    /// # Returns
    ///
    /// - `i16` - Key state bitmask (high bit set when pressed).
    fn GetKeyState(virtual_key: i32) -> i16;
    /// Creates a native Win32 window.
    ///
    /// # Arguments
    ///
    /// - `u32` - Extended window style.
    /// - `*const u16` - Window class name.
    /// - `*const u16` - Window title.
    /// - `u32` - Window style.
    /// - `i32` - X position.
    /// - `i32` - Y position.
    /// - `i32` - Outer width.
    /// - `i32` - Outer height.
    /// - `isize` - Parent `HWND`.
    /// - `isize` - Menu `HMENU`.
    /// - `isize` - Module `HINSTANCE`.
    /// - `*mut c_void` - Optional creation parameter.
    ///
    /// # Returns
    ///
    /// - `isize` - Created `HWND` or `0`.
    fn CreateWindowExW(
        ex_style: u32,
        class_name: *const u16,
        window_name: *const u16,
        style: u32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        parent: isize,
        menu: isize,
        instance: isize,
        param: *mut c_void,
    ) -> isize;
    /// Default Win32 window procedure.
    ///
    /// # Arguments
    ///
    /// - `isize` - Window `HWND`.
    /// - `u32` - Message ID.
    /// - `usize` - `WPARAM`.
    /// - `isize` - `LPARAM`.
    ///
    /// # Returns
    ///
    /// - `isize` - `LRESULT`.
    fn DefWindowProcW(hwnd: isize, msg: u32, w_param: usize, l_param: isize) -> isize;
    /// Shows or hides a Win32 window.
    ///
    /// # Arguments
    ///
    /// - `isize` - Window `HWND`.
    /// - `i32` - Show command (`SW_SHOW` / `SW_HIDE`).
    ///
    /// # Returns
    ///
    /// - `i32` - Previous visibility state.
    fn ShowWindow(hwnd: isize, cmd_show: i32) -> i32;
    /// Updates a Win32 window title.
    ///
    /// # Arguments
    ///
    /// - `isize` - Window `HWND`.
    /// - `*const u16` - Null-terminated UTF-16 title.
    ///
    /// # Returns
    ///
    /// - `i32` - Non-zero on success.
    fn SetWindowTextW(hwnd: isize, text: *const u16) -> i32;
    /// Updates Win32 window size, position, or z-order.
    ///
    /// # Arguments
    ///
    /// - `isize` - Window `HWND`.
    /// - `isize` - Z-order target.
    /// - `i32` - X coordinate.
    /// - `i32` - Y coordinate.
    /// - `i32` - Outer width.
    /// - `i32` - Outer height.
    /// - `u32` - `SWP_*` flags.
    ///
    /// # Returns
    ///
    /// - `i32` - Non-zero on success.
    fn SetWindowPos(
        hwnd: isize,
        insert_after: isize,
        x: i32,
        y: i32,
        cx: i32,
        cy: i32,
        flags: u32,
    ) -> i32;
    /// Destroys a Win32 window.
    ///
    /// # Arguments
    ///
    /// - `isize` - Window `HWND`.
    ///
    /// # Returns
    ///
    /// - `i32` - Non-zero on success.
    fn DestroyWindow(hwnd: isize) -> i32;
    /// Acquires a GDI device context for a window client area.
    ///
    /// # Arguments
    ///
    /// - `isize` - Window `HWND`.
    ///
    /// # Returns
    ///
    /// - `isize` - Device context `HDC`.
    fn GetDC(hwnd: isize) -> isize;
    /// Releases a window device context.
    ///
    /// # Arguments
    ///
    /// - `isize` - Window `HWND`.
    /// - `isize` - Device context `HDC`.
    ///
    /// # Returns
    ///
    /// - `i32` - Non-zero if released.
    fn ReleaseDC(hwnd: isize, hdc: isize) -> i32;
    /// Queries per-window DPI on Windows 10+.
    ///
    /// # Arguments
    ///
    /// - `isize` - Window `HWND`.
    ///
    /// # Returns
    ///
    /// - `u32` - Dots per inch (default `96`).
    fn GetDpiForWindow(hwnd: isize) -> u32;
    /// Non-blockingly inspects the thread message queue.
    ///
    /// # Arguments
    ///
    /// - `*mut Win32Msg` - Output message buffer.
    /// - `isize` - Optional target `HWND`.
    /// - `u32` - Minimum message ID.
    /// - `u32` - Maximum message ID.
    /// - `u32` - Removal flags (`PM_REMOVE`).
    ///
    /// # Returns
    ///
    /// - `i32` - Non-zero when a message was retrieved.
    fn PeekMessageW(
        msg: *mut Win32Msg,
        hwnd: isize,
        filter_min: u32,
        filter_max: u32,
        remove_msg: u32,
    ) -> i32;
    /// Translates virtual-key messages into character messages.
    ///
    /// # Arguments
    ///
    /// - `*const Win32Msg` - Retrieved message.
    ///
    /// # Returns
    ///
    /// - `i32` - Non-zero if translated.
    fn TranslateMessage(msg: *const Win32Msg) -> i32;
    /// Dispatches a message to the window's `WndProc`.
    ///
    /// # Arguments
    ///
    /// - `*const Win32Msg` - Retrieved message.
    ///
    /// # Returns
    ///
    /// - `isize` - `LRESULT` from `WndProc`.
    fn DispatchMessageW(msg: *const Win32Msg) -> isize;
    /// Suspends the current thread with zero CPU usage until a new Win32 message enters the thread queue.
    ///
    /// # Returns
    ///
    /// - `i32` - Non-zero on success.
    fn WaitMessage() -> i32;
}

#[link(name = "gdi32")]
unsafe extern "system" {
    /// Sets the bitmap stretching mode on a Win32 device context (e.g. `HALFTONE`).
    ///
    /// # Arguments
    ///
    /// - `isize` - Destination `HDC`.
    /// - `i32` - Stretch mode constant (`STRETCH_HALFTONE`).
    ///
    /// # Returns
    ///
    /// - `i32` - Previous stretch mode or zero on failure.
    fn SetStretchBltMode(hdc: isize, mode: i32) -> i32;
    /// Queries a device-specific capability such as vertical display refresh rate (`VREFRESH`).
    ///
    /// # Arguments
    ///
    /// - `isize` - Target device context `HDC`.
    /// - `i32` - Capability index constant.
    ///
    /// # Returns
    ///
    /// - `i32` - Queried capability value.
    fn GetDeviceCaps(hdc: isize, index: i32) -> i32;
    /// Copies pixel rows from a top-down DIB directly to a Win32 device context.
    ///
    /// # Arguments
    ///
    /// - `isize` - Destination `HDC`.
    /// - `i32` - Destination X.
    /// - `i32` - Destination Y.
    /// - `i32` - Destination width.
    /// - `i32` - Destination height.
    /// - `i32` - Source X.
    /// - `i32` - Source Y.
    /// - `i32` - Source width.
    /// - `i32` - Source height.
    /// - `*const c_void` - Pixel data pointer.
    /// - `*const BitmapInfo` - DIB header descriptor.
    /// - `u32` - Color table usage.
    /// - `u32` - Raster operation (`SRCCOPY`).
    ///
    /// # Returns
    ///
    /// - `i32` - Scanlines copied.
    fn StretchDIBits(
        hdc: isize,
        x_dest: i32,
        y_dest: i32,
        dest_width: i32,
        dest_height: i32,
        x_src: i32,
        y_src: i32,
        src_width: i32,
        src_height: i32,
        bits: *const c_void,
        lpbmi: *const BitmapInfo,
        usage: u32,
        rop: u32,
    ) -> i32;
}

/// Queries the native vertical display refresh rate via Win32 GDI `GetDeviceCaps(..., VREFRESH)`.
///
/// # Arguments
///
/// - `usize` - Target Win32 `HWND` (`0` queries the primary screen DC).
///
/// # Returns
///
/// - `f32` - Native monitor refresh rate in Hertz (`60.0` fallback).
#[must_use]
pub(crate) fn win32_display_refresh_rate_hz(hwnd: usize) -> f32 {
    // SAFETY: Standard Win32 GDI device-context acquisition and release.
    unsafe {
        let hdc: isize = GetDC(hwnd as isize);
        if hdc == 0 {
            return 60.0;
        }
        let hz: i32 = GetDeviceCaps(hdc, GDI_VREFRESH);
        let _: i32 = ReleaseDC(hwnd as isize, hdc);
        if hz >= 24 {
            (hz as f32).clamp(24.0, 480.0)
        } else {
            60.0
        }
    }
}

/// Encodes a Rust `&str` as null-terminated UTF-16 for Win32 `*W` APIs.
///
/// # Arguments
///
/// - `&str` - UTF-8 string slice.
///
/// # Returns
///
/// - `Vec<u16>` - Null-terminated UTF-16 code units.
pub(crate) fn win32_wide(text: &str) -> Vec<u16> {
    let mut wide: Vec<u16> = text.encode_utf16().collect();
    wide.push(0);
    wide
}

/// Enables Per-Monitor-V2 DPI awareness dynamically on Windows 10+ when `SetProcessDpiAwarenessContext` is available.
fn win32_enable_dpi_awareness() {
    let user32_wide: Vec<u16> = win32_wide(WIN32_DLL_USER32);
    // SAFETY: Dynamically resolves optional Windows 10+ DPI API without failing on older runtimes.
    unsafe {
        let user32: isize = GetModuleHandleW(user32_wide.as_ptr());
        if user32 != 0 {
            let sym: *mut c_void =
                GetProcAddress(user32 as *mut c_void, WIN32_SYM_SET_DPI_CONTEXT.as_ptr());
            if !sym.is_null() {
                let func: unsafe extern "system" fn(isize) -> i32 = transmute(sym);
                let _: i32 = func(DPI_AWARENESS_CONTEXT_PER_MONITOR_V2);
            }
        }
    }
}

/// Queries live Win32 keyboard modifier flags using `GetKeyState`.
///
/// # Returns
///
/// - `Modifiers` - Active Shift/Ctrl/Alt/Meta flags.
#[must_use]
fn win32_modifiers() -> Modifiers {
    // SAFETY: Standard Win32 keyboard state query.
    unsafe {
        let shift: bool = GetKeyState(VK_SHIFT) < 0;
        let ctrl: bool = GetKeyState(VK_CONTROL) < 0;
        let alt: bool = GetKeyState(VK_MENU) < 0;
        let meta: bool = GetKeyState(VK_LWIN) < 0 || GetKeyState(VK_RWIN) < 0;
        Modifiers::new(shift, ctrl, alt, meta)
    }
}

/// Maps a Win32 `VK_*` virtual key code into a platform-neutral `KeyCode`.
///
/// # Arguments
///
/// - `u32` - Win32 virtual key code from `w_param`.
///
/// # Returns
///
/// - `KeyCode` - Normalized key code.
#[must_use]
fn win32_vk_to_keycode(vk: u32) -> KeyCode {
    match vk {
        0x08 => KeyCode::Backspace,
        0x09 => KeyCode::Tab,
        0x0D => KeyCode::Enter,
        0x1B => KeyCode::Escape,
        0x20 => KeyCode::Space,
        0x21 => KeyCode::PageUp,
        0x22 => KeyCode::PageDown,
        0x23 => KeyCode::End,
        0x24 => KeyCode::Home,
        0x25 => KeyCode::ArrowLeft,
        0x26 => KeyCode::ArrowUp,
        0x27 => KeyCode::ArrowRight,
        0x28 => KeyCode::ArrowDown,
        0x2E => KeyCode::Delete,
        0x30..=0x39 => KeyCode::Character((vk as u8) as char),
        0x41..=0x5A => KeyCode::Character(((vk as u8) + 32) as char),
        _ => KeyCode::Unknown,
    }
}

/// Pushes a translated Win32 window event into the thread-local queue.
///
/// # Arguments
///
/// - `isize` - Originating `HWND`.
/// - `Win32EventPayload` - Translated event payload.
fn win32_push_event(hwnd: isize, payload: Win32EventPayload) {
    let _: Result<(), _> = WIN32_EVENT_QUEUE.try_with(|q: &RefCell<Vec<Win32QueuedEvent>>| {
        if let Ok(mut guard) = q.try_borrow_mut() {
            guard.push(Win32QueuedEvent {
                hwnd: hwnd as usize,
                payload,
            });
        }
    });
}

/// Native Win32 window procedure translating OS messages into queued cross-platform events.
///
/// # Arguments
///
/// - `isize` - `HWND`.
/// - `u32` - Message ID.
/// - `usize` - `WPARAM`.
/// - `isize` - `LPARAM`.
///
/// # Returns
///
/// - `isize` - `LRESULT`.
pub(crate) unsafe extern "system" fn win32_wnd_proc(
    hwnd: isize,
    msg: u32,
    w_param: usize,
    l_param: isize,
) -> isize {
    match msg {
        WM_CLOSE => {
            win32_push_event(hwnd, Win32EventPayload::Close);
            return 0;
        }
        WM_DESTROY => {
            win32_push_event(hwnd, Win32EventPayload::Destroyed);
        }
        WM_ACTIVATEAPP => {
            win32_push_event(hwnd, Win32EventPayload::Activated(w_param != 0));
        }
        WM_SIZE => {
            if w_param == SIZE_MINIMIZED {
                win32_push_event(hwnd, Win32EventPayload::Minimized(true));
            } else {
                win32_push_event(hwnd, Win32EventPayload::Minimized(false));
                let phys_w: u32 = ((l_param & 0xFFFF) as u32).max(1);
                let phys_h: u32 = (((l_param >> 16) & 0xFFFF) as u32).max(1);
                // SAFETY: Valid `HWND` passed by Win32.
                let dpi: u32 = unsafe { GetDpiForWindow(hwnd) };
                let scale: f32 = if dpi > 0 {
                    (dpi as f32) / WIN32_DEFAULT_DPI
                } else {
                    1.0
                };
                win32_push_event(hwnd, Win32EventPayload::Resized(phys_w, phys_h, scale));
            }
        }
        WM_SETTINGCHANGE | WM_THEMECHANGED => {
            win32_push_event(hwnd, Win32EventPayload::ThemeUpdated);
        }
        WM_CLIPBOARDUPDATE => {
            win32_push_event(hwnd, Win32EventPayload::ClipboardUpdated);
        }
        WM_POWERBROADCAST if w_param == PBT_POWERSTATUSCHANGE => {
            let mut st: Win32SystemPowerStatus = Win32SystemPowerStatus::default();
            // SAFETY: Stack-allocated `Win32SystemPowerStatus` passed to `GetSystemPowerStatus`.
            let ok: i32 = unsafe { GetSystemPowerStatus(&mut st) };
            if ok != 0 {
                let low_power: bool = (st.system_status_flag & 1) != 0;
                win32_push_event(hwnd, Win32EventPayload::PowerStatusChanged(low_power));
            }
        }
        WM_DPICHANGED => {
            let dpi: u32 = (w_param & 0xFFFF) as u32;
            let scale: f32 = if dpi > 0 {
                (dpi as f32) / WIN32_DEFAULT_DPI
            } else {
                1.0
            };
            let mut rect: Win32Rect = Win32Rect::default();
            // SAFETY: Valid `HWND` and stack `Win32Rect`.
            let _: i32 = unsafe { GetClientRect(hwnd, &mut rect) };
            let phys_w: u32 = (rect.right - rect.left).max(1) as u32;
            let phys_h: u32 = (rect.bottom - rect.top).max(1) as u32;
            win32_push_event(hwnd, Win32EventPayload::Resized(phys_w, phys_h, scale));
        }
        WM_PAINT => {
            // SAFETY: Validates client rect so Win32 stops re-sending WM_PAINT in a spin loop.
            let _: i32 = unsafe { ValidateRect(hwnd, null()) };
            win32_push_event(hwnd, Win32EventPayload::Redraw);
            return 0;
        }
        WM_SETFOCUS => {
            win32_push_event(hwnd, Win32EventPayload::Focused(true));
        }
        WM_KILLFOCUS => {
            win32_push_event(hwnd, Win32EventPayload::Focused(false));
        }
        WM_MOUSEMOVE => {
            let x: i32 = (l_param & 0xFFFF) as i16 as i32;
            let y: i32 = ((l_param >> 16) & 0xFFFF) as i16 as i32;
            win32_push_event(
                hwnd,
                Win32EventPayload::PointerMove(x, y, win32_modifiers()),
            );
        }
        WM_LBUTTONDOWN | WM_RBUTTONDOWN | WM_MBUTTONDOWN => {
            let x: i32 = (l_param & 0xFFFF) as i16 as i32;
            let y: i32 = ((l_param >> 16) & 0xFFFF) as i16 as i32;
            let btn: PointerButton = match msg {
                WM_RBUTTONDOWN => PointerButton::Secondary,
                WM_MBUTTONDOWN => PointerButton::Auxiliary,
                _ => PointerButton::Primary,
            };
            win32_push_event(
                hwnd,
                Win32EventPayload::PointerDown(x, y, btn, win32_modifiers()),
            );
        }
        WM_LBUTTONUP | WM_RBUTTONUP | WM_MBUTTONUP => {
            let x: i32 = (l_param & 0xFFFF) as i16 as i32;
            let y: i32 = ((l_param >> 16) & 0xFFFF) as i16 as i32;
            let btn: PointerButton = match msg {
                WM_RBUTTONUP => PointerButton::Secondary,
                WM_MBUTTONUP => PointerButton::Auxiliary,
                _ => PointerButton::Primary,
            };
            win32_push_event(
                hwnd,
                Win32EventPayload::PointerUp(x, y, btn, win32_modifiers()),
            );
        }
        WM_MOUSEWHEEL => {
            let wheel_delta: i16 = ((w_param >> 16) & 0xFFFF) as i16;
            let dy: f32 = -((wheel_delta as f32) / 120.0) * 40.0;
            win32_push_event(hwnd, Win32EventPayload::Scroll(dy));
        }
        WM_KEYDOWN => {
            let repeat: bool = (l_param & (1 << 30)) != 0;
            let code: KeyCode = win32_vk_to_keycode(w_param as u32);
            win32_push_event(
                hwnd,
                Win32EventPayload::KeyDown(code, win32_modifiers(), repeat),
            );
        }
        WM_KEYUP => {
            let code: KeyCode = win32_vk_to_keycode(w_param as u32);
            win32_push_event(hwnd, Win32EventPayload::KeyUp(code, win32_modifiers()));
        }
        WM_CHAR => {
            if let Some(ch) = char::from_u32(w_param as u32)
                && !ch.is_control()
            {
                win32_push_event(hwnd, Win32EventPayload::Text(ch));
            }
        }
        _ => {}
    }
    // SAFETY: Direct delegation to standard Win32 default window procedure.
    unsafe { DefWindowProcW(hwnd, msg, w_param, l_param) }
}

/// Creates a native Win32 window with exact client-area dimensions via `AdjustWindowRectEx` and returns `(hwnd, hinstance, scale_factor)`.
///
/// # Arguments
///
/// - `&WindowConfig` - Declarative window config.
/// - `&mut bool` - Tracks whether the class has already been registered.
///
/// # Returns
///
/// - `Result<(usize, usize, f32), PlatformError>` - Native handles and DPI scale factor.
pub(crate) fn win32_create_window(
    config: &WindowConfig,
    class_registered: &mut bool,
) -> Result<(usize, usize, f32), PlatformError> {
    win32_enable_dpi_awareness();
    let class_wide: Vec<u16> = win32_wide(WIN32_CLASS_NAME);
    let title_wide: Vec<u16> = win32_wide(config.get_title());
    // SAFETY: All pointers reference live UTF-16 vectors for the duration of Win32 calls.
    unsafe {
        let hinstance: isize = GetModuleHandleW(null());
        if !*class_registered {
            let wc: WndClassExW = WndClassExW {
                cb_size: size_of::<WndClassExW>() as u32,
                style: 0,
                lpfn_wnd_proc: win32_wnd_proc,
                cb_cls_extra: 0,
                cb_wnd_extra: 0,
                h_instance: hinstance,
                h_icon: 0,
                h_cursor: 0,
                hbr_background: 0,
                lpsz_menu_name: null(),
                lpsz_class_name: class_wide.as_ptr(),
                h_icon_sm: 0,
            };
            let _: u16 = RegisterClassExW(&wc);
            *class_registered = true;
        }
        let style: u32 = if config.get_decorations() {
            WS_OVERLAPPEDWINDOW
        } else {
            WS_POPUP
        };
        let mut rect: Win32Rect = Win32Rect {
            left: 0,
            top: 0,
            right: config.get_width().max(1) as i32,
            bottom: config.get_height().max(1) as i32,
        };
        let _: i32 = AdjustWindowRectEx(&mut rect, style, 0, 0);
        let outer_w: i32 = (rect.right - rect.left).max(1);
        let outer_h: i32 = (rect.bottom - rect.top).max(1);
        let (x, y): (i32, i32) = config
            .position_value()
            .unwrap_or((CW_USEDEFAULT, CW_USEDEFAULT));
        let hwnd: isize = CreateWindowExW(
            0,
            class_wide.as_ptr(),
            title_wide.as_ptr(),
            style,
            x,
            y,
            outer_w,
            outer_h,
            0,
            0,
            hinstance,
            null_mut(),
        );
        if hwnd == 0 {
            return Err(PlatformError::InitializationFailed(
                ERR_WIN32_CREATE_WINDOW.to_string(),
            ));
        }
        if config.get_always_on_top() {
            let _: i32 = SetWindowPos(hwnd, HWND_TOPMOST, 0, 0, 0, 0, SWP_NOMOVE_NOSIZE);
        }
        if config.get_visible() {
            let _: i32 = ShowWindow(hwnd, SW_SHOW);
        }
        let dpi: u32 = GetDpiForWindow(hwnd);
        let scale: f32 = if dpi > 0 {
            (dpi as f32) / WIN32_DEFAULT_DPI
        } else {
            1.0
        };
        Ok((hwnd as usize, hinstance as usize, scale))
    }
}

/// Blits an RGBA8 `Framebuffer` directly onto a Win32 `HWND` client rectangle via `StretchDIBits`.
///
/// # Arguments
///
/// - `usize` - Win32 `HWND`.
/// - `&Framebuffer` - Source RGBA8 framebuffer.
///
/// # Returns
///
/// - `Result<(), PlatformError>` - Presentation status.
pub(crate) fn win32_present_dib(
    hwnd: usize,
    framebuffer: &Framebuffer,
) -> Result<(), PlatformError> {
    if hwnd == 0 {
        return Ok(());
    }
    let bgra: Vec<u8> = framebuffer.to_bgra_bytes();
    let src_w: i32 = framebuffer.get_width() as i32;
    let src_h: i32 = framebuffer.get_height() as i32;
    if src_w <= 0 || src_h <= 0 {
        return Ok(());
    }
    let info: BitmapInfo = BitmapInfo {
        header: BitmapInfoHeader {
            bi_size: size_of::<BitmapInfoHeader>() as u32,
            bi_width: src_w,
            bi_height: -src_h,
            bi_planes: 1,
            bi_bit_count: 32,
            bi_compression: BI_RGB,
            bi_size_image: bgra.len() as u32,
            bi_x_pels_per_meter: 0,
            bi_y_pels_per_meter: 0,
            bi_clr_used: 0,
            bi_clr_important: 0,
        },
        colors: [0],
    };
    // SAFETY: `bgra` and `info` stay alive across the synchronous `StretchDIBits` call.
    unsafe {
        let mut client: Win32Rect = Win32Rect::default();
        let got_rect: i32 = GetClientRect(hwnd as isize, &mut client);
        let dst_w: i32 = if got_rect != 0 && (client.right - client.left) > 0 {
            client.right - client.left
        } else {
            src_w
        };
        let dst_h: i32 = if got_rect != 0 && (client.bottom - client.top) > 0 {
            client.bottom - client.top
        } else {
            src_h
        };
        let hdc: isize = GetDC(hwnd as isize);
        if hdc != 0 {
            if dst_w != src_w || dst_h != src_h {
                let _: i32 = SetStretchBltMode(hdc, STRETCH_HALFTONE);
            }
            let _: i32 = StretchDIBits(
                hdc,
                0,
                0,
                dst_w,
                dst_h,
                0,
                0,
                src_w,
                src_h,
                bgra.as_ptr().cast::<c_void>(),
                &info,
                DIB_RGB_COLORS,
                SRCCOPY,
            );
            let _: i32 = ReleaseDC(hwnd as isize, hdc);
        }
    }
    Ok(())
}

/// Blocks the thread with zero CPU usage via Win32 `WaitMessage` until an OS message wakes the window.
pub(crate) fn win32_wait_for_message() {
    // SAFETY: Standard Win32 thread message queue blocking wait.
    unsafe {
        let _: i32 = WaitMessage();
    }
}

/// Pumps pending Win32 messages via `PeekMessageW` and drains events captured by `win32_wnd_proc`.
///
/// # Arguments
///
/// - `&mut HashMap<WindowId, WindowsWindowRecord>` - Active Win32 windows map.
/// - `&mut Vec<WindowEvent>` - Event sink receiving translated window events.
pub(crate) fn win32_pump_messages(
    windows: &mut HashMap<WindowId, WindowsWindowRecord>,
    events: &mut Vec<WindowEvent>,
) {
    let mut msg: Win32Msg = Win32Msg::default();
    // SAFETY: Standard non-blocking Win32 message pump.
    unsafe {
        while PeekMessageW(&mut msg, 0, 0, 0, PM_REMOVE) != 0 {
            let _: i32 = TranslateMessage(&msg);
            let _: isize = DispatchMessageW(&msg);
        }
    }
    let queued: Vec<Win32QueuedEvent> = WIN32_EVENT_QUEUE
        .try_with(|q: &RefCell<Vec<Win32QueuedEvent>>| {
            q.try_borrow_mut()
                .map(|mut g: RefMut<'_, Vec<Win32QueuedEvent>>| take(&mut *g))
                .unwrap_or_default()
        })
        .unwrap_or_default();
    for item in queued.into_iter() {
        let mut target: Option<(WindowId, f32)> = None;
        for (wid, rec) in windows.iter() {
            if rec.get_hwnd() == item.hwnd {
                target = Some((*wid, rec.get_state().get_scale_factor()));
                break;
            }
        }
        let Some((wid, scale)) = target else {
            continue;
        };
        match item.payload {
            Win32EventPayload::Close => {
                events.push(WindowEvent::CloseRequested(wid));
            }
            Win32EventPayload::Resized(phys_w, phys_h, new_scale) => {
                let s: f32 = new_scale.clamp(1.0, 4.0);
                let exact_lw: f32 = ((phys_w as f32) / s).max(1.0);
                let exact_lh: f32 = ((phys_h as f32) / s).max(1.0);
                let log_w: u32 = exact_lw.round().max(1.0) as u32;
                let log_h: u32 = exact_lh.round().max(1.0) as u32;
                if let Some(rec) = windows.get_mut(&wid) {
                    rec.get_mut_config().set_width(log_w);
                    rec.get_mut_config().set_height(log_h);
                    rec.get_mut_state().set_scale_factor(s);
                    rec.get_mut_state().set_logical_width(exact_lw);
                    rec.get_mut_state().set_logical_height(exact_lh);
                    rec.get_mut_state().set_physical_width(phys_w);
                    rec.get_mut_state().set_physical_height(phys_h);
                }
                events.push(WindowEvent::Resized(wid, log_w, log_h, s));
            }
            Win32EventPayload::Redraw => {
                events.push(WindowEvent::RedrawRequested(wid));
            }
            Win32EventPayload::Focused(focused) => {
                if let Some(rec) = windows.get_mut(&wid) {
                    rec.get_mut_state().set_focused(focused);
                }
                events.push(WindowEvent::Focused(wid, focused));
            }
            Win32EventPayload::PointerMove(px, py, mods) => {
                let pos: Point = Point::new((px as f32) / scale, (py as f32) / scale);
                let ev: PointerEvent = PointerEvent::new(
                    1,
                    PointerDevice::Mouse,
                    pos,
                    PointerButton::Primary,
                    0.0,
                    mods,
                );
                events.push(WindowEvent::PointerMove(wid, ev));
            }
            Win32EventPayload::PointerDown(px, py, btn, mods) => {
                let pos: Point = Point::new((px as f32) / scale, (py as f32) / scale);
                let ev: PointerEvent =
                    PointerEvent::new(1, PointerDevice::Mouse, pos, btn, 1.0, mods);
                events.push(WindowEvent::PointerDown(wid, ev));
            }
            Win32EventPayload::PointerUp(px, py, btn, mods) => {
                let pos: Point = Point::new((px as f32) / scale, (py as f32) / scale);
                let ev: PointerEvent =
                    PointerEvent::new(1, PointerDevice::Mouse, pos, btn, 0.0, mods);
                events.push(WindowEvent::PointerUp(wid, ev));
            }
            Win32EventPayload::Scroll(dy) => {
                events.push(WindowEvent::Scroll(wid, Point::new(0.0, dy)));
            }
            Win32EventPayload::KeyDown(code, mods, repeat) => {
                events.push(WindowEvent::KeyDown(wid, KeyEvent::new(code, mods, repeat)));
            }
            Win32EventPayload::KeyUp(code, mods) => {
                events.push(WindowEvent::KeyUp(wid, KeyEvent::new(code, mods, false)));
            }
            Win32EventPayload::Text(ch) => {
                events.push(WindowEvent::TextInput(wid, ch.to_string()));
            }
            Win32EventPayload::Activated(active) => {
                let ev: SystemEvent = if active {
                    SystemEvent::Foreground
                } else {
                    SystemEvent::Inactive
                };
                events.push(WindowEvent::System(ev));
            }
            Win32EventPayload::Minimized(minimized) => {
                let ev: SystemEvent = if minimized {
                    SystemEvent::Background
                } else {
                    SystemEvent::Foreground
                };
                events.push(WindowEvent::System(ev));
            }
            Win32EventPayload::ClipboardUpdated => {
                events.push(WindowEvent::System(SystemEvent::ClipboardChanged));
            }
            Win32EventPayload::ThemeUpdated => {
                events.push(WindowEvent::ThemeChanged(wid, ThemeMode::Light));
            }
            Win32EventPayload::PowerStatusChanged(low_power) => {
                events.push(WindowEvent::System(SystemEvent::LowPowerModeChanged(
                    low_power,
                )));
            }
            Win32EventPayload::Destroyed => {
                events.push(WindowEvent::System(SystemEvent::Terminate));
            }
        }
    }
}

/// Updates a Win32 window title via `SetWindowTextW`.
///
/// # Arguments
///
/// - `usize` - `HWND`.
/// - `&str` - New UTF-8 title.
pub(crate) fn win32_set_title(hwnd: usize, title: &str) {
    if hwnd == 0 {
        return;
    }
    let wide: Vec<u16> = win32_wide(title);
    // SAFETY: `wide` is null-terminated UTF-16 valid for the call duration.
    unsafe {
        let _: i32 = SetWindowTextW(hwnd as isize, wide.as_ptr());
    }
}

/// Resizes a Win32 window via `AdjustWindowRectEx` + `SetWindowPos` so its client area matches `(width, height)`.
///
/// # Arguments
///
/// - `usize` - `HWND`.
/// - `u32` - Desired client width.
/// - `u32` - Desired client height.
pub(crate) fn win32_set_size(hwnd: usize, width: u32, height: u32) {
    if hwnd == 0 {
        return;
    }
    // SAFETY: Valid Win32 `GetWindowLongW`, `AdjustWindowRectEx`, and `SetWindowPos` calls.
    unsafe {
        let style: u32 = GetWindowLongW(hwnd as isize, -16) as u32;
        let mut rect: Win32Rect = Win32Rect {
            left: 0,
            top: 0,
            right: width.max(1) as i32,
            bottom: height.max(1) as i32,
        };
        let _: i32 = AdjustWindowRectEx(&mut rect, style, 0, 0);
        let _: i32 = SetWindowPos(
            hwnd as isize,
            0,
            0,
            0,
            (rect.right - rect.left).max(1),
            (rect.bottom - rect.top).max(1),
            SWP_NOMOVE_NOZORDER,
        );
    }
}

/// Toggles Win32 always-on-top z-order via `SetWindowPos`.
///
/// # Arguments
///
/// - `usize` - `HWND`.
/// - `bool` - Always-on-top flag.
pub(crate) fn win32_set_topmost(hwnd: usize, on_top: bool) {
    if hwnd == 0 {
        return;
    }
    let z: isize = if on_top { HWND_TOPMOST } else { HWND_NOTOPMOST };
    // SAFETY: Valid Win32 `SetWindowPos` call.
    unsafe {
        let _: i32 = SetWindowPos(hwnd as isize, z, 0, 0, 0, 0, SWP_NOMOVE_NOSIZE);
    }
}

/// Shows or hides a Win32 window via `ShowWindow`.
///
/// # Arguments
///
/// - `usize` - `HWND`.
/// - `bool` - Visibility flag.
pub(crate) fn win32_set_visible(hwnd: usize, visible: bool) {
    if hwnd == 0 {
        return;
    }
    // SAFETY: Valid Win32 `ShowWindow` call.
    unsafe {
        let _: i32 = ShowWindow(hwnd as isize, if visible { SW_SHOW } else { SW_HIDE });
    }
}

/// Destroys a Win32 window via `DestroyWindow`.
///
/// # Arguments
///
/// - `usize` - `HWND`.
pub(crate) fn win32_destroy(hwnd: usize) {
    if hwnd == 0 {
        return;
    }
    // SAFETY: Valid Win32 `DestroyWindow` call.
    unsafe {
        let _: i32 = DestroyWindow(hwnd as isize);
    }
}
