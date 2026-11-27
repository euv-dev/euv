use super::*;

/// Pixel access, blitting, BGRA conversion, and deterministic hashing for `Framebuffer`.
impl Framebuffer {
    /// Allocates a new zero-initialized RGBA8 framebuffer of `(width, height)`.
    ///
    /// # Arguments
    ///
    /// - `u32` - Physical width in pixels.
    /// - `u32` - Physical height in pixels.
    ///
    /// # Returns
    ///
    /// - `Self` - Allocated framebuffer.
    #[must_use]
    pub fn allocate(width: u32, height: u32) -> Self {
        let w: u32 = width.max(1);
        let h: u32 = height.max(1);
        let byte_len: usize = (w as usize).saturating_mul(h as usize).saturating_mul(4);
        Self {
            width: w,
            height: h,
            pixels: vec![0; byte_len],
        }
    }

    /// Resizes the framebuffer only when dimensions changed, avoiding per-frame reallocation.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable framebuffer.
    /// - `u32` - Target width.
    /// - `u32` - Target height.
    ///
    /// # Returns
    ///
    /// - `bool` - True if dimensions changed and buffer was resized.
    pub fn resize(&mut self, width: u32, height: u32) -> bool {
        let w: u32 = width.max(1);
        let h: u32 = height.max(1);
        if self.get_width() == w && self.get_height() == h {
            return false;
        }
        self.set_width(w);
        self.set_height(h);
        let byte_len: usize = (w as usize).saturating_mul(h as usize).saturating_mul(4);
        self.get_mut_pixels().resize(byte_len, 0);
        true
    }

    /// Fills the entire framebuffer with solid `color` using 64-bit wide stores.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable framebuffer.
    /// - `Color` - Clear color.
    pub fn clear(&mut self, color: Color) {
        let rgba: [u8; 4] = color.to_rgba_bytes();
        let pair: [u8; 8] = [
            rgba[0], rgba[1], rgba[2], rgba[3], rgba[0], rgba[1], rgba[2], rgba[3],
        ];
        let (wide, tail): (&mut [[u8; 8]], &mut [u8]) = self.get_mut_pixels().as_chunks_mut::<8>();
        for chunk in wide.iter_mut() {
            *chunk = pair;
        }
        for chunk in tail.as_chunks_mut::<4>().0.iter_mut() {
            *chunk = rgba;
        }
    }

    /// Fills or composites a contiguous horizontal pixel span `[x0, x1)` on row `y`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable framebuffer.
    /// - `u32` - Inclusive start X pixel coordinate.
    /// - `u32` - Exclusive end X pixel coordinate.
    /// - `u32` - Row Y pixel coordinate.
    /// - `Color` - Foreground color to write or alpha-blend.
    pub fn fill_span(&mut self, x0: u32, x1: u32, y: u32, color: Color) {
        let sa: u8 = color.get_a();
        let width: u32 = self.get_width();
        let height: u32 = self.get_height();
        if sa == 0 || y >= height || x0 >= x1 || x0 >= width {
            return;
        }
        let end_x: u32 = x1.min(width);
        let row_base: usize = (y as usize) * (width as usize);
        let start_byte: usize = (row_base + (x0 as usize)) * 4;
        let end_byte: usize = (row_base + (end_x as usize)) * 4;
        let Some(span) = self.get_mut_pixels().get_mut(start_byte..end_byte) else {
            return;
        };
        if sa == 255 {
            let rgba: [u8; 4] = [color.get_r(), color.get_g(), color.get_b(), 255];
            let pair: [u8; 8] = [
                rgba[0], rgba[1], rgba[2], rgba[3], rgba[0], rgba[1], rgba[2], rgba[3],
            ];
            let (wide, tail): (&mut [[u8; 8]], &mut [u8]) = span.as_chunks_mut::<8>();
            for chunk in wide.iter_mut() {
                *chunk = pair;
            }
            for chunk in tail.as_chunks_mut::<4>().0.iter_mut() {
                *chunk = rgba;
            }
            return;
        }
        let sr: u32 = color.get_r() as u32;
        let sg: u32 = color.get_g() as u32;
        let sb: u32 = color.get_b() as u32;
        let sa_u32: u32 = sa as u32;
        let inv_a: u32 = 255 - sa_u32;
        for px in span.as_chunks_mut::<4>().0.iter_mut() {
            let dr: u32 = px[0] as u32;
            let dg: u32 = px[1] as u32;
            let db: u32 = px[2] as u32;
            let da: u32 = px[3] as u32;
            let vr: u32 = sr * sa_u32 + dr * inv_a + 128;
            let vg: u32 = sg * sa_u32 + dg * inv_a + 128;
            let vb: u32 = sb * sa_u32 + db * inv_a + 128;
            let va: u32 = da * inv_a + 128;
            px[0] = ((vr + (vr >> 8)) >> 8) as u8;
            px[1] = ((vg + (vg >> 8)) >> 8) as u8;
            px[2] = ((vb + (vb >> 8)) >> 8) as u8;
            px[3] = (sa_u32 + ((va + (va >> 8)) >> 8)).min(255) as u8;
        }
    }

    /// Fills or composites a contiguous horizontal pixel span `[x0, x1)` on row `y` using gamma-correct linear-light blending.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable framebuffer.
    /// - `u32` - Inclusive start X pixel coordinate.
    /// - `u32` - Exclusive end X pixel coordinate.
    /// - `u32` - Row Y pixel coordinate.
    /// - `Color` - Foreground color to write or linear-light blend.
    pub fn fill_span_linear(&mut self, x0: u32, x1: u32, y: u32, color: Color) {
        let sa: u8 = color.get_a();
        let width: u32 = self.get_width();
        let height: u32 = self.get_height();
        if sa == 0 || y >= height || x0 >= x1 || x0 >= width {
            return;
        }
        if sa == 255 {
            self.fill_span(x0, x1, y, color);
            return;
        }
        let end_x: u32 = x1.min(width);
        let row_base: usize = (y as usize) * (width as usize);
        let start_byte: usize = (row_base + (x0 as usize)) * 4;
        let end_byte: usize = (row_base + (end_x as usize)) * 4;
        let Some(span) = self.get_mut_pixels().get_mut(start_byte..end_byte) else {
            return;
        };
        let a: f32 = (sa as f32) * (1.0 / 255.0);
        let inv_a: f32 = 1.0 - a;
        let sr_a: f32 = SRGB_TO_LIN_LUT[color.get_r() as usize] * a;
        let sg_a: f32 = SRGB_TO_LIN_LUT[color.get_g() as usize] * a;
        let sb_a: f32 = SRGB_TO_LIN_LUT[color.get_b() as usize] * a;
        let sa_u32: u32 = sa as u32;
        let inv_a_u32: u32 = 255 - sa_u32;
        for px in span.as_chunks_mut::<4>().0.iter_mut() {
            let dr: f32 = SRGB_TO_LIN_LUT[px[0] as usize];
            let dg: f32 = SRGB_TO_LIN_LUT[px[1] as usize];
            let db: f32 = SRGB_TO_LIN_LUT[px[2] as usize];
            let ir: usize = ((sr_a + dr * inv_a) * 4095.0).clamp(0.0, 4095.0) as usize;
            let ig: usize = ((sg_a + dg * inv_a) * 4095.0).clamp(0.0, 4095.0) as usize;
            let ib: usize = ((sb_a + db * inv_a) * 4095.0).clamp(0.0, 4095.0) as usize;
            let va: u32 = (px[3] as u32) * inv_a_u32 + 128;
            px[0] = ((LIN_TO_SRGB_Q4_LUT[ir] + 8) >> 4).min(255) as u8;
            px[1] = ((LIN_TO_SRGB_Q4_LUT[ig] + 8) >> 4).min(255) as u8;
            px[2] = ((LIN_TO_SRGB_Q4_LUT[ib] + 8) >> 4).min(255) as u8;
            px[3] = (sa_u32 + ((va + (va >> 8)) >> 8)).min(255) as u8;
        }
    }

