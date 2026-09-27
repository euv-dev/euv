use super::*;

/// A deterministic xorshift64* pseudo-random generator used by particle
/// emitters.
///
/// The engine intentionally avoids both the `rand` crate (dependency weight
/// for a wasm target) and `js_sys::Math::random` (non-deterministic, and
/// unusable outside a JS runtime, which would make host-side unit tests
/// impossible). A seeded generator keeps emitter behavior reproducible.
#[derive(Clone, Copy, Data, Debug, New, PartialEq, PartialOrd)]
pub struct ParticleRng {
    /// The current generator state. Never zero after construction.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) state: u64,
}

/// The static configuration of a particle emitter, describing how new
/// particles are spawned and how they evolve over their lifetime.
#[derive(Clone, Copy, Data, Debug, New, PartialEq, PartialOrd)]
pub struct ParticleConfig {
    /// The number of particles spawned per second while the emitter is active.
    #[get(type(copy))]
    #[set(pub(crate))]
    pub(crate) emission_rate: f64,
    /// The maximum number of simultaneously live particles.
    #[get(type(copy))]
    #[set(pub(crate))]
    pub(crate) max_particles: usize,
    /// The minimum particle lifetime in seconds.
    #[get(type(copy))]
    #[set(pub(crate))]
    pub(crate) lifetime_min: f64,
    /// The maximum particle lifetime in seconds.
    #[get(type(copy))]
    #[set(pub(crate))]
    pub(crate) lifetime_max: f64,
    /// The minimum initial particle speed in world units per second.
    #[get(type(copy))]
    #[set(pub(crate))]
    pub(crate) speed_min: f64,
    /// The maximum initial particle speed in world units per second.
    #[get(type(copy))]
    #[set(pub(crate))]
    pub(crate) speed_max: f64,
    /// The central emission direction in radians.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) angle: f64,
    /// The total cone width around `angle` in radians within which
    /// particle directions are uniformly randomized.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) spread: f64,
    /// The constant acceleration applied to every live particle.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) gravity: Vector2D,
    /// The particle color at birth.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) color_start: Color,
    /// The particle color at death. Set its alpha to 0.0 for a fade-out.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) color_end: Color,
    /// The particle radius at birth in world units.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) size_start: f64,
    /// The particle radius at death in world units.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) size_end: f64,
}

/// A single live particle.
#[derive(Clone, Copy, Data, Debug, New, PartialEq, PartialOrd)]
pub struct Particle {
    /// The current world-space position.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) position: Vector2D,
    /// The current velocity in world units per second.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) velocity: Vector2D,
    /// The time this particle has been alive, in seconds.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) age: f64,
    /// The total lifetime of this particle, in seconds.
    #[get(type(copy))]
    #[set(pub(crate))]
    pub(crate) lifetime: f64,
}

/// A point emitter that continuously (or in bursts) spawns particles,
/// integrates their motion, and records them into a `DrawList` as colored
/// circles whose color and size interpolate over each particle's lifetime.
#[derive(Clone, Data, Debug, New, PartialEq)]
pub struct ParticleEmitter {
    /// The world-space emission point.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) position: Vector2D,
    /// The emitter configuration.
    #[get(type(copy))]
    #[set(pub(crate))]
    pub(crate) config: ParticleConfig,
    /// All currently live particles.
    #[get(type(clone))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    pub(crate) particles: Vec<Particle>,
    /// The fractional particle spawn budget carried between updates.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    pub(crate) emit_accumulator: f64,
    /// Whether continuous emission is currently enabled.
    #[get(type(copy))]
    #[set(pub(crate))]
    #[new(skip)]
    pub(crate) active: bool,
    /// The emitter's deterministic random generator.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    pub(crate) rng: ParticleRng,
}
