use super::*;

#[link(name = "kernel32")]
unsafe extern "system" {
    /// Body of the `GetModuleHandleW` free function.
    ///
    /// # Arguments
    ///
    /// - `*const u16` - A `*const u16` parameter.
    ///
    /// # Returns
    ///
    /// - `isize` - A `isize` value.
    fn GetModuleHandleW(module_name: *const u16) -> isize;
}

#[link(name = "user32")]
unsafe extern "system" {
    /// Body of the `RegisterClassExW` free function.
    ///
    /// # Arguments
    ///
    /// - `*const WndClassExW` - A `*const WndClassExW` parameter.
    ///
    /// # Returns
    ///
    /// - `u16` - A `u16` value.
    fn RegisterClassExW(wc: *const WndClassExW) -> u16;
    /// Body of the `CreateWindowExW` free function.
    ///
    /// # Arguments
    ///
    /// - `u32` - A 32-bit unsigned integer (`u32`).
    /// - `*const u16` - A `*const u16` parameter.
    /// - `*const u16` - A `*const u16` parameter.
    /// - `u32` - A 32-bit unsigned integer (`u32`).
    /// - `i32` - A 32-bit signed integer (`i32`).
    /// - `i32` - A 32-bit signed integer (`i32`).
    /// - `i32` - A 32-bit signed integer (`i32`).
    /// - `i32` - A 32-bit signed integer (`i32`).
    /// - `isize` - A `isize` parameter.
    /// - `isize` - A `isize` parameter.
    /// - `isize` - A `isize` parameter.
    /// - `*mut c_void` - A `*mut c_void` parameter.
    ///
    /// # Returns
    ///
    /// - `isize` - A `isize` value.
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
    /// Body of the `DefWindowProcW` free function.
    ///
    /// # Arguments
    ///
    /// - `isize` - A `isize` parameter.
    /// - `u32` - A 32-bit unsigned integer (`u32`).
    /// - `usize` - A non-negative integer (`usize`).
    /// - `isize` - A `isize` parameter.
    ///
    /// # Returns
    ///
    /// - `isize` - A `isize` value.
    fn DefWindowProcW(hwnd: isize, msg: u32, w_param: usize, l_param: isize) -> isize;
    /// Body of the `ShowWindow` free function.
    ///
    /// # Arguments
    ///
    /// - `isize` - A `isize` parameter.
    /// - `i32` - A 32-bit signed integer (`i32`).
    ///
    /// # Returns
    ///
    /// - `i32` - A 32-bit signed integer.
    fn ShowWindow(hwnd: isize, cmd_show: i32) -> i32;
    /// Body of the `SetWindowTextW` free function.
    ///
    /// # Arguments
    ///
    /// - `isize` - A `isize` parameter.
    /// - `*const u16` - A `*const u16` parameter.
    ///
    /// # Returns
    ///
    /// - `i32` - A 32-bit signed integer.
    fn SetWindowTextW(hwnd: isize, text: *const u16) -> i32;
    /// Body of the `SetWindowPos` free function.
    ///
    /// # Arguments
    ///
    /// - `isize` - A `isize` parameter.
    /// - `isize` - A `isize` parameter.
    /// - `i32` - A 32-bit signed integer (`i32`).
    /// - `i32` - A 32-bit signed integer (`i32`).
    /// - `i32` - A 32-bit signed integer (`i32`).
    /// - `i32` - A 32-bit signed integer (`i32`).
    /// - `u32` - A 32-bit unsigned integer (`u32`).
    ///
    /// # Returns
    ///
    /// - `i32` - A 32-bit signed integer.
    fn SetWindowPos(
        hwnd: isize,
        insert_after: isize,
        x: i32,
        y: i32,
        cx: i32,
        cy: i32,
        flags: u32,
    ) -> i32;
    /// Body of the `DestroyWindow` free function.
    ///
    /// # Arguments
    ///
    /// - `isize` - A `isize` parameter.
    ///
    /// # Returns
    ///
    /// - `i32` - A 32-bit signed integer.
    fn DestroyWindow(hwnd: isize) -> i32;
    /// Body of the `GetDC` free function.
    ///
    /// # Arguments
    ///
    /// - `isize` - A `isize` parameter.
    ///
    /// # Returns
    ///
    /// - `isize` - A `isize` value.
    fn GetDC(hwnd: isize) -> isize;
    /// Body of the `ReleaseDC` free function.
    ///
    /// # Arguments
    ///
    /// - `isize` - A `isize` parameter.
    /// - `isize` - A `isize` parameter.
    ///
    /// # Returns
    ///
    /// - `i32` - A 32-bit signed integer.
    fn ReleaseDC(hwnd: isize, hdc: isize) -> i32;
    /// Body of the `GetDpiForWindow` free function.
    ///
    /// # Arguments
    ///
    /// - `isize` - A `isize` parameter.
    ///
    /// # Returns
    ///
    /// - `u32` - A 32-bit unsigned integer.
    fn GetDpiForWindow(hwnd: isize) -> u32;
    /// Body of the `PeekMessageW` free function.
    ///
    /// # Arguments
    ///
    /// - `*mut Win32Msg` - A `*mut Win32Msg` parameter.
    /// - `isize` - A `isize` parameter.
    /// - `u32` - A 32-bit unsigned integer (`u32`).
    /// - `u32` - A 32-bit unsigned integer (`u32`).
    /// - `u32` - A 32-bit unsigned integer (`u32`).
    ///
    /// # Returns
    ///
    /// - `i32` - A 32-bit signed integer.
    fn PeekMessageW(
        msg: *mut Win32Msg,
        hwnd: isize,
        filter_min: u32,
        filter_max: u32,
        remove_msg: u32,
    ) -> i32;
    /// Body of the `TranslateMessage` free function.
    ///
    /// # Arguments
    ///
    /// - `*const Win32Msg` - A `*const Win32Msg` parameter.
    ///
    /// # Returns
    ///
    /// - `i32` - A 32-bit signed integer.
    fn TranslateMessage(msg: *const Win32Msg) -> i32;
    /// Body of the `DispatchMessageW` free function.
    ///
    /// # Arguments
    ///
    /// - `*const Win32Msg` - A `*const Win32Msg` parameter.
    ///
    /// # Returns
    ///
    /// - `isize` - A `isize` value.
    fn DispatchMessageW(msg: *const Win32Msg) -> isize;
}

