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

/// Locks an `ANativeWindow*`, copies RGBA8 rows respecting hardware stride, and posts the frame.
///
/// # Arguments
///
/// - `usize` - Raw `ANativeWindow*` pointer.
/// - `&Framebuffer` - Rendered RGBA8 framebuffer.
///
/// # Returns
///
/// - `Result<(), PlatformError>` - Presentation status.
pub(crate) fn android_present_native_window(
    native_window: usize,
    framebuffer: &Framebuffer,
) -> Result<(), PlatformError> {
    if native_window == 0 {
        return Ok(());
    }
    let src_w: usize = framebuffer.get_width() as usize;
    let src_h: usize = framebuffer.get_height() as usize;
    let src_bytes: &[u8] = framebuffer.get_pixels().as_slice();
    let mut buffer: ANativeWindowBuffer = ANativeWindowBuffer {
        width: 0,
        height: 0,
        stride: 0,
        format: 0,
        bits: null_mut(),
        reserved: [0; 6],
    };
    // SAFETY: Locks the NDK hardware surface buffer, copies bounded scanlines, and always unlocks.
    unsafe {
        let _: i32 = ANativeWindow_setBuffersGeometry(
            native_window as *mut c_void,
            src_w as i32,
            src_h as i32,
            ANDROID_WINDOW_FORMAT_RGBA_8888,
        );
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
        let _: i32 = ANativeWindow_unlockAndPost(native_window as *mut c_void);
    }
    Ok(())
}
