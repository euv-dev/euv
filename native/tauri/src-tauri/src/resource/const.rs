/// Maximum number of resource downloads running at the same time.
pub(crate) const DOWNLOAD_CONCURRENCY: usize = 6;

/// JavaScript resource suffix whose source is needed for dependency discovery.
pub(crate) const JS_EXTENSION: &str = ".js";

/// JavaScript module resource suffix whose source is needed for discovery.
pub(crate) const MJS_EXTENSION: &str = ".mjs";

/// Error returned when a required remote resource has no bytes.
pub(crate) const EMPTY_RESOURCE_ERROR: &str = "empty resource";

/// Extension prefix used for files receiving an incomplete response.
pub(crate) const DOWNLOAD_TEMP_SUFFIX: &str = "download";
