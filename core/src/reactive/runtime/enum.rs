use super::*;

/// Subscriber handle stored on reactive signal and memo dependency lists.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SubscriberId {
    /// A memoized derived computation subscriber.
    Memo(MemoId),
    /// A side-effect subscriber.
    Effect(EffectId),
}

/// Source dependency handle recorded by an active observer.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DependencyId {
    /// Dependency on a source signal.
    Signal(SignalId),
    /// Dependency on a memoized computation.
    Memo(MemoId),
}

/// Propagation state for glitch-free push-pull memo evaluation.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum NodeState {
    /// Cached value is up to date.
    #[default]
    Clean,
    /// Upstream dependency may have changed and requires verification.
    Check,
    /// Guaranteed dirty and must recompute before reading.
    Dirty,
}

/// Inline small-list storage eliminating heap allocations for 0..=2 reactive subscribers or dependencies.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) enum SmallSubList<T> {
    /// Zero items stored inline without heap allocation.
    #[default]
    Empty,
    /// Single item stored inline without heap allocation.
    One(T),
    /// Two distinct items stored inline in insertion order without heap allocation.
    Two(T, T),
    /// Heap-backed fallback used when three or more items are present.
    Many(Vec<T>),
}

/// Zero-allocation snapshot of upstream `MemoId` dependencies during `NodeState::Check` verification.
#[derive(Clone, Copy, Debug)]
pub(crate) enum MemoCheckDeps {
    /// Up to four upstream memo dependencies stored inline on the stack without heap allocation.
    Inline([MemoId; 4], u8),
    /// Cursor-sliced `(start, end)` range in `ReactiveRuntime::check_scratch` for five or more memo dependencies.
    Spill(usize, usize),
}

/// Fused single-borrow preparation outcome for reading a typed `Memo<T>`.
pub(crate) enum MemoReadPrep<T> {
    /// Target memo slot is disposed or invalid.
    Dead,
    /// Memo was already `NodeState::Clean`; dependency tracked and value cloned in one borrow.
    Ready(Option<T>),
    /// Memo is `NodeState::Dirty`; compute closure and prior value staged with tracking frame pushed.
    Compute(StagedMemo),
    /// Memo is `NodeState::Check`; upstream memo dependencies extracted for pull verification.
    Check(MemoCheckDeps),
}

/// Fused single-borrow preparation outcome for `evaluate_memo`.
pub(crate) enum MemoEvalPrep {
    /// Memo is already `NodeState::Clean` or disposed and requires no work.
    Clean,
    /// Memo is `NodeState::Dirty`; compute closure and prior value staged with tracking frame pushed.
    Compute(StagedMemo),
    /// Memo is `NodeState::Check`; upstream memo dependencies extracted for pull verification.
    Check(MemoCheckDeps),
}
