use super::*;

/// Color constructors, packing, and blending operations.
impl Color {
    /// Constructs an opaque color from RGB channels.
    ///
    /// # Arguments
    ///
    /// - `u8` - Red channel.
    /// - `u8` - Green channel.
    /// - `u8` - Blue channel.
    ///
    /// # Returns
    ///
    /// - `Self` - Opaque RGBA color.
    #[must_use]
    #[inline(always)]
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    /// Constructs a color from RGBA channels.
    ///
    /// # Arguments
    ///
    /// - `u8` - Red channel.
    /// - `u8` - Green channel.
    /// - `u8` - Blue channel.
    /// - `u8` - Alpha channel.
    ///
    /// # Returns
    ///
    /// - `Self` - Constructed RGBA color.
    #[must_use]
    #[inline(always)]
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Constructs an opaque color from a packed `0xRRGGBB` integer literal.
    ///
    /// # Arguments
    ///
    /// - `u32` - Packed 24-bit RGB hexadecimal value.
    ///
    /// # Returns
    ///
    /// - `Self` - Opaque RGBA color.
    #[must_use]
    #[inline(always)]
    pub const fn hex(rgb: u32) -> Self {
        Self {
            r: ((rgb >> 16) & 0xFF) as u8,
            g: ((rgb >> 8) & 0xFF) as u8,
            b: (rgb & 0xFF) as u8,
            a: 255,
        }
    }

    /// Constructs a color from a packed `0xRRGGBBAA` integer literal.
    ///
    /// # Arguments
    ///
    /// - `u32` - Packed 32-bit RGBA hexadecimal value.
    ///
    /// # Returns
    ///
    /// - `Self` - Constructed RGBA color.
    #[must_use]
    #[inline(always)]
    pub const fn hexa(rgba: u32) -> Self {
        Self {
            r: ((rgba >> 24) & 0xFF) as u8,
            g: ((rgba >> 16) & 0xFF) as u8,
            b: ((rgba >> 8) & 0xFF) as u8,
            a: (rgba & 0xFF) as u8,
        }
    }

    /// Returns a copy of this color with its alpha replaced by `alpha`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Source color.
    /// - `u8` - Replacement alpha channel.
    ///
    /// # Returns
    ///
    /// - `Self` - Color with updated alpha channel.
    #[must_use]
    #[inline(always)]
    pub fn with_alpha(&self, alpha: u8) -> Self {
        Self::new(self.get_r(), self.get_g(), self.get_b(), alpha)
    }

    /// Returns a copy of this color with alpha multiplied by `opacity` in `[0.0, 1.0]`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Source color.
    /// - `f32` - Opacity factor clamped to `[0.0, 1.0]`.
    ///
    /// # Returns
    ///
    /// - `Self` - Color with scaled alpha channel.
    #[must_use]
    #[inline(always)]
    pub fn multiply_alpha(&self, opacity: f32) -> Self {
        let clamped: f32 = opacity.clamp(0.0, 1.0);
        let scaled: u8 = ((self.get_a() as f32) * clamped + 0.5).clamp(0.0, CHANNEL_MAX_F32) as u8;
        self.with_alpha(scaled)
    }

    /// Converts the color to normalized `[r, g, b, a]` floating-point components in `[0.0, 1.0]`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Source color.
    ///
    /// # Returns
    ///
    /// - `[f32; 4]` - Normalized RGBA components for GPU uniforms and vertex attributes.
    #[must_use]
    #[inline(always)]
    pub fn to_f32_array(&self) -> [f32; 4] {
        [
            (self.get_r() as f32) / CHANNEL_MAX_F32,
            (self.get_g() as f32) / CHANNEL_MAX_F32,
            (self.get_b() as f32) / CHANNEL_MAX_F32,
            (self.get_a() as f32) / CHANNEL_MAX_F32,
        ]
    }

    /// Packs the color into little-endian RGBA8 bytes `[r, g, b, a]`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Source color.
    ///
    /// # Returns
    ///
    /// - `[u8; 4]` - RGBA byte array.
    #[must_use]
    #[inline(always)]
    pub fn to_rgba_bytes(&self) -> [u8; 4] {
        [self.get_r(), self.get_g(), self.get_b(), self.get_a()]
    }

    /// Packs the color into BGRA8 bytes `[b, g, r, a]` used by Win32 GDI and Cocoa/Metal.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Source color.
    ///
    /// # Returns
    ///
    /// - `[u8; 4]` - BGRA byte array.
    #[must_use]
    #[inline(always)]
    pub fn to_bgra_bytes(&self) -> [u8; 4] {
        [self.get_b(), self.get_g(), self.get_r(), self.get_a()]
    }

    /// Linearly interpolates between `self` and `other` by factor `t` in `[0.0, 1.0]`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Start color at `t = 0.0`.
    /// - `Self` - End color at `t = 1.0`.
    /// - `f32` - Interpolation parameter clamped to `[0.0, 1.0]`.
    ///
    /// # Returns
    ///
    /// - `Self` - Interpolated RGBA color.
    #[must_use]
    #[inline(always)]
    pub fn lerp(&self, other: Self, t: f32) -> Self {
        let f: f32 = t.clamp(0.0, 1.0);
        let inv: f32 = 1.0 - f;
        let r: u8 = ((self.get_r() as f32) * inv + (other.get_r() as f32) * f + 0.5) as u8;
        let g: u8 = ((self.get_g() as f32) * inv + (other.get_g() as f32) * f + 0.5) as u8;
        let b: u8 = ((self.get_b() as f32) * inv + (other.get_b() as f32) * f + 0.5) as u8;
        let a: u8 = ((self.get_a() as f32) * inv + (other.get_a() as f32) * f + 0.5) as u8;
        Self::new(r, g, b, a)
    }

    /// Porter-Duff source-over composites `self` on top of opaque or translucent `dst`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Foreground source color.
    /// - `Self` - Background destination color.
    ///
    /// # Returns
    ///
    /// - `Self` - Composited RGBA color.
    #[must_use]
    #[inline(always)]
    pub fn blend_over(&self, dst: Self) -> Self {
        let sa: u32 = self.get_a() as u32;
        if sa == 0 {
            return dst;
        }
        if sa == 255 {
            return *self;
        }
        let inv_sa: u32 = 255 - sa;
        let da: u32 = dst.get_a() as u32;
        let out_a: u32 = sa + ((da * inv_sa + 127) / 255);
        if out_a == 0 {
            return COLOR_TRANSPARENT;
        }
        let r: u32 = (((self.get_r() as u32) * sa)
            + (((dst.get_r() as u32) * da * inv_sa + 127) / 255))
            / out_a;
        let g: u32 = (((self.get_g() as u32) * sa)
            + (((dst.get_g() as u32) * da * inv_sa + 127) / 255))
            / out_a;
        let b: u32 = (((self.get_b() as u32) * sa)
            + (((dst.get_b() as u32) * da * inv_sa + 127) / 255))
            / out_a;
        Self::new(
            r.min(255) as u8,
            g.min(255) as u8,
            b.min(255) as u8,
            out_a.min(255) as u8,
        )
    }
}
