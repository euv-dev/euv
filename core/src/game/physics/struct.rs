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

/// Fluent query descriptor for 2D physics raycasting against `PhysicsWorld2D`.
#[derive(Clone, Copy, Data, Debug, PartialEq)]
pub struct RaycastQuery2D {
    /// Ray origin in world units.
    #[get(type(copy))]
    pub(crate) origin: Point,
    /// Ray direction vector (normalized automatically during cast).
    #[get(type(copy))]
    pub(crate) direction: Point,
    /// Maximum ray travel distance in world units.
    #[get(type(copy))]
    pub(crate) max_distance: f32,
    /// Collision layer filter bitmask (defaults to `u32::MAX`).
    #[get(type(copy))]
    pub(crate) layer_mask: u32,
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

/// Cached per-collider integer cell bounds and layer filters reused across broad-phase passes.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct BodyGridBounds {
    /// Arena index of the entity inside `GameWorld::entities`.
    pub(crate) entity_idx: u32,
    /// Inclusive minimum spatial-grid X cell.
    pub(crate) min_cx: i32,
    /// Inclusive maximum spatial-grid X cell.
    pub(crate) max_cx: i32,
    /// Inclusive minimum spatial-grid Y cell.
    pub(crate) min_cy: i32,
    /// Inclusive maximum spatial-grid Y cell.
    pub(crate) max_cy: i32,
    /// Collision layer bitmask of the collider.
    pub(crate) layer: u32,
    /// Collision target filter bitmask of the collider.
    pub(crate) mask: u32,
}

/// Sparse open-addressed spatial-hash cell bucket used when world bounds exceed the dense flat grid limit.
#[derive(Clone, Copy, Debug)]
pub(crate) struct SparseGridCell {
    /// Packed `(cx, cy)` integer grid key (`u64::MAX` when empty).
    pub(crate) key: u64,
    /// Cell X coordinate.
    pub(crate) cx: i32,
    /// Cell Y coordinate.
    pub(crate) cy: i32,
    /// Head index into `PhysicsWorld2D::sparse_nodes` (`u32::MAX` when empty).
    pub(crate) head: u32,
    /// Number of colliders stored in this cell's linked list.
    pub(crate) count: u32,
}

/// Intrusive linked-list entry storing a `BodyGridBounds` slot index inside a sparse spatial-hash bucket.
#[derive(Clone, Copy, Debug)]
pub(crate) struct SparseGridNode {
    /// Index into `PhysicsWorld2D::body_bounds`.
    pub(crate) body_slot: u32,
    /// Next entry index in `PhysicsWorld2D::sparse_nodes` (`u32::MAX` at tail).
    pub(crate) next: u32,
}

/// Reusable zero-allocation flat spatial-grid broad-phase accelerator and 2D impulse physics solver.
#[derive(Clone, Data, Debug)]
pub struct PhysicsWorld2D {
    /// Active simulation parameters.
    #[get(type(copy))]
    pub(crate) config: PhysicsConfig,
    /// Reusable per-step collider cell bounds and layer masks.
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) body_bounds: Vec<BodyGridBounds>,
    /// Dense counting-sort cell occupancy counts (`len == grid_w * grid_h`).
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) cell_counts: Vec<u32>,
    /// Dense counting-sort cell start offsets (`len == grid_w * grid_h`).
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) cell_offsets: Vec<u32>,
    /// Dense counting-sort per-cell write cursors (`len == grid_w * grid_h`).
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) cell_cursors: Vec<u32>,
    /// Flat contiguous array of `body_bounds` indices grouped by cell.
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) cell_items: Vec<u32>,
    /// Dense cell indices with at least 2 colliders (`(cell_idx, cx, cy)`).
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) active_cells: Vec<(usize, i32, i32)>,
    /// Reusable sparse open-addressed cell table for ultra-wide worlds exceeding `FLAT_GRID_MAX_CELLS`.
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) sparse_table: Vec<SparseGridCell>,
    /// Reusable sparse linked-list node storage.
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) sparse_nodes: Vec<SparseGridNode>,
    /// Reusable list of occupied sparse table slots with `count >= 2`.
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) sparse_active: Vec<usize>,
    /// Scratch bucket collecting body indices of one sparse cell in deterministic order.
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) sparse_bucket_scratch: Vec<u32>,
    /// Most recent step telemetry.
    #[get(type(copy))]
    pub(crate) last_stats: PhysicsStepStats,
}
