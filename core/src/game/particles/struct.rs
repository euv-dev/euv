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

/// Zero-allocation fixed-capacity 2D particle simulation engine using active-prefix SoA hot kinematics.
#[derive(Clone, Data, Debug)]
pub struct ParticleSystem {
    /// Maximum concurrent particle slots pre-allocated in every SoA lane.
    #[get(type(copy))]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) capacity: usize,
    /// Hot SoA world-space X coordinates compacted in `[0..active_count)`.
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) pos_x: Vec<f32>,
    /// Hot SoA world-space Y coordinates compacted in `[0..active_count)`.
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) pos_y: Vec<f32>,
    /// Hot SoA X velocities compacted in `[0..active_count)`.
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) vel_x: Vec<f32>,
    /// Hot SoA Y velocities compacted in `[0..active_count)`.
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) vel_y: Vec<f32>,
    /// Hot SoA elapsed particle ages in seconds compacted in `[0..active_count)`.
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) age: Vec<f32>,
    /// Hot SoA total particle lifetimes in seconds compacted in `[0..active_count)`.
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) lifetime: Vec<f32>,
    /// Hot SoA linear drag coefficients compacted in `[0..active_count)`.
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) drag: Vec<f32>,
    /// Cold SoA starting particle radii aligned 1:1 with `[0..active_count)`.
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) start_size: Vec<f32>,
    /// Cold SoA ending particle radii aligned 1:1 with `[0..active_count)`.
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) end_size: Vec<f32>,
    /// Cold SoA starting RGBA colors aligned 1:1 with `[0..active_count)`.
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) start_color: Vec<Color>,
    /// Cold SoA ending RGBA colors aligned 1:1 with `[0..active_count)`.
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) end_color: Vec<Color>,
    /// Conservative lower bound on remaining lifetime across active particles enabling branchless updates.
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) min_remaining_life: f32,
    /// Next circular overwrite cursor used when `active_count == capacity`.
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
    /// Live active particle count occupying `[0..active_count)`.
    #[get(type(copy))]
    pub(crate) active_count: usize,
}
