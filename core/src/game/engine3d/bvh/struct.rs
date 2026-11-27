use super::*;

/// World-space triangle stored in the flat BVH leaf array for ray-traced shadows, reflections, and GI.
#[derive(Clone, Copy, Data, Debug, PartialEq)]
pub struct BvhTriangle {
    /// First vertex position in world space.
    #[get(type(copy))]
    pub(crate) v0: Vec3,
    /// Edge vector `v1 - v0`.
    #[get(type(copy))]
    pub(crate) edge1: Vec3,
    /// Edge vector `v2 - v0`.
    #[get(type(copy))]
    pub(crate) edge2: Vec3,
    /// Unit face normal.
    #[get(type(copy))]
    pub(crate) normal: Vec3,
    /// Precomputed triangle centroid for O(N log N) BVH partitioning.
    #[get(type(copy))]
    pub(crate) centroid: Vec3,
    /// Precomputed triangle bounding box.
    #[get(type(copy))]
    pub(crate) bounds: Aabb3D,
    /// Surface PBR material referenced on reflection/GI hits.
    #[get(type(copy))]
    pub(crate) material: PbrMaterial,
}

/// Cache-friendly flat BVH tree node (interior node or leaf triangle range).
#[derive(Clone, Copy, Data, Debug, PartialEq)]
pub struct BvhNode3D {
    /// World-space AABB enclosing this subtree.
    #[get(type(copy))]
    pub(crate) bounds: Aabb3D,
    /// Left child node index (`0` for leaves).
    #[get(type(copy))]
    pub(crate) left: u32,
    /// Right child node index (`0` for leaves).
    #[get(type(copy))]
    pub(crate) right: u32,
    /// First triangle index in `Bvh3D::triangles` for leaves.
    #[get(type(copy))]
    pub(crate) first_tri: u32,
    /// Triangle count (`> 0` denotes a leaf node).
    #[get(type(copy))]
    pub(crate) tri_count: u32,
}

/// Closest ray-triangle hit result returned by `Bvh3D::trace_closest`.
#[derive(Clone, Copy, Data, Debug, PartialEq)]
pub struct RayHit3D {
    /// Ray parametric hit distance `t`.
    #[get(type(copy))]
    pub(crate) distance: f32,
    /// World-space hit position.
    #[get(type(copy))]
    pub(crate) point: Vec3,
    /// Unit surface normal at the hit point.
    #[get(type(copy))]
    pub(crate) normal: Vec3,
    /// Surface PBR material at the hit point.
    #[get(type(copy))]
    pub(crate) material: PbrMaterial,
}

/// Cache-compact 36-byte hot geometry triangle for L1-dense Möller–Trumbore ray intersections.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct BvhGeomTri {
    /// First vertex `(x, y, z)`.
    pub(crate) v0: [f32; 3],
    /// First edge `(v1 - v0)`.
    pub(crate) e1: [f32; 3],
    /// Second edge `(v2 - v0)`.
    pub(crate) e2: [f32; 3],
}

/// 4-wide Structure-of-Arrays leaf triangle packet enabling auto-vectorized 4-lane Möller–Trumbore ray tests.
#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct BvhTri4 {
    /// Four triangle `v0.x` coordinates.
    pub(crate) v0x: [f32; 4],
    /// Four triangle `v0.y` coordinates.
    pub(crate) v0y: [f32; 4],
    /// Four triangle `v0.z` coordinates.
    pub(crate) v0z: [f32; 4],
    /// Four triangle `edge1.x` components (`0.0` for inactive padded lanes).
    pub(crate) e1x: [f32; 4],
    /// Four triangle `edge1.y` components (`0.0` for inactive padded lanes).
    pub(crate) e1y: [f32; 4],
    /// Four triangle `edge1.z` components (`0.0` for inactive padded lanes).
    pub(crate) e1z: [f32; 4],
    /// Four triangle `edge2.x` components (`0.0` for inactive padded lanes).
    pub(crate) e2x: [f32; 4],
    /// Four triangle `edge2.y` components (`0.0` for inactive padded lanes).
    pub(crate) e2y: [f32; 4],
    /// Four triangle `edge2.z` components (`0.0` for inactive padded lanes).
    pub(crate) e2z: [f32; 4],
    /// Global triangle indices inside `Bvh3D::triangles` for each lane (`u32::MAX` for padded lanes).
    pub(crate) tri_indices: [u32; 4],
}

/// Cache-line-paired 32-byte BVH node storing sibling pairs `(2k, 2k + 1)` together in one 64-byte cache line.
#[repr(C, align(32))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct BvhFlatNode {
    /// Minimum AABB corner `(x, y, z)`.
    pub(crate) min: [f32; 3],
    /// Even index of the contiguous child pair `(first_child, first_child + 1)` when `tri_count == 0`, or first `BvhTri4` packet index when `tri_count > 0`.
    pub(crate) child_or_first: u32,
    /// Maximum AABB corner `(x, y, z)`.
    pub(crate) max: [f32; 3],
    /// Leaf `BvhTri4` packet count (`0` for interior nodes).
    pub(crate) tri_count: u32,
}

/// Flat-array Bounding Volume Hierarchy accelerating ray-traced shadows, reflections, and GI in O(log N).
#[derive(Clone, Data, Debug, Default)]
pub struct Bvh3D {
    /// Contiguous BVH nodes with root at index `0`.
    pub(crate) nodes: Vec<BvhNode3D>,
    /// Partitioned world-space triangle metadata and shading materials.
    pub(crate) triangles: Vec<BvhTriangle>,
    /// Hot 36-byte packed geometry mirror aligned 1:1 with `triangles` for fast single-triangle occluder hits.
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) geom: Vec<BvhGeomTri>,
    /// 4-wide SoA leaf triangle packets referenced by `BvhFlatNode` leaves for SIMD ray intersection.
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) leaf_packets: Vec<BvhTri4>,
    /// Hot 32-byte DFS-ordered BVH mirror for branchless cache-dense FMA traversal.
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) flat_nodes: Vec<BvhFlatNode>,
}
