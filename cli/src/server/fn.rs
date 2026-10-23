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
        EuvError::Message(ERROR_GLOBAL_STATE_ALREADY_INITIALIZED.to_string())
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
/// The wasm-bindgen JS bridge wasm-pack wrote into the output directory is
/// read, its module graph is inlined, and the result is wrapped in a
/// self-contained IIFE that fetches the `.wasm` module and calls `main()`.
/// The page then carries no JavaScript request of its own, which is what
/// lets [`remove_generated_js`](crate::build::remove_generated_js) delete
/// every generated `.js` afterwards.
///
/// There is no fallback path. A missing bridge, an unreadable snippet, or
/// a construct the inliner cannot reproduce is an error: silently
/// degrading to `<script type="module">import … from './pkg/x.js'` would
/// leave the deployed tree importing a file this build is about to delete.
///
/// # Arguments
///
/// - `&HtmlConfig` - The HTML configuration holding the serving root and import path.
///
/// # Returns
///
/// - `Result<String, EuvError>` - The bootstrap script body to inline into
///   the generated HTML, or the reason the bridge could not be inlined.
async fn resolve_inline_js(config: &HtmlConfig) -> Result<String, EuvError> {
    let (pkg_dir, wasm_url): (PathBuf, String) = resolve_bridge_location(config)?;
    let js_name: String = config
        .get_import_path()
        .rsplit('/')
        .next()
        .unwrap_or_default()
        .to_string();
    let js_path: PathBuf = pkg_dir.join(&js_name);
    if !js_path.is_file() {
        return Err(EuvError::IoPath {
            message: format!(
                "wasm-pack wrote no JS bridge to inline; {} is missing, so this build \
                 cannot produce a JavaScript-free page",
                js_path.display()
            ),
            path: js_path,
            error: io::Error::new(io::ErrorKind::NotFound, "no wasm-bindgen JS bridge"),
        });
    }
    build_inline_bridge(&pkg_dir, &js_name, &wasm_url).await
}

/// Derives the wasm-pack output directory and the `.wasm` URL the inlined
/// bootstrap must fetch, from the configured import path alone.
///
/// The import path is relative to the serving root, so its directory part
/// names the output directory even when `--out-dir` points somewhere other
/// than `{www-dir}/pkg`, at any nesting depth, or at an absolute location.
/// The `.wasm` URL is the bridge's own URL with the `.js` suffix swapped,
/// which keeps it correct for every one of those layouts.
///
/// # Arguments
///
/// - `&HtmlConfig` - The HTML configuration holding the serving root and import path.
///
/// # Returns
///
/// - `Result<(PathBuf, String), EuvError>` - The output directory and the
///   `.wasm` URL, or an error when the import path does not name a bridge.
fn resolve_bridge_location(config: &HtmlConfig) -> Result<(PathBuf, String), EuvError> {
    let import_path: &str = config.get_import_path();
    let Some((relative_dir, js_name)) = import_path.rsplit_once('/') else {
        return Err(EuvError::Message(format!(
            "import path '{import_path}' does not name a generated bridge, so this build \
             cannot produce a JavaScript-free page"
        )));
    };
    let js_name: &str = js_name.split(['?', '#']).next().unwrap_or(js_name);
    let Some(stem) = js_name.strip_suffix(JS_EXTENSION) else {
        return Err(EuvError::Message(format!(
            "import path '{import_path}' does not name a JavaScript bridge, so this build \
             cannot produce a JavaScript-free page"
        )));
    };
    let pkg_dir: PathBuf = config.get_serving_root().join(relative_dir);
    let wasm_url: String = format!("{relative_dir}/{stem}{WASM_FILE_SUFFIX}");
    Ok((pkg_dir, wasm_url))
}

/// Normalises every bootstrap in `template` to the inline form.
///
/// A template that already uses [`INLINE_JS_PLACEHOLDER`] is returned
/// unchanged. A legacy template that still imports the bridge by URL has
/// each `<script>` element carrying that import replaced, as a whole
/// element, by the inline-bootstrap placeholder — the author's other
/// scripts, their attributes, and their bodies are left exactly as
/// written, so a page that also embeds analytics or a font loader keeps
/// them and never ends up calling `main()` twice.
///
/// A template with neither placeholder gets the placeholder injected into
/// its `</body>`, because a page without it can never boot the WASM module.
///
/// # Arguments
///
/// - `&str` - The template source.
///
/// # Returns
///
/// - `Result<String, EuvError>` - The template with exactly one bootstrap
///   placeholder, or an error naming what made that impossible.
pub(crate) fn normalize_bootstrap(template: &str) -> Result<String, EuvError> {
    let script_tag: String = format!("<script>{INLINE_JS_PLACEHOLDER}</script>");
    let template: String = replace_module_bootstrap(template, &script_tag);
    if template.contains(IMPORT_PATH_PLACEHOLDER) {
        return Err(EuvError::Message(String::from(
            "the index.html template still carries __IMPORT_PATH__ outside a bootstrap \
             script; replace it with __EUV_INLINE_JS__",
        )));
    }
    match template.matches(INLINE_JS_PLACEHOLDER).count() {
        1 => return Ok(template),
        0 => {}
        _ => {
            return Err(EuvError::Message(String::from(
                "the index.html template must contain exactly one WASM bootstrap",
            )));
        }
    }
    let body_close: usize = template.rfind("</body>").ok_or_else(|| {
        EuvError::Message(String::from(
            "the index.html template has no __EUV_INLINE_JS__ placeholder and no </body> to \
             inject one before; add the placeholder so the page boots its WASM module",
        ))
    })?;
    let mut out: String = String::with_capacity(template.len() + script_tag.len());
    out.push_str(&template[..body_close]);
    out.push_str(&script_tag);
    out.push('\n');
    out.push_str(&template[body_close..]);
    Ok(out)
}

