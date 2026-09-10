use super::*;

/// Implements `Default` for `BodyCollider`, returning an AABB collider with default values.
impl Default for BodyCollider {
    /// Constructs a default [`BodyCollider`] value.
    ///
    /// # Returns
    ///
    /// - `BodyCollider` - A default-constructed instance with the documented initial state.
    fn default() -> BodyCollider {
        BodyCollider::Aabb(AabbCollider::default())
    }
}

/// Implements `Default` for `BodyCollider3D`, returning a 3D AABB collider with default values.
impl Default for BodyCollider3D {
    /// Constructs a default [`BodyCollider3D`] value.
    ///
    /// # Returns
    ///
    /// - `BodyCollider3D` - A default-constructed instance with the documented initial state.
    fn default() -> BodyCollider3D {
        BodyCollider3D::Aabb(AabbCollider3D::default())
    }
}

/// Implements default configuration for `PhysicsConfig`.
impl Default for PhysicsConfig {
    /// Constructs a default [`PhysicsConfig`] value.
    ///
    /// # Returns
    ///
    /// - `PhysicsConfig` - A default-constructed instance with the documented initial state.
    fn default() -> PhysicsConfig {
        PhysicsConfig::new(
            Vector2D::new(0.0, DEFAULT_GRAVITY),
            DEFAULT_LINEAR_DAMPING,
            DEFAULT_ANGULAR_DAMPING,
        )
    }
}

/// Implements body creation and force management for `RigidBody2D`.
impl RigidBody2D {
    /// Creates a new dynamic rigid body with default mass and the given position.
    ///
    /// # Arguments
    ///
    /// - `u64` - The unique ID.
    /// - `Vector2D` - The initial position.
    ///
    /// # Returns
    ///
    /// - `RigidBody2D` - The new body.
    pub fn new_dynamic(id: u64, position: Vector2D) -> RigidBody2D {
        let mass: f64 = PHYSICS_DEFAULT_MASS;
        RigidBody2D::new(
            id,
            position,
            mass,
            1.0 / mass,
            DEFAULT_RESTITUTION,
            DEFAULT_FRICTION,
            BodyType::Dynamic,
        )
    }

    /// Creates a new static rigid body at the given position with infinite mass.
    ///
    /// # Arguments
    ///
    /// - `u64` - The unique ID.
    /// - `Vector2D` - The position.
    ///
    /// # Returns
    ///
    /// - `RigidBody2D` - The new static body.
    pub fn new_static(id: u64, position: Vector2D) -> RigidBody2D {
        RigidBody2D::new(
            id,
            position,
            PHYSICS_STATIC_MASS,
            0.0,
            DEFAULT_RESTITUTION,
            DEFAULT_FRICTION,
            BodyType::Static,
        )
    }

    /// Applies a force to the body's force accumulator.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The force vector.
    pub fn apply_force(&mut self, force: Vector2D) {
        *self.get_mut_force_accumulator() += force;
    }

    /// Applies an instantaneous impulse, directly changing velocity.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The impulse vector.
    pub fn apply_impulse(&mut self, impulse: Vector2D) {
        let inverse_mass: f64 = self.get_inverse_mass();
        if inverse_mass == 0.0 {
            return;
        }
        *self.get_mut_velocity() += impulse.scaled(inverse_mass);
    }

    /// Sets the mass of the body, updating the inverse mass.
    /// A mass of 0 makes the body static (infinite mass).
    ///
    /// # Arguments
    ///
    /// - `f64` - The new mass.
    pub fn update_mass(&mut self, mass: f64) {
        self.set_mass(mass);
        self.set_inverse_mass(if mass > 0.0 { 1.0 / mass } else { 0.0 });
    }

    /// Returns `true` if this body is affected by forces and collisions.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the body is dynamic.
    pub fn is_dynamic(&self) -> bool {
        self.get_body_type() == BodyType::Dynamic
    }

    /// Attaches a collider shape to this body.
    ///
    /// # Arguments
    ///
    /// - `BodyCollider` - The collider to attach.
    pub fn update_collider(&mut self, collider: BodyCollider) {
        self.set_collider(Some(collider));
    }

    /// Returns the world-space bounding box of the attached collider, if any.
    ///
    /// # Returns
    ///
    /// - `Option<Rect>` - The bounding box, or `None` if no collider is attached.
    pub fn bounding_box(&self) -> Option<Rect> {
        let collider: Option<BodyCollider> = self.get_collider();
        match collider? {
            BodyCollider::Aabb(aabb) => {
                let aabb_rect: Rect = aabb.get_rect();
                let mut offset_rect: Rect = aabb_rect;
                offset_rect.set_x(
                    offset_rect.get_x() + self.get_position().get_x() - aabb_rect.get_width() * 0.5,
                );
                offset_rect.set_y(
                    offset_rect.get_y() + self.get_position().get_y()
                        - aabb_rect.get_height() * 0.5,
                );
                Some(offset_rect)
            }
            BodyCollider::Circle(circle) => {
                let diameter: f64 = circle.get_circle().get_radius() * 2.0;
                Some(Rect::from_center(self.get_position(), diameter, diameter))
            }
        }
    }
}

