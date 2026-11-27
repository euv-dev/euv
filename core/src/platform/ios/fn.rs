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

#[link(name = "UIKit", kind = "framework")]
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

/// Sends a 0-arg Objective-C message returning a pointer on iOS / iPadOS.
///
/// # Arguments
///
/// - `*mut c_void` - Objective-C receiver.
/// - `*mut c_void` - Selector pointer.
///
/// # Returns
///
/// - `*mut c_void` - Returned object pointer.
pub(crate) fn uikit_send_ptr0(receiver: *mut c_void, sel: *mut c_void) -> *mut c_void {
    if receiver.is_null() {
        return null_mut();
    }
    // SAFETY: Matches 64-bit ARM64/x86_64 Apple Objective-C ABI for 0-arg pointer return.
    unsafe {
        let func: unsafe extern "C" fn(*mut c_void, *mut c_void) -> *mut c_void =
            transmute(objc_msgSend as *const ());
        func(receiver, sel)
    }
}

/// Sends a 0-arg Objective-C message returning `isize` (`NSInteger`).
///
/// # Arguments
///
/// - `*mut c_void` - Objective-C receiver.
/// - `*mut c_void` - Selector pointer.
///
/// # Returns
///
/// - `isize` - Returned `NSInteger`.
pub(crate) fn uikit_send_isize0(receiver: *mut c_void, sel: *mut c_void) -> isize {
    if receiver.is_null() {
        return 0;
    }
    // SAFETY: Matches 64-bit Apple Objective-C ABI for `NSInteger`.
    unsafe {
        let func: unsafe extern "C" fn(*mut c_void, *mut c_void) -> isize =
            transmute(objc_msgSend as *const ());
        func(receiver, sel)
    }
}

/// Sends a 0-arg Objective-C message returning `f64` (`CGFloat`).
///
/// # Arguments
///
/// - `*mut c_void` - Objective-C receiver.
/// - `*mut c_void` - Selector pointer.
///
/// # Returns
///
/// - `f64` - Returned `CGFloat`.
pub(crate) fn uikit_send_f64_0(receiver: *mut c_void, sel: *mut c_void) -> f64 {
    if receiver.is_null() {
        return 2.0;
    }
    // SAFETY: Matches 64-bit Apple Objective-C ABI for `CGFloat`.
    unsafe {
        let func: unsafe extern "C" fn(*mut c_void, *mut c_void) -> f64 =
            transmute(objc_msgSend as *const ());
        func(receiver, sel)
    }
}

/// Queries `UIDevice.currentDevice.userInterfaceIdiom` to distinguish iOS Phone vs iPadOS Tablet.
///
/// # Returns
///
/// - `PlatformKind` - `PlatformKind::IpadOs` on iPad hardware, `PlatformKind::Ios` on iPhone.
#[must_use]
pub fn ios_detect_platform_kind() -> PlatformKind {
    // SAFETY: Static null-terminated Objective-C class and selector names.
    let (device_cls, sel_current, sel_idiom): (*mut c_void, *mut c_void, *mut c_void) = unsafe {
        (
            objc_getClass(UIKIT_CLS_UI_DEVICE.as_ptr().cast::<c_char>()),
            sel_registerName(UIKIT_SEL_CURRENT_DEVICE.as_ptr().cast::<c_char>()),
            sel_registerName(UIKIT_SEL_USER_INTERFACE_IDIOM.as_ptr().cast::<c_char>()),
        )
    };
    let device: *mut c_void = uikit_send_ptr0(device_cls, sel_current);
    let idiom: isize = uikit_send_isize0(device, sel_idiom);
    if idiom == UI_USER_INTERFACE_IDIOM_PAD {
        PlatformKind::IpadOs
    } else {
        PlatformKind::Ios
    }
}

/// Queries the native Retina scale factor via `UIScreen.mainScreen.nativeScale`.
///
/// # Returns
///
/// - `f32` - Device scale factor (`2.0` or `3.0`).
#[must_use]
pub fn ios_screen_scale() -> f32 {
    // SAFETY: Static null-terminated Objective-C class and selector names.
    let (screen_cls, sel_main, sel_scale): (*mut c_void, *mut c_void, *mut c_void) = unsafe {
        (
            objc_getClass(UIKIT_CLS_UI_SCREEN.as_ptr().cast::<c_char>()),
            sel_registerName(UIKIT_SEL_MAIN_SCREEN.as_ptr().cast::<c_char>()),
            sel_registerName(UIKIT_SEL_NATIVE_SCALE.as_ptr().cast::<c_char>()),
        )
    };
    let screen: *mut c_void = uikit_send_ptr0(screen_cls, sel_main);
    let scale: f64 = uikit_send_f64_0(screen, sel_scale);
    if scale > 0.0 { scale as f32 } else { 2.0 }
}

