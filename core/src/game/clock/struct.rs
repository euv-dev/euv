use super::*;

/// Fixed-timestep accumulator and variable-rate frame telemetry clock for real-time game loops.
#[derive(Clone, Copy, Data, Debug, New, PartialEq)]
pub struct FrameClock {
    /// Fixed simulation step duration in seconds (default `1.0 / 60.0`).
    #[get(type(copy))]
    pub(crate) fixed_dt: f32,
    /// Upper cap on fixed substeps executed per rendered frame to prevent spiral-of-death stalls.
    #[get(type(copy))]
    pub(crate) max_substeps: u32,
    /// Remaining accumulated simulation time in seconds.
    #[new(skip)]
    #[get(type(copy))]
    pub(crate) accumulator: f32,
    /// Clamped variable delta time of the most recent frame in seconds.
    #[new(skip)]
    #[get(type(copy))]
    pub(crate) delta_secs: f32,
    /// Total elapsed wall/simulation time in seconds.
    #[new(skip)]
    #[get(type(copy))]
    pub(crate) elapsed_secs: f64,
    /// Interpolation alpha in `[0.0, 1.0]` between the previous and next fixed simulation states.
    #[new(skip)]
    #[get(type(copy))]
    pub(crate) alpha: f32,
    /// Smoothed frames-per-second measurement.
    #[new(skip)]
    #[get(type(copy))]
    pub(crate) fps: f32,
    /// Total rendered frames elapsed.
    #[new(skip)]
    #[get(type(copy))]
    pub(crate) frame_count: u64,
    /// Total fixed simulation steps executed.
    #[new(skip)]
    #[get(type(copy))]
    pub(crate) fixed_step_count: u64,
}
