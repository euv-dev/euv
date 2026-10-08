use super::*;

/// The result of a collision check between two colliders, containing
/// the contact normal and penetration depth if they overlap.
#[derive(Clone, Copy, Data, Debug, Default, New, PartialEq, PartialOrd)]
pub struct CollisionResult {
    /// The minimum translation vector (MTV) pointing from the first collider to the second.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) normal: Vector2D,
    /// The penetration depth (how far the colliders overlap along the normal).
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) depth: f64,
    /// The contact point where the collision occurs.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) contact_point: Vector2D,
}

/// An axis-aligned bounding box collider wrapping a `Rect`.
#[derive(Clone, Copy, Data, Debug, Default, New, PartialEq, PartialOrd)]
pub struct AabbCollider {
    /// The underlying rectangle defining the bounding box.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) rect: Rect,
}

/// A circle collider wrapping a `Circle`.
#[derive(Clone, Copy, Data, Debug, Default, New, PartialEq, PartialOrd)]
pub struct CircleCollider {
    /// The underlying circle defining the collider.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) circle: Circle,
}

/// The result of a collision check between two 3D colliders, containing
/// the contact normal and penetration depth if they overlap.
#[derive(Clone, Copy, Data, Debug, Default, New, PartialEq, PartialOrd)]
pub struct CollisionResult3D {
    /// The minimum translation vector (MTV) pointing from the first collider to the second.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) normal: Vector3D,
    /// The penetration depth (how far the colliders overlap along the normal).
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) depth: f64,
    /// The contact point where the collision occurs.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) contact_point: Vector3D,
}

/// A 3D axis-aligned bounding box collider wrapping an `AABB3D`.
#[derive(Clone, Copy, Data, Debug, Default, New, PartialEq, PartialOrd)]
pub struct AabbCollider3D {
    /// The underlying bounding box defining the collider.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) aabb: AABB3D,
}

/// A 3D sphere collider wrapping a `Sphere`.
#[derive(Clone, Copy, Data, Debug, Default, New, PartialEq, PartialOrd)]
pub struct SphereCollider3D {
    /// The underlying sphere defining the collider.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) sphere: Sphere,
}
