use super::*;

#[link(name = "objc")]
unsafe extern "C" {
    /// Body of the `objc_getClass` free function.
    ///
    /// # Arguments
    ///
    /// - `*const c_char` - A `*const c_char` parameter.
    ///
    /// # Returns
    ///
    /// - `*mut c_void` - A `*mut c_void` value.
    fn objc_getClass(name: *const c_char) -> *mut c_void;
    /// Body of the `sel_registerName` free function.
    ///
    /// # Arguments
    ///
    /// - `*const c_char` - A `*const c_char` parameter.
    ///
    /// # Returns
    ///
    /// - `*mut c_void` - A `*mut c_void` value.
    fn sel_registerName(name: *const c_char) -> *mut c_void;
    /// Body of the `objc_msgSend` free function.
    fn objc_msgSend();
}

#[link(name = "AppKit", kind = "framework")]
unsafe extern "C" {}

#[link(name = "QuartzCore", kind = "framework")]
unsafe extern "C" {}

#[link(name = "CoreFoundation", kind = "framework")]
unsafe extern "C" {
    /// Copies a byte slice into an immutable CoreFoundation `CFDataRef`.
    ///
    /// # Arguments
    ///
    /// - `*const c_void` - Optional allocator (`null` for default).
    /// - `*const u8` - Source pixel byte pointer.
    /// - `isize` - Byte length.
    ///
    /// # Returns
    ///
    /// - `*mut c_void` - Retained `CFDataRef` owning a copy of the pixels.
    fn CFDataCreate(allocator: *const c_void, bytes: *const u8, length: isize) -> *mut c_void;
    /// Releases a CoreFoundation object reference.
    ///
    /// # Arguments
    ///
    /// - `*const c_void` - CoreFoundation object pointer.
    fn CFRelease(cf: *const c_void);
    /// Returns the current thread's `CFRunLoopRef`.
    ///
    /// # Returns
    ///
    /// - `*mut c_void` - Current `CFRunLoopRef`.
    fn CFRunLoopGetCurrent() -> *mut c_void;
    /// Returns current absolute CoreFoundation timestamp in seconds.
    ///
    /// # Returns
    ///
    /// - `f64` - Absolute time in seconds.
    fn CFAbsoluteTimeGetCurrent() -> f64;
    /// Creates a repeating `CFRunLoopTimerRef` capable of firing inside modal live-resize tracking loops.
    ///
    /// # Arguments
    ///
    /// - `*const c_void` - Allocator (`null`).
    /// - `f64` - Initial fire date.
    /// - `f64` - Repeat interval in seconds.
    /// - `usize` - Flags (`0`).
    /// - `isize` - Order (`0`).
    /// - `unsafe extern "C" fn(*mut c_void, *mut c_void)` - Timer callback.
    /// - `*mut CfRunLoopTimerContext` - Context carrying user info pointer.
    ///
    /// # Returns
    ///
    /// - `*mut c_void` - Retained `CFRunLoopTimerRef`.
    fn CFRunLoopTimerCreate(
        allocator: *const c_void,
        fire_date: f64,
        interval: f64,
        flags: usize,
        order: isize,
        callout: unsafe extern "C" fn(*mut c_void, *mut c_void),
        context: *mut CfRunLoopTimerContext,
    ) -> *mut c_void;
    /// Attaches a `CFRunLoopTimerRef` to `run_loop` under `mode` (e.g. `kCFRunLoopCommonModes`).
    ///
    /// # Arguments
    ///
    /// - `*mut c_void` - Target `CFRunLoopRef`.
    /// - `*mut c_void` - Timer handle.
    /// - `*const c_void` - Run-loop mode `CFStringRef`.
    fn CFRunLoopAddTimer(run_loop: *mut c_void, timer: *mut c_void, mode: *const c_void);
    /// Invalidates a `CFRunLoopTimerRef` and removes it from all run-loop modes.
    ///
    /// # Arguments
    ///
    /// - `*mut c_void` - Timer handle.
    fn CFRunLoopTimerInvalidate(timer: *mut c_void);
    /// Global CoreFoundation constant for `kCFRunLoopCommonModes` (includes `NSEventTrackingRunLoopMode`).
    static kCFRunLoopCommonModes: *const c_void;
}

