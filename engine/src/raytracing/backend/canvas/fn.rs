use super::*;

/// Acquires the 2D context for a canvas and reports why it failed.
///
/// # Arguments
///
/// - `&HtmlCanvasElement` - The canvas to draw into.
///
/// # Returns
///
/// - `Result<CanvasRenderingContext2d, JsValue>` - The context, or the
///   reason the browser refused one.
pub(crate) fn attach_canvas_context(
    canvas: &HtmlCanvasElement,
) -> Result<CanvasRenderingContext2d, JsValue> {
    let lookup: Result<Option<Object>, JsValue> = canvas.get_context(CANVAS_CONTEXT_ID_2D);
    let context_object: Object = match lookup {
        Ok(Some(object)) => object,
        Ok(None) => {
            return Err(backend_error(
                ERROR_CANVAS_UNAVAILABLE,
                ERROR_CANVAS_UNAVAILABLE.to_string(),
            ));
        }
        Err(reason) => return Err(backend_error(ERROR_CANVAS_UNAVAILABLE, describe(&reason))),
    };
    match context_object.dyn_into() {
        Ok(context) => Ok(context),
        Err(_) => Err(backend_error(
            ERROR_CANVAS_UNAVAILABLE,
            ERROR_CANVAS_UNAVAILABLE.to_string(),
        )),
    }
}

/// Allocates the surface's image and the array the image reads.
///
/// The two are built together because their lengths must agree by
/// construction: the array is allocated for exactly one surface, and
/// the `ImageData` is constructed over that exact array. A size the
/// browser refuses therefore fails here, before any state is changed.
///
/// # Arguments
///
/// - `u32` - The surface width in pixels.
///
/// # Returns
///
/// - `Result<(ImageData, Uint8ClampedArray), JsValue>` - The image and
///   its backing store, or the reason the browser refused them.
pub(crate) fn allocate_canvas_surface(
    width: u32,
    height: u32,
) -> Result<(ImageData, Uint8ClampedArray), JsValue> {
    let expected: usize = match surface_rgba_length(width, height) {
        Some(bytes) => bytes,
        None => {
            return Err(backend_error(
                ERROR_DIMENSION_OVERFLOW,
                format!("{width}x{height} overflows the addressable byte count"),
            ));
        }
    };
    let length: u32 = match u32::try_from(expected) {
        Ok(elements) => elements,
        Err(_) => {
            return Err(backend_error(
                ERROR_DIMENSION_OVERFLOW,
                format!("{width}x{height} needs {expected} bytes, over one typed array"),
            ));
        }
    };
    let backing: Uint8ClampedArray = Uint8ClampedArray::new_with_length(length);
    let image: ImageData =
        match ImageData::new_with_js_u8_clamped_array_and_sh(&backing, width, height) {
            Ok(value) => value,
            Err(reason) => return Err(backend_error(ERROR_CANVAS_IMAGE_DATA, describe(&reason))),
        };
    Ok((image, backing))
}

/// The RGBA byte length of one surface, or `None` when the size cannot
/// be addressed.
///
/// The multiplication is checked on both axes: a size that overflows
/// is not clamped into a smaller picture, it is refused, because a
/// silently clamped frame and a correctly sized frame are
/// indistinguishable on screen.
///
/// # Arguments
///
/// - `u32` - The surface width in pixels.
///
/// # Returns
///
/// - `Option<usize>` - `width * height * 4`, or `None` when that
///   count overflows.
pub(crate) fn surface_rgba_length(width: u32, height: u32) -> Option<usize> {
    (width as usize)
        .checked_mul(height as usize)?
        .checked_mul(4)
}
