use super::*;

unsafe extern "C" {
    /// Dynamically opens a POSIX shared object.
    ///
    /// # Arguments
    ///
    /// - `*const c_char` - Shared library name.
    /// - `i32` - `RTLD_*` resolution flags.
    ///
    /// # Returns
    ///
    /// - `*mut c_void` - Shared library handle or null.
    fn dlopen(filename: *const c_char, flag: i32) -> *mut c_void;
    /// Resolves a dynamic symbol from an open shared library handle.
    ///
    /// # Arguments
    ///
    /// - `*mut c_void` - Shared library handle.
    /// - `*const c_char` - Null-terminated symbol name.
    ///
    /// # Returns
    ///
    /// - `*mut c_void` - Symbol address or null.
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

/// Queries the Linux display DPI scale factor from environment variables or `XResourceManagerString` (`Xft.dpi`).
///
/// # Arguments
///
/// - `*mut c_void` - Loaded `libX11` handle.
/// - `*mut c_void` - Active X11 `Display*`.
///
/// # Returns
///
/// - `f32` - Clamped DPI scale factor in `[1.0, 4.0]`.
pub(crate) fn linux_x11_query_scale_factor(x11: *mut c_void, display: *mut c_void) -> f32 {
    for var_name in LINUX_DPI_ENV_VARS.iter() {
        if let Ok(val) = env::var(var_name)
            && let Ok(parsed) = val.trim().parse::<f32>()
            && parsed.is_finite()
            && parsed >= 1.0
        {
            return parsed.clamp(1.0, 4.0);
        }
    }
    let rm_sym: *mut c_void = linux_dlsym(x11, SYM_X_RESOURCE_MANAGER_STRING);
    if !rm_sym.is_null() && !display.is_null() {
        // SAFETY: Valid `XResourceManagerString` ABI call returning an owned Xlib string.
        unsafe {
            let x_rm: unsafe extern "C" fn(*mut c_void) -> *const c_char = transmute(rm_sym);
            let raw_ptr: *const c_char = x_rm(display);
            if !raw_ptr.is_null()
                && let Ok(text) = CStr::from_ptr(raw_ptr).to_str()
            {
                for line in text.lines() {
                    let trimmed: &str = line.trim();
                    if let Some(rest) = trimmed.strip_prefix(XFT_DPI_PREFIX)
                        && let Ok(dpi) = rest.trim().parse::<f32>()
                        && dpi.is_finite()
                        && dpi > 48.0
                    {
                        return (dpi / X11_DEFAULT_DPI).clamp(1.0, 4.0);
                    }
                }
            }
        }
    }
    1.0
}

/// Attempts to open a native Linux display connection via X11 or Wayland `dlopen`.
///
/// # Arguments
///
/// - `&WindowConfig` - Declarative window config.
///
/// # Returns
///
/// - `Result<LinuxNativeWindowHandle, PlatformError>` - Resolved display, window, GC, visual, depth, and DPI scale.
pub(crate) fn linux_create_native_window(
    config: &WindowConfig,
) -> Result<LinuxNativeWindowHandle, PlatformError> {
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
            let gc_sym: *mut c_void = linux_dlsym(x11, SYM_X_DEFAULT_GC);
            let visual_sym: *mut c_void = linux_dlsym(x11, SYM_X_DEFAULT_VISUAL);
            let depth_sym: *mut c_void = linux_dlsym(x11, SYM_X_DEFAULT_DEPTH);
            let select_sym: *mut c_void = linux_dlsym(x11, SYM_X_SELECT_INPUT);
            let intern_sym: *mut c_void = linux_dlsym(x11, SYM_X_INTERN_ATOM);
            let proto_sym: *mut c_void = linux_dlsym(x11, SYM_X_SET_WM_PROTOCOLS);
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
                    let scale_factor: f32 = linux_x11_query_scale_factor(x11, display);
                    let phys_w: u32 = ((config.get_width() as f32) * scale_factor)
                        .round()
                        .max(1.0) as u32;
                    let phys_h: u32 = ((config.get_height() as f32) * scale_factor)
                        .round()
                        .max(1.0) as u32;
                    let (pos_x, pos_y): (i32, i32) = config.position_value().unwrap_or((0, 0));
                    let win: u64 = x_create(display, root, pos_x, pos_y, phys_w, phys_h, 0, 0, 0);
                    let gc: usize = if !gc_sym.is_null() {
                        let x_gc: unsafe extern "C" fn(*mut c_void, i32) -> *mut c_void =
                            transmute(gc_sym);
                        x_gc(display, screen) as usize
                    } else {
                        0
                    };
                    let visual: usize = if !visual_sym.is_null() {
                        let x_vis: unsafe extern "C" fn(*mut c_void, i32) -> *mut c_void =
                            transmute(visual_sym);
                        x_vis(display, screen) as usize
                    } else {
                        0
                    };
                    let depth: u32 = if !depth_sym.is_null() {
                        let x_dep: unsafe extern "C" fn(*mut c_void, i32) -> i32 =
                            transmute(depth_sym);
                        x_dep(display, screen).max(24) as u32
                    } else {
                        24
                    };
                    if !select_sym.is_null() {
                        let x_select: unsafe extern "C" fn(*mut c_void, u64, isize) -> i32 =
                            transmute(select_sym);
                        let _: i32 = x_select(display, win, X11_EVENT_MASK);
                    }
                    let mut wm_delete_atom: u64 = 0;
                    if !intern_sym.is_null() && !proto_sym.is_null() {
                        let x_intern: unsafe extern "C" fn(*mut c_void, *const c_char, i32) -> u64 =
                            transmute(intern_sym);
                        let x_proto: unsafe extern "C" fn(*mut c_void, u64, *mut u64, i32) -> i32 =
                            transmute(proto_sym);
                        wm_delete_atom =
                            x_intern(display, ATOM_WM_DELETE_WINDOW.as_ptr().cast::<c_char>(), 0);
                        if wm_delete_atom != 0 {
                            let _: i32 = x_proto(display, win, &mut wm_delete_atom, 1);
                        }
                    }
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
                    return Ok(LinuxNativeWindowHandle {
                        lib: x11 as usize,
                        is_wayland: false,
                        display: display as usize,
                        window: win,
                        gc,
                        visual,
                        depth,
                        wm_delete_atom,
                        scale_factor,
                    });
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
                    return Ok(LinuxNativeWindowHandle {
                        lib: wl as usize,
                        is_wayland: true,
                        display: wl_display as usize,
                        window: 0,
                        gc: 0,
                        visual: 0,
                        depth: 24,
                        wm_delete_atom: 0,
                        scale_factor: 1.0,
                    });
                }
            }
        }
    }
    Err(PlatformError::InitializationFailed(
        ERR_LINUX_NO_DISPLAY.to_string(),
    ))
}