    /// Blends a contiguous glyph alpha coverage row directly onto framebuffer row `dy` starting at `dx`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable framebuffer.
    /// - `u32` - Destination starting X pixel coordinate.
    /// - `u32` - Destination Y pixel coordinate.
    /// - `&[u8]` - Glyph coverage alpha bytes for the clipped horizontal slice.
    /// - `Color` - Foreground text color.
    ///
    /// # Returns
    ///
    /// - `usize` - Number of non-zero coverage pixels shaded.
    pub fn blend_glyph_row(&mut self, dx: u32, dy: u32, coverage: &[u8], color: Color) -> usize {
        let width: u32 = self.get_width();
        let height: u32 = self.get_height();
        let color_a: u32 = color.get_a() as u32;
        if color_a == 0 || dy >= height || dx >= width || coverage.is_empty() {
            return 0;
        }
        let max_len: usize = ((width - dx) as usize).min(coverage.len());
        let start_byte: usize = ((dy as usize) * (width as usize) + (dx as usize)) * 4;
        let end_byte: usize = start_byte + max_len * 4;
        let Some(dst_span) = self.get_mut_pixels().get_mut(start_byte..end_byte) else {
            return 0;
        };
        let sr: u32 = color.get_r() as u32;
        let sg: u32 = color.get_g() as u32;
        let sb: u32 = color.get_b() as u32;
        let mut shaded: usize = 0;
        for (px, alpha_byte) in dst_span
            .as_chunks_mut::<4>()
            .0
            .iter_mut()
            .zip(coverage[..max_len].iter().copied())
        {
            if alpha_byte == 0 {
                continue;
            }
            let sa_u32: u32 = if color_a == 255 {
                alpha_byte as u32
            } else {
                let prod: u32 = color_a * (alpha_byte as u32) + 128;
                (prod + (prod >> 8)) >> 8
            };
            if sa_u32 == 255 {
                px[0] = sr as u8;
                px[1] = sg as u8;
                px[2] = sb as u8;
                px[3] = 255;
            } else if sa_u32 > 0 {
                let inv_a: u32 = 255 - sa_u32;
                let vr: u32 = sr * sa_u32 + (px[0] as u32) * inv_a + 128;
                let vg: u32 = sg * sa_u32 + (px[1] as u32) * inv_a + 128;
                let vb: u32 = sb * sa_u32 + (px[2] as u32) * inv_a + 128;
                let va: u32 = (px[3] as u32) * inv_a + 128;
                px[0] = ((vr + (vr >> 8)) >> 8) as u8;
                px[1] = ((vg + (vg >> 8)) >> 8) as u8;
                px[2] = ((vb + (vb >> 8)) >> 8) as u8;
                px[3] = (sa_u32 + ((va + (va >> 8)) >> 8)).min(255) as u8;
            }
            shaded += 1;
        }
        shaded
    }

    /// Reads the RGBA `Color` at pixel coordinates `(x, y)`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Framebuffer reference.
    /// - `u32` - X pixel coordinate.
    /// - `u32` - Y pixel coordinate.
    ///
    /// # Returns
    ///
    /// - `Option<Color>` - Pixel color if within bounds.
    #[must_use]
    pub fn get_pixel(&self, x: u32, y: u32) -> Option<Color> {
        if x >= self.get_width() || y >= self.get_height() {
            return None;
        }
        let offset: usize = ((y as usize) * (self.get_width() as usize) + (x as usize)) * 4;
        let slice: &[u8] = self.get_pixels().get(offset..offset + 4)?;
        Some(Color::new(slice[0], slice[1], slice[2], slice[3]))
    }

    /// Blends `color` over the pixel at `(x, y)` using gamma-correct linear-light compositing with optional sub-LSB dithering.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable framebuffer.
    /// - `u32` - X pixel coordinate.
    /// - `u32` - Y pixel coordinate.
    /// - `Color` - Foreground color to composite.
    /// - `f32` - Normalized dither offset in `[-0.5, 0.5]` to eliminate 8-bit banding.
    pub fn blend_pixel_linear_dithered(&mut self, x: u32, y: u32, color: Color, dither: f32) {
        let sa: u8 = color.get_a();
        if sa == 0 || x >= self.get_width() || y >= self.get_height() {
            return;
        }
        let offset: usize = ((y as usize) * (self.get_width() as usize) + (x as usize)) * 4;
        let pixels: &mut [u8] = self.get_mut_pixels().as_mut_slice();
        if offset + 4 > pixels.len() {
            return;
        }
        if sa == 255 {
            pixels[offset] = color.get_r();
            pixels[offset + 1] = color.get_g();
            pixels[offset + 2] = color.get_b();
            pixels[offset + 3] = 255;
            return;
        }
        let a: f32 = (sa as f32) * (1.0 / 255.0);
        let inv_a: f32 = 1.0 - a;
        let sr: f32 = SRGB_TO_LIN_LUT[color.get_r() as usize];
        let sg: f32 = SRGB_TO_LIN_LUT[color.get_g() as usize];
        let sb: f32 = SRGB_TO_LIN_LUT[color.get_b() as usize];
        let dr: f32 = SRGB_TO_LIN_LUT[pixels[offset] as usize];
        let dg: f32 = SRGB_TO_LIN_LUT[pixels[offset + 1] as usize];
        let db: f32 = SRGB_TO_LIN_LUT[pixels[offset + 2] as usize];
        let ir: usize = ((sr * a + dr * inv_a) * 4095.0).clamp(0.0, 4095.0) as usize;
        let ig: usize = ((sg * a + dg * inv_a) * 4095.0).clamp(0.0, 4095.0) as usize;
        let ib: usize = ((sb * a + db * inv_a) * 4095.0).clamp(0.0, 4095.0) as usize;
        let dq: i32 = (dither * 16.0) as i32 + 8;
        pixels[offset] = (((LIN_TO_SRGB_Q4_LUT[ir] as i32) + dq) >> 4).clamp(0, 255) as u8;
        pixels[offset + 1] = (((LIN_TO_SRGB_Q4_LUT[ig] as i32) + dq) >> 4).clamp(0, 255) as u8;
        pixels[offset + 2] = (((LIN_TO_SRGB_Q4_LUT[ib] as i32) + dq) >> 4).clamp(0, 255) as u8;
        let sa_u32: u32 = sa as u32;
        let va: u32 = (pixels[offset + 3] as u32) * (255 - sa_u32) + 128;
        pixels[offset + 3] = (sa_u32 + ((va + (va >> 8)) >> 8)).min(255) as u8;
    }

