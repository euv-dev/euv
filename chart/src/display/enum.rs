use super::*;

/// A single drawing instruction in the display list.
///
/// The display list is the boundary between the chart model and the
/// rendering backends: the model produces it, and every backend knows how
/// to replay it. Nothing above this layer mentions WebGPU, WebGL, or
/// Canvas2D, which is what lets a single API serve all three.
#[derive(Clone, Debug, Default)]
pub enum DrawCmd {
    /// Fills a path.
    Fill {
        /// The path to fill.
        path: Path,
        /// The fill paint.
        paint: Fill,
        /// The overall opacity applied to the fill, from `0.0` to `1.0`.
        opacity: f64,
    },
    /// Strokes a path.
    Stroke {
        /// The path to stroke.
        path: Path,
        /// The stroke paint.
        paint: Stroke,
        /// The geometry of the stroke.
        style: LineStyle,
        /// The overall opacity applied to the stroke, from `0.0` to `1.0`.
        opacity: f64,
    },
    /// Draws a text run.
    Text {
        /// The text to draw.
        text: String,
        /// The horizontal anchor of the text, in canvas pixels.
        x: f64,
        /// The vertical anchor of the text, in canvas pixels.
        y: f64,
        /// The style of the text.
        style: TextStyle,
    },
    /// Draws an image.
    Image {
        /// The image element to draw.
        image: HtmlImageElement,
        /// The horizontal position of the image, in canvas pixels.
        x: f64,
        /// The vertical position of the image, in canvas pixels.
        y: f64,
        /// The width of the image, in canvas pixels.
        width: f64,
        /// The height of the image, in canvas pixels.
        height: f64,
    },
    /// Intersects the clip region with a path.
    Clip {
        /// The path to clip against.
        path: Path,
    },
    /// Restores the clip region that was in effect before the matching
    /// [`DrawCmd::Clip`].
    #[default]
    PopClip,
    /// Draws a filled triangle strip, the shape of an area chart.
    ///
    /// The strip is a first-class instruction rather than a path so the
    /// GPU backends can emit it without running the tessellator: a strip
    /// is already triangles, and the area series is the hottest path in
    /// the whole crate.
    Strip {
        /// The vertices along the top edge of the strip.
        points: Vec<Point>,
        /// The vertical coordinate the strip is closed against.
        baseline: f32,
        /// The fill paint.
        paint: Fill,
        /// The overall opacity applied to the strip, from `0.0` to `1.0`.
        opacity: f64,
    },
}

/// The kind of a display command, used to group commands that can share
/// one batch.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum DrawKind {
    /// A filled path.
    #[default]
    Fill,
    /// A stroked path.
    Stroke,
    /// A text run.
    Text,
    /// An image.
    Image,
    /// A clip instruction.
    Clip,
    /// A clip-restore instruction.
    PopClip,
    /// A filled triangle strip.
    Strip,
}
