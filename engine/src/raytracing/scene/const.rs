/// The default maximum number of bounces evaluated by
/// [`RayTraceScene::trace`](crate::RayTraceScene::trace) before returning. Higher values yield more
/// accurate reflections at increased computational cost.
pub(crate) const RAYTRACE_DEFAULT_MAX_BOUNCES: u32 = 4;

/// The minimum ray parameter `t` accepted as a valid intersection. Hits
/// closer than this are treated as self-intersection artifacts and skipped.
pub(crate) const RAYTRACE_DEFAULT_T_MIN: f64 = 0.001;

/// The maximum ray parameter `t` evaluated before a ray is considered to
/// have escaped the scene. Pairs with `RAYTRACE_DEFAULT_T_MIN`.
pub(crate) const RAYTRACE_DEFAULT_T_MAX: f64 = 1000.0;

/// The Moller-Trumbore determinant magnitude below which a ray is treated as
/// parallel to the triangle plane and therefore unable to hit it. Pairs with
/// the shared [`EPSILON`](crate::EPSILON) tolerance.
pub(crate) const RAYTRACE_TRIANGLE_EPSILON: f64 = 1e-8;

/// The maximum number of primitives a single BVH leaf may hold before the
/// builder splits it further. A small leaf keeps the exact per-primitive
/// tests visible in profiles; a larger value trades that for shorter
/// traversal sequences on uniform scenes.
pub(crate) const RAYTRACE_BVH_LEAF_SIZE: usize = 4;

/// The fixed traversal stack size, sized so a hierarchy can never outgrow
/// it.
///
/// Traversal pushes at most one extra entry per internal node it pops, so
/// the pending stack never exceeds `RAYTRACE_BVH_MAX_DEPTH + 1` entries.
/// Sizing the array well above that bound keeps the stack a plain array on
/// the caller's frame, which is what makes a query allocation free.
pub(crate) const RAYTRACE_BVH_STACK_SIZE: usize = 64;

/// The deepest a BVH subtree may nest before the builder stops splitting.
///
/// The surface-area heuristic deliberately does not guarantee a balanced
/// tree: on a geometric spread it can keep choosing to peel off a single
/// cheap primitive and chain linearly, so depth is **not** bounded by
/// `log2(n)`. A linear chain would overflow the fixed traversal stack and
/// silently drop the near child of every node below the overflow, which
/// loses real geometry rather than merely costing time. Capping the build
/// depth here and emitting a leaf at the cap bounds the hierarchy by
/// construction; a leaf only ever tests more primitives, never fewer, so
/// the cap trades speed for a guarantee that no hit can be lost.
pub(crate) const RAYTRACE_BVH_MAX_DEPTH: usize = 48;

/// The number of subdivisions each axis of a node's bounds is split into
/// while binning its primitives for the surface-area-heuristic cost.
pub(crate) const RAYTRACE_BVH_BINS: usize = 16;

/// The largest scene that is traced by walking every primitive directly.
///
/// Below this count the hierarchy cannot pay for itself: a build makes every
/// query pay one extra bounds test and one indirect hop per primitive, and
/// at a handful of primitives a straight scan is measurably cheaper. Above
/// it the scan's cost grows linearly while the traversal does not, so the
/// build is worth it.
pub(crate) const RAYTRACE_LINEAR_MAX_PRIMITIVES: usize = 4;

/// The centroid axis must span more than this fraction of the widest
/// centroid axis before it is worth splitting on it, so flat or
/// coincident centroid sets fall back to a median split instead.
pub(crate) const RAYTRACE_BVH_AXIS_EPSILON: f64 = 1e-9;

/// A shadow ray's origin is pushed this far along the surface normal
/// before it leaves the shading point, so a surface never shadows itself
/// through numerical slop at its own footprint.
pub(crate) const RAYTRACE_SHADOW_SELF_OFFSET: f64 = 0.001;

/// A shadow segment stops this far short of a finite light position, so
/// geometry that merely encloses the emitter does not stay lit-in-shadow.
pub(crate) const RAYTRACE_SHADOW_LIGHT_EPSILON: f64 = 0.001;
