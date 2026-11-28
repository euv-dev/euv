use super::*;

/// Recursive walk state shared across concentric `Sunburst` rings.
#[derive(Debug)]
pub(crate) struct SunburstWalk<'a> {
    /// Center coordinate of the sunburst diagram.
    pub(crate) center: Point,
    /// Innermost hollow start radius.
    pub(crate) inner_r: f32,
    /// Radial ring width per hierarchy level.
    pub(crate) ring_step: f32,
    /// Half angular padding between adjacent sibling wedges in radians.
    pub(crate) pad_rad: f32,
    /// Whether text labels are rendered inside large sectors.
    pub(crate) show_labels: bool,
    /// Active series metadata.
    pub(crate) meta: SeriesMeta<'a>,
    /// Incrementing hit index across recursive branches.
    pub(crate) hit_idx: usize,
    /// Target hit-test output vector.
    pub(crate) hits: &'a mut Vec<EuvChartDatumHit>,
}
