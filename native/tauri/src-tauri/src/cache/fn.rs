use super::*;

/// Injects page-level scripts into a webview as soon as a page starts loading.
/// Registered via [`Builder::on_page_load`], so it runs for **every** webview,
/// including dynamically created ones, on every navigation.
///
/// Currently injects:
/// - [`LINK_INTERCEPTOR_SCRIPT`] — routes external links to the system browser.
/// - [`IMMERSIVE_SAFE_AREA_SCRIPT`] — applies the real top safe-area inset so
///   the immersive WebView's page navbar does not overlap the status bar.
///
/// # Arguments
///
/// - `&Webview` - The webview that is loading a page.
/// - `&PageLoadPayload<'_>` - The page load event payload (used to gate on start).
fn inject_page_scripts(webview: &Webview, payload: &PageLoadPayload<'_>) {
    // Inject as early as possible so listeners are registered before user
    // interaction. We inject on the start event of every page load.
    if payload.event() != PageLoadEvent::Started {
        return;
    }
    let scripts: String =
        format!(";{LINK_INTERCEPTOR_SCRIPT};{IMMERSIVE_SAFE_AREA_SCRIPT};{GPU_PROBE_SCRIPT};");
    if let Err(error) = webview.eval(&scripts) {
        euv_log!("[EUV] failed to inject native page scripts: {}", error);
    }
}

/// Initializes and runs the Tauri application with EUV cache scheme.
///
/// Sets up the custom URI scheme handler, deploys bundled cache,
/// and starts the background update or initial fetch process.
///
/// # Errors
///
/// Reports initialization failures to standard error without panicking.
#[cfg_attr(target_os = "ios", tauri::mobile_entry_point)]
pub fn run() {
    #[cfg(target_os = "android")]
    {
        tauri::android_binding!(com, euv, run, tauri::wry);
    }
    if let Err(error) = shared_client() {
        eprintln!("Failed to initialize the HTTP client: {error}");
        exit(1);
    }
    match Builder::default()
        .append_invoke_initialization_script(format!(
            ";{LINK_INTERCEPTOR_SCRIPT};{IMMERSIVE_SAFE_AREA_SCRIPT};{GPU_PROBE_SCRIPT};"
        ))
        .manage(CacheSession::default())
        // Registered once at the Builder level so it runs for EVERY webview the
        // app owns, including ones created dynamically at runtime. It injects a
        // script that routes any external link to the system browser instead of
        // navigating in-app, plus the immersive safe-area bridge that keeps the
        // page navbar clear of the system status bar.
        .on_page_load(inject_page_scripts)
        .plugin(tauri_plugin_opener::init())
        .setup(|app: &mut App| {
            app.handle().plugin(
                tauri_plugin_log::Builder::default()
                    .level(::log::LevelFilter::Info)
                    .build(),
            )?;
            #[cfg(any(target_os = "ios", target_os = "macos"))]
            {
                let Some(main_window): Option<WebviewWindow> =
                    app.get_webview_window(MAIN_WINDOW_LABEL)
                else {
                    return Err(io::Error::other("The native mobile window was not created").into());
                };
                spawn(async move {
                    if let Err(error) = main_window.with_webview(|view: PlatformWebview| {
                        // SAFETY: Tauri supplies the live WebView on its UI thread.
                        if unsafe {
                            euv_configure_apple_display(
                                view.inner(),
                                i32::from(IMMERSIVE_MODE),
                                i32::from(MAX_FRAME_RATE_ENABLED),
                            )
                        } != 1
                        {
                            eprintln!("Failed to configure the native display policy");
                            exit(1);
                        }
                        #[cfg(target_os = "ios")]
                        // SAFETY: The same Tauri-owned WebView is still live on its UI thread.
                        if unsafe { euv_configure_ios_webview(view.inner()) } != 1 {
                            eprintln!("Failed to configure the native mobile viewport");
                            exit(1);
                        }
                    }) {
                        eprintln!("Failed to dispatch native viewport configuration: {error}");
                        exit(1);
                    }
                });
            }
            let handle: AppHandle = app.handle().clone();
            #[cfg(debug_assertions)]
            let _: Result<(), AppHandle> = APP_HANDLE.set(handle.clone());
            spawn(async move {
                initial_fetch(handle).await;
            });
            Ok(())
        })
        .register_asynchronous_uri_scheme_protocol(
            SCHEME_NAME,
            move |context: UriSchemeContext<'_, Wry>,
                  request: Request<Vec<u8>>,
                  responder: UriSchemeResponder| {
                let app_handle: AppHandle = context.app_handle().clone();
                spawn(async move {
                    let response: response::Response<Cow<'static, [u8]>> =
                        handle_euv_scheme(&app_handle, request).await;
                    responder.respond(response);
                });
            },
        )
        .invoke_handler(generate_handler![
            load_cached_resource,
            wait_for_cached_resource,
            update_cache,
            resolve_bridge_group_permissions
        ])
        .build(generate_context!())
    {
        Ok(app) => {
            #[cfg(target_os = "ios")]
            {
                // SAFETY: Building the runtime registers its delegate on the main thread.
                if unsafe { euv_install_scene_lifetime_fix() } != 1 {
                    eprintln!("Failed to install the iOS scene ownership repair");
                    exit(1);
                }
            }
            app.run(|_: &AppHandle, _: RunEvent| {});
        }
        Err(error) => {
            eprintln!("Failed to initialize the application: {error}");
            exit(1);
        }
    }
}

/// Resolves the application cache directory path.
///
/// # Arguments
///
/// - `&AppHandle` - The Tauri application handle used to resolve the cache directory.
///
/// # Returns
///
/// - `Result<PathBuf, CacheError>` - The cache root directory path, or an error if resolution fails.
pub(crate) fn get_cache_root(app_handle: &AppHandle) -> Result<PathBuf, CacheError> {
    let mut cache_directory: PathBuf = app_handle
        .path()
        .app_cache_dir()
        .map_err(cache_read_error)?;
    cache_directory.push(CACHE_DIR);
    Ok(cache_directory)
}

