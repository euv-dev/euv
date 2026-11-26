use super::*;

/// Default zero-gravity top-down 2D physics configuration.
impl Default for PhysicsConfig {
    /// Creates a default zero-gravity `PhysicsConfig` with `64px` spatial-hash cells.
    ///
    /// # Returns
    ///
    /// - `Self` - Default physics configuration.
    fn default() -> Self {
        Self {
            gravity: Point::new(0.0, 0.0),
            cell_size: DEFAULT_SPATIAL_CELL_SIZE,
            position_correction: 0.8,
            arena_bounds: None,
        }
    }
}

/// Default empty 2D physics world.
impl Default for PhysicsWorld2D {
    /// Creates a `PhysicsWorld2D` using `PhysicsConfig::default()`.
    ///
    /// # Returns
    ///
    /// - `Self` - Ready physics world.
    fn default() -> Self {
        Self::with_config(PhysicsConfig::default())
    }
}

/// Fluent query builder for `RaycastQuery2D`.
impl RaycastQuery2D {
    /// Creates a 2D raycast query from `origin` along `direction` with default `1000.0` distance and all layers enabled.
    ///
    /// # Arguments
    ///
    /// - `Point` - Ray origin.
    /// - `Point` - Ray direction vector.
    ///
    /// # Returns
    ///
    /// - `Self` - Ready raycast query.
    #[must_use]
    pub const fn new(origin: Point, direction: Point) -> Self {
        Self {
            origin,
            direction,
            max_distance: 1000.0,
            layer_mask: u32::MAX,
        }
    }

    /// Sets maximum travel distance for the raycast.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source query.
    /// - `f32` - Maximum distance in world units.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated query.
    #[must_use]
    pub fn with_max_distance(mut self, max_distance: f32) -> Self {
        self.set_max_distance(max_distance.max(0.0));
        self
    }

    /// Sets the collision layer filter bitmask for the raycast.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source query.
    /// - `u32` - Collision layer bitmask.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated query.
    #[must_use]
    pub fn with_layer_mask(mut self, layer_mask: u32) -> Self {
        self.set_layer_mask(layer_mask);
        self
    }
}

/// Broad-phase spatial hashing, narrow-phase contact resolution, and raycasting for `PhysicsWorld2D`.
impl PhysicsWorld2D {
    /// Creates a `PhysicsWorld2D` configured with `config`.
    ///
    /// # Arguments
    ///
    /// - `PhysicsConfig` - Physics simulation parameters.
    ///
    /// # Returns
    ///
    /// - `Self` - Configured 2D physics solver.
    #[must_use]
    pub fn with_config(config: PhysicsConfig) -> Self {
        Self {
            config,
            grid: HashMap::with_capacity(128),
            seen_pairs: HashSet::with_capacity(256),
            last_stats: PhysicsStepStats::default(),
        }
    }