/// Implements body management and simulation for `PhysicsWorld2D`.
impl PhysicsWorld2D {
    /// Creates a new physics world with the given configuration.
    ///
    /// # Arguments
    ///
    /// - `PhysicsConfig` - The simulation configuration.
    ///
    /// # Returns
    ///
    /// - `PhysicsWorld2D` - The new world.
    pub fn with_config(config: PhysicsConfig) -> PhysicsWorld2D {
        let mut world: PhysicsWorld2D = PhysicsWorld2D::new(config);
        world.set_grid(SpatialHashGrid2D::with_default_size());
        world
    }

    /// Adds a rigid body to the world.
    ///
    /// # Arguments
    ///
    /// - `RigidBody2D` - The body to add.
    pub fn add_body(&mut self, body: RigidBody2D) {
        self.get_mut_bodies().push(body);
    }

    /// Removes the body with the given ID.
    ///
    /// # Arguments
    ///
    /// - `u64` - The ID of the body to remove.
    pub fn remove_body(&mut self, id: u64) {
        self.get_mut_bodies()
            .retain(|body: &RigidBody2D| body.get_id() != id);
    }

    /// Returns a reference to the body with the given ID.
    ///
    /// # Arguments
    ///
    /// - `u64` - The body ID.
    ///
    /// # Returns
    ///
    /// - `Option<&RigidBody2D>` - The body reference, if found.
    pub fn get_body(&self, id: u64) -> Option<&RigidBody2D> {
        self.get_bodies()
            .iter()
            .find(|body: &&RigidBody2D| body.get_id() == id)
    }

    /// Returns a mutable reference to the body with the given ID.
    ///
    /// # Arguments
    ///
    /// - `u64` - The body ID.
    ///
    /// # Returns
    ///
    /// - `Option<&mut RigidBody2D>` - The mutable body reference, if found.
    pub fn get_body_mut(&mut self, id: u64) -> Option<&mut RigidBody2D> {
        self.get_mut_bodies()
            .iter_mut()
            .find(|body: &&mut RigidBody2D| body.get_id() == id)
    }
}

/// Implements `Default` for `PhysicsWorld2D` as an empty world.
impl Default for PhysicsWorld2D {
    /// Constructs a default [`PhysicsWorld2D`] value.
    ///
    /// # Returns
    ///
    /// - `PhysicsWorld2D` - A default-constructed instance with the documented initial state.
    fn default() -> PhysicsWorld2D {
        PhysicsWorld2D::with_config(PhysicsConfig::default())
    }
}

/// Implements collision detection and resolution for `RigidBody2D`.
impl RigidBody2D {
    /// Checks collision with another body based on both bodies' collider shapes.
    ///
    /// # Arguments
    ///
    /// - `&RigidBody2D` - The other body to check against.
    ///
    /// # Returns
    ///
    /// - `Option<CollisionResult>` - The collision result, or `None`.
    fn check_collision_with(&self, other: &RigidBody2D) -> Option<CollisionResult> {
        let a_bbox: Rect = self.bounding_box()?;
        let b_bbox: Rect = other.bounding_box()?;
        if !Rect::broad_phase_alias(a_bbox, b_bbox) {
            return None;
        }
        let self_collider: Option<BodyCollider> = self.get_collider();
        let other_collider: Option<BodyCollider> = other.get_collider();
        let position_delta: Vector2D = other.get_position() - self.get_position();
        match (self_collider, other_collider) {
            (Some(BodyCollider::Aabb(aabb_a)), Some(BodyCollider::Aabb(aabb_b))) => {
                let aabb_b_rect: Rect = aabb_b.get_rect();
                let offset_aabb_b: AabbCollider = AabbCollider::new(Rect::new(
                    aabb_b_rect.get_x() + position_delta.get_x(),
                    aabb_b_rect.get_y() + position_delta.get_y(),
                    aabb_b_rect.get_width(),
                    aabb_b_rect.get_height(),
                ));
                aabb_a.collide_with_aabb(&offset_aabb_b)
            }
            (Some(BodyCollider::Circle(circle_a)), Some(BodyCollider::Circle(circle_b))) => {
                let circle_b_inner: Circle = circle_b.get_circle();
                let offset_circle_b: CircleCollider = CircleCollider::new(Circle::new(
                    circle_b_inner.get_center() + position_delta,
                    circle_b_inner.get_radius(),
                ));
                circle_a.collide_with_circle(&offset_circle_b)
            }
            (Some(BodyCollider::Aabb(aabb)), Some(BodyCollider::Circle(circle))) => {
                let circle_inner: Circle = circle.get_circle();
                let offset_circle: CircleCollider = CircleCollider::new(Circle::new(
                    circle_inner.get_center() + position_delta,
                    circle_inner.get_radius(),
                ));
                aabb.collide_with_circle(&offset_circle)
            }
            (Some(BodyCollider::Circle(circle)), Some(BodyCollider::Aabb(aabb))) => {
                let aabb_rect: Rect = aabb.get_rect();
                let offset_aabb: AabbCollider = AabbCollider::new(Rect::new(
                    aabb_rect.get_x() + position_delta.get_x(),
                    aabb_rect.get_y() + position_delta.get_y(),
                    aabb_rect.get_width(),
                    aabb_rect.get_height(),
                ));
                offset_aabb
                    .collide_with_circle(&circle)
                    .map(|mut result: CollisionResult| {
                        result.set_normal(-result.get_normal());
                        result
                    })
            }
            _ => None,
        }
    }