/// Converts a cache directory lookup error into a readable cache error.
///
/// # Arguments
/// - `T` - The displayable directory lookup error.
///
/// # Returns
/// - `CacheError` - The cache read error containing the original message.
fn cache_read_error<T>(error: T) -> CacheError
where
    T: fmt::Display,
{
    CacheError::Read(error.to_string())
}

/// Returns the path to the active version pointer file.
///
/// # Arguments
///
/// - `&Path` - The cache root directory path.
///
/// # Returns
///
/// - `PathBuf` - The path to the active pointer file.
fn active_pointer_path(cache_root: &Path) -> PathBuf {
    cache_root.join(ACTIVE_LINK)
}

/// Reads the currently active version name from the pointer file asynchronously.
///
/// # Arguments
///
/// - `&Path` - The cache root directory path.
///
/// # Returns
///
/// - `Option<String>` - The active version name if it exists and is valid, otherwise `None`.
async fn read_active_version(cache_root: &Path) -> Option<String> {
    let name: String = read_to_string(active_pointer_path(cache_root)).await.ok()?;
    let name: String = name.trim().to_string();
    if name.is_empty()
        || !name.starts_with(VERSION_PREFIX)
        || !name
            .bytes()
            .all(|byte: u8| byte.is_ascii_alphanumeric() || byte == VERSION_SEPARATOR)
    {
        return None;
    }
    if metadata(cache_root.join(&name)).await.ok()?.is_dir() {
        Some(name)
    } else {
        None
    }
}

/// Generates a new version directory name using the current timestamp.
///
/// # Returns
///
/// - `String` - A version name in the format `v_<timestamp_millis>`.
fn new_version_name() -> String {
    let timestamp: u128 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    format!("{VERSION_PREFIX}{timestamp}")
}

/// Switches the active version pointer to the specified version asynchronously.
///
/// Uses atomic rename to ensure the pointer file is updated safely.
///
/// # Arguments
///
/// - `&Path` - The cache root directory path.
/// - `&str` - The version name to set as active.
///
/// # Returns
///
/// - `Result<(), CacheError>` - Ok if the switch succeeded, or a write error.
async fn switch_active_version(cache_root: &Path, version_name: &str) -> Result<(), CacheError> {
    let pointer: PathBuf = active_pointer_path(cache_root);
    let temporary: PathBuf = cache_root.join(format!(".active_tmp_{}", id()));
    write(&temporary, version_name)
        .await
        .map_err(|error: io::Error| CacheError::Write(error.to_string()))?;
    rename(&temporary, &pointer)
        .await
        .map_err(|error: io::Error| CacheError::Write(error.to_string()))?;
    euv_log!("[EUV] switched active: {}", version_name);
    Ok(())
}

/// Removes old version directories, keeping only the most recent ones.
///
/// Deletion is performed concurrently for faster cleanup.
///
/// # Arguments
///
/// - `&Path` - The cache root directory path.
/// - `&str` - The current active version name (will not be removed).
/// - `Option<&Path>` - The running session snapshot, which must remain available.
async fn cleanup_old_versions(cache_root: &Path, current: &str, pinned: Option<&Path>) {
    let mut read_dir_result: ReadDir = match read_dir(cache_root).await {
        Ok(entries) => entries,
        Err(_) => return,
    };
    let mut versions: Vec<String> = Vec::new();
    while let Ok(Some(entry)) = read_dir_result.next_entry().await {
        let name: String = entry.file_name().to_string_lossy().to_string();
        if name.starts_with(VERSION_PREFIX)
            && name != current
            && pinned != Some(entry.path().as_path())
            && let Ok(entry_metadata) = entry.metadata().await
            && entry_metadata.is_dir()
        {
            versions.push(name);
        }
    }
    versions.sort();
    let max_old: usize = MAX_KEPT_VERSIONS.saturating_sub(1);
    if versions.len() <= max_old {
        return;
    }
    let to_remove: usize = versions.len() - max_old;
    let remove_handles: Vec<JoinHandle<()>> = versions[..to_remove]
        .iter()
        .map(|name: &String| {
            let dir_path: PathBuf = cache_root.join(name);
            let log_name: String = name.clone();
            tauri::async_runtime::spawn(async move {
                remove_dir_all(&dir_path).await.ok();
                euv_log!("[EUV] removed old version: {}", log_name);
            })
        })
        .collect();
    for handle in remove_handles {
        handle.await.ok();
    }
}

/// Returns a reference to the global shared HTTP client.
///
/// Reuses a single `Client` instance with connection pooling across all fetch operations,
/// avoiding repeated TLS handshake and DNS resolution overhead.
///
/// # Returns
///
/// - `Result<&'static Client, CacheError>` - The shared client or its initialization error.
fn shared_client() -> Result<&'static Client, CacheError> {
    match HTTP_CLIENT.get_or_init(|| {
        Client::builder()
            .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS))
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .map_err(|error: Error| error.to_string())
    }) {
        Ok(client) => Ok(client),
        Err(error) => Err(CacheError::Fetch(error.clone())),
    }
}

/// Strips leading slashes and `./` prefixes from a relative path.
///
/// Avoids repeated `trim_start_matches` + `to_string()` allocations by centralizing the logic.
///
/// # Arguments
///
/// - `&str` - The relative path to clean.
///
/// # Returns
///
/// - `String` - The cleaned path.
fn clean_relative_path(path: &str) -> String {
    let mut result: &str = path;
    loop {
        let trimmed: &str = result.trim_start_matches('/');
        if let Some(stripped) = trimmed.strip_prefix("./") {
            result = stripped;
        } else if trimmed.len() < result.len() {
            result = trimmed;
        } else {
            break;
        }
    }
    result.to_string()
}