/// Queries the active iOS / iPadOS dark mode appearance via `UIScreen.mainScreen.traitCollection.userInterfaceStyle`.
///
/// # Returns
///
/// - `ThemeMode` - Active UIKit interface style.
#[must_use]
pub fn ios_system_theme() -> ThemeMode {
    // SAFETY: Static null-terminated Objective-C class and selector names.
    let (screen_cls, sel_main, sel_traits, sel_style): (
        *mut c_void,
        *mut c_void,
        *mut c_void,
        *mut c_void,
    ) = unsafe {
        (
            objc_getClass(UIKIT_CLS_UI_SCREEN.as_ptr().cast::<c_char>()),
            sel_registerName(UIKIT_SEL_MAIN_SCREEN.as_ptr().cast::<c_char>()),
            sel_registerName(UIKIT_SEL_TRAIT_COLLECTION.as_ptr().cast::<c_char>()),
            sel_registerName(UIKIT_SEL_USER_INTERFACE_STYLE.as_ptr().cast::<c_char>()),
        )
    };
    let screen: *mut c_void = uikit_send_ptr0(screen_cls, sel_main);
    let traits: *mut c_void = uikit_send_ptr0(screen, sel_traits);
    let style: isize = uikit_send_isize0(traits, sel_style);
    if style == UI_USER_INTERFACE_STYLE_DARK {
        ThemeMode::Dark
    } else {
        ThemeMode::Light
    }
}

/// Allocates a `UIWindow`, `UIView`, and `CAMetalLayer` via direct UIKit / QuartzCore FFI.
///
/// # Returns
///
/// - `(usize, usize, usize)` - Tuple `(ui_window, ui_view, metal_layer)`.
pub(crate) fn ios_create_native_handles() -> (usize, usize, usize) {
    // SAFETY: Static null-terminated Objective-C names.
    let (win_cls, view_cls, layer_cls, sel_alloc, sel_init, sel_layer): (
        *mut c_void,
        *mut c_void,
        *mut c_void,
        *mut c_void,
        *mut c_void,
        *mut c_void,
    ) = unsafe {
        (
            objc_getClass(UIKIT_CLS_UI_WINDOW.as_ptr().cast::<c_char>()),
            objc_getClass(UIKIT_CLS_UI_VIEW.as_ptr().cast::<c_char>()),
            objc_getClass(UIKIT_CLS_CA_METAL_LAYER.as_ptr().cast::<c_char>()),
            sel_registerName(UIKIT_SEL_ALLOC.as_ptr().cast::<c_char>()),
            sel_registerName(UIKIT_SEL_INIT.as_ptr().cast::<c_char>()),
            sel_registerName(UIKIT_SEL_LAYER.as_ptr().cast::<c_char>()),
        )
    };
    let ui_window: *mut c_void = uikit_send_ptr0(uikit_send_ptr0(win_cls, sel_alloc), sel_init);
    let ui_view: *mut c_void = uikit_send_ptr0(uikit_send_ptr0(view_cls, sel_alloc), sel_init);
    let metal_layer: *mut c_void = uikit_send_ptr0(layer_cls, sel_layer);
    (ui_window as usize, ui_view as usize, metal_layer as usize)
}

/// Presents an RGBA8 `Framebuffer` directly onto a UIKit `CALayer` / `CAMetalLayer` via CoreGraphics.
///
/// # Arguments
///
/// - `usize` - Raw `CALayer*` pointer.
/// - `&Framebuffer` - Rendered RGBA8 pixel buffer.
///
/// # Returns
///
/// - `Result<(), PlatformError>` - Presentation status.
pub(crate) fn ios_present_cg_image(
    layer_ptr: usize,
    framebuffer: &Framebuffer,
) -> Result<(), PlatformError> {
    if layer_ptr == 0 {
        return Ok(());
    }
    let width: usize = framebuffer.get_width() as usize;
    let height: usize = framebuffer.get_height() as usize;
    let bytes: &[u8] = framebuffer.get_pixels().as_slice();
    // SAFETY: CoreGraphics wraps `bytes` synchronously during `setContents:` and releases all CF refs.
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
            IOS_CG_BITMAP_INFO_RGBA8,
            provider,
            null(),
            false,
            0,
        );
        if !cg_image.is_null() {
            let sel: *mut c_void =
                sel_registerName(UIKIT_SEL_SET_CONTENTS.as_ptr().cast::<c_char>());
            let func: unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void) -> *mut c_void =
                transmute(objc_msgSend as *const ());
            let _: *mut c_void = func(layer_ptr as *mut c_void, sel, cg_image);
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

/// Pauses or resumes a UIKit `CADisplayLink*` via `-[CADisplayLink setPaused:]`.
///
/// # Arguments
///
/// - `usize` - Raw `CADisplayLink*` pointer (`0` when no display link is attached).
/// - `bool` - `true` to pause display-link ticks in background; `false` to resume.
pub(crate) fn uikit_set_display_link_paused(display_link: usize, paused: bool) {
    if display_link == 0 {
        return;
    }
    // SAFETY: `objc_msgSend` matches `void (id, SEL, BOOL)` ABI on 64-bit Apple platforms.
    unsafe {
        let sel: *mut c_void = sel_registerName(UIKIT_SEL_SET_PAUSED.as_ptr().cast::<c_char>());
        let func: unsafe extern "C" fn(*mut c_void, *mut c_void, i8) =
            transmute(objc_msgSend as *const ());
        func(display_link as *mut c_void, sel, if paused { 1 } else { 0 });
    }
}