    /// Resolves a collision with another body using impulse-based response
    /// and position correction.
    ///
    /// # Arguments
    ///
    /// - `&mut RigidBody2D` - The other body involved in the collision.
    /// - `&CollisionResult` - The collision data.
    fn resolve_collision_with(&mut self, other: &mut RigidBody2D, result: &CollisionResult) {
        let self_inverse_mass: f64 = self.get_inverse_mass();
        let other_inverse_mass: f64 = other.get_inverse_mass();
        let relative_velocity: Vector2D = other.get_velocity() - self.get_velocity();
        let velocity_along_normal: f64 = relative_velocity.dot(result.get_normal());
        if velocity_along_normal > 0.0 {
            return;
        }
        let restitution: f64 = self.get_restitution().min(other.get_restitution());
        let inverse_mass_sum: f64 = self_inverse_mass + other_inverse_mass;
        if inverse_mass_sum == 0.0 {
            return;
        }
        let impulse_magnitude: f64 =
            -(1.0 + restitution) * velocity_along_normal / inverse_mass_sum;
        let impulse: Vector2D = result.get_normal().scaled(impulse_magnitude);
        *self.get_mut_velocity() -= impulse.scaled(self_inverse_mass);
        *other.get_mut_velocity() += impulse.scaled(other_inverse_mass);
        let correction: Vector2D = result
            .get_normal()
            .scaled((result.get_depth() * PHYSICS_POSITION_PERCENT / inverse_mass_sum).max(0.0));
        *self.get_mut_position() -= correction.scaled(self_inverse_mass);
        *other.get_mut_position() += correction.scaled(other_inverse_mass);
    }
}

/// Implements simulation stepping and collision resolution for `PhysicsWorld2D`.
impl PhysicsWorld2D {
    /// Performs one physics simulation step using semi-implicit Euler integration.
    ///
    /// Applies gravity to dynamic bodies, integrates velocity from accumulated forces,
    /// applies damping, integrates position, and resolves collisions.
    ///
    /// # Arguments
    ///
    /// - `f64` - The fixed delta time in seconds.
    pub fn step(&mut self, delta_time: f64) {
        let config: PhysicsConfig = self.get_config();
        // Hoist loop-invariant damping factors out of the per-body loop.
        let damping_factor: f64 = (1.0 - config.get_linear_damping() * delta_time).max(0.0);
        let angular_damping: f64 = (1.0 - config.get_angular_damping() * delta_time).max(0.0);
        let gravity: Vector2D = config.get_gravity();
        for body in self.get_mut_bodies() {
            if !body.is_dynamic() {
                continue;
            }
            let body_mass: f64 = body.get_mass();
            let body_inverse_mass: f64 = body.get_inverse_mass();
            *body.get_mut_force_accumulator() += gravity.scaled(body_mass);
            let force: Vector2D = body.get_force_accumulator();
            *body.get_mut_velocity() += force.scaled(body_inverse_mass * delta_time);
            // In-place damping and integration avoid temporary vector copies.
            *body.get_mut_velocity() *= damping_factor;
            let current_velocity: Vector2D = body.get_velocity();
            *body.get_mut_position() += current_velocity.scaled(delta_time);
            body.set_force_accumulator(Vector2D::zero());
            *body.get_mut_angular_velocity() *= angular_damping;
            let current_angular_velocity: f64 = body.get_angular_velocity();
            *body.get_mut_rotation() += current_angular_velocity * delta_time;
        }
        self.resolve_collisions();
    }

    /// Detects and resolves all collisions between bodies in the world.
    ///
    /// Uses a spatial hash grid for broad-phase culling followed by narrow-phase
    /// shape-specific collision detection, then applies impulse-based resolution.
    /// This reduces the broad-phase from O(n²) to near O(n) for typical scenes.
    fn resolve_collisions(&mut self) {
        let body_count: usize = self.get_bodies().len();
        if body_count < 2 {
            return;
        }
        // Rebuild the persistent grid once per step and collect the candidate
        // pair list once; every solver iteration then reuses both (the grid is
        // unchanged between iterations), eliminating per-iteration re-queries and
        // per-query allocations.
        // OPT 33: the candidate pair list is now backed by `self.pair_buffer`,
        // a persistent `Vec<(usize, usize)>` field on `PhysicsWorld2D`. Cleared
        // at the top of each step instead of allocating a fresh `Vec` — across
        // thousands of physics steps per app run the heap churn adds up.
        self.pair_buffer.clear();
        {
            let (bodies, grid, query_buffer, query_seen) = (
                &self.bodies,
                &mut self.grid,
                &mut self.query_buffer,
                &mut self.query_seen,
            );
            grid.clear();
            for (index, body) in bodies.iter().enumerate() {
                if let Some(bbox) = body.bounding_box() {
                    grid.insert(index, bbox.min(), bbox.max());
                }
            }
            let pairs: &mut Vec<(usize, usize)> = &mut self.pair_buffer;
            for (i, body) in bodies.iter().enumerate() {
                let Some(bbox) = body.bounding_box() else {
                    continue;
                };
                grid.query_into(bbox.min(), bbox.max(), query_buffer, query_seen);
                for &j in query_buffer.iter() {
                    if j > i {
                        pairs.push((i, j));
                    }
                }
            }
        }
        // OPT 33: copy out the pairs slice into a stack-local binding once
        // per step so the immutable borrow on `self.pair_buffer` ends before
        // the `self.get_mut_bodies()` mutable borrow below. Method-call-
        // based disjoint borrow rules in Rust 2024 are not fine-grained
        // enough to allow `self.pair_buffer.iter()` alongside
        // `self.get_mut_bodies()` for a method that takes `&mut self`.
        // The clone happens once per step (not per iteration) — `pairs` is
        // typically O(n) short, much smaller than the per-pair body work.
        let pairs_snapshot: Vec<(usize, usize)> = self.pair_buffer.clone();
        for iteration in 0..PHYSICS_MAX_ITERATIONS {
            let mut any_collision: bool = false;
            for &(i, j) in pairs_snapshot.iter() {
                let (left, right) = self.get_mut_bodies().split_at_mut(j);
                let body_a: &mut RigidBody2D = &mut left[i];
                let body_b: &mut RigidBody2D = &mut right[0];
                if body_a.get_inverse_mass() == 0.0 && body_b.get_inverse_mass() == 0.0 {
                    continue;
                }
                if let Some(result) = body_a.check_collision_with(body_b) {
                    body_a.resolve_collision_with(body_b, &result);
                    any_collision = true;
                }
            }
            if !any_collision {
                break;
            }
            let _: u32 = iteration;
        }
    }
}

