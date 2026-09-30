/// The default name assigned to an entity when none is provided.
pub(crate) const DEFAULT_ENTITY_NAME: &str = "Entity";

/// The event channel name of the `EntityEvent::Collision` variant.
pub(crate) const ENTITY_EVENT_NAME_COLLISION: &str = "collision";

/// The event channel name of the `EntityEvent::TriggerEnter` variant.
pub(crate) const ENTITY_EVENT_NAME_TRIGGER_ENTER: &str = "trigger_enter";

/// The event channel name of the `EntityEvent::TriggerExit` variant.
pub(crate) const ENTITY_EVENT_NAME_TRIGGER_EXIT: &str = "trigger_exit";

/// The event channel name of the `EntityEvent::Spawn` variant.
pub(crate) const ENTITY_EVENT_NAME_SPAWN: &str = "spawn";

/// The event channel name of the `EntityEvent::Destroy` variant.
pub(crate) const ENTITY_EVENT_NAME_DESTROY: &str = "destroy";