#[link(name = "CoreGraphics", kind = "framework")]
unsafe extern "C" {
    /// Body of the `CGColorSpaceCreateDeviceRGB` free function.
    ///
    /// # Returns
    ///
    /// - `*mut c_void` - A `*mut c_void` value.
    fn CGColorSpaceCreateDeviceRGB() -> *mut c_void;
    /// Body of the `CGColorSpaceRelease` free function.
    ///
    /// # Arguments
    ///
    /// - `*mut c_void` - A `*mut c_void` parameter.
    fn CGColorSpaceRelease(space: *mut c_void);
    /// Creates a `CGDataProviderRef` retaining `data` (`CFDataRef`) safely across asynchronous compositing.
    ///
    /// # Arguments
    ///
    /// - `*mut c_void` - Source `CFDataRef`.
    ///
    /// # Returns
    ///
    /// - `*mut c_void` - Created `CGDataProviderRef`.
    fn CGDataProviderCreateWithCFData(data: *mut c_void) -> *mut c_void;
    /// Body of the `CGDataProviderRelease` free function.
    ///
    /// # Arguments
    ///
    /// - `*mut c_void` - A `*mut c_void` parameter.
    fn CGDataProviderRelease(provider: *mut c_void);
    /// Body of the `CGImageCreate` free function.
    ///
    /// # Arguments
    ///
    /// - `usize` - A non-negative integer (`usize`).
    /// - `usize` - A non-negative integer (`usize`).
    /// - `usize` - A non-negative integer (`usize`).
    /// - `usize` - A non-negative integer (`usize`).
    /// - `usize` - A non-negative integer (`usize`).
    /// - `*mut c_void` - A `*mut c_void` parameter.
    /// - `u32` - A 32-bit unsigned integer (`u32`).
    /// - `*mut c_void` - A `*mut c_void` parameter.
    /// - `*const f64` - A `*const f64` parameter.
    /// - `bool` - A boolean (`bool`).
    /// - `i32` - A 32-bit signed integer (`i32`).
    ///
    /// # Returns
    ///
    /// - `*mut c_void` - A `*mut c_void` value.
    fn CGImageCreate(
        width: usize,
        height: usize,
        bits_per_component: usize,
        bits_per_pixel: usize,
        bytes_per_row: usize,
        space: *mut c_void,
        bitmap_info: u32,
        provider: *mut c_void,
        decode: *const f64,
        should_interpolate: bool,
        intent: i32,
    ) -> *mut c_void;
    /// Body of the `CGImageRelease` free function.
    ///
    /// # Arguments
    ///
    /// - `*mut c_void` - A `*mut c_void` parameter.
    fn CGImageRelease(image: *mut c_void);
}

/// Looks up an Objective-C class pointer from a null-terminated class name constant.
///
/// # Arguments
///
/// - `&'static str` - Null-terminated Objective-C class name.
///
/// # Returns
///
/// - `*mut c_void` - Class pointer, or null if unavailable.
pub(crate) fn objc_class(name: &'static str) -> *mut c_void {
    // SAFETY: `name` is a static null-terminated ASCII string passed to libobjc.
    unsafe { objc_getClass(name.as_ptr().cast::<c_char>()) }
}

/// Registers an Objective-C selector pointer from a null-terminated selector name constant.
///
/// # Arguments
///
/// - `&'static str` - Null-terminated Objective-C selector name.
///
/// # Returns
///
/// - `*mut c_void` - Selector pointer.
pub(crate) fn objc_sel(name: &'static str) -> *mut c_void {
    // SAFETY: `name` is a static null-terminated ASCII string passed to libobjc.
    unsafe { sel_registerName(name.as_ptr().cast::<c_char>()) }
}

/// Sends a zero-argument Objective-C message returning a pointer.
///
/// # Arguments
///
/// - `*mut c_void` - Objective-C receiver.
/// - `*mut c_void` - Objective-C selector.
///
/// # Returns
///
/// - `*mut c_void` - Returned object pointer.
pub(crate) fn objc_send_ptr0(receiver: *mut c_void, sel: *mut c_void) -> *mut c_void {
    if receiver.is_null() {
        return null_mut();
    }
    // SAFETY: `objc_msgSend` is transmuted to match the exact 0-arg Objective-C ABI.
    unsafe {
        let func: unsafe extern "C" fn(*mut c_void, *mut c_void) -> *mut c_void =
            transmute(objc_msgSend as *const ());
        func(receiver, sel)
    }
}

/// Sends a zero-argument Objective-C message returning `f64` (`CGFloat` on 64-bit Apple platforms).
///
/// # Arguments
///
/// - `*mut c_void` - Objective-C receiver.
/// - `*mut c_void` - Objective-C selector.
///
/// # Returns
///
/// - `f64` - Returned floating-point scalar.
pub(crate) fn objc_send_f64_0(receiver: *mut c_void, sel: *mut c_void) -> f64 {
    if receiver.is_null() {
        return 1.0;
    }
    // SAFETY: `objc_msgSend` matches the 64-bit Apple ABI returning `CGFloat` in `d0`/`xmm0`.
    unsafe {
        let func: unsafe extern "C" fn(*mut c_void, *mut c_void) -> f64 =
            transmute(objc_msgSend as *const ());
        func(receiver, sel)
    }
}

/// Sends a single pointer-argument Objective-C message returning a pointer.
///
/// # Arguments
///
/// - `*mut c_void` - Objective-C receiver.
/// - `*mut c_void` - Objective-C selector.
/// - `*mut c_void` - Argument pointer.
///
/// # Returns
///
/// - `*mut c_void` - Returned object pointer.
pub(crate) fn objc_send_ptr1(
    receiver: *mut c_void,
    sel: *mut c_void,
    arg: *mut c_void,
) -> *mut c_void {
    if receiver.is_null() {
        return null_mut();
    }
    // SAFETY: `objc_msgSend` matches the 1-pointer-arg Objective-C ABI.
    unsafe {
        let func: unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void) -> *mut c_void =
            transmute(objc_msgSend as *const ());
        func(receiver, sel, arg)
    }
}

/// Sends a single boolean-argument Objective-C message returning void.
///
/// # Arguments
///
/// - `*mut c_void` - Objective-C receiver.
/// - `*mut c_void` - Objective-C selector.
/// - `bool` - Objective-C `BOOL` argument.
pub(crate) fn objc_send_void_bool(receiver: *mut c_void, sel: *mut c_void, value: bool) {
    if receiver.is_null() {
        return;
    }
    // SAFETY: `objc_msgSend` matches `void (id, SEL, BOOL)` ABI.
    unsafe {
        let func: unsafe extern "C" fn(*mut c_void, *mut c_void, i8) =
            transmute(objc_msgSend as *const ());
        func(receiver, sel, if value { 1 } else { 0 });
    }
}

