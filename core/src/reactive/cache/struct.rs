use super::*;

/// A fixed-capacity LRU cache.
///
/// The cache holds at most `capacity` entries. When a
/// `put` would exceed the capacity, the
/// least-recently-used entry is evicted. `get` updates
/// the recency so the just-read entry becomes the most-
/// recently-used.
///
/// `peek` and `contains` are O(1). `put`, `get`, and
/// `remove` are O(n) in the number of cached entries
/// because promoting or dropping a key scans the
/// recency deque (`VecDeque::retain`); `iter` is O(n).
///
/// # Capacity edge cases
///
/// - `capacity = 0` - the cache accepts no entries. Both
///   `put` and `get` behave as no-ops (well, `get` still
///   evicts because there's nothing to evict; `put`
///   silently drops the entry).
/// - `capacity = 1` - the cache holds exactly one entry.
///   Every `put` evicts the previous entry.
///
/// # Lombok `New` derivation
///
/// `#[derive(New)]` generates `LruCache::new(capacity)` —
/// the `map` and `order` fields are skipped with
/// `#[new(skip)]` so Lombok falls back to
/// `<HashMap as Default>::default()` and
/// `<VecDeque as Default>::default()` (which both call
/// `new()` internally), preserving the canonical
/// single-argument call site.
#[derive(Clone, Data, Debug, New)]
pub struct LruCache<K, V>
where
    K: Clone + Eq + Hash,
{
    /// The maximum number of entries before eviction
    /// kicks in.
    #[get(pub(crate))]
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) capacity: usize,
    /// The current entries, keyed by K. Default-initialised
    /// via `#[new(skip)]` (`HashMap::new()`).
    #[new(skip)]
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) map: HashMap<K, V>,
    /// The MRU-first order. Front = most recently used,
    /// back = least recently used. Default-initialised
    /// via `#[new(skip)]` (`VecDeque::new()`).
    #[new(skip)]
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) order: VecDeque<K>,
}
