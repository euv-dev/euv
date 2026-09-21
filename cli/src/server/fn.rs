use super::*;

/// Sets the global application state.
///
/// # Arguments
///
/// - `Arc<AppState>` - The shared application state to store globally.
///
/// # Returns
///
/// - `Result<(), EuvError>` - Indicates success or failure of the initialization.
pub(crate) fn set_global_state(state: Arc<AppState>) -> Result<(), EuvError> {
    APP_STATE.set(state).map_err(|_: Arc<AppState>| {
        EuvError::Message(String::from("Global state already initialized"))
    })
}

/// Retrieves the global application state.
///
/// # Returns
///
/// - `Option<Arc<AppState>>` - The global state if initialized.
pub(crate) fn get_global_state() -> Option<Arc<AppState>> {
    APP_STATE.get().cloned()
}

/// Resolves the bootstrap script body to inline into the generated HTML.
///
/// When JS bridge inlining is enabled and the `pkg/<name>.js` file is present
/// after a successful `wasm-pack` build, reads the bridge, strips its
/// top-level `import` statements, inlines the 2 snippet helper modules, and
/// wraps everything in a synchronous IIFE that fetches the wasm and calls
/// `main()`. The result is zero extra HTTP requests for the JS bridge file
/// and no ES module graph parsing on the critical path.
///
/// When inlining is disabled (env var or missing `pkg/<name>.js` file),
/// returns the classic `<script type="module">` import fallback so the
/// page still boots.
async fn resolve_inline_js(config: &HtmlConfig) -> String {
    if inline_bridge_disabled() {
        return build_module_fallback_bridge(config.get_import_path());
    }
    let pkg_dir: PathBuf = config.get_serving_root().join(PKG_DIR_NAME);
    let js_name: String = config
        .get_import_path()
        .rsplit('/')
        .next()
        .unwrap_or("")
        .to_string();
    if js_name.is_empty() {
        return build_module_fallback_bridge(config.get_import_path());
    }
    let js_path: PathBuf = pkg_dir.join(&js_name);
    if !js_path.exists() {
        return build_module_fallback_bridge(config.get_import_path());
    }
    let wasm_url: String = if let Some(stem) = js_name.strip_suffix(".js") {
        format!("pkg/{stem}_bg.wasm")
    } else {
        config.get_import_path().replace(".js", "_bg.wasm")
    };
    match build_inline_bridge(&pkg_dir, &js_name, &wasm_url).await {
        Ok(snippet) => snippet,
        Err(error) => {
            log::warn!("Falling back to module import bridge: {error}");
            build_module_fallback_bridge(config.get_import_path())
        }
    }
}

