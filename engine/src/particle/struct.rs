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
    pub state: u64,
}

/// The static configuration of a particle emitter, describing how new
/// particles are spawned and how they evolve over their lifetime.
///
/// Every field is publicly settable, so a caller composes a config by
/// starting from [`ParticleConfig::default`] and overriding the tunables it
/// cares about before handing it to [`ParticleEmitter::create`].
#[derive(Clone, Copy, Data, Debug, New, PartialEq, PartialOrd)]
pub struct ParticleConfig {
    /// The number of particles spawned per second while the emitter is active.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    pub emission_rate: f64,
    /// The maximum number of simultaneously live particles.
    #[get(pub(crate), type(copy))]
    pub max_particles: usize,
    /// The minimum particle lifetime in seconds.
    #[get(type(copy))]
    pub lifetime_min: f64,
    /// The maximum particle lifetime in seconds.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    pub lifetime_max: f64,
    /// The minimum initial particle speed in world units per second.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    pub speed_min: f64,
    /// The maximum initial particle speed in world units per second.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    pub speed_max: f64,
    /// The central emission direction in radians.
    #[get(pub(crate), type(copy))]
    #[new(skip)]
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub(crate) angle: f64,
    /// The total cone width around `angle` in radians within which
    /// particle directions are uniformly randomized.
    #[get(pub(crate), type(copy))]
    #[new(skip)]
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub(crate) spread: f64,
    /// The constant acceleration applied to every live particle.
    #[get(pub(crate), type(copy))]
    #[new(skip)]
    #[get_mut(pub(crate))]
    pub gravity: Vector2D,
    /// The particle color at birth.
    #[get(pub(crate), type(copy))]
    #[new(skip)]
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub(crate) color_start: Color,
    /// The particle color at death. Set its alpha to 0.0 for a fade-out.
    #[get(pub(crate), type(copy))]
    #[new(skip)]
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub(crate) color_end: Color,
    /// The particle radius at birth in world units.
    #[get(pub(crate), type(copy))]
    #[new(skip)]
    #[get_mut(pub(crate))]
    pub size_start: f64,
    /// The particle radius at death in world units.
    #[get(pub(crate), type(copy))]
    #[new(skip)]
    #[get_mut(pub(crate))]
    pub size_end: f64,
}

/// A single live particle.
#[derive(Clone, Copy, Data, Debug, New, PartialEq, PartialOrd)]
pub struct Particle {
    /// The current world-space position.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    pub position: Vector2D,
    #[get(type(clone))]
    #[get_mut(pub(crate))]
    pub velocity: Vector2D,
    /// The time this particle has been alive, in seconds.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) age: f64,
    /// The total lifetime of this particle, in seconds.
    #[get(type(copy))]
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub lifetime: f64,
}

/// A point emitter that continuously (or in bursts) spawns particles,
/// integrates their motion, and records them into a `DrawList` as colored
/// circles whose color and size interpolate over each particle's lifetime.
#[derive(Clone, Data, Debug, New, PartialEq)]
pub struct ParticleEmitter {
    /// The world-space emission point.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    pub position: Vector2D,
    /// The emitter configuration.
    #[get(type(clone))]
    #[get_mut(pub(crate))]
    pub config: ParticleConfig,
    /// All currently live particles.
    #[get(type(clone))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    pub particles: Vec<Particle>,
    /// The fractional particle spawn budget carried between updates.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    pub emit_accumulator: f64,
    /// Whether continuous emission is currently enabled.
    #[get(pub(crate), type(copy))]
    #[new(skip)]
    #[get_mut(pub(crate))]
    pub active: bool,
    /// The emitter's deterministic random generator.
    #[get(type(copy), pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    pub(crate) rng: ParticleRng,
}
