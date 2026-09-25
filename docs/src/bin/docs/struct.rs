use super::*;
use lombok_macros::{CustomDebug, Data, New};

/// One parsed CLI invocation.
#[derive(Clone, PartialEq, Eq, Data, New, CustomDebug)]
pub struct Args {
    /// Source markdown directory (must contain `config.toml` + `*.md`).
    #[get(pub)]
    #[set(pub)]
    src_dir: PathBuf,
    /// Output directory; wasm artifacts land in `<out>/<pkg_dir_name>/`.
    #[get(pub)]
    #[set(pub)]
    out_dir: PathBuf,
    /// Wasm package name forwarded to wasm-pack `--out-name`.
    #[get(pub)]
    #[set(pub)]
    name: &'static str,
    /// `true` → `euv build --release`, `false` → `euv build --debug`.
    #[get(pub)]
    #[set(pub)]
    release: bool,
    /// Custom `index.html` template path. When `Some`, the file at this
    /// path replaces the CLI's bundled template; when `None`, the
    /// CLI falls back to `<manifest_dir>/<template.html>`.
    #[get(pub)]
    #[set(pub)]
    index_html: Option<PathBuf>,
}
