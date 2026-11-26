/// Default window title for the interactive documentation viewer.
pub const DOCS_WINDOW_TITLE: &str = "euv Documentation Viewer";
/// Default logical window width for the docs viewer.
pub const DOCS_WINDOW_WIDTH: u32 = 1200;
/// Default logical window height for the docs viewer.
pub const DOCS_WINDOW_HEIGHT: u32 = 800;
/// CLI flag enabling headless verification mode.
pub const DOCS_FLAG_HEADLESS: &str = "--headless";
/// CLI flag triggering static HTML/JSON site generation.
pub const DOCS_FLAG_GENERATE: &str = "--generate";
/// CLI flag keeping the native documentation viewer open in an interactive event loop.
pub const DOCS_FLAG_INTERACTIVE: &str = "--interactive";

pub(crate) const SEARCH_PLACEHOLDER: &str = "Search documentation...";
pub(crate) const TOC_CARD_TITLE: &str = "On This Page";
pub(crate) const EMPTY_PAGE_TITLE: &str = "Welcome to euv";
pub(crate) const EMPTY_PAGE_MSG: &str = "Select a page from the sidebar to begin reading.";