/// Resolves the embedded document graph without requiring unrelated build output.
///
/// # Returns
/// - `Vec<String>` - Entry documents and every locally referenced resource.
fn bundled_snapshot_paths() -> Vec<String> {
    let mut pending: VecDeque<String> = snapshot_paths(BUNDLED_FILES).into();
    let mut required: Vec<String> = Vec::new();
    let mut visited: HashSet<String> = HashSet::new();
    while let Some(path) = pending.pop_front() {
        if !visited.insert(path.clone()) {
            continue;
        }
        let Some((_, bytes)): Option<&(&str, &[u8])> = BUNDLED_FILES
            .iter()
            .find(|(name, _): &&(&str, &[u8])| *name == path)
        else {
            continue;
        };
        if !path.ends_with(HTML_EXTENSION)
            && !path.ends_with(JS_EXTENSION)
            && !path.ends_with(MJS_EXTENSION)
        {
            required.push(path);
            continue;
        }
        let mut references: Vec<String> = Vec::new();
        let source: Cow<'_, str> = String::from_utf8_lossy(bytes);
        if path.ends_with(HTML_EXTENSION) {
            let mut scripts: Vec<String> = Vec::new();
            extract_resources_with_scraper(&source, &mut references, &mut scripts);
            for script in &scripts {
                extract_resources_from_js_ast(script, &mut references);
            }
        } else if path.ends_with(JS_EXTENSION) || path.ends_with(MJS_EXTENSION) {
            extract_resources_from_js_ast(&source, &mut references);
        }
        for reference in references {
            if !is_external_resource_url(&reference) {
                pending.push_back(resolve_local_path(&path, &reference));
            }
        }
        required.push(path);
    }
    required
}

/// Seeds bundled files on first launch and repairs incomplete older snapshots.
///
/// Complete remote snapshots are preserved across launches. An older root-only
/// cache is not complete once theme pages are part of the packaged inventory;
/// replace that unusable snapshot with the complete offline bundle before
/// publishing readiness. Background updates still take effect next launch.
///
/// # Arguments
///
/// - `&AppHandle` - The Tauri application handle.
///
/// # Returns
///
/// - `bool` - `true` if an active version exists (reused or freshly seeded),
///   `false` on error or when there is nothing to seed.
pub(crate) async fn deploy_bundled_cache(app_handle: &AppHandle) -> bool {
    let cache_root: PathBuf = match get_cache_root(app_handle) {
        Ok(directory) => directory,
        Err(_) => return false,
    };
    if create_dir_all(&cache_root).await.is_err() {
        return false;
    }
    // Preserve complete remote snapshots, but do not pin older root-only caches
    // forever: they cannot serve the theme navigation paths compiled into WASM.
    if let Some(active) = read_active_version(&cache_root).await {
        let directory: PathBuf = cache_root.join(&active);
        if let Ok(minimum) = bundled_release_version()
            && verified_release(&directory, &minimum).await.is_ok()
            && complete_snapshot(&directory, &bundled_snapshot_paths()).await
        {
            app_handle.state::<CacheSession>().publish(directory);
            euv_log!("[EUV] active cache present, reusing: {active}");
            return true;
        }
    }
    // No active version yet (fresh install): seed from the bundled snapshot so
    // the first launch has content to render while the initial fetch runs.
    if !BUNDLED_FILES
        .iter()
        .any(|(file, data): &(&str, &[u8])| *file == INDEX_FILE_NAME && !data.is_empty())
    {
        return false;
    }
    let version_name: String = new_version_name();
    let version_dir: PathBuf = cache_root.join(&version_name);
    if tokio::fs::create_dir(&version_dir).await.is_err() {
        return false;
    }
    let mut count: usize = 0;
    for (rel_path, data) in BUNDLED_FILES {
        let target: PathBuf = version_dir.join(rel_path);
        if let Some(parent) = target.parent() {
            create_dir_all(parent).await.ok();
        }
        if write(&target, data).await.is_ok() {
            count += 1;
        } else {
            remove_dir_all(&version_dir).await.ok();
            return false;
        }
    }
    if count == 0 {
        remove_dir_all(&version_dir).await.ok();
        return false;
    }
    let valid: bool = match bundled_release_version() {
        Ok(minimum) => verified_release(&version_dir, &minimum).await.is_ok(),
        Err(_) => false,
    };
    if !valid {
        remove_dir_all(&version_dir).await.ok();
        return false;
    }
    if switch_active_version(&cache_root, &version_name)
        .await
        .is_err()
    {
        remove_dir_all(&version_dir).await.ok();
        return false;
    }
    app_handle.state::<CacheSession>().publish(version_dir);
    cleanup_old_versions(&cache_root, &version_name, None).await;
    euv_log!("[EUV] deployed {count} bundled files, serving: {version_name}");
    true
}

/// Checks whether an active cache deployment exists asynchronously.
///
/// # Arguments
///
/// - `&AppHandle` - The Tauri application handle.
///
/// # Returns
///
/// - `bool` - `true` if an active version is present, `false` otherwise.
pub(crate) async fn has_active_cache(app_handle: &AppHandle) -> bool {
    app_handle.state::<CacheSession>().directory().is_some()
}

/// Reads the authoritative website version embedded in this native bundle.
///
/// # Returns
/// - `Result<String, String>` - Embedded release identity or a metadata failure.
fn bundled_release_version() -> Result<String, String> {
    let (_, bytes): &(&str, &[u8]) = BUNDLED_FILES
        .iter()
        .find(|(path, _): &&(&str, &[u8])| *path == RELEASE_NAME)
        .ok_or_else(|| "BundledMetadataMissing".to_string())?;
    Ok(parse_release(bytes, "0.0.0")?.version)
}

