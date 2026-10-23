use super::*;

/// A colour with 8 bits per channel.
///
/// The chart pipeline packs colours into a single `u32` (`0xRRGGBBAA`)
/// before they reach the GPU, so the type keeps the four channels as
/// separate bytes and does the packing once, at upload time.
#[derive(Clone, Copy, Data, Debug, Default, Eq, Hash, PartialEq)]
pub struct Color {
    /// The red channel.
    #[get(type(copy))]
    pub red: u8,
    /// The green channel.
    #[get(type(copy))]
    pub green: u8,
    /// The blue channel.
    #[get(type(copy))]
    pub blue: u8,
    /// The alpha channel, where `255` is fully opaque.
    #[get(type(copy))]
    pub alpha: u8,
}

/// A colour stop inside a gradient.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GradientStop {
    /// The position of the stop along the gradient, from `0.0` to `1.0`.
    pub offset: f64,
    /// The colour at the stop.
    pub color: Color,
}

/// A linear colour gradient.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct LinearGradient {
    /// The start point, in chart coordinates.
    pub start: (f64, f64),
    /// The end point, in chart coordinates.
    pub end: (f64, f64),
    /// The colour stops, ordered by offset.
    pub stops: Vec<GradientStop>,
}

/// A radial colour gradient.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct RadialGradient {
    /// The centre of the gradient, in chart coordinates.
    pub center: (f64, f64),
    /// The radius of the gradient, in chart pixels.
    #[get(type(copy))]
    pub radius: f64,
    /// The colour stops, ordered by offset.
    pub stops: Vec<GradientStop>,
}

/// A drop shadow applied to a filled or stroked shape.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Shadow {
    /// The horizontal offset of the shadow.
    pub offset_x: f64,
    /// The vertical offset of the shadow.
    pub offset_y: f64,
    /// The blur radius of the shadow.
    pub blur: f64,
    /// The colour of the shadow.
    pub color: Color,
}

/// A text style, covering the font, the alignment, and the wrapping
/// behaviour of a label.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct TextStyle {
    /// The font family list, as a CSS `font-family` value.
    pub family: String,
    /// The font size, in pixels.
    #[get(type(copy))]
    pub size: f64,
    /// The font weight, as a CSS `font-weight` value.
    pub weight: String,
    /// The font style, as a CSS `font-style` value.
    pub style: String,
    /// The horizontal alignment of the text relative to its anchor point.
    pub align: TextAlign,
    /// The vertical alignment of the text relative to its anchor point.
    pub vertical_align: TextVerticalAlign,
    /// The line height, as a multiple of the font size.
    #[get(type(copy))]
    pub line_height: f64,
    /// Whether the text wraps when it exceeds its box.
    #[get(type(copy))]
    pub wrap: bool,
    /// What happens to text that does not fit.
    pub overflow: TextOverflow,
    /// The colour of the text.
    pub color: Color,
    /// The colour painted behind the text, if any.
    #[get(skip)]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub background: Option<Color>,
    /// The padding between the text and its background, in pixels.
    pub padding: (f64, f64, f64, f64),
    /// The border width drawn around the text, in pixels.
    #[get(type(copy))]
    pub border_width: f64,
    /// The colour of the border drawn around the text.
    #[get(skip)]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub border_color: Option<Color>,
    /// The radius of the background's rounded corners, in pixels.
    #[get(type(copy))]
    pub border_radius: f64,
    /// The rotation of the text, in degrees, around its anchor point.
    #[get(type(copy))]
    pub rotate: f64,
    /// The stroke width of the text outline.
    #[get(type(copy))]
    pub stroke_width: f64,
    /// The colour of the text outline.
    #[get(skip)]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub stroke_color: Option<Color>,
    /// The per-run style overrides, keyed by the name used in the text's
    /// `{name|…}` markup.
    pub rich: Vec<(String, RichTextStyle)>,
}

/// The style of one run inside a rich-text label.
#[derive(Clone, Debug, PartialEq)]
pub struct RichTextStyle {
    /// The font family list, as a CSS `font-family` value.
    pub family: Option<String>,
    /// The font size, in pixels.
    pub size: Option<f64>,
    /// The font weight, as a CSS `font-weight` value.
    pub weight: Option<String>,
    /// The font style, as a CSS `font-style` value.
    pub style: Option<String>,
    /// The colour of the run.
    pub color: Option<Color>,
    /// The colour painted behind the run.
    pub background: Option<Color>,
    /// The padding between the run and its background.
    pub padding: Option<(f64, f64, f64, f64)>,
    /// The line height of the run, as a multiple of the font size.
    pub line_height: Option<f64>,
}

/// The line style of a shape: its width, its caps, its joins, and its
/// dash pattern.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct LineStyle {
    /// The line width, in pixels.
    #[get(type(copy))]
    pub width: f64,
    /// How the ends of an open stroke are drawn.
    pub cap: LineCap,
    /// How the corners between segments are drawn.
    pub join: LineJoin,
    /// The dash pattern applied along the stroke.
    pub dash: DashPattern,
    /// The opacity of the stroke, from `0.0` to `1.0`.
    #[get(type(copy))]
    pub opacity: f64,
    /// The shadow cast by the stroke.
    #[get(skip)]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub shadow: Option<Shadow>,
    /// The colour of the stroke.
    pub color: Color,
}

/// A fill and a stroke, as attached to a shape in a series.
#[derive(Clone, Debug, PartialEq)]
pub struct ShapeStyle {
    /// The fill of the shape.
    pub fill: Fill,
    /// The stroke of the shape.
    pub stroke: Stroke,
    /// The stroke's geometry parameters.
    pub line_style: LineStyle,
    /// The shadow cast by the shape.
    pub shadow: Option<Shadow>,
    /// The opacity of the whole shape, from `0.0` to `1.0`.
    pub opacity: f64,
}