/// Sends a single `isize` argument Objective-C message returning void.
///
/// # Arguments
///
/// - `*mut c_void` - Objective-C receiver.
/// - `*mut c_void` - Objective-C selector.
/// - `isize` - Integer argument.
pub(crate) fn objc_send_void_isize(receiver: *mut c_void, sel: *mut c_void, value: isize) {
    if receiver.is_null() {
        return;
    }
    // SAFETY: `objc_msgSend` matches `void (id, SEL, NSInteger)` ABI.
    unsafe {
        let func: unsafe extern "C" fn(*mut c_void, *mut c_void, isize) =
            transmute(objc_msgSend as *const ());
        func(receiver, sel, value);
    }
}

/// Sends a single `CgSize` argument Objective-C message returning void.
///
/// # Arguments
///
/// - `*mut c_void` - Objective-C receiver.
/// - `*mut c_void` - Objective-C selector.
/// - `CgSize` - Size argument.
pub(crate) fn objc_send_void_size(receiver: *mut c_void, sel: *mut c_void, size: CgSize) {
    if receiver.is_null() {
        return;
    }
    // SAFETY: `objc_msgSend` matches `void (id, SEL, NSSize)` ABI.
    unsafe {
        let func: unsafe extern "C" fn(*mut c_void, *mut c_void, CgSize) =
            transmute(objc_msgSend as *const ());
        func(receiver, sel, size);
    }
}

/// Sends a single `f64` argument Objective-C message returning void.
///
/// # Arguments
///
/// - `*mut c_void` - Objective-C receiver.
/// - `*mut c_void` - Objective-C selector.
/// - `f64` - `CGFloat` argument.
pub(crate) fn objc_send_void_f64(receiver: *mut c_void, sel: *mut c_void, value: f64) {
    if receiver.is_null() {
        return;
    }
    // SAFETY: `objc_msgSend` matches `void (id, SEL, CGFloat)` ABI on 64-bit Apple targets.
    unsafe {
        let func: unsafe extern "C" fn(*mut c_void, *mut c_void, f64) =
            transmute(objc_msgSend as *const ());
        func(receiver, sel, value);
    }
}

/// Sends a zero-argument Objective-C message returning `bool`.
///
/// # Arguments
///
/// - `*mut c_void` - Objective-C receiver.
/// - `*mut c_void` - Objective-C selector.
///
/// # Returns
///
/// - `bool` - Objective-C `BOOL` result.
pub(crate) fn objc_send_bool0(receiver: *mut c_void, sel: *mut c_void) -> bool {
    if receiver.is_null() {
        return false;
    }
    // SAFETY: `objc_msgSend` matches `BOOL (id, SEL)` ABI.
    unsafe {
        let func: unsafe extern "C" fn(*mut c_void, *mut c_void) -> i8 =
            transmute(objc_msgSend as *const ());
        func(receiver, sel) != 0
    }
}

/// Sends a zero-argument Objective-C message returning `usize`.
///
/// # Arguments
///
/// - `*mut c_void` - Objective-C receiver.
/// - `*mut c_void` - Objective-C selector.
///
/// # Returns
///
/// - `usize` - Returned `NSUInteger`.
pub(crate) fn objc_send_usize0(receiver: *mut c_void, sel: *mut c_void) -> usize {
    if receiver.is_null() {
        return 0;
    }
    // SAFETY: `objc_msgSend` matches `NSUInteger (id, SEL)` ABI.
    unsafe {
        let func: unsafe extern "C" fn(*mut c_void, *mut c_void) -> usize =
            transmute(objc_msgSend as *const ());
        func(receiver, sel)
    }
}

/// Sends a zero-argument Objective-C message returning `u16`.
///
/// # Arguments
///
/// - `*mut c_void` - Objective-C receiver.
/// - `*mut c_void` - Objective-C selector.
///
/// # Returns
///
/// - `u16` - Returned unsigned 16-bit hardware key code.
pub(crate) fn objc_send_u16_0(receiver: *mut c_void, sel: *mut c_void) -> u16 {
    if receiver.is_null() {
        return 0;
    }
    // SAFETY: `objc_msgSend` matches `unsigned short (id, SEL)` ABI.
    unsafe {
        let func: unsafe extern "C" fn(*mut c_void, *mut c_void) -> u16 =
            transmute(objc_msgSend as *const ());
        func(receiver, sel)
    }
}

/// Sends a zero-argument Objective-C message returning `CgPoint`.
///
/// # Arguments
///
/// - `*mut c_void` - Objective-C receiver.
/// - `*mut c_void` - Objective-C selector.
///
/// # Returns
///
/// - `CgPoint` - Returned 2D point in window coordinates.
pub(crate) fn objc_send_point0(receiver: *mut c_void, sel: *mut c_void) -> CgPoint {
    if receiver.is_null() {
        return CgPoint::default();
    }
    // SAFETY: `CGPoint` (16 bytes, two `f64`) is returned in floating-point registers on arm64 and x86_64.
    unsafe {
        let func: unsafe extern "C" fn(*mut c_void, *mut c_void) -> CgPoint =
            transmute(objc_msgSend as *const ());
        func(receiver, sel)
    }
}

