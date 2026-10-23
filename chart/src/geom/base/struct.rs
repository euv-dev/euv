use super::*;

/// A two-dimensional affine transform stored as the six meaningful entries
/// of a 3×3 matrix.
///
/// The matrix is applied to a point `(x, y)` as:
///
/// ```text
/// x' = a·x + c·y + e
/// y' = b·x + d·y + f
/// ```
///
/// This is the same layout SVG and Canvas2D use, so a transform built here
/// can be handed to a Canvas2D context verbatim.
#[derive(Clone, Copy, Debug, PartialEq, lombok_macros::Data)]
pub struct Transform2D {
    /// The horizontal scale / shear term applied to `x`.
    #[get(type(copy))]
    pub a: f64,
    /// The vertical shear term applied to `x`.
    #[get(type(copy))]
    pub b: f64,
    /// The horizontal shear term applied to `y`.
    #[get(type(copy))]
    pub c: f64,
    /// The vertical scale / shear term applied to `y`.
    #[get(type(copy))]
    pub d: f64,
    /// The horizontal translation.
    #[get(type(copy))]
    pub e: f64,
    /// The vertical translation.
    #[get(type(copy))]
    pub f: f64,
}

/// An axis-aligned bounding box, stored as its minimum corner plus its
/// extent.
///
/// The `min` / `max` form (rather than `x` / `y` / `width` / `height`)
/// keeps the empty-box case representable: a box that has not been
/// extended yet is `min > max` on every axis, which is the standard
/// sentinel for "no geometry yet".
#[derive(Clone, Copy, Debug, PartialEq, lombok_macros::Data)]
pub struct BBox {
    /// The smallest `x` reached by the geometry.
    #[get(type(copy))]
    pub min_x: f64,
    /// The smallest `y` reached by the geometry.
    #[get(type(copy))]
    pub min_y: f64,
    /// The largest `x` reached by the geometry.
    #[get(type(copy))]
    pub max_x: f64,
    /// The largest `y` reached by the geometry.
    #[get(type(copy))]
    pub max_y: f64,
}

/// A point in path space.
///
/// Paths are stored in 32-bit floats: they are consumed by the GPU
/// backends, which upload them verbatim, and pixel-space coordinates never
/// need more than 24 bits of mantissa. Layout arithmetic happens in
/// `f64` and is narrowed once, when a path is built.
#[derive(Clone, Copy, Debug, Default, PartialEq, lombok_macros::Data)]
pub struct Point {
    /// The horizontal coordinate.
    #[get(type(copy))]
    pub x: f32,
    /// The vertical coordinate.
    #[get(type(copy))]
    pub y: f32,
}

/// A closed polygon: a flat list of points plus an explicit count.
///
/// The flat representation keeps the hot triangulation loop free of
/// indirection: the inner scan reads `points[i]` directly instead of
/// chasing a per-vertex pointer.
#[derive(Clone, Debug, Default, PartialEq, lombok_macros::Data)]
pub struct Polygon {
    /// The vertices, in order.
    pub points: Vec<Point>,
}

/// A sequence of drawing commands describing one or more sub-paths.
///
/// A path is the unit of geometry in the chart renderer: axes, series
/// polylines, pie slices, radar rings, and text decorations are all
/// expressed as paths. Paths are immutable once built — the builder
/// produces the final command buffer, and the tessellator consumes it
/// without mutating it.
#[derive(Clone, Debug, Default, PartialEq, lombok_macros::Data)]
pub struct Path {
    /// The command buffer.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) commands: Vec<PathCommand>,
    /// The rule used to decide which parts of the path are inside it.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) fill_rule: FillRule,
}

/// A fluent builder for [`Path`].
///
/// Every method returns `Self` so a path can be assembled in one
/// expression without a mutable borrow in the middle of a call chain.
#[derive(Clone, Debug, lombok_macros::Data)]
pub struct PathBuilder {
    /// The commands of the path under construction.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) commands: Vec<PathCommand>,
    /// The rule the finished path is filled with.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) fill_rule: FillRule,
    /// The point the path is currently at.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) current: Point,
    /// The point the current sub-path started at.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) start: Point,
}

/// An ellipse: the shape an arc is cut out of.
///
/// The type stays inside the crate: it gathers the four numbers an arc
/// needs, so that the function that cuts the arc does not take eight
/// of them.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Ellipse {
    /// The horizontal position of the centre.
    pub(crate) center_x: f64,
    /// The vertical position of the centre.
    pub(crate) center_y: f64,
    /// The radius along the horizontal axis.
    pub(crate) radius_x: f64,
    /// The radius along the vertical axis.
    pub(crate) radius_y: f64,
}

/// The geometric parameters of a stroke.
///
/// The colour lives in the paint layer; this struct carries only what the
/// tessellator needs to turn a path into stroke geometry.
#[derive(Clone, Debug, PartialEq, lombok_macros::Data)]
pub struct StrokeStyle {
    /// The line width, in pixels.
    #[get(type(copy))]
    pub width: f32,
    /// How the ends of an open stroke are drawn.
    #[get(type(copy))]
    pub cap: LineCap,
    /// How the corners between segments are drawn.
    #[get(type(copy))]
    pub join: LineJoin,
    /// The dash pattern applied along the stroke.
    pub dash: DashPattern,
}

/// A dash pattern applied to a stroke.
///
/// `offset` shifts the pattern along the path; a negative offset moves the
/// pattern backwards.
#[derive(Clone, Debug, Default, PartialEq, lombok_macros::Data)]
pub struct DashPattern {
    /// The alternating on/off lengths, in pixels.
    ///
    /// An empty slice means "solid". An odd-length array is repeated twice
    /// to make the on/off alternation well-defined.
    pub pattern: Vec<f32>,
    /// The distance into the pattern at which the dash starts.
    #[get(type(copy))]
    pub offset: f32,
}
