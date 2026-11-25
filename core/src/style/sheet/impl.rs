use super::*;

/// Fluent zero-allocation builder methods for `Style`.
impl Style {
    /// Returns a new empty style initialized with defaults.
    ///
    /// # Returns
    ///
    /// - `Self` - Default `Style`.
    #[must_use]
    pub fn empty() -> Self {
        Self::default()
    }

    /// Sets the box display mode.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source style.
    /// - `Display` - Target display mode.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated style.
    #[must_use]
    pub fn display(mut self, display: Display) -> Self {
        self.get_mut_layout().set_display(display);
        self
    }

    /// Sets the positioning mode.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source style.
    /// - `Position` - Target position mode.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated style.
    #[must_use]
    pub fn position(mut self, position: Position) -> Self {
        self.get_mut_layout().set_position(position);
        self
    }

    /// Configures a horizontal flex row.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source style.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated style.
    #[must_use]
    pub fn row(mut self) -> Self {
        if self.get_layout().get_display() != Display::None {
            self.get_mut_layout().set_display(Display::Flex);
        }
        self.get_mut_layout().set_flex_direction(FlexDirection::Row);
        self
    }

    /// Configures a vertical flex column.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source style.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated style.
    #[must_use]
    pub fn column(mut self) -> Self {
        if self.get_layout().get_display() != Display::None {
            self.get_mut_layout().set_display(Display::Flex);
        }
        self.get_mut_layout()
            .set_flex_direction(FlexDirection::Column);
        self
    }

    /// Configures a grid layout with `columns` equal-width columns.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source style.
    /// - `u16` - Number of grid columns (clamped to at least 1).
    ///
    /// # Returns
    ///
    /// - `Self` - Updated style.
    #[must_use]
    pub fn grid(mut self, columns: u16) -> Self {
        if self.get_layout().get_display() != Display::None {
            self.get_mut_layout().set_display(Display::Grid);
        }
        self.get_mut_layout().set_grid_columns(columns.max(1));
        self
    }

    /// Sets flex wrapping behavior.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source style.
    /// - `FlexWrap` - Wrap mode.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated style.
    #[must_use]
    pub fn wrap(mut self, wrap: FlexWrap) -> Self {
        self.get_mut_layout().set_flex_wrap(wrap);
        self
    }

    /// Sets main-axis justification.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source style.
    /// - `JustifyContent` - Justification mode.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated style.
    #[must_use]
    pub fn justify(mut self, justify: JustifyContent) -> Self {
        self.get_mut_layout().set_justify_content(justify);
        self
    }

    /// Sets cross-axis alignment.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source style.
    /// - `AlignItems` - Cross-axis alignment.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated style.
    #[must_use]
    pub fn align(mut self, align: AlignItems) -> Self {
        self.get_mut_layout().set_align_items(align);
        self
    }

    /// Sets flex grow factor.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source style.
    /// - `f32` - Flex grow factor.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated style.
    #[must_use]
    pub fn flex_grow(mut self, grow: f32) -> Self {
        self.get_mut_layout().set_flex_grow(grow.max(0.0));
        self
    }

    /// Sets flex shrink factor.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source style.
    /// - `f32` - Flex shrink factor.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated style.
    #[must_use]
    pub fn flex_shrink(mut self, shrink: f32) -> Self {
        self.get_mut_layout().set_flex_shrink(shrink.max(0.0));
        self
    }

    /// Sets width length.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source style.
    /// - `Length` - Target width.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated style.
    #[must_use]
    pub fn width(mut self, width: Length) -> Self {
        self.get_mut_layout().set_width(width);
        self
    }

    /// Sets height length.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source style.
    /// - `Length` - Target height.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated style.
    #[must_use]
    pub fn height(mut self, height: Length) -> Self {
        self.get_mut_layout().set_height(height);
        self
    }

    /// Sets minimum width constraint.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source style.
    /// - `Length` - Minimum width.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated style.
    #[must_use]
    pub fn min_width(mut self, min_width: Length) -> Self {
        self.get_mut_layout().set_min_width(min_width);
        self
    }

