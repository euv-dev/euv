use super::*;

/// CLI target selector covering all seven euv platforms plus `native` and `all`.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, ValueEnum)]
pub enum TargetArg {
    /// Browser WebAssembly target (`wasm32-unknown-unknown`).
    Web,
    /// Android native NDK target (`aarch64-linux-android`).
    Android,
    /// iOS native UIKit/Metal target (`aarch64-apple-ios`).
    Ios,
    /// iPadOS native UIKit/Metal target (`aarch64-apple-ios`).
    Ipad,
    /// macOS native AppKit/Metal target (`aarch64-apple-darwin` / `x86_64-apple-darwin`).
    Macos,
    /// Windows native Win32/D3D12 target (`x86_64-pc-windows-msvc` / `aarch64-pc-windows-msvc`).
    Windows,
    /// Linux native Wayland/X11/Vulkan target (`x86_64-unknown-linux-gnu` / `aarch64-unknown-linux-gnu`).
    Linux,
    /// Current host native target.
    Native,
    /// Every supported euv platform.
    All,
}

/// CLI render pipeline mode selector.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, ValueEnum)]
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

/// Target mode for the `euv dev` hot-reload and OTA development server.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, ValueEnum)]
pub enum DevTargetArg {
    /// Build and live-reload all project Markdown documentation pages.
    Docs,
    /// Build and live-reload WebAssembly zero-JS bundles plus Dynamic WASM OTA feed.
    Web,
    /// Watch Rust sources, rebuild, hot-restart the native app, and serve Dynamic WASM OTA updates.
    Native,
}

/// Subcommands under `euv docs` for building or live-serving project Markdown documentation.
#[derive(Clone, Debug, Eq, PartialEq, Subcommand)]
pub enum DocsAction {
    /// Builds all Markdown files under a project or docs directory into a static site.
    Build {
        /// Shared source, output, title, and base URL flags.
        #[command(flatten)]
        build: DocsBuildArgs,
    },
    /// Builds and serves Markdown documentation with live file-watching and SSE hot reload.
    Dev {
        /// Shared source, output, title, and base URL flags.
        #[command(flatten)]
        build: DocsBuildArgs,
        /// Shared dev server bind, poll interval, and smoke-run flags.
        #[command(flatten)]
        server: ServerBindArgs,
    },
}

/// Top-level `euv` CLI subcommands.
#[derive(Clone, Debug, Eq, PartialEq, Subcommand)]
pub enum Commands {
    /// Runs an euv application for a target platform and render mode, with optional `--watch` hot reload.
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
        /// Enable file-watching hot reload and live OTA update serving.
        #[arg(short = 'w', long, default_value_t = false)]
        watch: bool,
        /// HTTP hot-reload/OTA port when `--watch` is enabled.
        #[arg(long, default_value_t = 8080)]
        port: u16,
        /// Print the execution plan without spawning Cargo.
        #[arg(long, default_value_t = false)]
        plan: bool,
    },
    /// Starts the hot-reload and Dynamic WASM OTA development server for Web, Native, or Docs.
    Dev {
        /// Development pipeline target (`web`, `native`, or `docs`).
        #[arg(long, value_enum, default_value_t = DevTargetArg::Web)]
        target: DevTargetArg,
        /// Rendering pipeline mode default.
        #[arg(long, value_enum, default_value_t = RenderArg::Auto)]
        render: RenderArg,
        /// Build and serve in release mode when rebuilding Web or Native targets.
        #[arg(long, default_value_t = false)]
        release: bool,
        /// Optional project root directory to watch (defaults to current directory).
        #[arg(long)]
        path: Option<PathBuf>,
        /// Optional output directory for built bundles.
        #[arg(long)]
        out: Option<PathBuf>,
        /// Optional Cargo package name to rebuild on changes.
        #[arg(long)]
        package: Option<String>,
        /// Shared dev server bind, poll interval, and smoke-run flags.
        #[command(flatten)]
        server: ServerBindArgs,
        /// Enable live Dynamic WASM OTA manifest + `.wasm` hot-update endpoint (`--ota=false` disables).
        #[arg(long, default_value_t = true, num_args = 0..=1, default_missing_value = "true")]
        ota: bool,
        /// Print the structured dev server plan without spawning processes.
        #[arg(long, default_value_t = false)]
        plan: bool,
    },
    /// Builds or live-serves a static documentation site from all `.md` files in a project.
    Docs {
        /// Optional `build` or `dev` subcommand (defaults to building all project `.md` files).
        #[command(subcommand)]
        action: Option<DocsAction>,
        /// Shared source, output, title, and base URL flags.
        #[command(flatten)]
        build: DocsBuildArgs,
        /// Enable live watch and SSE hot-reload dev server directly from `euv docs --watch`.
        #[arg(short = 'w', long, default_value_t = false)]
        watch: bool,
        /// Shared dev server bind, poll interval, and smoke-run flags when `--watch` is active.
        #[command(flatten)]
        server: ServerBindArgs,
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
    /// Formats workspace Rust sources (`cargo fmt`) and normalizes indentation inside all `euv` macro blocks (`view!`, `style!`, etc.).
    Fmt {
        /// Verify formatting without modifying files.
        #[arg(long, default_value_t = false)]
        check: bool,
        /// Optional root directory or `.rs` file path (defaults to current directory).
        #[arg(long)]
        path: Option<PathBuf>,
        /// Format only `euv` procedural macro bodies without invoking `cargo fmt`.
        #[arg(long, default_value_t = false)]
        macros_only: bool,
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
