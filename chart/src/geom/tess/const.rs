/// The default maximum distance, in pixels, between a flattened curve and
/// the true curve it approximates.
///
/// Bézier segments are subdivided until the control polygon deviates from
/// the chord by less than this. A quarter-pixel tolerance is below the
/// threshold at which antialiasing can resolve the difference, so raising
/// it further would only cost fill rate.
pub(crate) const DEFAULT_FLATTEN_TOLERANCE: f64 = 0.25;

/// The default number of segments used to approximate a full circle when
/// building rounded shapes.
pub(crate) const DEFAULT_CIRCLE_SEGMENTS: usize = 32;

/// The largest ratio between the miter length and the stroke width before
/// the join is beveled instead of mitred.
///
/// The SVG default is 4: a miter longer than four times the stroke width
/// is visually a spike, so the join is cut flat.
pub(crate) const MITER_LIMIT: f32 = 4.0;
