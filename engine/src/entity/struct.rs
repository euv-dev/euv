use super::*;

/// The fundamental entity in the engine, combining a transform with
/// a collection of behavior-defining components.
#[derive(Clone, Data, New)]
pub struct Entity {
    /// The unique identifier of this entity.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    pub(crate) id: u64,
    /// The human-readable name of this entity.
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) name: String,
    /// The world-space transform (position, rotation, scale).
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) transform: Transform2D,
    /// Whether this entity is active and should receive updates.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    pub(crate) active: bool,
    /// The list of components attached to this entity.
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) components: Vec<ComponentRc>,
    /// Optional tags for grouping and querying entities.
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) tags: Vec<String>,
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
