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

    /// Computes exact rounded integer multiplication `(a * b + 127) / 255` for 8-bit channels without division.
    ///
    /// # Arguments
    ///
    /// - `u8` - First 8-bit channel operand.
    /// - `u8` - Second 8-bit channel operand.
    ///
    /// # Returns
    ///
    /// - `u8` - Exact rounded 8-bit product.
    #[must_use]
    #[inline(always)]
    pub const fn mul_channel_u8(a: u8, b: u8) -> u8 {
        ((((a as u32) * (b as u32) + 128) * 257) >> 16) as u8
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

    /// Returns a copy of this color with alpha scaled by an 8-bit factor `alpha_scale` in `0..=255`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Source color.
    /// - `u8` - 8-bit alpha scale factor (`0` = transparent, `255` = unchanged).
    ///
    /// # Returns
    ///
    /// - `Self` - Color with exact fixed-point scaled alpha channel.
    #[must_use]
    #[inline(always)]
    pub const fn multiply_alpha_u8(&self, alpha_scale: u8) -> Self {
        let Self { r, g, b, a } = *self;
        if alpha_scale == 255 || a == 0 {
            return Self { r, g, b, a };
        }
        if alpha_scale == 0 {
            return Self { r, g, b, a: 0 };
        }
        Self {
            r,
            g,
            b,
            a: Self::mul_channel_u8(a, alpha_scale),
        }
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
        if opacity >= 1.0 {
            return *self;
        }
        if opacity <= 0.0 || opacity.is_nan() {
            return self.with_alpha(0);
        }
        let a: u32 = self.get_a() as u32;
        if a == 0 {
            return *self;
        }
        let scale_u32: u32 = (opacity * CHANNEL_MAX_F32 + 0.5) as u32;
        let scaled: u8 = (((a * scale_u32 + 128) * 257) >> 16) as u8;
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
        const INV_CHANNEL_MAX_F32: f32 = 1.0 / CHANNEL_MAX_F32;
        [
            (self.get_r() as f32) * INV_CHANNEL_MAX_F32,
            (self.get_g() as f32) * INV_CHANNEL_MAX_F32,
            (self.get_b() as f32) * INV_CHANNEL_MAX_F32,
            (self.get_a() as f32) * INV_CHANNEL_MAX_F32,
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

    /// Linearly interpolates between `self` and `other` using an exact 8-bit integer weight `0..=255`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Start color at `weight = 0`.
    /// - `Self` - End color at `weight = 255`.
    /// - `u8` - Integer interpolation weight in `0..=255`.
    ///
    /// # Returns
    ///
    /// - `Self` - Interpolated RGBA color.
    #[must_use]
    #[inline(always)]
    pub const fn lerp_u8(&self, other: Self, weight: u8) -> Self {
        if weight == 0 {
            return *self;
        }
        if weight == 255 {
            return other;
        }
        let Self {
            r: r0,
            g: g0,
            b: b0,
            a: a0,
        } = *self;
        let Self {
            r: r1,
            g: g1,
            b: b1,
            a: a1,
        } = other;
        let w: u32 = weight as u32;
        let inv_w: u32 = 255 - w;
        let r: u8 = ((((r0 as u32) * inv_w + (r1 as u32) * w + 128) * 257) >> 16) as u8;
        let g: u8 = ((((g0 as u32) * inv_w + (g1 as u32) * w + 128) * 257) >> 16) as u8;
        let b: u8 = ((((b0 as u32) * inv_w + (b1 as u32) * w + 128) * 257) >> 16) as u8;
        let a: u8 = ((((a0 as u32) * inv_w + (a1 as u32) * w + 128) * 257) >> 16) as u8;
        Self { r, g, b, a }
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
        if t <= 0.0 || t.is_nan() {
            return *self;
        }
        if t >= 1.0 {
            return other;
        }
        let w: u32 = (t * 256.0 + 0.5) as u32;
        let inv_w: u32 = 256 - w;
        let rb0: u32 = (self.get_r() as u32) | ((self.get_b() as u32) << 16);
        let rb1: u32 = (other.get_r() as u32) | ((other.get_b() as u32) << 16);
        let rb: u32 = rb0 * inv_w + rb1 * w + 0x0080_0080;
        let ga0: u32 = (self.get_g() as u32) | ((self.get_a() as u32) << 16);
        let ga1: u32 = (other.get_g() as u32) | ((other.get_a() as u32) << 16);
        let ga: u32 = ga0 * inv_w + ga1 * w + 0x0080_0080;
        Self::new(
            ((rb >> 8) & 0xFF) as u8,
            ((ga >> 8) & 0xFF) as u8,
            ((rb >> 24) & 0xFF) as u8,
            ((ga >> 24) & 0xFF) as u8,
        )
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
        if da == 255 {
            let r: u8 = ((((self.get_r() as u32) * sa + (dst.get_r() as u32) * inv_sa + 128) * 257)
                >> 16) as u8;
            let g: u8 = ((((self.get_g() as u32) * sa + (dst.get_g() as u32) * inv_sa + 128) * 257)
                >> 16) as u8;
            let b: u8 = ((((self.get_b() as u32) * sa + (dst.get_b() as u32) * inv_sa + 128) * 257)
                >> 16) as u8;
            return Self::new(r, g, b, 255);
        }
        let da_inv: u32 = ((da * inv_sa + 128) * 257) >> 16;
        let out_a: u32 = sa + da_inv;
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
