use super::*;

/// Installed toolchain status for a single Rust cross-compilation target.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Getter, New)]
pub struct TargetReadiness {
    /// Target platform label.
    pub(crate) name: String,
    /// Rust compiler target triple.
    pub(crate) triple: String,
    /// Whether `rustup target list --installed` includes `triple`.
    #[get(type(copy))]
    pub(crate) installed: bool,
}

/// Complete environment readiness report produced by `euv doctor`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Getter, New)]
pub struct DoctorReport {
    /// Host `rustc --version` output.
    pub(crate) rustc_version: String,
    /// Host `cargo --version` output.
    pub(crate) cargo_version: String,
    /// Installed `wasm-pack --version` output, or `"missing"`.
    pub(crate) wasm_pack_version: String,
    /// Detected Apple SDK path via `xcrun`, or `"missing"`.
    pub(crate) apple_sdk_path: String,
    /// Detected `ANDROID_NDK_HOME` path, or `"missing"`.
    pub(crate) android_ndk_home: String,
    /// Readiness status across all seven euv platforms.
    pub(crate) targets: Vec<TargetReadiness>,
}

/// Result summary emitted after scaffolding a new euv project via `euv new`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Getter, New)]
pub struct ScaffoldReport {
    /// Absolute or relative root path of the created project.
    pub(crate) project_dir: String,
    /// List of generated files.
    pub(crate) files: Vec<String>,
}
