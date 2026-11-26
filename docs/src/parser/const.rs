pub(crate) const FRONTMATTER_DELIMITER: &str = "---";
pub(crate) const MARKDOWN_EXT: &str = "md";
pub(crate) const README_STEM: &str = "README";
pub(crate) const INDEX_STEM: &str = "index";
pub(crate) const DEFAULT_SECTION: &str = "General";
pub(crate) const GUIDE_SECTION: &str = "Guide";
pub(crate) const GENERIC_DOCS_DIR: &str = "docs";
pub(crate) const GENERIC_SRC_DIR: &str = "src";
pub(crate) const DEFAULT_CODE_LANG: &str = "text";
pub(crate) const ROUTE_ROOT: &str = "/";
pub(crate) const FALLBACK_SLUG_PREFIX: &str = "section";
pub(crate) const IGNORED_SCAN_DIRS: [&str; 12] = [
    "target",
    ".git",
    "node_modules",
    "dist",
    "www",
    ".cargo",
    ".idea",
    ".vscode",
    ".next",
    "pkg",
    "out",
    "build",
];
