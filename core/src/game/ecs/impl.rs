use super::*;

/// Default identity transform at world origin.
impl Default for EntityTransform {
    /// Creates an identity `EntityTransform` at `(0, 0)` with unit scale.
    ///
    /// # Returns
    ///
    /// - `Self` - Identity transform.
    fn default() -> Self {
        Self {
            position: Point::new(0.0, 0.0),
            rotation_rad: 0.0,
            scale: Size::new(1.0, 1.0),
        }
    }
}

/// Convenience constructors for `RigidBody2D`.
impl RigidBody2D {
    /// Creates a dynamic rigid body with `mass`, `restitution`, and `linear_drag`.
    ///
    /// # Arguments
    ///
    /// - `f32` - Positive mass in kilograms.
    /// - `f32` - Restitution bounciness in `[0.0, 1.0]`.
    /// - `f32` - Linear velocity drag per second.
    ///
    /// # Returns
    ///
    /// - `Self` - Dynamic rigid body component.
    #[must_use]
    pub fn dynamic(mass: f32, restitution: f32, linear_drag: f32) -> Self {
        Self {
            kind: BodyKind::Dynamic,
            inv_mass: 1.0 / mass.max(0.001),
            restitution: restitution.clamp(0.0, 1.0),
            linear_drag: linear_drag.max(0.0),
            gravity_scale: 1.0,
        }
    }

    /// Creates an immovable static body with zero inverse mass.
    ///
    /// # Arguments
    ///
    /// - `f32` - Surface restitution bounciness in `[0.0, 1.0]`.
    ///
    /// # Returns
    ///
    /// - `Self` - Static rigid body component.
    #[must_use]
    pub fn static_body(restitution: f32) -> Self {
        Self {
            kind: BodyKind::Static,
            inv_mass: 0.0,
            restitution: restitution.clamp(0.0, 1.0),
            linear_drag: 0.0,
            gravity_scale: 0.0,
        }
    }

    /// Creates a kinematic body driven directly by velocity without impulse reaction.
    ///
    /// # Returns
    ///
    /// - `Self` - Kinematic rigid body component.
    #[must_use]
    pub fn kinematic() -> Self {
        Self {
            kind: BodyKind::Kinematic,
            inv_mass: 0.0,
            restitution: 0.5,
            linear_drag: 0.0,
            gravity_scale: 0.0,
        }
    }
}

/// World-space bounding box queries for `Collider2D`.
impl Collider2D {
    /// Creates a solid circle collider on layer `layer` colliding with `mask`.
    ///
    /// # Arguments
    ///
    /// - `f32` - Circle radius in world units.
    /// - `u32` - Collision layer bitmask.
    /// - `u32` - Interaction mask.
    /// - `bool` - Whether this collider acts as a trigger volume.
    ///
    /// # Returns
    ///
    /// - `Self` - Configured circle collider.
    #[must_use]
    pub fn circle(radius: f32, layer: u32, mask: u32, is_trigger: bool) -> Self {
        Self {
            shape: ColliderShape::Circle {
                radius: radius.max(0.5),
            },
            layer,
            mask,
            is_trigger,
        }
    }

    /// Creates an axis-aligned box collider on layer `layer` colliding with `mask`.
    ///
    /// # Arguments
    ///
    /// - `f32` - Full box width in world units.
    /// - `f32` - Full box height in world units.
    /// - `u32` - Collision layer bitmask.
    /// - `u32` - Interaction mask.
    /// - `bool` - Whether this collider acts as a trigger volume.
    ///
    /// # Returns
    ///
    /// - `Self` - Configured AABB collider.
    #[must_use]
    pub fn aabb(width: f32, height: f32, layer: u32, mask: u32, is_trigger: bool) -> Self {
        Self {
            shape: ColliderShape::Aabb {
                width: width.max(1.0),
                height: height.max(1.0),
            },
            layer,
            mask,
            is_trigger,
        }
    }

    /// Computes the world-space axis-aligned bounding rectangle of this collider centered at `center`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Collider reference.
    /// - `Point` - Entity world center position.
    ///
    /// # Returns
    ///
    /// - `Rect` - World-space AABB rectangle.
    #[must_use]
    pub fn world_aabb(&self, center: Point) -> Rect {
        match self.get_shape() {
            ColliderShape::Aabb { width, height } => Rect::new(
                center.get_x() - width * 0.5,
                center.get_y() - height * 0.5,
                width,
                height,
            ),
            ColliderShape::Circle { radius } => Rect::new(
                center.get_x() - radius,
                center.get_y() - radius,
                radius * 2.0,
                radius * 2.0,
            ),
        }
    }
}

