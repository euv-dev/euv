use super::*;

/// Configuration parameters for the physics world simulation.
#[derive(Clone, Copy, Data, Debug, New, PartialEq, PartialOrd)]
pub struct PhysicsConfig {
    /// The gravitational acceleration vector in pixels per second squared.
    #[get(type(copy))]
    pub(crate) gravity: Vector2D,
    /// The linear velocity damping coefficient applied per second.
    #[get(type(copy))]
    pub(crate) linear_damping: f64,
    /// The angular velocity damping coefficient applied per second.
    #[get(type(copy))]
    pub(crate) angular_damping: f64,
}

/// A 2D rigid body participating in the physics simulation.
#[derive(Clone, Data, Debug, New, PartialEq)]
pub struct RigidBody2D {
    /// The unique identifier of this body, typically matching a game object ID.
    #[get(type(copy))]
    pub(crate) id: u64,
    /// The world-space position of the body's center.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    pub(crate) position: Vector2D,
    /// The linear velocity in pixels per second.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[new(skip)]
    pub(crate) velocity: Vector2D,
    /// The accumulated force to be applied during the next physics step.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    pub(crate) force_accumulator: Vector2D,
    /// The rotation angle in radians.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[new(skip)]
    pub(crate) rotation: f64,
    /// The angular velocity in radians per second.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[new(skip)]
    pub(crate) angular_velocity: f64,
    /// The mass of the body in kilograms. Static bodies have a mass of 0.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    pub(crate) mass: f64,
    /// The precomputed inverse mass (1/mass). Static bodies have 0 inverse mass.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) inverse_mass: f64,
    /// The restitution (bounciness) coefficient in the range 0.0 to 1.0.
    #[get(type(copy))]
    pub(crate) restitution: f64,
    /// The friction coefficient for surface contact.
    #[get(type(copy))]
    pub(crate) friction: f64,
    /// How this body participates in the simulation.
    #[get(type(copy))]
    pub(crate) body_type: BodyType,
    /// The collider shape attached to this body.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) collider: Option<BodyCollider>,
}

/// The physics world managing all rigid bodies and simulation steps.
#[derive(Clone, Data, Debug, New, PartialEq)]
pub struct PhysicsWorld2D {
    /// All rigid bodies in the world.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    pub(crate) bodies: Vec<RigidBody2D>,
    /// The simulation configuration.
    #[get(type(copy))]
    pub(crate) config: PhysicsConfig,
    /// The persistent broad-phase grid, cleared and re-inserted each step so its
    /// `HashMap` allocation is reused instead of rebuilt per step.
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    pub(crate) grid: SpatialHashGrid2D,
    /// Scratch buffer reused by grid queries to avoid a per-query `Vec` allocation.
    #[get_mut(pub(crate))]
    #[new(skip)]
    pub(crate) query_buffer: Vec<usize>,
    /// Scratch `HashSet` reused by grid queries for candidate dedup.
    #[get_mut(pub(crate))]
    #[new(skip)]
    pub(crate) query_seen: HashSet<usize>,
    /// Reusable broad-phase candidate pair list, rebuilt once per step and
    /// iterated by every solver iteration (the grid is unchanged between them).
    #[get_mut(pub(crate))]
    #[new(skip)]
    pub(crate) pair_buffer: Vec<(usize, usize)>,
}

/// Configuration parameters for the 3D physics world simulation.
#[derive(Clone, Copy, Data, Debug, New, PartialEq, PartialOrd)]
pub struct PhysicsConfig3D {
    /// The gravitational acceleration vector in meters per second squared.
    #[get(type(copy))]
    pub(crate) gravity: Vector3D,
    /// The linear velocity damping coefficient applied per second.
    #[get(type(copy))]
    pub(crate) linear_damping: f64,
    /// The angular velocity damping coefficient applied per second.
    #[get(type(copy))]
    pub(crate) angular_damping: f64,
}

/// A 3D rigid body participating in the physics simulation.
#[derive(Clone, Data, Debug, New, PartialEq)]
pub struct RigidBody3D {
    /// The unique identifier of this body, typically matching a game object ID.
    #[get(type(copy))]
    pub(crate) id: u64,
    /// The world-space position of the body's center.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    pub(crate) position: Vector3D,
    /// The linear velocity in meters per second.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[new(skip)]
    pub(crate) velocity: Vector3D,
    /// The accumulated force to be applied during the next physics step.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    pub(crate) force_accumulator: Vector3D,
    /// The rotation as a quaternion.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[new(skip)]
    pub(crate) rotation: Quaternion,
    /// The angular velocity as a 3D vector (axis * speed).
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[new(skip)]
    pub(crate) angular_velocity: Vector3D,
    /// The accumulated torque to be applied during the next physics step.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    pub(crate) torque_accumulator: Vector3D,
    /// The mass of the body in kilograms. Static bodies have a mass of 0.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    pub(crate) mass: f64,
    /// The precomputed inverse mass (1/mass). Static bodies have 0 inverse mass.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) inverse_mass: f64,
    /// The precomputed inverse moment of inertia (scalar, axis-aligned). Static
    /// bodies have 0 inverse inertia. Used to convert accumulated torque into
    /// angular velocity each step.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    pub(crate) inverse_inertia: f64,
    /// The restitution (bounciness) coefficient in the range 0.0 to 1.0.
    #[get(type(copy))]
    pub(crate) restitution: f64,
    /// The friction coefficient for surface contact.
    #[get(type(copy))]
    pub(crate) friction: f64,
    /// How this body participates in the simulation.
    #[get(type(copy))]
    pub(crate) body_type: BodyType,
    /// The 3D collider shape attached to this body.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) collider: Option<BodyCollider3D>,
}

/// The 3D physics world managing all rigid bodies and simulation steps.
#[derive(Clone, Data, Debug, New, PartialEq)]
pub struct PhysicsWorld3D {
    /// All rigid bodies in the world.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    pub(crate) bodies: Vec<RigidBody3D>,
    /// The simulation configuration.
    #[get(type(copy))]
    pub(crate) config: PhysicsConfig3D,
    /// The persistent broad-phase grid, cleared and re-inserted each step so its
    /// `HashMap` allocation is reused instead of rebuilt per step.
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    pub(crate) grid: SpatialHashGrid3D,
    /// Scratch buffer reused by grid queries to avoid a per-query `Vec` allocation.
    #[get_mut(pub(crate))]
    #[new(skip)]
    pub(crate) query_buffer: Vec<usize>,
    /// Scratch `HashSet` reused by grid queries for candidate dedup.
    #[get_mut(pub(crate))]
    #[new(skip)]
    pub(crate) query_seen: HashSet<usize>,
    /// Reusable broad-phase candidate pair list, rebuilt once per step and
    /// iterated by every solver iteration (the grid is unchanged between them).
    #[get_mut(pub(crate))]
    #[new(skip)]
    pub(crate) pair_buffer: Vec<(usize, usize)>,
    /// OPT 33: reusable scratch vector for per-body AABB3D snapshots taken
    /// before draining the spatial grid. The 3D `resolve_collisions` path
    /// needs to materialise bbox bounds into a Vec to satisfy the
    /// immutable-then-mutable borrow split; persisting the buffer avoids a
    /// per-step `Vec::with_capacity(bodies.len())` allocation.
    #[get_mut(pub(crate))]
    #[new(skip)]
    pub(crate) bbox_buffer: Vec<(usize, AABB3D)>,
}
