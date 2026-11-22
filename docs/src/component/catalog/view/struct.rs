use super::*;

/// One stored project in the public catalog.
///
/// The catalog is served as JSON by the artifact API and parsed here, so
/// the documentation home page renders it in Rust and needs no script.
///
/// The fields are crate-visible and the accessors are public, so a
/// consumer reads the artifact through `get_name()` and cannot freeze
/// the field layout into the published API.
#[derive(Clone, Data, Debug, Default, New, PartialEq)]
pub struct DocsCatalogArtifact {
    /// Project name; also the path segment of its published page.
    #[get(type(clone))]
    pub(crate) name: String,
    /// Number of files in the published artifact.
    #[get(type(copy))]
    pub(crate) file_count: u64,
    /// Total size of the published artifact in bytes.
    #[get(type(copy))]
    pub(crate) total_bytes: u64,
    /// Hex-encoded SHA-256 of the published artifact.
    #[get(type(clone))]
    pub(crate) sha256: String,
    /// Publication time as seconds since the Unix epoch.
    #[get(type(copy))]
    pub(crate) updated_at: i64,
}

/// The parsed public catalog.
///
/// The field is crate-visible and the accessor is public, so a consumer
/// reads the list through `get_artifacts()`.
#[derive(Clone, Data, Debug, Default, New, PartialEq)]
pub struct DocsCatalog {
    /// The stored projects, in the order the API listed them.
    #[get(type(clone))]
    pub(crate) artifacts: Vec<DocsCatalogArtifact>,
}

/// The props of the catalog component.
#[derive(Clone, Debug, Default)]
pub(crate) struct DocsCatalogProps {
    /// The route the catalog is rendered on, whose locale selects the
    /// labels the cards carry.
    pub(crate) route_signal: Signal<String>,
}
