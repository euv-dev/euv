use super::*;

/// euv CLI for real-time WASM compilation and hot-reload development server.
///
/// Defines the command-line interface with three sub-commands
/// (`run` / `build` / `fmt`). All arguments after `--` are transparently
/// forwarded to `wasm-pack build` without interpretation.
#[derive(Clone, Data, Debug, New, Parser)]
#[command(name = "euv")]
#[command(about = "euv development server with live WASM compilation")]
pub struct Cli {
    /// The mode to run in: run (build + server), build (build only), or fmt (format)
    #[command(subcommand)]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) command: Mode,
}

/// euv-specific arguments combined with wasm-pack passthrough.
///
/// Only `--crate-path`, `--port`, `--www-dir`, `--index-html`, `--no-gitignore`, and `--dev`/`--release`/`--profiling`
/// belong to euv; everything in `wasm_pack_args` is forwarded to `wasm-pack build` as-is.
#[derive(Clone, Data, Debug, Parser)]
pub struct ModeArgs {
    /// Path to the Rust crate containing the WASM application
    #[arg(short, long, default_value = ".")]
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) crate_path: PathBuf,
    /// Port for the development server
    #[arg(short, long, default_value_t = 80)]
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) port: u16,
    /// Directory name for static assets and generated HTML (relative to crate-path)
    #[arg(long, default_value = "www")]
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) www_dir: String,
    /// Path to a custom index.html template file.
    /// When specified, uses this file instead of the built-in template.
    /// The placeholders `__IMPORT_PATH__` and `__RELOAD_ROUTE__` will be replaced.
    #[arg(long)]
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) index_html: Option<PathBuf>,
    /// Create a development build. Enable debug info, and disable optimizations
    #[arg(long)]
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) dev: bool,
    /// Create a release build. Enable optimizations and disable debug info
    #[arg(long)]
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) release: bool,
    /// Create a profiling build. Enable optimizations and debug info
    #[arg(long)]
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) profiling: bool,
    /// Remove the .gitignore file from the output directory after building
    #[arg(long)]
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) no_gitignore: bool,
    /// Arguments transparently forwarded to `wasm-pack build`
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) wasm_pack_args: Vec<String>,
}

/// Arguments for the `fmt` subcommand.
///
/// Formats euv macro invocations (`html!`, `class!`, `vars!`, `watch!`)
/// in Rust source files.
#[derive(Clone, Data, Debug, New, Parser)]
pub struct FmtArgs {
    /// Path to the directory or file to format
    #[arg(short, long, default_value = ".")]
    pub(crate) path: PathBuf,
    /// Check if formatting is needed without modifying files
    #[arg(long, default_value_t = false)]
    #[get(type(copy))]
    pub(crate) check: bool,
}

/// Configuration for server URL display.
///
/// Groups all parameters needed by `print_server_urls` into a single struct.
#[derive(Clone, Data, Debug, New)]
pub(crate) struct ServerUrlConfig {
    /// The port number the server is listening on.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) port: u16,
    /// The serving route prefix (e.g. "www" or "wwws").
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) route_prefix: String,
    /// The index HTML file name (e.g. "index.html").
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) index_html_file_name: String,
}

/// A namespace import clause found in the bridge: `import * as alias from
/// "spec"`.
///
/// Collected while scanning the bridge's imports so the inlined IIFE can
/// rebuild the alias as a plain object once the module graph is gone.
pub(crate) struct NamespaceImport {
    /// The local binding the clause introduces.
    pub(crate) alias: String,
    /// The module specifier, verbatim with its quotes.
    pub(crate) spec: String,
}
