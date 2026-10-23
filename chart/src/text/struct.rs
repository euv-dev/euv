use super::*;

/// A cache of rasterised glyphs, packed into a single texture.
///
/// The GPU backends cannot draw text directly: WebGPU and WebGL2 have no
/// text API. The atlas solves that by rasterising each glyph once into
/// an offscreen canvas and then drawing the glyphs as textured quads. The
/// canvas is handed to the GPU as a texture source, so no pixel read-back
/// is ever needed.
///
/// The Canvas2D backend does not use the atlas at all — it has a real
/// text API — but it shares the measurer, so the three backends lay text
/// out identically.
#[derive(Clone, Data, Debug)]
pub struct GlyphAtlas {
    /// The canvas the glyphs are rasterised into.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) canvas: HtmlCanvasElement,
    /// The 2D context used to rasterise glyphs.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) context: CanvasRenderingContext2d,
    /// The rasterised glyphs, keyed by character and font.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) entries: HashMap<GlyphKey, GlyphEntry>,
    /// The horizontal position where the next glyph is placed.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) cursor_x: f32,
    /// The vertical position of the current shelf.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) cursor_y: f32,
    /// The height of the current shelf.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) shelf_height: f32,
    /// The width of the atlas, in pixels.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) width: u32,
    /// The height of the atlas, in pixels.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) height: u32,
    /// Whether the atlas changed since the last upload.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) dirty: bool,
}

/// One positioned glyph, ready to be drawn as a textured quad.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GlyphQuad {
    /// The left edge of the glyph's bitmap, in canvas pixels.
    pub x: f32,
    /// The top edge of the glyph's bitmap, in canvas pixels.
    pub y: f32,
    /// The width of the glyph's bitmap, in canvas pixels.
    pub width: f32,
    /// The height of the glyph's bitmap, in canvas pixels.
    pub height: f32,
    /// The horizontal texture coordinate of the glyph's top-left corner.
    pub u: f32,
    /// The vertical texture coordinate of the glyph's top-left corner.
    pub v: f32,
    /// The width of the glyph's bitmap in texture coordinates.
    pub u_size: f32,
    /// The height of the glyph's bitmap in texture coordinates.
    pub v_size: f32,
    /// The packed colour of the glyph, as `0xRRGGBBAA`.
    ///
    /// A rich-text run carries its own colour; a plain run leaves the
    /// colour at zero, which means "use the colour of the label".
    pub color: u32,
}

/// One line of a shaped text run.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TextLine {
    /// The text of the line.
    pub text: String,
    /// The horizontal position of the line's left edge, in canvas pixels.
    pub left: f64,
    /// The vertical position of the line's baseline, in canvas pixels.
    pub baseline: f64,
    /// The advance width of the line.
    pub width: f64,
}

/// A shaped run of text: the glyph quads that draw it, plus the metrics
/// of the run as a whole.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TextRun {
    /// The glyph quads, in reading order.
    pub quads: Vec<GlyphQuad>,
    /// The lines of the run, with the position of each baseline.
    pub lines: Vec<TextLine>,
    /// The advance width of the whole run.
    pub width: f64,
    /// The height of the whole run.
    pub height: f64,
    /// The distance from the run's top to the first baseline.
    pub ascent: f64,
    /// The distance from the last baseline down to the bottom of the
    /// lowest glyph.
    pub descent: f64,
    /// The distance between two consecutive baselines.
    pub line_height: f64,
    /// The number of lines in the run.
    pub line_count: usize,
}

/// Combines the measurer and the atlas into a text shaper.
///
/// The shaper is the single place where text is turned into positioned
/// glyphs, so the Canvas2D backend and the GPU backends lay text out
/// identically — the same label lands on the same pixel no matter which
/// backend is in use.
#[derive(Clone, Data, Debug)]
pub struct TextShaper {
    /// The measurer used to size the text.
    pub measurer: TextMeasurer,
    /// The atlas the glyphs are rasterised into.
    pub atlas: GlyphAtlas,
}

/// The measured metrics of a text run.
///
/// All values are in CSS pixels, for the font the run was measured with.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TextMetrics {
    /// The advance width of the run.
    pub width: f64,
    /// The height of the run's bounding box, from the top of the ascent
    /// to the bottom of the descent.
    pub height: f64,
    /// The distance from the baseline up to the top of the tallest
    /// glyph.
    pub ascent: f64,
    /// The distance from the baseline down to the bottom of the lowest
    /// glyph.
    pub descent: f64,
    /// The distance between two consecutive baselines.
    pub line_height: f64,
}

/// A glyph rendered into the atlas.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GlyphEntry {
    /// The horizontal texture coordinate of the glyph's left edge, from
    /// `0.0` to `1.0`.
    pub u: f32,
    /// The vertical texture coordinate of the glyph's top edge, from `0.0`
    /// to `1.0`.
    pub v: f32,
    /// The width of the glyph's bitmap, in atlas pixels.
    pub width: f32,
    /// The height of the glyph's bitmap, in atlas pixels.
    pub height: f32,
    /// The horizontal distance from the pen position to the left edge of
    /// the glyph's bitmap, in pixels.
    pub bearing_x: f32,
    /// The vertical distance from the baseline down to the top edge of
    /// the glyph's bitmap, in pixels.
    pub bearing_y: f32,
    /// The advance width of the glyph, in pixels.
    pub advance: f32,
}

/// The key that identifies one glyph in the atlas.
///
/// The font string is part of the key because the same character renders
/// differently in different fonts, and the atlas holds one rasterisation
/// per combination.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct GlyphKey {
    /// The character the glyph stands for.
    pub character: char,
    /// The CSS `font` shorthand the glyph was rasterised with.
    pub font: String,
}

/// Measures text with a 2D canvas context, caching the results.
///
/// Every label in a chart is measured at least once per layout, and
/// layout runs on every data change. Measuring is a cross-language call
/// into the browser, so the results are cached by text and font: a chart
/// with a thousand axis labels makes a thousand calls the first time and
/// none afterwards.
#[derive(Clone, Data)]
pub struct TextMeasurer {
    /// The offscreen context used for measuring.
    ///
    /// `None` when the host has no DOM; the measurer then falls back to
    /// an approximation, which is what lets the layout run on a server
    /// and in a native build.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) context: Option<CanvasRenderingContext2d>,
    /// The cache of already-measured runs.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) cache: Rc<RefCell<HashMap<(String, String), TextMetrics>>>,
}