/// Blits an RGBA8 `Framebuffer` directly onto a live X11 `Window` via `XCreateImage` + `XPutImage` + `XFlush`.
///
/// # Arguments
///
/// - `usize` - Loaded `libX11` handle.
/// - `&LinuxWindowRecord` - Active X11 window metadata containing `display`, `window`, `gc`, `visual`, and `depth`.
/// - `&Framebuffer` - Rendered RGBA8 framebuffer.
///
/// # Returns
///
/// - `Result<(), PlatformError>` - Presentation status.
pub(crate) fn linux_x11_present_framebuffer(
    lib: usize,
    rec: &LinuxWindowRecord,
    framebuffer: &Framebuffer,
) -> Result<(), PlatformError> {
    if lib == 0
        || rec.get_is_wayland()
        || rec.get_display() == 0
        || rec.get_window() == 0
        || rec.get_gc() == 0
        || rec.get_visual() == 0
    {
        return Ok(());
    }
    let w: u32 = framebuffer.get_width();
    let h: u32 = framebuffer.get_height();
    if w == 0 || h == 0 {
        return Ok(());
    }
    let x11: *mut c_void = lib as *mut c_void;
    let create_img_sym: *mut c_void = linux_dlsym(x11, SYM_X_CREATE_IMAGE);
    let put_img_sym: *mut c_void = linux_dlsym(x11, SYM_X_PUT_IMAGE);
    let free_sym: *mut c_void = linux_dlsym(x11, SYM_X_FREE);
    let flush_sym: *mut c_void = linux_dlsym(x11, SYM_X_FLUSH);
    if create_img_sym.is_null() || put_img_sym.is_null() {
        return Ok(());
    }
    let mut bgra: Vec<u8> = framebuffer.to_bgra_bytes();
    let stride: i32 = (w as i32) * 4;
    // SAFETY: `bgra` remains alive across `XPutImage`; `XFree` frees only the XImage header without touching `bgra`.
    unsafe {
        let x_create_img: unsafe extern "C" fn(
            *mut c_void,
            *mut c_void,
            u32,
            i32,
            i32,
            *mut c_char,
            u32,
            u32,
            i32,
            i32,
        ) -> *mut c_void = transmute(create_img_sym);
        let x_put_img: unsafe extern "C" fn(
            *mut c_void,
            u64,
            *mut c_void,
            *mut c_void,
            i32,
            i32,
            i32,
            i32,
            u32,
            u32,
        ) -> i32 = transmute(put_img_sym);
        let display: *mut c_void = rec.get_display() as *mut c_void;
        let visual: *mut c_void = rec.get_visual() as *mut c_void;
        let gc: *mut c_void = rec.get_gc() as *mut c_void;
        let img: *mut c_void = x_create_img(
            display,
            visual,
            rec.get_depth(),
            X11_Z_PIXMAP,
            0,
            bgra.as_mut_ptr().cast::<c_char>(),
            w,
            h,
            32,
            stride,
        );
        if !img.is_null() {
            let _: i32 = x_put_img(display, rec.get_window(), gc, img, 0, 0, 0, 0, w, h);
            if !free_sym.is_null() {
                let x_free: unsafe extern "C" fn(*mut c_void) -> i32 = transmute(free_sym);
                let _: i32 = x_free(img);
            }
        }
        if !flush_sym.is_null() {
            let x_flush: unsafe extern "C" fn(*mut c_void) -> i32 = transmute(flush_sym);
            let _: i32 = x_flush(display);
        }
    }
    Ok(())
}

