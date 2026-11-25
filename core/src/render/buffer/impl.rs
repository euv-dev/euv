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

    /// Fills the entire framebuffer with solid `color`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable framebuffer.
    /// - `Color` - Clear color.
    pub fn clear(&mut self, color: Color) {
        let rgba: [u8; 4] = color.to_rgba_bytes();
        for chunk in self.get_mut_pixels().as_chunks_mut::<4>().0.iter_mut() {
            *chunk = rgba;
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

    /// Computes a deterministic 64-bit FNV-1a hash of the framebuffer pixels for cross-platform parity verification.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Framebuffer to hash.
    ///
    /// # Returns
    ///
    /// - `u64` - Deterministic pixel hash.
    #[must_use]
    pub fn fnv1a_hash(&self) -> u64 {
        let mut hash: u64 = FNV1A_OFFSET_BASIS;
        for byte in self.get_pixels().iter().copied() {
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
