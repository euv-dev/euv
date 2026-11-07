use std::{
    env,
    ffi::OsStr,
    fs, io,
    path::{Path, PathBuf},
    process::{Command, Output},
};

use cc::{Build, Tool};

/// The macOS target identifier shared by Apple compiler configuration.
const MACOS_TARGET: &str = "macos";

/// Cargo output directory containing generated native build inputs.
const CARGO_OUTPUT_DIRECTORY: &str = "OUT_DIR";

/// Enables automatic reference counting for native Objective-C adapters.
const OBJC_ARC_FLAG: &str = "-fobjc-arc";

/// Keeps Cargo's cache-directory watch valid without bundling a resource.
const BUNDLED_CACHE_PLACEHOLDER: &str = ".empty";

/// Collects bundled resource paths relative to their cache directory.
///
/// # Arguments
/// - `&Path` - Directory to traverse.
/// - `&Path` - Root used to calculate relative resource paths.
/// - `&mut Vec<PathBuf>` - Destination for discovered paths.
///
/// # Returns
/// - `()` - Completes after visiting the directory tree.
fn walk_dir(directory: &Path, base: &Path, results: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(directory) {
        for entry in entries.flatten() {
            let path: PathBuf = entry.path();
            if path.is_dir() {
                walk_dir(&path, base, results);
            } else if let Ok(relative) = path.strip_prefix(base) {
                results.push(relative.to_path_buf());
            }
        }
    }
}

/// Writes generated source only when its bytes have changed.
///
/// # Arguments
/// - `&Path` - Destination generated source file.
/// - `&str` - Complete generated source contents.
///
/// # Returns
/// - `io::Result<()>` - Result of preserving or writing the generated source.
fn write_if_changed(destination: &Path, contents: &str) -> io::Result<()> {
    if fs::read(destination).is_ok_and(|existing: Vec<u8>| existing == contents.as_bytes()) {
        return Ok(());
    }
    fs::write(destination, contents)
}

