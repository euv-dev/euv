use super::*;

/// Computed Sankey column node layout bounds and running port cursors.
#[derive(Clone, Debug)]
pub(crate) struct SankeyNodeBox {
    /// Topological column depth.
    pub(crate) depth: usize,
    /// Computed pixel rectangle.
    pub(crate) rect: Rect,
    /// Node fill color.
    pub(crate) color: Color,
    /// Total node flow magnitude (`max(in, out)`).
    pub(crate) flow: f32,
    /// Running Y cursor along the right edge for outgoing ribbons.
    pub(crate) out_cursor_y: f32,
    /// Running Y cursor along the left edge for incoming ribbons.
    pub(crate) in_cursor_y: f32,
}

/// Positioned hierarchical tree node used during orthogonal or radial rendering.
#[derive(Clone, Debug)]
pub(crate) struct TreeLayoutNode {
    /// Node display label.
    pub(crate) name: String,
    /// Node numeric weight.
    pub(crate) value: f32,
    /// Optional explicit color override.
    pub(crate) color: Option<Color>,
    /// Zero-based hierarchy depth.
    pub(crate) depth: usize,
    /// Normalized leaf order coordinate in `[0.0, 1.0]`.
    pub(crate) order: f32,
    /// Projected screen pixel position.
    pub(crate) pos: Point,
    /// Optional parent index inside the flat layout array.
    pub(crate) parent: Option<usize>,
}

/// Shared mutable traversal state for recursive squarified `Treemap` rendering.
#[derive(Debug)]
pub(crate) struct TreemapWalk<'a> {
    /// Whether parent branch header bars are rendered.
    pub(crate) show_headers: bool,
    /// Gap between adjacent tiles in logical pixels.
    pub(crate) cell_gap: f32,
    /// Tile corner radius in logical pixels.
    pub(crate) corner_radius: f32,
    /// Series metadata.
    pub(crate) meta: SeriesMeta<'a>,
    /// Running hit index counter.
    pub(crate) hit_idx: usize,
    /// Target hit-test list.
    pub(crate) hits: &'a mut Vec<EuvChartDatumHit>,
}

/// Geo equirectangular projection frame mapping `(lon, lat)` into pixel `Rect`.
#[derive(Clone, Copy, Debug)]
pub(crate) struct GeoProjection {
    /// Minimum visible longitude in degrees.
    pub(crate) min_lon: f32,
    /// Maximum visible longitude in degrees.
    pub(crate) max_lon: f32,
    /// Minimum visible latitude in degrees.
    pub(crate) min_lat: f32,
    /// Maximum visible latitude in degrees.
    pub(crate) max_lat: f32,
    /// Target pixel viewport inside the chart plot.
    pub(crate) viewport: Rect,
}
