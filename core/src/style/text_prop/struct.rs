use super::*;

/// Typography properties controlling deterministic text layout and glyph rendering.
#[derive(Clone, Copy, Data, Debug, PartialEq)]
pub struct TextStyle {
    /// Semantic font family.
    #[get(type(copy))]
    pub(crate) family: FontFamily,
    /// Font size in logical pixels.
    #[get(type(copy))]
    pub(crate) font_size: f32,
    /// Font weight.
    #[get(type(copy))]
    pub(crate) weight: FontWeight,
    /// Line height multiplier relative to `font_size`.
    #[get(type(copy))]
    pub(crate) line_height: f32,
    /// Extra tracking between characters in logical pixels.
    #[get(type(copy))]
    pub(crate) letter_spacing: f32,
    /// Horizontal alignment.
    #[get(type(copy))]
    pub(crate) align: TextAlign,
    /// Line wrapping / truncation behavior.
    #[get(type(copy))]
    pub(crate) overflow: TextOverflow,
    /// Foreground glyph color.
    #[get(type(copy))]
    pub(crate) color: Color,
    /// Whether text is underlined.
    #[get(type(copy))]
    pub(crate) underline: bool,
    /// Whether text has strikethrough decoration.
    #[get(type(copy))]
    pub(crate) strikethrough: bool,
}
