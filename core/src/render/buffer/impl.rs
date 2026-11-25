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
            px[0] = ((sr * sa_u32 + dr * inv_a + 127) / 255) as u8;
            px[1] = ((sg * sa_u32 + dg * inv_a + 127) / 255) as u8;
            px[2] = ((sb * sa_u32 + db * inv_a + 127) / 255) as u8;
            px[3] = (sa_u32 + (da * inv_a + 127) / 255).min(255) as u8;
        }
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
