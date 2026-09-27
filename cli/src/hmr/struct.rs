use super::*;

/// A serializable bag of preserved state entries.
///
/// Each entry is a `(key, value)` string pair. The
/// caller is responsible for serializing non-string
/// values (numbers, booleans, etc.) to strings before
/// insertion.
///
/// # Wire format
///
/// ```json
/// {"entries": {"k1": "v1", "k2": "v2"}}
/// ```
#[derive(Clone, Data, Debug, Default, Eq, PartialEq)]
pub struct HmrState {
    /// The preserved entries, keyed by name.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) entries: HashMap<String, String>,
}