/// Queries an `NSView`'s logical bounds size on Apple Silicon.
///
/// # Arguments
///
/// - `*mut c_void` - `NSView*` pointer.
///
/// # Returns
///
/// - `Option<(u32, u32)>` - Current logical `(width, height)` in points if measurable.
#[cfg(target_arch = "aarch64")]
pub(crate) fn macos_view_logical_size(ns_view: *mut c_void) -> Option<(u32, u32)> {
    if ns_view.is_null() {
        return None;
    }
    // SAFETY: On arm64 macOS, `objc_msgSend` returns `CGRect` directly in `d0..d3`.
    let rect: CgRect = unsafe {
        let func: unsafe extern "C" fn(*mut c_void, *mut c_void) -> CgRect =
            transmute(objc_msgSend as *const ());
        func(ns_view, objc_sel(OBJC_SEL_BOUNDS))
    };
    if rect.size.width >= 1.0 && rect.size.height >= 1.0 {
        Some((
            rect.size.width.round() as u32,
            rect.size.height.round() as u32,
        ))
    } else {
        None
    }
}

/// Fallback logical size query for non-arm64 macOS hosts.
///
/// # Arguments
///
/// - `*mut c_void` - Unused `NSView*` pointer.
///
/// # Returns
///
/// - `Option<(u32, u32)>` - Always `None` on non-arm64 fallback.
#[cfg(not(target_arch = "aarch64"))]
pub(crate) fn macos_view_logical_size(_ns_view: *mut c_void) -> Option<(u32, u32)> {
    None
}

/// Translates a macOS virtual key code and optional character into a cross-platform `KeyCode`.
///
/// # Arguments
///
/// - `u16` - AppKit virtual key code.
/// - `&str` - UTF-8 characters reported by `NSEvent.characters`.
///
/// # Returns
///
/// - `KeyCode` - Normalized `euv` key code.
pub(crate) fn macos_map_key_code(raw_code: u16, chars: &str) -> KeyCode {
    match raw_code {
        36 | 76 => KeyCode::Enter,
        51 => KeyCode::Backspace,
        117 => KeyCode::Delete,
        53 => KeyCode::Escape,
        48 => KeyCode::Tab,
        49 => KeyCode::Space,
        123 => KeyCode::ArrowLeft,
        124 => KeyCode::ArrowRight,
        125 => KeyCode::ArrowDown,
        126 => KeyCode::ArrowUp,
        115 => KeyCode::Home,
        119 => KeyCode::End,
        116 => KeyCode::PageUp,
        121 => KeyCode::PageDown,
        0 => KeyCode::Character('a'),
        11 => KeyCode::Character('b'),
        8 => KeyCode::Character('c'),
        2 => KeyCode::Character('d'),
        14 => KeyCode::Character('e'),
        3 => KeyCode::Character('f'),
        5 => KeyCode::Character('g'),
        4 => KeyCode::Character('h'),
        34 => KeyCode::Character('i'),
        38 => KeyCode::Character('j'),
        40 => KeyCode::Character('k'),
        37 => KeyCode::Character('l'),
        46 => KeyCode::Character('m'),
        45 => KeyCode::Character('n'),
        31 => KeyCode::Character('o'),
        35 => KeyCode::Character('p'),
        12 => KeyCode::Character('q'),
        15 => KeyCode::Character('r'),
        1 => KeyCode::Character('s'),
        17 => KeyCode::Character('t'),
        32 => KeyCode::Character('u'),
        9 => KeyCode::Character('v'),
        13 => KeyCode::Character('w'),
        7 => KeyCode::Character('x'),
        16 => KeyCode::Character('y'),
        6 => KeyCode::Character('z'),
        _ => chars
            .chars()
            .next()
            .map(KeyCode::Character)
            .unwrap_or(KeyCode::Unknown),
    }
}

/// Decodes AppKit modifier flags into cross-platform `Modifiers`.
///
/// # Arguments
///
/// - `usize` - Raw `NSEventModifierFlags` bitmask.
///
/// # Returns
///
/// - `Modifiers` - Normalized modifier state.
pub(crate) fn macos_map_modifiers(flags: usize) -> Modifiers {
    Modifiers::new(
        (flags & NS_MOD_SHIFT) != 0,
        (flags & NS_MOD_CTRL) != 0,
        (flags & NS_MOD_ALT) != 0,
        (flags & NS_MOD_CMD) != 0,
    )
}

