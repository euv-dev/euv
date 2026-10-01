mod codegen;

use std::{env::var, fs::write, path::PathBuf};

use chrono::Local;

use codegen::{
    BUILD_STATE_FILE_NAME, ENV_CARGO_PKG_AUTHORS, ENV_CARGO_PKG_DESCRIPTION, ENV_CARGO_PKG_EDITION,
    ENV_CARGO_PKG_LICENSE, ENV_CARGO_PKG_NAME, ENV_CARGO_PKG_REPOSITORY, ENV_CARGO_PKG_VERSION,
    ENV_KEY_EUV_AUTHORS_KEY, ENV_KEY_EUV_BUILD_CLOCK_KEY, ENV_KEY_EUV_BUILD_DATE_KEY,
    ENV_KEY_EUV_BUILD_TIME_KEY, ENV_KEY_EUV_BUILD_TIMESTAMP_KEY, ENV_KEY_EUV_DESCRIPTION_KEY,
    ENV_KEY_EUV_EDITION_KEY, ENV_KEY_EUV_LICENSE_KEY, ENV_KEY_EUV_PACKAGE_NAME_KEY,
    ENV_KEY_EUV_REPOSITORY_KEY, ENV_KEY_EUV_REPOSITORY_NAME_KEY, ENV_KEY_EUV_VERSION_KEY,
    ENV_OUT_DIR, EUV_EDITION_FALLBACK, REPOSITORY_SUFFIX_GIT,
};

/// Entry point of the build script.
///
/// Reads package metadata from the standard `CARGO_PKG_*` environment variables
/// that Cargo exposes to every build script (works regardless of whether the
/// `[package]` fields are inline literals or inherited via
/// `[workspace.package]`). Writes a build-state marker and emits
/// `cargo:rustc-env=` lines so the example runtime can report its version.
///
/// The edition falls back to [`EUV_EDITION_FALLBACK`] because Cargo does not
/// export `CARGO_PKG_EDITION` for workspace-inherited fields.
///
/// # Returns
///
/// - `Result<(), Box<dyn std::error::Error>>` - `Ok(())` after every metadata
///   variable was read and every `cargo:rustc-env=` line was emitted, or the
///   I/O error that aborted the build script.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir: String = var(ENV_OUT_DIR)?;
    let state_file_path: PathBuf = PathBuf::from(&out_dir).join(BUILD_STATE_FILE_NAME);
    let package_name: String = var(ENV_CARGO_PKG_NAME)?;
    let version_value: String = var(ENV_CARGO_PKG_VERSION)?;
    let description_value: String = var(ENV_CARGO_PKG_DESCRIPTION)?;
    let repository_value: String = var(ENV_CARGO_PKG_REPOSITORY)?;
    let authors_value: String = var(ENV_CARGO_PKG_AUTHORS)?;
    let license_value: String = var(ENV_CARGO_PKG_LICENSE)?;
    let edition_value: String =
        var(ENV_CARGO_PKG_EDITION).unwrap_or_else(|_| EUV_EDITION_FALLBACK.to_string());
    let repository_name_value: String = repository_value
        .trim_end_matches('/')
        .trim_end_matches(REPOSITORY_SUFFIX_GIT)
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
