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
