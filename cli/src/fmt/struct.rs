use super::*;

/// Structured summary produced by `euv fmt` across workspace text files, Rust sources, and macro invocations.
#[derive(Clone, Data, Debug, Default, Deserialize, Eq, New, PartialEq, Serialize)]
pub struct FmtReport {
    /// Total `.rs` files scanned across the target workspace or directory.
    #[get(type(copy))]
    pub(crate) files_scanned: usize,
    /// Number of `.rs` files whose macro blocks required reformatting.
    #[get(type(copy))]
    pub(crate) files_formatted: usize,
    /// Total `euv` macro blocks (`view!`, `style!`, etc.) normalized.
    #[get(type(copy))]
    pub(crate) macros_formatted: usize,
    /// Total non-Rust text files (`.md`, `.yml`, `.yaml`, `.json`, `.toml`, etc.) scanned via `crate-cli`.
    #[get(type(copy))]
    pub(crate) text_files_scanned: usize,
    /// Number of non-Rust text files normalized via `crate-cli`.
    #[get(type(copy))]
    pub(crate) text_files_formatted: usize,
    /// Whether `--check` dry-run verification mode was active.
    #[get(type(copy))]
    pub(crate) check_only: bool,
    /// Relative or display paths of files needing or receiving formatting.
    pub(crate) changed_files: Vec<String>,
}

/// Per-worker formatting counters and changed paths returned by parallel macro formatting chunks.
#[derive(Debug, Default, New)]
pub(crate) struct FmtChunkSummary {
    /// Number of `.rs` files in this chunk whose macro blocks required reformatting.
    pub(crate) files_formatted: usize,
    /// Total `euv` macro blocks normalized in this chunk.
    pub(crate) macros_formatted: usize,
    /// Display paths of changed files in deterministic sorted chunk order.
    pub(crate) changed_files: Vec<String>,
}
