use std::{
    env,
    ffi::OsStr,
    fs, io,
    path::{Path, PathBuf},
};

use cc::Build;

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

/// Generates stable configuration and bundled resource source for Tauri.
///
/// # Returns
/// - `io::Result<()>` - Success after generating inputs, or an IO failure.
fn main() -> io::Result<()> {
    tauri_build::build();
    if env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("ios") {
        let mut lifecycle: Build = Build::new();
        lifecycle
            .cpp(true)
            .flag("-fobjc-arc")
            .file("apple/lifecycle.mm")
            .try_compile("euv_ios_lifecycle")
            .map_err(io::Error::other)?;
        println!("cargo:rerun-if-changed=apple/lifecycle.mm");
    }
    let manifest_dir: PathBuf =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").map_err(io::Error::other)?);
    let out_dir: PathBuf = PathBuf::from(env::var("OUT_DIR").map_err(io::Error::other)?);
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
    let config_code: String = format!(
        "pub(crate) const REMOTE_URL:&str=\"{}\";pub(crate) const CACHE_DIR:&str=\"{}\";",
        remote_url, cache_directory,
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
