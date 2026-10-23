/// A single drawing command inside a [`Path`].
///
/// The command set is deliberately minimal and flat: every curve is
/// expressed in its own primitive form, and flattening to line segments
/// happens later, in [`tess`]. Keeping curves symbolic in the command
/// buffer means a stroke can be expanded with a different flattening
/// tolerance than a fill without rebuilding the path.
///
/// Every variant carries data, and `#[derive(Default)]` needs one
/// without any, so this enum has no default.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PathCommand {
    /// Starts a new sub-path at the given point.
    ///
    /// A `MoveTo` implicitly closes any sub-path that is still open, which
    /// matches the SVG filling rule: every sub-path is treated as closed
    /// when it is filled, whether or not it was explicitly closed.
    MoveTo(f32, f32),
    /// Appends a straight line segment to the current point.
    LineTo(f32, f32),
    /// Appends a quadratic Bézier segment.
    ///
    /// The first pair is the control point, the second pair is the
    /// endpoint.
    QuadTo(f32, f32, f32, f32),
    /// Appends a cubic Bézier segment.
    ///
    /// The first two pairs are the control points, the third pair is the
    /// endpoint.
    CubicTo(f32, f32, f32, f32, f32, f32),
    /// Closes the current sub-path with a straight line back to its start.
    Close,
}

/// How the interior of a self-intersecting path is determined.
///
/// Only the fill operation consults this; strokes are unaffected.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum FillRule {
    /// Counts winding direction: a point inside an odd number of
    /// same-direction contours is filled.
    ///
    /// This is the default and matches what chart geometry expects — a
    /// donut hole winds the opposite way from the outer ring, so it is
    /// punched out.
    #[default]
    NonZero,
    /// Counts crossings: a point inside an odd number of contours is
    /// filled, regardless of direction.
    EvenOdd,
}

/// How the ends of an open stroked line are drawn.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum LineCap {
    /// The line stops exactly at the endpoint.
    #[default]
    Butt,
    /// The line is extended by half the stroke width, forming a square cap.
    Square,
    /// The line is extended by half the stroke width, forming a round cap.
    Round,
}

/// How the corners between two stroked segments are drawn.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum LineJoin {
    /// The corner is a single point; the outer edges are mitred.
    #[default]
    Miter,
    /// The corner is rounded.
    Round,
    /// The outer edges are bevelled off.
    Bevel,
}
