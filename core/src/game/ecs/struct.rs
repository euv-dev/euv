use super::*;

/// Generation-checked O(1) handle referencing an entity inside `GameWorld`.
#[derive(Clone, Copy, Data, Debug, Default, Eq, Hash, New, PartialEq)]
pub struct EntityId {
    /// Dense slot index in the entity arena.
    #[get(type(copy))]
    pub(crate) index: u32,
    /// Generation counter rejecting stale handles after despawn and reuse.
    #[get(type(copy))]
    pub(crate) generation: u32,
}

/// 2D spatial transform component in world coordinates.
#[derive(Clone, Copy, Data, Debug, New, PartialEq)]
pub struct EntityTransform {
    /// World-space center position.
    #[get(type(copy))]
    pub(crate) position: Point,
    /// Orientation angle in radians (`0.0` points right along `+X`).
    #[get(type(copy))]
    pub(crate) rotation_rad: f32,
    /// Non-uniform scale factors `(sx, sy)`.
    #[get(type(copy))]
    pub(crate) scale: Size,
}

/// Linear and angular velocity component in world units per second.
#[derive(Clone, Copy, Data, Debug, Default, New, PartialEq)]
pub struct Velocity2D {
    /// Horizontal velocity in world units per second.
    #[get(type(copy))]
    pub(crate) vx: f32,
    /// Vertical velocity in world units per second.
    #[get(type(copy))]
    pub(crate) vy: f32,
    /// Angular spin velocity in radians per second.
    #[get(type(copy))]
    pub(crate) angular: f32,
}

/// Rigid-body dynamics parameters for 2D physics simulation.
#[derive(Clone, Copy, Data, Debug, New, PartialEq)]
pub struct RigidBody2D {
    /// Body dynamics category (`Static`, `Dynamic`, or `Kinematic`).
    #[get(type(copy))]
    pub(crate) kind: BodyKind,
    /// Inverse mass (`0.0` for infinite-mass static/kinematic bodies).
    #[get(type(copy))]
    pub(crate) inv_mass: f32,
    /// Coefficient of restitution (`0.0` inelastic .. `1.0` perfectly elastic bounce).
    #[get(type(copy))]
    pub(crate) restitution: f32,
    /// Linear drag damping factor per second.
    #[get(type(copy))]
    pub(crate) linear_drag: f32,
    /// Multiplier applied to global world gravity.
    #[get(type(copy))]
    pub(crate) gravity_scale: f32,
}

/// Collision volume and bitmask filtering component.
#[derive(Clone, Copy, Data, Debug, New, PartialEq)]
pub struct Collider2D {
    /// Geometric collision shape.
    #[get(type(copy))]
    pub(crate) shape: ColliderShape,
    /// Layer bitmask this collider belongs to.
    #[get(type(copy))]
    pub(crate) layer: u32,
    /// Bitmask of layers this collider interacts with.
    #[get(type(copy))]
    pub(crate) mask: u32,
    /// When `true`, emits overlap events without applying physical impulse separation.
    #[get(type(copy))]
    pub(crate) is_trigger: bool,
}

/// Visual appearance component rendered by `GameRenderContext::render_world`.
#[derive(Clone, Copy, Data, Debug, New, PartialEq)]
pub struct Sprite2D {
    /// Render primitive shape.
    #[get(type(copy))]
    pub(crate) shape: EntityVisualShape,
    /// Logical world-space width and height.
    #[get(type(copy))]
    pub(crate) size: Size,
    /// Primary fill or tint color.
    #[get(type(copy))]
    pub(crate) color: Color,
    /// Optional accent outline or core glow color.
    #[get(type(copy))]
    pub(crate) accent: Color,
    /// Draw layer ordering (lower values render behind higher values).
    #[get(type(copy))]
    pub(crate) z_order: i32,
    /// Corner radius for `EntityVisualShape::Rect`.
    #[get(type(copy))]
    pub(crate) corner_radius: f32,
    /// Visibility toggle.
    #[get(type(copy))]
    pub(crate) visible: bool,
}

/// Complete entity record stored in the contiguous `GameWorld` arena.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct EntityRecord {
    /// Generational handle matching this slot.
    #[get(type(copy))]
    pub(crate) id: EntityId,
    /// Whether the entity slot is currently active.
    #[get(type(copy))]
    pub(crate) alive: bool,
    /// Gameplay tag category.
    #[get(type(copy))]
    pub(crate) tag: EntityTag,
    /// Spatial transform.
    #[get(type(copy))]
    pub(crate) transform: EntityTransform,
    /// Linear and angular velocity.
    #[get(type(copy))]
    pub(crate) velocity: Velocity2D,
    /// Optional rigid-body physics dynamics.
    #[get(skip)]
    pub(crate) body: Option<RigidBody2D>,
    /// Optional collision volume.
    #[get(skip)]
    pub(crate) collider: Option<Collider2D>,
    /// Optional visual sprite component.
    #[get(skip)]
    pub(crate) sprite: Option<Sprite2D>,
    /// Current hit points for combat/destructible entities.
    #[get(type(copy))]
    pub(crate) health: f32,
    /// Maximum hit points.
    #[get(type(copy))]
    pub(crate) max_health: f32,
    /// Remaining lifetime in seconds (`None` means immortal).
    #[get(skip)]
    pub(crate) ttl_secs: Option<f32>,
    /// Optional user-defined state payload integer (e.g., AI phase or score value).
    #[get(type(copy))]
    pub(crate) user_data: u64,
}

/// High-performance generational ECS arena managing game entities with O(1) spawn, despawn, and cache-friendly iteration.
#[derive(Clone, Data, Debug, Default)]
pub struct GameWorld {
    /// Contiguous slot array of entity records.
    pub(crate) entities: Vec<EntityRecord>,
    /// Free slot index stack for O(1) reuse without reallocation.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) free_list: Vec<u32>,
    /// Cached count of live entities.
    #[get(type(copy))]
    pub(crate) live_count: usize,
    /// Monotonic world tick counter.
    #[get(type(copy))]
    pub(crate) tick: u64,
}
