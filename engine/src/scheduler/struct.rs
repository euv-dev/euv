use super::*;

/// Configuration parameters for the fixed-timestep scheduler.
#[derive(Clone, Copy, Data, Debug, New, PartialEq, PartialOrd)]
pub struct SchedulerConfig {
    /// The fixed simulation timestep in seconds (e.g., 1/60 for 60 Hz updates).
    #[get(type(copy))]
    pub(crate) fixed_timestep: f64,
    /// The maximum allowed frame time in seconds before the scheduler starts dropping updates.
    #[get(type(copy))]
    pub(crate) max_frame_time: f64,
}

/// The runtime state of a scheduler instance.
#[derive(Clone, Data, Debug, New, PartialEq)]
pub struct SchedulerState {
    /// The accumulated time waiting to be processed by fixed updates.
    #[get(pub, type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    pub(crate) accumulator: f64,
    /// The timestamp of the previous frame in seconds, or `UNINITIALIZED_TIME` before the first frame.
    #[get(pub, type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) last_time: f64,
    /// Whether the scheduler is currently running and scheduling animation frames.
    #[get(pub, type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    pub(crate) running: bool,
    /// The most recent `requestAnimationFrame` ID, used to cancel the next frame.
    #[get(pub, type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    pub(crate) raf_id: Option<i32>,
    /// The total number of fixed update steps executed since the scheduler started.
    #[get(pub, type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    pub(crate) update_count: u64,
    /// The total number of render frames executed since the scheduler started.
    #[get(pub, type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    pub(crate) frame_count: u64,
}

/// A handle to a running scheduler, allowing the caller to stop it later.
#[derive(Clone, Data, New)]
pub struct SchedulerHandle {
    /// The shared scheduler state, held behind `EngineCell` (an
    /// `UnsafeCell`-backed `Sync` newtype) so multiple closure
    /// captures can mutate it without `RefCell`'s runtime borrow
    /// check. Mirrors `core::reactive::schedule` shape.
    #[get(pub)]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) state: Rc<EngineCell<SchedulerState>>,
    /// The shared closure cell keeping the RAF callback alive. Held
    /// behind `MaybeEngineCell` because the cell is empty both
    /// before `spawn` runs and after cleanup tears the closure down.
    #[get(pub)]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) closure_cell: RafClosureCell,
}
