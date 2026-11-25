use super::*;

/// Result summary emitted after generating a static documentation bundle.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Getter, New)]
pub struct GeneratedSiteReport {
    /// Total HTML pages written.
    #[get(type(copy))]
    pub(crate) page_count: usize,
    /// UTC RFC-3339 build timestamp.
    pub(crate) generated_at: String,
    /// Path to the generated JSON search index.
    pub(crate) index_path: String,
}
