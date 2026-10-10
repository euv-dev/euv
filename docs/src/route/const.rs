/// The route every request collapses onto when it carries no path at all
/// (an empty hash, or a hash holding only separators).
pub(crate) const ROUTE_ROOT: &str = "/";

/// The path separator used by every generated route.
pub(crate) const ROUTE_SEPARATOR: char = '/';

/// The doubled separator a link assembled from a prefix and a path that
/// already begins with one produces.
pub(crate) const ROUTE_SEPARATOR_PAIR: &str = "//";

/// The suffix `build.rs` appends to the route of a leaf markdown page
/// (`README.md` / `index.md` become a directory route with a trailing
/// separator instead).
pub(crate) const ROUTE_HTML_SUFFIX: &str = ".html";

/// The number of spellings [`route_candidates`] produces: the request as
/// written plus at most two rewrites.
pub(crate) const ROUTE_CANDIDATE_CAPACITY: usize = 3;

/// The number of spellings [`route_variants`] produces per served route.
pub(crate) const ROUTE_VARIANT_CAPACITY: usize = 7;

/// The `.html` suffix followed by a separator — the shape a link helper
/// produces when it appends a slash to a leaf route it has already
/// spelled with its suffix.
pub(crate) const HTML_WITH_SEPARATOR: &str = ".html/";

/// The number of spellings [`route_candidates`] considers: the bare
/// path, the directory form, the file form, and the route as it
/// arrived.
pub(crate) const ROUTE_CANDIDATE_COUNT: usize = 4;