/// Pumps pending AppKit `NSEvent`s non-blockingly and translates mouse, scroll, and keyboard input into `WindowEvent`s.
///
/// # Arguments
///
/// - `&HashMap<WindowId, MacOsWindowRecord>` - Open native macOS windows.
/// - `&mut Vec<WindowEvent>` - Output event queue.
pub(crate) fn macos_pump_events(
    windows: &HashMap<WindowId, MacOsWindowRecord>,
    out_events: &mut Vec<WindowEvent>,
) {
    let app_cls: *mut c_void = objc_class(OBJC_CLS_NS_APPLICATION);
    let app: *mut c_void = objc_send_ptr0(app_cls, objc_sel(OBJC_SEL_SHARED_APP));
    if app.is_null() {
        return;
    }
    let mode_ns: *mut c_void = macos_ns_string(NS_DEFAULT_RUN_LOOP_MODE);
    let next_sel: *mut c_void = objc_sel(OBJC_SEL_NEXT_EVENT);
    let send_sel: *mut c_void = objc_sel(OBJC_SEL_SEND_EVENT);
    let type_sel: *mut c_void = objc_sel(OBJC_SEL_TYPE);
    let win_sel: *mut c_void = objc_sel(OBJC_SEL_WINDOW);
    let loc_sel: *mut c_void = objc_sel(OBJC_SEL_LOCATION_IN_WINDOW);
    let mod_sel: *mut c_void = objc_sel(OBJC_SEL_MODIFIER_FLAGS);
    let key_sel: *mut c_void = objc_sel(OBJC_SEL_KEY_CODE);
    let chars_sel: *mut c_void = objc_sel(OBJC_SEL_CHARACTERS);
    let utf8_sel: *mut c_void = objc_sel(OBJC_SEL_UTF8_STRING);
    let scroll_x_sel: *mut c_void = objc_sel(OBJC_SEL_SCROLLING_DELTA_X);
    let scroll_y_sel: *mut c_void = objc_sel(OBJC_SEL_SCROLLING_DELTA_Y);
    let precise_scroll_sel: *mut c_void = objc_sel(OBJC_SEL_HAS_PRECISE_SCROLL);
    let mut processed: usize = 0;
    while processed < 64 {
        // SAFETY: Invokes `-[NSApplication nextEventMatchingMask:untilDate:inMode:dequeue:]` non-blockingly (`untilDate:nil`).
        let event: *mut c_void = unsafe {
            let next_fn: unsafe extern "C" fn(
                *mut c_void,
                *mut c_void,
                usize,
                *mut c_void,
                *mut c_void,
                i8,
            ) -> *mut c_void = transmute(objc_msgSend as *const ());
            next_fn(app, next_sel, NS_EVENT_MASK_ANY, null_mut(), mode_ns, 1)
        };
        if event.is_null() {
            break;
        }
        let ev_type: usize = objc_send_usize0(event, type_sel);
        let ev_win: usize = objc_send_ptr0(event, win_sel) as usize;
        let mods: Modifiers = macos_map_modifiers(objc_send_usize0(event, mod_sel));
        let matched: Option<(WindowId, f32)> = windows
            .iter()
            .find(|(_, rec): &(&WindowId, &MacOsWindowRecord)| {
                rec.get_ns_window() == ev_win || ev_win == 0
            })
            .map(|(id, rec): (&WindowId, &MacOsWindowRecord)| {
                (*id, rec.get_state().get_logical_height())
            });
        let mut forward_to_appkit: bool = true;
        if let Some((wid, logical_h)) = matched {
            let loc: CgPoint = objc_send_point0(event, loc_sel);
            let ui_x: f32 = loc.x as f32;
            let ui_y: f32 = logical_h - (loc.y as f32);
            let inside_content: bool = ui_y >= 0.0 && ui_y <= logical_h && ui_x >= 0.0;
            match ev_type {
                NS_EVENT_LEFT_MOUSE_DOWN if inside_content => {
                    out_events.push(WindowEvent::PointerDown(
                        wid,
                        PointerEvent::new(
                            1,
                            PointerDevice::Mouse,
                            Point::new(ui_x, ui_y),
                            PointerButton::Primary,
                            1.0,
                            mods,
                        ),
                    ));
                }
                NS_EVENT_LEFT_MOUSE_UP => {
                    out_events.push(WindowEvent::PointerUp(
                        wid,
                        PointerEvent::new(
                            1,
                            PointerDevice::Mouse,
                            Point::new(ui_x, ui_y),
                            PointerButton::Primary,
                            0.0,
                            mods,
                        ),
                    ));
                }
                NS_EVENT_RIGHT_MOUSE_DOWN if inside_content => {
                    out_events.push(WindowEvent::PointerDown(
                        wid,
                        PointerEvent::new(
                            1,
                            PointerDevice::Mouse,
                            Point::new(ui_x, ui_y),
                            PointerButton::Secondary,
                            1.0,
                            mods,
                        ),
                    ));
                }
                NS_EVENT_RIGHT_MOUSE_UP if inside_content => {
                    out_events.push(WindowEvent::PointerUp(
                        wid,
                        PointerEvent::new(
                            1,
                            PointerDevice::Mouse,
                            Point::new(ui_x, ui_y),
                            PointerButton::Secondary,
                            0.0,
                            mods,
                        ),
                    ));
                }
                NS_EVENT_MOUSE_MOVED if inside_content => {
                    let next_ptr: PointerEvent = PointerEvent::new(
                        1,
                        PointerDevice::Mouse,
                        Point::new(ui_x, ui_y),
                        PointerButton::Primary,
                        0.0,
                        mods,
                    );
                    if let Some(WindowEvent::PointerMove(prev_wid, prev_ptr)) =
                        out_events.last_mut()
                        && *prev_wid == wid
                    {
                        *prev_ptr = next_ptr;
                    } else {
                        out_events.push(WindowEvent::PointerMove(wid, next_ptr));
                    }
                }
                NS_EVENT_LEFT_MOUSE_DRAGGED => {
                    let next_ptr: PointerEvent = PointerEvent::new(
                        1,
                        PointerDevice::Mouse,
                        Point::new(ui_x, ui_y),
                        PointerButton::Primary,
                        1.0,
                        mods,
                    );
                    if let Some(WindowEvent::PointerMove(prev_wid, prev_ptr)) =
                        out_events.last_mut()
                        && *prev_wid == wid
                    {
                        *prev_ptr = next_ptr;
                    } else {
                        out_events.push(WindowEvent::PointerMove(wid, next_ptr));
                    }
                }
                NS_EVENT_SCROLL_WHEEL if inside_content => {
                    let is_precise: bool = objc_send_bool0(event, precise_scroll_sel);
                    let multiplier: f32 = if is_precise { 1.0 } else { 18.0 };
                    let dx: f32 = -(objc_send_f64_0(event, scroll_x_sel) as f32) * multiplier;
                    let dy: f32 = -(objc_send_f64_0(event, scroll_y_sel) as f32) * multiplier;
                    if let Some(WindowEvent::Scroll(prev_wid, prev_delta)) = out_events.last_mut()
                        && *prev_wid == wid
                    {
                        *prev_delta = Point::new(prev_delta.get_x() + dx, prev_delta.get_y() + dy);
                    } else {
                        out_events.push(WindowEvent::Scroll(wid, Point::new(dx, dy)));
                    }
                }
                NS_EVENT_KEY_DOWN => {
                    let raw_key: u16 = objc_send_u16_0(event, key_sel);
                    let chars_ns: *mut c_void = objc_send_ptr0(event, chars_sel);
                    let utf8_ptr: *const c_char =
                        objc_send_ptr0(chars_ns, utf8_sel).cast::<c_char>();
                    let text: String = if utf8_ptr.is_null() {
                        String::new()
                    } else {
                        // SAFETY: `UTF8String` returns a valid null-terminated UTF-8 string when non-null.
                        unsafe { CStr::from_ptr(utf8_ptr).to_str().unwrap_or("").to_string() }
                    };
                    let key_code: KeyCode = macos_map_key_code(raw_key, &text);
                    out_events.push(WindowEvent::KeyDown(
                        wid,
                        KeyEvent::new(key_code, mods, false),
                    ));
                    if !mods.get_meta() && !mods.get_ctrl() {
                        let printable: String =
                            text.chars().filter(|c: &char| !c.is_control()).collect();
                        if !printable.is_empty() {
                            out_events.push(WindowEvent::TextInput(wid, printable));
                        }
                        forward_to_appkit = false;
                    }
                }
                NS_EVENT_KEY_UP => {
                    let raw_key: u16 = objc_send_u16_0(event, key_sel);
                    let chars_ns: *mut c_void =
                        objc_send_ptr0(event, objc_sel(OBJC_SEL_CHARS_IGNORING_MODS));
                    let utf8_ptr: *const c_char =
                        objc_send_ptr0(chars_ns, utf8_sel).cast::<c_char>();
                    let text: String = if utf8_ptr.is_null() {
                        String::new()
                    } else {
                        // SAFETY: `UTF8String` returns a valid null-terminated C string when non-null.
                        unsafe { CStr::from_ptr(utf8_ptr).to_str().unwrap_or("").to_string() }
                    };
                    let key_code: KeyCode = macos_map_key_code(raw_key, &text);
                    out_events.push(WindowEvent::KeyUp(
                        wid,
                        KeyEvent::new(key_code, mods, false),
                    ));
                    if !mods.get_meta() {
                        forward_to_appkit = false;
                    }
                }
                _ => {}
            }
        }
        if forward_to_appkit {
            let _: *mut c_void = objc_send_ptr1(app, send_sel, event);
        }
        processed += 1;
    }
    let _: *mut c_void = objc_send_ptr0(app, objc_sel(OBJC_SEL_UPDATE_WINDOWS));
}

