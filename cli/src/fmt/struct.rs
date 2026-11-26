use super::*;

/// Structured summary produced by `euv fmt` across workspace Rust sources and macro invocations.
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
    /// Whether `--check` dry-run verification mode was active.
    #[get(type(copy))]
    pub(crate) check_only: bool,
    /// Relative or display paths of files needing or receiving macro formatting.
    pub(crate) changed_files: Vec<String>,
}
