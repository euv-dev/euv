use super::*;

/// Resolution helpers for `Length`.
impl Length {
    /// Resolves this length into logical pixels given parent and viewport dimensions.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Length specification to resolve.
    /// - `f32` - Parent dimension along the relevant axis.
    /// - `Size` - Root viewport logical size.
    ///
    /// # Returns
    ///
    /// - `Option<f32>` - Resolved pixel size, or `None` for `Auto` and `Fraction`.
    #[must_use]
    #[inline(always)]
    pub fn resolve(&self, parent_extent: f32, viewport: Size) -> Option<f32> {
        match *self {
            Self::Auto | Self::Fraction(_) => None,
            Self::Px(px) => Some(px),
            Self::Percent(pct) => Some(parent_extent * (pct * 0.01)),
            Self::ViewportWidth(vw) => Some(viewport.get_width() * (vw * 0.01)),
            Self::ViewportHeight(vh) => Some(viewport.get_height() * (vh * 0.01)),
        }
    }
}

/// Convenience constructors and sums for `Edges`.
impl Edges {
    /// Creates uniform edge insets on all four sides.
    ///
    /// # Arguments
    ///
    /// - `f32` - Uniform inset in logical pixels.
    ///
    /// # Returns
    ///
    /// - `Self` - Uniform `Edges`.
    #[must_use]
    #[inline(always)]
    pub const fn all(value: f32) -> Self {
        Self {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
    }

    /// Creates symmetric vertical and horizontal edge insets.
    ///
    /// # Arguments
    ///
    /// - `f32` - Vertical (top and bottom) inset.
    /// - `f32` - Horizontal (left and right) inset.
    ///
    /// # Returns
    ///
    /// - `Self` - Symmetric `Edges`.
    #[must_use]
    #[inline(always)]
    pub const fn symmetric(vertical: f32, horizontal: f32) -> Self {
        Self {
            top: vertical,
            right: horizontal,
            bottom: vertical,
            left: horizontal,
        }
    }

    /// Returns total horizontal thickness (`left + right`).
    ///
    /// # Arguments
    ///
    /// - `&Self` - Edge insets.
    ///
    /// # Returns
    ///
    /// - `f32` - Combined horizontal inset.
    #[must_use]
    #[inline(always)]
    pub fn horizontal(&self) -> f32 {
        self.get_left() + self.get_right()
    }

    /// Returns total vertical thickness (`top + bottom`).
    ///
    /// # Arguments
    ///
    /// - `&Self` - Edge insets.
    ///
    /// # Returns
    ///
    /// - `f32` - Combined vertical inset.
    #[must_use]
    #[inline(always)]
    pub fn vertical(&self) -> f32 {
        self.get_top() + self.get_bottom()
    }
}

/// Convenience constructors for `CornerRadius`.
impl CornerRadius {
    /// Creates uniform corner radius on all four corners.
    ///
    /// # Arguments
    ///
    /// - `f32` - Uniform radius in logical pixels.
    ///
    /// # Returns
    ///
    /// - `Self` - Uniform `CornerRadius`.
    #[must_use]
    #[inline(always)]
    pub const fn all(radius: f32) -> Self {
        Self {
            top_left: radius,
            top_right: radius,
            bottom_right: radius,
            bottom_left: radius,
        }
    }

    /// Returns the maximum corner radius across all four corners.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Corner radii.
    ///
    /// # Returns
    ///
    /// - `f32` - Largest radius in logical pixels.
    #[must_use]
    #[inline(always)]
    pub fn max_radius(&self) -> f32 {
        self.get_top_left()
            .max(self.get_top_right())
            .max(self.get_bottom_right())
            .max(self.get_bottom_left())
    }
}

/// Geometry queries and clipping helpers for `Rect`.
impl Rect {
    /// Returns the right edge coordinate (`x + width`).
    ///
    /// # Arguments
    ///
    /// - `&Self` - Rectangle.
    ///
    /// # Returns
    ///
    /// - `f32` - Right edge X coordinate.
    #[must_use]
    #[inline(always)]
    pub fn right(&self) -> f32 {
        self.get_x() + self.get_width()
    }

    /// Returns the bottom edge coordinate (`y + height`).
    ///
    /// # Arguments
    ///
    /// - `&Self` - Rectangle.
    ///
    /// # Returns
    ///
    /// - `f32` - Bottom edge Y coordinate.
    #[must_use]
    #[inline(always)]
    pub fn bottom(&self) -> f32 {
        self.get_y() + self.get_height()
    }

    /// Tests whether `point` lies inside this rectangle.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Rectangle bounds.
    /// - `Point` - Point to test.
    ///
    /// # Returns
    ///
    /// - `bool` - True when the point is within `[x, right)` and `[y, bottom)`.
    #[must_use]
    #[inline(always)]
    pub fn contains(&self, point: Point) -> bool {
        point.get_x() >= self.get_x()
            && point.get_x() < self.right()
            && point.get_y() >= self.get_y()
            && point.get_y() < self.bottom()
    }

    /// Computes the intersection rectangle of `self` and `other`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - First rectangle.
    /// - `Self` - Second rectangle.
    ///
    /// # Returns
    ///
    /// - `Option<Self>` - Non-empty overlapping rectangle, or `None` when disjoint.
    #[must_use]
    #[inline(always)]
    pub fn intersect(&self, other: Self) -> Option<Self> {
        let x0: f32 = self.get_x().max(other.get_x());
        let y0: f32 = self.get_y().max(other.get_y());
        let x1: f32 = self.right().min(other.right());
        let y1: f32 = self.bottom().min(other.bottom());
        if x1 > x0 && y1 > y0 {
            Some(Self::new(x0, y0, x1 - x0, y1 - y0))
        } else {
            None
        }
    }

    /// Computes the bounding union of `self` and `other`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - First rectangle.
    /// - `Self` - Second rectangle.
    ///
    /// # Returns
    ///
    /// - `Self` - Enclosing rectangle containing both inputs.
    #[must_use]
    #[inline(always)]
    pub fn union(&self, other: Self) -> Self {
        if self.get_width() <= 0.0 || self.get_height() <= 0.0 {
            return other;
        }
        if other.get_width() <= 0.0 || other.get_height() <= 0.0 {
            return *self;
        }
        let x0: f32 = self.get_x().min(other.get_x());
        let y0: f32 = self.get_y().min(other.get_y());
        let x1: f32 = self.right().max(other.right());
        let y1: f32 = self.bottom().max(other.bottom());
        Self::new(x0, y0, x1 - x0, y1 - y0)
    }

    /// Insets the rectangle by `edges`, clamping width and height to non-negative values.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Outer rectangle.
    /// - `Edges` - Edge insets to subtract.
    ///
    /// # Returns
    ///
    /// - `Self` - Inset inner rectangle.
    #[must_use]
    #[inline(always)]
    pub fn inset(&self, edges: Edges) -> Self {
        let w: f32 = (self.get_width() - edges.horizontal()).max(0.0);
        let h: f32 = (self.get_height() - edges.vertical()).max(0.0);
        Self::new(
            self.get_x() + edges.get_left(),
            self.get_y() + edges.get_top(),
            w,
            h,
        )
    }
}
