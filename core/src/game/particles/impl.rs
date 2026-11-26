use super::*;

/// Default radial spark burst emitter configuration.
impl Default for ParticleEmitterConfig {
    /// Creates a radial neon spark burst configuration.
    ///
    /// # Returns
    ///
    /// - `Self` - Default particle emitter config.
    fn default() -> Self {
        Self {
            base_angle: 0.0,
            spread_rad: TAU,
            min_speed: 60.0,
            max_speed: 220.0,
            min_life: 0.25,
            max_life: 0.65,
            start_size: 4.5,
            end_size: 0.8,
            start_color: Color::rgba(56, 189, 248, 255),
            end_color: Color::rgba(99, 102, 241, 0),
            drag: 2.2,
        }
    }
}

/// Default 1,024-slot particle system pool.
impl Default for ParticleSystem {
    /// Allocates a `ParticleSystem` with 1,024 pooled slots.
    ///
    /// # Returns
    ///
    /// - `Self` - Ready particle system.
    fn default() -> Self {
        Self::with_capacity(1024)
    }
}

/// Particle emission, deterministic PRNG, and zero-alloc update methods for `ParticleSystem`.
impl ParticleSystem {
    /// Allocates a fixed-capacity `ParticleSystem` with `capacity` pre-allocated slots.
    ///
    /// # Arguments
    ///
    /// - `usize` - Maximum concurrent particles.
    ///
    /// # Returns
    ///
    /// - `Self` - Pre-allocated particle system.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        let cap: usize = capacity.max(16);
        Self {
            pool: vec![Particle::default(); cap],
            cursor: 0,
            rng_state: 0x9E37_79B9_7F4A_7C15,
            gravity: Point::new(0.0, 0.0),
            active_count: 0,
        }
    }

    /// Generates a deterministic pseudo-random `f32` in `[0.0, 1.0)`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable particle system.
    ///
    /// # Returns
    ///
    /// - `f32` - Normalized random scalar.
    fn next_unit_f32(&mut self) -> f32 {
        let mut s: u64 = self.get_rng_state();
        s = s
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        self.set_rng_state(s);
        ((s >> 40) as f32) / ((1_u32 << 24) as f32)
    }

    /// Emits `count` particles at `origin` using `config` without allocating heap memory.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable particle system.
    /// - `Point` - World-space emission origin.
    /// - `usize` - Number of particles to emit.
    /// - `ParticleEmitterConfig` - Emission parameters.
    pub fn emit_burst(&mut self, origin: Point, count: usize, config: ParticleEmitterConfig) {
        let cap: usize = self.get_pool().len();
        if cap == 0 {
            return;
        }
        for _ in 0..count {
            let r_ang: f32 = self.next_unit_f32();
            let r_spd: f32 = self.next_unit_f32();
            let r_life: f32 = self.next_unit_f32();
            let angle: f32 = config.get_base_angle() + (r_ang - 0.5) * config.get_spread_rad();
            let speed: f32 =
                config.get_min_speed() + (config.get_max_speed() - config.get_min_speed()) * r_spd;
            let life: f32 = (config.get_min_life()
                + (config.get_max_life() - config.get_min_life()) * r_life)
                .max(0.02);
            let slot_idx: usize = self.get_cursor() % cap;
            self.set_cursor((slot_idx + 1) % cap);
            let p: &mut Particle = &mut self.get_mut_pool()[slot_idx];
            p.set_active(true);
            p.set_position(origin);
            p.set_velocity(Point::new(angle.cos() * speed, angle.sin() * speed));
            p.set_age(0.0);
            p.set_lifetime(life);
            p.set_start_size(config.get_start_size());
            p.set_end_size(config.get_end_size());
            p.set_start_color(config.get_start_color());
            p.set_end_color(config.get_end_color());
            p.set_drag(config.get_drag());
        }
    }

    /// Advances all active particles by `dt_secs` and updates `active_count`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable particle system.
    /// - `f32` - Elapsed timestep in seconds.
    ///
    /// # Returns
    ///
    /// - `usize` - Number of particles still active after the update.
    pub fn update(&mut self, dt_secs: f32) -> usize {
        let gx: f32 = self.get_gravity().get_x();
        let gy: f32 = self.get_gravity().get_y();
        let mut live: usize = 0;
        for p in self.get_mut_pool().iter_mut() {
            if !p.get_active() {
                continue;
            }
            let next_age: f32 = p.get_age() + dt_secs;
            if next_age >= p.get_lifetime() {
                p.set_active(false);
                continue;
            }
            let damp: f32 = (1.0 - p.get_drag() * dt_secs).clamp(0.0, 1.0);
            let vx: f32 = (p.get_velocity().get_x() + gx * dt_secs) * damp;
            let vy: f32 = (p.get_velocity().get_y() + gy * dt_secs) * damp;
            let pos: Point = p.get_position();
            p.set_age(next_age);
            p.set_velocity(Point::new(vx, vy));
            p.set_position(Point::new(
                pos.get_x() + vx * dt_secs,
                pos.get_y() + vy * dt_secs,
            ));
            live += 1;
        }
        self.set_active_count(live);
        live
    }

    /// Clears all active particles immediately.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable particle system.
    pub fn clear(&mut self) {
        for p in self.get_mut_pool().iter_mut() {
            p.set_active(false);
        }
        self.set_active_count(0);
    }
}
