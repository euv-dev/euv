use super::*;

/// Lightweight non-cryptographic 64-bit FxHash hasher for O(1) selector key lookups.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct SelectorFxHasher {
    /// Accumulated 64-bit hash state.
    pub(crate) hash: u64,
}

/// Open-addressed hash table slot storing a cached hash, selector key, and boolean signal.
#[derive(CustomDebug)]
pub(crate) struct SelectorEntry<K> {
    /// Precomputed 64-bit FxHash digest of `key` for fast mismatch rejection and rehashing.
    pub(crate) hash: u64,
    /// Key represented by this slot.
    #[debug(skip)]
    pub(crate) key: K,
    /// Per-key boolean selection signal.
    pub(crate) signal: Signal<bool>,
}

/// Compact open-addressed linear-probing table mapping selector keys to `Signal<bool>`.
#[derive(CustomDebug)]
pub(crate) struct SelectorTable<K> {
    /// Power-of-two bucket storage (empty until the first key is registered).
    #[debug(skip)]
    pub(crate) entries: Vec<Option<SelectorEntry<K>>>,
    /// Number of occupied slots currently stored in `entries`.
    pub(crate) len: usize,
}

/// Consolidated selector state combining the active key, cached active signal, and keyed table.
#[derive(CustomDebug)]
pub(crate) struct SelectorState<K>
where
    K: Clone + Eq + Hash + 'static,
{
    /// Currently selected key value.
    #[debug(skip)]
    pub(crate) active_key: Option<K>,
    /// Cached `Signal<bool>` for `active_key` to eliminate a table lookup on deselection.
    pub(crate) active_sig: Option<Signal<bool>>,
    /// Per-key boolean selection signal table.
    #[debug(skip)]
    pub(crate) table: SelectorTable<K>,
}

/// O(1) keyed reactive selector notifying only the previously selected and newly selected keys.
#[derive(CustomDebug)]
pub struct Selector<K>
where
    K: Clone + Eq + Hash + 'static,
{
    /// Shared selector state holding the active key, cached active signal, and keyed table.
    #[debug(skip)]
    pub(crate) state: Rc<RefCell<SelectorState<K>>>,
    /// Effect watching the source key producer.
    pub(crate) watcher: Effect,
}
