use super::*;

/// Root CLI parser for the `euv` cross-platform native toolchain.
#[derive(Clone, Debug, PartialEq, Eq, Parser, Getter)]
#[command(
    name = "euv",
    version,
    about = "Cross-platform native Rust UI CLI for euv"
)]
pub struct Cli {
    /// Selected subcommand to execute.
    #[command(subcommand)]
    pub(crate) command: Commands,
}