    /// Blends a foreground RGBA color with continuous floating-point alpha `alpha_f` in `[0.0, 1.0]` using linear-light compositing and IGN dithering.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable framebuffer.
    /// - `u32` - X pixel coordinate.
    /// - `u32` - Y pixel coordinate.
    /// - `Color` - Foreground RGB tint (its own alpha is already folded into `alpha_f`).
    /// - `f32` - Continuous opacity in `[0.0, 1.0]`.
    /// - `f32` - Normalized dither offset in `[-0.5, 0.5]`.
    pub fn blend_pixel_f32_dithered(
        &mut self,
        x: u32,
        y: u32,
        color: Color,
        alpha_f: f32,
        dither: f32,
    ) {
        if alpha_f <= 0.0002 || x >= self.get_width() || y >= self.get_height() {
            return;
        }
        let offset: usize = ((y as usize) * (self.get_width() as usize) + (x as usize)) * 4;
        let pixels: &mut [u8] = self.get_mut_pixels().as_mut_slice();
        if offset + 4 > pixels.len() {
            return;
        }
        let a: f32 = alpha_f.clamp(0.0, 1.0);
        if a >= 0.9995 {
            pixels[offset] = color.get_r();
            pixels[offset + 1] = color.get_g();
            pixels[offset + 2] = color.get_b();
            pixels[offset + 3] = 255;
            return;
        }
        let inv_a: f32 = 1.0 - a;
        let sr: f32 = SRGB_TO_LIN_LUT[color.get_r() as usize];
        let sg: f32 = SRGB_TO_LIN_LUT[color.get_g() as usize];
        let sb: f32 = SRGB_TO_LIN_LUT[color.get_b() as usize];
        let dr: f32 = SRGB_TO_LIN_LUT[pixels[offset] as usize];
        let dg: f32 = SRGB_TO_LIN_LUT[pixels[offset + 1] as usize];
        let db: f32 = SRGB_TO_LIN_LUT[pixels[offset + 2] as usize];
        let ir: usize = ((sr * a + dr * inv_a) * 4095.0).clamp(0.0, 4095.0) as usize;
        let ig: usize = ((sg * a + dg * inv_a) * 4095.0).clamp(0.0, 4095.0) as usize;
        let ib: usize = ((sb * a + db * inv_a) * 4095.0).clamp(0.0, 4095.0) as usize;
        let dq: i32 = (dither * 16.0) as i32 + 8;
        pixels[offset] = (((LIN_TO_SRGB_Q4_LUT[ir] as i32) + dq) >> 4).clamp(0, 255) as u8;
        pixels[offset + 1] = (((LIN_TO_SRGB_Q4_LUT[ig] as i32) + dq) >> 4).clamp(0, 255) as u8;
        pixels[offset + 2] = (((LIN_TO_SRGB_Q4_LUT[ib] as i32) + dq) >> 4).clamp(0, 255) as u8;
        let sa_u32: u32 = (a * 255.0) as u32;
        let va: u32 = (pixels[offset + 3] as u32) * (255 - sa_u32) + 128;
        pixels[offset + 3] = (sa_u32 + ((va + (va >> 8)) >> 8)).min(255) as u8;
    }

    /// Blends `color` over the pixel at `(x, y)` using fast integer Porter-Duff source-over.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable framebuffer.
    /// - `u32` - X pixel coordinate.
    /// - `u32` - Y pixel coordinate.
    /// - `Color` - Foreground color to composite.
    pub fn blend_pixel(&mut self, x: u32, y: u32, color: Color) {
        let sa: u8 = color.get_a();
        if sa == 0 || x >= self.get_width() || y >= self.get_height() {
            return;
        }
        let offset: usize = ((y as usize) * (self.get_width() as usize) + (x as usize)) * 4;
        let pixels: &mut [u8] = self.get_mut_pixels().as_mut_slice();
        if offset + 4 > pixels.len() {
            return;
        }
        if sa == 255 {
            pixels[offset] = color.get_r();
            pixels[offset + 1] = color.get_g();
            pixels[offset + 2] = color.get_b();
            pixels[offset + 3] = 255;
            return;
        }
        let dst: Color = Color::new(
            pixels[offset],
            pixels[offset + 1],
            pixels[offset + 2],
            pixels[offset + 3],
        );
        let blended: Color = color.blend_over(dst);
        pixels[offset] = blended.get_r();
        pixels[offset + 1] = blended.get_g();
        pixels[offset + 2] = blended.get_b();
        pixels[offset + 3] = blended.get_a();
    }

    /// Converts RGBA8 pixels into BGRA8 byte order required by Win32 GDI, X11, and Cocoa/Metal.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Source RGBA8 framebuffer.
    ///
    /// # Returns
    ///
    /// - `Vec<u8>` - Packed BGRA8 pixel buffer.
    #[must_use]
    pub fn to_bgra_bytes(&self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::with_capacity(self.get_pixels().len());
        for chunk in self.get_pixels().as_chunks::<4>().0.iter() {
            out.push(chunk[2]);
            out.push(chunk[1]);
            out.push(chunk[0]);
            out.push(chunk[3]);
        }
        out
    }

