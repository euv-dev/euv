use super::*;

/// A grid cell key for 2D spatial partitioning, combining x and y indices.
pub type CellKey2D = (i32, i32);

/// A grid cell key for 3D spatial partitioning, combining x, y, and z indices.
pub type CellKey3D = (i32, i32, i32);

/// A list of body indices stored within a single grid cell.
pub type CellEntries = Vec<usize>;

/// A hash map from 2D cell keys to lists of body indices.
pub type SpatialCellMap2D = HashMap<CellKey2D, CellEntries>;

/// A hash map from 3D cell keys to lists of body indices.
pub type SpatialCellMap3D = HashMap<CellKey3D, CellEntries>;

/// The four child node handles of a subdivided 2D quadtree node, ordered
/// low-x/low-y, high-x/low-y, low-x/high-y, high-x/high-y.
pub type QuadTreeChildren2D = [usize; 4];

/// The list of body entries stored directly inside one 2D quadtree node.
pub type QuadTreeEntryList2D = Vec<QuadTreeEntry2D>;

/// The flat node arena backing a 2D quadtree, addressed by node handle.
pub type QuadTreeNodeList2D = Vec<QuadTreeNode2D>;

/// The explicit traversal stack used by 2D quadtree queries, so no recursion
/// is needed and no `Box`-linked child borrows are required.
pub type QuadTreeNodeStack2D = Vec<usize>;
