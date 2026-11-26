use super::*;

/// Constructors and bounding-box helpers for `RigidBody3D`.
impl RigidBody3D {
    /// Creates a dynamic 3D rigid body with given mass, position, and box half-extents.
    ///
    /// # Arguments
    ///
    /// - `Vec3` - Initial center position.
    /// - `Vec3` - Box half-extents `(hx, hy, hz)`.
    /// - `f32` - Positive mass in kilograms.
    ///
    /// # Returns
    ///
    /// - `Self` - Dynamic rigid body.
    #[must_use]
    pub fn dynamic_box(position: Vec3, half_extents: Vec3, mass: f32) -> Self {
        Self {
            position,
            velocity: Vec3::zero(),
            half_extents,
            inv_mass: 1.0 / mass.max(0.01),
            restitution: 0.35,
            linear_damping: 0.04,
            grounded: false,
        }
    }

    /// Creates a static immovable 3D box collider.
    ///
    /// # Arguments
    ///
    /// - `Vec3` - Center position.
    /// - `Vec3` - Box half-extents `(hx, hy, hz)`.
    ///
    /// # Returns
    ///
    /// - `Self` - Static rigid body (`inv_mass = 0.0`).
    #[must_use]
    pub fn static_box(position: Vec3, half_extents: Vec3) -> Self {
        Self {
            position,
            velocity: Vec3::zero(),
            half_extents,
            inv_mass: 0.0,
            restitution: 0.25,
            linear_damping: 0.0,
            grounded: true,
        }
    }

    /// Computes this body's world-space `Aabb3D`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Body reference.
    ///
    /// # Returns
    ///
    /// - `Aabb3D` - World-space bounding box.
    #[must_use]
    pub fn world_aabb(&self) -> Aabb3D {
        let p: Vec3 = self.get_position();
        let h: Vec3 = self.get_half_extents();
        Aabb3D::new(p.sub(h), p.add(h))
    }

    /// Applies an instantaneous linear impulse to a dynamic rigid body.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable body reference.
    /// - `Vec3` - Linear impulse vector in `N*s`.
    pub fn apply_impulse(&mut self, impulse: Vec3) {
        let inv_m: f32 = self.get_inv_mass();
        if inv_m > 0.0 {
            let next_v: Vec3 = self.get_velocity().add(impulse.scale(inv_m));
            self.set_velocity(next_v);
        }
    }
}

/// Default Earth-gravity 3D physics world.
impl Default for PhysicsWorld3D {
    /// Creates an empty `PhysicsWorld3D` with `-9.81 m/s^2` gravity and ground at `y = 0.0`.
    ///
    /// # Returns
    ///
    /// - `Self` - Ready 3D physics world.
    fn default() -> Self {
        Self {
            gravity: Vec3::new(0.0, -9.81, 0.0),
            ground_y: 0.0,
            bodies: Vec::new(),
        }
    }
}

/// Rigid-body integration, AABB collision resolution, and raycasting for `PhysicsWorld3D`.
impl PhysicsWorld3D {
    /// Adds a `RigidBody3D` to the simulation and returns its handle index.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable physics world.
    /// - `RigidBody3D` - Body to insert.
    ///
    /// # Returns
    ///
    /// - `usize` - Body index.
    pub fn add_body(&mut self, body: RigidBody3D) -> usize {
        let idx: usize = self.get_bodies().len();
        self.get_mut_bodies().push(body);
        idx
    }

