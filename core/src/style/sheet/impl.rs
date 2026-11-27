use super::*;

/// Fluent zero-allocation builder methods for `Style`.
impl Style {
    /// Applies any strongly-typed `StyleApply` value (e.g. `AlignItems::Center`, `Overflow::Hidden`, `Shadow`) to this style.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source style.
    /// - `T` - Typed property implementing `StyleApply`.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated style.
    #[must_use]
    #[inline(always)]
    pub fn apply<T>(self, prop: T) -> Self
    where
        T: StyleApply,
    {
        prop.apply_to(self)
    }

    /// Sets the flex container axis direction (`FlexDirection::Row` or `FlexDirection::Column`).
    ///
    /// # Arguments
    ///
    /// - `Self` - Source style.
    /// - `FlexDirection` - Target flex axis direction.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated style.
    #[must_use]
    #[inline(always)]
    pub fn direction(mut self, direction: FlexDirection) -> Self {
        if self.get_layout().get_display() != Display::None {
            self.get_mut_layout().set_display(Display::Flex);
        }
        self.get_mut_layout().set_flex_direction(direction);
        self
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
    pub fn font_family(mut self, family: FontFamily) -> Self {
        self.get_mut_text().set_family(family);
        self
    }

    /// Sets italic font slant.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source style.
    /// - `bool` - Whether italic slant is active.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated style.
    #[must_use]
    #[inline(always)]
    pub fn italic(mut self, italic: bool) -> Self {
        self.get_mut_text().set_italic(italic);
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
    #[inline(always)]
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
    #[inline(always)]
    pub fn text_overflow(mut self, overflow: TextOverflow) -> Self {
        self.get_mut_text().set_overflow(overflow);
        self
    }

    /// Enables or disables native text selection on this node and its descendant text subtree.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source style.
    /// - `bool` - Whether native text selection is enabled (`true`) or explicitly disabled (`false`).
    ///
    /// # Returns
    ///
    /// - `Self` - Updated style.
    #[must_use]
    #[inline(always)]
    pub fn selectable(mut self, selectable: bool) -> Self {
        self.get_mut_text()
            .set_selectable(selectable)
            .set_selectable_explicit(true);
        self
    }

    /// Sets the translucent highlight fill color used when rendering selected text ranges.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source style.
    /// - `Color` - Selection highlight color.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated style.
    #[must_use]
    #[inline(always)]
    pub fn selection_color(mut self, color: Color) -> Self {
        self.get_mut_text().set_selection_color(Some(color));
        self
    }

    /// Configures whether scroll deltas stop at this node (`true`) or chain/penetrate to parent scrollables (`false`).
    ///
    /// # Arguments
    ///
    /// - `Self` - Source style.
    /// - `bool` - `true` blocks scroll penetration to ancestors; `false` permits scroll chaining.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated style.
    #[must_use]
    #[inline(always)]
    pub fn stop_scroll_propagation(mut self, stop: bool) -> Self {
        self.get_mut_paint().set_stop_scroll_propagation(stop);
        self
    }

    /// Configures whether click/pointer events stop bubbling at this node (`true`) or bubble to ancestors (`false`).
    ///
    /// # Arguments
    ///
    /// - `Self` - Source style.
    /// - `bool` - `true` blocks event bubbling; `false` permits bubbling.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated style.
    #[must_use]
    #[inline(always)]
    pub fn stop_event_propagation(mut self, stop: bool) -> Self {
        self.get_mut_paint().set_stop_event_propagation(stop);
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
    #[inline(always)]
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

/// `StyleApply` implementation for `Display`.
impl StyleApply for Display {
    /// Applies `Display` to `style`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Target display mode.
    /// - `Style` - Style accumulator.
    ///
    /// # Returns
    ///
    /// - `Style` - Updated style.
    #[inline(always)]
    fn apply_to(self, style: Style) -> Style {
        style.display(self)
    }
}

/// `StyleApply` implementation for `Position`.
impl StyleApply for Position {
    /// Applies `Position` to `style`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Target positioning mode.
    /// - `Style` - Style accumulator.
    ///
    /// # Returns
    ///
    /// - `Style` - Updated style.
    #[inline(always)]
    fn apply_to(self, style: Style) -> Style {
        style.position(self)
    }
}

/// `StyleApply` implementation for `FlexDirection`.
impl StyleApply for FlexDirection {
    /// Applies `FlexDirection` to `style`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Target flex axis direction.
    /// - `Style` - Style accumulator.
    ///
    /// # Returns
    ///
    /// - `Style` - Updated style.
    #[inline(always)]
    fn apply_to(self, style: Style) -> Style {
        style.direction(self)
    }
}

/// `StyleApply` implementation for `FlexWrap`.
impl StyleApply for FlexWrap {
    /// Applies `FlexWrap` to `style`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Target flex wrap mode.
    /// - `Style` - Style accumulator.
    ///
    /// # Returns
    ///
    /// - `Style` - Updated style.
    #[inline(always)]
    fn apply_to(self, style: Style) -> Style {
        style.wrap(self)
    }
}

/// `StyleApply` implementation for `JustifyContent`.
impl StyleApply for JustifyContent {
    /// Applies `JustifyContent` to `style`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Target main-axis justification.
    /// - `Style` - Style accumulator.
    ///
    /// # Returns
    ///
    /// - `Style` - Updated style.
    #[inline(always)]
    fn apply_to(self, style: Style) -> Style {
        style.justify(self)
    }
}

/// `StyleApply` implementation for `AlignItems`.
impl StyleApply for AlignItems {
    /// Applies `AlignItems` to `style`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Target cross-axis alignment.
    /// - `Style` - Style accumulator.
    ///
    /// # Returns
    ///
    /// - `Style` - Updated style.
    #[inline(always)]
    fn apply_to(self, style: Style) -> Style {
        style.align(self)
    }
}

/// `StyleApply` implementation for `Overflow`.
impl StyleApply for Overflow {
    /// Applies `Overflow` to `style`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Target overflow behavior.
    /// - `Style` - Style accumulator.
    ///
    /// # Returns
    ///
    /// - `Style` - Updated style.
    #[inline(always)]
    fn apply_to(self, style: Style) -> Style {
        style.overflow(self)
    }
}

/// `StyleApply` implementation for `CursorIcon`.
impl StyleApply for CursorIcon {
    /// Applies `CursorIcon` to `style`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Target cursor hint.
    /// - `Style` - Style accumulator.
    ///
    /// # Returns
    ///
    /// - `Style` - Updated style.
    #[inline(always)]
    fn apply_to(self, style: Style) -> Style {
        style.cursor(self)
    }
}

/// `StyleApply` implementation for `PointerEvents`.
impl StyleApply for PointerEvents {
    /// Applies `PointerEvents` to `style`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Target pointer-events mode.
    /// - `Style` - Style accumulator.
    ///
    /// # Returns
    ///
    /// - `Style` - Updated style.
    #[inline(always)]
    fn apply_to(self, style: Style) -> Style {
        style.pointer_events(self)
    }
}

/// `StyleApply` implementation for `FontWeight`.
impl StyleApply for FontWeight {
    /// Applies `FontWeight` to `style`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Target font weight.
    /// - `Style` - Style accumulator.
    ///
    /// # Returns
    ///
    /// - `Style` - Updated style.
    #[inline(always)]
    fn apply_to(self, style: Style) -> Style {
        style.font_weight(self)
    }
}

/// `StyleApply` implementation for `FontFamily`.
impl StyleApply for FontFamily {
    /// Applies `FontFamily` to `style`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Target font family.
    /// - `Style` - Style accumulator.
    ///
    /// # Returns
    ///
    /// - `Style` - Updated style.
    #[inline(always)]
    fn apply_to(self, style: Style) -> Style {
        style.font_family(self)
    }
}

/// `StyleApply` implementation for `TextAlign`.
impl StyleApply for TextAlign {
    /// Applies `TextAlign` to `style`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Target horizontal text alignment.
    /// - `Style` - Style accumulator.
    ///
    /// # Returns
    ///
    /// - `Style` - Updated style.
    #[inline(always)]
    fn apply_to(self, style: Style) -> Style {
        style.text_align(self)
    }
}

/// `StyleApply` implementation for `TextOverflow`.
impl StyleApply for TextOverflow {
    /// Applies `TextOverflow` to `style`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Target text overflow strategy.
    /// - `Style` - Style accumulator.
    ///
    /// # Returns
    ///
    /// - `Style` - Updated style.
    #[inline(always)]
    fn apply_to(self, style: Style) -> Style {
        style.text_overflow(self)
    }
}

/// `StyleApply` implementation for `Shadow`.
impl StyleApply for Shadow {
    /// Applies `Shadow` to `style`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Drop shadow specification.
    /// - `Style` - Style accumulator.
    ///
    /// # Returns
    ///
    /// - `Style` - Updated style.
    #[inline(always)]
    fn apply_to(self, style: Style) -> Style {
        style.shadow(self)
    }
}

/// `StyleApply` implementation for `LinearGradient`.
impl StyleApply for LinearGradient {
    /// Applies `LinearGradient` to `style`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Linear gradient fill.
    /// - `Style` - Style accumulator.
    ///
    /// # Returns
    ///
    /// - `Style` - Updated style.
    #[inline(always)]
    fn apply_to(self, style: Style) -> Style {
        style.gradient(self)
    }
}

/// `StyleApply` implementation for `CornerRadius`.
impl StyleApply for CornerRadius {
    /// Applies `CornerRadius` to `style`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Corner radius specification.
    /// - `Style` - Style accumulator.
    ///
    /// # Returns
    ///
    /// - `Style` - Updated style.
    #[inline(always)]
    fn apply_to(self, style: Style) -> Style {
        let mut out: Style = style;
        out.get_mut_paint().set_radius(self);
        out
    }
}

/// `StyleApply` implementation for `Style` allowing base-style merging inside newline-separated `style!` blocks.
impl StyleApply for Style {
    /// Replaces `style` with `self` as a base preset inside `style!`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Base style preset.
    /// - `Style` - Previous accumulator.
    ///
    /// # Returns
    ///
    /// - `Style` - Base style value.
    #[inline(always)]
    fn apply_to(self, _style: Style) -> Style {
        self
    }
}
