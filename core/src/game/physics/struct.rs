use super::*;

/// Global 2D rigid-body physics configuration.
#[derive(Clone, Copy, Data, Debug, New, PartialEq)]
pub struct PhysicsConfig {
    /// Global gravitational acceleration `(gx, gy)` in world units per second squared.
    #[get(type(copy))]
    pub(crate) gravity: Point,
    /// Spatial-hash grid cell size for O(N) broad-phase collision pairing.
    #[get(type(copy))]
    pub(crate) cell_size: f32,
    /// Baumgarte positional separation factor in `[0.0, 1.0]` preventing interpenetration.
    #[get(type(copy))]
    pub(crate) position_correction: f32,
    /// Optional world arena containment rectangle bouncing dynamic bodies at borders.
    #[get(skip)]
    pub(crate) arena_bounds: Option<Rect>,
}

/// Contact event reported when two colliders overlap or collide during a physics step.
#[derive(Clone, Copy, Data, Debug, New, PartialEq)]
pub struct CollisionEvent {
    /// First colliding entity handle.
    #[get(type(copy))]
    pub(crate) entity_a: EntityId,
    /// Second colliding entity handle.
    #[get(type(copy))]
    pub(crate) entity_b: EntityId,
    /// Contact normal pointing from `entity_a` toward `entity_b`.
    #[get(type(copy))]
    pub(crate) normal: Point,
    /// Penetration depth in world units.
    #[get(type(copy))]
    pub(crate) penetration: f32,
    /// Whether either collider was marked as a trigger volume.
    #[get(type(copy))]
    pub(crate) is_trigger: bool,
}

/// Result of a 2D physics raycast intersection query.
#[derive(Clone, Copy, Data, Debug, New, PartialEq)]
pub struct RaycastHit {
    /// Hit entity handle.
    #[get(type(copy))]
    pub(crate) entity: EntityId,
    /// World-space impact point.
    #[get(type(copy))]
    pub(crate) point: Point,
    /// Surface normal at the hit point.
    #[get(type(copy))]
    pub(crate) normal: Point,
    /// Distance along the ray from its origin.
    #[get(type(copy))]
    pub(crate) distance: f32,
}

/// Telemetry counters produced by a single `PhysicsWorld2D::step` invocation.
#[derive(Clone, Copy, Data, Debug, Default, Eq, Hash, New, PartialEq)]
pub struct PhysicsStepStats {
    /// Number of bodies integrated during the step.
    #[get(type(copy))]
    pub(crate) bodies_integrated: usize,
    /// Number of candidate broad-phase pairs inspected.
    #[get(type(copy))]
    pub(crate) broad_phase_pairs: usize,
    /// Number of narrow-phase collisions or triggers detected.
    #[get(type(copy))]
    pub(crate) collisions_detected: usize,
}

/// Reusable O(N) spatial-hash broad-phase accelerator and 2D impulse physics solver.
#[derive(Clone, Data, Debug)]
pub struct PhysicsWorld2D {
    /// Active simulation parameters.
    #[get(type(copy))]
    pub(crate) config: PhysicsConfig,
    /// Cell-indexed spatial hash buckets reused across steps.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) grid: HashMap<(i32, i32), Vec<usize>>,
    /// Deduplication set for candidate broad-phase index pairs.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) seen_pairs: HashSet<(usize, usize)>,
    /// Most recent step telemetry.
    #[get(type(copy))]
    pub(crate) last_stats: PhysicsStepStats,
}