    /// Advances all rigid bodies by `dt` seconds with gravity, ground collisions, and pairwise AABB impulses.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable physics world.
    /// - `f32` - Timestep in seconds.
    ///
    /// # Returns
    ///
    /// - `usize` - Number of collisions resolved during this step.
    pub fn step(&mut self, dt: f32) -> usize {
        let step_dt: f32 = dt.clamp(0.0, 0.1);
        let gravity: Vec3 = self.get_gravity();
        let ground_y: f32 = self.get_ground_y();
        let mut collisions: usize = 0;
        for body in self.get_mut_bodies().iter_mut() {
            let inv_m: f32 = body.get_inv_mass();
            if inv_m <= 0.0 {
                continue;
            }
            let damp: f32 = (1.0 - body.get_linear_damping() * step_dt).clamp(0.0, 1.0);
            let mut vel: Vec3 = body.get_velocity().add(gravity.scale(step_dt)).scale(damp);
            let mut pos: Vec3 = body.get_position().add(vel.scale(step_dt));
            let min_y: f32 = ground_y + body.get_half_extents().get_y();
            let mut grounded: bool = false;
            if pos.get_y() <= min_y {
                pos.set_y(min_y);
                if vel.get_y() < 0.0 {
                    vel.set_y(-vel.get_y() * body.get_restitution());
                    if vel.get_y().abs() < 0.25 {
                        vel.set_y(0.0);
                    }
                    vel.set_x(vel.get_x() * 0.92);
                    vel.set_z(vel.get_z() * 0.92);
                    collisions += 1;
                }
                grounded = true;
            }
            body.set_position(pos);
            body.set_velocity(vel);
            body.set_grounded(grounded);
        }
        let count: usize = self.get_bodies().len();
        for i in 0..count {
            for j in (i + 1)..count {
                let a: RigidBody3D = self.get_bodies()[i];
                let b: RigidBody3D = self.get_bodies()[j];
                let inv_sum: f32 = a.get_inv_mass() + b.get_inv_mass();
                if inv_sum <= 0.0 {
                    continue;
                }
                let delta: Vec3 = b.get_position().sub(a.get_position());
                let ox: f32 = a.get_half_extents().get_x() + b.get_half_extents().get_x()
                    - delta.get_x().abs();
                let oy: f32 = a.get_half_extents().get_y() + b.get_half_extents().get_y()
                    - delta.get_y().abs();
                let oz: f32 = a.get_half_extents().get_z() + b.get_half_extents().get_z()
                    - delta.get_z().abs();
                if ox <= 0.0 || oy <= 0.0 || oz <= 0.0 {
                    continue;
                }
                let (normal, pen): (Vec3, f32) = if ox <= oy && ox <= oz {
                    let s: f32 = if delta.get_x() < 0.0 { -1.0 } else { 1.0 };
                    (Vec3::new(s, 0.0, 0.0), ox)
                } else if oy <= oz {
                    let s: f32 = if delta.get_y() < 0.0 { -1.0 } else { 1.0 };
                    (Vec3::new(0.0, s, 0.0), oy)
                } else {
                    let s: f32 = if delta.get_z() < 0.0 { -1.0 } else { 1.0 };
                    (Vec3::new(0.0, 0.0, s), oz)
                };
                let corr: Vec3 = normal.scale(pen / inv_sum);
                let mut a_pos: Vec3 = a.get_position();
                let mut b_pos: Vec3 = b.get_position();
                a_pos = a_pos.sub(corr.scale(a.get_inv_mass()));
                b_pos = b_pos.add(corr.scale(b.get_inv_mass()));
                let rel_v: Vec3 = b.get_velocity().sub(a.get_velocity());
                let vn: f32 = rel_v.dot(normal);
                let mut a_vel: Vec3 = a.get_velocity();
                let mut b_vel: Vec3 = b.get_velocity();
                if vn < 0.0 {
                    let rest: f32 = a.get_restitution().min(b.get_restitution());
                    let j_imp: f32 = -(1.0 + rest) * vn / inv_sum;
                    let imp: Vec3 = normal.scale(j_imp);
                    a_vel = a_vel.sub(imp.scale(a.get_inv_mass()));
                    b_vel = b_vel.add(imp.scale(b.get_inv_mass()));
                }
                let bodies_mut: &mut [RigidBody3D] = self.get_mut_bodies().as_mut_slice();
                bodies_mut[i].set_position(a_pos);
                bodies_mut[i].set_velocity(a_vel);
                bodies_mut[j].set_position(b_pos);
                bodies_mut[j].set_velocity(b_vel);
                collisions += 1;
            }
        }
        collisions
    }

    /// Casts a `Ray3D` against all 3D rigid bodies up to `max_distance` and returns the closest hit.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Physics world reference.
    /// - `&Ray3D` - Query ray.
    /// - `f32` - Maximum cast distance in meters.
    ///
    /// # Returns
    ///
    /// - `Option<RayHitPhysics3D>` - Closest hit body and contact position if any.
    #[must_use]
    pub fn raycast(&self, ray: &Ray3D, max_distance: f32) -> Option<RayHitPhysics3D> {
        let mut best: Option<RayHitPhysics3D> = None;
        let mut best_t: f32 = max_distance.max(0.0);
        for (idx, body) in self.get_bodies().iter().enumerate() {
            let aabb: Aabb3D = body.world_aabb();
            let t: f32 =
                aabb.intersect_ray_near(ray.get_origin(), ray.get_inv_direction(), 0.001, best_t);
            if t.is_finite() && t < best_t {
                best_t = t;
                let pt: Vec3 = ray.get_origin().add(ray.get_direction().scale(t));
                best = Some(RayHitPhysics3D::new(idx, t, pt));
            }
        }
        best
    }
}