/// Forwards `PhysicsWorld2D::step` through the [`Updatable`] trait so that
/// physics worlds participate in the same update loop as entities, animators,
/// and scene managers. The inherent [`PhysicsWorld2D::step`] method is the
/// canonical implementation; this impl exists purely for trait dispatch.
/// The inherent call resolves first when both are in scope, so there is no
/// recursion.
impl Updatable for PhysicsWorld2D {
    /// Advances the simulation by `delta_time` seconds.
    ///
    /// # Arguments
    ///
    /// - `f64` - Seconds elapsed since the previous update.
    fn update(&mut self, delta_time: f64) {
        PhysicsWorld2D::step(self, delta_time);
    }
}

/// Implements default configuration for `PhysicsConfig3D`.
impl Default for PhysicsConfig3D {
    /// Constructs a default [`PhysicsConfig3D`] value.
    ///
    /// # Returns
    ///
    /// - `PhysicsConfig3D` - A default-constructed instance with the documented initial state.
    fn default() -> PhysicsConfig3D {
        PhysicsConfig3D::new(
            Vector3D::new(0.0, DEFAULT_GRAVITY_3D, 0.0),
            DEFAULT_LINEAR_DAMPING,
            DEFAULT_ANGULAR_DAMPING,
        )
    }
}

/// Implements body creation and force management for `RigidBody3D`.
impl RigidBody3D {
    /// Creates a new dynamic 3D rigid body with default mass and the given position.
    ///
    /// # Arguments
    ///
    /// - `u64` - The unique ID.
    /// - `Vector3D` - The initial position.
    ///
    /// # Returns
    ///
    /// - `RigidBody3D` - The new body.
    pub fn new_dynamic(id: u64, position: Vector3D) -> RigidBody3D {
        let mass: f64 = PHYSICS_DEFAULT_MASS;
        let mut body: RigidBody3D = RigidBody3D::new(
            id,
            position,
            mass,
            1.0 / mass,
            DEFAULT_RESTITUTION,
            DEFAULT_FRICTION,
            BodyType::Dynamic,
        );
        body.update_inertia(mass);
        body
    }

    /// Creates a new static 3D rigid body at the given position with infinite mass.
    ///
    /// # Arguments
    ///
    /// - `u64` - The unique ID.
    /// - `Vector3D` - The position.
    ///
    /// # Returns
    ///
    /// - `RigidBody3D` - The new static body.
    pub fn new_static(id: u64, position: Vector3D) -> RigidBody3D {
        RigidBody3D::new(
            id,
            position,
            PHYSICS_STATIC_MASS,
            0.0,
            DEFAULT_RESTITUTION,
            DEFAULT_FRICTION,
            BodyType::Static,
        )
    }

    /// Applies a force to the body's force accumulator.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The force vector.
    pub fn apply_force(&mut self, force: Vector3D) {
        *self.get_mut_force_accumulator() += force;
    }

    /// Applies a torque to the body's torque accumulator.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The torque vector.
    pub fn apply_torque(&mut self, torque: Vector3D) {
        *self.get_mut_torque_accumulator() += torque;
    }

    /// Applies an instantaneous impulse, directly changing velocity.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The impulse vector.
    pub fn apply_impulse(&mut self, impulse: Vector3D) {
        let inverse_mass: f64 = self.get_inverse_mass();
        if inverse_mass == 0.0 {
            return;
        }
        *self.get_mut_velocity() += impulse.scaled(inverse_mass);
    }

    /// Sets the mass of the body, updating the inverse mass.
    /// A mass of 0 makes the body static (infinite mass).
    ///
    /// # Arguments
    ///
    /// - `f64` - The new mass.
    pub fn update_mass(&mut self, mass: f64) {
        self.set_mass(mass);
        self.set_inverse_mass(if mass > 0.0 { 1.0 / mass } else { 0.0 });
    }

    /// Sets the moment of inertia of the body, updating the inverse inertia.
    /// An inertia of 0 makes the body non-rotatable (used for static bodies).
    ///
    /// # Arguments
    ///
    /// - `f64` - The new moment of inertia.
    pub fn update_inertia(&mut self, inertia: f64) {
        self.set_inverse_inertia(if inertia > 0.0 { 1.0 / inertia } else { 0.0 });
    }