/// Allocates an `NSString*` from a Rust `&str`.
///
/// # Arguments
///
/// - `&str` - UTF-8 string slice.
///
/// # Returns
///
/// - `*mut c_void` - Autoreleased `NSString*` pointer, or null on failure.
pub(crate) fn macos_ns_string(text: &str) -> *mut c_void {
    let Ok(cstr) = CString::new(text) else {
        return null_mut();
    };
    let cls: *mut c_void = objc_class(OBJC_CLS_NS_STRING);
    let sel: *mut c_void = objc_sel(OBJC_SEL_STRING_WITH_UTF8);
    objc_send_ptr1(cls, sel, cstr.as_ptr().cast_mut().cast::<c_void>())
}

/// Queries the live backing scale factor for a specific `NSWindow` (or main screen fallback).
///
/// # Arguments
///
/// - `*mut c_void` - Target `NSWindow*` pointer.
///
/// # Returns
///
/// - `f32` - Active Retina backing scale factor (`1.0`, `2.0`, etc.).
#[must_use]
pub(crate) fn macos_window_backing_scale(ns_window: *mut c_void) -> f32 {
    if !ns_window.is_null() {
        let scale: f64 = objc_send_f64_0(ns_window, objc_sel(OBJC_SEL_BACKING_SCALE));
        if scale > 0.0 {
            return scale as f32;
        }
    }
    macos_backing_scale_factor()
}

/// Installs a 60Hz `CFRunLoopTimer` on `kCFRunLoopCommonModes` so live window border dragging repaints continuously.
///
/// # Arguments
///
/// - `unsafe extern "C" fn(*mut c_void, *mut c_void)` - Callback invoked during modal resize tracking.
/// - `*mut c_void` - Opaque user context pointer passed to `callout`.
///
/// # Returns
///
/// - `*mut c_void` - Retained `CFRunLoopTimerRef` handle to be released via `macos_remove_live_resize_timer`.
pub(crate) fn macos_install_live_resize_timer(
    callout: unsafe extern "C" fn(*mut c_void, *mut c_void),
    info: *mut c_void,
) -> *mut c_void {
    let mut ctx: CfRunLoopTimerContext = CfRunLoopTimerContext {
        version: 0,
        info,
        retain: null(),
        release: null(),
        copy_description: null(),
    };
    // SAFETY: CoreFoundation run-loop timer APIs called on the main AppKit thread.
    unsafe {
        let fire_date: f64 = CFAbsoluteTimeGetCurrent() + 0.016;
        let timer: *mut c_void =
            CFRunLoopTimerCreate(null(), fire_date, 0.016, 0, 0, callout, &mut ctx);
        if !timer.is_null() {
            let rl: *mut c_void = CFRunLoopGetCurrent();
            if !rl.is_null() {
                CFRunLoopAddTimer(rl, timer, kCFRunLoopCommonModes);
            }
        }
        timer
    }
}

