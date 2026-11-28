use super::*;

/// Default neutral interactive state spanning `0%..=100%` with all series visible.
impl Default for EuvChartState {
    /// Creates an initial neutral chart interaction state.
    ///
    /// # Returns
    ///
    /// - `Self` - Default chart state.
    fn default() -> Self {
        Self {
            hidden_series: Vec::new(),
            zoom_start_pct: 0.0,
            zoom_end_pct: 100.0,
            magic_type: EuvChartMagicType::Default,
            data_view_open: false,
            selected_pie_slice: None,
            hover_pos: None,
        }
    }
}

/// Constructors and hit-testing helpers for `EuvChartDatumHit`.
impl EuvChartDatumHit {
    /// Creates a circular point datum hit target.
    ///
    /// # Arguments
    ///
    /// - `(usize, String, usize)` - Tuple `(series_index, series_name, data_index)`.
    /// - `(String, f32, String)` - Tuple `(category_label, value, detail)`.
    /// - `Color` - Swatch color.
    /// - `Point` - Anchor coordinate.
    /// - `f32` - Hit radius in logical pixels.
    ///
    /// # Returns
    ///
    /// - `Self` - Point hit record.
    #[must_use]
    pub(crate) fn point_hit(
        id: (usize, String, usize),
        info: (String, f32, String),
        color: Color,
        anchor: Point,
        hit_radius: f32,
    ) -> Self {
        let (series_index, series_name, data_index): (usize, String, usize) = id;
        let (category_label, value, detail): (String, f32, String) = info;
        Self {
            series_index,
            series_name,
            data_index,
            category_label,
            value,
            detail,
            color,
            anchor,
            hit_radius,
            hit_rect: None,
        }
    }

    /// Creates a rectangular box datum hit target.
    ///
    /// # Arguments
    ///
    /// - `(usize, String, usize)` - Tuple `(series_index, series_name, data_index)`.
    /// - `(String, f32, String)` - Tuple `(category_label, value, detail)`.
    /// - `Color` - Swatch color.
    /// - `Point` - Tooltip anchor point.
    /// - `Rect` - Bounding hit rectangle.
    ///
    /// # Returns
    ///
    /// - `Self` - Rectangular hit record.
    #[must_use]
    pub(crate) fn rect_hit(
        id: (usize, String, usize),
        info: (String, f32, String),
        color: Color,
        anchor: Point,
        rect: Rect,
    ) -> Self {
        let (series_index, series_name, data_index): (usize, String, usize) = id;
        let (category_label, value, detail): (String, f32, String) = info;
        Self {
            series_index,
            series_name,
            data_index,
            category_label,
            value,
            detail,
            color,
            anchor,
            hit_radius: 12.0,
            hit_rect: Some(rect),
        }
    }

    /// Returns the optional rectangular hit bounds.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Datum hit record.
    ///
    /// # Returns
    ///
    /// - `Option<Rect>` - Hit box when rectangular.
    #[must_use]
    pub fn hit_rect_opt(&self) -> Option<Rect> {
        let Self { hit_rect, .. } = *self;
        hit_rect
    }

    /// Tests whether `pt` falls inside this datum's hit box or circular hit radius.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Datum hit record.
    /// - `Point` - Probe pointer coordinate.
    ///
    /// # Returns
    ///
    /// - `bool` - True when `pt` hits this datum.
    #[must_use]
    pub fn contains(&self, pt: Point) -> bool {
        if let Some(r) = self.hit_rect_opt() {
            return pt.get_x() >= r.get_x()
                && pt.get_x() <= r.get_x() + r.get_width()
                && pt.get_y() >= r.get_y()
                && pt.get_y() <= r.get_y() + r.get_height();
        }
        let dx: f32 = pt.get_x() - self.get_anchor().get_x();
        let dy: f32 = pt.get_y() - self.get_anchor().get_y();
        (dx * dx + dy * dy) <= self.get_hit_radius() * self.get_hit_radius()
    }
}