    /// Returns `true` if this body is affected by forces and collisions.
    ///
    /// # Returns
    ///
    /// - `bool` - True if this body is dynamic.
    pub fn is_dynamic(&self) -> bool {
        self.get_body_type() == BodyType::Dynamic
    }

    /// Attaches a 3D collider shape to this body.
    ///
    /// # Arguments
    ///
    /// - `BodyCollider3D` - The collider to attach.
    pub fn update_collider(&mut self, collider: BodyCollider3D) {
        self.set_collider(Some(collider));
    }

    /// Returns the world-space 3D bounding box of the attached collider, if any.
    ///
    /// # Returns
    ///
    /// - `Option<AABB3D>` - The bounding box, or `None` if no collider is attached.
    pub fn bounding_box(&self) -> Option<AABB3D> {
        let collider: Option<BodyCollider3D> = self.get_collider();
        let position: Vector3D = self.get_position();
        match collider? {
            BodyCollider3D::Aabb(aabb) => {
                let center: Vector3D = aabb.get_aabb().center();
                let size: Vector3D = aabb.get_aabb().size();
                Some(AABB3D::from_center(
                    position + center,
                    size.get_x(),
                    size.get_y(),
                    size.get_z(),
                ))
            }
            BodyCollider3D::Sphere(sphere) => {
                let sphere_inner: Sphere = sphere.get_sphere();
                let diameter: f64 = sphere_inner.get_radius() * 2.0;
                Some(AABB3D::from_center(
                    position + sphere_inner.get_center(),
                    diameter,
                    diameter,
                    diameter,
                ))
            }
        }
    }
}

/// Implements body management and simulation for `PhysicsWorld3D`.
impl PhysicsWorld3D {
    /// Creates a new 3D physics world with the given configuration.
    ///
    /// # Arguments
    ///
    /// - `PhysicsConfig3D` - The simulation configuration.
    ///
    /// # Returns
    ///
    /// - `PhysicsWorld3D` - The new world.
    pub fn with_config(config: PhysicsConfig3D) -> PhysicsWorld3D {
        let mut world: PhysicsWorld3D = PhysicsWorld3D::new(config);
        world.set_grid(SpatialHashGrid3D::with_default_size());
        world
    }

    /// Adds a rigid body to the world.
    ///
    /// # Arguments
    ///
    /// - `RigidBody3D` - The body to add.
    pub fn add_body(&mut self, body: RigidBody3D) {
        self.get_mut_bodies().push(body);
    }

    /// Removes the body with the given ID.
    ///
    /// # Arguments
    ///
    /// - `u64` - The ID of the body to remove.
    pub fn remove_body(&mut self, id: u64) {
        self.get_mut_bodies()
            .retain(|body: &RigidBody3D| body.get_id() != id);
    }

    /// Returns a reference to the body with the given ID.
    ///
    /// # Arguments
    ///
    /// - `u64` - The body ID.
    ///
    /// # Returns
    ///
    /// - `Option<&RigidBody3D>` - The body reference, if found.
    pub fn get_body(&self, id: u64) -> Option<&RigidBody3D> {
        self.get_bodies()
            .iter()
            .find(|body: &&RigidBody3D| body.get_id() == id)
    }

    /// Returns a mutable reference to the body with the given ID.
    ///
    /// # Arguments
    ///
    /// - `u64` - The body ID.
    ///
    /// # Returns
    ///
    /// - `Option<&mut RigidBody3D>` - The mutable body reference, if found.
    pub fn get_body_mut(&mut self, id: u64) -> Option<&mut RigidBody3D> {
        self.get_mut_bodies()
            .iter_mut()
            .find(|body: &&mut RigidBody3D| body.get_id() == id)
    }

    /// Performs one physics simulation step using semi-implicit Euler integration.
    ///
    /// Applies gravity to dynamic bodies, integrates velocity from accumulated forces,
    /// applies damping, integrates position, and resolves collisions.
    ///
    /// # Arguments
    ///
    /// - `f64` - The fixed delta time in seconds.
    pub fn step(&mut self, delta_time: f64) {
        let config: PhysicsConfig3D = self.get_config();
        // Hoist loop-invariant damping factors out of the per-body loop.
        let damping_factor: f64 = (1.0 - config.get_linear_damping() * delta_time).max(0.0);
        let angular_damping: f64 = (1.0 - config.get_angular_damping() * delta_time).max(0.0);
        let gravity: Vector3D = config.get_gravity();
        for body in self.get_mut_bodies() {
            if !body.is_dynamic() {
                continue;
            }
            let body_mass: f64 = body.get_mass();
            let body_inverse_mass: f64 = body.get_inverse_mass();
            *body.get_mut_force_accumulator() += gravity.scaled(body_mass);
            let force: Vector3D = body.get_force_accumulator();
            *body.get_mut_velocity() += force.scaled(body_inverse_mass * delta_time);
            // In-place damping and integration avoid temporary vector copies.
            *body.get_mut_velocity() *= damping_factor;
            let current_velocity: Vector3D = body.get_velocity();
            *body.get_mut_position() += current_velocity.scaled(delta_time);
            body.set_force_accumulator(Vector3D::zero());
            *body.get_mut_angular_velocity() *= angular_damping;
            let body_inverse_inertia: f64 = body.get_inverse_inertia();
            let torque: Vector3D = body.get_torque_accumulator();
            *body.get_mut_angular_velocity() += torque.scaled(body_inverse_inertia * delta_time);
            let angular_velocity: Vector3D = body.get_angular_velocity();
            let rotation_delta: Quaternion = Quaternion::new(
                angular_velocity.get_x() * delta_time * 0.5,
                angular_velocity.get_y() * delta_time * 0.5,
                angular_velocity.get_z() * delta_time * 0.5,
                1.0,
            );
            body.set_rotation((rotation_delta * body.get_rotation()).normalized());
            body.set_torque_accumulator(Vector3D::zero());
        }
        self.resolve_collisions();
    }