    /// Computes a deterministic 64-bit 4-lane word-folded FNV-1a hash of the framebuffer pixels.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Framebuffer to hash.
    ///
    /// # Returns
    ///
    /// - `u64` - Deterministic endian-independent pixel hash.
    #[must_use]
    pub fn fnv1a_hash(&self) -> u64 {
        let bytes: &[u8] = self.get_pixels().as_slice();
        let mut h0: u64 = FNV1A_OFFSET_BASIS;
        let mut h1: u64 = FNV1A_OFFSET_BASIS ^ 0x9E37_79B9_7F4A_7C15;
        let mut h2: u64 = FNV1A_OFFSET_BASIS ^ 0xBF58_476D_1CE4_E5B9;
        let mut h3: u64 = FNV1A_OFFSET_BASIS ^ 0x94D0_49BB_1331_11EB;
        let (blocks, tail): (&[[u8; 32]], &[u8]) = bytes.as_chunks::<32>();
        for blk in blocks.iter() {
            let w0: u64 = u64::from_le_bytes([
                blk[0], blk[1], blk[2], blk[3], blk[4], blk[5], blk[6], blk[7],
            ]);
            let w1: u64 = u64::from_le_bytes([
                blk[8], blk[9], blk[10], blk[11], blk[12], blk[13], blk[14], blk[15],
            ]);
            let w2: u64 = u64::from_le_bytes([
                blk[16], blk[17], blk[18], blk[19], blk[20], blk[21], blk[22], blk[23],
            ]);
            let w3: u64 = u64::from_le_bytes([
                blk[24], blk[25], blk[26], blk[27], blk[28], blk[29], blk[30], blk[31],
            ]);
            h0 = (h0 ^ w0).wrapping_mul(FNV1A_PRIME);
            h1 = (h1 ^ w1).wrapping_mul(FNV1A_PRIME);
            h2 = (h2 ^ w2).wrapping_mul(FNV1A_PRIME);
            h3 = (h3 ^ w3).wrapping_mul(FNV1A_PRIME);
        }
        let mut hash: u64 = h0 ^ h1.rotate_left(13) ^ h2.rotate_left(29) ^ h3.rotate_left(47);
        hash = hash.wrapping_mul(FNV1A_PRIME);
        for byte in tail.iter().copied() {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(FNV1A_PRIME);
        }
        hash
    }

    /// Borrows the entire framebuffer as a single zero-copy mutable `PixelBand`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable framebuffer.
    ///
    /// # Returns
    ///
    /// - `PixelBand<'_>` - Full-surface mutable pixel band.
    pub fn as_band_mut(&mut self) -> PixelBand<'_> {
        let Self {
            width,
            height,
            pixels,
        } = self;
        PixelBand {
            width: *width,
            y_offset: 0,
            height: *height,
            pixels: pixels.as_mut_slice(),
        }
    }

    /// Splits rows `[y0, y1)` of this framebuffer into `band_count` disjoint mutable `PixelBand` slices for safe parallel rasterization.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable framebuffer.
    /// - `u32` - Inclusive starting physical Y row.
    /// - `u32` - Exclusive ending physical Y row.
    /// - `usize` - Requested number of parallel bands.
    ///
    /// # Returns
    ///
    /// - `Vec<PixelBand<'_>>` - Non-overlapping mutable horizontal bands covering `[y0, y1)`.
    pub fn split_bands_mut(&mut self, y0: u32, y1: u32, band_count: usize) -> Vec<PixelBand<'_>> {
        let Self {
            width,
            height,
            pixels,
        } = self;
        let fb_w: u32 = *width;
        let fb_h: u32 = *height;
        let start_y: u32 = y0.min(fb_h);
        let end_y: u32 = y1.clamp(start_y, fb_h);
        let total_rows: u32 = end_y - start_y;
        if total_rows == 0 || fb_w == 0 {
            return Vec::new();
        }
        let count: usize = band_count.max(1).min(total_rows as usize);
        let rows_per: usize = (total_rows as usize).div_ceil(count);
        let row_stride: usize = (fb_w as usize) * 4;
        let start_byte: usize = (start_y as usize) * row_stride;
        let end_byte: usize = (end_y as usize) * row_stride;
        let Some(slice) = pixels.get_mut(start_byte..end_byte) else {
            return Vec::new();
        };
        let mut bands: Vec<PixelBand<'_>> = Vec::with_capacity(count);
        let mut cur_y: u32 = start_y;
        for chunk in slice.chunks_mut(rows_per * row_stride) {
            let chunk_rows: u32 = (chunk.len() / row_stride) as u32;
            bands.push(PixelBand {
                width: fb_w,
                y_offset: cur_y,
                height: chunk_rows,
                pixels: chunk,
            });
            cur_y += chunk_rows;
        }
        bands
    }
}

/// Zero-copy band rasterization primitives for `PixelBand`.
impl<'a> PixelBand<'a> {
    /// Returns the full framebuffer width in physical pixels.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Pixel band reference.
    ///
    /// # Returns
    ///
    /// - `u32` - Width in physical pixels.
    #[must_use]
    pub fn width(&self) -> u32 {
        let Self { width, .. } = self;
        *width
    }

    /// Returns the inclusive top physical Y row owned by this band.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Pixel band reference.
    ///
    /// # Returns
    ///
    /// - `u32` - Starting physical Y row.
    #[must_use]
    pub fn y_offset(&self) -> u32 {
        let Self { y_offset, .. } = self;
        *y_offset
    }

    /// Returns the exclusive bottom physical Y row owned by this band.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Pixel band reference.
    ///
    /// # Returns
    ///
    /// - `u32` - Ending physical Y row.
    #[must_use]
    pub fn y_end(&self) -> u32 {
        let Self {
            y_offset, height, ..
        } = self;
        *y_offset + *height
    }

    /// Returns the mutable byte slice for global physical row `y` if owned by this band.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable pixel band.
    /// - `u32` - Global physical Y row coordinate.
    ///
    /// # Returns
    ///
    /// - `Option<&mut [u8]>` - Mutable row byte slice (`width * 4` bytes).
    pub fn row_slice_mut(&mut self, y: u32) -> Option<&mut [u8]> {
        let Self {
            width,
            y_offset,
            height,
            pixels,
        } = self;
        if y < *y_offset || y >= *y_offset + *height {
            return None;
        }
        let local_y: usize = (y - *y_offset) as usize;
        let stride: usize = (*width as usize) * 4;
        let start: usize = local_y * stride;
        pixels.get_mut(start..start + stride)
    }

    /// Fills the entire band with solid `color` using 64-bit wide stores.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable pixel band.
    /// - `Color` - Clear color.
    pub fn clear(&mut self, color: Color) {
        let Self { pixels, .. } = self;
        let rgba: [u8; 4] = color.to_rgba_bytes();
        let pair: [u8; 8] = [
            rgba[0], rgba[1], rgba[2], rgba[3], rgba[0], rgba[1], rgba[2], rgba[3],
        ];
        let (wide, tail): (&mut [[u8; 8]], &mut [u8]) = pixels.as_chunks_mut::<8>();
        for chunk in wide.iter_mut() {
            *chunk = pair;
        }
        for chunk in tail.as_chunks_mut::<4>().0.iter_mut() {
            *chunk = rgba;
        }
    }

