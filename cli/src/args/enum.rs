use super::*;

/// CLI target selector covering all seven euv platforms plus `native` and `all`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum, Serialize, Deserialize)]
pub enum TargetArg {
    /// Browser WebAssembly target (`wasm32-unknown-unknown`).
    Web,
    /// Android native NDK target (`aarch64-linux-android`).
    Android,
    /// iOS native UIKit/Metal target (`aarch64-apple-ios`).
    Ios,
    /// iPadOS native UIKit/Metal target (`aarch64-apple-ios`).
    Ipad,
    /// macOS native AppKit/Metal target (`aarch64-apple-darwin`).
    Macos,
    /// Windows native Win32/D3D12 target (`x86_64-pc-windows-msvc`).
    Windows,
    /// Linux native Wayland/X11/Vulkan target (`x86_64-unknown-linux-gnu`).
    Linux,
    /// Current host native target.
    Native,
    /// Every supported euv platform.
    All,
}

/// CLI render pipeline mode selector.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum, Serialize, Deserialize)]
pub enum RenderArg {
    /// Automatically prefer hardware GPU with software fallback.
    Auto,
    /// Deterministic software CPU rasterizer.
    Software,
    /// Native GPU instanced pipeline.
    Hardware,
    /// Hybrid CPU vector + GPU texture compositor.
    Hybrid,
}

/// Top-level `euv` CLI subcommands.
#[derive(Clone, Debug, PartialEq, Eq, Subcommand)]
pub enum Commands {
    /// Runs an euv application for a target platform and render mode.
    Run {
        /// Target platform to launch.
        #[arg(long, value_enum, default_value_t = TargetArg::Native)]
        target: TargetArg,
        /// Rendering pipeline mode.
        #[arg(long, value_enum, default_value_t = RenderArg::Auto)]
        render: RenderArg,
        /// Build and run in release mode.
        #[arg(long, default_value_t = false)]
        release: bool,
        /// Optional Cargo package name to run.
        #[arg(long)]
        package: Option<String>,
        /// Print the execution plan without spawning Cargo.
        #[arg(long, default_value_t = false)]
        plan: bool,
    },
    /// Builds native or Web bundles for one or all seven targets.
    Build {
        /// Target platform (or `all`).
        #[arg(long, value_enum, default_value_t = TargetArg::Native)]
        target: TargetArg,
        /// Rendering pipeline mode default.
        #[arg(long, value_enum, default_value_t = RenderArg::Auto)]
        render: RenderArg,
        /// Enable release optimization profile.
        #[arg(long, default_value_t = false)]
        release: bool,
        /// Print structured JSON build plans without executing.
        #[arg(long, default_value_t = false)]
        plan: bool,
    },
    /// Checks Rust compilation across one or all seven cross-platform targets.
    Check {
        /// Check all seven platform targets in sequence.
        #[arg(long, default_value_t = false)]
        all_targets: bool,
        /// Single target to check when `--all-targets` is omitted.
        #[arg(long, value_enum, default_value_t = TargetArg::Native)]
        target: TargetArg,
        /// Optional Cargo package name to check.
        #[arg(long)]
        package: Option<String>,
        /// Print the planned commands without running Cargo.
        #[arg(long, default_value_t = false)]
        plan: bool,
    },
    /// Formats workspace Rust sources using `cargo fmt`.
    Fmt {
        /// Verify formatting without modifying files.
        #[arg(long, default_value_t = false)]
        check: bool,
    },
    /// Inspects installed Rust targets, SDKs, and cross-platform tooling readiness.
    Doctor {
        /// Emit machine-readable JSON output.
        #[arg(long, default_value_t = false)]
        json: bool,
    },
    /// Scaffolds a new cross-platform euv native project.
    New {
        /// Name of the new project directory and crate.
        name: String,
        /// Optional parent directory (defaults to current working directory).
        #[arg(long)]
        path: Option<PathBuf>,
    },
}