/// Generates `index.html` based on the build profile.
///
/// Uses `INDEX_HTML_RELEASE` when `is_release` is `true` (no live-reload script),
/// otherwise uses `INDEX_HTML_DEV` (includes live-reload instrumentation).
///
/// The wasm-bindgen JS bridge is **inlined** into the HTML at build time
/// (Rust reads `pkg/<name>.js`, strips its top-level `import` statements,
/// inlines the 2 snippet helpers, and wraps the body in a synchronous IIFE)
/// so the browser incurs zero extra HTTP requests for the JS bridge file
/// and skips ES module graph parsing. Set the `EUV_NO_INLINE_BRIDGE` env
/// var to opt out.
///
/// Then writes the template with the import path placeholder replaced to disk.
///
/// # Arguments
///
/// - `&HtmlConfig` - The HTML generation configuration.
///
/// # Returns
///
/// - `Result<String, EuvError>` - The generated HTML content written to disk.
pub(crate) async fn generate_html(config: &HtmlConfig) -> Result<String, EuvError> {
    let template_content: String = if let Some(custom_path) = config.try_get_custom_index_html() {
        let bytes: Vec<u8> =
            read(custom_path)
                .await
                .map_err(|error: io::Error| EuvError::IoPath {
                    message: String::from("Failed to read custom index.html"),
                    path: custom_path.to_path_buf(),
                    error,
                })?;
        String::from_utf8(bytes).map_err(|error: FromUtf8Error| EuvError::Utf8 {
            message: String::from("Custom index.html is not valid UTF-8"),
            error,
        })?
    } else if config.get_is_release() {
        INDEX_HTML_RELEASE.to_string()
    } else {
        INDEX_HTML_DEV.to_string()
    };
    let inline_js: String = resolve_inline_js(config).await;
    let base_href_tag: String = if config.get_is_dev_server() {
        format!(
            "<base href=\"/{}/\" />",
            config
                .get_serving_root()
                .file_name()
                .and_then(|name: &ffi::OsStr| name.to_str())
                .unwrap_or("")
        )
    } else {
        String::new()
    };
    let html: String = template_content
        .replace(BASE_HREF_PLACEHOLDER, &base_href_tag)
        .replace(IMPORT_PATH_PLACEHOLDER, config.get_import_path())
        .replace(RELOAD_ROUTE_PLACEHOLDER, RELOAD_ROUTE)
        .replace(INLINE_JS_PLACEHOLDER, &inline_js);
    let index_path: PathBuf = config.get_serving_root().join(INDEX_HTML_FILE_NAME);
    create_dir_all(config.get_serving_root())
        .await
        .map_err(|error: io::Error| EuvError::Io {
            message: String::from("Failed to create static directory"),
            error,
        })?;
    write(&index_path, &html)
        .await
        .map_err(|error: io::Error| EuvError::Io {
            message: String::from("Failed to write index.html"),
            error,
        })?;
    Ok(html)
}

/// Resolves the effective www directory, handling wasm-pack nested output.
///
/// # Arguments
///
/// - `&Path` - The candidate www directory path.
///
/// # Returns
///
/// - `PathBuf` - The resolved www directory containing `index.html`.
pub async fn resolve_www_dir(www_dir: &Path) -> PathBuf {
    if metadata(www_dir.join(INDEX_HTML_FILE_NAME)).await.is_ok() {
        return www_dir.to_path_buf();
    }
    let parent_name: Option<&str> = www_dir
        .file_name()
        .and_then(|file_name_os_str: &ffi::OsStr| file_name_os_str.to_str());
    if let Some(name) = parent_name {
        let nested: PathBuf = www_dir.join(name);
        if metadata(nested.join(INDEX_HTML_FILE_NAME)).await.is_ok() {
            return nested;
        }
    }
    www_dir.to_path_buf()
}

/// Resolves the pkg directory for serving WASM artifacts.
///
/// Delegates to `resolve_out_dir` which respects `--out-dir`
/// from wasm-pack args or defaults to `{www_dir}/pkg`.
///
/// # Arguments
///
/// - `&ModeArgs` - The CLI arguments for resolving out_dir.
///
/// # Returns
///
/// - `PathBuf` - The resolved pkg directory containing WASM build artifacts.
pub fn resolve_pkg_dir(args: &ModeArgs) -> PathBuf {
    resolve_out_dir(args)
}

/// Resolves a file path within a base directory with path-traversal protection.
///
/// Canonicalizes both the file path and the base directory, then verifies
/// that the canonical file path starts with the canonical base directory.
///
/// # Arguments
///
/// - `&Path` - The base directory to resolve within.
/// - `&str` - The relative path to the file.
///
/// # Returns
///
/// - `Option<PathBuf>` - The resolved file path if valid, `None` otherwise.
pub async fn resolve_file_in_base(base: &Path, path: &str) -> Option<PathBuf> {
    let file_path: PathBuf = base.join(path);
    let canonical_path: PathBuf = canonicalize(&file_path).await.ok()?;
    let base_canonical: PathBuf = canonicalize(base).await.ok()?;
    canonical_path
        .starts_with(&base_canonical)
        .then_some(file_path)
}
