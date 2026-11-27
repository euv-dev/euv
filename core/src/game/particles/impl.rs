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

/// Particle emission, deterministic PRNG, and active-prefix SoA kinematics for `ParticleSystem`.
impl ParticleSystem {
    /// Allocates a fixed-capacity `ParticleSystem` with `capacity` pre-allocated SoA slots.
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
            capacity: cap,
            pos_x: vec![0.0; cap],
            pos_y: vec![0.0; cap],
            vel_x: vec![0.0; cap],
            vel_y: vec![0.0; cap],
            age: vec![0.0; cap],
            lifetime: vec![0.0; cap],
            drag: vec![0.0; cap],
            start_size: vec![0.0; cap],
            end_size: vec![0.0; cap],
            start_color: vec![Color::default(); cap],
            end_color: vec![Color::default(); cap],
            min_remaining_life: f32::INFINITY,
            cursor: 0,
            rng_state: 0x9E37_79B9_7F4A_7C15,
            gravity: Point::new(0.0, 0.0),
            active_count: 0,
        }
    }

    /// Generates a deterministic pseudo-random `f32` in `[0.0, 1.0)` advancing `state`.
    ///
    /// # Arguments
    ///
    /// - `&mut u64` - Mutable PRNG state word.
    ///
    /// # Returns
    ///
    /// - `f32` - Normalized random scalar.
    #[inline(always)]
    fn next_unit_f32_from(state: &mut u64) -> f32 {
        let next: u64 = state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        *state = next;
        ((next >> 40) as f32) * (1.0 / ((1_u32 << 24) as f32))
    }

    /// Emits `count` particles at `origin` using `config` into the compacted active prefix without heap allocations.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable particle system.
    /// - `Point` - World-space emission origin.
    /// - `usize` - Number of particles to emit.
    /// - `ParticleEmitterConfig` - Emission parameters.
    pub fn emit_burst(&mut self, origin: Point, count: usize, config: ParticleEmitterConfig) {
        let Self {
            capacity,
            pos_x,
            pos_y,
            vel_x,
            vel_y,
            age,
            lifetime,
            drag,
            start_size,
            end_size,
            start_color,
            end_color,
            min_remaining_life,
            cursor,
            rng_state,
            active_count,
            ..
        } = self;
        let cap: usize = *capacity;
        if cap == 0 || count == 0 {
            return;
        }
        let base_angle: f32 = config.get_base_angle();
        let spread_rad: f32 = config.get_spread_rad();
        let min_speed: f32 = config.get_min_speed();
        let speed_span: f32 = config.get_max_speed() - min_speed;
        let min_life: f32 = config.get_min_life();
        let life_span: f32 = config.get_max_life() - min_life;
        let s_size: f32 = config.get_start_size();
        let e_size: f32 = config.get_end_size();
        let s_col: Color = config.get_start_color();
        let e_col: Color = config.get_end_color();
        let p_drag: f32 = config.get_drag();
        let ox: f32 = origin.get_x();
        let oy: f32 = origin.get_y();
        let mut cur_rng: u64 = *rng_state;
        let mut live: usize = *active_count;
        let mut cur_slot: usize = *cursor;
        let mut min_rem: f32 = *min_remaining_life;
        for _ in 0..count {
            let r_ang: f32 = Self::next_unit_f32_from(&mut cur_rng);
            let r_spd: f32 = Self::next_unit_f32_from(&mut cur_rng);
            let r_life: f32 = Self::next_unit_f32_from(&mut cur_rng);
            let angle: f32 = base_angle + (r_ang - 0.5) * spread_rad;
            let speed: f32 = min_speed + speed_span * r_spd;
            let life: f32 = (min_life + life_span * r_life).max(0.02);
            let slot_idx: usize = if live < cap {
                let s: usize = live;
                live += 1;
                s
            } else {
                let s: usize = cur_slot % cap;
                cur_slot = (s + 1) % cap;
                s
            };
            let (sin_a, cos_a): (f32, f32) = angle.sin_cos();
            pos_x[slot_idx] = ox;
            pos_y[slot_idx] = oy;
            vel_x[slot_idx] = cos_a * speed;
            vel_y[slot_idx] = sin_a * speed;
            age[slot_idx] = 0.0;
            lifetime[slot_idx] = life;
            drag[slot_idx] = p_drag;
            start_size[slot_idx] = s_size;
            end_size[slot_idx] = e_size;
            start_color[slot_idx] = s_col;
            end_color[slot_idx] = e_col;
            min_rem = min_rem.min(life);
        }
        *rng_state = cur_rng;
        *active_count = live;
        *cursor = cur_slot;
        *min_remaining_life = min_rem;
    }

    /// Advances all active particles in `[0..active_count)` via auto-vectorized SoA kinematics and swap-removes expired slots.
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
        let Self {
            pos_x,
            pos_y,
            vel_x,
            vel_y,
            age,
            lifetime,
            drag,
            start_size,
            end_size,
            start_color,
            end_color,
            min_remaining_life,
            gravity,
            active_count,
            ..
        } = self;
        let mut n: usize = *active_count;
        if n == 0 {
            return 0;
        }
        let gx_dt: f32 = gravity.get_x() * dt_secs;
        let gy_dt: f32 = gravity.get_y() * dt_secs;
        let neg_dt: f32 = -dt_secs;
        let next_min_bound: f32 = *min_remaining_life - dt_secs;
        if next_min_bound > 0.0 {
            let px: &mut [f32] = &mut pos_x[..n];
            let py: &mut [f32] = &mut pos_y[..n];
            let vx: &mut [f32] = &mut vel_x[..n];
            let vy: &mut [f32] = &mut vel_y[..n];
            let ag: &mut [f32] = &mut age[..n];
            let dr: &[f32] = &drag[..n];
            for i in 0..n {
                ag[i] += dt_secs;
                let damp: f32 = dr[i].mul_add(neg_dt, 1.0).clamp(0.0, 1.0);
                let nvx: f32 = (vx[i] + gx_dt) * damp;
                let nvy: f32 = (vy[i] + gy_dt) * damp;
                vx[i] = nvx;
                vy[i] = nvy;
                px[i] = nvx.mul_add(dt_secs, px[i]);
                py[i] = nvy.mul_add(dt_secs, py[i]);
            }
            *min_remaining_life = next_min_bound;
            return n;
        }
        let mut i: usize = 0;
        let mut recomputed_min_rem: f32 = f32::INFINITY;
        while i < n {
            let next_age: f32 = age[i] + dt_secs;
            let life: f32 = lifetime[i];
            if next_age >= life {
                n -= 1;
                if i < n {
                    pos_x[i] = pos_x[n];
                    pos_y[i] = pos_y[n];
                    vel_x[i] = vel_x[n];
                    vel_y[i] = vel_y[n];
                    age[i] = age[n];
                    lifetime[i] = lifetime[n];
                    drag[i] = drag[n];
                    start_size[i] = start_size[n];
                    end_size[i] = end_size[n];
                    start_color[i] = start_color[n];
                    end_color[i] = end_color[n];
                }
                continue;
            }
            let damp: f32 = drag[i].mul_add(neg_dt, 1.0).clamp(0.0, 1.0);
            let nvx: f32 = (vel_x[i] + gx_dt) * damp;
            let nvy: f32 = (vel_y[i] + gy_dt) * damp;
            age[i] = next_age;
            vel_x[i] = nvx;
            vel_y[i] = nvy;
            pos_x[i] = nvx.mul_add(dt_secs, pos_x[i]);
            pos_y[i] = nvy.mul_add(dt_secs, pos_y[i]);
            recomputed_min_rem = recomputed_min_rem.min(life - next_age);
            i += 1;
        }
        *active_count = n;
        *min_remaining_life = recomputed_min_rem;
        n
    }

    /// Returns a snapshot of the active particle at compacted slot `index`, or `None` if `index >= active_count`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Particle system reference.
    /// - `usize` - Active particle index in `0..active_count`.
    ///
    /// # Returns
    ///
    /// - `Option<Particle>` - Reconstructed `Particle` record for inspection.
    #[must_use]
    pub fn active_particle(&self, index: usize) -> Option<Particle> {
        let Self {
            pos_x,
            pos_y,
            vel_x,
            vel_y,
            age,
            lifetime,
            drag,
            start_size,
            end_size,
            start_color,
            end_color,
            active_count,
            ..
        } = self;
        if index >= *active_count {
            return None;
        }
        Some(Particle {
            active: true,
            position: Point::new(pos_x[index], pos_y[index]),
            velocity: Point::new(vel_x[index], vel_y[index]),
            age: age[index],
            lifetime: lifetime[index],
            start_size: start_size[index],
            end_size: end_size[index],
            start_color: start_color[index],
            end_color: end_color[index],
            drag: drag[index],
        })
    }

    /// Clears all active particles in O(1) while retaining pre-allocated SoA lane capacities.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable particle system.
    pub fn clear(&mut self) {
        let Self {
            active_count,
            cursor,
            min_remaining_life,
            ..
        } = self;
        *active_count = 0;
        *cursor = 0;
        *min_remaining_life = f32::INFINITY;
    }
}