/// Optional component accessors for `EntityRecord`.
impl EntityRecord {
    /// Returns the attached `RigidBody2D` component if present.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Entity record reference.
    ///
    /// # Returns
    ///
    /// - `Option<RigidBody2D>` - Rigid-body component value.
    #[must_use]
    pub fn body_component(&self) -> Option<RigidBody2D> {
        let Self { body, .. } = self;
        *body
    }

    /// Returns the attached `Collider2D` component if present.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Entity record reference.
    ///
    /// # Returns
    ///
    /// - `Option<Collider2D>` - Collider component value.
    #[must_use]
    pub fn collider_component(&self) -> Option<Collider2D> {
        let Self { collider, .. } = self;
        *collider
    }

    /// Returns the attached `Sprite2D` component if present.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Entity record reference.
    ///
    /// # Returns
    ///
    /// - `Option<Sprite2D>` - Sprite component value.
    #[must_use]
    pub fn sprite_component(&self) -> Option<Sprite2D> {
        let Self { sprite, .. } = self;
        *sprite
    }

    /// Returns remaining lifetime in seconds if the entity is time-limited.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Entity record reference.
    ///
    /// # Returns
    ///
    /// - `Option<f32>` - Remaining TTL seconds.
    #[must_use]
    pub fn ttl_value(&self) -> Option<f32> {
        let Self { ttl_secs, .. } = self;
        *ttl_secs
    }

    /// Computes the world-space visual bounding box of this entity.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Entity record reference.
    ///
    /// # Returns
    ///
    /// - `Rect` - World-space bounding rectangle.
    #[must_use]
    pub fn world_bounds(&self) -> Rect {
        let pos: Point = self.get_transform().get_position();
        if let Some(sp) = self.sprite_component() {
            let w: f32 = sp.get_size().get_width() * self.get_transform().get_scale().get_width();
            let h: f32 = sp.get_size().get_height() * self.get_transform().get_scale().get_height();
            Rect::new(pos.get_x() - w * 0.5, pos.get_y() - h * 0.5, w, h)
        } else if let Some(col) = self.collider_component() {
            col.world_aabb(pos)
        } else {
            Rect::new(pos.get_x(), pos.get_y(), 1.0, 1.0)
        }
    }
}

