use super::*;

/// Pluggable text measurement interface enabling deterministic cross-platform text layout.
pub trait TextMeasurer {
    /// Measures and wraps `text` under `style` within an optional `max_width` constraint.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Text measurer reference.
    /// - `&str` - UTF-8 text to lay out.
    /// - `TextStyle` - Typography style parameters.
    /// - `Option<f32>` - Optional maximum line width in logical pixels.
    ///
    /// # Returns
    ///
    /// - `TextLayoutResult` - Wrapped lines and aggregate bounding box.
    fn measure(&self, text: &str, style: TextStyle, max_width: Option<f32>) -> TextLayoutResult;

    /// Measures only the aggregate bounding box of `text` under `style` within an optional `max_width` constraint.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Text measurer reference.
    /// - `&str` - UTF-8 text to measure.
    /// - `TextStyle` - Typography style parameters.
    /// - `Option<f32>` - Optional maximum line width in logical pixels.
    ///
    /// # Returns
    ///
    /// - `Size` - Aggregate bounding box without requiring callers to materialize wrapped lines.
    fn measure_size(&self, text: &str, style: TextStyle, max_width: Option<f32>) -> Size {
        self.measure(text, style, max_width).get_size()
    }
}