    /// Sets minimum height constraint.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source style.
    /// - `Length` - Minimum height.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated style.
    #[must_use]
    pub fn min_height(mut self, min_height: Length) -> Self {
        self.get_mut_layout().set_min_height(min_height);
        self
    }

    /// Sets maximum width constraint.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source style.
    /// - `Length` - Maximum width.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated style.
    #[must_use]
    pub fn max_width(mut self, max_width: Length) -> Self {
        self.get_mut_layout().set_max_width(max_width);
        self
    }

    /// Sets maximum height constraint.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source style.
    /// - `Length` - Maximum height.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated style.
    #[must_use]
    pub fn max_height(mut self, max_height: Length) -> Self {
        self.get_mut_layout().set_max_height(max_height);
        self
    }

    /// Sets an explicit width-to-height aspect ratio constraint.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source style.
    /// - `f32` - Positive aspect ratio (`width / height`).
    ///
    /// # Returns
    ///
    /// - `Self` - Updated style.
    #[must_use]
    pub fn aspect_ratio(mut self, ratio: f32) -> Self {
        if ratio > 0.0 && ratio.is_finite() {
            self.get_mut_layout().set_aspect_ratio(Some(ratio));
        }
        self
    }

    /// Sets uniform gap along both row and column axes.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source style.
    /// - `f32` - Gap in logical pixels.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated style.
    #[must_use]
    pub fn gap(mut self, gap: f32) -> Self {
        let clamped: f32 = gap.max(0.0);
        self.get_mut_layout().set_row_gap(clamped);
        self.get_mut_layout().set_column_gap(clamped);
        self
    }

    /// Sets inner padding edges.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source style.
    /// - `Edges` - Inner padding insets.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated style.
    #[must_use]
    pub fn padding(mut self, padding: Edges) -> Self {
        self.get_mut_layout().set_padding(padding);
        self
    }

    /// Sets outer margin edges.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source style.
    /// - `Edges` - Outer margin insets.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated style.
    #[must_use]
    pub fn margin(mut self, margin: Edges) -> Self {
        self.get_mut_layout().set_margin(margin);
        self
    }

