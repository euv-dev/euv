use super::*;

/// Identity default for `Transform2D`.
impl Default for Transform2D {
    /// Creates an identity 2D transform (`scale = 1.0`, `translate = 0.0`, `rotation = 0.0`).
    ///
    /// # Returns
    ///
    /// - `Self` - Identity `Transform2D`.
    #[inline(always)]
    fn default() -> Self {
        Self::new(0.0, 0.0, 1.0, 1.0, 0.0)
    }
}

/// Default paint properties (`opacity = 1.0`, transparent background).
impl Default for PaintStyle {
    /// Creates default transparent paint properties.
    ///
    /// # Returns
    ///
    /// - `Self` - Default `PaintStyle`.
    #[inline(always)]
    fn default() -> Self {
        Self {
            background: None,
            gradient: None,
            border: Border::default(),
            radius: CornerRadius::default(),
            shadow: None,
            focus_ring: None,
            opacity: 1.0,
            transform: Transform2D::default(),
            cursor: CursorIcon::Default,
            pointer_events: PointerEvents::Auto,
            stop_scroll_propagation: false,
            stop_event_propagation: false,
        }
    }
}

/// Exact fixed-point alpha scaling and visibility helpers for `Shadow`.
impl Shadow {
    /// Returns a copy of this shadow with its tint alpha scaled by `opacity` in `[0.0, 1.0]`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Source drop shadow.
    /// - `f32` - Opacity multiplier clamped to `[0.0, 1.0]`.
    ///
    /// # Returns
    ///
    /// - `Self` - Shadow with exact fixed-point scaled tint alpha.
    #[must_use]
    #[inline(always)]
    pub fn with_opacity(&self, opacity: f32) -> Self {
        if opacity >= 1.0 {
            return *self;
        }
        let Self {
            offset_x,
            offset_y,
            blur_radius,
            spread_radius,
            color,
        } = *self;
        Self {
            offset_x,
            offset_y,
            blur_radius,
            spread_radius,
            color: color.multiply_alpha(opacity),
        }
    }

    /// Returns a copy of this shadow with its tint alpha scaled by an 8-bit integer factor `0..=255`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Source drop shadow.
    /// - `u8` - 8-bit integer alpha multiplier (`0` = transparent, `255` = unchanged).
    ///
    /// # Returns
    ///
    /// - `Self` - Shadow with exact fixed-point scaled tint alpha.
    #[must_use]
    #[inline(always)]
    pub const fn with_alpha_u8(&self, alpha_scale: u8) -> Self {
        let Self {
            offset_x,
            offset_y,
            blur_radius,
            spread_radius,
            color,
        } = *self;
        Self {
            offset_x,
            offset_y,
            blur_radius,
            spread_radius,
            color: color.multiply_alpha_u8(alpha_scale),
        }
    }

    /// Returns `true` when this shadow has non-zero alpha and non-zero visual extent.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Drop shadow to inspect.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the shadow contributes visible pixels.
    #[must_use]
    #[inline(always)]
    pub const fn is_visible(&self) -> bool {
        let Self {
            offset_x,
            offset_y,
            blur_radius,
            spread_radius,
            color,
        } = *self;
        let Color { a, .. } = color;
        if a == 0 {
            return false;
        }
        if blur_radius > 0.0 {
            return true;
        }
        if spread_radius > 0.0 {
            return true;
        }
        offset_x != 0.0 || offset_y != 0.0
    }
}

/// Inspection helpers for `Transform2D`.
impl Transform2D {
    /// Returns true when the transform is the exact identity transform.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Transform to inspect.
    ///
    /// # Returns
    ///
    /// - `bool` - True when translation and rotation are zero and scale is one.
    #[must_use]
    #[inline(always)]
    pub fn is_identity(&self) -> bool {
        self.get_translate_x() == 0.0
            && self.get_translate_y() == 0.0
            && self.get_scale_x() == 1.0
            && self.get_scale_y() == 1.0
            && self.get_rotation_rad() == 0.0
    }
}

/// Non-panicking optional field readers for `PaintStyle`.
impl PaintStyle {
    /// Returns the optional background color.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Paint style.
    ///
    /// # Returns
    ///
    /// - `Option<Color>` - Background color if set.
    #[must_use]
    #[inline(always)]
    pub fn background_color(&self) -> Option<Color> {
        let Self { background, .. } = *self;
        background
    }

    /// Returns the optional linear gradient fill.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Paint style.
    ///
    /// # Returns
    ///
    /// - `Option<LinearGradient>` - Linear gradient if set.
    #[must_use]
    #[inline(always)]
    pub fn background_gradient(&self) -> Option<LinearGradient> {
        let Self { gradient, .. } = *self;
        gradient
    }

    /// Returns the optional drop shadow.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Paint style.
    ///
    /// # Returns
    ///
    /// - `Option<Shadow>` - Drop shadow if set.
    #[must_use]
    #[inline(always)]
    pub fn box_shadow(&self) -> Option<Shadow> {
        let Self { shadow, .. } = *self;
        shadow
    }

    /// Returns the optional focus ring stroke.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Paint style.
    ///
    /// # Returns
    ///
    /// - `Option<Border>` - Focus ring stroke if set.
    #[must_use]
    #[inline(always)]
    pub fn focus_ring_border(&self) -> Option<Border> {
        let Self { focus_ring, .. } = *self;
        focus_ring
    }
}