/// Fetches a full snapshot of the remote HTML page and all linked resources.
///
/// Verifies that all critical resources (scripts, stylesheets, images, WASM) are present
/// before switching the active version pointer.
///
/// # Arguments
///
/// - `&Path` - The cache root directory path.
/// - `&CacheSession` - The running session whose snapshot must be retained.
///
/// # Returns
///
/// - `Result<String, CacheError>` - The new version name on success, or a fetch/write error.
async fn fetch_full_snapshot(
    cache_root: &Path,
    session: &CacheSession,
) -> Result<String, CacheError> {
    let mut minimum: String = bundled_release_version().map_err(CacheError::Fetch)?;
    if let Some(active) = read_active_version(cache_root).await {
        let active_version: String = verified_release(&cache_root.join(active), &minimum)
            .await
            .map_err(CacheError::Fetch)?;
        minimum = active_version;
    }
    let version_name: String = new_version_name();
    activate_release(
        shared_client()?,
        RELEASE_BASE,
        cache_root,
        &version_name,
        &minimum,
        &snapshot_paths(BUNDLED_FILES),
    )
    .await
    .map_err(CacheError::Fetch)?;
    cleanup_old_versions(cache_root, &version_name, session.directory().as_deref()).await;
    Ok(version_name)
}

/// Performs the initial cache fetch, retrying on failure until successful.
///
/// # Arguments
///
/// - `AppHandle` - The Tauri application handle (consumed for async task ownership).
pub(crate) async fn initial_fetch(app_handle: AppHandle) {
    let session: State<'_, CacheSession> = app_handle.state::<CacheSession>();
    let _update: MutexGuard<'_, ()> = session.get_update().lock().await;
    if has_active_cache(&app_handle).await || deploy_bundled_cache(&app_handle).await {
        return;
    }
    euv_log!("[EUV] initial fetch started");
    let cache_root: PathBuf = match get_cache_root(&app_handle) {
        Ok(directory) => directory,
        Err(error) => {
            session.fail(error.to_string());
            return;
        }
    };
    if let Err(error) = create_dir_all(&cache_root).await {
        session.fail(error.to_string());
        return;
    }
    loop {
        match fetch_full_snapshot(&cache_root, &session).await {
            Ok(version) => {
                session.publish(cache_root.join(&version));
                // Publish the first complete snapshot to this session without reloading.
                euv_log!("[EUV] initial fetch ready: {}", version);
                return;
            }
            Err(error) => {
                euv_log!("[EUV] initial fetch failed: {}, retrying", error);
                tokio::time::sleep(Duration::from_millis(RETRY_INTERVAL_MILLIS)).await;
            }
        }
    }
}

/// Resolves a relative reference into a local cache path, relative to the
/// directory of the referencing file.
///
/// Operates on local (path-only) values so the
/// on-disk layout matches the served URL layout. For example, a `pkg/euv.js`
/// that references `euv_bg.wasm` yields `pkg/euv_bg.wasm`; a reference of
/// `../assets/x.png` yields `assets/x.png`.
///
/// # Arguments
///
/// - `&str` - The clean path of the referencing file (e.g. `pkg/euv.js`).
/// - `&str` - The relative reference as written in the source.
///
/// # Returns
///
/// - `String` - The cleaned local path where the referenced file should be stored.
fn resolve_local_path(referencing_clean_path: &str, reference: &str) -> String {
    // Root-relative reference is anchored at the cache root.
    if let Some(rooted) = reference.strip_prefix('/') {
        return clean_relative_path(rooted);
    }
    // Start from the directory of the referencing file.
    let dir: &str = match referencing_clean_path.rfind('/') {
        Some(pos) => &referencing_clean_path[..pos],
        None => "",
    };
    let mut segments: Vec<&str> = dir.split('/').filter(|s: &&str| !s.is_empty()).collect();
    for part in reference.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                segments.pop();
            }
            other => segments.push(other),
        }
    }
    segments.join("/")
}