    /// Sets overflow behavior.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source style.
    /// - `Overflow` - Target overflow behavior.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated style.
    #[must_use]
    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.get_mut_layout().set_overflow(overflow);
        self
    }

    /// Sets stacking z-index.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source style.
    /// - `i32` - Stacking layer index.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated style.
    #[must_use]
    pub fn z_index(mut self, z_index: i32) -> Self {
        self.get_mut_layout().set_z_index(z_index);
        self
    }

    /// Sets absolute or fixed inset coordinates.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source style.
    /// - `Option<f32>` - Top offset.
    /// - `Option<f32>` - Right offset.
    /// - `Option<f32>` - Bottom offset.
    /// - `Option<f32>` - Left offset.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated style.
    #[must_use]
    pub fn inset(
        mut self,
        top: Option<f32>,
        right: Option<f32>,
        bottom: Option<f32>,
        left: Option<f32>,
    ) -> Self {
        self.get_mut_layout().set_top(top);
        self.get_mut_layout().set_right(right);
        self.get_mut_layout().set_bottom(bottom);
        self.get_mut_layout().set_left(left);
        self
    }

    /// Sets solid background fill color.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source style.
    /// - `Color` - Background color.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated style.
    #[must_use]
    pub fn background(mut self, color: Color) -> Self {
        self.get_mut_paint().set_background(Some(color));
        self
    }

    /// Sets linear gradient fill.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source style.
    /// - `LinearGradient` - Linear gradient specification.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated style.
    #[must_use]
    pub fn gradient(mut self, gradient: LinearGradient) -> Self {
        self.get_mut_paint().set_gradient(Some(gradient));
        self
    }

    /// Sets border stroke width and color.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source style.
    /// - `f32` - Border width in logical pixels.
    /// - `Color` - Border color.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated style.
    #[must_use]
    pub fn border(mut self, width: f32, color: Color) -> Self {
        self.get_mut_paint()
            .set_border(Border::new(width.max(0.0), color));
        self
    }

    /// Sets uniform corner radius.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source style.
    /// - `f32` - Corner radius in logical pixels.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated style.
    #[must_use]
    pub fn radius(mut self, radius: f32) -> Self {
        self.get_mut_paint()
            .set_radius(CornerRadius::all(radius.max(0.0)));
        self
    }

    /// Sets drop shadow.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source style.
    /// - `Shadow` - Shadow definition.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated style.
    #[must_use]
    pub fn shadow(mut self, shadow: Shadow) -> Self {
        self.get_mut_paint().set_shadow(Some(shadow));
        self
    }

    /// Sets focus ring stroke.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source style.
    /// - `Border` - Focus ring width and color.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated style.
    #[must_use]
    pub fn focus_ring(mut self, ring: Border) -> Self {
        self.get_mut_paint().set_focus_ring(Some(ring));
        self
    }

    /// Sets opacity in `[0.0, 1.0]`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source style.
    /// - `f32` - Opacity factor.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated style.
    #[must_use]
    pub fn opacity(mut self, opacity: f32) -> Self {
        self.get_mut_paint().set_opacity(opacity.clamp(0.0, 1.0));
        self
    }

    /// Sets cursor icon hint.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source style.
    /// - `CursorIcon` - Target cursor icon.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated style.
    #[must_use]
    pub fn cursor(mut self, cursor: CursorIcon) -> Self {
        self.get_mut_paint().set_cursor(cursor);
        self
    }

    /// Sets pointer hit-test participation behavior.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source style.
    /// - `PointerEvents` - Target pointer-events mode.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated style.
    #[must_use]
    pub fn pointer_events(mut self, pointer_events: PointerEvents) -> Self {
        self.get_mut_paint().set_pointer_events(pointer_events);
        self
    }

    /// Sets foreground text color.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source style.
    /// - `Color` - Text color.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated style.
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.get_mut_text().set_color(color);
        self
    }

    /// Sets font size in logical pixels.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source style.
    /// - `f32` - Font size in pixels.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated style.
    #[must_use]
    pub fn font_size(mut self, size: f32) -> Self {
        self.get_mut_text().set_font_size(size.max(6.0));
        self
    }

    /// Sets font weight.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source style.
    /// - `FontWeight` - Target font weight.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated style.
    #[must_use]
    pub fn font_weight(mut self, weight: FontWeight) -> Self {
        self.get_mut_text().set_weight(weight);
        self
    }

    /// Sets font family.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source style.
    /// - `FontFamily` - Semantic font family.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated style.
    #[must_use]
    pub fn font_family(mut self, family: FontFamily) -> Self {
        self.get_mut_text().set_family(family);
        self
    }

    /// Sets horizontal text alignment.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source style.
    /// - `TextAlign` - Text alignment.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated style.
    #[must_use]
    pub fn text_align(mut self, align: TextAlign) -> Self {
        self.get_mut_text().set_align(align);
        self
    }

    /// Sets text overflow behavior.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source style.
    /// - `TextOverflow` - Overflow strategy.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated style.
    #[must_use]
    pub fn text_overflow(mut self, overflow: TextOverflow) -> Self {
        self.get_mut_text().set_overflow(overflow);
        self
    }
}

/// Resolution helpers for `ResponsiveStyle`.
impl ResponsiveStyle {
    /// Resolves the active `Style` for the supplied `Breakpoint`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Responsive style definition.
    /// - `Breakpoint` - Current viewport breakpoint.
    ///
    /// # Returns
    ///
    /// - `Style` - Resolved concrete style.
    #[must_use]
    pub fn resolve(&self, breakpoint: Breakpoint) -> Style {
        let Self {
            base,
            compact,
            medium,
        } = *self;
        match breakpoint {
            Breakpoint::Compact => compact.unwrap_or(base),
            Breakpoint::Medium => medium.unwrap_or(base),
            Breakpoint::Expanded => base,
        }
    }
}
