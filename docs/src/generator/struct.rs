use super::*;

/// Result summary emitted after generating a static documentation bundle.
#[derive(Clone, Debug, Deserialize, Eq, Getter, New, PartialEq, Serialize)]
pub struct GeneratedSiteReport {
    /// Total HTML pages written.
    #[get(type(copy))]
    pub(crate) page_count: usize,
    /// UTC RFC-3339 build timestamp.
    pub(crate) generated_at: String,
    /// Path to the generated JSON search index.
    pub(crate) index_path: String,
}

/// Configuration controlling static documentation site generation and optional dev live-reload.
#[derive(Clone, Data, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DocsBuildOptions {
    /// Optional explicit site brand title overriding frontmatter and directory heuristics.
    pub(crate) title: Option<String>,
    /// Optional URL base prefix (for example `""` or `"/docs"`).
    pub(crate) base_url: String,
    /// Optional SSE live-reload endpoint injected in dev mode (for example `Some("/__euv_hmr")`).
    pub(crate) hmr_endpoint: Option<String>,
    /// Optional source root directory whose static assets are copied into the build output.
    pub(crate) assets_root: Option<PathBuf>,
    /// When `true`, writes both `<route>/index.html` and `<route>.html` for universal static hosting.
    #[get(type(copy))]
    pub(crate) clean_urls: bool,
}

/// Shared immutable context passed into page HTML generation to keep helper signatures compact.
#[derive(Clone, Copy, Debug, Getter)]
pub(crate) struct PageRenderContext<'a> {
    /// Parent documentation site containing navigation metadata.
    #[get(pub(crate), type(copy))]
    pub(crate) site: &'a DocSite,
    /// Previous page in canonical sidebar reading order.
    #[get(pub(crate), type(copy))]
    pub(crate) prev_page: Option<&'a DocPage>,
    /// Next page in canonical sidebar reading order.
    #[get(pub(crate), type(copy))]
    pub(crate) next_page: Option<&'a DocPage>,
    /// Build timestamp formatted as RFC-3339.
    #[get(pub(crate), type(copy))]
    pub(crate) generated_at: &'a str,
    /// Script-safe compact JSON search index payload.
    #[get(pub(crate), type(copy))]
    pub(crate) search_json_compact: &'a str,
    /// Active documentation build options.
    #[get(pub(crate), type(copy))]
    pub(crate) options: &'a DocsBuildOptions,
}

/// Mutable state threaded through Markdown chunk rendering for a single page.
pub(crate) struct MarkdownRenderState<'a> {
    /// Current documentation page being rendered.
    pub(crate) page: &'a DocPage,
    /// Normalized base URL prefix without trailing slash.
    pub(crate) base: &'a str,
    /// Per-page heading anchor slug counter for deduplication.
    pub(crate) seen_slugs: BTreeMap<String, usize>,
    /// Whether the leading H1 matching `page.title` has already been skipped.
    pub(crate) skipped_title_h1: bool,
    /// Destination HTML buffer.
    pub(crate) out: &'a mut String,
}
