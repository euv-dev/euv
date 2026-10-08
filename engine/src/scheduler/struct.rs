use super::*;

/// Configuration parameters for the fixed-timestep scheduler.
#[derive(Clone, Copy, Data, Debug, New, PartialEq, PartialOrd)]
pub struct SchedulerConfig {
    /// The fixed simulation timestep in seconds (e.g., 1/60 for 60 Hz updates).
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) fixed_timestep: f64,
    /// The maximum allowed frame time in seconds before the scheduler starts dropping updates.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) max_frame_time: f64,
}

/// The runtime state of a scheduler instance.
#[derive(Clone, Data, Debug, New, PartialEq)]
pub struct SchedulerState {
    /// The accumulated time waiting to be processed by fixed updates.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    pub(crate) accumulator: f64,
    /// The timestamp of the previous frame in seconds, or `UNINITIALIZED_TIME` before the first frame.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) last_time: f64,
    /// Whether the scheduler is currently running and scheduling animation frames.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    pub(crate) running: bool,
    /// The most recent `requestAnimationFrame` ID, used to cancel the next frame.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    pub(crate) raf_id: Option<i32>,
    /// The total number of fixed update steps executed since the scheduler started.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    pub(crate) update_count: u64,
    /// The total number of render frames executed since the scheduler started.
    #[get(type(copy))]
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
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) state: Rc<EngineCell<SchedulerState>>,
    /// The shared closure cell keeping the RAF callback alive. Held
    /// behind `MaybeEngineCell` because the cell is empty both
    /// before `spawn` runs and after cleanup tears the closure down.
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) closure_cell: RafClosureCell,
}

/// A registry of [`Updatable`] tasks driven by the fixed-timestep scheduler.
///
/// The scheduler itself only knows how to run a single [`TickHandler`]; every
/// other simulation object in the engine — `Timer`, `Tween`,
/// `ParticleEmitter`, `Entity`, `Animator`, `SceneManager`, and the
/// `PhysicsWorld2D` / `PhysicsWorld3D` containers — exposes its advancement
/// through the [`Updatable`] trait instead. This registry is the driver that
/// gives those objects a heartbeat: [`SchedulerState::tick`] calls
/// [`TaskRegistry::update_all`] once per fixed step, immediately *after* the
/// handler's `on_update` callback returns, so gameplay logic registered in
/// `on_update` sees task state that has already advanced this step.
///
/// Tasks are updated in registration order, which makes the relative ordering
/// of independent tasks explicit and reproducible rather than dependent on
/// container iteration order.
#[derive(Data, Default, New)]
pub struct TaskRegistry {
    /// The registered tasks, in registration order. `Box<dyn Updatable>`
    /// erases the concrete task type so heterogeneous tasks (a `Timer` next
    /// to a `Tween<f64>` next to a `ParticleEmitter`) coexist in one list.
    #[get_mut(pub(crate))]
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) tasks: Vec<Box<dyn Updatable>>,
}

/// A handle to a task registered with a [`TaskRegistry`].
///
/// The handle is returned by [`TaskRegistry::register`] and is the only way
/// to remove that task later. It identifies the task by its index in the
/// registry's insertion-ordered task list, which keeps registration and
/// removal O(1) for the common append-then-remove-last pattern.
#[derive(Clone, Copy, Data, Debug, New, PartialEq, PartialOrd)]
pub struct TaskHandle {
    /// The zero-based index of the task in the registry's task list.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    pub(crate) id: u64,
}
