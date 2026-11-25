use super::*;

unsafe extern "C" {
    /// Body of the `dlopen` free function.
    ///
    /// # Arguments
    ///
    /// - `*const c_char` - A `*const c_char` parameter.
    /// - `i32` - A 32-bit signed integer (`i32`).
    ///
    /// # Returns
    ///
    /// - `*mut c_void` - A `*mut c_void` value.
    fn dlopen(filename: *const c_char, flag: i32) -> *mut c_void;
    /// Body of the `dlsym` free function.
    ///
    /// # Arguments
    ///
    /// - `*mut c_void` - A `*mut c_void` parameter.
    /// - `*const c_char` - A `*const c_char` parameter.
    ///
    /// # Returns
    ///
    /// - `*mut c_void` - A `*mut c_void` value.
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

/// Loads a symbol pointer from a `dlopen` handle.
///
/// # Arguments
///
/// - `*mut c_void` - Shared library handle.
/// - `&'static str` - Null-terminated symbol name.
///
/// # Returns
///
/// - `*mut c_void` - Resolved symbol pointer, or null.
pub(crate) fn linux_dlsym(handle: *mut c_void, sym: &'static str) -> *mut c_void {
    if handle.is_null() {
        return null_mut();
    }
    // SAFETY: `sym` is a static null-terminated ASCII symbol name.
    unsafe { dlsym(handle, sym.as_ptr().cast::<c_char>()) }
}

/// Attempts to open a native Linux display connection via X11 or Wayland `dlopen`.
///
/// # Arguments
///
/// - `&WindowConfig` - Declarative window config.
///
/// # Returns
///
/// - `Result<(usize, bool, usize, u64), PlatformError>` - Tuple `(lib_handle, is_wayland, display_ptr, window_id)`.
pub(crate) fn linux_create_native_window(
    config: &WindowConfig,
) -> Result<(usize, bool, usize, u64), PlatformError> {
    // SAFETY: Dynamically loads standard Linux display libraries at runtime without link-time dependency.
    unsafe {
        let x11: *mut c_void = dlopen(LINUX_LIB_X11.as_ptr().cast::<c_char>(), RTLD_LAZY);
        if !x11.is_null() {
            let open_sym: *mut c_void = linux_dlsym(x11, SYM_X_OPEN_DISPLAY);
            let screen_sym: *mut c_void = linux_dlsym(x11, SYM_X_DEFAULT_SCREEN);
            let root_sym: *mut c_void = linux_dlsym(x11, SYM_X_ROOT_WINDOW);
            let create_sym: *mut c_void = linux_dlsym(x11, SYM_X_CREATE_SIMPLE_WINDOW);
            let store_sym: *mut c_void = linux_dlsym(x11, SYM_X_STORE_NAME);
            let map_sym: *mut c_void = linux_dlsym(x11, SYM_X_MAP_WINDOW);
            let flush_sym: *mut c_void = linux_dlsym(x11, SYM_X_FLUSH);
            if !open_sym.is_null()
                && !screen_sym.is_null()
                && !root_sym.is_null()
                && !create_sym.is_null()
            {
                let x_open: unsafe extern "C" fn(*const c_char) -> *mut c_void =
                    transmute(open_sym);
                let display: *mut c_void = x_open(null());
                if !display.is_null() {
                    let x_screen: unsafe extern "C" fn(*mut c_void) -> i32 = transmute(screen_sym);
                    let x_root: unsafe extern "C" fn(*mut c_void, i32) -> u64 = transmute(root_sym);
                    let x_create: unsafe extern "C" fn(
                        *mut c_void,
                        u64,
                        i32,
                        i32,
                        u32,
                        u32,
                        u32,
                        u64,
                        u64,
                    ) -> u64 = transmute(create_sym);
                    let screen: i32 = x_screen(display);
                    let root: u64 = x_root(display, screen);
                    let (pos_x, pos_y): (i32, i32) = config.position_value().unwrap_or((0, 0));
                    let win: u64 = x_create(
                        display,
                        root,
                        pos_x,
                        pos_y,
                        config.get_width().max(1),
                        config.get_height().max(1),
                        0,
                        0,
                        0,
                    );
                    if !store_sym.is_null()
                        && let Ok(c_title) = CString::new(config.get_title().as_str())
                    {
                        let x_store: unsafe extern "C" fn(*mut c_void, u64, *const c_char) -> i32 =
                            transmute(store_sym);
                        let _: i32 = x_store(display, win, c_title.as_ptr());
                    }
                    if config.get_visible() && !map_sym.is_null() {
                        let x_map: unsafe extern "C" fn(*mut c_void, u64) -> i32 =
                            transmute(map_sym);
                        let _: i32 = x_map(display, win);
                    }
                    if !flush_sym.is_null() {
                        let x_flush: unsafe extern "C" fn(*mut c_void) -> i32 =
                            transmute(flush_sym);
                        let _: i32 = x_flush(display);
                    }
                    return Ok((x11 as usize, false, display as usize, win));
                }
            }
        }
        let wl: *mut c_void = dlopen(LINUX_LIB_WAYLAND.as_ptr().cast::<c_char>(), RTLD_LAZY);
        if !wl.is_null() {
            let connect_sym: *mut c_void = linux_dlsym(wl, SYM_WL_DISPLAY_CONNECT);
            if !connect_sym.is_null() {
                let wl_connect: unsafe extern "C" fn(*const c_char) -> *mut c_void =
                    transmute(connect_sym);
                let wl_display: *mut c_void = wl_connect(null());
                if !wl_display.is_null() {
                    return Ok((wl as usize, true, wl_display as usize, 0));
                }
            }
        }
    }
    Err(PlatformError::InitializationFailed(
        ERR_LINUX_NO_DISPLAY.to_string(),
    ))
}

/// Updates an X11 window title via dynamically resolved `XStoreName`.
///
/// # Arguments
///
/// - `usize` - Loaded `libX11` handle.
/// - `usize` - X11 `Display*`.
/// - `u64` - X11 `Window`.
/// - `&str` - New title.
pub(crate) fn linux_x11_set_title(lib: usize, display: usize, window: u64, title: &str) {
    if lib == 0 || display == 0 || window == 0 {
        return;
    }
    let sym: *mut c_void = linux_dlsym(lib as *mut c_void, SYM_X_STORE_NAME);
    if sym.is_null() {
        return;
    }
    if let Ok(c_title) = CString::new(title) {
        // SAFETY: Valid `XStoreName` ABI call with live `CString`.
        unsafe {
            let func: unsafe extern "C" fn(*mut c_void, u64, *const c_char) -> i32 = transmute(sym);
            let _: i32 = func(display as *mut c_void, window, c_title.as_ptr());
        }
    }
}

/// Resizes an X11 window via dynamically resolved `XResizeWindow`.
///
/// # Arguments
///
/// - `usize` - Loaded `libX11` handle.
/// - `usize` - X11 `Display*`.
/// - `u64` - X11 `Window`.
/// - `u32` - Width.
/// - `u32` - Height.
pub(crate) fn linux_x11_resize(lib: usize, display: usize, window: u64, width: u32, height: u32) {
    if lib == 0 || display == 0 || window == 0 {
        return;
    }
    let sym: *mut c_void = linux_dlsym(lib as *mut c_void, SYM_X_RESIZE_WINDOW);
    if sym.is_null() {
        return;
    }
    // SAFETY: Valid `XResizeWindow` ABI call.
    unsafe {
        let func: unsafe extern "C" fn(*mut c_void, u64, u32, u32) -> i32 = transmute(sym);
        let _: i32 = func(display as *mut c_void, window, width.max(1), height.max(1));
    }
}

/// Destroys an X11 window via dynamically resolved `XDestroyWindow`.
///
/// # Arguments
///
/// - `usize` - Loaded `libX11` handle.
/// - `usize` - X11 `Display*`.
/// - `u64` - X11 `Window`.
pub(crate) fn linux_x11_destroy(lib: usize, display: usize, window: u64) {
    if lib == 0 || display == 0 || window == 0 {
        return;
    }
    let sym: *mut c_void = linux_dlsym(lib as *mut c_void, SYM_X_DESTROY_WINDOW);
    if sym.is_null() {
        return;
    }
    // SAFETY: Valid `XDestroyWindow` ABI call.
    unsafe {
        let func: unsafe extern "C" fn(*mut c_void, u64) -> i32 = transmute(sym);
        let _: i32 = func(display as *mut c_void, window);
    }
}
