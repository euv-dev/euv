use super::*;

impl CanvasRayTraceBackend {
    /// Acquires the 2D context and allocates the surface's image and
    /// backing array.
    ///
    /// # Arguments
    ///
    /// - `&HtmlCanvasElement` - The canvas to draw into.
    /// - `u32` - The surface width in pixels.
    /// - `u32` - The surface height in pixels.
    ///
    /// # Returns
    ///
    /// - `Result<Self, JsValue>` - The ready backend, or the reason the
    ///   browser refused a 2D context or an image.
    pub fn attach(canvas: &HtmlCanvasElement, width: u32, height: u32) -> Result<Self, JsValue> {
        let context: CanvasRenderingContext2d = attach_canvas_context(canvas)?;
        let (image, backing): (ImageData, Uint8ClampedArray) =
            allocate_canvas_surface(width, height)?;
        Ok(Self {
            context,
            image,
            backing,
            image_width: width,
            image_height: height,
        })
    }

    /// The largest per-axis surface length this backend accepts.
    ///
    /// Canvas 2D exposes no driver limit of its own: the browser
    /// rejects an oversized surface through the `ImageData`
    /// constructor, which is a per-call error, not a queryable
    /// budget. Reporting `None` leaves that rejection to the one place
    /// that can actually observe it.
    ///
    /// # Returns
    ///
    /// - `Option<u32>` - `None`, because no Canvas 2D limit exists to
    ///   enforce up front.
    pub fn dimension_limit(&self) -> Option<u32> {
        None
    }

    /// Reallocates the surface only when the size actually changed.
    ///
    /// The new image and backing array are allocated first and swapped
    /// in only once the allocation has been accepted, so a rejected
    /// size leaves the previous, still renderable, surface untouched.
    /// The canvas element's own width and height are the caller's to
    /// commit: this backend never depends on them having changed
    /// already.
    ///
    /// # Arguments
    ///
    /// - `u32` - The surface width in pixels.
    /// - `u32` - The surface height in pixels.
    ///
    /// # Returns
    ///
    /// - `Result<(), JsValue>` - `Ok` once the surface is sized.
    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), JsValue> {
        if self.get_image_width() == width && self.get_image_height() == height {
            return Ok(());
        }
        let (image, backing): (ImageData, Uint8ClampedArray) =
            allocate_canvas_surface(width, height)?;
        self.set_image(image);
        self.set_backing(backing);
        self.set_image_width(width);
        self.set_image_height(height);
        Ok(())
    }

    /// Copies the traced RGBA bytes into the reused image and blits it.
    ///
    /// The bytes go through the backing array's own `copy_from`, which
    /// writes the JavaScript `Uint8ClampedArray` the `ImageData` was
    /// constructed over. `ImageData::data` cannot be used for this:
    /// its `Clamped<Vec<u8>>` is a Rust copy, so writing into it leaves
    /// the image's own store untouched and the blit presents the
    /// original transparent pixels.
    ///
    /// The length is exact. A buffer that no longer matches the surface
    /// is an error rather than a truncated or padded frame, because a
    /// clipped upload still produces a picture that only a pixel
    /// comparison can catch.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - The traced pixels, one RGBA byte per channel.
    ///
    /// # Returns
    ///
    /// - `Result<(), JsValue>` - `Ok` once the blit was accepted.
    pub fn present(&mut self, pixels: &[u8]) -> Result<(), JsValue> {
        let width: u32 = self.get_image_width();
        let height: u32 = self.get_image_height();
        if width == 0 || height == 0 {
            return Err(backend_error(
                ERROR_CANVAS_RELEASED,
                ERROR_CANVAS_RELEASED.to_string(),
            ));
        }
        let expected: usize = match surface_rgba_length(width, height) {
            Some(bytes) => bytes,
            None => {
                return Err(backend_error(
                    ERROR_DIMENSION_OVERFLOW,
                    format!("{width}x{height} overflows the addressable byte count"),
                ));
            }
        };
        if pixels.len() != expected {
            return Err(backend_error(
                ERROR_CANVAS_PIXEL_LENGTH,
                format!("{} bytes for {expected} byte surface", pixels.len()),
            ));
        }
        self.get_backing().copy_from(pixels);
        match self.get_context().put_image_data(
            self.get_image(),
            CANVAS_BLIT_ORIGIN,
            CANVAS_BLIT_ORIGIN,
        ) {
            Ok(()) => Ok(()),
            Err(reason) => Err(backend_error(ERROR_CANVAS_PUT, describe(&reason))),
        }
    }

    /// Marks the surface as released.
    ///
    /// The `ImageData`, its backing array, and the 2D context are
    /// JavaScript objects owned by this struct, so they are reclaimed
    /// when the adapter holding the backend is dropped; there is no
    /// explicit teardown call for any of them. What release does own
    /// is the surface size: zeroing it means a later `present` is
    /// rejected on the length check instead of blitting a surface the
    /// caller has already torn down. The 2D context belongs to the
    /// canvas, so it is never destroyed here.
    ///
    /// # Returns
    ///
    /// - `()` - Cleanup never fails.
    pub fn release(&mut self) {
        self.set_image_width(0);
        self.set_image_height(0);
    }
}
