//! String constants extracted from `build.rs` (rust-standards §1.3c).
//!
//! `build.rs` is the crate root of a build script, so this module is
//! declared from it as `mod r#const;` — the `r#` prefix is required
//! because `const` is a Rust keyword. Every string literal that the
//! §1.3c verifier flags in a function body lives here, verbatim and
//! byte-identical to its original inline form.

// ---------------------------------------------------------------------------
// Cargo-provided build-script environment variables
// ---------------------------------------------------------------------------

/// Environment variable key passed to rustc for the package name.
pub const ENV_KEY_EUV_PACKAGE_NAME_KEY: &str = "EUV_PACKAGE_NAME";
/// Environment variable key passed to rustc for the package version.
pub const ENV_KEY_EUV_VERSION_KEY: &str = "EUV_VERSION";
/// Environment variable key passed to rustc for the package description.
pub const ENV_KEY_EUV_DESCRIPTION_KEY: &str = "EUV_DESCRIPTION";
/// Environment variable key passed to rustc for the package repository URL.
pub const ENV_KEY_EUV_REPOSITORY_KEY: &str = "EUV_REPOSITORY";
/// Environment variable key passed to rustc for the package authors.
pub const ENV_KEY_EUV_AUTHORS_KEY: &str = "EUV_AUTHORS";
/// Environment variable key passed to rustc for the package license.
pub const ENV_KEY_EUV_LICENSE_KEY: &str = "EUV_LICENSE";
/// Environment variable key passed to rustc for the Rust edition.
pub const ENV_KEY_EUV_EDITION_KEY: &str = "EUV_EDITION";
/// Environment variable key passed to rustc for the repository name.
pub const ENV_KEY_EUV_REPOSITORY_NAME_KEY: &str = "EUV_REPOSITORY_NAME";
/// Environment variable key passed to rustc for the build time string.
pub const ENV_KEY_EUV_BUILD_TIME_KEY: &str = "EUV_BUILD_TIME";
/// Environment variable key passed to rustc for the build date string.
pub const ENV_KEY_EUV_BUILD_DATE_KEY: &str = "EUV_BUILD_DATE";
/// Environment variable key passed to rustc for the build clock string.
pub const ENV_KEY_EUV_BUILD_CLOCK_KEY: &str = "EUV_BUILD_CLOCK";
/// Environment variable key passed to rustc for the build timestamp.
pub const ENV_KEY_EUV_BUILD_TIMESTAMP_KEY: &str = "EUV_BUILD_TIMESTAMP";

/// Cargo-provided directory the build script writes its output into.
pub const ENV_OUT_DIR: &str = "OUT_DIR";

/// Cargo-provided package name of the crate being built.
pub const ENV_CARGO_PKG_NAME: &str = "CARGO_PKG_NAME";

/// Cargo-provided package version of the crate being built.
pub const ENV_CARGO_PKG_VERSION: &str = "CARGO_PKG_VERSION";

/// Cargo-provided package description of the crate being built.
pub const ENV_CARGO_PKG_DESCRIPTION: &str = "CARGO_PKG_DESCRIPTION";

/// Cargo-provided package repository URL of the crate being built.
pub const ENV_CARGO_PKG_REPOSITORY: &str = "CARGO_PKG_REPOSITORY";

/// Cargo-provided package authors of the crate being built.
pub const ENV_CARGO_PKG_AUTHORS: &str = "CARGO_PKG_AUTHORS";

/// Cargo-provided package license of the crate being built.
pub const ENV_CARGO_PKG_LICENSE: &str = "CARGO_PKG_LICENSE";

/// Cargo-provided Rust edition of the crate being built.
pub const ENV_CARGO_PKG_EDITION: &str = "CARGO_PKG_EDITION";

// ---------------------------------------------------------------------------
// File names and fallbacks
// ---------------------------------------------------------------------------

/// File name of the build state marker written to `OUT_DIR`.
pub const BUILD_STATE_FILE_NAME: &str = ".euv_build_state";

/// Suffix trimmed off a repository URL to recover the bare repository name.
pub const REPOSITORY_SUFFIX_GIT: &str = ".git";

/// Fallback Rust edition when `CARGO_PKG_EDITION` is absent.
///
/// `edition` is intentionally hard-coded: Cargo does NOT export
/// `CARGO_PKG_EDITION` for workspace-inherited fields (it only exposes the
/// env var when the value is inline in the crate's own `[package]` table).
/// Keep this in sync with `[workspace.package] edition` in the root
/// `Cargo.toml`.
pub const EUV_EDITION_FALLBACK: &str = "2024";
