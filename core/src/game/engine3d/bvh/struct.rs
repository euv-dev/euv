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

/// Flat-array Bounding Volume Hierarchy accelerating ray-traced shadows, reflections, and GI in O(log N).
#[derive(Clone, Data, Debug, Default)]
pub struct Bvh3D {
    /// Contiguous BVH nodes with root at index `0`.
    pub(crate) nodes: Vec<BvhNode3D>,
    /// Partitioned world-space triangle storage.
    pub(crate) triangles: Vec<BvhTriangle>,
}
