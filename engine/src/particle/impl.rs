use super::*;

/// Implements deterministic pseudo-random number generation for `ParticleRng`.
impl ParticleRng {
    /// Creates a generator from the given seed. A zero seed is replaced with
    /// the default seed, since xorshift degenerates at zero.
    ///
    /// # Arguments
    ///
    /// - `u64` - The seed value.
    ///
    /// # Returns
    ///
    /// - `ParticleRng` - The seeded generator.
    pub fn with_seed(seed: u64) -> ParticleRng {
        if seed == 0 {
            ParticleRng::new(PARTICLE_DEFAULT_RNG_SEED)
        } else {
            ParticleRng::new(seed)
        }
    }

    /// Advances the generator and returns the next 64-bit value.
    ///
    /// # Returns
    ///
    /// - `u64` - The next pseudo-random value.
    pub fn next_u64(&mut self) -> u64 {
        let mut state: u64 = self.get_state();
        state ^= state >> 12;
        state ^= state << 25;
        state ^= state >> 27;
        self.set_state(state);
        state.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    /// Returns the next pseudo-random value uniformly distributed in [0.0, 1.0).
    ///
    /// # Returns
    ///
    /// - `f64` - The next value in the unit interval.
    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }

    /// Returns the next pseudo-random value uniformly distributed in [min, max).
    ///
    /// # Arguments
    ///
    /// - `f64` - The inclusive lower bound.
    /// - `f64` - The exclusive upper bound.
    ///
    /// # Returns
    ///
    /// - `f64` - The next value in the given range.
    pub fn range(&mut self, min: f64, max: f64) -> f64 {
        min + (max - min) * self.next_f64()
    }
}

/// Implements `Default` for `ParticleRng` using the default seed.
impl Default for ParticleRng {
    /// Constructs a default [`ParticleRng`] value.
    ///
    /// # Returns
    ///
    /// - `ParticleRng` - A default-constructed instance with the documented initial state.
    fn default() -> ParticleRng {
        ParticleRng::with_seed(PARTICLE_DEFAULT_RNG_SEED)
    }
}

/// Implements `Default` for `ParticleConfig` with sensible engine defaults.
impl Default for ParticleConfig {
    /// Constructs a default [`ParticleConfig`] value.
    ///
    /// # Returns
    ///
    /// - `ParticleConfig` - A default-constructed instance with the documented initial state.
    fn default() -> ParticleConfig {
        ParticleConfig {
            emission_rate: PARTICLE_DEFAULT_EMISSION_RATE,
            max_particles: PARTICLE_DEFAULT_MAX_COUNT,
            lifetime_min: PARTICLE_DEFAULT_LIFETIME_MIN,
            lifetime_max: PARTICLE_DEFAULT_LIFETIME_MAX,
            speed_min: PARTICLE_DEFAULT_SPEED_MIN,
            speed_max: PARTICLE_DEFAULT_SPEED_MAX,
            angle: PARTICLE_DEFAULT_ANGLE,
            spread: PARTICLE_DEFAULT_SPREAD,
            gravity: Vector2D::zero(),
            color_start: Color::white(),
            color_end: Color::transparent(),
            size_start: PARTICLE_DEFAULT_SIZE_START,
            size_end: PARTICLE_DEFAULT_SIZE_END,
        }
    }
}

/// Implements creation, simulation, and rendering for `ParticleEmitter`.
impl ParticleEmitter {
    /// Creates an active emitter at the given position with the given config.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The world-space emission point.
    /// - `ParticleConfig` - The emitter configuration.
    ///
    /// # Returns
    ///
    /// - `ParticleEmitter` - The new emitter.
    pub fn create(position: Vector2D, config: ParticleConfig) -> ParticleEmitter {
        let mut emitter: ParticleEmitter = ParticleEmitter::new(position, config);
        emitter.set_active(true);
        emitter
    }

