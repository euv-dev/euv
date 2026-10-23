use super::*;

/// Draws the series of a chart into the display list.
///
/// The renderer builds the *paths* of the series; turning them into
/// triangles is the backend's job, because each backend has its own
/// way of getting geometry onto the screen.
#[derive(Clone, Debug)]
pub struct SeriesRenderer {}

/// A node of a graph series.
#[derive(Clone, Debug)]
pub(crate) struct GraphNode {
    /// The name of the node.
    pub name: String,
    /// The value of the node.
    pub value: f64,
    /// The horizontal position of the node.
    pub x: f64,
    /// The vertical position of the node.
    pub y: f64,
    /// The size of the node's symbol.
    pub size: f64,
}

/// A link between two nodes of a graph series.
#[derive(Clone, Debug)]
pub(crate) struct GraphLink {
    /// The name of the node the link starts at.
    pub source: String,
    /// The name of the node the link ends at.
    pub target: String,
    /// The value of the link.
    pub value: f64,
}

/// The geometry of a radar coordinate system.
#[derive(Clone, Copy, Data, Debug)]
pub(crate) struct RadarGeometry {
    /// The centre of the radar.
    #[get(type(copy))]
    pub center_x: f64,
    /// The centre of the radar.
    #[get(type(copy))]
    pub center_y: f64,
    /// The radius of the radar.
    #[get(type(copy))]
    pub radius: f64,
    /// The number of sides of the radar.
    #[get(type(copy))]
    pub sides: usize,
    /// The angle the first side starts at, in radians.
    #[get(type(copy))]
    pub start_angle: f64,
}

/// A node of a sankey series.
#[derive(Clone, Debug)]
pub(crate) struct SankeyNode {
    /// The name of the node.
    pub name: String,
    /// The value of the node.
    pub value: f64,
    /// The column the node sits in.
    pub column: usize,
    /// The vertical position of the node's top edge.
    pub top: f64,
    /// The height of the node.
    pub height: f64,
}

/// A link of a sankey series.
#[derive(Clone, Debug)]
pub(crate) struct SankeyLink {
    /// The name of the node the link starts at.
    pub source: String,
    /// The name of the node the link ends at.
    pub target: String,
    /// The value of the link.
    pub value: f64,
}

/// A sector of a sunburst.
#[derive(Clone, Debug)]
pub(crate) struct SunburstSector {
    /// The name of the sector.
    pub name: String,
    /// The value of the sector.
    pub value: f64,
    /// The ring the sector is drawn in.
    pub depth: usize,
    /// The angle the sector starts at, in radians.
    pub start_angle: f64,
    /// The angle the sector ends at, in radians.
    pub end_angle: f64,
}

/// A tile of a treemap.
#[derive(Clone, Debug)]
pub(crate) struct TreeTile {
    /// The name of the tile.
    pub name: String,
    /// The value of the tile.
    pub value: f64,
    /// The rectangle the tile occupies.
    pub rect: BBox,
    /// The depth of the tile in the hierarchy.
    pub depth: usize,
}
