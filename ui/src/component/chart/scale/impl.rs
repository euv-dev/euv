use super::*;

/// Projection and inverse-mapping methods for `EuvChartAxisScale`.
impl EuvChartAxisScale {
    /// Normalizes a domain value into `[0.0, 1.0]` respecting `Category`, `Value`, `Time`, `Log`, and `inverse`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Computed axis scale.
    /// - `f32` - Domain value or category index.
    ///
    /// # Returns
    ///
    /// - `f32` - Normalized coordinate in `[0.0, 1.0]` (unclamped when outside range).
    #[must_use]
    pub fn normalize(&self, value: f32) -> f32 {
        let min_v: f32 = self.get_min();
        let max_v: f32 = self.get_max();
        let raw_t: f32 = match self.get_axis_type() {
            EuvChartAxisType::Log => {
                let lo: f32 = min_v.max(SCALE_LOG_MIN_POSITIVE).log10();
                let hi: f32 = max_v.max(SCALE_LOG_MIN_POSITIVE * 10.0).log10();
                let span: f32 = (hi - lo).max(1e-6);
                let v_log: f32 = value.max(SCALE_LOG_MIN_POSITIVE).log10();
                (v_log - lo) / span
            }
            EuvChartAxisType::Category => {
                let count: usize = self.get_categories().len();
                if count == 0 {
                    0.5
                } else {
                    (value + 0.5) / (count as f32)
                }
            }
            EuvChartAxisType::Value | EuvChartAxisType::Time => {
                let span: f32 = max_v - min_v;
                if span.abs() < 1e-6 {
                    0.5
                } else {
                    (value - min_v) / span
                }
            }
        };
        if self.get_inverse() {
            1.0 - raw_t
        } else {
            raw_t
        }
    }

    /// Projects a domain value onto a horizontal pixel coordinate inside `plot_rect`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Horizontal axis scale.
    /// - `f32` - Domain value or category index.
    /// - `Rect` - Target plot rectangle.
    ///
    /// # Returns
    ///
    /// - `f32` - Logical X pixel coordinate.
    #[must_use]
    pub fn map_x(&self, value: f32, plot_rect: Rect) -> f32 {
        plot_rect.get_x() + self.normalize(value) * plot_rect.get_width()
    }

    /// Projects a domain value onto a vertical pixel coordinate inside `plot_rect` (bottom-up for numeric axes, top-down for category bars).
    ///
    /// # Arguments
    ///
    /// - `&Self` - Vertical axis scale.
    /// - `f32` - Domain value or category index.
    /// - `Rect` - Target plot rectangle.
    ///
    /// # Returns
    ///
    /// - `f32` - Logical Y pixel coordinate.
    #[must_use]
    pub fn map_y(&self, value: f32, plot_rect: Rect) -> f32 {
        let t: f32 = self.normalize(value);
        match self.get_axis_type() {
            EuvChartAxisType::Category => plot_rect.get_y() + t * plot_rect.get_height(),
            EuvChartAxisType::Value | EuvChartAxisType::Time | EuvChartAxisType::Log => {
                plot_rect.get_y() + (1.0 - t) * plot_rect.get_height()
            }
        }
    }

    /// Returns the baseline pixel Y corresponding to `0.0` (clamped inside `plot_rect`).
    ///
    /// # Arguments
    ///
    /// - `&Self` - Vertical numeric axis scale.
    /// - `Rect` - Target plot rectangle.
    ///
    /// # Returns
    ///
    /// - `f32` - Baseline Y coordinate in logical pixels.
    #[must_use]
    pub fn baseline_y(&self, plot_rect: Rect) -> f32 {
        let zero_val: f32 = if self.get_axis_type() == EuvChartAxisType::Log {
            self.get_min().max(SCALE_LOG_MIN_POSITIVE)
        } else {
            0.0_f32.clamp(self.get_min(), self.get_max())
        };
        self.map_y(zero_val, plot_rect).clamp(
            plot_rect.get_y(),
            plot_rect.get_y() + plot_rect.get_height(),
        )
    }

    /// Returns the baseline pixel X corresponding to `0.0` (clamped inside `plot_rect`) for horizontal bar charts.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Horizontal numeric axis scale.
    /// - `Rect` - Target plot rectangle.
    ///
    /// # Returns
    ///
    /// - `f32` - Baseline X coordinate in logical pixels.
    #[must_use]
    pub fn baseline_x(&self, plot_rect: Rect) -> f32 {
        let zero_val: f32 = if self.get_axis_type() == EuvChartAxisType::Log {
            self.get_min().max(SCALE_LOG_MIN_POSITIVE)
        } else {
            0.0_f32.clamp(self.get_min(), self.get_max())
        };
        self.map_x(zero_val, plot_rect)
            .clamp(plot_rect.get_x(), plot_rect.get_x() + plot_rect.get_width())
    }
}

/// Color interpolation methods for `EuvChartVisualEvaluator`.
impl EuvChartVisualEvaluator {
    /// Resolves a numeric `value` into its mapped `Color` under `Continuous` or `Piecewise` rules.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Visual-map evaluator.
    /// - `f32` - Scalar data value.
    ///
    /// # Returns
    ///
    /// - `Color` - Interpolated or bucketed RGBA color.
    #[must_use]
    pub fn map_color(&self, value: f32) -> Color {
        if self.get_kind() == EuvChartVisualMapKind::Piecewise {
            for piece in self.get_pieces().iter() {
                let ge_min: bool = piece.min_opt().is_none_or(|lo: f32| value >= lo);
                let le_max: bool = piece.max_opt().is_none_or(|hi: f32| value <= hi);
                if ge_min && le_max {
                    return piece.get_color();
                }
            }
            if let Some(last) = self.get_pieces().last() {
                return last.get_color();
            }
        }
        let stops: &[Color] = self.get_stops().as_slice();
        if stops.is_empty() {
            return Color::hex(ECHART_PALETTE_LIGHT[0]);
        }
        if stops.len() == 1 {
            return stops[0];
        }
        let span: f32 = (self.get_max() - self.get_min()).max(1e-6);
        let t: f32 = ((value - self.get_min()) / span).clamp(0.0, 1.0);
        let scaled: f32 = t * ((stops.len() - 1) as f32);
        let idx: usize = (scaled.floor() as usize).min(stops.len() - 2);
        let local_t: f32 = (scaled - (idx as f32)).clamp(0.0, 1.0);
        stops[idx].lerp(stops[idx + 1], local_t)
    }
}