    /// Creates an active emitter at the given position using the default
    /// configuration.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The world-space emission point.
    ///
    /// # Returns
    ///
    /// - `ParticleEmitter` - The new emitter.
    pub fn with_defaults(position: Vector2D) -> ParticleEmitter {
        ParticleEmitter::create(position, ParticleConfig::default())
    }

    /// Advances the emitter by the given delta time: spawns new particles
    /// from the emission budget while active, integrates motion and gravity,
    /// and removes expired particles.
    ///
    /// Integration and expiry share a single sweep over the live particles.
    /// A read cursor integrates each particle and a write cursor compacts the
    /// survivors downward, so the whole update costs one pass instead of the
    /// integrate pass plus a separate `retain` pass. Survivors keep their
    /// relative order: a dead slot is only ever refilled by swapping a live
    /// particle into it, and the element that gets displaced lands past the
    /// final truncation point. The compaction also moves no memory when
    /// nothing expired, where `retain` still walks the whole vector.
    ///
    /// # Arguments
    ///
    /// - `f64` - The time elapsed since the last update, in seconds.
    pub fn update(&mut self, delta_time: f64) {
        let delta_time: f64 = delta_time.max(0.0);
        let config: ParticleConfig = self.get_config();
        if self.get_active() {
            *self.get_mut_emit_accumulator() += config.get_emission_rate() * delta_time;
            let mut spawn_count: usize = self.get_emit_accumulator() as usize;
            let capacity: usize = config
                .get_max_particles()
                .saturating_sub(self.get_mut_particles().len());
            spawn_count = spawn_count.min(capacity);
            *self.get_mut_emit_accumulator() -= spawn_count as f64;
            for _ in 0..spawn_count {
                self.spawn_particle();
            }
        }
        let gravity_step: Vector2D = config.get_gravity().scaled(delta_time);
        let particles: &mut Vec<Particle> = self.get_mut_particles();
        let mut survivors: usize = 0;
        for index in 0..particles.len() {
            let expired: bool = {
                let particle: &mut Particle = &mut particles[index];
                *particle.get_mut_velocity() += gravity_step;
                let velocity: Vector2D = particle.get_velocity();
                *particle.get_mut_position() += velocity.scaled(delta_time);
                *particle.get_mut_age() += delta_time;
                particle.get_age() >= particle.get_lifetime()
            };
            if !expired {
                if survivors != index {
                    particles.swap(survivors, index);
                }
                survivors += 1;
            }
        }
        particles.truncate(survivors);
    }

    /// Spawns the given number of particles immediately, regardless of the
    /// active flag, clamped to the remaining particle capacity.
    ///
    /// # Arguments
    ///
    /// - `usize` - The number of particles to spawn.
    pub fn burst(&mut self, count: usize) {
        let capacity: usize = self
            .get_config()
            .get_max_particles()
            .saturating_sub(self.get_particles().len());
        for _ in 0..count.min(capacity) {
            self.spawn_particle();
        }
    }

    /// Records all live particles into the given draw list as filled circles.
    ///
    /// Each particle's color and radius are interpolated between the
    /// configured start and end values by its normalized age, and the
    /// color is then snapped to the nearest entry in `PARTICLE_PALETTE`
    /// via `quantize`. Quantizing is visually nearly identical for
    /// gradient particles (32-bucket quantization of a smooth gradient is
    /// imperceptible at canvas resolution) and dramatically increases
    /// the number of particles that share the same `DrawCommand::FillCircle::color`
    /// field, so the DrawList replay loop coalesces `set_fill_style_str`
    /// calls across the entire batch instead of switching style for every
    /// particle.
    ///
    /// # Arguments
    ///
    /// - `&mut DrawList` - The draw list to record commands into.
    pub fn render(&self, draw_list: &mut DrawList) {
        let config: ParticleConfig = self.get_config();
        for particle in self.get_particles().iter() {
            let t: f64 = if particle.get_lifetime() > 0.0 {
                (particle.get_age() / particle.get_lifetime()).min(1.0)
            } else {
                1.0
            };
            let color: Color = config.get_color_start().lerp(config.get_color_end(), t);
            let radius: f64 = Numeric::lerp(config.get_size_start(), config.get_size_end(), t);
            if radius <= 0.0 || color.get_alpha() <= 0.0 {
                continue;
            }
            let bucket: u8 = Self::quantize(&color);
            let quantized_color: Color = PARTICLE_PALETTE[bucket as usize];
            draw_list.fill_circle(particle.get_position(), radius, quantized_color);
        }
    }

