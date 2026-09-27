use super::*;

/// One parsed CLI invocation.
#[derive(Clone, CustomDebug, Data, Eq, New, PartialEq)]
pub struct Args {
    /// Source markdown directory (must contain `config.toml` + `*.md`).
    #[get]
    #[set]
    src_dir: PathBuf,
    /// Output directory; wasm artifacts land in `<out>/<pkg_dir_name>/`.
    #[get]
    #[set]
    out_dir: PathBuf,
    /// Wasm package name forwarded to wasm-pack `--out-name`.
    #[get]
    #[set]
    name: &'static str,
    /// `true` → `euv build --release`, `false` → `euv build --debug`.
    #[get]
    #[set]
    release: bool,
    /// Custom `index.html` template path. When `Some`, the file at this
    /// path replaces the CLI's bundled template; when `None`, the
    /// CLI falls back to `<manifest_dir>/<template.html>`.
    #[get]
    #[set]
    index_html: Option<PathBuf>,
}