/// Interactive state transitions for `EuvChartState`.
impl EuvChartState {
    /// Initializes chart interaction state from a declarative `EuvChartOption`.
    ///
    /// # Arguments
    ///
    /// - `&EuvChartOption` - Declarative chart option.
    ///
    /// # Returns
    ///
    /// - `Self` - Initialized state honoring initial series visibility and data-zoom range.
    #[must_use]
    pub fn from_option(option: &EuvChartOption) -> Self {
        let mut st: Self = Self::default();
        for s in option.get_series().iter() {
            if !s.get_visible() {
                st.get_mut_hidden_series().push(s.get_name().clone());
            }
        }
        if let Some(dz) = option.get_data_zoom().first() {
            st.set_zoom_start_pct(dz.get_start_pct().clamp(0.0, 99.0));
            st.set_zoom_end_pct(dz.get_end_pct().clamp(st.get_zoom_start_pct() + 1.0, 100.0));
        }
        st
    }

    /// Returns whether the series named `name` is currently visible.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Chart state.
    /// - `&str` - Series name.
    ///
    /// # Returns
    ///
    /// - `bool` - True when visible.
    #[must_use]
    pub fn is_series_visible(&self, name: &str) -> bool {
        !self
            .get_hidden_series()
            .iter()
            .any(|h: &String| h.as_str() == name)
    }

    /// Toggles visibility for `name` and returns its new visible boolean state.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable chart state.
    /// - `&str` - Series name to toggle.
    ///
    /// # Returns
    ///
    /// - `bool` - New visibility (`true` = now visible, `false` = now hidden).
    pub fn toggle_series(&mut self, name: &str) -> bool {
        if let Some(pos) = self
            .get_hidden_series()
            .iter()
            .position(|h: &String| h.as_str() == name)
        {
            let _: String = self.get_mut_hidden_series().remove(pos);
            true
        } else {
            self.get_mut_hidden_series().push(name.to_string());
            false
        }
    }

    /// Sets the active zoom percentage window `[start_pct, end_pct]` respecting `min_span_pct`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable chart state.
    /// - `f32` - Requested lower percentage.
    /// - `f32` - Requested upper percentage.
    /// - `f32` - Minimum allowed span percentage.
    pub fn set_zoom(&mut self, start_pct: f32, end_pct: f32, min_span_pct: f32) {
        let min_span: f32 = min_span_pct.clamp(2.0, 95.0);
        let lo: f32 = start_pct.clamp(0.0, 100.0 - min_span);
        let hi: f32 = end_pct.clamp(lo + min_span, 100.0);
        self.set_zoom_start_pct(lo);
        self.set_zoom_end_pct(hi);
    }

    /// Zooms the active window centered around normalized plot cursor `anchor_frac` in `[0.0, 1.0]`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable chart state.
    /// - `f32` - Wheel delta (negative zooms in, positive zooms out).
    /// - `f32` - Normalized horizontal cursor position in `[0.0, 1.0]`.
    /// - `f32` - Minimum allowed span percentage.
    ///
    /// # Returns
    ///
    /// - `bool` - True when the zoom window changed.
    pub fn zoom_by_wheel(&mut self, wheel_delta: f32, anchor_frac: f32, min_span_pct: f32) -> bool {
        if wheel_delta.abs() < 1e-4 {
            return false;
        }
        let cur_start: f32 = self.get_zoom_start_pct();
        let cur_end: f32 = self.get_zoom_end_pct();
        let cur_span: f32 = (cur_end - cur_start).max(1.0);
        let factor: f32 = if wheel_delta < 0.0 { 0.85 } else { 1.18 };
        let min_span: f32 = min_span_pct.clamp(2.0, 95.0);
        let next_span: f32 = (cur_span * factor).clamp(min_span, 100.0);
        let pivot: f32 = cur_start + cur_span * anchor_frac.clamp(0.0, 1.0);
        let mut next_start: f32 = pivot - next_span * anchor_frac.clamp(0.0, 1.0);
        let mut next_end: f32 = next_start + next_span;
        if next_start < 0.0 {
            next_end = (next_end - next_start).min(100.0);
            next_start = 0.0;
        }
        if next_end > 100.0 {
            next_start = (next_start - (next_end - 100.0)).max(0.0);
            next_end = 100.0;
        }
        let changed: bool =
            (next_start - cur_start).abs() > 0.05 || (next_end - cur_end).abs() > 0.05;
        self.set_zoom_start_pct(next_start);
        self.set_zoom_end_pct(next_end);
        changed
    }

