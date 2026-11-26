use super::*;

/// Target platform metadata describing Rust triple, system APIs, and output bundle.
#[derive(Clone, Debug, Deserialize, Eq, Getter, New, PartialEq, Serialize)]
pub struct TargetSpec {
    /// Human-readable target key (`web`, `android`, `ios`, `ipad`, `macos`, `windows`, `linux`).
    pub(crate) name: String,
    /// Rust compiler target triple.
    pub(crate) rust_triple: String,
    /// Direct OS/browser system API stack used by `euv-core`.
    pub(crate) system_api: String,
    /// Primary packaged output artifact path.
    pub(crate) artifact: String,
    /// Core `PlatformKind` discriminant.
    pub(crate) platform_kind: String,
    /// Preferred hardware GPU backend reported by `euv-core`.
    pub(crate) preferred_gpu: String,
}

/// Single executable command step inside a cross-platform `BuildPlan`.
#[derive(Clone, Debug, Deserialize, Eq, Getter, New, PartialEq, Serialize)]
pub struct BuildStep {
    /// Human-readable purpose of this step.
    pub(crate) description: String,
    /// Executable program name.
    pub(crate) program: String,
    /// Command-line arguments passed to `program`.
    pub(crate) args: Vec<String>,
}

/// Complete declarative build plan for one target platform.
#[derive(Clone, Debug, Deserialize, Eq, Getter, New, PartialEq, Serialize)]
pub struct BuildPlan {
    /// Target specification metadata.
    pub(crate) target: TargetSpec,
    /// Selected default render mode (`Auto`, `Software`, `Hardware`, `Hybrid`).
    pub(crate) render_mode: String,
    /// Whether release optimization profile is enabled.
    #[get(type(copy))]
    pub(crate) release: bool,
    /// Ordered build and packaging steps.
    pub(crate) steps: Vec<BuildStep>,
}