/// Links Clang's availability checks when Rust bypasses compiler-driver defaults.
///
/// # Arguments
/// - `&Build` - The configured native compiler and target.
/// - `&str` - The Apple target operating system.
///
/// # Returns
/// - `io::Result<()>` - Success after locating the selected SDK runtime archive.
fn link_apple_runtime(build: &Build, target_os: &str) -> io::Result<()> {
    let target: String = env::var("TARGET").map_err(io::Error::other)?;
    let runtime: &str = if target_os == MACOS_TARGET {
        "clang_rt.osx"
    } else if target.contains("sim") || target.starts_with("x86_64") {
        "clang_rt.iossim"
    } else {
        "clang_rt.ios"
    };
    let compiler: Tool = build.try_get_compiler().map_err(io::Error::other)?;
    let output: Output = Command::new(compiler.path())
        .arg(format!("--print-file-name=lib{runtime}.a"))
        .output()?;
    if !output.status.success() {
        return Err(io::Error::other(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }
    let file: PathBuf = PathBuf::from(
        String::from_utf8(output.stdout)
            .map_err(io::Error::other)?
            .trim(),
    );
    if !file.is_file() {
        return Err(io::Error::other(format!(
            "Apple compiler runtime is missing: {}",
            file.display()
        )));
    }
    let architecture: String = env::var("CARGO_CFG_TARGET_ARCH").map_err(io::Error::other)?;
    let slice: &str = if architecture == "aarch64" {
        "arm64"
    } else {
        &architecture
    };
    let directory: PathBuf =
        PathBuf::from(env::var(CARGO_OUTPUT_DIRECTORY).map_err(io::Error::other)?);
    let archive: PathBuf = directory.join(format!("lib{runtime}.a"));
    if fs::read(&file)?.starts_with(b"!<arch>\n") {
        fs::copy(&file, &archive)?;
    } else {
        let output: Output = Command::new("xcrun")
            .args(["lipo", "-thin", slice])
            .arg(&file)
            .arg("-output")
            .arg(&archive)
            .output()?;
        if !output.status.success() {
            return Err(io::Error::other(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }
    }
    println!("cargo:rustc-link-search=native={}", directory.display());
    println!("cargo:rustc-link-lib=static={runtime}");
    Ok(())
}

/// Generates stable configuration and bundled resource source for Tauri.
///
/// # Returns
/// - `io::Result<()>` - Success after generating inputs, or an IO failure.
fn main() -> io::Result<()> {
    tauri_build::build();
    let target_os: String = env::var("CARGO_CFG_TARGET_OS").map_err(io::Error::other)?;
    if target_os == "ios" || target_os == MACOS_TARGET {
        let mut display: Build = Build::new();
        display
            .flag(OBJC_ARC_FLAG)
            .file("apple/display.m")
            .try_compile("euv_apple_display")
            .map_err(io::Error::other)?;
        println!("cargo:rerun-if-changed=apple/display.m");
        println!("cargo:rustc-link-lib=framework=QuartzCore");
        link_apple_runtime(&display, &target_os)?;
    }
    if target_os == "ios" {
        let mut lifecycle: Build = Build::new();
        lifecycle
            .cpp(true)
            .flag(OBJC_ARC_FLAG)
            .file("apple/lifecycle.mm")
            .try_compile("euv_ios_lifecycle")
            .map_err(io::Error::other)?;
        println!("cargo:rerun-if-changed=apple/lifecycle.mm");
    }
    let manifest_dir: PathBuf =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").map_err(io::Error::other)?);
    let out_dir: PathBuf =
        PathBuf::from(env::var(CARGO_OUTPUT_DIRECTORY).map_err(io::Error::other)?);
    let config_path: PathBuf = manifest_dir
        .parent()
        .ok_or_else(|| io::Error::other("Manifest directory has no parent"))?
        .join("app.config.json");
    println!("cargo:rerun-if-changed={}", config_path.display());
    let config_content: String = fs::read_to_string(&config_path)?;
    let config: serde_json::Value =
        serde_json::from_str(&config_content).map_err(io::Error::other)?;
    let remote_url: &str = config["remote"]["url"]
        .as_str()
        .ok_or_else(|| io::Error::other("remote.url must be a string"))?;
    let cache_directory: &str = config["cache"]["directory"]
        .as_str()
        .ok_or_else(|| io::Error::other("cache.directory must be a string"))?;
    let immersive: bool = config["ui"]["immersiveMode"]
        .as_bool()
        .ok_or_else(|| io::Error::other("ui.immersiveMode must be a boolean"))?;
    let maximum_refresh: bool = config["ui"]["maxFrameRateEnabled"]
        .as_bool()
        .ok_or_else(|| io::Error::other("ui.maxFrameRateEnabled must be a boolean"))?;
    let config_code: String = format!(
        "pub(crate) const REMOTE_URL:&str=\"{}\";pub(crate) const CACHE_DIR:&str=\"{}\";#[cfg(any(target_os=\"ios\",target_os=\"macos\"))]pub(crate) const IMMERSIVE_MODE:bool={};#[cfg(any(target_os=\"ios\",target_os=\"macos\"))]pub(crate) const MAX_FRAME_RATE_ENABLED:bool={};",
        remote_url, cache_directory, immersive, maximum_refresh,
    );
    write_if_changed(&out_dir.join("config_generated.rs"), &config_code)?;
    let bundled_cache_dir: PathBuf = manifest_dir.join("bundled-cache");
    // A missing watched directory is permanently dirty. Materialize an ignored
    // marker without embedding it in the generated resource table.
    fs::create_dir_all(&bundled_cache_dir)?;
    let placeholder: PathBuf = bundled_cache_dir.join(BUNDLED_CACHE_PLACEHOLDER);
    if !placeholder.exists() {
        fs::write(&placeholder, [])?;
    }
    let generated_file: PathBuf = out_dir.join("bundled_cache_data.rs");
    println!("cargo:rerun-if-changed=bundled-cache");
    if bundled_cache_dir.exists() && bundled_cache_dir.is_dir() {
        let mut files: Vec<PathBuf> = Vec::new();
        walk_dir(&bundled_cache_dir, &bundled_cache_dir, &mut files);
        files.retain(|file_path: &PathBuf| {
            file_path.file_name().is_some_and(|name: &OsStr| {
                name != "_manifest.json" && name != BUNDLED_CACHE_PLACEHOLDER
            })
        });
        files.sort();
        let mut code: String = String::from("pub(crate) const BUNDLED_FILES:&[(&str,&[u8])]=&[");
        for file in &files {
            let relative_str: String = file
                .to_str()
                .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidData))?
                .replace('\\', "/");
            let absolute_str: String = bundled_cache_dir
                .join(file)
                .to_str()
                .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidData))?
                .replace('\\', "/");
            code.push_str(&format!(
                "(\"{relative_str}\",include_bytes!(\"{absolute_str}\")),"
            ));
        }
        code.push_str("];");
        write_if_changed(&generated_file, &code)?;
        for file in &files {
            println!(
                "cargo:rerun-if-changed={}",
                bundled_cache_dir.join(file).display()
            );
        }
    } else {
        write_if_changed(
            &generated_file,
            "pub(crate) const BUNDLED_FILES:&[(&str,&[u8])]=&[];",
        )?;
    }
    Ok(())
}
