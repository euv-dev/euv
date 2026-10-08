use super::*;

/// A 3D ray carrying the parameters needed for traversal and depth tracking.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct Ray {
    /// The world-space origin of the ray.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) origin: Vector3D,
    /// The unit direction of the ray.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    pub(crate) direction: Vector3D,
    /// The minimum acceptable `t` value for valid intersections.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) t_min: f64,
    /// The maximum `t` value before the ray escapes the scene.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) t_max: f64,
    /// The current recursion depth (used to terminate recursive bounces).
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) depth: u32,
}

/// The result of a ray-object intersection.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct Hit {
    /// The ray parameter at the hit point.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) t: f64,
    /// The world-space hit position.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) position: Vector3D,
    /// The outward unit normal at the hit point.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) normal: Vector3D,
    /// The material of the hit surface.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) material: Material,
}

/// A ray-traceable geometric surface with an attached material.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct Occluder {
    /// The geometric shape represented by this occluder.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) kind: OccluderKind,
    /// For spheres: the center. For AABBs: the minimum corner. Unused by
    /// triangles, whose vertices live in `vertices`.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) center: Vector3D,
    /// For spheres: `.x` is the radius. For AABBs: the maximum corner.
    /// Unused by triangles, whose vertices live in `vertices`.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) extent: Vector3D,
    /// For triangles: the three corners in winding order. Unused by spheres
    /// and AABBs, which are fully described by `center` and `extent`.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) vertices: [Vector3D; 3],
    /// The surface material.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) material: Material,
}

/// An owned ray-tracing scene with precomputed shadow data.
///
/// `RayTraceScene` bundles the occluder list together with the
/// `(center, radius)` shadow bounding spheres consumed by
/// [`soft_shadow_factor`], computing them exactly once at construction.
/// Tracing through [`RayTraceScene::trace`] performs no heap allocation
/// per ray or per bounce, making the scene the single canonical entry
/// point for tracing many rays against a static scene.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct RayTraceScene {
    /// All occluding surfaces in the scene.
    #[get_mut(skip)]
    #[set(skip)]
    #[get(pub(crate))]
    pub(crate) occluders: Vec<Occluder>,
    /// Precomputed `(center, radius)` shadow bounding spheres, one per
    /// occluder, in the same order as `occluders`.
    #[get_mut(skip)]
    #[set(skip)]
    #[get(pub(crate))]
    pub(crate) shadow_points: Vec<(Vector3D, f64)>,
}