#[link(name = "gdi32")]
unsafe extern "system" {
    /// Body of the `StretchDIBits` free function.
    ///
    /// # Arguments
    ///
    /// - `isize` - A `isize` parameter.
    /// - `i32` - A 32-bit signed integer (`i32`).
    /// - `i32` - A 32-bit signed integer (`i32`).
    /// - `i32` - A 32-bit signed integer (`i32`).
    /// - `i32` - A 32-bit signed integer (`i32`).
    /// - `i32` - A 32-bit signed integer (`i32`).
    /// - `i32` - A 32-bit signed integer (`i32`).
    /// - `i32` - A 32-bit signed integer (`i32`).
    /// - `i32` - A 32-bit signed integer (`i32`).
    /// - `*const c_void` - A `*const c_void` parameter.
    /// - `*const BitmapInfo` - A `*const BitmapInfo` parameter.
    /// - `u32` - A 32-bit unsigned integer (`u32`).
    /// - `u32` - A 32-bit unsigned integer (`u32`).
    ///
    /// # Returns
    ///
    /// - `i32` - A 32-bit signed integer.
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

/// Default Win32 window procedure delegating unhandled messages to `DefWindowProcW`.
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
    // SAFETY: Direct delegation to standard Win32 default window procedure.
    unsafe { DefWindowProcW(hwnd, msg, w_param, l_param) }
}

/// Creates a native Win32 window and returns `(hwnd, hinstance, scale_factor)`.
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
            config.get_width() as i32,
            config.get_height() as i32,
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

/// Blits an RGBA8 `Framebuffer` directly onto a Win32 `HWND` via `StretchDIBits`.
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
    let w: i32 = framebuffer.get_width() as i32;
    let h: i32 = framebuffer.get_height() as i32;
    let info: BitmapInfo = BitmapInfo {
        header: BitmapInfoHeader {
            bi_size: size_of::<BitmapInfoHeader>() as u32,
            bi_width: w,
            bi_height: -h,
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
        let hdc: isize = GetDC(hwnd as isize);
        if hdc != 0 {
            let _: i32 = StretchDIBits(
                hdc,
                0,
                0,
                w,
                h,
                0,
                0,
                w,
                h,
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

/// Pumps pending Win32 messages via `PeekMessageW` without blocking.
///
/// # Arguments
///
/// - `&mut Vec<WindowEvent>` - Event sink receiving translated window events.
/// - `Option<WindowId>` - Primary window identifier.
pub(crate) fn win32_pump_messages(events: &mut Vec<WindowEvent>, primary_id: Option<WindowId>) {
    let mut msg: Win32Msg = Win32Msg::default();
    // SAFETY: Standard non-blocking Win32 message pump.
    unsafe {
        while PeekMessageW(&mut msg, 0, 0, 0, PM_REMOVE) != 0 {
            if msg.message == WM_CLOSE
                && let Some(win_id) = primary_id
            {
                events.push(WindowEvent::CloseRequested(win_id));
            }
            let _: i32 = TranslateMessage(&msg);
            let _: isize = DispatchMessageW(&msg);
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

/// Resizes a Win32 window via `SetWindowPos`.
///
/// # Arguments
///
/// - `usize` - `HWND`.
/// - `u32` - Width.
/// - `u32` - Height.
pub(crate) fn win32_set_size(hwnd: usize, width: u32, height: u32) {
    if hwnd == 0 {
        return;
    }
    // SAFETY: Valid Win32 `SetWindowPos` call.
    unsafe {
        let _: i32 = SetWindowPos(
            hwnd as isize,
            0,
            0,
            0,
            width as i32,
            height as i32,
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
