use super::*;

/// Backend-agnostic vector/UI drawing primitive emitted by `PaintPass` and consumed by all renderers.
#[derive(Clone, Debug, PartialEq)]
pub enum DrawCommand {
    /// Clears the target surface to `Color`.
    Clear(Color),
    /// Draws a rounded or sharp filled rectangle with optional border stroke.
    FillRoundedRect {
        /// Bounds in logical pixels.
        rect: Rect,
        /// Fill color.
        color: Color,
        /// Corner radii.
        radius: CornerRadius,
        /// Optional border stroke.
        border: Border,
    },
    /// Draws a two-stop linear gradient inside a rounded rectangle.
    LinearGradientRect {
        /// Bounds in logical pixels.
        rect: Rect,
        /// Gradient specification.
        gradient: LinearGradient,
        /// Corner radii.
        radius: CornerRadius,
    },
    /// Draws a soft drop shadow under a rounded rectangle.
    BoxShadow {
        /// Casting box bounds.
        rect: Rect,
        /// Corner radii.
        radius: CornerRadius,
        /// Shadow parameters.
        shadow: Shadow,
    },
    /// Draws a laid-out text run using the shared deterministic glyph atlas.
    TextRun {
        /// Bounding rectangle in logical pixels.
        rect: Rect,
        /// UTF-8 text string.
        text: String,
        /// Typography style.
        style: TextStyle,
    },
    /// Draws a blinking text-input caret aligned to exact rasterized glyph advances.
    TextCaret {
        /// Text layout bounding rectangle in logical pixels.
        text_rect: Rect,
        /// Input inner content clip rectangle in logical pixels.
        content_rect: Rect,
        /// Active input text preceding the caret.
        text: String,
        /// Typography style matching the input's `TextRun`.
        style: TextStyle,
        /// Caret height in logical pixels.
        caret_height: f32,
        /// Caret fill color.
        color: Color,
        /// Whether the input wraps across multiple lines.
        multiline: bool,
    },
    /// Blits a decoded RGBA8 image buffer into `rect`.
    Image {
        /// Destination bounds.
        rect: Rect,
        /// Image pixel buffer.
        buffer: ImageBuffer,
        /// Corner radii mask.
        radius: CornerRadius,
    },
    /// Draws a straight anti-aliased line segment (used by custom canvases and dividers).
    Line {
        /// Start point.
        from: Point,
        /// End point.
        to: Point,
        /// Line color.
        color: Color,
        /// Stroke width.
        width: f32,
    },
    /// Draws a filled anti-aliased circle (used by badges, avatars, spinners, and custom canvases).
    Circle {
        /// Center point.
        center: Point,
        /// Circle radius.
        radius: f32,
        /// Fill color.
        color: Color,
    },
    /// Draws a filled triangle primitive (used by 2D game meshes, spaceships, lasers, and polygons).
    Triangle {
        /// First vertex in logical pixels.
        p0: Point,
        /// Second vertex in logical pixels.
        p1: Point,
        /// Third vertex in logical pixels.
        p2: Point,
        /// Fill color.
        color: Color,
    },
    /// Blits a cropped sub-region of an `ImageBuffer` sprite sheet with optional tint color.
    SpriteRegion {
        /// Destination bounds in logical pixels.
        rect: Rect,
        /// Source sprite-sheet RGBA8 buffer.
        buffer: ImageBuffer,
        /// Source pixel sub-rectangle `(x, y, w, h)`.
        src_rect: Rect,
        /// Multiplicative tint color (`COLOR_WHITE` for untinted).
        tint: Color,
    },
    /// Pushes an axis-aligned clip rectangle onto the clip stack.
    PushClip(Rect),
    /// Pops the top clip rectangle from the clip stack.
    PopClip,
}
