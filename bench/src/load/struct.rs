use super::*;

/// Reactive updates plus class and style snapshots, without browser DOM calls.
#[derive(Clone, Debug, Getter, New)]
#[new(pub(crate))]
pub(crate) struct CoreLoad {
    /// Reactive text segments merged into real class and style snapshots.
    #[get(pub(crate))]
    pub(crate) attributes: Vec<AttributeValue>,
    /// Updated signal whose subscription must run on every tick.
    #[get(pub(crate), type(copy))]
    pub(crate) signal: Signal<u64>,
    /// Observed subscription notifications included in the checksum.
    #[get(pub(crate))]
    pub(crate) notifications: Rc<Cell<u64>>,
}

/// Fixed-step collision broad phase plus live-particle draw command recording.
#[derive(Clone, Data, Debug, New)]
#[new(pub(crate))]
pub(crate) struct EngineLoad {
    /// Persistent live-particle population exercised by update and rendering.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) emitter: ParticleEmitter,
    /// Reused draw-command storage retained between measured frames.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) draw: DrawList,
    /// Deterministic rigid-body scene advanced by fixed simulation steps.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) world: PhysicsWorld2D,
}

/// Dynamic chart layout and display-list generation using approximate text.
#[derive(Clone, Data, Debug, New)]
#[new(pub(crate))]
pub(crate) struct ChartLoad {
    /// Three series containing scalar, tuple and object-shaped data.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) option: ChartOption,
    /// Explicit approximate text metrics for the native workload.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) measurer: TextMeasurer,
    /// Chart renderer emitting actual display-list instructions.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) renderer: SeriesRenderer,
    /// Display-list storage reused across frames.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) list: DisplayList,
    /// Total data-point population included in work validation.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) point_count: usize,
}

/// A single frame invokes every enabled subsystem, not an aggregate of timings.
#[derive(Clone, Data, Debug, New)]
#[new(pub(crate))]
pub(crate) struct Workload {
    /// Reactive fixture present only in core and mixed suites.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) core: Option<CoreLoad>,
    /// Simulation fixture present only in engine and mixed suites.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) engine: Option<EngineLoad>,
    /// Chart fixture present only in chart and mixed suites.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) chart: Option<ChartLoad>,
}

/// Actual subsystem invocations and observable work produced by measured ticks.
#[derive(
    Clone, Copy, Data, Debug, Default, Eq, Hash, New, Ord, PartialEq, PartialOrd, Serialize,
)]
#[new(pub(crate))]
pub(crate) struct Work {
    /// Actual reactive subsystem invocations, not inferred configuration.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) core_ticks: u64,
    /// Actual simulation subsystem invocations.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) engine_ticks: u64,
    /// Actual chart subsystem invocations.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) chart_ticks: u64,
    /// Observable output checksum used to compare equivalent work.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) checksum: u64,
}