    /// Integrates velocities/positions, resolves spatial-hash collisions and arena bounds, and returns contact events.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable physics world.
    /// - `&mut GameWorld` - Mutable ECS world containing entities and colliders.
    /// - `f32` - Fixed physics timestep in seconds.
    ///
    /// # Returns
    ///
    /// - `Vec<CollisionEvent>` - All solid and trigger contacts detected this step.
    pub fn step(&mut self, world: &mut GameWorld, dt_secs: f32) -> Vec<CollisionEvent> {
        let cfg: PhysicsConfig = self.get_config();
        let gx: f32 = cfg.get_gravity().get_x();
        let gy: f32 = cfg.get_gravity().get_y();
        let inv_cell: f32 = 1.0 / cfg.get_cell_size().max(8.0);
        let arena_opt: Option<Rect> = {
            let PhysicsConfig { arena_bounds, .. } = cfg;
            arena_bounds
        };
        let mut integrated: usize = 0;
        for bucket in self.get_mut_grid().values_mut() {
            bucket.clear();
        }
        self.get_mut_seen_pairs().clear();
        let entity_len: usize = world.get_entities().len();
        for idx in 0..entity_len {
            let ent: &mut EntityRecord = &mut world.get_mut_entities()[idx];
            if !ent.get_alive() {
                continue;
            }
            if let Some(body) = ent.body_component() {
                match body.get_kind() {
                    BodyKind::Static => {}
                    BodyKind::Kinematic => {
                        let vel: Velocity2D = ent.get_velocity();
                        let pos: Point = ent.get_transform().get_position();
                        let next_pos: Point = Point::new(
                            pos.get_x() + vel.get_vx() * dt_secs,
                            pos.get_y() + vel.get_vy() * dt_secs,
                        );
                        let next_rot: f32 =
                            ent.get_transform().get_rotation_rad() + vel.get_angular() * dt_secs;
                        ent.get_mut_transform().set_position(next_pos);
                        ent.get_mut_transform().set_rotation_rad(next_rot);
                        integrated += 1;
                    }
                    BodyKind::Dynamic => {
                        let mut vel: Velocity2D = ent.get_velocity();
                        let g_scale: f32 = body.get_gravity_scale();
                        let damp: f32 = (1.0 - body.get_linear_drag() * dt_secs).clamp(0.0, 1.0);
                        vel.set_vx((vel.get_vx() + gx * g_scale * dt_secs) * damp);
                        vel.set_vy((vel.get_vy() + gy * g_scale * dt_secs) * damp);
                        let pos: Point = ent.get_transform().get_position();
                        let mut nx: f32 = pos.get_x() + vel.get_vx() * dt_secs;
                        let mut ny: f32 = pos.get_y() + vel.get_vy() * dt_secs;
                        if let Some(arena) = arena_opt {
                            let (half_w, half_h): (f32, f32) = match ent.collider_component() {
                                Some(col) => match col.get_shape() {
                                    ColliderShape::Circle { radius } => (radius, radius),
                                    ColliderShape::Aabb { width, height } => {
                                        (width * 0.5, height * 0.5)
                                    }
                                },
                                None => (0.0, 0.0),
                            };
                            let rest: f32 = body.get_restitution();
                            let min_x: f32 = arena.get_x() + half_w;
                            let max_x: f32 = arena.right() - half_w;
                            let min_y: f32 = arena.get_y() + half_h;
                            let max_y: f32 = arena.bottom() - half_h;
                            if nx < min_x {
                                nx = min_x;
                                vel.set_vx(vel.get_vx().abs() * rest);
                            } else if nx > max_x {
                                nx = max_x;
                                vel.set_vx(-vel.get_vx().abs() * rest);
                            }
                            if ny < min_y {
                                ny = min_y;
                                vel.set_vy(vel.get_vy().abs() * rest);
                            } else if ny > max_y {
                                ny = max_y;
                                vel.set_vy(-vel.get_vy().abs() * rest);
                            }
                        }
                        let next_rot: f32 =
                            ent.get_transform().get_rotation_rad() + vel.get_angular() * dt_secs;
                        ent.set_velocity(vel);
                        ent.get_mut_transform().set_position(Point::new(nx, ny));
                        ent.get_mut_transform().set_rotation_rad(next_rot);
                        integrated += 1;
                    }
                }
            }
            if let Some(col) = ent.collider_component() {
                let aabb: Rect = col.world_aabb(ent.get_transform().get_position());
                let min_cx: i32 = (aabb.get_x() * inv_cell).floor() as i32;
                let max_cx: i32 = (aabb.right() * inv_cell).floor() as i32;
                let min_cy: i32 = (aabb.get_y() * inv_cell).floor() as i32;
                let max_cy: i32 = (aabb.bottom() * inv_cell).floor() as i32;
                for cy in min_cy..=max_cy {
                    for cx in min_cx..=max_cx {
                        self.get_mut_grid().entry((cx, cy)).or_default().push(idx);
                    }
                }
            }
        }
        let mut candidate_pairs: Vec<(usize, usize)> = Vec::new();
        {
            let Self {
                grid, seen_pairs, ..
            } = self;
            for bucket in grid.values() {
                let n: usize = bucket.len();
                for i in 0..n {
                    let a: usize = bucket[i];
                    for &b in &bucket[(i + 1)..] {
                        let pair: (usize, usize) = if a < b { (a, b) } else { (b, a) };
                        if seen_pairs.insert(pair) {
                            candidate_pairs.push(pair);
                        }
                    }
                }
            }
        }
        let broad_count: usize = candidate_pairs.len();
        let mut events: Vec<CollisionEvent> = Vec::new();
        for (ia, ib) in candidate_pairs.into_iter() {
            let (id_a, pos_a, vel_a, body_a, col_a): (
                EntityId,
                Point,
                Velocity2D,
                RigidBody2D,
                Collider2D,
            ) = {
                let ent: &EntityRecord = &world.get_entities()[ia];
                let Some(col) = ent.collider_component() else {
                    continue;
                };
                (
                    ent.get_id(),
                    ent.get_transform().get_position(),
                    ent.get_velocity(),
                    ent.body_component()
                        .unwrap_or_else(|| RigidBody2D::static_body(0.5)),
                    col,
                )
            };
            let (id_b, pos_b, vel_b, body_b, col_b): (
                EntityId,
                Point,
                Velocity2D,
                RigidBody2D,
                Collider2D,
            ) = {
                let ent: &EntityRecord = &world.get_entities()[ib];
                let Some(col) = ent.collider_component() else {
                    continue;
                };
                (
                    ent.get_id(),
                    ent.get_transform().get_position(),
                    ent.get_velocity(),
                    ent.body_component()
                        .unwrap_or_else(|| RigidBody2D::static_body(0.5)),
                    col,
                )
            };
            if (col_a.get_layer() & col_b.get_mask()) == 0
                || (col_b.get_layer() & col_a.get_mask()) == 0
            {
                continue;
            }
            let Some((normal, penetration)) =
                Self::narrow_phase(pos_a, col_a.get_shape(), pos_b, col_b.get_shape())
            else {
                continue;
            };
            let is_trigger: bool = col_a.get_is_trigger() || col_b.get_is_trigger();
            events.push(CollisionEvent::new(
                id_a,
                id_b,
                normal,
                penetration,
                is_trigger,
            ));
            if is_trigger {
                continue;
            }
            let inv_a: f32 = body_a.get_inv_mass();
            let inv_b: f32 = body_b.get_inv_mass();
            let inv_sum: f32 = inv_a + inv_b;
            if inv_sum <= 0.0 {
                continue;
            }
            let corr_mag: f32 = (penetration / inv_sum) * cfg.get_position_correction();
            let corr_x: f32 = normal.get_x() * corr_mag;
            let corr_y: f32 = normal.get_y() * corr_mag;
            let next_pos_a: Point = Point::new(
                pos_a.get_x() - corr_x * inv_a,
                pos_a.get_y() - corr_y * inv_a,
            );
            let next_pos_b: Point = Point::new(
                pos_b.get_x() + corr_x * inv_b,
                pos_b.get_y() + corr_y * inv_b,
            );
            let rvx: f32 = vel_b.get_vx() - vel_a.get_vx();
            let rvy: f32 = vel_b.get_vy() - vel_a.get_vy();
            let vel_along_normal: f32 = rvx * normal.get_x() + rvy * normal.get_y();
            let mut next_vel_a: Velocity2D = vel_a;
            let mut next_vel_b: Velocity2D = vel_b;
            if vel_along_normal < 0.0 {
                let e: f32 = body_a.get_restitution().min(body_b.get_restitution());
                let j: f32 = -(1.0 + e) * vel_along_normal / inv_sum;
                let imp_x: f32 = j * normal.get_x();
                let imp_y: f32 = j * normal.get_y();
                next_vel_a.set_vx(vel_a.get_vx() - imp_x * inv_a);
                next_vel_a.set_vy(vel_a.get_vy() - imp_y * inv_a);
                next_vel_b.set_vx(vel_b.get_vx() + imp_x * inv_b);
                next_vel_b.set_vy(vel_b.get_vy() + imp_y * inv_b);
            }
            world.get_mut_entities()[ia]
                .get_mut_transform()
                .set_position(next_pos_a);
            world.get_mut_entities()[ia].set_velocity(next_vel_a);
            world.get_mut_entities()[ib]
                .get_mut_transform()
                .set_position(next_pos_b);
            world.get_mut_entities()[ib].set_velocity(next_vel_b);
        }
        self.set_last_stats(PhysicsStepStats::new(integrated, broad_count, events.len()));
        events
    }

