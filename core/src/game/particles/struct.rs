use super::*;

/// Individual pooled particle simulated without heap allocations.
#[derive(Clone, Copy, Data, Debug, Default, PartialEq)]
pub struct Particle {
    /// Whether this pool slot is currently alive.
    #[get(type(copy))]
    pub(crate) active: bool,
    /// Current world-space position.
    #[get(type(copy))]
    pub(crate) position: Point,
    /// Current linear velocity in world units per second.
    #[get(type(copy))]
    pub(crate) velocity: Point,
    /// Elapsed particle age in seconds.
    #[get(type(copy))]
    pub(crate) age: f32,
    /// Total lifetime in seconds.
    #[get(type(copy))]
    pub(crate) lifetime: f32,
    /// Initial radius in world units.
    #[get(type(copy))]
    pub(crate) start_size: f32,
    /// Final radius at end of life.
    #[get(type(copy))]
    pub(crate) end_size: f32,
    /// Initial RGBA color.
    #[get(type(copy))]
    pub(crate) start_color: Color,
    /// Final RGBA color interpolated over lifetime.
    #[get(type(copy))]
    pub(crate) end_color: Color,
    /// Linear drag per second.
    #[get(type(copy))]
    pub(crate) drag: f32,
}

/// Declarative burst/stream emitter configuration for `ParticleSystem`.
#[derive(Clone, Copy, Data, Debug, PartialEq)]
pub struct ParticleEmitterConfig {
    /// Center direction angle in radians.
    #[get(type(copy))]
    pub(crate) base_angle: f32,
    /// Angular cone spread in radians (`TAU` for full radial explosion).
    #[get(type(copy))]
    pub(crate) spread_rad: f32,
    /// Minimum initial launch speed.
    #[get(type(copy))]
    pub(crate) min_speed: f32,
    /// Maximum initial launch speed.
    #[get(type(copy))]
    pub(crate) max_speed: f32,
    /// Minimum lifetime in seconds.
    #[get(type(copy))]
    pub(crate) min_life: f32,
    /// Maximum lifetime in seconds.
    #[get(type(copy))]
    pub(crate) max_life: f32,
    /// Starting particle radius.
    #[get(type(copy))]
    pub(crate) start_size: f32,
    /// Ending particle radius.
    #[get(type(copy))]
    pub(crate) end_size: f32,
    /// Starting particle color.
    #[get(type(copy))]
    pub(crate) start_color: Color,
    /// Ending particle color.
    #[get(type(copy))]
    pub(crate) end_color: Color,
    /// Linear drag per second.
    #[get(type(copy))]
    pub(crate) drag: f32,
}

/// Zero-allocation fixed-capacity 2D particle simulation engine.
#[derive(Clone, Data, Debug)]
pub struct ParticleSystem {
    /// Pre-allocated contiguous particle pool.
    pub(crate) pool: Vec<Particle>,
    /// Next circular allocation cursor.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    pub(crate) cursor: usize,
    /// Deterministic PRNG state for cross-platform bit-identical particle bursts.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    pub(crate) rng_state: u64,
    /// Gravity applied to active particles.
    #[get(type(copy))]
    pub(crate) gravity: Point,
    /// Live active particle count updated during `update`.
    #[get(type(copy))]
    pub(crate) active_count: usize,
}