/// Converts an X11 modifier state bitmask into platform-neutral `Modifiers`.
///
/// # Arguments
///
/// - `u32` - X11 `state` bitmask.
///
/// # Returns
///
/// - `Modifiers` - Decoded keyboard modifiers.
#[must_use]
fn linux_x11_modifiers(state: u32) -> Modifiers {
    Modifiers::new(
        (state & X11_SHIFT_MASK) != 0,
        (state & X11_CTRL_MASK) != 0,
        (state & X11_ALT_MASK) != 0,
        (state & X11_META_MASK) != 0,
    )
}

/// Maps an X11 KeySym into a unified `KeyCode`.
///
/// # Arguments
///
/// - `u64` - X11 KeySym value returned by `XLookupString`.
/// - `Option<char>` - Printable character decoded by `XLookupString`.
///
/// # Returns
///
/// - `KeyCode` - Normalized key code.
#[must_use]
fn linux_x11_keysym_to_keycode(keysym: u64, ch: Option<char>) -> KeyCode {
    match keysym {
        0xFF08 => KeyCode::Backspace,
        0xFF09 => KeyCode::Tab,
        0xFF0D | 0xFF8D => KeyCode::Enter,
        0xFF1B => KeyCode::Escape,
        0xFF50 => KeyCode::Home,
        0xFF51 => KeyCode::ArrowLeft,
        0xFF52 => KeyCode::ArrowUp,
        0xFF53 => KeyCode::ArrowRight,
        0xFF54 => KeyCode::ArrowDown,
        0xFF55 => KeyCode::PageUp,
        0xFF56 => KeyCode::PageDown,
        0xFF57 => KeyCode::End,
        0xFFFF => KeyCode::Delete,
        0x0020 => KeyCode::Space,
        _ => match ch {
            Some(c) if !c.is_control() => KeyCode::Character(c),
            _ => KeyCode::Unknown,
        },
    }
}

/// Looks up the active `WindowId`, scale factor, and `WM_DELETE_WINDOW` atom matching an X11 window XID.
///
/// # Arguments
///
/// - `u64` - Native X11 window XID.
/// - `&HashMap<WindowId, LinuxWindowRecord>` - Active Linux window table.
///
/// # Returns
///
/// - `Option<(WindowId, f32, u64)>` - Matching window id, DPI scale, and delete atom.
#[must_use]
fn linux_find_window(
    xid: u64,
    map: &HashMap<WindowId, LinuxWindowRecord>,
) -> Option<(WindowId, f32, u64)> {
    for (wid, rec) in map.iter() {
        if !rec.get_is_wayland() && rec.get_window() == xid {
            return Some((
                *wid,
                rec.get_state().get_scale_factor(),
                rec.get_wm_delete_atom(),
            ));
        }
    }
    None
}