    /// Casts a 2D ray from `origin` along normalized `direction` up to `max_distance` against colliders matching `layer_mask`.
    ///
    /// # Arguments
    ///
    /// - `&GameWorld` - Game world to query.
    /// - `Point` - Ray origin in world units.
    /// - `Point` - Ray direction vector (normalized automatically).
    /// - `f32` - Maximum ray distance.
    /// - `u32` - Layer bitmask filter.
    ///
    /// # Returns
    ///
    /// - `Option<RaycastHit>` - Closest ray intersection hit, if any.
    #[must_use]
    pub fn raycast(
        world: &GameWorld,
        origin: Point,
        direction: Point,
        max_distance: f32,
        layer_mask: u32,
    ) -> Option<RaycastHit> {
        let len: f32 =
            (direction.get_x() * direction.get_x() + direction.get_y() * direction.get_y()).sqrt();
        if len <= 1e-6 || max_distance <= 0.0 {
            return None;
        }
        let dx: f32 = direction.get_x() / len;
        let dy: f32 = direction.get_y() / len;
        let mut closest: Option<RaycastHit> = None;
        let mut best_t: f32 = max_distance;
        for ent in world.get_entities().iter() {
            if !ent.get_alive() {
                continue;
            }
            let Some(col) = ent.collider_component() else {
                continue;
            };
            if (col.get_layer() & layer_mask) == 0 {
                continue;
            }
            let pos: Point = ent.get_transform().get_position();
            let hit_opt: Option<(f32, Point)> = match col.get_shape() {
                ColliderShape::Circle { radius } => {
                    Self::ray_vs_circle(origin, (dx, dy), pos, radius, best_t)
                }
                ColliderShape::Aabb { width, height } => Self::ray_vs_aabb(
                    origin,
                    (dx, dy),
                    Rect::new(
                        pos.get_x() - width * 0.5,
                        pos.get_y() - height * 0.5,
                        width,
                        height,
                    ),
                    best_t,
                ),
            };
            if let Some((t, normal)) = hit_opt
                && t < best_t
            {
                best_t = t;
                let hit_pt: Point = Point::new(origin.get_x() + dx * t, origin.get_y() + dy * t);
                closest = Some(RaycastHit::new(ent.get_id(), hit_pt, normal, t));
            }
        }
        closest
    }