    /// Detects and resolves all collisions between bodies in the 3D world.
    ///
    /// Uses a spatial hash grid for broad-phase culling followed by narrow-phase
    /// shape-specific collision detection, then applies impulse-based resolution.
    /// This reduces the broad-phase from O(n²) to near O(n) for typical scenes.
    fn resolve_collisions(&mut self) {
        let body_count: usize = self.get_bodies().len();
        if body_count < 2 {
            return;
        }
        // Rebuild the persistent grid once per step and collect the candidate
        // pair list once; every solver iteration then reuses both (the grid is
        // unchanged between iterations), eliminating per-iteration re-queries and
        // per-query allocations.
        // OPT 33: candidate pair list backed by `self.pair_buffer`, a persistent
        // field on `PhysicsWorld3D`. See `PhysicsWorld2D::resolve_collisions`
        // for the rationale.
        self.pair_buffer.clear();
        // Collect bboxes first (immutable borrow of bodies) then drain the
        // spatial grid (mutable borrow). Splitting avoids the split-borrow
        // limitation that method-call-based accessors introduce.
        // OPT 33: pre-size the bboxes scratch Vec to the current body count
        // so the first allocation does not double-grow on subsequent frames
        // (each body's bbox is `O(1)` and the Vec is rebuilt every step).
        // The actual allocation still happens here — the persistent
        // `bbox_buffer` field idea was rejected because the immutable-then-
        // mutable borrow split on `self.bodies` cannot hold both a `&mut`
        // borrow on `bbox_buffer` and the source `iter()` simultaneously
        // even with edition 2024 split-borrow rules.
        let body_count: usize = self.get_bodies().len();
        let mut bboxes: Vec<(usize, AABB3D)> = Vec::with_capacity(body_count);
        bboxes.extend(
            self.get_bodies()
                .iter()
                .enumerate()
                .filter_map(|(index, body)| body.bounding_box().map(|bbox| (index, bbox))),
        );
        {
            let Self {
                grid,
                query_buffer,
                query_seen,
                ..
            } = self;
            let grid: &mut SpatialHashGrid3D = grid;
            let query_buffer: &mut Vec<usize> = query_buffer;
            let query_seen: &mut HashSet<usize> = query_seen;
            grid.clear();
            let pairs: &mut Vec<(usize, usize)> = &mut self.pair_buffer;
            for (index, bbox) in bboxes.iter() {
                grid.insert(*index, bbox.get_min(), bbox.get_max());
            }
            for (i, (_, bbox)) in bboxes.iter().enumerate() {
                grid.query_into(bbox.get_min(), bbox.get_max(), query_buffer, query_seen);
                for &j in query_buffer.iter() {
                    if j > i {
                        pairs.push((i, j));
                    }
                }
            }
        }
        // OPT 33: copy out the pairs slice into a stack-local binding once
        // per step so the immutable borrow on `self.pair_buffer` ends before
        // the `self.get_mut_bodies()` mutable borrow below. See the matching
        // comment in `PhysicsWorld2D::resolve_collisions` for rationale.
        let pairs_snapshot: Vec<(usize, usize)> = self.pair_buffer.clone();
        for iteration in 0..PHYSICS_MAX_ITERATIONS {
            let mut any_collision: bool = false;
            for &(i, j) in pairs_snapshot.iter() {
                let (left, right) = self.get_mut_bodies().split_at_mut(j);
                let body_a: &mut RigidBody3D = &mut left[i];
                let body_b: &mut RigidBody3D = &mut right[0];
                if body_a.get_inverse_mass() == 0.0 && body_b.get_inverse_mass() == 0.0 {
                    continue;
                }
                if let Some(result) = Self::check_collision_3d(body_a, body_b) {
                    Self::resolve_collision_3d(body_a, body_b, &result);
                    any_collision = true;
                }
            }
            if !any_collision {
                break;
            }
            let _: u32 = iteration;
        }
    }

