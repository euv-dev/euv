use super::*;

/// Implements static factory and ID generation methods for `Entity`.
impl Entity {
    /// Generates the next unique entity ID using a global atomic counter.
    ///
    /// # Returns
    ///
    /// - `u64` - The next unique ID.
    pub fn generate_id() -> u64 {
        NEXT_ENTITY_ID.fetch_add(1, Ordering::Relaxed)
    }

    /// Creates a new entity with the given name and a default identity transform.
    ///
    /// Creates a new entity with the given name and a default identity transform.
    ///
    /// # Arguments
    ///
    /// - `N: AsRef<str>` - The name of the entity.
    ///
    /// # Returns
    ///
    /// - `Entity` - The newly created entity.
    pub fn create<N>(name: N) -> Entity
    where
        N: AsRef<str>,
    {
        Entity::new(
            Self::generate_id(),
            name.as_ref().to_string(),
            Transform2D::identity(),
            true,
            Vec::new(),
            Vec::new(),
        )
    }

    /// Creates a new entity at the specified position with a default name.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The initial position.
    ///
    /// # Returns
    ///
    /// - `Entity` - The newly created entity.
    pub fn create_at(position: Vector2D) -> Entity {
        let mut entity: Entity = Self::create(DEFAULT_ENTITY_NAME);
        entity.get_mut_transform().set_position(position);
        entity
    }
}

/// Implements lifecycle and component management methods for `Entity`.
impl Entity {
    /// Adds a component to this entity and calls its `on_start` lifecycle method.
    ///
    /// # Arguments
    ///
    /// - `ComponentRc` - The component to add.
    pub fn add_component(&mut self, component: ComponentRc) {
        component.get_mut().on_start();
        self.get_mut_components().push(component);
    }

    /// Removes the first component matching the given name.
    ///
    /// # Arguments
    ///
    /// - `&str` - The component name to match.
    ///
    /// # Returns
    ///
    /// - `Option<ComponentRc>` - The removed component, if found.
    pub fn remove_component_by_name<N>(&mut self, name: N) -> Option<ComponentRc>
    where
        N: AsRef<str>,
    {
        let target: &str = name.as_ref();
        let position: Option<usize> = self
            .get_components()
            .iter()
            .position(|component: &ComponentRc| component.get().name() == target);
        let index: usize = position?;
        let removed: ComponentRc = self.get_mut_components().remove(index);
        removed.get_mut().on_destroy();
        Some(removed)
    }

    /// Returns the first component matching the given name.
    ///
    /// # Arguments
    ///
    /// - `&str` - The component name to match.
    ///
    /// # Returns
    ///
    /// - `Option<ComponentRc>` - The matching component, if found.
    pub fn get_component_by_name<N>(&self, name: N) -> Option<ComponentRc>
    where
        N: AsRef<str>,
    {
        let target: &str = name.as_ref();
        self.get_components()
            .iter()
            .find(|component: &&ComponentRc| component.get().name() == target)
            .cloned()
    }

    /// Calls `on_update` on all active components.
    ///
    /// # Arguments
    ///
    /// - `f64` - The delta time in seconds.
    pub fn update(&mut self, delta_time: f64) {
        if !self.get_active() {
            return;
        }
        for component in self.get_components() {
            component.get_mut().on_update(delta_time);
        }
    }

    /// Calls `on_render` on all active components, recording into the draw list.
    ///
    /// # Arguments
    ///
    /// - `&mut DrawList` - The draw list to record commands into.
    pub fn render(&self, draw_list: &mut DrawList) {
        if !self.get_active() {
            return;
        }
        let transform: Transform2D = self.get_transform();
        for component in self.get_components() {
            component.get_mut().on_render(draw_list, &transform);
        }
    }

    /// Calls `on_destroy` on all components and clears the component list.
    pub fn destroy(&mut self) {
        for component in self.get_components() {
            component.get_mut().on_destroy();
        }
        self.get_mut_components().clear();
    }

    /// Adds a tag string to this entity.
    ///
    /// # Arguments
    ///
    /// - `String` - The tag to add.
    pub fn add_tag(&mut self, tag: String) {
        if !self.get_tags().contains(&tag) {
            self.get_mut_tags().push(tag);
        }
    }