    /// Casts a 2D ray defined by a fluent `RaycastQuery2D` against `world`.
    ///
    /// # Arguments
    ///
    /// - `&GameWorld` - Game world to query.
    /// - `&RaycastQuery2D` - Raycast query descriptor.
    ///
    /// # Returns
    ///
    /// - `Option<RaycastHit>` - Closest ray intersection hit, if any.
    #[must_use]
    pub fn raycast_query(world: &GameWorld, query: &RaycastQuery2D) -> Option<RaycastHit> {
        Self::raycast(
            world,
            query.get_origin(),
            query.get_direction(),
            query.get_max_distance(),
            query.get_layer_mask(),
        )
    }

    /// Evaluates narrow-phase overlap between two shapes returning `(normal_a_to_b, penetration)`.
    ///
    /// # Arguments
    ///
    /// - `Point` - Center of shape A.
    /// - `ColliderShape` - Geometry of shape A.
    /// - `Point` - Center of shape B.
    /// - `ColliderShape` - Geometry of shape B.
    ///
    /// # Returns
    ///
    /// - `Option<(Point, f32)>` - Contact normal and positive penetration depth if overlapping.
    fn narrow_phase(
        pos_a: Point,
        shape_a: ColliderShape,
        pos_b: Point,
        shape_b: ColliderShape,
    ) -> Option<(Point, f32)> {
        match (shape_a, shape_b) {
            (ColliderShape::Circle { radius: ra }, ColliderShape::Circle { radius: rb }) => {
                let dx: f32 = pos_b.get_x() - pos_a.get_x();
                let dy: f32 = pos_b.get_y() - pos_a.get_y();
                let r_sum: f32 = ra + rb;
                let dist_sq: f32 = dx * dx + dy * dy;
                if dist_sq >= r_sum * r_sum {
                    return None;
                }
                let dist: f32 = dist_sq.sqrt();
                if dist <= 1e-5 {
                    Some((Point::new(1.0, 0.0), r_sum))
                } else {
                    Some((Point::new(dx / dist, dy / dist), r_sum - dist))
                }
            }
            (
                ColliderShape::Aabb {
                    width: wa,
                    height: ha,
                },
                ColliderShape::Aabb {
                    width: wb,
                    height: hb,
                },
            ) => {
                let dx: f32 = pos_b.get_x() - pos_a.get_x();
                let dy: f32 = pos_b.get_y() - pos_a.get_y();
                let ox: f32 = (wa + wb) * 0.5 - dx.abs();
                if ox <= 0.0 {
                    return None;
                }
                let oy: f32 = (ha + hb) * 0.5 - dy.abs();
                if oy <= 0.0 {
                    return None;
                }
                if ox < oy {
                    let nx: f32 = if dx < 0.0 { -1.0 } else { 1.0 };
                    Some((Point::new(nx, 0.0), ox))
                } else {
                    let ny: f32 = if dy < 0.0 { -1.0 } else { 1.0 };
                    Some((Point::new(0.0, ny), oy))
                }
            }
            (ColliderShape::Circle { radius }, ColliderShape::Aabb { width, height }) => {
                Self::circle_vs_aabb(pos_a, radius, pos_b, width, height)
            }
            (ColliderShape::Aabb { width, height }, ColliderShape::Circle { radius }) => {
                Self::circle_vs_aabb(pos_b, radius, pos_a, width, height)
                    .map(|(n, pen): (Point, f32)| (Point::new(-n.get_x(), -n.get_y()), pen))
            }
        }
    }

