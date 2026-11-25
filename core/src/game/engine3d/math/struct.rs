use super::*;

/// Three-dimensional floating-point vector for 3D positions, directions, normals, and RGB radiance.
#[derive(Clone, Copy, Data, Debug, Default, New, PartialEq)]
pub struct Vec3 {
    /// X component.
    #[get(type(copy))]
    pub(crate) x: f32,
    /// Y component.
    #[get(type(copy))]
    pub(crate) y: f32,
    /// Z component.
    #[get(type(copy))]
    pub(crate) z: f32,
}

/// Column-major 4x4 transformation and perspective projection matrix.
#[derive(Clone, Copy, Data, Debug, PartialEq)]
pub struct Mat4 {
    /// Sixteen contiguous matrix elements in column-major order.
    #[get(type(copy))]
    pub(crate) m: [f32; 16],
}

/// Axis-aligned 3D bounding box used for frustum culling, chunk streaming, and BVH acceleration.
#[derive(Clone, Copy, Data, Debug, New, PartialEq)]
pub struct Aabb3D {
    /// Minimum corner `(min_x, min_y, min_z)`.
    #[get(type(copy))]
    pub(crate) min: Vec3,
    /// Maximum corner `(max_x, max_y, max_z)`.
    #[get(type(copy))]
    pub(crate) max: Vec3,
}

/// 3D ray with origin, normalized direction, and precomputed reciprocal direction for fast BVH traversal.
#[derive(Clone, Copy, Data, Debug, PartialEq)]
pub struct Ray3D {
    /// Ray origin in world space.
    #[get(type(copy))]
    pub(crate) origin: Vec3,
    /// Unit ray direction vector.
    #[get(type(copy))]
    pub(crate) direction: Vec3,
    /// Precomputed `1.0 / direction` for branchless slab AABB testing.
    #[get(type(copy))]
    pub(crate) inv_direction: Vec3,
}

/// Six-plane view frustum extracted from a view-projection matrix for O(1) AABB culling.
#[derive(Clone, Copy, Data, Debug, PartialEq)]
pub struct Frustum3D {
    /// Six normalized plane equations `(a, b, c, d)` where `ax + by + cz + d >= 0` is inside.
    #[get(type(copy))]
    pub(crate) planes: [[f32; 4]; 6],
}

/// 3D perspective camera supporting free-fly, third-person follow, yaw/pitch orbit, and frustum culling.
#[derive(Clone, Copy, Data, Debug, PartialEq)]
pub struct Camera3D {
    /// World-space eye position.
    #[get(type(copy))]
    pub(crate) position: Vec3,
    /// Horizontal yaw angle in radians.
    #[get(type(copy))]
    pub(crate) yaw: f32,
    /// Vertical pitch angle in radians (clamped to `[-1.4, 1.4]`).
    #[get(type(copy))]
    pub(crate) pitch: f32,
    /// Vertical field of view in degrees.
    #[get(type(copy))]
    pub(crate) fov_deg: f32,
    /// Near clipping plane distance.
    #[get(type(copy))]
    pub(crate) near: f32,
    /// Far clipping plane distance.
    #[get(type(copy))]
    pub(crate) far: f32,
}