    /// Fills or Porter-Duff blends horizontal span `[x0, x1)` on global row `y`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable pixel band.
    /// - `u32` - Inclusive start X coordinate.
    /// - `u32` - Exclusive end X coordinate.
    /// - `u32` - Global physical Y coordinate.
    /// - `Color` - Foreground RGBA color.
    pub fn fill_span(&mut self, x0: u32, x1: u32, y: u32, color: Color) {
        let sa: u8 = color.get_a();
        let Self {
            width,
            y_offset,
            height,
            pixels,
        } = self;
        if sa == 0 || y < *y_offset || y >= *y_offset + *height || x0 >= x1 || x0 >= *width {
            return;
        }
        let end_x: u32 = x1.min(*width);
        let row_base: usize = ((y - *y_offset) as usize) * (*width as usize);
        let start_byte: usize = (row_base + (x0 as usize)) * 4;
        let end_byte: usize = (row_base + (end_x as usize)) * 4;
        let Some(span) = pixels.get_mut(start_byte..end_byte) else {
            return;
        };
        if sa == 255 {
            let rgba: [u8; 4] = [color.get_r(), color.get_g(), color.get_b(), 255];
            let pair: [u8; 8] = [
                rgba[0], rgba[1], rgba[2], rgba[3], rgba[0], rgba[1], rgba[2], rgba[3],
            ];
            let (wide, tail): (&mut [[u8; 8]], &mut [u8]) = span.as_chunks_mut::<8>();
            for chunk in wide.iter_mut() {
                *chunk = pair;
            }
            for chunk in tail.as_chunks_mut::<4>().0.iter_mut() {
                *chunk = rgba;
            }
            return;
        }
        let sr: u32 = color.get_r() as u32;
        let sg: u32 = color.get_g() as u32;
        let sb: u32 = color.get_b() as u32;
        let sa_u32: u32 = sa as u32;
        let inv_a: u32 = 255 - sa_u32;
        for px in span.as_chunks_mut::<4>().0.iter_mut() {
            let dr: u32 = px[0] as u32;
            let dg: u32 = px[1] as u32;
            let db: u32 = px[2] as u32;
            let da: u32 = px[3] as u32;
            let vr: u32 = sr * sa_u32 + dr * inv_a + 128;
            let vg: u32 = sg * sa_u32 + dg * inv_a + 128;
            let vb: u32 = sb * sa_u32 + db * inv_a + 128;
            let va: u32 = da * inv_a + 128;
            px[0] = ((vr + (vr >> 8)) >> 8) as u8;
            px[1] = ((vg + (vg >> 8)) >> 8) as u8;
            px[2] = ((vb + (vb >> 8)) >> 8) as u8;
            px[3] = (sa_u32 + ((va + (va >> 8)) >> 8)).min(255) as u8;
        }
    }

    /// Fills or LUT linear-light blends horizontal span `[x0, x1)` on global row `y`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable pixel band.
    /// - `u32` - Inclusive start X coordinate.
    /// - `u32` - Exclusive end X coordinate.
    /// - `u32` - Global physical Y coordinate.
    /// - `Color` - Foreground RGBA color.
    pub fn fill_span_linear(&mut self, x0: u32, x1: u32, y: u32, color: Color) {
        let sa: u8 = color.get_a();
        if sa == 255 {
            self.fill_span(x0, x1, y, color);
            return;
        }
        let Self {
            width,
            y_offset,
            height,
            pixels,
        } = self;
        if sa == 0 || y < *y_offset || y >= *y_offset + *height || x0 >= x1 || x0 >= *width {
            return;
        }
        let end_x: u32 = x1.min(*width);
        let row_base: usize = ((y - *y_offset) as usize) * (*width as usize);
        let start_byte: usize = (row_base + (x0 as usize)) * 4;
        let end_byte: usize = (row_base + (end_x as usize)) * 4;
        let Some(span) = pixels.get_mut(start_byte..end_byte) else {
            return;
        };
        if end_x - x0 >= 128 {
            let (lut_r, lut_g, lut_b, lut_a): ([u8; 256], [u8; 256], [u8; 256], [u8; 256]) =
                build_linear_blend_luts(color);
            for px in span.as_chunks_mut::<4>().0.iter_mut() {
                px[0] = lut_r[px[0] as usize];
                px[1] = lut_g[px[1] as usize];
                px[2] = lut_b[px[2] as usize];
                px[3] = lut_a[px[3] as usize];
            }
            return;
        }
        let a: f32 = (sa as f32) * (1.0 / 255.0);
        let inv_a: f32 = 1.0 - a;
        let sr_a: f32 = SRGB_TO_LIN_LUT[color.get_r() as usize] * a;
        let sg_a: f32 = SRGB_TO_LIN_LUT[color.get_g() as usize] * a;
        let sb_a: f32 = SRGB_TO_LIN_LUT[color.get_b() as usize] * a;
        let sa_u32: u32 = sa as u32;
        let inv_a_u32: u32 = 255 - sa_u32;
        for px in span.as_chunks_mut::<4>().0.iter_mut() {
            let dr: f32 = SRGB_TO_LIN_LUT[px[0] as usize];
            let dg: f32 = SRGB_TO_LIN_LUT[px[1] as usize];
            let db: f32 = SRGB_TO_LIN_LUT[px[2] as usize];
            let ir: usize = ((sr_a + dr * inv_a) * 4095.0).clamp(0.0, 4095.0) as usize;
            let ig: usize = ((sg_a + dg * inv_a) * 4095.0).clamp(0.0, 4095.0) as usize;
            let ib: usize = ((sb_a + db * inv_a) * 4095.0).clamp(0.0, 4095.0) as usize;
            let va: u32 = (px[3] as u32) * inv_a_u32 + 128;
            px[0] = ((LIN_TO_SRGB_Q4_LUT[ir] + 8) >> 4).min(255) as u8;
            px[1] = ((LIN_TO_SRGB_Q4_LUT[ig] + 8) >> 4).min(255) as u8;
            px[2] = ((LIN_TO_SRGB_Q4_LUT[ib] + 8) >> 4).min(255) as u8;
            px[3] = (sa_u32 + ((va + (va >> 8)) >> 8)).min(255) as u8;
        }
    }

    /// Composites horizontal span `[x0, x1)` on global row `y` using precomputed bit-exact 256-entry linear-light channel LUTs.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable pixel band.
    /// - `u32` - Inclusive start X coordinate.
    /// - `u32` - Exclusive end X coordinate.
    /// - `u32` - Global physical Y coordinate.
    /// - `&([u8; 256], [u8; 256], [u8; 256], [u8; 256])` - Precomputed per-channel lookup tables.
    pub fn fill_span_linear_luts(
        &mut self,
        x0: u32,
        x1: u32,
        y: u32,
        luts: &([u8; 256], [u8; 256], [u8; 256], [u8; 256]),
    ) {
        let Self {
            width,
            y_offset,
            height,
            pixels,
        } = self;
        if y < *y_offset || y >= *y_offset + *height || x0 >= x1 || x0 >= *width {
            return;
        }
        let end_x: u32 = x1.min(*width);
        let row_base: usize = ((y - *y_offset) as usize) * (*width as usize);
        let start_byte: usize = (row_base + (x0 as usize)) * 4;
        let end_byte: usize = (row_base + (end_x as usize)) * 4;
        let Some(span) = pixels.get_mut(start_byte..end_byte) else {
            return;
        };
        let (lut_r, lut_g, lut_b, lut_a): &([u8; 256], [u8; 256], [u8; 256], [u8; 256]) = luts;
        for px in span.as_chunks_mut::<4>().0.iter_mut() {
            px[0] = lut_r[px[0] as usize];
            px[1] = lut_g[px[1] as usize];
            px[2] = lut_b[px[2] as usize];
            px[3] = lut_a[px[3] as usize];
        }
    }