/// Invalidates and releases a `CFRunLoopTimerRef` created by `macos_install_live_resize_timer`.
///
/// # Arguments
///
/// - `*mut c_void` - Timer handle.
pub(crate) fn macos_remove_live_resize_timer(timer: *mut c_void) {
    if timer.is_null() {
        return;
    }
    // SAFETY: Valid `CFRunLoopTimerRef` owned by caller.
    unsafe {
        CFRunLoopTimerInvalidate(timer);
        CFRelease(timer.cast_const());
    }
}

/// Queries the macOS main display backing scale factor via `NSScreen.mainScreen.backingScaleFactor`.
///
/// # Returns
///
/// - `f32` - Retina display scale factor (`1.0`, `2.0`, etc.).
#[must_use]
pub fn macos_backing_scale_factor() -> f32 {
    let screen_cls: *mut c_void = objc_class(OBJC_CLS_NS_SCREEN);
    let screen: *mut c_void = objc_send_ptr0(screen_cls, objc_sel(OBJC_SEL_MAIN_SCREEN));
    if screen.is_null() {
        return 1.0;
    }
    let scale: f64 = objc_send_f64_0(screen, objc_sel(OBJC_SEL_BACKING_SCALE));
    if scale > 0.0 { scale as f32 } else { 1.0 }
}

/// Detects the current macOS system appearance (`Light` or `Dark`) via `NSApplication.effectiveAppearance`.
///
/// # Returns
///
/// - `ThemeMode` - Detected macOS appearance mode.
#[must_use]
pub fn macos_system_theme() -> ThemeMode {
    let app_cls: *mut c_void = objc_class(OBJC_CLS_NS_APPLICATION);
    let app: *mut c_void = objc_send_ptr0(app_cls, objc_sel(OBJC_SEL_SHARED_APP));
    let appearance: *mut c_void = objc_send_ptr0(app, objc_sel(OBJC_SEL_EFFECTIVE_APPEARANCE));
    let name_ns: *mut c_void = objc_send_ptr0(appearance, objc_sel(OBJC_SEL_NAME));
    let utf8_ptr: *const c_char =
        objc_send_ptr0(name_ns, objc_sel(OBJC_SEL_UTF8_STRING)).cast::<c_char>();
    if utf8_ptr.is_null() {
        return ThemeMode::Light;
    }
    // SAFETY: `UTF8String` returns a valid null-terminated C string when non-null.
    let name: &str = unsafe { CStr::from_ptr(utf8_ptr).to_str().unwrap_or("") };
    if name.contains(MACOS_DARK_APPEARANCE_MARKER) {
        ThemeMode::Dark
    } else {
        ThemeMode::Light
    }
}

/// Creates a native `NSWindow` with an attached `CAMetalLayer` directly via AppKit + QuartzCore FFI.
///
/// # Arguments
///
/// - `&WindowConfig` - Declarative window configuration.
///
/// # Returns
///
/// - `Result<(usize, usize, usize), PlatformError>` - Tuple `(ns_window, ns_view, metal_layer)`.
pub(crate) fn macos_create_native_window(
    config: &WindowConfig,
) -> Result<(usize, usize, usize), PlatformError> {
    let app_cls: *mut c_void = objc_class(OBJC_CLS_NS_APPLICATION);
    let app: *mut c_void = objc_send_ptr0(app_cls, objc_sel(OBJC_SEL_SHARED_APP));
    if !app.is_null() {
        objc_send_void_isize(app, objc_sel(OBJC_SEL_SET_ACTIVATION_POLICY), 0);
        let _: *mut c_void = objc_send_ptr0(app, objc_sel(OBJC_SEL_FINISH_LAUNCHING));
    }
    let win_cls: *mut c_void = objc_class(OBJC_CLS_NS_WINDOW);
    if win_cls.is_null() {
        return Err(PlatformError::InitializationFailed(
            ERR_MACOS_APPKIT_MISSING.to_string(),
        ));
    }
    let allocated: *mut c_void = objc_send_ptr0(win_cls, objc_sel(OBJC_SEL_ALLOC));
    let mut style_mask: usize = if config.get_decorations() {
        let base_mask: usize = NS_WINDOW_STYLE_TITLED | NS_WINDOW_STYLE_CLOSABLE;
        let chrome_mask: usize = base_mask | NS_WINDOW_STYLE_MINIATURIZABLE;
        chrome_mask | NS_WINDOW_STYLE_FULL_SIZE_CONTENT
    } else {
        NS_WINDOW_STYLE_BORDERLESS
    };
    if config.get_resizable() {
        style_mask |= NS_WINDOW_STYLE_RESIZABLE;
    }
    let (pos_x, pos_y): (f64, f64) = config
        .position_value()
        .map(|(x, y): (i32, i32)| (x as f64, y as f64))
        .unwrap_or((120.0, 120.0));
    let rect: CgRect = CgRect {
        origin: CgPoint { x: pos_x, y: pos_y },
        size: CgSize {
            width: config.get_width() as f64,
            height: config.get_height() as f64,
        },
    };
    // SAFETY: Invokes `-[NSWindow initWithContentRect:styleMask:backing:defer:]` with exact ABI signature.
    let ns_window: *mut c_void = unsafe {
        let init_fn: unsafe extern "C" fn(
            *mut c_void,
            *mut c_void,
            CgRect,
            usize,
            usize,
            i8,
        ) -> *mut c_void = transmute(objc_msgSend as *const ());
        init_fn(
            allocated,
            objc_sel(OBJC_SEL_INIT_WINDOW),
            rect,
            style_mask,
            NS_BACKING_STORE_BUFFERED,
            0,
        )
    };
    let title_ns: *mut c_void = macos_ns_string(config.get_title());
    if !title_ns.is_null() {
        let _: *mut c_void = objc_send_ptr1(ns_window, objc_sel(OBJC_SEL_SET_TITLE), title_ns);
    }
    if config.get_decorations() {
        objc_send_void_bool(ns_window, objc_sel(OBJC_SEL_SET_TITLEBAR_TRANSPARENT), true);
        objc_send_void_isize(ns_window, objc_sel(OBJC_SEL_SET_TITLE_VISIBILITY), 1);
    }
    let ns_view: *mut c_void = objc_send_ptr0(ns_window, objc_sel(OBJC_SEL_CONTENT_VIEW));
    let layer_cls: *mut c_void = objc_class(OBJC_CLS_CA_METAL_LAYER);
    let metal_layer: *mut c_void = objc_send_ptr0(layer_cls, objc_sel(OBJC_SEL_LAYER));
    if !ns_view.is_null() && !metal_layer.is_null() {
        let scale: f64 = macos_backing_scale_factor() as f64;
        objc_send_void_f64(metal_layer, objc_sel(OBJC_SEL_SET_CONTENTS_SCALE), scale);
        objc_send_void_bool(ns_view, objc_sel(OBJC_SEL_SET_WANTS_LAYER), true);
        let _: *mut c_void = objc_send_ptr1(ns_view, objc_sel(OBJC_SEL_SET_LAYER), metal_layer);
    }
    if config.get_always_on_top() {
        objc_send_void_isize(
            ns_window,
            objc_sel(OBJC_SEL_SET_LEVEL),
            NS_FLOATING_WINDOW_LEVEL,
        );
    }
    if config.get_visible() {
        objc_send_void_bool(ns_window, objc_sel(OBJC_SEL_SET_ACCEPTS_MOUSE_MOVED), true);
        let _: *mut c_void = objc_send_ptr0(ns_window, objc_sel(OBJC_SEL_CENTER));
        let _: *mut c_void =
            objc_send_ptr1(ns_window, objc_sel(OBJC_SEL_MAKE_KEY_FRONT), null_mut());
        if !app.is_null() {
            objc_send_void_bool(app, objc_sel(OBJC_SEL_ACTIVATE_IGNORING), true);
        }
    }
    Ok((ns_window as usize, ns_view as usize, metal_layer as usize))
}