/// Entity spawning, recycling, querying, and lifetime ticking for `GameWorld`.
impl GameWorld {
    /// Allocates a `GameWorld` pre-sized for `capacity` entities with zero initial allocations during play.
    ///
    /// # Arguments
    ///
    /// - `usize` - Initial entity slot capacity.
    ///
    /// # Returns
    ///
    /// - `Self` - Pre-allocated game world.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            entities: Vec::with_capacity(capacity),
            free_list: Vec::with_capacity(capacity / 4),
            live_count: 0,
            tick: 0,
        }
    }

    /// Spawns a new entity at `position` with `tag` and returns its generational `EntityId`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable game world.
    /// - `EntityTag` - Semantic gameplay tag.
    /// - `Point` - Initial world-space position.
    ///
    /// # Returns
    ///
    /// - `EntityId` - Generation-checked entity handle.
    pub fn spawn(&mut self, tag: EntityTag, position: Point) -> EntityId {
        let transform: EntityTransform = EntityTransform::new(position, 0.0, Size::new(1.0, 1.0));
        let id: EntityId = if let Some(index) = self.get_mut_free_list().pop() {
            let slot: &mut EntityRecord = &mut self.get_mut_entities()[index as usize];
            let next_gen: u32 = slot.get_id().get_generation().wrapping_add(1).max(1);
            let next_id: EntityId = EntityId::new(index, next_gen);
            *slot = EntityRecord {
                id: next_id,
                alive: true,
                tag,
                transform,
                velocity: Velocity2D::default(),
                body: None,
                collider: None,
                sprite: None,
                health: 100.0,
                max_health: 100.0,
                ttl_secs: None,
                user_data: 0,
            };
            next_id
        } else {
            let index: u32 = self.get_entities().len() as u32;
            let next_id: EntityId = EntityId::new(index, 1);
            self.get_mut_entities().push(EntityRecord {
                id: next_id,
                alive: true,
                tag,
                transform,
                velocity: Velocity2D::default(),
                body: None,
                collider: None,
                sprite: None,
                health: 100.0,
                max_health: 100.0,
                ttl_secs: None,
                user_data: 0,
            });
            next_id
        };
        let next_live: usize = self.get_live_count().saturating_add(1);
        self.set_live_count(next_live);
        id
    }

    /// Despawns `id` in O(1), invalidating stale handles and recycling its arena slot.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable game world.
    /// - `EntityId` - Target entity handle.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the entity was alive and removed.
    pub fn despawn(&mut self, id: EntityId) -> bool {
        let idx: usize = id.get_index() as usize;
        let removed: bool = match self.get_mut_entities().get_mut(idx) {
            Some(slot) if slot.get_alive() && slot.get_id() == id => {
                slot.set_alive(false);
                true
            }
            _ => false,
        };
        if removed {
            self.get_mut_free_list().push(id.get_index());
            let next_live: usize = self.get_live_count().saturating_sub(1);
            self.set_live_count(next_live);
        }
        removed
    }

    /// Returns `true` when `id` refers to a currently alive entity.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Game world reference.
    /// - `EntityId` - Entity handle to validate.
    ///
    /// # Returns
    ///
    /// - `bool` - Live status.
    #[must_use]
    pub fn is_alive(&self, id: EntityId) -> bool {
        self.get_entity(id).is_some()
    }

    /// Returns an immutable reference to the live entity matching `id`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Game world reference.
    /// - `EntityId` - Entity handle.
    ///
    /// # Returns
    ///
    /// - `Option<&EntityRecord>` - Live entity reference.
    #[must_use]
    pub fn get_entity(&self, id: EntityId) -> Option<&EntityRecord> {
        let slot: &EntityRecord = self.get_entities().get(id.get_index() as usize)?;
        if slot.get_alive() && slot.get_id() == id {
            Some(slot)
        } else {
            None
        }
    }

    /// Returns a mutable reference to the live entity matching `id`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable game world.
    /// - `EntityId` - Entity handle.
    ///
    /// # Returns
    ///
    /// - `Option<&mut EntityRecord>` - Mutable live entity reference.
    pub fn get_entity_mut(&mut self, id: EntityId) -> Option<&mut EntityRecord> {
        let slot: &mut EntityRecord = self.get_mut_entities().get_mut(id.get_index() as usize)?;
        if slot.get_alive() && slot.get_id() == id {
            Some(slot)
        } else {
            None
        }
    }

    /// Decrements `ttl_secs` across all time-limited entities and despawns expired ones.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable game world.
    /// - `f32` - Elapsed timestep in seconds.
    ///
    /// # Returns
    ///
    /// - `usize` - Number of entities expired and despawned.
    pub fn tick_lifetimes(&mut self, dt_secs: f32) -> usize {
        let mut expired: Vec<EntityId> = Vec::new();
        for slot in self.get_mut_entities().iter_mut() {
            if !slot.get_alive() {
                continue;
            }
            if let Some(rem) = slot.ttl_value() {
                let next: f32 = rem - dt_secs;
                if next <= 0.0 {
                    expired.push(slot.get_id());
                } else {
                    slot.set_ttl_secs(Some(next));
                }
            }
        }
        let mut removed: usize = 0;
        for id in expired.into_iter() {
            if self.despawn(id) {
                removed += 1;
            }
        }
        let next_tick: u64 = self.get_tick().wrapping_add(1);
        self.set_tick(next_tick);
        removed
    }

    /// Clears all entities while retaining arena backing allocations.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable game world.
    pub fn clear(&mut self) {
        self.get_mut_entities().clear();
        self.get_mut_free_list().clear();
        self.set_live_count(0);
    }

    /// Counts all live entities matching `tag`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Game world reference.
    /// - `EntityTag` - Target gameplay tag.
    ///
    /// # Returns
    ///
    /// - `usize` - Live entity count matching `tag`.
    #[must_use]
    pub fn count_by_tag(&self, tag: EntityTag) -> usize {
        self.get_entities()
            .iter()
            .filter(|e: &&EntityRecord| e.get_alive() && e.get_tag() == tag)
            .count()
    }
}