    /// Narrow-phase Circle-vs-AABB overlap test returning normal pointing from circle to box.
    ///
    /// # Arguments
    ///
    /// - `Point` - Circle center.
    /// - `f32` - Circle radius.
    /// - `Point` - AABB center.
    /// - `f32` - AABB full width.
    /// - `f32` - AABB full height.
    ///
    /// # Returns
    ///
    /// - `Option<(Point, f32)>` - Contact normal and penetration depth.
    fn circle_vs_aabb(
        c_pos: Point,
        radius: f32,
        b_pos: Point,
        bw: f32,
        bh: f32,
    ) -> Option<(Point, f32)> {
        let half_w: f32 = bw * 0.5;
        let half_h: f32 = bh * 0.5;
        let rel_x: f32 = c_pos.get_x() - b_pos.get_x();
        let rel_y: f32 = c_pos.get_y() - b_pos.get_y();
        let closest_x: f32 = rel_x.clamp(-half_w, half_w);
        let closest_y: f32 = rel_y.clamp(-half_h, half_h);
        let dx: f32 = rel_x - closest_x;
        let dy: f32 = rel_y - closest_y;
        let dist_sq: f32 = dx * dx + dy * dy;
        if dist_sq >= radius * radius {
            return None;
        }
        let dist: f32 = dist_sq.sqrt();
        if dist <= 1e-5 {
            let ox: f32 = half_w - rel_x.abs();
            let oy: f32 = half_h - rel_y.abs();
            if ox < oy {
                let nx: f32 = if rel_x < 0.0 { 1.0 } else { -1.0 };
                Some((Point::new(nx, 0.0), ox + radius))
            } else {
                let ny: f32 = if rel_y < 0.0 { 1.0 } else { -1.0 };
                Some((Point::new(0.0, ny), oy + radius))
            }
        } else {
            Some((Point::new(-dx / dist, -dy / dist), radius - dist))
        }
    }