/// Pumps all queued X11 events non-blockingly via `XPending` / `XNextEvent` and translates them to `WindowEvent`s.
///
/// # Arguments
///
/// - `usize` - Loaded `libX11` handle.
/// - `&mut HashMap<WindowId, LinuxWindowRecord>` - Active Linux windows map.
/// - `&mut Vec<WindowEvent>` - Event sink receiving translated window events.
pub(crate) fn linux_x11_pump_events(
    lib: usize,
    windows: &mut HashMap<WindowId, LinuxWindowRecord>,
    out_events: &mut Vec<WindowEvent>,
) {
    if lib == 0 || windows.is_empty() {
        return;
    }
    let x11: *mut c_void = lib as *mut c_void;
    let pending_sym: *mut c_void = linux_dlsym(x11, SYM_X_PENDING);
    let next_sym: *mut c_void = linux_dlsym(x11, SYM_X_NEXT_EVENT);
    let lookup_sym: *mut c_void = linux_dlsym(x11, SYM_X_LOOKUP_STRING);
    if pending_sym.is_null() || next_sym.is_null() {
        return;
    }
    let displays: Vec<usize> = {
        let mut seen: Vec<usize> = Vec::new();
        for rec in windows.values() {
            if !rec.get_is_wayland() && rec.get_display() != 0 && !seen.contains(&rec.get_display())
            {
                seen.push(rec.get_display());
            }
        }
        seen
    };
    // SAFETY: Standard Xlib non-blocking event loop with matching LP64 event struct layouts.
    unsafe {
        let x_pending: unsafe extern "C" fn(*mut c_void) -> i32 = transmute(pending_sym);
        let x_next: unsafe extern "C" fn(*mut c_void, *mut X11EventRaw) -> i32 =
            transmute(next_sym);
        let x_lookup: Option<
            unsafe extern "C" fn(
                *mut X11KeyOrButtonEvent,
                *mut c_char,
                i32,
                *mut u64,
                *mut c_void,
            ) -> i32,
        > = if lookup_sym.is_null() {
            None
        } else {
            let func: unsafe extern "C" fn(
                *mut X11KeyOrButtonEvent,
                *mut c_char,
                i32,
                *mut u64,
                *mut c_void,
            ) -> i32 = transmute::<
                *mut c_void,
                unsafe extern "C" fn(
                    *mut X11KeyOrButtonEvent,
                    *mut c_char,
                    i32,
                    *mut u64,
                    *mut c_void,
                ) -> i32,
            >(lookup_sym);
            Some(func)
        };
        for disp_addr in displays.into_iter() {
            let disp: *mut c_void = disp_addr as *mut c_void;
            let mut raw: X11EventRaw = X11EventRaw { pad: [0; 24] };
            while x_pending(disp) > 0 {
                let _: i32 = x_next(disp, &mut raw);
                let ev_type: i32 = (raw.pad[0] & 0xFFFF_FFFF) as i32;
                match ev_type {
                    X11_CONFIGURE_NOTIFY => {
                        let cfg_ev: X11ConfigureEvent =
                            *(&raw as *const X11EventRaw).cast::<X11ConfigureEvent>();
                        if let Some((wid, scale, _)) = linux_find_window(cfg_ev.window, windows) {
                            let phys_w: u32 = cfg_ev.width.max(1) as u32;
                            let phys_h: u32 = cfg_ev.height.max(1) as u32;
                            let log_w: u32 = ((phys_w as f32) / scale).round().max(1.0) as u32;
                            let log_h: u32 = ((phys_h as f32) / scale).round().max(1.0) as u32;
                            if let Some(rec) = windows.get_mut(&wid) {
                                let prev_w: u32 = rec.get_state().get_physical_width();
                                let prev_h: u32 = rec.get_state().get_physical_height();
                                if prev_w != phys_w || prev_h != phys_h {
                                    rec.get_mut_config().set_width(log_w);
                                    rec.get_mut_config().set_height(log_h);
                                    rec.get_mut_state().set_logical_width(log_w as f32);
                                    rec.get_mut_state().set_logical_height(log_h as f32);
                                    rec.get_mut_state().set_physical_width(phys_w);
                                    rec.get_mut_state().set_physical_height(phys_h);
                                    out_events.push(WindowEvent::Resized(wid, log_w, log_h, scale));
                                }
                            }
                        }
                    }
                    X11_EXPOSE => {
                        let kb: X11KeyOrButtonEvent =
                            *(&raw as *const X11EventRaw).cast::<X11KeyOrButtonEvent>();
                        if let Some((wid, _, _)) = linux_find_window(kb.window, windows) {
                            out_events.push(WindowEvent::RedrawRequested(wid));
                        }
                    }
                    X11_CLIENT_MESSAGE => {
                        let cm: X11ClientMessageEvent =
                            *(&raw as *const X11EventRaw).cast::<X11ClientMessageEvent>();
                        if let Some((wid, _, wm_del)) = linux_find_window(cm.window, windows)
                            && wm_del != 0
                            && cm.data[0] == wm_del
                        {
                            out_events.push(WindowEvent::CloseRequested(wid));
                        }
                    }
                    X11_FOCUS_IN | X11_FOCUS_OUT => {
                        let kb: X11KeyOrButtonEvent =
                            *(&raw as *const X11EventRaw).cast::<X11KeyOrButtonEvent>();
                        if let Some((wid, _, _)) = linux_find_window(kb.window, windows) {
                            let focused: bool = ev_type == X11_FOCUS_IN;
                            if let Some(rec) = windows.get_mut(&wid) {
                                rec.get_mut_state().set_focused(focused);
                            }
                            out_events.push(WindowEvent::Focused(wid, focused));
                        }
                    }
                    X11_MOTION_NOTIFY => {
                        let mot: X11MotionEvent =
                            *(&raw as *const X11EventRaw).cast::<X11MotionEvent>();
                        if let Some((wid, scale, _)) = linux_find_window(mot.window, windows) {
                            let pos: Point =
                                Point::new((mot.x as f32) / scale, (mot.y as f32) / scale);
                            let ev: PointerEvent = PointerEvent::new(
                                1,
                                PointerDevice::Mouse,
                                pos,
                                PointerButton::Primary,
                                0.0,
                                linux_x11_modifiers(mot.state),
                            );
                            out_events.push(WindowEvent::PointerMove(wid, ev));
                        }
                    }
                    X11_BUTTON_PRESS | X11_BUTTON_RELEASE => {
                        let btn: X11KeyOrButtonEvent =
                            *(&raw as *const X11EventRaw).cast::<X11KeyOrButtonEvent>();
                        if let Some((wid, scale, _)) = linux_find_window(btn.window, windows) {
                            if ev_type == X11_BUTTON_PRESS && (btn.detail == 4 || btn.detail == 5) {
                                let dy: f32 = if btn.detail == 4 { -40.0 } else { 40.0 };
                                out_events.push(WindowEvent::Scroll(wid, Point::new(0.0, dy)));
                            } else if btn.detail <= 3 {
                                let button: PointerButton = match btn.detail {
                                    2 => PointerButton::Auxiliary,
                                    3 => PointerButton::Secondary,
                                    _ => PointerButton::Primary,
                                };
                                let pos: Point =
                                    Point::new((btn.x as f32) / scale, (btn.y as f32) / scale);
                                let pressed: bool = ev_type == X11_BUTTON_PRESS;
                                let pev: PointerEvent = PointerEvent::new(
                                    1,
                                    PointerDevice::Mouse,
                                    pos,
                                    button,
                                    if pressed { 1.0 } else { 0.0 },
                                    linux_x11_modifiers(btn.state),
                                );
                                if pressed {
                                    out_events.push(WindowEvent::PointerDown(wid, pev));
                                } else {
                                    out_events.push(WindowEvent::PointerUp(wid, pev));
                                }
                            }
                        }
                    }
                    X11_KEY_PRESS | X11_KEY_RELEASE => {
                        let mut key_ev: X11KeyOrButtonEvent =
                            *(&raw as *const X11EventRaw).cast::<X11KeyOrButtonEvent>();
                        if let Some((wid, _, _)) = linux_find_window(key_ev.window, windows) {
                            let mut buf: [u8; 16] = [0; 16];
                            let mut keysym: u64 = 0;
                            let mut len: i32 = 0;
                            if let Some(lookup_fn) = x_lookup {
                                len = lookup_fn(
                                    &mut key_ev,
                                    buf.as_mut_ptr().cast::<c_char>(),
                                    16,
                                    &mut keysym,
                                    null_mut(),
                                );
                            }
                            let decoded_str: Option<&str> = if len > 0 {
                                from_utf8(&buf[..(len as usize).min(16)]).ok()
                            } else {
                                None
                            };
                            let first_ch: Option<char> =
                                decoded_str.and_then(|s: &str| s.chars().next());
                            let code: KeyCode = linux_x11_keysym_to_keycode(keysym, first_ch);
                            let kev: KeyEvent =
                                KeyEvent::new(code, linux_x11_modifiers(key_ev.state), false);
                            if ev_type == X11_KEY_PRESS {
                                out_events.push(WindowEvent::KeyDown(wid, kev));
                                if let Some(text) = decoded_str
                                    && text.chars().all(|c: char| !c.is_control())
                                    && !text.is_empty()
                                {
                                    out_events.push(WindowEvent::TextInput(wid, text.to_string()));
                                }
                            } else {
                                out_events.push(WindowEvent::KeyUp(wid, kev));
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
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
