use super::*;

/// A uniform-grid spatial hash for broad-phase collision culling in 2D.
///
/// Bodies are inserted by their world-space axis-aligned bounding box.
/// A query returns all candidate indices whose AABBs overlap the query region,
/// dramatically reducing narrow-phase collision checks from O(n²) to near O(n).
#[derive(Clone, Data, Debug, New, PartialEq)]
pub struct SpatialHashGrid2D {
    /// The world-space size of each grid cell.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) cell_size: f64,
    /// The inverse of `cell_size`, precomputed for fast coordinate-to-cell hashing.
    #[get(type(copy))]
    #[set(pub(crate))]
    #[new(skip)]
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    pub(crate) inverse_cell_size: f64,
    /// The hash map from cell key to the list of body indices occupying that cell.
    #[get_mut(pub(crate))]
    #[new(skip)]
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) cells: SpatialCellMap2D,
}

/// A uniform-grid spatial hash for broad-phase collision culling in 3D.
///
/// Bodies are inserted by their world-space axis-aligned bounding box.
/// A query returns all candidate indices whose AABBs overlap the query region,
/// dramatically reducing narrow-phase collision checks from O(n²) to near O(n).
#[derive(Clone, Data, Debug, New, PartialEq)]
pub struct SpatialHashGrid3D {
    /// The world-space size of each grid cell.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) cell_size: f64,
    /// The inverse of `cell_size`, precomputed for fast coordinate-to-cell hashing.
    #[get(type(copy))]
    #[set(pub(crate))]
    #[new(skip)]
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    pub(crate) inverse_cell_size: f64,
    /// The hash map from cell key to the list of body indices occupying that cell.
    #[get_mut(pub(crate))]
    #[new(skip)]
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) cells: SpatialCellMap3D,
}

/// One inserted body inside a 2D quadtree, stored with its exact
/// world-space axis-aligned bounding box.
///
/// Carrying the box is what makes the structure a *correct* broad phase: a
/// quadtree query prunes whole nodes by region overlap, then confirms each
/// surviving candidate with an exact box-vs-box test instead of reporting
/// every body that merely shares a node.
#[derive(Clone, Copy, Data, Debug, New, PartialEq)]
pub struct QuadTreeEntry2D {
    /// The caller-owned body index, returned verbatim by queries.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) index: usize,
    /// The minimum corner of the body's world-space bounding box.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) min: Vector2D,
    /// The maximum corner of the body's world-space bounding box.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) max: Vector2D,
}

/// One node of the 2D quadtree, holding an axis-aligned region plus the
/// entries that could not be pushed further down.
///
/// A node has either four child handles (subdivided) or none (leaf); children
/// live in the parent's flat arena rather than behind `Box` pointers, so every
/// node is reachable through a single shared borrow and the structure stays
/// `Clone`/`Debug` derivable without interior mutability.
#[derive(Clone, Data, Debug, New, PartialEq)]
pub struct QuadTreeNode2D {
    /// The minimum corner of the region this node covers.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) min: Vector2D,
    /// The maximum corner of the region this node covers.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) max: Vector2D,
    /// The subdivision level, `0` at the root and incrementing per split.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) depth: usize,
    /// The child node handles, all unset until the node subdivides.
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub(crate) children: QuadTreeChildren2D,
    /// A flag that is `true` once the node owns four valid child handles.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub(crate) leaf: bool,
    /// A flag that is `true` once this node retains an entry which is not fully
    /// contained by the node's own region.
    ///
    /// Such an entry stays in the parent forever, so a region-overlap prune is
    /// unsound for this node: the body can sit entirely outside the node's
    /// region and still overlap a query. Marking the node `loose` makes the
    /// query skip the region prune and fall back to the exact box test, which
    /// is what keeps a body larger than the root region findable.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub(crate) loose: bool,
    /// The entries stored directly in this node, including any body that is
    /// too large for, or straddles the boundary of, a single child region.
    #[set(pub(crate))]
    pub(crate) entries: QuadTreeEntryList2D,
}

/// A 2D quadtree broad-phase acceleration structure, interface-compatible
/// with [`SpatialHashGrid2D`].
///
/// Bodies are inserted by their world-space axis-aligned bounding box. A node
/// subdivides into four children once its entry count exceeds the capacity and
/// its depth is below the maximum; an entry whose box straddles a split
/// boundary stays in the parent node, so no body is ever duplicated and every
/// query visits each body exactly once.
///
/// Unlike the uniform grid, the quadtree concentrates resolution where bodies
/// are dense, which makes it the better structure for scenes with strongly
/// clustered content. A body larger than the root region is still inserted and
/// still returned by every query overlapping it — it simply resides at the
/// root and is confirmed by its exact box test.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct QuadTree2D {
    /// The arena of all live nodes; node 0 is always the root.
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) nodes: QuadTreeNodeList2D,
    /// The number of entries a node may hold before subdividing.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    pub(crate) capacity: usize,
    /// The deepest subdivision level a node may reach.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) max_depth: usize,
    /// The total number of inserted bodies across every node.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub(crate) count: usize,
}