/// Allocates a CoreGraphics device RGB color space pointer suitable for caching across frames.
///
/// # Returns
///
/// - `usize` - Raw `CGColorSpaceRef` pointer as `usize`.
pub(crate) fn macos_create_device_rgb_space() -> usize {
    // SAFETY: `CGColorSpaceCreateDeviceRGB` returns a retained `CGColorSpaceRef` valid for the process lifetime.
    unsafe { CGColorSpaceCreateDeviceRGB() as usize }
}

/// Blits an RGBA8 `Framebuffer` directly into a `CALayer` / `CAMetalLayer` via `CFData` + `CGImageCreate` + `setContents:`.
///
/// # Arguments
///
/// - `usize` - Raw `CALayer*` pointer.
/// - `usize` - Cached `CGColorSpaceRef` pointer.
/// - `&Framebuffer` - Source RGBA8 pixel framebuffer.
///
/// # Returns
///
/// - `Result<(), PlatformError>` - Presentation status.
pub(crate) fn macos_present_cg_image(
    layer_ptr: usize,
    cached_space: usize,
    framebuffer: &Framebuffer,
) -> Result<(), PlatformError> {
    if layer_ptr == 0 {
        return Ok(());
    }
    let width: usize = framebuffer.get_width() as usize;
    let height: usize = framebuffer.get_height() as usize;
    let bytes: &[u8] = framebuffer.get_pixels().as_slice();
    // SAFETY: `CFDataCreate` copies `bytes` into Apple-owned storage retained safely by CoreAnimation.
    unsafe {
        let owned_space: bool = cached_space == 0;
        let color_space: *mut c_void = if owned_space {
            CGColorSpaceCreateDeviceRGB()
        } else {
            cached_space as *mut c_void
        };
        let cf_data: *mut c_void = CFDataCreate(null(), bytes.as_ptr(), bytes.len() as isize);
        let provider: *mut c_void = if cf_data.is_null() {
            null_mut()
        } else {
            CGDataProviderCreateWithCFData(cf_data)
        };
        let cg_image: *mut c_void = if provider.is_null() {
            null_mut()
        } else {
            CGImageCreate(
                width,
                height,
                8,
                32,
                width * 4,
                color_space,
                CG_BITMAP_INFO_RGBA8,
                provider,
                null(),
                true,
                0,
            )
        };
        if !cg_image.is_null() {
            let _: *mut c_void = objc_send_ptr1(
                layer_ptr as *mut c_void,
                objc_sel(OBJC_SEL_SET_CONTENTS),
                cg_image,
            );
            CGImageRelease(cg_image);
        }
        if !provider.is_null() {
            CGDataProviderRelease(provider);
        }
        if !cf_data.is_null() {
            CFRelease(cf_data.cast_const());
        }
        if owned_space && !color_space.is_null() {
            CGColorSpaceRelease(color_space);
        }
    }
    Ok(())
}