    /// Checks collision between two 3D bodies based on both bodies' collider shapes.
    ///
    /// # Arguments
    ///
    /// - `&RigidBody3D` - The first body.
    /// - `&RigidBody3D` - The second body.
    ///
    /// # Returns
    ///
    /// - `Option<CollisionResult3D>` - The collision result, or `None`.
    fn check_collision_3d(a: &RigidBody3D, b: &RigidBody3D) -> Option<CollisionResult3D> {
        let a_bbox: AABB3D = a.bounding_box()?;
        let b_bbox: AABB3D = b.bounding_box()?;
        if !AABB3D::broad_phase(a_bbox, b_bbox) {
            return None;
        }
        let a_collider: Option<BodyCollider3D> = a.get_collider();
        let b_collider: Option<BodyCollider3D> = b.get_collider();
        let position_delta: Vector3D = b.get_position() - a.get_position();
        match (a_collider, b_collider) {
            (Some(BodyCollider3D::Aabb(aabb_a)), Some(BodyCollider3D::Aabb(aabb_b))) => {
                let aabb_b_inner: AABB3D = aabb_b.get_aabb();
                let offset_aabb: AabbCollider3D = AabbCollider3D::new(AABB3D::new(
                    aabb_b_inner.get_min() + position_delta,
                    aabb_b_inner.get_max() + position_delta,
                ));
                aabb_a.collide_with_aabb(&offset_aabb)
            }
            (Some(BodyCollider3D::Sphere(sphere_a)), Some(BodyCollider3D::Sphere(sphere_b))) => {
                let sphere_b_inner: Sphere = sphere_b.get_sphere();
                let offset_sphere: SphereCollider3D = SphereCollider3D::new(Sphere::new(
                    sphere_b_inner.get_center() + position_delta,
                    sphere_b_inner.get_radius(),
                ));
                sphere_a.collide_with_sphere(&offset_sphere)
            }
            (Some(BodyCollider3D::Aabb(aabb)), Some(BodyCollider3D::Sphere(sphere))) => {
                let sphere_inner: Sphere = sphere.get_sphere();
                let offset_sphere: SphereCollider3D = SphereCollider3D::new(Sphere::new(
                    sphere_inner.get_center() + position_delta,
                    sphere_inner.get_radius(),
                ));
                aabb.collide_with_sphere(&offset_sphere)
            }
            (Some(BodyCollider3D::Sphere(sphere)), Some(BodyCollider3D::Aabb(aabb))) => {
                let aabb_inner: AABB3D = aabb.get_aabb();
                let offset_aabb: AabbCollider3D = AabbCollider3D::new(AABB3D::new(
                    aabb_inner.get_min() + position_delta,
                    aabb_inner.get_max() + position_delta,
                ));
                offset_aabb
                    .collide_with_sphere(&sphere)
                    .map(|mut result: CollisionResult3D| {
                        result.set_normal(-result.get_normal());
                        result
                    })
            }
            _ => None,
        }
    }

    /// Resolves a collision between two 3D bodies using impulse-based response
    /// and position correction.
    ///
    /// # Arguments
    ///
    /// - `&mut RigidBody3D` - The first body.
    /// - `&mut RigidBody3D` - The second body.
    /// - `&CollisionResult3D` - The collision data.
    fn resolve_collision_3d(a: &mut RigidBody3D, b: &mut RigidBody3D, result: &CollisionResult3D) {
        let a_inverse_mass: f64 = a.get_inverse_mass();
        let b_inverse_mass: f64 = b.get_inverse_mass();
        let relative_velocity: Vector3D = b.get_velocity() - a.get_velocity();
        let velocity_along_normal: f64 = relative_velocity.dot(result.get_normal());
        if velocity_along_normal > 0.0 {
            return;
        }
        let restitution: f64 = a.get_restitution().min(b.get_restitution());
        let inverse_mass_sum: f64 = a_inverse_mass + b_inverse_mass;
        if inverse_mass_sum == 0.0 {
            return;
        }
        let impulse_magnitude: f64 =
            -(1.0 + restitution) * velocity_along_normal / inverse_mass_sum;
        let impulse: Vector3D = result.get_normal().scaled(impulse_magnitude);
        *a.get_mut_velocity() -= impulse.scaled(a_inverse_mass);
        *b.get_mut_velocity() += impulse.scaled(b_inverse_mass);
        let correction: Vector3D = result
            .get_normal()
            .scaled((result.get_depth() * PHYSICS_POSITION_PERCENT / inverse_mass_sum).max(0.0));
        *a.get_mut_position() -= correction.scaled(a_inverse_mass);
        *b.get_mut_position() += correction.scaled(b_inverse_mass);
    }
}

/// Forwards `PhysicsWorld3D::step` through the [`Updatable`] trait so that
/// 3D physics worlds participate in the same update loop as their 2D
/// counterparts, entities, animators, and scene managers. The inherent
/// [`PhysicsWorld3D::step`] method is the canonical implementation; this impl
/// exists purely for trait dispatch. The inherent call resolves first when
/// both are in scope, so there is no recursion.
impl Updatable for PhysicsWorld3D {
    /// Advances the simulation by `delta_time` seconds.
    ///
    /// # Arguments
    ///
    /// - `f64` - Seconds elapsed since the previous update.
    fn update(&mut self, delta_time: f64) {
        PhysicsWorld3D::step(self, delta_time);
    }
}

/// Implements `Default` for `PhysicsWorld3D` as an empty world.
impl Default for PhysicsWorld3D {
    /// Constructs a default [`PhysicsWorld3D`] value.
    ///
    /// # Returns
    ///
    /// - `PhysicsWorld3D` - A default-constructed instance with the documented initial state.
    fn default() -> PhysicsWorld3D {
        PhysicsWorld3D::with_config(PhysicsConfig3D::default())
    }
}

#[cfg(test)]
mod tests {
    //! Regression tests for the 3D physics step pipeline.
    //!
    //! These tests live inline (rather than under `engine/tests/`) because the
    //! `physics` module does not yet `pub use r#impl`, so external tests cannot
    //! reach methods like `step()` or `apply_torque()`. Once the module is
    //! reorganised to expose impls publicly, these can move to an integration
    //! test target alongside `input/fn.rs` and `webgpu/fn.rs`.
    use super::*;

