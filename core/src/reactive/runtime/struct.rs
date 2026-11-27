use super::*;

/// Storage entry for a type-erased signal inside the reactive arena.
#[derive(CustomDebug, Data)]
pub(crate) struct SignalEntry {
    /// Generation counter for this slot.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) generation: u32,
    /// Monotonic version counter incremented on every write.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) version: u64,
    /// Type-erased signal payload; `None` when slot is free.
    #[debug(skip)]
    #[get(skip)]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) value: Option<Box<dyn Any>>,
    /// Downstream subscribers that depend on this signal.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) subscribers: SmallSubList<SubscriberId>,
}

/// Storage entry for a memoized derived computation inside the reactive arena.
#[derive(CustomDebug, Data)]
pub(crate) struct MemoEntry {
    /// Generation counter for this slot.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) generation: u32,
    /// Monotonic version counter incremented when computed value changes.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) version: u64,
    /// Current push-pull evaluation state.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) state: NodeState,
    /// Cached computed value; `None` when uninitialized or freed.
    #[debug(skip)]
    #[get(skip)]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) value: Option<Box<dyn Any>>,
    /// Type-erased recomputation closure returning `(new_value, changed)`.
    #[debug(skip)]
    #[get(skip)]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) compute: Option<MemoComputeFn>,
    /// Upstream dependencies tracked during the previous evaluation.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) dependencies: SmallSubList<DependencyId>,
    /// Downstream subscribers depending on this memo.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) subscribers: SmallSubList<SubscriberId>,
    /// Topological depth level used to evaluate memos before effects in order.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) depth: u32,
}

/// Storage entry for a reactive side effect inside the reactive arena.
#[derive(CustomDebug, Data)]
pub(crate) struct EffectEntry {
    /// Generation counter for this slot.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) generation: u32,
    /// Whether this effect is currently alive.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) alive: bool,
    /// Whether this effect is queued for execution.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) scheduled: bool,
    /// Type-erased side-effect closure.
    #[debug(skip)]
    #[get(skip)]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) callback: Option<Rc<RefCell<dyn FnMut()>>>,
    /// Upstream dependencies tracked during the previous execution.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) dependencies: SmallSubList<DependencyId>,
    /// Execution priority depth so parent effects run before nested ones.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) depth: u32,
    /// Active freeze nesting depth (`> 0` suspends execution while tracking dirty notifications).
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) frozen_depth: u32,
    /// Whether upstream signals or memos dirtied this effect while frozen.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) pending_while_frozen: bool,
}

/// Ownership scope entry that disposes all owned reactive resources together.
#[derive(CustomDebug, Data)]
pub(crate) struct ScopeEntry {
    /// Generation counter for this scope slot.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) generation: u32,
    /// Whether this scope is alive.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) alive: bool,
    /// Whether this scope is currently frozen (`PageRetention::Freeze`).
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) frozen: bool,
    /// Signals owned by this scope.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) signals: Vec<SignalId>,
    /// Memos owned by this scope.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) memos: Vec<MemoId>,
    /// Effects owned by this scope.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) effects: Vec<EffectId>,
    /// Child scopes owned by this scope.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) children: Vec<ScopeId>,
    /// Cleanup callbacks invoked when this scope is disposed.
    #[debug(skip)]
    #[get(skip)]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) cleanups: Vec<Box<dyn FnOnce()>>,
}

/// Thread-local arena storing all reactive signals, memos, effects, and scopes.
#[derive(CustomDebug, Data)]
pub struct ReactiveRuntime {
    /// Signal slot arena.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) signals: Vec<SignalEntry>,
    /// Free-list indices for recycled signal slots.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) free_signals: Vec<u32>,
    /// Memo slot arena.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) memos: Vec<MemoEntry>,
    /// Free-list indices for recycled memo slots.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) free_memos: Vec<u32>,
    /// Effect slot arena.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) effects: Vec<EffectEntry>,
    /// Free-list indices for recycled effect slots.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) free_effects: Vec<u32>,
    /// Scope slot arena.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) scopes: Vec<ScopeEntry>,
    /// Free-list indices for recycled scope slots.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) free_scopes: Vec<u32>,
    /// Stack of currently executing observers and their dependency-reuse cursors.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) observer_stack: Vec<Option<(SubscriberId, usize)>>,
    /// Stack of active ownership scopes.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) scope_stack: Vec<ScopeId>,
    /// Pending effects awaiting flush.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) pending_effects: Vec<EffectId>,
    /// Reusable scratch queue storing `(subscriber, target_state)` for zero-allocation dirty propagation.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) dirty_queue: Vec<(SubscriberId, NodeState)>,
    /// Reusable cursor-sliced scratch stack for zero-allocation `NodeState::Check` verification of >4 memo deps.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) check_scratch: Vec<MemoId>,
    /// Reusable scratch buffer holding `(depth, EffectId)` batches across `flush_effects` runs.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) flush_batch_scratch: Vec<(u32, EffectId)>,
    /// Active batch nesting depth; flush executes only when returning to zero.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) batch_depth: u32,
    /// Guard flag preventing recursive flush loops.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) flushing: bool,
    /// Global version clock incremented on every reactive write.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) clock: u64,
    /// Type-keyed root-lifetime global state signal registry.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) global_states: HashMap<TypeId, SignalId>,
    /// String-keyed root-lifetime named global state signal registry.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) named_global_states: HashMap<String, SignalId>,
    /// Type-keyed cross-component custom event channel signals.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) event_channels: HashMap<TypeId, SignalId>,
}

/// Snapshot of runtime diagnostic statistics for telemetry and benchmarking.
#[derive(Clone, Copy, Debug, Default, Eq, Getter, Hash, New, Ord, PartialEq, PartialOrd)]
pub struct ReactiveStats {
    /// Total live signals in the current thread runtime.
    #[get(type(copy))]
    pub(crate) live_signals: usize,
    /// Total live memos in the current thread runtime.
    #[get(type(copy))]
    pub(crate) live_memos: usize,
    /// Total live effects in the current thread runtime.
    #[get(type(copy))]
    pub(crate) live_effects: usize,
    /// Total live ownership scopes in the current thread runtime.
    #[get(type(copy))]
    pub(crate) live_scopes: usize,
    /// Global reactive write clock.
    #[get(type(copy))]
    pub(crate) clock: u64,
}

/// Panic-safe RAII guard that pops an active tracking frame from `observer_stack` on unwind.
#[derive(Debug)]
pub(crate) struct TrackingGuard {
    /// Remains `true` until normal completion disarms the guard before committing.
    pub(crate) active: bool,
}