    /// Pans the active zoomed window horizontally by `delta_pct` while preserving span length.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable chart state.
    /// - `f32` - Percentage shift along the axis.
    ///
    /// # Returns
    ///
    /// - `bool` - True when the zoom window moved.
    pub fn pan_by_pct(&mut self, delta_pct: f32) -> bool {
        let cur_start: f32 = self.get_zoom_start_pct();
        let cur_end: f32 = self.get_zoom_end_pct();
        let span: f32 = cur_end - cur_start;
        if span >= 99.9 {
            return false;
        }
        let next_start: f32 = (cur_start + delta_pct).clamp(0.0, 100.0 - span);
        let next_end: f32 = next_start + span;
        let changed: bool = (next_start - cur_start).abs() > 0.05;
        self.set_zoom_start_pct(next_start);
        self.set_zoom_end_pct(next_end);
        changed
    }

    /// Cycles the toolbox `MagicType` override (`Default -> Line -> Bar -> Stack -> Default`).
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable chart state.
    ///
    /// # Returns
    ///
    /// - `EuvChartMagicType` - Newly active MagicType mode.
    pub fn cycle_magic_type(&mut self) -> EuvChartMagicType {
        let next: EuvChartMagicType = match self.get_magic_type() {
            EuvChartMagicType::Default => EuvChartMagicType::Line,
            EuvChartMagicType::Line => EuvChartMagicType::Bar,
            EuvChartMagicType::Bar => EuvChartMagicType::Stack,
            EuvChartMagicType::Stack => EuvChartMagicType::Default,
        };
        self.set_magic_type(next);
        next
    }

    /// Toggles the tabular `DataView` panel open or closed.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable chart state.
    ///
    /// # Returns
    ///
    /// - `bool` - New open state.
    pub fn toggle_data_view(&mut self) -> bool {
        let next: bool = !self.get_data_view_open();
        self.set_data_view_open(next);
        next
    }

    /// Toggles radial pop-out selection for pie slice `idx`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable chart state.
    /// - `usize` - Clicked slice index.
    pub fn toggle_pie_slice(&mut self, idx: usize) {
        let next: Option<usize> = if self.selected_pie_slice_opt() == Some(idx) {
            None
        } else {
            Some(idx)
        };
        self.set_selected_pie_slice(next);
    }

    /// Restores legend visibility, zoom bounds, magic type, pie selection, and data-view back to `option` defaults.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable chart state.
    /// - `&EuvChartOption` - Declarative chart option defaults.
    pub fn restore(&mut self, option: &EuvChartOption) {
        *self = Self::from_option(option);
    }

    /// Returns the optional popped-out pie slice index.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Chart state.
    ///
    /// # Returns
    ///
    /// - `Option<usize>` - Active popped-out slice index.
    #[must_use]
    pub fn selected_pie_slice_opt(&self) -> Option<usize> {
        let Self {
            selected_pie_slice, ..
        } = *self;
        selected_pie_slice
    }

    /// Returns the optional active hover pointer coordinate.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Chart state.
    ///
    /// # Returns
    ///
    /// - `Option<Point>` - Pointer coordinate in logical canvas space.
    #[must_use]
    pub fn hover_pos_opt(&self) -> Option<Point> {
        let Self { hover_pos, .. } = *self;
        hover_pos
    }
}

/// Safe optional field accessor for `EuvChartRenderResult`.
impl EuvChartRenderResult {
    /// Returns the bottom interactive slider geometry when a `DataZoom::Slider` was drawn.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Chart render result.
    ///
    /// # Returns
    ///
    /// - `Option<EuvChartSliderRects>` - Slider hit geometry.
    #[must_use]
    pub fn slider_rects_opt(&self) -> Option<EuvChartSliderRects> {
        let Self { slider_rects, .. } = *self;
        slider_rects
    }
}