    /// Composites a constant-opacity horizontal shadow span `[x0, x1)` on global row `y` with L1 LUT linear light and IGN dithering.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable pixel band.
    /// - `u32` - Inclusive start X coordinate.
    /// - `u32` - Exclusive end X coordinate.
    /// - `u32` - Global physical Y coordinate.
    /// - `Color` - Shadow tint RGB.
    /// - `f32` - Continuous shadow opacity in `[0.0, 1.0]`.
    pub fn blend_shadow_span_const_alpha(
        &mut self,
        x0: u32,
        x1: u32,
        y: u32,
        color: Color,
        alpha_f: f32,
    ) {
        if alpha_f <= 0.0005 {
            return;
        }
        let Self {
            width,
            y_offset,
            height,
            pixels,
        } = self;
        if y < *y_offset || y >= *y_offset + *height || x0 >= x1 || x0 >= *width {
            return;
        }
        let end_x: u32 = x1.min(*width);
        let row_base: usize = ((y - *y_offset) as usize) * (*width as usize);
        let start_byte: usize = (row_base + (x0 as usize)) * 4;
        let end_byte: usize = (row_base + (end_x as usize)) * 4;
        let Some(span) = pixels.get_mut(start_byte..end_byte) else {
            return;
        };
        let a: f32 = alpha_f.clamp(0.0, 1.0);
        let inv_a: f32 = 1.0 - a;
        let sr_a: f32 = SRGB_TO_LIN_LUT[color.get_r() as usize] * a;
        let sg_a: f32 = SRGB_TO_LIN_LUT[color.get_g() as usize] * a;
        let sb_a: f32 = SRGB_TO_LIN_LUT[color.get_b() as usize] * a;
        let sa_u32: u32 = (a * 255.0) as u32;
        let inv_a_u32: u32 = 255 - sa_u32;
        let y_term: f32 = (y as f32) * 0.005_837_15;
        for (cur_x, px) in (x0..).zip(span.as_chunks_mut::<4>().0.iter_mut()) {
            let n: f32 = (52.982_918 * ((cur_x as f32) * 0.067_110_56 + y_term).fract()).fract();
            let dq: i32 = ((n - 0.5) * 16.0) as i32 + 8;
            let dr: f32 = SRGB_TO_LIN_LUT[px[0] as usize];
            let dg: f32 = SRGB_TO_LIN_LUT[px[1] as usize];
            let db: f32 = SRGB_TO_LIN_LUT[px[2] as usize];
            let ir: usize = ((sr_a + dr * inv_a) * 4095.0).clamp(0.0, 4095.0) as usize;
            let ig: usize = ((sg_a + dg * inv_a) * 4095.0).clamp(0.0, 4095.0) as usize;
            let ib: usize = ((sb_a + db * inv_a) * 4095.0).clamp(0.0, 4095.0) as usize;
            let va: u32 = (px[3] as u32) * inv_a_u32 + 128;
            px[0] = (((LIN_TO_SRGB_Q4_LUT[ir] as i32) + dq) >> 4).clamp(0, 255) as u8;
            px[1] = (((LIN_TO_SRGB_Q4_LUT[ig] as i32) + dq) >> 4).clamp(0, 255) as u8;
            px[2] = (((LIN_TO_SRGB_Q4_LUT[ib] as i32) + dq) >> 4).clamp(0, 255) as u8;
            px[3] = (sa_u32 + ((va + (va >> 8)) >> 8)).min(255) as u8;
        }
    }

    /// Blends a contiguous glyph alpha coverage row onto global row `dy` starting at `dx`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable pixel band.
    /// - `u32` - Destination starting X coordinate.
    /// - `u32` - Global destination Y coordinate.
    /// - `&[u8]` - Glyph alpha coverage slice.
    /// - `Color` - Foreground text color.
    ///
    /// # Returns
    ///
    /// - `usize` - Number of non-zero coverage pixels shaded.
    pub fn blend_glyph_row(&mut self, dx: u32, dy: u32, coverage: &[u8], color: Color) -> usize {
        let color_a: u32 = color.get_a() as u32;
        let Self {
            width,
            y_offset,
            height,
            pixels,
        } = self;
        if color_a == 0
            || dy < *y_offset
            || dy >= *y_offset + *height
            || dx >= *width
            || coverage.is_empty()
        {
            return 0;
        }
        let max_len: usize = ((*width - dx) as usize).min(coverage.len());
        let start_byte: usize =
            (((dy - *y_offset) as usize) * (*width as usize) + (dx as usize)) * 4;
        let end_byte: usize = start_byte + max_len * 4;
        let Some(dst_span) = pixels.get_mut(start_byte..end_byte) else {
            return 0;
        };
        let sr: u32 = color.get_r() as u32;
        let sg: u32 = color.get_g() as u32;
        let sb: u32 = color.get_b() as u32;
        let mut shaded: usize = 0;
        for (px, alpha_byte) in dst_span
            .as_chunks_mut::<4>()
            .0
            .iter_mut()
            .zip(coverage[..max_len].iter().copied())
        {
            if alpha_byte == 0 {
                continue;
            }
            let sa_u32: u32 = if color_a == 255 {
                alpha_byte as u32
            } else {
                let prod: u32 = color_a * (alpha_byte as u32) + 128;
                (prod + (prod >> 8)) >> 8
            };
            if sa_u32 == 255 {
                px[0] = sr as u8;
                px[1] = sg as u8;
                px[2] = sb as u8;
                px[3] = 255;
            } else if sa_u32 > 0 {
                let inv_a: u32 = 255 - sa_u32;
                let vr: u32 = sr * sa_u32 + (px[0] as u32) * inv_a + 128;
                let vg: u32 = sg * sa_u32 + (px[1] as u32) * inv_a + 128;
                let vb: u32 = sb * sa_u32 + (px[2] as u32) * inv_a + 128;
                let va: u32 = (px[3] as u32) * inv_a + 128;
                px[0] = ((vr + (vr >> 8)) >> 8) as u8;
                px[1] = ((vg + (vg >> 8)) >> 8) as u8;
                px[2] = ((vb + (vb >> 8)) >> 8) as u8;
                px[3] = (sa_u32 + ((va + (va >> 8)) >> 8)).min(255) as u8;
            }
            shaded += 1;
        }
        shaded
    }

