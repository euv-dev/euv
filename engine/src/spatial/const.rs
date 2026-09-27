/// The default cell size for the 2D spatial hash grid in world units.
pub(crate) const SPATIAL_DEFAULT_CELL_SIZE_2D: f64 = 64.0;

/// The default cell size for the 3D spatial hash grid in world units.
pub(crate) const SPATIAL_DEFAULT_CELL_SIZE_3D: f64 = 64.0;

/// The default number of entries a 2D quadtree node holds before subdividing.
pub(crate) const SPATIAL_DEFAULT_CAPACITY_2D: usize = 8;

/// The default maximum subdivision depth of the 2D quadtree.
pub(crate) const SPATIAL_DEFAULT_MAX_DEPTH_2D: usize = 8;

/// The upper bound clamp applied to a caller-supplied 2D quadtree depth.
pub(crate) const SPATIAL_MAX_DEPTH_2D: usize = 24;

/// The default half-extent of the 2D quadtree root region in world units.
pub(crate) const SPATIAL_DEFAULT_HALF_EXTENT_2D: f64 = 1024.0;

/// The index of the root node inside the 2D quadtree node arena.
pub(crate) const SPATIAL_QUAD_TREE_ROOT_INDEX: usize = 0;

/// The child handle held by a 2D quadtree node that has not subdivided yet.
pub(crate) const SPATIAL_QUAD_TREE_NO_CHILD: usize = usize::MAX;

/// The share of one child region, used to place the mid split of a quadtree node.
pub(crate) const SPATIAL_QUAD_TREE_MID_RATIO: f64 = 0.5;
