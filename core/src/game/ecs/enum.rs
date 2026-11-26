use super::*;

/// Dynamics classification for a 2D physics body inside `GameWorld`.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum BodyKind {
    /// Immovable static geometry such as arena walls, platforms, and tiles.
    Static,
    /// Full rigid body integrated by forces, gravity, drag, and collision impulses.
    #[default]
    Dynamic,
    /// Script-driven body whose velocity moves it without receiving external collision impulses.
    Kinematic,
}

/// Collision volume shape attached to a game entity.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ColliderShape {
    /// Axis-aligned bounding box defined by full `(width, height)` centered on entity position.
    Aabb {
        /// Full width in world units.
        width: f32,
        /// Full height in world units.
        height: f32,
    },
    /// Circle collider defined by `radius` centered on entity position.
    Circle {
        /// Radius in world units.
        radius: f32,
    },
}

/// Visual primitive rendered for an entity when no sprite-sheet buffer is attached.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum EntityVisualShape {
    /// Axis-aligned or rounded rectangle.
    #[default]
    Rect,
    /// Smooth anti-aliased filled circle.
    Circle,
    /// Directional triangle pointing along `rotation_rad`.
    Triangle,
    /// Diamond / rotated kite silhouette.
    Diamond,
}

/// Semantic gameplay tag used for fast collision filtering and query grouping.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum EntityTag {
    /// Untyped generic game object.
    #[default]
    Generic,
    /// Player-controlled avatar.
    Player,
    /// Hostile AI agent or boss.
    Enemy,
    /// Player or enemy projectile.
    Projectile,
    /// Collectible pickup or power-up.
    Pickup,
    /// Arena wall or obstacle.
    Obstacle,
    /// Visual decoration or effect anchor.
    Effect,
}