/// Lazily initializes the cached `scraper::Selector` instances used by
/// `extract_resources_with_scraper`.
///
/// All four selector strings are compile-time constants, so the inner `Result`
/// is `Ok` in practice. The `Result` wrapper exists so the initializer can
/// avoid `unwrap` / `expect` per rust-standards §R11.4.
///
/// # Returns
///
/// - `Result<[Selector; 4], String>` - Parsed resource selectors, or an invalid selector error.
fn init_resource_selectors() -> Result<[Selector; 4], String> {
    Ok([
        Selector::parse(SCRIPT_SRC_SELECTOR).map_err(|e: SelectorErrorKind<'_>| e.to_string())?,
        Selector::parse(LINK_HREF_SELECTOR).map_err(|e: SelectorErrorKind<'_>| e.to_string())?,
        Selector::parse(IMG_SRC_SELECTOR).map_err(|e: SelectorErrorKind<'_>| e.to_string())?,
        Selector::parse(INLINE_SCRIPT_SELECTOR)
            .map_err(|e: SelectorErrorKind<'_>| e.to_string())?,
    ])
}

/// Returns true if the URL points to a non-cacheable resource (absolute URL,
/// protocol-relative URL, or inline data URI) and should be skipped during
/// resource discovery.
///
/// # Arguments
///
/// - `&str` - Shared reference to a `str`.
///
/// # Returns
///
/// - `bool` - A boolean.
fn is_external_resource_url(url: &str) -> bool {
    url.starts_with(HTTP_PREFIX)
        || url.starts_with(HTTPS_PREFIX)
        || url.starts_with(SCHEME_RELATIVE)
        || url.starts_with(DATA_PREFIX)
}

/// Extracts resource paths from HTML using the scraper HTML parser.
///
/// Parses `<script src>`, `<link href>`, and `<img src>` tags to discover
/// external resources.  Also collects inline `<script>` bodies for further
/// AST-level analysis (e.g. IIFE inline-bridge wasm URLs).
///
/// # Arguments
///
/// - `&str` - The HTML content to parse.
/// - `&mut Vec<String>` - The collection to append discovered resource paths to.
/// - `&mut Vec<String>` - The collection to append inline script contents to.
fn extract_resources_with_scraper(
    html: &str,
    paths: &mut Vec<String>,
    inline_scripts: &mut Vec<String>,
) {
    let selectors: [Selector; 4] = match RESOURCE_SELECTORS.get_or_init(init_resource_selectors) {
        Ok(s) => s.clone(),
        Err(error) => {
            euv_log!("[EUV] failed to compile resource selectors: {}", error);
            return;
        }
    };
    let document: Html = Html::parse_document(html);
    let script_selector: &Selector = &selectors[0];
    let link_selector: &Selector = &selectors[1];
    let img_selector: &Selector = &selectors[2];
    let inline_script_selector: &Selector = &selectors[3];
    for element in document.select(script_selector) {
        if let Some(src) = element.value().attr("src")
            && !is_external_resource_url(src)
            && !paths.contains(&src.to_string())
        {
            paths.push(src.to_string());
        }
    }
    for element in document.select(link_selector) {
        if let Some(href) = element.value().attr("href")
            && !is_external_resource_url(href)
            && !paths.contains(&href.to_string())
        {
            paths.push(href.to_string());
        }
    }
    for element in document.select(img_selector) {
        if let Some(src) = element.value().attr("src")
            && !is_external_resource_url(src)
            && !paths.contains(&src.to_string())
        {
            paths.push(src.to_string());
        }
    }
    for element in document.select(inline_script_selector) {
        let script_content: String = element.text().collect::<String>();
        if !script_content.trim().is_empty() {
            inline_scripts.push(script_content);
        }
    }
}

/// Recursively walks an SWC AST expression to find string literals that look
/// like resource paths (`.wasm`, `.js`, `.css`, `.png`, etc.).
///
/// # Arguments
///
/// - `&Expr` - The expression to walk.
/// - `&mut Vec<String>` - The collection to append discovered resource paths to.
fn extract_resources_from_expr(expr: &Expr, results: &mut Vec<String>) {
    match expr {
        Expr::Lit(Lit::Str(str_lit)) => {
            let value: String = str_lit.value.to_string_lossy().into_owned();
            if is_resource_path(&value) && !results.contains(&value) {
                results.push(value);
            }
        }
        Expr::Call(call) => {
            for arg in &call.args {
                extract_resources_from_expr(&arg.expr, results);
            }
            if let Callee::Expr(callee_expr) = &call.callee {
                extract_resources_from_expr(callee_expr, results);
            }
        }
        Expr::New(new_expr) => {
            if let Some(args) = &new_expr.args {
                for arg in args {
                    extract_resources_from_expr(&arg.expr, results);
                }
            }
            extract_resources_from_expr(&new_expr.callee, results);
        }
        Expr::Member(member) => {
            extract_resources_from_expr(&member.obj, results);
            if let MemberProp::Computed(computed) = &member.prop {
                extract_resources_from_expr(&computed.expr, results);
            }
        }
        Expr::Bin(bin) => {
            extract_resources_from_expr(&bin.left, results);
            extract_resources_from_expr(&bin.right, results);
        }
        Expr::Unary(unary) => {
            extract_resources_from_expr(&unary.arg, results);
        }
        Expr::Paren(paren) => {
            extract_resources_from_expr(&paren.expr, results);
        }
        Expr::Tpl(tpl) => {
            for expr in &tpl.exprs {
                extract_resources_from_expr(expr, results);
            }
        }
        Expr::Cond(cond) => {
            extract_resources_from_expr(&cond.test, results);
            extract_resources_from_expr(&cond.cons, results);
            extract_resources_from_expr(&cond.alt, results);
        }
        Expr::Seq(seq) => {
            for expr in &seq.exprs {
                extract_resources_from_expr(expr, results);
            }
        }
        Expr::Array(arr) => {
            for elem in arr.elems.iter().flatten() {
                extract_resources_from_expr(&elem.expr, results);
            }
        }
        Expr::Object(obj) => {
            for prop in &obj.props {
                match prop {
                    PropOrSpread::Prop(prop) => match &**prop {
                        Prop::KeyValue(kv) => {
                            extract_resources_from_expr(&kv.value, results);
                        }
                        Prop::Assign(assign) => {
                            extract_resources_from_expr(&assign.value, results);
                        }
                        _ => {}
                    },
                    PropOrSpread::Spread(spread) => {
                        extract_resources_from_expr(&spread.expr, results);
                    }
                }
            }
        }
        Expr::Arrow(arrow) => match &*arrow.body {
            ArrowFunctionBody::FunctionBody(block) => {
                for stmt in &block.stmts {
                    extract_resources_from_stmt(stmt, results);
                }
            }
            ArrowFunctionBody::Expr(expr) => {
                extract_resources_from_expr(expr, results);
            }
        },
        Expr::Fn(fn_expr) => {
            if let Some(body) = &fn_expr.function.body {
                for stmt in &body.stmts {
                    extract_resources_from_stmt(stmt, results);
                }
            }
        }
        Expr::Await(await_expr) => {
            extract_resources_from_expr(&await_expr.arg, results);
        }
        Expr::Yield(yield_expr) => {
            if let Some(arg) = &yield_expr.arg {
                extract_resources_from_expr(arg, results);
            }
        }
        Expr::Assign(assign) => {
            extract_resources_from_expr(&assign.right, results);
        }
        Expr::Update(update) => {
            extract_resources_from_expr(&update.arg, results);
        }
        _ => {}
    }
}

/// Recursively walks an SWC AST statement to find string literals that look
/// like resource paths.
///
/// # Arguments
///
/// - `&Stmt` - The statement to walk.
/// - `&mut Vec<String>` - The collection to append discovered resource paths to.
fn extract_resources_from_stmt(stmt: &Stmt, results: &mut Vec<String>) {
    match stmt {
        Stmt::Expr(expr_stmt) => {
            extract_resources_from_expr(&expr_stmt.expr, results);
        }
        Stmt::Decl(decl) => extract_resources_from_decl(decl, results),
        Stmt::Block(block) => {
            for stmt in &block.stmts {
                extract_resources_from_stmt(stmt, results);
            }
        }
        Stmt::If(if_stmt) => {
            extract_resources_from_expr(&if_stmt.test, results);
            extract_resources_from_stmt(&if_stmt.cons, results);
            if let Some(alt) = &if_stmt.alt {
                extract_resources_from_stmt(alt, results);
            }
        }
        Stmt::For(for_stmt) => {
            if let Some(init) = &for_stmt.init {
                match init {
                    VarDeclOrExpr::VarDecl(var_decl) => {
                        for declarator in &var_decl.decls {
                            if let Some(init) = &declarator.init {
                                extract_resources_from_expr(init, results);
                            }
                        }
                    }
                    VarDeclOrExpr::Expr(expr) => {
                        extract_resources_from_expr(expr, results);
                    }
                }
            }
            if let Some(test) = &for_stmt.test {
                extract_resources_from_expr(test, results);
            }
            if let Some(update) = &for_stmt.update {
                extract_resources_from_expr(update, results);
            }
            extract_resources_from_stmt(&for_stmt.body, results);
        }
        Stmt::While(while_stmt) => {
            extract_resources_from_expr(&while_stmt.test, results);
            extract_resources_from_stmt(&while_stmt.body, results);
        }
        Stmt::DoWhile(do_while) => {
            extract_resources_from_stmt(&do_while.body, results);
            extract_resources_from_expr(&do_while.test, results);
        }
        Stmt::Return(return_stmt) => {
            if let Some(arg) = &return_stmt.arg {
                extract_resources_from_expr(arg, results);
            }
        }
        Stmt::Throw(throw_stmt) => {
            extract_resources_from_expr(&throw_stmt.arg, results);
        }
        Stmt::Try(try_stmt) => {
            for stmt in &try_stmt.block.stmts {
                extract_resources_from_stmt(stmt, results);
            }
            if let Some(handler) = &try_stmt.handler {
                for stmt in &handler.body.stmts {
                    extract_resources_from_stmt(stmt, results);
                }
            }
            if let Some(finalizer) = &try_stmt.finalizer {
                for stmt in &finalizer.stmts {
                    extract_resources_from_stmt(stmt, results);
                }
            }
        }
        Stmt::Switch(switch_stmt) => {
            extract_resources_from_expr(&switch_stmt.discriminant, results);
            for case in &switch_stmt.cases {
                if let Some(test) = &case.test {
                    extract_resources_from_expr(test, results);
                }
                for stmt in &case.cons {
                    extract_resources_from_stmt(stmt, results);
                }
            }
        }
        Stmt::Labeled(labeled) => {
            extract_resources_from_stmt(&labeled.body, results);
        }
        _ => {}
    }
}

/// Collects resources from variable initializers and function declarations.
///
/// # Arguments
/// - `&Decl` - The declaration whose executable body should be visited.
/// - `&mut Vec<String>` - Destination for discovered resource references.
///
/// # Returns
/// - `()` - Completes after visiting the declaration.
fn extract_resources_from_decl(decl: &Decl, results: &mut Vec<String>) {
    match decl {
        Decl::Var(var_decl) => {
            for declarator in &var_decl.decls {
                if let Some(init) = &declarator.init {
                    extract_resources_from_expr(init, results);
                }
            }
        }
        Decl::Fn(function) => {
            if let Some(body) = &function.function.body {
                for stmt in &body.stmts {
                    extract_resources_from_stmt(stmt, results);
                }
            }
        }
        _ => {}
    }
}

/// Parses inline JavaScript with SWC and extracts string literals that look
/// like resource paths.
///
/// Uses `parse_file_as_program` to parse the script content, then recursively
/// walks the AST to find all string literals ending in known resource
/// extensions (see `RESOURCE_EXTENSIONS`).
///
/// # Arguments
///
/// - `&str` - The JavaScript source code to parse.
/// - `&mut Vec<String>` - The collection to append discovered resource paths to.
fn extract_resources_from_js_ast(js: &str, results: &mut Vec<String>) {
    let source_map: SourceMap = SourceMap::default();
    let source_file: Lrc<SourceFile> =
        source_map.new_source_file(FileName::Anon.into(), js.to_string());
    let comments: SingleThreadedComments = SingleThreadedComments::default();
    let mut recovered_errors: Vec<ParserError> = Vec::new();
    let program: Result<Program, _> = parse_file_as_program(
        &source_file,
        Syntax::Es(Default::default()),
        EsVersion::Es2022,
        Some(&comments),
        &mut recovered_errors,
    );
    if let Ok(program) = program {
        match program {
            Program::Module(module) => {
                for item in module.body {
                    match item {
                        ModuleItem::ModuleDecl(module_decl) => match module_decl {
                            ModuleDecl::ExportDecl(export) => {
                                extract_resources_from_decl(&export.decl, results)
                            }
                            ModuleDecl::Import(import) => {
                                let value: String = import.src.value.to_string_lossy().into_owned();
                                if is_resource_path(&value) {
                                    results.push(value);
                                }
                            }
                            ModuleDecl::ExportAll(export) => {
                                let value: String = export.src.value.to_string_lossy().into_owned();
                                if is_resource_path(&value) {
                                    results.push(value);
                                }
                            }
                            ModuleDecl::ExportNamed(export) => {
                                if let Some(source) = export.src {
                                    let value: String = source.value.to_string_lossy().into_owned();
                                    if is_resource_path(&value) {
                                        results.push(value);
                                    }
                                }
                            }
                            _ => {}
                        },
                        ModuleItem::Stmt(stmt) => {
                            extract_resources_from_stmt(&stmt, results);
                        }
                    }
                }
            }
            Program::Script(script) => {
                for stmt in script.body {
                    extract_resources_from_stmt(&stmt, results);
                }
            }
        }
    }
}

/// Checks whether a string value looks like a local resource path that should
/// be cached.
///
/// Accepts paths ending in common web resource extensions (see
/// `RESOURCE_EXTENSIONS`) that do not contain an absolute URL authority
/// (`SCHEME_SEPARATOR`) and are not protocol-relative or inline-data URIs.
///
/// # Arguments
///
/// - `&str` - The string value to evaluate.
///
/// # Returns
///
/// - `bool` - `true` if the value looks like a local resource path.
fn is_resource_path(value: &str) -> bool {
    let value: &str = value.split(['?', '#']).next().unwrap_or_default();
    if value.contains(SCHEME_SEPARATOR)
        || value.starts_with(DATA_PREFIX)
        || value.starts_with(SCHEME_RELATIVE)
    {
        return false;
    }
    RESOURCE_EXTENSIONS
        .iter()
        .any(|suffix: &&str| value.ends_with(*suffix))
}

/// Returns the MIME type string for a given file path based on its extension.
///
/// Uses `rfind` to locate the last dot in the path and performs case-insensitive
/// comparison on the extension only, avoiding a full `to_lowercase()` heap allocation.
///
/// # Arguments
///
/// - `&str` - The file path or extension to look up.
///
/// # Returns
///
/// - `&'static str` - The corresponding MIME type string, or `"application/octet-stream"` as fallback.
pub(crate) fn mime_for(path: &str) -> &'static str {
    let extension: &str = match path.rfind('.') {
        Some(pos) => &path[pos..],
        None => "",
    };
    if extension.eq_ignore_ascii_case(HTML_EXTENSION) || extension.eq_ignore_ascii_case(".htm") {
        "text/html"
    } else if extension.eq_ignore_ascii_case(JS_EXTENSION)
        || extension.eq_ignore_ascii_case(MJS_EXTENSION)
    {
        "application/javascript"
    } else if extension.eq_ignore_ascii_case(WASM_EXTENSION) {
        "application/wasm"
    } else if extension.eq_ignore_ascii_case(".css") {
        "text/css"
    } else if extension.eq_ignore_ascii_case(".json") {
        "application/json"
    } else if extension.eq_ignore_ascii_case(".png") {
        "image/png"
    } else if extension.eq_ignore_ascii_case(".jpg") || extension.eq_ignore_ascii_case(".jpeg") {
        "image/jpeg"
    } else if extension.eq_ignore_ascii_case(".gif") {
        "image/gif"
    } else if extension.eq_ignore_ascii_case(".svg") {
        "image/svg+xml"
    } else if extension.eq_ignore_ascii_case(".ico") {
        "image/x-icon"
    } else if extension.eq_ignore_ascii_case(".woff") {
        "font/woff"
    } else if extension.eq_ignore_ascii_case(".woff2") {
        "font/woff2"
    } else if extension.eq_ignore_ascii_case(".ttf") {
        "font/ttf"
    } else if extension.eq_ignore_ascii_case(".otf") {
        "font/otf"
    } else if extension.eq_ignore_ascii_case(".webp") {
        "image/webp"
    } else {
        "application/octet-stream"
    }
}

/// Creates a valid server error response when response construction fails.
///
/// # Arguments
/// - `E` - The displayable response construction error.
///
/// # Returns
/// - `response::Response<Cow<'static, [u8]>>` - A response with status 500.
fn scheme_response_error<E>(error: E) -> response::Response<Cow<'static, [u8]>>
where
    E: fmt::Display,
{
    euv_log!("[EUV] failed to build scheme response: {error}");
    let mut response: response::Response<Cow<'static, [u8]>> =
        response::Response::new(Cow::Owned(Vec::new()));
    *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
    response
}

/// Handles the custom `euv://` URI scheme protocol for serving cached resources.
///
/// Reads the requested file from the active cache directory and injects
/// the reload listener script and debug panel (in debug mode) into index.html.
/// Uses exponential backoff when waiting for cache readiness.
///
/// # Arguments
///
/// - `&AppHandle` - The Tauri application handle.
/// - `Request<Vec<u8>>` - The incoming HTTP request.
///
/// # Returns
///
/// - `response::Response<Cow<'static, [u8]>>` - The HTTP response with the file content or an error status.
pub(crate) async fn handle_euv_scheme(
    app_handle: &AppHandle,
    request: Request<Vec<u8>>,
) -> response::Response<Cow<'static, [u8]>> {
    let path: &str = request.uri().path();
    let path_trimmed: &str = path.trim_start_matches('/');
    let path_decoded: String = percent_encoding::percent_decode_str(path_trimmed)
        .decode_utf8_lossy()
        .into_owned();
    let is_index: bool = path_decoded.is_empty() || path_decoded == INDEX_FILE_NAME;
    let is_document: bool = is_index || path_decoded.ends_with(HTML_EXTENSION);
    let session: State<'_, CacheSession> = app_handle.state::<CacheSession>();
    let active_dir: Arc<Path> =
        match tokio::time::timeout(Duration::from_secs(2), session.wait()).await {
            Ok(Ok(directory)) => directory,
            _ => {
                return http::Response::builder()
                    .status(503)
                    .body(Cow::Owned(CACHE_NOT_READY_BODY.to_vec()))
                    .unwrap_or_else(scheme_response_error);
            }
        };
    let file_path: PathBuf = if is_index {
        active_dir.join(INDEX_FILE_NAME)
    } else {
        active_dir.join(&path_decoded)
    };
    #[cfg(debug_assertions)]
    {
        let serve_path: String = file_path.to_string_lossy().into_owned();
        euv_log!("[EUV] serving: {}", serve_path);
    }
    match read(&file_path).await {
        Ok(data) => {
            let mime: &str = mime_for(&path_decoded);
            let mut builder: response::Builder = http::Response::builder()
                .status(200)
                .header(CONTENT_TYPE_HEADER, mime)
                .header("Cache-Control", "no-store, no-cache, must-revalidate")
                .header("Pragma", "no-cache")
                .header("Access-Control-Allow-Origin", "*");
            if path_decoded.ends_with(WASM_EXTENSION) {
                builder = builder
                    .header("Cross-Origin-Embedder-Policy", "require-corp")
                    .header("Cross-Origin-Opener-Policy", "same-origin");
            }
            let body: Vec<u8> = if is_document {
                let html: String = local_document(&String::from_utf8_lossy(&data));
                let extra_scripts: String = RELOAD_LISTENER_SCRIPT.to_string();
                #[cfg(debug_assertions)]
                let extra_scripts: String = {
                    let source: String = String::from(SOURCE_ACTIVE);
                    let dir_path: String = active_dir.to_string_lossy().into_owned();
                    let debug_script: String = DEBUG_PANEL_SCRIPT
                        .replace("{{SOURCE}}", &source)
                        .replace("{{PATH}}", &dir_path)
                        .to_string();
                    format!("{extra_scripts}{debug_script}")
                };
                let injected: String = if let Some(pos) = html.find("</head>") {
                    format!("{}{}{}", &html[..pos], extra_scripts, &html[pos..])
                } else if let Some(pos) = html.find("<body") {
                    format!("{}{}{}", &html[..pos], extra_scripts, &html[pos..])
                } else {
                    format!("{}{}", extra_scripts, html)
                };
                injected.into_bytes()
            } else {
                data
            };
            builder
                .body(Cow::Owned(body))
                .unwrap_or_else(scheme_response_error)
        }
        Err(_) => http::Response::builder()
            .status(404)
            .header(CONTENT_TYPE_HEADER, "text/plain")
            .body(Cow::Owned(RESOURCE_NOT_FOUND_BODY.to_vec()))
            .unwrap_or_else(scheme_response_error),
    }
}

/// Tauri command that triggers an immediate cache update from the remote source.
///
/// Fetches the latest resources and switches the active version pointer so the
/// new content takes effect on the **next** application launch.  The command
/// returns only after the update has completed (or failed).
///
/// This is the client-initiated counterpart to the former automatic
/// `background_update`: instead of pulling on every native startup, the
/// frontend decides *when* to pull (e.g. via `window.bridge.core.invoke("update_cache")`).
///
/// # Arguments
///
/// - `AppHandle` - The Tauri application handle.
///
/// # Returns
///
/// - `Result<CacheUpdateResult, String>` - On the happy path, a `CacheUpdateResult`
///   with `success: true` and the freshly minted `new_version_name`. Fetch /
///   write failures are also surfaced through `Ok(...)` with `success: false`
///   and a descriptive `message` so the webview can retry without losing the
///   structured payload. Only Tauri-side panics / programmer errors escape as
///   `Err(...)`.
#[tauri::command]
pub(crate) async fn update_cache(app_handle: AppHandle) -> Result<CacheUpdateResult, String> {
    let cache_root: PathBuf =
        get_cache_root(&app_handle).map_err(|error: CacheError| error.to_string())?;
    let session: State<'_, CacheSession> = app_handle.state::<CacheSession>();
    let _update: MutexGuard<'_, ()> = session.get_update().lock().await;
    match fetch_full_snapshot(&cache_root, &session).await {
        Ok(snapshot) => {
            session.publish(cache_root.join(&snapshot));
            euv_log!(
                "[EUV] update_cache done (effective next launch): {}",
                snapshot
            );
            Ok(CacheUpdateResult {
                result: CacheUpdateStatus::Success,
                data: snapshot,
                message: "cache updated successfully".to_string(),
            })
        }
        Err(error) => {
            let error_message: String = error.to_string();
            euv_log!("[EUV] update_cache failed: {}", error_message);
            Ok(CacheUpdateResult {
                result: CacheUpdateStatus::Failed,
                data: String::new(),
                message: error_message,
            })
        }
    }
}

/// Tauri command that returns cached page metadata.
///
/// # Arguments
///
/// - `AppHandle` - The Tauri application handle.
///
/// # Returns
///
/// - `Result<CachedPage, String>` - The cached page info on success, or an error message string.
#[tauri::command]
pub(crate) async fn load_cached_resource(app_handle: AppHandle) -> Result<CachedPage, String> {
    let directory: Option<Arc<Path>> = app_handle.state::<CacheSession>().directory();
    let from_cache: bool = directory.is_some();
    let source: String = if from_cache {
        SOURCE_ACTIVE
    } else {
        SOURCE_NONE
    }
    .to_string();
    let cache_path: String = directory
        .as_deref()
        .map(|path: &Path| path.to_string_lossy().into_owned())
        .unwrap_or_default();
    Ok(CachedPage {
        from_cache,
        remote_url: REMOTE_URL.to_string(),
        source,
        cache_path,
    })
}

/// Waits for a complete session snapshot without repeated filesystem lookups.
///
/// # Arguments
/// - `AppHandle` - The application whose cache initialization is pending.
///
/// # Returns
/// - `Result<CachedPage, String>` - Metadata for the first complete snapshot.
#[tauri::command]
pub(crate) async fn wait_for_cached_resource(app_handle: AppHandle) -> Result<CachedPage, String> {
    let directory: Arc<Path> = app_handle.state::<CacheSession>().wait().await?;
    Ok(CachedPage {
        from_cache: true,
        remote_url: REMOTE_URL.to_string(),
        source: SOURCE_ACTIVE.to_string(),
        cache_path: directory.to_string_lossy().to_string(),
    })
}
