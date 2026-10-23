use super::*;

/// Returns the Canvas2D name of a line cap.
///
/// # Arguments
///
/// - `LineCap` - A `LineCap` parameter.
///
/// # Returns
///
/// - `&'static str` - A `'static str` value.
pub(crate) fn line_cap_name(cap: LineCap) -> &'static str {
    match cap {
        LineCap::Butt => "butt",
        LineCap::Square => "square",
        LineCap::Round => "round",
    }
}

/// Returns the Canvas2D name of a line join.
///
/// # Arguments
///
/// - `LineJoin` - A `LineJoin` parameter.
///
/// # Returns
///
/// - `&'static str` - A `'static str` value.
pub(crate) fn line_join_name(join: LineJoin) -> &'static str {
    match join {
        LineJoin::Miter => "miter",
        LineJoin::Round => "round",
        LineJoin::Bevel => "bevel",
    }
}