    /// Blends `color` over global pixel `(x, y)` using fast integer Porter-Duff source-over.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable pixel band.
    /// - `u32` - X pixel coordinate.
    /// - `u32` - Global Y pixel coordinate.
    /// - `Color` - Foreground color.
    pub fn blend_pixel(&mut self, x: u32, y: u32, color: Color) {
        let sa: u8 = color.get_a();
        let Self {
            width,
            y_offset,
            height,
            pixels,
        } = self;
        if sa == 0 || x >= *width || y < *y_offset || y >= *y_offset + *height {
            return;
        }
        let offset: usize = (((y - *y_offset) as usize) * (*width as usize) + (x as usize)) * 4;
        if offset + 4 > pixels.len() {
            return;
        }
        if sa == 255 {
            pixels[offset] = color.get_r();
            pixels[offset + 1] = color.get_g();
            pixels[offset + 2] = color.get_b();
            pixels[offset + 3] = 255;
            return;
        }
        let dst: Color = Color::new(
            pixels[offset],
            pixels[offset + 1],
            pixels[offset + 2],
            pixels[offset + 3],
        );
        let blended: Color = color.blend_over(dst);
        pixels[offset] = blended.get_r();
        pixels[offset + 1] = blended.get_g();
        pixels[offset + 2] = blended.get_b();
        pixels[offset + 3] = blended.get_a();
    }

    /// Blends `color` over global pixel `(x, y)` using L1 LUT linear-light compositing with optional sub-LSB dithering.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable pixel band.
    /// - `u32` - X pixel coordinate.
    /// - `u32` - Global Y pixel coordinate.
    /// - `Color` - Foreground color.
    /// - `f32` - Normalized dither offset in `[-0.5, 0.5]`.
    pub fn blend_pixel_linear_dithered(&mut self, x: u32, y: u32, color: Color, dither: f32) {
        let sa: u8 = color.get_a();
        let Self {
            width,
            y_offset,
            height,
            pixels,
        } = self;
        if sa == 0 || x >= *width || y < *y_offset || y >= *y_offset + *height {
            return;
        }
        let offset: usize = (((y - *y_offset) as usize) * (*width as usize) + (x as usize)) * 4;
        if offset + 4 > pixels.len() {
            return;
        }
        if sa == 255 {
            pixels[offset] = color.get_r();
            pixels[offset + 1] = color.get_g();
            pixels[offset + 2] = color.get_b();
            pixels[offset + 3] = 255;
            return;
        }
        let a: f32 = (sa as f32) * (1.0 / 255.0);
        let inv_a: f32 = 1.0 - a;
        let sr: f32 = SRGB_TO_LIN_LUT[color.get_r() as usize];
        let sg: f32 = SRGB_TO_LIN_LUT[color.get_g() as usize];
        let sb: f32 = SRGB_TO_LIN_LUT[color.get_b() as usize];
        let dr: f32 = SRGB_TO_LIN_LUT[pixels[offset] as usize];
        let dg: f32 = SRGB_TO_LIN_LUT[pixels[offset + 1] as usize];
        let db: f32 = SRGB_TO_LIN_LUT[pixels[offset + 2] as usize];
        let ir: usize = ((sr * a + dr * inv_a) * 4095.0).clamp(0.0, 4095.0) as usize;
        let ig: usize = ((sg * a + dg * inv_a) * 4095.0).clamp(0.0, 4095.0) as usize;
        let ib: usize = ((sb * a + db * inv_a) * 4095.0).clamp(0.0, 4095.0) as usize;
        let dq: i32 = (dither * 16.0) as i32 + 8;
        pixels[offset] = (((LIN_TO_SRGB_Q4_LUT[ir] as i32) + dq) >> 4).clamp(0, 255) as u8;
        pixels[offset + 1] = (((LIN_TO_SRGB_Q4_LUT[ig] as i32) + dq) >> 4).clamp(0, 255) as u8;
        pixels[offset + 2] = (((LIN_TO_SRGB_Q4_LUT[ib] as i32) + dq) >> 4).clamp(0, 255) as u8;
        let sa_u32: u32 = sa as u32;
        let va: u32 = (pixels[offset + 3] as u32) * (255 - sa_u32) + 128;
        pixels[offset + 3] = (sa_u32 + ((va + (va >> 8)) >> 8)).min(255) as u8;
    }

    /// Blends a foreground RGB color with continuous floating-point opacity `alpha_f` onto global pixel `(x, y)` via L1 LUT linear light.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable pixel band.
    /// - `u32` - X pixel coordinate.
    /// - `u32` - Global Y pixel coordinate.
    /// - `Color` - Foreground RGB tint.
    /// - `f32` - Continuous opacity in `[0.0, 1.0]`.
    /// - `f32` - Normalized dither offset in `[-0.5, 0.5]`.
    pub fn blend_pixel_f32_dithered(
        &mut self,
        x: u32,
        y: u32,
        color: Color,
        alpha_f: f32,
        dither: f32,
    ) {
        let Self {
            width,
            y_offset,
            height,
            pixels,
        } = self;
        if alpha_f <= 0.0002 || x >= *width || y < *y_offset || y >= *y_offset + *height {
            return;
        }
        let offset: usize = (((y - *y_offset) as usize) * (*width as usize) + (x as usize)) * 4;
        if offset + 4 > pixels.len() {
            return;
        }
        let a: f32 = alpha_f.clamp(0.0, 1.0);
        if a >= 0.9995 {
            pixels[offset] = color.get_r();
            pixels[offset + 1] = color.get_g();
            pixels[offset + 2] = color.get_b();
            pixels[offset + 3] = 255;
            return;
        }
        let inv_a: f32 = 1.0 - a;
        let sr: f32 = SRGB_TO_LIN_LUT[color.get_r() as usize];
        let sg: f32 = SRGB_TO_LIN_LUT[color.get_g() as usize];
        let sb: f32 = SRGB_TO_LIN_LUT[color.get_b() as usize];
        let dr: f32 = SRGB_TO_LIN_LUT[pixels[offset] as usize];
        let dg: f32 = SRGB_TO_LIN_LUT[pixels[offset + 1] as usize];
        let db: f32 = SRGB_TO_LIN_LUT[pixels[offset + 2] as usize];
        let ir: usize = ((sr * a + dr * inv_a) * 4095.0).clamp(0.0, 4095.0) as usize;
        let ig: usize = ((sg * a + dg * inv_a) * 4095.0).clamp(0.0, 4095.0) as usize;
        let ib: usize = ((sb * a + db * inv_a) * 4095.0).clamp(0.0, 4095.0) as usize;
        let dq: i32 = (dither * 16.0) as i32 + 8;
        pixels[offset] = (((LIN_TO_SRGB_Q4_LUT[ir] as i32) + dq) >> 4).clamp(0, 255) as u8;
        pixels[offset + 1] = (((LIN_TO_SRGB_Q4_LUT[ig] as i32) + dq) >> 4).clamp(0, 255) as u8;
        pixels[offset + 2] = (((LIN_TO_SRGB_Q4_LUT[ib] as i32) + dq) >> 4).clamp(0, 255) as u8;
        let sa_u32: u32 = (a * 255.0) as u32;
        let va: u32 = (pixels[offset + 3] as u32) * (255 - sa_u32) + 128;
        pixels[offset + 3] = (sa_u32 + ((va + (va >> 8)) >> 8)).min(255) as u8;
    }