    /// Tests whether this entity has the given tag.
    ///
    /// # Arguments
    ///
    /// - `&str` - The tag to check.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the tag is present.
    pub fn has_tag<T>(&self, tag: T) -> bool
    where
        T: AsRef<str>,
    {
        let target: &str = tag.as_ref();
        self.get_tags().iter().any(|t: &String| t == target)
    }
}

/// Forwards `Entity::update` through the [`Updatable`] trait so that collections
/// of heterogeneous updateable objects can be driven by the scheduler.
///
/// The inherent [`Entity::update`] method is the canonical implementation;
/// this impl exists purely for trait dispatch. The inherent call resolves
/// first when both are in scope, so there is no recursion.
impl Updatable for Entity {
    /// Advances the simulation by `delta_time` seconds.
    ///
    /// # Arguments
    ///
    /// - `f64` - Seconds elapsed since the previous update.
    fn update(&mut self, delta_time: f64) {
        Entity::update(self, delta_time);
    }
}

/// Implements event subscription, emission, and management for `EventBus`.
impl EventBus {
    /// Creates a new empty event bus.
    ///
    /// # Returns
    ///
    /// - `EventBus` - The new event bus.
    pub fn create() -> EventBus {
        EventBus::new()
    }

    /// Subscribes a handler to the named event channel.
    ///
    /// # Arguments
    ///
    /// - `String` - The event name to subscribe to.
    /// - `EventHandler` - The handler closure to call when the event is emitted.
    pub fn subscribe(&mut self, event_name: String, handler: EventHandler) {
        self.get_mut_handlers()
            .entry(event_name)
            .or_default()
            .push(handler);
    }

    /// Emits an event to all handlers subscribed to the matching channel.
    ///
    /// The event name is derived from the `EntityEvent` variant.
    ///
    /// # Arguments
    ///
    /// - `&EntityEvent` - The event to emit.
    pub fn emit(&self, event: &EntityEvent) {
        let event_name: &str = Self::event_name(event);
        if let Some(handlers) = self.get_handlers().get(event_name) {
            for handler in handlers {
                handler(event);
            }
        }
    }

    /// Removes all handlers for the named event channel.
    ///
    /// # Arguments
    ///
    /// - `E: AsRef<str>` - The event name to clear.
    pub fn unsubscribe_all<E>(&mut self, event_name: E)
    where
        E: AsRef<str>,
    {
        self.get_mut_handlers().remove(event_name.as_ref());
    }

    /// Returns the number of handlers registered for the named event.
    ///
    /// # Arguments
    ///
    /// - `&str` - The event name.
    ///
    /// # Returns
    ///
    /// - `usize` - The handler count.
    pub fn handler_count<E>(&self, event_name: E) -> usize
    where
        E: AsRef<str>,
    {
        self.get_handlers()
            .get(event_name.as_ref())
            .map(|handlers: &Vec<EventHandler>| handlers.len())
            .unwrap_or_default()
    }

    /// Derives the event channel name from an `EntityEvent` variant.
    ///
    /// # Arguments
    ///
    /// - `&EntityEvent` - The event.
    ///
    /// # Returns
    ///
    /// - `&str` - The channel name (borrowed — no allocation per emit;
    ///   the `Custom` variant borrows its `name` field).
    fn event_name(event: &EntityEvent) -> &str {
        match event {
            EntityEvent::Collision { .. } => "collision",
            EntityEvent::TriggerEnter { .. } => "trigger_enter",
            EntityEvent::TriggerExit { .. } => "trigger_exit",
            EntityEvent::Spawn => "spawn",
            EntityEvent::Destroy => "destroy",
            EntityEvent::Custom { name, .. } => name.as_str(),
        }
    }
}

/// Implements `Default` for `EventBus` as a new empty bus.
impl Default for EventBus {
    /// Constructs a default [`EventBus`] value.
    ///
    /// # Returns
    ///
    /// - `EventBus` - A default-constructed instance with the documented initial state.
    fn default() -> EventBus {
        EventBus::create()
    }
}
