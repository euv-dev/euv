use super::*;

/// One parsed CLI invocation.
#[derive(Clone, CustomDebug, Data, Eq, New, PartialEq)]
pub struct Args {
    /// Source markdown directory (must contain `config.toml` + `*.md`).
    src_dir: PathBuf,
    /// Output directory; wasm artifacts land in `<out>/<pkg_dir_name>/`.
    out_dir: PathBuf,
    /// Wasm package name forwarded to wasm-pack `--out-name`.
    name: &'static str,
    /// `true` → `euv build --release`, `false` → `euv build --debug`.
    release: bool,
    /// Custom `index.html` template path. When `Some`, the file at this
    /// path replaces the CLI's bundled template; when `None`, the
    /// CLI falls back to `<manifest_dir>/<template.html>`.
    index_html: Option<PathBuf>,
}
