use super::*;

/// Root CLI parser for the `euv` cross-platform native toolchain.
#[derive(Clone, Debug, Eq, Getter, Parser, PartialEq)]
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

/// Shared documentation source, output, title, and base URL arguments.
#[derive(Args, Clone, Debug, Eq, Getter, PartialEq)]
pub struct DocsBuildArgs {
    /// Optional source project or Markdown directory (defaults to current working directory).
    pub(crate) src: Option<PathBuf>,
    /// Output directory for generated HTML, search index, sitemap, and assets.
    #[arg(long, default_value = DEFAULT_DOCS_OUT_DIR)]
    pub(crate) out: PathBuf,
    /// Optional site brand title overriding frontmatter and folder name.
    #[arg(long)]
    pub(crate) title: Option<String>,
    /// Optional URL base prefix (for example `/docs`).
    #[arg(long, default_value = "")]
    pub(crate) base: String,
}

/// Shared local HTTP dev server bind, watcher poll interval, and smoke-run arguments.
#[derive(Args, Clone, Debug, Eq, Getter, PartialEq)]
pub struct ServerBindArgs {
    /// Host bind interface.
    #[arg(long, default_value = DEFAULT_DEV_HOST)]
    pub(crate) host: String,
    /// HTTP dev and SSE HMR server port (`0` selects an ephemeral port).
    #[arg(long, default_value_t = 8080)]
    #[get(type(copy))]
    pub(crate) port: u16,
    /// File watcher polling interval in milliseconds.
    #[arg(long, default_value_t = 250)]
    #[get(type(copy))]
    pub(crate) poll_ms: u64,
    /// Run initial build and server readiness check then exit immediately.
    #[arg(long, default_value_t = false)]
    #[get(type(copy))]
    pub(crate) once: bool,
}
