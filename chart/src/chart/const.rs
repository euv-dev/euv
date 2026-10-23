/// The radius, in pixels, within which a pointer picks up a data point.
///
/// The radius is generous on purpose: a data point is often a line
/// rather than a dot, and a click that lands on the line should count.
pub(crate) const HIT_RADIUS: f64 = 8.0;