    const EPSILON: f64 = 1e-9;

    /// Regression test for the bug where `RigidBody3D::apply_torque`
    /// accumulated torque into `torque_accumulator` but
    /// `PhysicsWorld3D::step()` only zeroed it without ever converting it
    /// into angular velocity.
    #[test]
    fn step_applies_torque_to_3d_angular_velocity() {
        let mut world: PhysicsWorld3D = PhysicsWorld3D::default();
        let mut body: RigidBody3D = RigidBody3D::new_dynamic(1, Vector3D::new(0.0, 0.0, 0.0));
        // Default inertia = mass (1.0) so inverse_inertia == 1.0; applying
        // torque (0, 0, 2) for a 1 s step should give omega == (0, 0, 2).
        body.apply_torque(Vector3D::new(0.0, 0.0, 2.0));
        world.add_body(body);

        world.step(1.0);
        let omega: Vector3D = world.get_body(1).unwrap().get_angular_velocity();
        assert!(
            omega.get_x().abs() < EPSILON,
            "unexpected x angular velocity: {}",
            omega.get_x(),
        );
        assert!(
            omega.get_y().abs() < EPSILON,
            "unexpected y angular velocity: {}",
            omega.get_y(),
        );
        let expected_z: f64 = 2.0;
        assert!(
            (omega.get_z() - expected_z).abs() < EPSILON,
            "expected z angular velocity {}, got {}",
            expected_z,
            omega.get_z(),
        );

        // Accumulator must be cleared after the step so subsequent steps do
        // not re-apply the same torque.
        world.step(1.0);
        let omega_after: Vector3D = world.get_body(1).unwrap().get_angular_velocity();
        assert!(
            (omega_after.get_z() - expected_z).abs() < EPSILON,
            "torque_accumulator leaked into a second step: z = {}",
            omega_after.get_z(),
        );
    }

    /// Static bodies (inverse_inertia == 0) must ignore torque entirely:
    /// applying torque to a static body must not produce angular velocity.
    #[test]
    fn step_static_3d_body_ignores_torque() {
        let mut world: PhysicsWorld3D = PhysicsWorld3D::default();
        let mut body: RigidBody3D = RigidBody3D::new_static(1, Vector3D::new(0.0, 0.0, 0.0));
        body.apply_torque(Vector3D::new(1.0, 0.0, 0.0));
        world.add_body(body);

        world.step(1.0);
        let omega: Vector3D = world.get_body(1).unwrap().get_angular_velocity();
        assert!(
            omega.get_x().abs() < EPSILON
                && omega.get_y().abs() < EPSILON
                && omega.get_z().abs() < EPSILON,
            "static body must remain rotationally inert, got ({}, {}, {})",
            omega.get_x(),
            omega.get_y(),
            omega.get_z(),
        );
    }

    /// Torque applied across multiple steps must accumulate into angular
    /// velocity. Guards against the original bug returning silently.
    #[test]
    fn step_torque_accumulates_over_multiple_steps() {
        let mut world: PhysicsWorld3D = PhysicsWorld3D::default();
        let body: RigidBody3D = RigidBody3D::new_dynamic(1, Vector3D::new(0.0, 0.0, 0.0));
        world.add_body(body);

        for _ in 0..4 {
            world
                .get_body_mut(1)
                .unwrap()
                .apply_torque(Vector3D::new(0.0, 1.0, 0.0));
            world.step(1.0);
        }
        let omega: Vector3D = world.get_body(1).unwrap().get_angular_velocity();
        // Each step: omega_y += torque_y * inv_inertia * dt = 1.0.
        // After 4 steps: omega_y == 4.0.
        assert!(
            (omega.get_y() - 4.0).abs() < EPSILON,
            "expected cumulative angular velocity 4.0 on y axis, got {}",
            omega.get_y(),
        );
    }

    /// `update_inertia(0)` must zero out `inverse_inertia`, making torque
    /// application inert for that body until the inertia is restored.
    #[test]
    fn update_inertia_zeros_inverse_inertia() {
        let mut body: RigidBody3D = RigidBody3D::new_dynamic(1, Vector3D::new(0.0, 0.0, 0.0));
        assert!(
            (body.get_inverse_inertia() - 1.0).abs() < EPSILON,
            "default inverse_inertia must equal 1/mass = 1.0, got {}",
            body.get_inverse_inertia(),
        );
        body.update_inertia(0.0);
        assert!(
            body.get_inverse_inertia().abs() < EPSILON,
            "update_inertia(0) must zero out inverse_inertia, got {}",
            body.get_inverse_inertia(),
        );
    }

    /// 2D angular integration is unchanged by the 3D torque fix.
    #[test]
    fn step_2d_angular_velocity_unchanged() {
        let mut world: PhysicsWorld2D = PhysicsWorld2D::default();
        let body: RigidBody2D = RigidBody2D::new_dynamic(1, Vector2D::new(0.0, 0.0));
        world.add_body(body);

        world.step(1.0);
        let omega: f64 = world.get_body(1).unwrap().get_angular_velocity();
        assert!(
            omega.abs() < EPSILON,
            "2D angular velocity should remain 0 with no input, got {}",
            omega,
        );
    }
}
