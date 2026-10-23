/// The maximum number of segments a single elliptical arc is split into.
///
/// One cubic Bézier covers at most 90° of an ellipse with an error under
/// 0.03% of the radius, so a full circle needs four segments. The cap
/// exists only to bound the work a pathological arc can request.
pub(crate) const MAX_ARC_SEGMENTS: usize = 64;

/// The magic constant that converts a circular arc into a cubic Bézier.
///
/// `4/3 · tan(θ/4)`, where `θ` is the sweep of the arc.
pub(crate) const ARC_BEZIER_KAPPA: f64 = 0.5522847498307933;

/// The number of commands a path reserves room for when it is built.
///
/// A rectangle — the shape of every bar of a bar chart — is five
/// commands long, so a path that reserves this much never has to grow
/// its buffer.
pub(crate) const PATH_COMMAND_CAPACITY: usize = 8;
