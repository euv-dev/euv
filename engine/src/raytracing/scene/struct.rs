use super::*;

/// A 3D ray carrying the parameters needed for traversal and depth tracking.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct Ray {
    /// The world-space origin of the ray.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub origin: Vector3D,
    /// The unit direction of the ray.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    pub direction: Vector3D,
    /// The minimum acceptable `t` value for valid intersections.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub t_min: f64,
    /// The maximum `t` value before the ray escapes the scene.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub t_max: f64,
    /// The current recursion depth (used to terminate recursive bounces).
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub depth: u32,
}

/// The result of a ray-object intersection.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct Hit {
    /// The ray parameter at the hit point.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub t: f64,
    /// The world-space hit position.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub position: Vector3D,
    /// The outward unit normal at the hit point.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub normal: Vector3D,
    /// The material of the hit surface.
    #[get(pub(crate), type(clone))]
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
    pub kind: OccluderKind,
    /// For spheres: the center. For AABBs: the minimum corner. Unused by
    /// triangles, whose vertices live in `vertices`.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub center: Vector3D,
    /// For spheres: `.x` is the radius. For AABBs: the maximum corner.
    /// Unused by triangles, whose vertices live in `vertices`.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub extent: Vector3D,
    /// For triangles: the three corners in winding order. Unused by spheres
    /// and AABBs, which are fully described by `center` and `extent`.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub vertices: [Vector3D; 3],
    /// The surface material.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) material: Material,
}

/// One node of the flat bounding volume hierarchy owned by
/// [`RayTraceScene`].
///
/// The hierarchy is stored as a preorder array: a node is followed
/// immediately by its left child at `index + 1`, and `escape` holds the
/// index of the first node after this node's entire subtree. A node with
/// `count == 0` is internal, and traversal either descends into
/// `index + 1` or jumps to `escape` when the ray misses the bounds. A node
/// with `count > 0` is a leaf whose `start`/`count` slice of the scene's
/// `indices` array lists the primitives it owns, reordered but never
/// renumbered: every entry stays the occluder's original index.
#[derive(Clone, Copy, Data, Debug, PartialEq)]
pub(crate) struct RayTraceNode {
    /// The minimum corner of the axis-aligned bounds covering the subtree.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) lower: Vector3D,
    /// The maximum corner of the axis-aligned bounds covering the subtree.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) upper: Vector3D,
    /// The first entry of this leaf's primitive slice in the scene's
    /// `indices` array; unused when `count == 0`.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) start: usize,
    /// The number of primitives in this leaf; zero marks an internal node.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) count: usize,
    /// The index of the first node after this subtree, so a miss can skip
    /// the whole subtree in one step.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) escape: usize,
}

/// The per-primitive bounds and centroids a BVH build reads while
/// splitting.
///
/// Holding them apart from [`RayTraceScene`] keeps the build scratch out
/// of the stored scene: once the hierarchy is built the three arrays are
/// dropped, so nothing a query touches carries build-time state.
#[derive(Debug)]
pub(crate) struct BvhBuild {
    /// The minimum corner of each primitive's world-space bounds.
    pub(crate) lower: Vec<Vector3D>,
    /// The maximum corner of each primitive's world-space bounds.
    pub(crate) upper: Vec<Vector3D>,
    /// The midpoint of each primitive's bounds, used to bin and partition.
    pub(crate) centroids: Vec<Vector3D>,
}

/// A half-open slice of the permuted primitive index array that one BVH
/// node owns.
///
/// The named start and count fields are shared by measurement, partition,
/// and split helpers.
#[derive(Clone, Copy, Debug)]
pub(crate) struct BvhRange {
    /// The first entry of the range in the primitive index array.
    pub(crate) start: usize,
    /// The number of entries in the range.
    pub(crate) count: usize,
}

/// The single node one `build_bvh_subtree` call is responsible for
/// emitting.
///
/// The bounds, the primitive range and the nesting level describe which
/// subtree is being built and at what depth, which is distinct from the
/// mutable arrays every step shares.
#[derive(Clone, Copy, Debug)]
pub(crate) struct BvhSubtree {
    /// The `(min, max)` corners covering this subtree.
    pub(crate) bounds: (Vector3D, Vector3D),
    /// The primitives this subtree owns.
    pub(crate) range: BvhRange,
    /// The nesting level of this subtree, counting from zero at the root.
    pub(crate) depth: usize,
}

/// One centroid axis and the position on it where a range is cut.
///
/// The surface-area search chooses a split and the fallback median split
/// picks one too, so both hand the same pair of values to the partition
/// step instead of it re-reading two loose arguments.
#[derive(Clone, Copy, Debug)]
pub(crate) struct SahSplit {
    /// `0` for `x`, `1` for `y`, `2` for `z`.
    pub(crate) axis: usize,
    /// The split position on that axis.
    pub(crate) boundary: f64,
}

/// The mutable build state that every BVH construction step shares.
///
/// The node array under construction and the primitive index array it
/// permutes have to be threaded through every helper, including both sides
/// of every recursive call. Holding them behind one borrowed context keeps
/// each helper's remaining parameters to the decisions that helper alone
/// makes. Constructing the context does not allocate additional storage.
#[derive(Debug)]
pub(crate) struct BvhBuildContext<'a> {
    /// The preorder node array under construction.
    pub(crate) nodes: &'a mut Vec<RayTraceNode>,
    /// The primitive index array, partitioned in place.
    pub(crate) indices: &'a mut [usize],
    /// The per-primitive bounds and centroids the split search reads.
    pub(crate) build: &'a BvhBuild,
}

/// An owned ray-tracing scene with a precomputed bounding volume
/// hierarchy and shadow data.
///
/// `RayTraceScene` bundles the occluder list, the flat BVH over their
/// bounds, and the `(center, radius)` shadow bounding spheres consumed by
/// [`soft_shadow_factor`], computing both exactly once at construction.
/// Tracing through [`RayTraceScene::trace`] and querying through
/// [`RayTraceScene::closest_hit`] perform no heap allocation per ray or
/// per bounce, making the scene the single canonical entry point for
/// tracing many rays against a static scene.
#[derive(Clone, Debug, Getter, PartialEq)]
pub struct RayTraceScene {
    /// All occluding surfaces in the scene.
    #[get(pub(crate))]
    pub(crate) occluders: Vec<Occluder>,
    /// The flat preorder BVH over the occluders' bounds. Shared verbatim
    /// with the GPU encoders, so the layout is part of the internal
    /// contract rather than an implementation detail.
    #[get(pub(crate))]
    pub(crate) nodes: Vec<RayTraceNode>,
    /// The reordered primitive indexes addressed by the leaves' `start` and
    /// `count` fields. Every value stays the occluder's original index.
    #[get(pub(crate))]
    pub(crate) indices: Vec<usize>,
    /// Precomputed `(center, radius)` shadow bounding spheres, one per
    /// occluder, in the same order as `occluders`.
    #[get(pub(crate))]
    pub(crate) shadow_points: Vec<(Vector3D, f64)>,
}
