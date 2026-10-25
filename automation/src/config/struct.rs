use super::*;

/// Native runner configuration; browser binaries are explicit and overridable.
#[derive(Clone, Data, Debug, New)]
#[new(pub(crate))]
pub struct Settings {
    /// Directory containing generated fixture and diagnostic artifacts.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) output: PathBuf,
    /// ChromeDriver executable used for the owned browser session.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) driver: PathBuf,
    /// Matching wasm-bindgen executable used to generate the browser bridge.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) bindgen: PathBuf,
    /// Cargo executable used to compile the actual WASM fixture.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) cargo: PathBuf,
    /// Optional browser executable override.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    pub(crate) chrome: Option<PathBuf>,
    /// Optional URL of an externally served compatible fixture.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    pub(crate) url: Option<String>,
    /// Reuses already-built fixture files only when explicitly requested.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    pub(crate) skip_build: bool,
    /// Injects a deliberately wrong assertion to exercise failure diagnostics.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    pub(crate) probe_failure: bool,
}
