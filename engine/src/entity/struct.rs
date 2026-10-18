use super::*;

/// The fundamental entity in the engine, combining a transform with
/// a collection of behavior-defining components.
#[derive(Clone, Data, New)]
pub struct Entity {
    /// The unique identifier of this entity.
    ///
    /// `get_id` is public because tests/entity/fn.rs, tests/pool/fn.rs reads this field from
    /// outside this crate. `get_mut` / `set` stay crate-internal.
    #[get_mut(pub(crate))]
    #[get(type(copy))]
    #[set(pub(crate))]
    pub id: u64,
    /// The human-readable name of this entity.
    ///
    /// `get_name` is public because tests/entity/fn.rs, tests/pool/fn.rs, tests/remaining/fn.rs reads this field from
    /// outside this crate. `get_mut` / `set` stay crate-internal.
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub name: String,
    /// The world-space transform (position, rotation, scale).
    ///
    /// `get_transform` is public because tests/entity/fn.rs, tests/pool/fn.rs reads this field from
    /// outside this crate. `get_mut` / `set` stay crate-internal.
    #[get_mut(pub(crate))]
    #[get(type(copy))]
    #[set(pub(crate))]
    pub transform: Transform2D,
    /// Whether this entity is active and should receive updates.
    ///
    /// `get_active` / `set_active` are public because tests/entity/fn.rs, tests/pool/fn.rs reads this field from
    /// outside this crate. `get_mut` / `set` stay crate-internal.
    #[get_mut(pub(crate))]
    #[get(type(copy))]
    pub active: bool,
    /// The list of components attached to this entity.
    ///
    /// `get_components` is public because tests/entity/fn.rs, tests/pool/fn.rs reads this field from
    /// outside this crate. `get_mut` / `set` stay crate-internal.
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub components: Vec<ComponentRc>,
    /// Optional tags for grouping and querying entities.
    ///
    /// `get_tags` is public because tests/entity/fn.rs, tests/pool/fn.rs reads this field from
    /// outside this crate. `get_mut` / `set` stay crate-internal.
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub tags: Vec<String>,
}

/// A publish-subscribe event bus for decoupled inter-entity communication.
///
/// Entities and game systems can subscribe to named event channels and
/// emit events that are dispatched to all registered handlers.
#[derive(Clone, Data, New)]
pub struct EventBus {
    /// The map from event name to the list of registered handler closures.
    #[get_mut(pub(crate))]
    #[new(skip)]
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) handlers: EventHandlers,
}
