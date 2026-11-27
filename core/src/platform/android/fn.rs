use super::*;

#[link(name = "android")]
unsafe extern "C" {
    /// Body of the `ANativeWindow_getWidth` free function.
    ///
    /// # Arguments
    ///
    /// - `*mut c_void` - A `*mut c_void` parameter.
    ///
    /// # Returns
    ///
    /// - `i32` - A 32-bit signed integer.
    fn ANativeWindow_getWidth(window: *mut c_void) -> i32;
    /// Body of the `ANativeWindow_getHeight` free function.
    ///
    /// # Arguments
    ///
    /// - `*mut c_void` - A `*mut c_void` parameter.
    ///
    /// # Returns
    ///
    /// - `i32` - A 32-bit signed integer.
    fn ANativeWindow_getHeight(window: *mut c_void) -> i32;
    /// Body of the `ANativeWindow_setBuffersGeometry` free function.
    ///
    /// # Arguments
    ///
    /// - `*mut c_void` - A `*mut c_void` parameter.
    /// - `i32` - A 32-bit signed integer (`i32`).
    /// - `i32` - A 32-bit signed integer (`i32`).
    /// - `i32` - A 32-bit signed integer (`i32`).
    ///
    /// # Returns
    ///
    /// - `i32` - A 32-bit signed integer.
    fn ANativeWindow_setBuffersGeometry(
        window: *mut c_void,
        width: i32,
        height: i32,
        format: i32,
    ) -> i32;
    /// Body of the `ANativeWindow_lock` free function.
    ///
    /// # Arguments
    ///
    /// - `*mut c_void` - A `*mut c_void` parameter.
    /// - `*mut ANativeWindowBuffer` - A `*mut ANativeWindowBuffer` parameter.
    /// - `*mut c_void` - A `*mut c_void` parameter.
    ///
    /// # Returns
    ///
    /// - `i32` - A 32-bit signed integer.
    fn ANativeWindow_lock(
        window: *mut c_void,
        out_buffer: *mut ANativeWindowBuffer,
        in_out_dirty_bounds: *mut c_void,
    ) -> i32;
    /// Body of the `ANativeWindow_unlockAndPost` free function.
    ///
    /// # Arguments
    ///
    /// - `*mut c_void` - A `*mut c_void` parameter.
    ///
    /// # Returns
    ///
    /// - `i32` - A 32-bit signed integer.
    fn ANativeWindow_unlockAndPost(window: *mut c_void) -> i32;
}