    /// Reads the raw RGBA8 bytes at global pixel `(x, y)` if owned by this band.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Pixel band reference.
    /// - `u32` - X pixel coordinate.
    /// - `u32` - Global Y pixel coordinate.
    ///
    /// # Returns
    ///
    /// - `[u8; 4]` - Pixel RGBA8 bytes (`[0, 0, 0, 0]` when out of bounds).
    #[must_use]
    pub fn get_pixel_rgba(&self, x: u32, y: u32) -> [u8; 4] {
        let Self {
            width,
            y_offset,
            height,
            pixels,
        } = self;
        if x >= *width || y < *y_offset || y >= *y_offset + *height {
            return [0, 0, 0, 0];
        }
        let offset: usize = (((y - *y_offset) as usize) * (*width as usize) + (x as usize)) * 4;
        if offset + 4 > pixels.len() {
            return [0, 0, 0, 0];
        }
        [
            pixels[offset],
            pixels[offset + 1],
            pixels[offset + 2],
            pixels[offset + 3],
        ]
    }

    /// Writes raw RGBA8 bytes directly to global pixel `(x, y)` when owned by this band.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable pixel band.
    /// - `u32` - X pixel coordinate.
    /// - `u32` - Global Y pixel coordinate.
    /// - `[u8; 4]` - Target RGBA8 pixel bytes.
    pub fn set_pixel_rgba(&mut self, x: u32, y: u32, rgba: [u8; 4]) {
        let Self {
            width,
            y_offset,
            height,
            pixels,
        } = self;
        if x >= *width || y < *y_offset || y >= *y_offset + *height {
            return;
        }
        let offset: usize = (((y - *y_offset) as usize) * (*width as usize) + (x as usize)) * 4;
        if offset + 4 > pixels.len() {
            return;
        }
        pixels[offset] = rgba[0];
        pixels[offset + 1] = rgba[1];
        pixels[offset + 2] = rgba[2];
        pixels[offset + 3] = rgba[3];
    }

    /// Blends the current rasterized pixel at `(x, y)` back toward `saved_rgba` using linear-light clip coverage `cov`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable pixel band.
    /// - `u32` - X pixel coordinate.
    /// - `u32` - Global Y pixel coordinate.
    /// - `[u8; 4]` - Pre-command saved RGBA8 pixel bytes.
    /// - `f32` - Clip mask coverage in `[0.0, 1.0]` (`1.0` keeps current pixel, `0.0` restores `saved_rgba`).
    pub fn lerp_pixel_from_saved_linear(&mut self, x: u32, y: u32, saved_rgba: [u8; 4], cov: f32) {
        let Self {
            width,
            y_offset,
            height,
            pixels,
        } = self;
        if x >= *width || y < *y_offset || y >= *y_offset + *height {
            return;
        }
        let offset: usize = (((y - *y_offset) as usize) * (*width as usize) + (x as usize)) * 4;
        if offset + 4 > pixels.len() {
            return;
        }
        let cur: [u8; 4] = [
            pixels[offset],
            pixels[offset + 1],
            pixels[offset + 2],
            pixels[offset + 3],
        ];
        if cur == saved_rgba {
            return;
        }
        let c: f32 = cov.clamp(0.0, 1.0);
        let inv_c: f32 = 1.0 - c;
        let nr: f32 = SRGB_TO_LIN_LUT[cur[0] as usize];
        let ng: f32 = SRGB_TO_LIN_LUT[cur[1] as usize];
        let nb: f32 = SRGB_TO_LIN_LUT[cur[2] as usize];
        let or: f32 = SRGB_TO_LIN_LUT[saved_rgba[0] as usize];
        let og: f32 = SRGB_TO_LIN_LUT[saved_rgba[1] as usize];
        let ob: f32 = SRGB_TO_LIN_LUT[saved_rgba[2] as usize];
        let ir: usize = ((nr * c + or * inv_c) * 4095.0).clamp(0.0, 4095.0) as usize;
        let ig: usize = ((ng * c + og * inv_c) * 4095.0).clamp(0.0, 4095.0) as usize;
        let ib: usize = ((nb * c + ob * inv_c) * 4095.0).clamp(0.0, 4095.0) as usize;
        pixels[offset] = ((LIN_TO_SRGB_Q4_LUT[ir] + 8) >> 4).min(255) as u8;
        pixels[offset + 1] = ((LIN_TO_SRGB_Q4_LUT[ig] + 8) >> 4).min(255) as u8;
        pixels[offset + 2] = ((LIN_TO_SRGB_Q4_LUT[ib] + 8) >> 4).min(255) as u8;
        let na: f32 = (cur[3] as f32) * c + (saved_rgba[3] as f32) * inv_c;
        pixels[offset + 3] = na.round().clamp(0.0, 255.0) as u8;
    }
}

/// Damage accumulation helpers for `DamageRegion`.
impl DamageRegion {
    /// Marks the entire surface damaged.
    ///
    /// # Returns
    ///
    /// - `Self` - Full-surface damage region.
    #[must_use]
    pub fn full() -> Self {
        Self {
            full_damage: true,
            rect: None,
        }
    }

    /// Expands the damaged region to include `rect`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable damage region.
    /// - `Rect` - Newly damaged rectangle.
    pub fn add_rect(&mut self, rect: Rect) {
        if self.get_full_damage() {
            return;
        }
        let next: Rect = match self.bounds_rect() {
            Some(existing) => existing.union(rect),
            None => rect,
        };
        self.set_rect(Some(next));
    }

    /// Returns the damaged bounding rectangle, or full viewport when `full_damage` is true.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Damage region.
    /// - `u32` - Surface width.
    /// - `u32` - Surface height.
    ///
    /// # Returns
    ///
    /// - `Option<Rect>` - Effective clip rect to rasterize, or `None` if undamaged.
    #[must_use]
    pub fn resolve(&self, width: u32, height: u32) -> Option<Rect> {
        if self.get_full_damage() {
            return Some(Rect::new(0.0, 0.0, width as f32, height as f32));
        }
        self.bounds_rect()
    }

    /// Returns the optional accumulated damage rectangle.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Damage region.
    ///
    /// # Returns
    ///
    /// - `Option<Rect>` - Accumulated rectangle if any.
    #[must_use]
    pub fn bounds_rect(&self) -> Option<Rect> {
        let Self { rect, .. } = *self;
        rect
    }

    /// Resets the damage region to clean.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable damage region.
    pub fn clear(&mut self) {
        self.set_full_damage(false);
        self.set_rect(None);
    }
}
