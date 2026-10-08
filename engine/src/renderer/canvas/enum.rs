use super::*;

/// A single deferred draw operation recorded into a `DrawList`.
///
/// Commands carry the resolved style for the operation (fill/stroke color, line
/// width) so that replay can group consecutive same-style shapes into a single
/// path and skip redundant canvas state changes. Colors are stored as `Color`
/// and converted to CSS strings only at replay time.
#[derive(Clone, Debug, PartialEq)]
pub enum DrawCommand {
    /// Fills a rectangle. Carries the fill color.
    FillRect {
        /// The top-left position in world space.
        position: Vector2D,
        /// The width in pixels.
        width: f64,
        /// The height in pixels.
        height: f64,
        /// The fill color.
        color: Color,
    },
    /// Strokes the outline of a rectangle. Carries stroke color and line width.
    StrokeRect {
        /// The top-left position in world space.
        position: Vector2D,
        /// The width in pixels.
        width: f64,
        /// The height in pixels.
        height: f64,
        /// The stroke color.
        color: Color,
        /// The stroke line width in pixels.
        line_width: f64,
    },
    /// Fills a circle. Carries the fill color.
    FillCircle {
        /// The center in world space.
        center: Vector2D,
        /// The radius in pixels.
        radius: f64,
        /// The fill color.
        color: Color,
    },
    /// Strokes the outline of a circle. Carries stroke color and line width.
    StrokeCircle {
        /// The center in world space.
        center: Vector2D,
        /// The radius in pixels.
        radius: f64,
        /// The stroke color.
        color: Color,
        /// The stroke line width in pixels.
        line_width: f64,
    },
    /// Draws a line segment. Carries stroke color and line width.
    Line {
        /// The start point in world space.
        start: Vector2D,
        /// The end point in world space.
        end: Vector2D,
        /// The stroke color.
        color: Color,
        /// The stroke line width in pixels.
        line_width: f64,
    },
    /// Fills text at a position. Carries the fill color and font.
    FillText {
        /// The text to draw.
        text: String,
        /// The position in world space.
        position: Vector2D,
        /// The fill color.
        color: Color,
        /// The CSS font string.
        font: String,
    },
    /// Draws a transformed sprite sub-region (image, source rect, TRS transform).
    DrawSprite {
        /// The image to draw.
        image: HtmlImageElement,
        /// The source rectangle within the image.
        source: Rect,
        /// The world-space transform (position, rotation, scale). Scale signs flip.
        transform: Transform2D,
    },
    /// Draws an image sub-region at a destination rect (no rotation).
    DrawImageRect {
        /// The image to draw.
        image: HtmlImageElement,
        /// The source rectangle within the image.
        source: Rect,
        /// The destination top-left position in world space.
        dest_position: Vector2D,
        /// The destination width in pixels.
        dest_width: f64,
        /// The destination height in pixels.
        dest_height: f64,
    },
    /// Applies a global alpha to all subsequent commands until changed.
    SetGlobalAlpha {
        /// The alpha value in the range 0.0 to 1.0.
        alpha: f64,
    },
    /// Applies a blend mode to all subsequent commands until changed.
    SetBlendMode {
        /// The blend mode to apply.
        mode: BlendMode,
    },
    /// Draws a texture as a nine-slice, keeping the corners fixed while
    /// the edges and centre stretch to the destination rectangle.
    DrawNineSlice {
        /// The source texture to slice.
        image: HtmlImageElement,
        /// The border insets splitting the texture into nine regions.
        insets: NineSliceInsets,
        /// The top-left position in world space.
        dest_position: Vector2D,
        /// The destination width in pixels.
        dest_width: f64,
        /// The destination height in pixels.
        dest_height: f64,
    },
    /// Draws one named region of a sprite atlas texture.
    DrawAtlasRegion {
        /// The atlas texture holding the region.
        image: HtmlImageElement,
        /// The region within the atlas, in source pixels.
        source: Rect,
        /// The top-left position in world space.
        dest_position: Vector2D,
        /// The destination width in pixels.
        dest_width: f64,
        /// The destination height in pixels.
        dest_height: f64,
    },
}