unsafe extern "C" {
    /// Dynamically opens a shared library on Android bionic.
    ///
    /// # Arguments
    ///
    /// - `*const c_char` - Null-terminated library name.
    /// - `i32` - Dynamic linker flags.
    ///
    /// # Returns
    ///
    /// - `*mut c_void` - Library handle or null.
    fn dlopen(filename: *const c_char, flag: i32) -> *mut c_void;
    /// Dynamically resolves a symbol pointer from a loaded Android shared library.
    ///
    /// # Arguments
    ///
    /// - `*mut c_void` - Loaded library handle.
    /// - `*const c_char` - Null-terminated symbol name.
    ///
    /// # Returns
    ///
    /// - `*mut c_void` - Resolved symbol pointer or null.
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

/// Hints the preferred high-refresh frame rate to Android SurfaceFlinger via `ANativeWindow_setFrameRateWithChangeStrategy` (API 31+) or `ANativeWindow_setFrameRate` (API 30+).
///
/// # Arguments
///
/// - `usize` - Raw `ANativeWindow*` pointer.
/// - `f32` - Desired hardware refresh rate in Hertz (`60.0`..`480.0`).
///
/// # Returns
///
/// - `bool` - `true` when the NDK frame-rate hint symbol was resolved and returned status `0`.
pub fn android_set_native_window_frame_rate(native_window: usize, refresh_hz: f32) -> bool {
    if native_window == 0 || !refresh_hz.is_finite() || refresh_hz < MIN_REFRESH_RATE_HZ {
        return false;
    }
    let hz: f32 = refresh_hz.clamp(MIN_REFRESH_RATE_HZ, MAX_REFRESH_RATE_HZ);
    // SAFETY: Dynamically resolves optional API 30+/31+ NDK frame-rate symbols from `libandroid.so`.
    unsafe {
        let lib: *mut c_void = dlopen(
            ANDROID_LIB_NAME.as_ptr().cast::<c_char>(),
            ANDROID_RTLD_LAZY,
        );
        if lib.is_null() {
            return false;
        }
        let sym31: *mut c_void = dlsym(
            lib,
            SYM_ANDROID_SET_FRAME_RATE_STRATEGY
                .as_ptr()
                .cast::<c_char>(),
        );
        if !sym31.is_null() {
            let func: unsafe extern "C" fn(*mut c_void, f32, i8, i8) -> i32 = transmute(sym31);
            return func(
                native_window as *mut c_void,
                hz,
                ANDROID_FRAME_RATE_COMPAT_FIXED_SOURCE,
                ANDROID_CHANGE_FRAME_RATE_SEAMLESS,
            ) == 0;
        }
        let sym30: *mut c_void = dlsym(lib, SYM_ANDROID_SET_FRAME_RATE.as_ptr().cast::<c_char>());
        if !sym30.is_null() {
            let func: unsafe extern "C" fn(*mut c_void, f32, i8) -> i32 = transmute(sym30);
            return func(
                native_window as *mut c_void,
                hz,
                ANDROID_FRAME_RATE_COMPAT_FIXED_SOURCE,
            ) == 0;
        }
        false
    }
}

/// Queries physical pixel dimensions from an attached `ANativeWindow*`.
///
/// # Arguments
///
/// - `usize` - Raw `ANativeWindow*` pointer.
///
/// # Returns
///
/// - `Option<(u32, u32)>` - Physical `(width, height)` when the native window is attached.
#[must_use]
pub fn android_query_window_size(native_window: usize) -> Option<(u32, u32)> {
    if native_window == 0 {
        return None;
    }
    // SAFETY: Caller supplies a live `ANativeWindow*` owned by the Android lifecycle.
    unsafe {
        let w: i32 = ANativeWindow_getWidth(native_window as *mut c_void);
        let h: i32 = ANativeWindow_getHeight(native_window as *mut c_void);
        if w > 0 && h > 0 {
            Some((w as u32, h as u32))
        } else {
            None
        }
    }
}

/// Locks an `ANativeWindow*`, copies RGBA8 pixels using a single contiguous fast-path when stride matches, and posts the frame.
///
/// # Arguments
///
/// - `usize` - Raw `ANativeWindow*` pointer.
/// - `&Framebuffer` - Rendered RGBA8 framebuffer.
/// - `&mut u32` - Cached surface width avoiding redundant `ANativeWindow_setBuffersGeometry` calls.
/// - `&mut u32` - Cached surface height avoiding redundant `ANativeWindow_setBuffersGeometry` calls.
///
/// # Returns
///
/// - `Result<(), PlatformError>` - Presentation status.
pub(crate) fn android_present_native_window(
    native_window: usize,
    framebuffer: &Framebuffer,
    cached_w: &mut u32,
    cached_h: &mut u32,
) -> Result<(), PlatformError> {
    if native_window == 0 {
        return Ok(());
    }
    let fb_w: u32 = framebuffer.get_width();
    let fb_h: u32 = framebuffer.get_height();
    let src_w: usize = fb_w as usize;
    let src_h: usize = fb_h as usize;
    let src_bytes: &[u8] = framebuffer.get_pixels().as_slice();
    let mut buffer: ANativeWindowBuffer = ANativeWindowBuffer {
        width: 0,
        height: 0,
        stride: 0,
        format: 0,
        bits: null_mut(),
        reserved: [0; 6],
    };
    // SAFETY: Locks the NDK hardware surface buffer, copies bounded RGBA8 scanlines, and always unlocks.
    unsafe {
        if *cached_w != fb_w || *cached_h != fb_h {
            let _: i32 = ANativeWindow_setBuffersGeometry(
                native_window as *mut c_void,
                src_w as i32,
                src_h as i32,
                ANDROID_WINDOW_FORMAT_RGBA_8888,
            );
            *cached_w = fb_w;
            *cached_h = fb_h;
        }
        let status: i32 = ANativeWindow_lock(native_window as *mut c_void, &mut buffer, null_mut());
        if status != 0 {
            return Err(PlatformError::SurfaceError(
                ERR_ANDROID_LOCK_SURFACE.to_string(),
            ));
        }
        if !buffer.bits.is_null() && buffer.stride > 0 && buffer.height > 0 {
            let copy_rows: usize = src_h.min(buffer.height as usize);
            let copy_cols: usize = src_w.min(buffer.width.max(0) as usize);
            let dst_stride_bytes: usize = (buffer.stride as usize) * 4;
            let src_stride_bytes: usize = src_w * 4;
            let row_copy_bytes: usize = copy_cols * 4;
            let dst_base: *mut u8 = buffer.bits.cast::<u8>();
            if copy_cols == src_w
                && src_stride_bytes == dst_stride_bytes
                && copy_rows.saturating_mul(row_copy_bytes) <= src_bytes.len()
            {
                copy_nonoverlapping(
                    src_bytes.as_ptr(),
                    dst_base,
                    copy_rows.saturating_mul(row_copy_bytes),
                );
            } else {
                for row in 0..copy_rows {
                    let src_off: usize = row * src_stride_bytes;
                    let dst_off: usize = row * dst_stride_bytes;
                    if src_off + row_copy_bytes <= src_bytes.len() {
                        copy_nonoverlapping(
                            src_bytes.as_ptr().add(src_off),
                            dst_base.add(dst_off),
                            row_copy_bytes,
                        );
                    }
                }
            }
        }
        let _: i32 = ANativeWindow_unlockAndPost(native_window as *mut c_void);
    }
    Ok(())
}