    /// Returns the number of currently live particles.
    ///
    /// # Returns
    ///
    /// - `usize` - The live particle count.
    pub fn alive_count(&self) -> usize {
        self.get_particles().len()
    }

    /// Picks the palette index nearest to the given color by squared
    /// Euclidean distance in 4-channel RGBA space.
    ///
    /// Used by the render path to snap each particle's interpolated color
    /// to one of `PARTICLE_PALETTE_SIZE` preset colors. Quantizing collapses
    /// the smooth color gradient produced by `Color::lerp` into a small set
    /// of buckets, so many particles share the same `DrawCommand::FillCircle::color`
    /// and the DrawList replay loop can coalesce `set_fill_style_str` calls
    /// across the entire batch instead of switching style for every particle.
    ///
    /// # Arguments
    ///
    /// - `&Color` - The particle color to quantize.
    ///
    /// # Returns
    ///
    /// - `u8` - The index into `PARTICLE_PALETTE` of the nearest entry. The
    ///   result fits in `u8` because `PARTICLE_PALETTE_SIZE` is 32.
    pub(crate) fn quantize(color: &Color) -> u8 {
        let target_red: f64 = color.get_red();
        let target_green: f64 = color.get_green();
        let target_blue: f64 = color.get_blue();
        let target_alpha: f64 = color.get_alpha();
        let mut best_index: usize = 0;
        let mut best_distance: f64 = f64::INFINITY;
        for (index, candidate) in PARTICLE_PALETTE.iter().enumerate() {
            let delta_red: f64 = candidate.get_red() - target_red;
            let delta_green: f64 = candidate.get_green() - target_green;
            let delta_blue: f64 = candidate.get_blue() - target_blue;
            let delta_alpha: f64 = candidate.get_alpha() - target_alpha;
            let distance: f64 = delta_red * delta_red
                + delta_green * delta_green
                + delta_blue * delta_blue
                + delta_alpha * delta_alpha;
            if distance < best_distance {
                best_distance = distance;
                best_index = index;
            }
        }
        best_index as u8
    }

    /// Removes all live particles without changing the active flag.
    pub fn clear(&mut self) {
        self.get_mut_particles().clear();
        self.set_emit_accumulator(0.0);
    }

    /// Spawns a single particle with randomized direction, speed, and
    /// lifetime sampled from the configuration ranges.
    fn spawn_particle(&mut self) {
        let config: ParticleConfig = self.get_config();
        let half_spread: f64 = config.get_spread() / 2.0;
        let angle: f64 = config.get_angle() + self.get_mut_rng().range(-half_spread, half_spread);
        let speed: f64 = self
            .get_mut_rng()
            .range(config.get_speed_min(), config.get_speed_max());
        let lifetime: f64 = self
            .get_mut_rng()
            .range(config.get_lifetime_min(), config.get_lifetime_max())
            .max(EPSILON);
        let position: Vector2D = self.get_position();
        let particle: Particle = Particle::new(
            position,
            Vector2D::from_angle(angle).scaled(speed),
            0.0,
            lifetime,
        );
        self.get_mut_particles().push(particle);
    }
}

/// Forwards `ParticleEmitter::update` through the [`Updatable`] trait so
/// emitters can participate in the same generic update loop as entities,
/// animators, scenes, and physics worlds.
impl Updatable for ParticleEmitter {
    /// Advances the simulation by `delta_time` seconds.
    ///
    /// # Arguments
    ///
    /// - `f64` - Seconds elapsed since the previous update.
    fn update(&mut self, delta_time: f64) {
        ParticleEmitter::update(self, delta_time);
    }
}
