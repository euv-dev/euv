use super::*;

/// Single laid-out text line with its measured pixel width.
#[derive(Clone, Debug, Default, Getter, New, PartialEq)]
pub struct TextLine {
    /// Line text content.
    pub(crate) text: String,
    /// Measured logical width of this line in pixels.
    #[get(type(copy))]
    pub(crate) width: f32,
}

/// Complete text measurement result used identically by layout and rendering passes.
#[derive(Clone, Debug, Default, Getter, New, PartialEq)]
pub struct TextLayoutResult {
    /// Individual wrapped or truncated lines.
    pub(crate) lines: Vec<TextLine>,
    /// Total bounding box size in logical pixels.
    #[get(type(copy))]
    pub(crate) size: Size,
    /// Computed pixel height of each line.
    #[get(type(copy))]
    pub(crate) line_height_px: f32,
}

/// Deterministic zero-dependency text measurer guaranteeing identical wrapping across all 7 platforms.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DeterministicTextMeasurer;
