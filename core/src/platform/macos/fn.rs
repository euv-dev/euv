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
    /// Body of the `CGDataProviderCreateWithData` free function.
    ///
    /// # Arguments
    ///
    /// - `*mut c_void` - A `*mut c_void` parameter.
    /// - `*const c_void` - A `*const c_void` parameter.
    /// - `usize` - A non-negative integer (`usize`).
    /// - `*const c_void` - A `*const c_void` parameter.
    ///
    /// # Returns
    ///
    /// - `*mut c_void` - A `*mut c_void` value.
    fn CGDataProviderCreateWithData(
        info: *mut c_void,
        data: *const c_void,
        size: usize,
        release_data: *const c_void,
    ) -> *mut c_void;
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
    let win_cls: *mut c_void = objc_class(OBJC_CLS_NS_WINDOW);
    if win_cls.is_null() {
        return Err(PlatformError::InitializationFailed(
            ERR_MACOS_APPKIT_MISSING.to_string(),
        ));
    }
    let allocated: *mut c_void = objc_send_ptr0(win_cls, objc_sel(OBJC_SEL_ALLOC));
    let mut style_mask: usize = if config.get_decorations() {
        let base_mask: usize = NS_WINDOW_STYLE_TITLED | NS_WINDOW_STYLE_CLOSABLE;
        base_mask | NS_WINDOW_STYLE_MINIATURIZABLE
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
    let ns_view: *mut c_void = objc_send_ptr0(ns_window, objc_sel(OBJC_SEL_CONTENT_VIEW));
    let layer_cls: *mut c_void = objc_class(OBJC_CLS_CA_METAL_LAYER);
    let metal_layer: *mut c_void = objc_send_ptr0(layer_cls, objc_sel(OBJC_SEL_LAYER));
    if !ns_view.is_null() && !metal_layer.is_null() {
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
        let _: *mut c_void =
            objc_send_ptr1(ns_window, objc_sel(OBJC_SEL_MAKE_KEY_FRONT), null_mut());
    }
    Ok((ns_window as usize, ns_view as usize, metal_layer as usize))
}

/// Blits an RGBA8 `Framebuffer` directly into a `CALayer` / `CAMetalLayer` via `CGImageCreate` + `setContents:`.
///
/// # Arguments
///
/// - `usize` - Raw `CALayer*` pointer.
/// - `&Framebuffer` - Source RGBA8 pixel framebuffer.
///
/// # Returns
///
/// - `Result<(), PlatformError>` - Presentation status.
pub(crate) fn macos_present_cg_image(
    layer_ptr: usize,
    framebuffer: &Framebuffer,
) -> Result<(), PlatformError> {
    if layer_ptr == 0 {
        return Ok(());
    }
    let width: usize = framebuffer.get_width() as usize;
    let height: usize = framebuffer.get_height() as usize;
    let bytes: &[u8] = framebuffer.get_pixels().as_slice();
    // SAFETY: CoreGraphics image wraps `bytes` synchronously during `setContents:` and releases objects cleanly.
    unsafe {
        let color_space: *mut c_void = CGColorSpaceCreateDeviceRGB();
        let provider: *mut c_void = CGDataProviderCreateWithData(
            null_mut(),
            bytes.as_ptr().cast::<c_void>(),
            bytes.len(),
            null(),
        );
        let cg_image: *mut c_void = CGImageCreate(
            width,
            height,
            8,
            32,
            width * 4,
            color_space,
            CG_BITMAP_INFO_RGBA8,
            provider,
            null(),
            false,
            0,
        );
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
        if !color_space.is_null() {
            CGColorSpaceRelease(color_space);
        }
    }
    Ok(())
}
