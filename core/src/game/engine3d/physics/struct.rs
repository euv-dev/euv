use super::*;

/// Dynamic or static 3D rigid body with axis-aligned box collider and physical restitution/friction.
#[derive(Clone, Copy, Data, Debug, PartialEq)]
pub struct RigidBody3D {
    /// Center position in world meters.
    #[get(type(copy))]
    pub(crate) position: Vec3,
    /// Linear velocity in meters per second.
    #[get(type(copy))]
    pub(crate) velocity: Vec3,
    /// Axis-aligned box collider half-extents `(hx, hy, hz)`.
    #[get(type(copy))]
    pub(crate) half_extents: Vec3,
    /// Inverse mass (`0.0` = immovable static collider).
    #[get(type(copy))]
    pub(crate) inv_mass: f32,
    /// Coefficient of restitution (`0.0` = inelastic, `1.0` = perfectly bouncy).
    #[get(type(copy))]
    pub(crate) restitution: f32,
    /// Linear velocity damping factor per second (`0.0..=1.0`).
    #[get(type(copy))]
    pub(crate) linear_damping: f32,
    /// Whether this body is currently resting on the ground or another solid surface.
    #[get(type(copy))]
    pub(crate) grounded: bool,
}

/// Result of a 3D physics raycast against `PhysicsWorld3D` rigid bodies.
#[derive(Clone, Copy, Data, Debug, New, PartialEq)]
pub struct RayHitPhysics3D {
    /// Index of the hit `RigidBody3D`.
    #[get(type(copy))]
    pub(crate) body_index: usize,
    /// Hit distance along the query ray in meters.
    #[get(type(copy))]
    pub(crate) distance: f32,
    /// World-space contact point.
    #[get(type(copy))]
    pub(crate) point: Vec3,
}

/// Deterministic 3D rigid-body physics world with gravity, ground plane, impulse resolution, and raycasting.
#[derive(Clone, Data, Debug)]
pub struct PhysicsWorld3D {
    /// Uniform gravitational acceleration vector (defaults to `(0.0, -9.81, 0.0)`).
    #[get(type(copy))]
    pub(crate) gravity: Vec3,
    /// Horizontal ground plane height in meters (`y = ground_y`).
    #[get(type(copy))]
    pub(crate) ground_y: f32,
    /// Simulated 3D rigid bodies.
    pub(crate) bodies: Vec<RigidBody3D>,
}
