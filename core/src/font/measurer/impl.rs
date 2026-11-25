use super::*;

/// Deterministic cross-platform `TextMeasurer` implementation.
impl TextMeasurer for DeterministicTextMeasurer {
    /// Measures and lays out `text` identically on every operating system and rendering backend.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Measurer instance.
    /// - `&str` - Input UTF-8 string.
    /// - `TextStyle` - Active typography style.
    /// - `Option<f32>` - Optional maximum available width.
    ///
    /// # Returns
    ///
    /// - `TextLayoutResult` - Measured lines and bounding box.
    fn measure(&self, text: &str, style: TextStyle, max_width: Option<f32>) -> TextLayoutResult {
        let line_height_px: f32 = (style.get_font_size() * style.get_line_height()).max(1.0);
        if text.is_empty() {
            return TextLayoutResult::new(
                Vec::new(),
                Size::new(0.0, line_height_px),
                line_height_px,
            );
        }
        let mut lines: Vec<TextLine> = Vec::new();
        let wrap_limit: Option<f32> = match style.get_overflow() {
            TextOverflow::Wrap => max_width.filter(|w: &f32| *w > 0.0),
            TextOverflow::Clip | TextOverflow::Ellipsis => None,
        };
        for raw_line in text.split('\n') {
            if let Some(limit) = wrap_limit {
                let mut current: String = String::new();
                let mut current_w: f32 = 0.0;
                for ch in raw_line.chars() {
                    let ch_w: f32 = measure_char_advance(ch, style);
                    if !current.is_empty() && current_w + ch_w > limit {
                        lines.push(TextLine::new(take(&mut current), current_w));
                        current_w = 0.0;
                    }
                    current.push(ch);
                    current_w += ch_w;
                }
                lines.push(TextLine::new(current, current_w));
            } else {
                let mut width: f32 = 0.0;
                for ch in raw_line.chars() {
                    width += measure_char_advance(ch, style);
                }
                if style.get_overflow() == TextOverflow::Ellipsis
                    && let Some(limit) = max_width
                    && width > limit
                    && limit > 0.0
                {
                    let ellipsis_w: f32 = measure_char_advance('.', style) * 3.0;
                    let target_w: f32 = (limit - ellipsis_w).max(0.0);
                    let mut truncated: String = String::new();
                    let mut acc: f32 = 0.0;
                    for ch in raw_line.chars() {
                        let ch_w: f32 = measure_char_advance(ch, style);
                        if acc + ch_w > target_w {
                            break;
                        }
                        truncated.push(ch);
                        acc += ch_w;
                    }
                    truncated.push('.');
                    truncated.push('.');
                    truncated.push('.');
                    lines.push(TextLine::new(truncated, acc + ellipsis_w));
                } else {
                    lines.push(TextLine::new(raw_line.to_string(), width));
                }
            }
        }
        let max_line_w: f32 = lines.iter().fold(0.0_f32, |acc: f32, line: &TextLine| {
            acc.max(line.get_width())
        });
        let total_h: f32 = (lines.len().max(1) as f32) * line_height_px;
        TextLayoutResult::new(lines, Size::new(max_line_w, total_h), line_height_px)
    }
}
