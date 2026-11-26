use super::*;

/// Classification of live hot-reload events broadcast over SSE (`/__euv_hmr`).
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum HmrEventKind {
    /// Static docs or Web bundle rebuilt; browser clients should hot-reload.
    Reload,
    /// Dynamic WASM OTA binary updated; native and web clients may fetch `/__euv_ota/module.wasm`.
    OtaUpdate,
    /// Incremental rebuild reported a compilation or Markdown error.
    BuildError,
}
