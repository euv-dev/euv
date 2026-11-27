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
    /// Whether text is rendered in italic style.
    #[get(type(copy))]
    pub(crate) italic: bool,
    /// Whether text is underlined.
    #[get(type(copy))]
    pub(crate) underline: bool,
    /// Whether text has strikethrough decoration.
    #[get(type(copy))]
    pub(crate) strikethrough: bool,
    /// Whether text can be interactively selected and copied by the user.
    #[get(type(copy))]
    pub(crate) selectable: bool,
    /// Whether `selectable` was explicitly configured on this node (enabling opt-out under selectable ancestors).
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) selectable_explicit: bool,
    /// Optional override fill color for text selection highlight rectangles.
    #[get(skip)]
    pub(crate) selection_color: Option<Color>,
}
