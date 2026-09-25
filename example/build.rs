use std::{env::var, fs::write, path::PathBuf};

use chrono::Local;

/// Environment variable key passed to rustc for the package name.
const ENV_KEY_EUV_PACKAGE_NAME_KEY: &str = "EUV_PACKAGE_NAME";
/// Environment variable key passed to rustc for the package version.
const ENV_KEY_EUV_VERSION_KEY: &str = "EUV_VERSION";
/// Environment variable key passed to rustc for the package description.
const ENV_KEY_EUV_DESCRIPTION_KEY: &str = "EUV_DESCRIPTION";
/// Environment variable key passed to rustc for the package repository URL.
const ENV_KEY_EUV_REPOSITORY_KEY: &str = "EUV_REPOSITORY";
/// Environment variable key passed to rustc for the package authors.
const ENV_KEY_EUV_AUTHORS_KEY: &str = "EUV_AUTHORS";
/// Environment variable key passed to rustc for the package license.
const ENV_KEY_EUV_LICENSE_KEY: &str = "EUV_LICENSE";
/// Environment variable key passed to rustc for the Rust edition.
const ENV_KEY_EUV_EDITION_KEY: &str = "EUV_EDITION";
/// Environment variable key passed to rustc for the repository name.
const ENV_KEY_EUV_REPOSITORY_NAME_KEY: &str = "EUV_REPOSITORY_NAME";
/// Environment variable key passed to rustc for the build time string.
const ENV_KEY_EUV_BUILD_TIME_KEY: &str = "EUV_BUILD_TIME";
/// Environment variable key passed to rustc for the build date string.
const ENV_KEY_EUV_BUILD_DATE_KEY: &str = "EUV_BUILD_DATE";
/// Environment variable key passed to rustc for the build clock string.
const ENV_KEY_EUV_BUILD_CLOCK_KEY: &str = "EUV_BUILD_CLOCK";
/// Environment variable key passed to rustc for the build timestamp.
const ENV_KEY_EUV_BUILD_TIMESTAMP_KEY: &str = "EUV_BUILD_TIMESTAMP";
/// File name of the build state marker written to `OUT_DIR`.
const BUILD_STATE_FILE_NAME: &str = ".euv_build_state";

/// Entry point of the build script.
///
/// Reads package metadata from the standard `CARGO_PKG_*` environment variables
/// that Cargo exposes to every build script (works regardless of whether the
/// `[package]` fields are inline literals or inherited via
/// `[workspace.package]`). Writes a build-state marker and emits
/// `cargo:rustc-env=` lines so the example runtime can report its version.
///
/// `edition` is intentionally hard-coded: Cargo does NOT export
/// `CARGO_PKG_EDITION` for workspace-inherited fields (it only exposes the
/// env var when the value is inline in the crate's own `[package]` table).
/// Keep this in sync with `[workspace.package] edition` in the root
/// `Cargo.toml`.
const EUV_EDITION_FALLBACK: &str = "2024";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir: String = var("OUT_DIR")?;
    let state_file_path: PathBuf = PathBuf::from(&out_dir).join(BUILD_STATE_FILE_NAME);
    let package_name: String = var("CARGO_PKG_NAME")?;
    let version_value: String = var("CARGO_PKG_VERSION")?;
    let description_value: String = var("CARGO_PKG_DESCRIPTION")?;
    let repository_value: String = var("CARGO_PKG_REPOSITORY")?;
    let authors_value: String = var("CARGO_PKG_AUTHORS")?;
    let license_value: String = var("CARGO_PKG_LICENSE")?;
    let edition_value: String =
        var("CARGO_PKG_EDITION").unwrap_or_else(|_| EUV_EDITION_FALLBACK.to_string());
    let repository_name_value: String = repository_value
        .trim_end_matches('/')
        .trim_end_matches(".git")
        .split('/')
        .rev()
        .take(2)
        .collect::<Vec<&str>>()
        .into_iter()
        .rev()
        .collect::<Vec<&str>>()
        .join("/");
    let build_time_formatted: String = format!("{}", Local::now().format("%Y-%m-%d %H:%M:%S%.6f"));
    let build_date: String = format!("{}", Local::now().format("%Y-%m-%d"));
    let build_clock: String = format!("{}", Local::now().format("%H:%M:%S"));
    let build_timestamp: String = format!("{}", Local::now().timestamp_micros());
    write(&state_file_path, &build_time_formatted)?;
    println!("cargo:rustc-env={ENV_KEY_EUV_PACKAGE_NAME_KEY}={package_name}");
    println!("cargo:rustc-env={ENV_KEY_EUV_VERSION_KEY}={version_value}");
    println!("cargo:rustc-env={ENV_KEY_EUV_DESCRIPTION_KEY}={description_value}");
    println!("cargo:rustc-env={ENV_KEY_EUV_REPOSITORY_KEY}={repository_value}");
    println!("cargo:rustc-env={ENV_KEY_EUV_AUTHORS_KEY}={authors_value}");
    println!("cargo:rustc-env={ENV_KEY_EUV_LICENSE_KEY}={license_value}");
    println!("cargo:rustc-env={ENV_KEY_EUV_EDITION_KEY}={edition_value}");
    println!("cargo:rustc-env={ENV_KEY_EUV_REPOSITORY_NAME_KEY}={repository_name_value}");
    println!("cargo:rustc-env={ENV_KEY_EUV_BUILD_TIME_KEY}={build_time_formatted}");
    println!("cargo:rustc-env={ENV_KEY_EUV_BUILD_DATE_KEY}={build_date}");
    println!("cargo:rustc-env={ENV_KEY_EUV_BUILD_CLOCK_KEY}={build_clock}");
    println!("cargo:rustc-env={ENV_KEY_EUV_BUILD_TIMESTAMP_KEY}={build_timestamp}");
    Ok(())
}
