/// 64-bit FxHash mixing multiplier for fast non-cryptographic selector key hashing.
pub(crate) const FX_HASH_MULTIPLIER: u64 = 0x517cc1b727220a95;

/// Initial power-of-two bucket count allocated when a `SelectorTable` inserts its first key.
pub(crate) const SELECTOR_INITIAL_BUCKETS: usize = 16;
