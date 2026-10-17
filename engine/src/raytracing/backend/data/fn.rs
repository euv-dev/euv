use super::*;

/// Describes a thrown [`JsValue`] without ever losing it.
///
/// # Arguments
///
/// - `&JsValue` - The thrown value.
///
/// # Returns
///
/// - `String` - The value rendered as a string, or a debug rendering
///   when the value is not convertible.
pub(crate) fn describe(reason: &JsValue) -> String {
    match reason.as_string() {
        Some(text) if !text.is_empty() => text,
        _ => format!("{reason:?}"),
    }
}

/// Builds a [`JsValue`] error from a stable code and a detail string.
///
/// Every backend failure surfaces through this one constructor, so a
/// caller can branch on the code while a log keeps the driver's own
/// wording. A failure never degrades to `JsValue::UNDEFINED`, which
/// would make a dropped frame indistinguishable from a successful one.
///
/// # Arguments
///
/// - `&str` - The stable, machine-readable error code.
/// - `String` - The detail appended after the code.
///
/// # Returns
///
/// - `JsValue` - The error value.
pub(crate) fn backend_error(code: &str, detail: String) -> JsValue {
    JsValue::from_str(format!("[{code}] {detail}").as_str())
}

/// Validates a requested surface size and reports the RGBA bytes it
/// needs.
///
/// Two distinct budgets are checked, and neither is a substitute for
/// the other. The byte count is computed with checked arithmetic so a
/// request that overflows the address space is rejected instead of
/// wrapping into a short allocation; the optional limit is a
/// per-axis pixel ceiling the driver enforces on one texture or viewport
/// dimension, so it is compared against each dimension rather than
/// against the byte total. The byte budget of a storage binding is not
/// consulted here at all: a surface is not a packed buffer, and
/// rejecting a perfectly renderable size because it would exceed a
/// binding limit is the wrong failure.
///
/// The dimension is never silently clamped: a caller that asks for a
/// surface the hardware cannot address gets an error and keeps the
/// previous, still valid, size.
///
/// # Arguments
///
/// - `u32` - The requested width in pixels.
/// - `u32` - The requested height in pixels.
/// - `Option<u32>` - The largest pixel count allowed on one axis, when
///   the backend exposes one.
///
/// # Returns
///
/// - `Result<usize, JsValue>` - The RGBA byte length of the surface, or
///   the reason the size is not renderable.
pub(crate) fn validate_surface(
    width: u32,
    height: u32,
    limit: Option<u32>,
) -> Result<usize, JsValue> {
    if width == 0 || height == 0 {
        return Err(backend_error(
            ERROR_ZERO_DIMENSION,
            format!("requested {width}x{height}"),
        ));
    }
    let pixels: usize = match (width as usize).checked_mul(height as usize) {
        Some(product) => product,
        None => {
            return Err(backend_error(
                ERROR_DIMENSION_OVERFLOW,
                format!("{width}x{height} overflows the addressable pixel count"),
            ));
        }
    };
    let bytes: usize = match pixels.checked_mul(SURFACE_RGBA_CHANNELS) {
        Some(product) => product,
        None => {
            return Err(backend_error(
                ERROR_DIMENSION_OVERFLOW,
                format!("{width}x{height} overflows the addressable byte count"),
            ));
        }
    };
    if let Some(maximum) = limit
        && (width > maximum || height > maximum)
    {
        return Err(backend_error(
            ERROR_GPU_LIMIT,
            format!("{width}x{height} exceeds the {maximum} pixel limit on one axis"),
        ));
    }
    Ok(bytes)
}

/// Resizes a reused output buffer to exactly one surface of RGBA bytes.
///
/// The buffer is only reallocated when it has to grow, so a repeated
/// render at a fixed size reuses the same allocation.
///
/// # Arguments
///
/// - `&mut Vec<u8>` - The reused buffer.
/// - `usize` - The RGBA byte length of one surface.
///
/// # Returns
///
/// - `()` - The resize itself.
pub(crate) fn resize_pixel_buffer(pixels: &mut Vec<u8>, length: usize) {
    if pixels.len() < length {
        pixels.resize(length, 0);
    } else {
        pixels.truncate(length);
    }
}
