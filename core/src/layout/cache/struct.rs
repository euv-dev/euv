use super::*;

/// Available layout space passed down during a layout pass and cached for clean-subtree skipping.
#[derive(Clone, Copy, Data, Debug, Default, New, PartialEq)]
pub struct AvailableSpace {
    /// Available horizontal extent in logical pixels.
    #[get(type(copy))]
    pub(crate) width: f32,
    /// Available vertical extent in logical pixels.
    #[get(type(copy))]
    pub(crate) height: f32,
    /// Root viewport size for `vw`/`vh` length resolution.
    #[get(type(copy))]
    pub(crate) viewport: Size,
    /// Origin X coordinate where this node is placed.
    #[get(type(copy))]
    pub(crate) origin_x: f32,
    /// Origin Y coordinate where this node is placed.
    #[get(type(copy))]
    pub(crate) origin_y: f32,
}

/// Internal container constraint bundle passed to child layout solvers.
#[derive(Clone, Copy, Data, Debug, Default, New, PartialEq)]
pub(crate) struct ContainerFrame {
    /// Parent layout properties.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) parent_layout: LayoutStyle,
    /// Inner available dimensions inside padding and border.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) inner_avail: Size,
    /// Root viewport logical size.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) viewport: Size,
    /// Content origin in root coordinates.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) content_origin: Point,
}

/// Telemetry counters recorded per layout pass.
#[derive(Clone, Copy, Data, Debug, Default, Eq, Hash, New, Ord, PartialEq, PartialOrd)]
pub struct LayoutStats {
    /// Number of nodes laid out during this pass.
    #[get(type(copy))]
    pub(crate) nodes_computed: usize,
    /// Number of clean subtrees skipped in O(1) via constraint caching.
    #[get(type(copy))]
    pub(crate) subtrees_skipped: usize,
}
