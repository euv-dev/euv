/// Supported euv procedural macro names formatted inside `{ ... }` blocks by `euv fmt`.
pub(crate) const EUV_FMT_MACRO_NAMES: &[&str] = &[
    "view",
    "style",
    "tokens",
    "watch",
    "computed",
    "on_event",
    "emit",
    "global_state",
    "local_state",
];
/// Directory names skipped during recursive workspace `.rs` macro formatting.
pub(crate) const EUV_FMT_SKIP_DIRS: &[&str] = &[
    "target",
    ".git",
    "node_modules",
    "dist",
    "build",
    ".cargo",
    "vendor",
];
/// File extension matched by the workspace macro formatter.
pub(crate) const EUV_FMT_RS_EXT: &str = "rs";
/// Indentation width in spaces per nesting level inside formatted euv macro blocks.
pub(crate) const EUV_FMT_INDENT_SPACES: usize = 4;
/// Error message returned when `euv fmt --check` detects unformatted macro blocks.
pub(crate) const ERR_FMT_CHECK_FAILED: &str = "unformatted euv macro blocks found in";
