use super::*;

/// Cross-platform key-value local storage abstraction synchronized across native and web hosts.
#[derive(Clone, Data, Debug, Default)]
pub struct StorageStore {
    /// Stored UTF-8 key-value pairs.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) entries: HashMap<String, String>,
}