/// Replaces every `<script>` element whose body imports a module with
/// `replacement`, leaving every other byte of `template` untouched.
///
/// # Arguments
///
/// - `&str` - The complete HTML template.
///
/// # Returns
///
/// - `String` - The template with only legacy WASM bootstrap elements replaced.
fn replace_module_bootstrap(template: &str, replacement: &str) -> String {
    let lowered: String = template.to_ascii_lowercase();
    let mut out: String = String::with_capacity(template.len());
    let mut cursor: usize = 0;
    while let Some(open_offset) = lowered[cursor..].find("<script") {
        let open: usize = cursor + open_offset;
        let Some(open_end) = lowered[open..].find('>').map(|end: usize| open + end + 1) else {
            break;
        };
        let tag: &str = &template[open..open_end];
        let tag_end: Option<usize> = lowered[open_end..]
            .find("</script")
            .map(|end: usize| open_end + end);
        let Some(close_end) = tag_end
            .and_then(|close: usize| lowered[close..].find('>').map(|end: usize| close + end + 1))
        else {
            break;
        };
        let body: &str = &template[open_end..tag_end.unwrap_or(close_end)];
        out.push_str(&template[cursor..open]);
        if body.contains(IMPORT_PATH_PLACEHOLDER) && !tag.contains("src=") && !tag.contains("src =")
        {
            out.push_str(replacement);
        } else {
            out.push_str(&template[open..close_end]);
        }
        cursor = close_end;
    }
    out.push_str(&template[cursor..]);
    out
}

/// Generates `index.html` based on the build profile.
///
/// Uses `INDEX_HTML_RELEASE` when `is_release` is `true` (no live-reload script),
/// otherwise uses `INDEX_HTML_DEV` (includes live-reload instrumentation).
///
/// The wasm-bindgen JS bridge is **mandatory and self-contained**: Rust
/// reads the bridge wasm-pack wrote, inlines its module graph, and embeds
/// the body as a `<script>` in the page. The `.js`, `.d.ts`, `.js.map` and
/// `snippets/` files it inlined are then deleted, so the deployed tree
/// holds no JavaScript at all. A bridge that cannot be inlined fails the
/// build rather than degrading to an external module import.
///
/// A custom `--index-html` template may either carry
/// `__EUV_INLINE_JS__` or still carry the legacy `__IMPORT_PATH__` module
/// bootstrap, in which case the whole `<script>` element that imports the
/// bridge is replaced by the inline one and every other script the author
/// wrote is preserved verbatim.
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
                    message: ERROR_READ_CUSTOM_INDEX_HTML.to_string(),
                    path: custom_path.to_path_buf(),
                    error,
                })?;
        String::from_utf8(bytes).map_err(|error: FromUtf8Error| EuvError::Utf8 {
            message: ERROR_CUSTOM_INDEX_HTML_NOT_UTF8.to_string(),
            error,
        })?
    } else if config.get_is_release() {
        INDEX_HTML_RELEASE.to_string()
    } else {
        INDEX_HTML_DEV.to_string()
    };
    let inline_js: String = resolve_inline_js(config).await?;
    let template_content: String = normalize_bootstrap(&template_content)?;
    let base_href_tag: String = if let Some(prefix) = config.try_get_serving_route_prefix() {
        let path: String = format!("/{prefix}");
        format!("<base href=\"{}/\" />", path.trim_end_matches('/'))
    } else {
        String::new()
    };
    let html: String = template_content
        .replace(BASE_HREF_PLACEHOLDER, &base_href_tag)
        .replace(RELOAD_ROUTE_PLACEHOLDER, RELOAD_ROUTE)
        .replace(INLINE_JS_PLACEHOLDER, &inline_js)
        .replace(LOADING_STYLE_PLACEHOLDER, LOADING_STYLE)
        .replace(LOADING_ELEMENT_PLACEHOLDER, LOADING_ELEMENT)
        .replace(LOADING_REMOVAL_PLACEHOLDER, LOADING_REMOVAL_SCRIPT);
    let index_path: PathBuf = config.get_serving_root().join(INDEX_HTML_FILE_NAME);
    create_dir_all(config.get_serving_root())
        .await
        .map_err(|error: io::Error| EuvError::Io {
            message: ERROR_CREATE_STATIC_DIRECTORY.to_string(),
            error,
        })?;
    write(&index_path, &html)
        .await
        .map_err(|error: io::Error| EuvError::Io {
            message: ERROR_WRITE_INDEX_HTML.to_string(),
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
