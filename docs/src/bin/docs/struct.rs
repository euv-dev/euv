use super::*;

/// One parsed CLI invocation.
#[derive(Clone, CustomDebug, Data, Eq, New, PartialEq)]
pub(crate) struct Args {
    /// Source markdown directory (must contain `config.toml` + `*.md`).
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    src_dir: PathBuf,
    /// Output directory; wasm artifacts land in `<out>/<pkg_dir_name>/`.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    out_dir: PathBuf,
    /// Wasm package name forwarded to wasm-pack `--out-name`.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    name: &'static str,
    /// `true` → `euv build --release`, `false` → `euv build --debug`.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    release: bool,
    /// Custom `index.html` template path. When `Some`, the file at this
    /// path replaces the CLI's bundled template; when `None`, the
    /// CLI falls back to `<manifest_dir>/<template.html>`.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    index_html: Option<PathBuf>,
    /// Single-locale build selector (`--locale en`): `Some` compiles only
    /// that locale's bundle; `None` builds every locale (default bundle at
    /// the output root, one sub-directory per additional locale).
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    locale: Option<&'static str>,
}