    /// Ray-vs-circle analytic intersection test.
    ///
    /// # Arguments
    ///
    /// - `Point` - Ray origin.
    /// - `(f32, f32)` - Normalized ray direction.
    /// - `Point` - Circle center.
    /// - `f32` - Circle radius.
    /// - `f32` - Maximum distance.
    ///
    /// # Returns
    ///
    /// - `Option<(f32, Point)>` - Hit distance and outward surface normal.
    fn ray_vs_circle(
        origin: Point,
        dir: (f32, f32),
        center: Point,
        radius: f32,
        max_dist: f32,
    ) -> Option<(f32, Point)> {
        let oc_x: f32 = origin.get_x() - center.get_x();
        let oc_y: f32 = origin.get_y() - center.get_y();
        let b: f32 = oc_x * dir.0 + oc_y * dir.1;
        let c: f32 = oc_x * oc_x + oc_y * oc_y - radius * radius;
        let disc: f32 = b * b - c;
        if disc < 0.0 {
            return None;
        }
        let t: f32 = -b - disc.sqrt();
        if t < 0.0 || t > max_dist {
            return None;
        }
        let hx: f32 = origin.get_x() + dir.0 * t;
        let hy: f32 = origin.get_y() + dir.1 * t;
        let inv_r: f32 = 1.0 / radius.max(1e-5);
        Some((
            t,
            Point::new((hx - center.get_x()) * inv_r, (hy - center.get_y()) * inv_r),
        ))
    }

    /// Ray-vs-AABB slab intersection test.
    ///
    /// # Arguments
    ///
    /// - `Point` - Ray origin.
    /// - `(f32, f32)` - Normalized ray direction.
    /// - `Rect` - Axis-aligned box rectangle.
    /// - `f32` - Maximum distance.
    ///
    /// # Returns
    ///
    /// - `Option<(f32, Point)>` - Hit distance and outward surface normal.
    fn ray_vs_aabb(
        origin: Point,
        dir: (f32, f32),
        rect: Rect,
        max_dist: f32,
    ) -> Option<(f32, Point)> {
        let mut t_min: f32 = 0.0;
        let mut t_max: f32 = max_dist;
        let mut normal: Point = Point::new(0.0, 0.0);
        if dir.0.abs() < 1e-6 {
            if origin.get_x() < rect.get_x() || origin.get_x() > rect.right() {
                return None;
            }
        } else {
            let inv: f32 = 1.0 / dir.0;
            let mut t1: f32 = (rect.get_x() - origin.get_x()) * inv;
            let mut t2: f32 = (rect.right() - origin.get_x()) * inv;
            let mut nx: f32 = -1.0;
            if t1 > t2 {
                swap(&mut t1, &mut t2);
                nx = 1.0;
            }
            if t1 > t_min {
                t_min = t1;
                normal = Point::new(nx, 0.0);
            }
            t_max = t_max.min(t2);
            if t_min > t_max {
                return None;
            }
        }
        if dir.1.abs() < 1e-6 {
            if origin.get_y() < rect.get_y() || origin.get_y() > rect.bottom() {
                return None;
            }
        } else {
            let inv: f32 = 1.0 / dir.1;
            let mut t1: f32 = (rect.get_y() - origin.get_y()) * inv;
            let mut t2: f32 = (rect.bottom() - origin.get_y()) * inv;
            let mut ny: f32 = -1.0;
            if t1 > t2 {
                swap(&mut t1, &mut t2);
                ny = 1.0;
            }
            if t1 > t_min {
                t_min = t1;
                normal = Point::new(0.0, ny);
            }
            t_max = t_max.min(t2);
            if t_min > t_max {
                return None;
            }
        }
        Some((t_min, normal))
    }
}
