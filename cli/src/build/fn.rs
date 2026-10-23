use super::*;

/// Checks whether `wasm_pack_args` already contains a build mode flag.
///
/// Returns `true` if any of `--dev`, `--release`, or `--profiling`
/// is present in the arguments list.
///
/// # Arguments
///
/// - `&[String]` - The wasm-pack arguments to search.
///
/// # Returns
///
/// - `bool` - Whether a build mode flag is already present.
pub fn has_build_mode_flag(wasm_pack_args: &[String]) -> bool {
    wasm_pack_args
        .iter()
        .any(|arg: &String| arg == DEV_FLAG || arg == RELEASE_FLAG || arg == PROFILING_FLAG)
}

/// Filters out euv-specific arguments from the wasm-pack arguments.
///
/// First locates the genuine passthrough arguments by taking everything
/// after the last `--` separator (or the full list if no `--` is present).
/// Then removes all known euv-specific flags and their values so that
/// only wasm-pack-compatible arguments remain.
///
/// # Arguments
///
/// - `&[String]` - The raw wasm-pack arguments to filter.
///
/// # Returns
///
/// - `Vec<String>` - The filtered arguments safe for wasm-pack.
pub fn filter_euv_args(wasm_pack_args: &[String]) -> Vec<String> {
    let raw_args: &[String] = if let Some(position) = wasm_pack_args
        .iter()
        .rposition(|arg: &String| arg == DOUBLE_DASH)
    {
        &wasm_pack_args[position + 1..]
    } else {
        wasm_pack_args
    };
    let mut filtered: Vec<String> = Vec::new();
    let mut skip_next: bool = false;
    for arg in raw_args {
        if skip_next {
            skip_next = false;
            continue;
        }
        // Compare the flag NAME, not the whole argument: clap accepts both
        // `--port 3000` and `--port=3000`, and only the first form ever
        // matched `EUV_ARGS` exactly. Without the split, `--port=3000` was
        // forwarded to wasm-pack even though `reconcile_args` had already
        // consumed its value — the same string both used and leaked.
        let name: &str = arg.split('=').next().unwrap_or(arg.as_str());
        if EUV_ARGS.contains(&name) {
            if arg.contains('=') || name == NO_GITIGNORE_ARG {
                // `--no-gitignore` is a bare boolean: there is no value to
                // skip, so treating it like `--port 3000` ate whatever
                // argument followed it.
                continue;
            }
            skip_next = true;
            continue;
        }
        filtered.push(arg.clone());
    }
    filtered
}

/// Reconciles euv-specific arguments that may have been collected into
/// `wasm_pack_args` (e.g. when placed after `--`) back into the
/// corresponding `ModeArgs` fields.
///
/// Because clap's `trailing_var_arg` collects all unrecognized arguments
/// into `wasm_pack_args`, any euv flag placed after `--` is not parsed
/// by clap into its dedicated field. This function scans `wasm_pack_args`
/// for known euv flags and overwrites the `ModeArgs` fields so that the
/// rest of the codebase can rely on the typed accessors regardless of
/// argument order.
///
/// # Arguments
///
/// - `&mut ModeArgs` - The CLI arguments to reconcile in-place.
pub fn reconcile_args(args: &mut ModeArgs) {
    let wasm_pack_args: Vec<String> = args.get_wasm_pack_args().clone();
    let mut crate_path: Option<PathBuf> = None;
    let mut port: Option<u16> = None;
    let mut www_dir: Option<String> = None;
    let mut index_html: Option<Option<PathBuf>> = None;
    let mut no_gitignore: Option<bool> = None;
    let mut dev: Option<bool> = None;
    let mut release: Option<bool> = None;
    let mut profiling: Option<bool> = None;
    let mut iter: Iter<String> = wasm_pack_args.iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            CRATE_PATH_ARG | CRATE_PATH_ARG_SHORT => {
                if let Some(value) = iter.next() {
                    crate_path = Some(PathBuf::from(value));
                }
            }
            PORT_ARG | PORT_ARG_SHORT => {
                if let Some(value) = iter.next()
                    && let Ok(parsed_port) = value.parse::<u16>()
                {
                    port = Some(parsed_port);
                }
            }
            WWW_DIR_ARG => {
                if let Some(value) = iter.next() {
                    www_dir = Some(value.clone());
                }
            }
            INDEX_HTML_ARG => {
                if let Some(value) = iter.next() {
                    index_html = Some(Some(PathBuf::from(value)));
                }
            }
            NO_GITIGNORE_ARG => {
                no_gitignore = Some(true);
            }
            DEV_FLAG => {
                dev = Some(true);
            }
            RELEASE_FLAG => {
                release = Some(true);
            }
            PROFILING_FLAG => {
                profiling = Some(true);
            }
            other => {
                if let Some(value) = other.strip_prefix(&format!("{CRATE_PATH_ARG}=")) {
                    crate_path = Some(PathBuf::from(value));
                } else if let Some(value) = other.strip_prefix(&format!("{PORT_ARG}=")) {
                    if let Ok(parsed_port) = value.parse::<u16>() {
                        port = Some(parsed_port);
                    }
                } else if let Some(value) = other.strip_prefix(&format!("{WWW_DIR_ARG}=")) {
                    www_dir = Some(value.to_string());
                } else if let Some(value) = other.strip_prefix(&format!("{INDEX_HTML_ARG}=")) {
                    index_html = Some(Some(PathBuf::from(value)));
                }
            }
        }
    }
    if let Some(value) = crate_path {
        args.set_crate_path(value);
    }
    if let Some(value) = port {
        args.set_port(value);
    }
    if let Some(value) = www_dir {
        args.set_www_dir(value);
    }
    if let Some(value) = index_html {
        args.set_index_html(value);
    }
    if let Some(value) = no_gitignore {
        args.set_no_gitignore(value);
    }
    if let Some(value) = dev {
        args.set_dev(value);
    }
    if let Some(value) = release {
        args.set_release(value);
    }
    if let Some(value) = profiling {
        args.set_profiling(value);
    }
}

/// Resolves the build mode from CLI arguments.
///
/// First checks the explicit `--dev`, `--release`, and `--profiling` flags on `ModeArgs`.
/// If none of those are set, inspects `wasm_pack_args` for any build mode flag
/// that may have been forwarded by the user.
/// Defaults to `BuildMode::Dev` if no build mode flag is found anywhere.
///
/// # Arguments
///
/// - `&ModeArgs` - The CLI arguments containing the build mode flags and wasm_pack_args.
///
/// # Returns
///
/// - `BuildMode` - The resolved build mode.
pub fn resolve_build_mode(args: &ModeArgs) -> BuildMode {
    if args.get_profiling() {
        BuildMode::Profiling
    } else if args.get_release() {
        BuildMode::Release
    } else if args.get_dev() {
        BuildMode::Dev
    } else if args
        .get_wasm_pack_args()
        .iter()
        .any(|arg: &String| arg == PROFILING_FLAG)
    {
        BuildMode::Profiling
    } else if args
        .get_wasm_pack_args()
        .iter()
        .any(|arg: &String| arg == RELEASE_FLAG)
    {
        BuildMode::Release
    } else {
        BuildMode::Dev
    }
}

/// Converts a `BuildMode` to the corresponding wasm-pack flag string.
///
/// # Arguments
///
/// - `BuildMode` - The build mode to convert.
///
/// # Returns
///
/// - `&'static str` - The wasm-pack command-line flag.
pub fn build_mode_to_flag(build_mode: BuildMode) -> &'static str {
    match build_mode {
        BuildMode::Dev => DEV_FLAG,
        BuildMode::Release => RELEASE_FLAG,
        BuildMode::Profiling => PROFILING_FLAG,
    }
}

/// Builds a `Gitignore` matcher from the `.gitignore` file at the given root path.
///
/// # Arguments
///
/// - `&PathBuf` - The root directory where `.gitignore` is located.
///
/// # Returns
///
/// - `Gitignore` - The compiled gitignore matcher.
async fn build_gitignore(root: &PathBuf) -> Gitignore {
    let gitignore_path: PathBuf = root.join(GITIGNORE_FILE_NAME);
    let mut builder: GitignoreBuilder = GitignoreBuilder::new(root);
    let gitignore_exists: bool = metadata(&gitignore_path).await.is_ok();
    if gitignore_exists && let Some(error) = builder.add(&gitignore_path) {
        log::warn!("Failed to load .gitignore: {error}");
    }
    match builder.build() {
        Ok(gitignore) => {
            if gitignore_exists {
                log::info!("Loaded .gitignore to filter file change events");
            }
            gitignore
        }
        Err(error) => {
            log::warn!("Failed to build gitignore matcher: {error}");
            GitignoreBuilder::new(root)
                .build()
                .unwrap_or_else(|_error: ignore::Error| Gitignore::empty())
        }
    }
}

/// Extracts the value of `--out-name` from the wasm-pack arguments.
///
/// Returns `None` if `--out-name` is not specified.
///
/// # Arguments
///
/// - `&[String]` - The wasm-pack arguments to search.
///
/// # Returns
///
/// - `Option<String>` - The value of `--out-name` if found.
fn extract_out_name(wasm_pack_args: &[String]) -> Option<String> {
    let mut iter: Iter<'_, String> = wasm_pack_args.iter();
    while let Some(arg) = iter.next() {
        if arg == OUT_NAME_ARG {
            return iter.next().cloned();
        }
        if let Some(value) = arg.strip_prefix(&format!("{OUT_NAME_ARG}=")) {
            return Some(value.to_string());
        }
    }
    None
}

/// Extracts the value of `--out-dir` from the wasm-pack arguments.
///
/// Returns `None` if `--out-dir` is not specified.
///
/// # Arguments
///
/// - `&[String]` - The wasm-pack arguments to search.
///
/// # Returns
///
/// - `Option<String>` - The value of `--out-dir` if found.
fn extract_out_dir(wasm_pack_args: &[String]) -> Option<String> {
    let mut iter: Iter<'_, String> = wasm_pack_args.iter();
    while let Some(arg) = iter.next() {
        if arg == OUT_DIR_ARG {
            return iter.next().cloned();
        }
        if let Some(value) = arg.strip_prefix(&format!("{OUT_DIR_ARG}=")) {
            return Some(value.to_string());
        }
    }
    None
}

/// Resolves the output JS filename for HTML generation.
///
/// Uses `--out-name` from wasm-pack args if specified,
/// otherwise reads the crate name from `Cargo.toml` `[package] name` field
/// and replaces hyphens with underscores (matching wasm-pack behavior).
/// Appends `.js` extension to form the complete JS filename.
///
/// # Arguments
///
/// - `&ModeArgs` - The CLI arguments containing crate_path and wasm_pack_args.
///
/// # Returns
///
/// - `String` - The resolved JS filename with `.js` extension (e.g. `euv_example.js`).
pub fn resolve_out_name(args: &ModeArgs) -> String {
    let name: String = if let Some(out_name) = extract_out_name(args.get_wasm_pack_args()) {
        out_name.replace(STR_HYPHEN, STR_UNDERSCORE)
    } else {
        let cargo_toml_path: PathBuf = args.get_crate_path().join(CARGO_TOML_FILE_NAME);
        read_crate_name_from_toml(&cargo_toml_path)
            .unwrap_or_else(|| {
                args.get_crate_path()
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string()
            })
            .replace(STR_HYPHEN, STR_UNDERSCORE)
    };
    format!("{name}{JS_EXTENSION}")
}

/// Reads the `name` field from a Cargo.toml file.
///
/// Parses the file line-by-line looking for `name = "..."` within the `[package]` section.
///
/// # Arguments
///
/// - `&Path` - The path to the Cargo.toml file.
///
/// # Returns
///
/// - `Option<String>` - The crate name if found.
fn read_crate_name_from_toml(path: &Path) -> Option<String> {
    let content: String = sync_read_to_string(path).ok()?;
    let mut in_package: bool = false;
    for line in content.lines() {
        let trimmed: &str = line.trim();
        if trimmed.starts_with('[') {
            in_package = trimmed == "[package]";
            continue;
        }
        if in_package
            && trimmed.starts_with("name")
            && let Some(value) = trimmed.strip_prefix("name")
        {
            let value: &str = value.trim().strip_prefix('=')?.trim();
            let value: &str = value.strip_prefix('"')?.strip_suffix('"')?;
            return Some(value.to_string());
        }
    }
    None
}

/// Computes the relative path from a base directory to a target directory.
///
/// Compares the component sequences of both paths to find the common prefix,
/// then emits `..` for each remaining base component followed by the remaining
/// target components.
///
/// # Arguments
///
/// - `&Path` - The base directory path.
///
/// # Returns
///
/// - `PathBuf` - The relative path from base to target.
fn compute_relative_path(base: &Path, target: &Path) -> PathBuf {
    let base_components: Vec<Component> = base.components().collect();
    let target_components: Vec<Component> = target.components().collect();
    let common_len: usize = base_components
        .iter()
        .zip(target_components.iter())
        .take_while(
            |(base_component, target_component): &(&Component, &Component)| {
                base_component == target_component
            },
        )
        .count();
    let mut result: PathBuf = PathBuf::new();
    for _ in &base_components[common_len..] {
        result.push("..");
    }
    for component in &target_components[common_len..] {
        if let Component::Normal(os_str) = component {
            result.push(os_str);
        }
    }
    result
}

/// Resolves the serving root directory for the development server.
///
/// When the output directory is inside the www directory, returns the resolved www directory.
/// When the output directory is outside the www directory, returns the parent of the output directory
/// so that `index.html` and WASM artifacts are co-located under the same serving root.
///
/// # Arguments
///
/// - `&ModeArgs` - The CLI arguments containing crate_path, www_dir, wasm_pack_args.
///
/// # Returns
///
/// - `PathBuf` - The resolved serving root directory.
pub async fn resolve_serving_root(args: &ModeArgs) -> PathBuf {
    let www_absolute: PathBuf = args.get_crate_path().join(args.get_www_dir());
    let out_dir_absolute: PathBuf = resolve_out_dir(args);
    if out_dir_absolute.strip_prefix(&www_absolute).is_ok() {
        resolve_www_dir(&www_absolute).await
    } else {
        out_dir_absolute
            .parent()
            .map(|p: &Path| p.to_path_buf())
            .unwrap_or_else(|| www_absolute)
    }
}

/// Resolves the serving route prefix relative to the crate path.
///
/// Returns the forward-slash-separated path of the serving root relative to the crate path.
/// Used for server route registration and URL display.
///
/// # Arguments
///
/// - `&ModeArgs` - The CLI arguments containing crate_path, www_dir, wasm_pack_args.
///
/// # Returns
///
/// - `String` - The serving route prefix (e.g. `www` or `wwws`).
pub fn resolve_serving_route_prefix(args: &ModeArgs) -> String {
    let www_absolute: PathBuf = args.get_crate_path().join(args.get_www_dir());
    let out_dir_absolute: PathBuf = resolve_out_dir(args);
    let serving_root: PathBuf = if out_dir_absolute.strip_prefix(&www_absolute).is_ok() {
        www_absolute
    } else {
        out_dir_absolute
            .parent()
            .map(|p: &Path| p.to_path_buf())
            .unwrap_or_else(|| www_absolute)
    };
    serving_root
        .strip_prefix(args.get_crate_path())
        .map(|rel: &Path| {
            rel.to_string_lossy()
                .replace(CHAR_SLASH_BACK, STR_SLASH_FORWARD)
        })
        .unwrap_or_else(|_| {
            serving_root
                .file_name()
                .and_then(|name: &ffi::OsStr| name.to_str())
                .unwrap_or_default()
                .to_string()
        })
}

/// Resolves the JS import path for HTML generation.
///
/// Computes the relative path from the serving root to the output directory,
/// then appends the JS filename (from `resolve_out_name`, which includes `.js`)
/// to form the full import path (e.g. `./pkg/euv.js` or `./pksg/cc.js`).
///
/// # Arguments
///
/// - `&ModeArgs` - The CLI arguments containing crate_path, www_dir, wasm_pack_args.
///
/// # Returns
///
/// - `String` - The resolved JS import path relative to the serving root.
pub fn resolve_import_path(args: &ModeArgs) -> String {
    let out_name: String = resolve_out_name(args);
    let www_absolute: PathBuf = args.get_crate_path().join(args.get_www_dir());
    let out_dir_absolute: PathBuf = resolve_out_dir(args);
    let serving_root: PathBuf = if out_dir_absolute.strip_prefix(&www_absolute).is_ok() {
        www_absolute
    } else {
        out_dir_absolute
            .parent()
            .map(|p: &Path| p.to_path_buf())
            .unwrap_or_else(|| www_absolute)
    };
    let relative: PathBuf = compute_relative_path(&serving_root, &out_dir_absolute);
    let mut components: Vec<String> = relative
        .components()
        .filter_map(|component: Component| match component {
            Component::Normal(os_str) => os_str.to_str().map(|text: &str| text.to_string()),
            Component::ParentDir => Some(PARENT_DIR.to_string()),
            _ => None,
        })
        .collect();
    components.push(out_name);
    format!("{RELATIVE_PATH_PREFIX}{}", components.join(PATH_SEPARATOR))
}

/// Resolves the output directory for wasm-pack artifacts.
///
/// Uses `--out-dir` from wasm-pack args if specified,
/// otherwise defaults to `{www_dir}/pkg` so that build artifacts
/// are placed directly inside the www directory served
/// by the development server.
///
/// # Arguments
///
/// - `&ModeArgs` - The CLI arguments containing crate_path, www_dir, and wasm_pack_args.
///
/// # Returns
///
/// - `PathBuf` - The resolved output directory (absolute if crate_path is joined).
pub fn resolve_out_dir(args: &ModeArgs) -> PathBuf {
    let out_dir_path: PathBuf = PathBuf::from(
        extract_out_dir(args.get_wasm_pack_args())
            .unwrap_or_else(|| format!("{}/{PKG_DIR_NAME}", args.get_www_dir())),
    );
    if out_dir_path.is_absolute() {
        out_dir_path
    } else {
        args.get_crate_path().join(&out_dir_path)
    }
}

/// Formats euv macros, builds WASM, writes HTML, then removes generated glue.
///
/// The generated `.js`, `.d.ts`, `.js.map` and `snippets/` files are removed
/// only after the HTML carrying their inlined bootstrap has been written, so a
/// failed build leaves its artifacts on disk for diagnosis rather than deleting
/// the evidence and reporting success.
///
/// # Arguments
///
/// - `&ModeArgs` - The CLI arguments.
///
/// # Returns
///
/// - `Result<(), EuvError>` - Indicates success or failure of the build.
pub async fn run_build_only_pipeline(args: &ModeArgs) -> Result<(), EuvError> {
    let src_path: PathBuf = args.get_crate_path().join(SRC_DIR_NAME);
    if let Err(error) = format_dir(&src_path, FmtMode::Write).await {
        log::warn!("euv fmt error: {error}");
    }
    build_wasm(args).await?;
    log::info!("WASM build completed successfully");
    finalize_html_output(args, false).await?;
    Ok(())
}

/// Writes the page and removes the JavaScript it inlined, in that order.
///
/// Every failure propagates: a build that cannot inline its bridge, cannot
/// write its page, or cannot delete the JavaScript it just inlined reports the
/// failure rather than shipping a page that imports a deleted file.
///
/// # Arguments
///
/// - `&ModeArgs` - The CLI arguments.
/// - `bool` - Whether the page is being generated for the dev server, which
///   injects a `<base href>` tag.
///
/// # Returns
///
/// - `Result<String, EuvError>` - The generated HTML.
pub(crate) async fn finalize_html_output(
    args: &ModeArgs,
    is_dev_server: bool,
) -> Result<String, EuvError> {
    let html_config: HtmlConfig = HtmlConfig::new(
        resolve_serving_root(args).await,
        resolve_import_path(args),
        resolve_build_mode(args) == BuildMode::Release,
        args.try_get_index_html().clone(),
        is_dev_server.then(|| resolve_serving_route_prefix(args)),
    );
    let html: String = generate_html(&html_config).await?;
    let pkg_dir: PathBuf = resolve_out_dir(args);
    let removed: Vec<PathBuf> = remove_generated_js(&pkg_dir).await?;
    if !removed.is_empty() {
        log::info!(
            "Inlined {} generated JavaScript file(s) into {} and removed them from {}",
            removed.len(),
            INDEX_HTML_FILE_NAME,
            pkg_dir.display()
        );
    }
    assert_no_js_artifacts(&pkg_dir).await?;
    Ok(html)
}

/// Fails when the wasm-pack output directory still holds a JavaScript file
/// this build is responsible for.
///
/// Catches the case where a delete silently missed a file: the page would
/// still boot, but the deployed tree would carry the glue the build just
/// claimed to have removed.
///
/// # Arguments
///
/// - `&Path` - The wasm-pack output directory.
///
/// # Returns
///
/// - `Result<(), EuvError>` - `Ok` when no generated JavaScript remains.
async fn assert_no_js_artifacts(pkg_dir: &Path) -> Result<(), EuvError> {
    let mut pending: Vec<PathBuf> = vec![pkg_dir.to_path_buf()];
    while let Some(dir) = pending.pop() {
        let mut entries: ReadDir =
            read_dir(&dir)
                .await
                .map_err(|error: io::Error| EuvError::IoPath {
                    message: String::from("Failed to inspect finalized WASM output"),
                    path: dir.clone(),
                    error,
                })?;
        while let Some(entry) =
            entries
                .next_entry()
                .await
                .map_err(|error: io::Error| EuvError::IoPath {
                    message: String::from("Failed to inspect an entry of finalized WASM output"),
                    path: dir.clone(),
                    error,
                })?
        {
            let path: PathBuf = entry.path();
            let file_type: FileType =
                entry
                    .file_type()
                    .await
                    .map_err(|error: io::Error| EuvError::IoPath {
                        message: String::from("Failed to inspect a finalized WASM output file"),
                        path: path.clone(),
                        error,
                    })?;
            if file_type.is_dir() {
                pending.push(path);
                continue;
            }
            if is_generated_js_artifact(&path) {
                return Err(EuvError::Message(format!(
                    "{} survived the cleanup, so this build would deploy JavaScript; \
                     the page no longer imports it, and it cannot be served",
                    path.display()
                )));
            }
        }
    }
    Ok(())
}

/// Cleans the output directory before a fresh build.
///
/// Removes all files and subdirectories within the output directory
/// so that stale artifacts from previous builds do not remain.
/// The directory itself is preserved (recreated if missing).
///
/// # Arguments
///
/// - `&Path` - The output directory to clean.
pub async fn clean_out_dir(out_dir: &Path) {
    let mut entries: ReadDir = match read_dir(out_dir).await {
        Ok(dir) => dir,
        Err(_) => return,
    };
    while let Ok(Some(entry)) = entries.next_entry().await {
        let path: PathBuf = entry.path();
        if path.is_dir() {
            if let Err(error) = remove_dir_all(&path).await {
                log::warn!("Failed to remove directory '{}': {error}", path.display());
            }
        } else if let Err(error) = remove_file(&path).await {
            log::warn!("Failed to remove file '{}': {error}", path.display());
        }
    }
}

/// Executes a full build pipeline: euv fmt, build wasm, generate HTML, then
/// remove the JavaScript that was inlined into it.
///
/// Only a pipeline whose wasm build, page write and JavaScript cleanup all
/// succeeded broadcasts a reload. A failure at any phase notifies the client
/// of the error and leaves the previous page in memory, so a browser is never
/// pointed at a page whose inlined bootstrap references a deleted file.
///
/// # Arguments
///
/// - `&ModeArgs` - The CLI arguments.
/// - `Option<&broadcast::Sender<ReloadEvent>>` - Optional reload channel for notifying clients.
///
/// # Returns
///
/// - `Result<String, EuvError>` - The generated HTML with reload script injected on success.
pub async fn run_build_pipeline(
    args: &ModeArgs,
    reload_tx: Option<&broadcast::Sender<ReloadEvent>>,
) -> Result<String, EuvError> {
    let src_path: PathBuf = args.get_crate_path().join(SRC_DIR_NAME);
    if let Err(error) = format_dir(&src_path, FmtMode::Write).await {
        log::warn!("euv fmt error: {error}");
    }
    if let Err(error) = build_wasm(args).await {
        log::error!("WASM build failed: {error}");
        notify_reload(reload_tx, Err(error.to_string()));
        return Err(error);
    }
    log::info!("WASM build completed successfully");
    let html: String = match finalize_html_output(args, true).await {
        Ok(html) => html,
        Err(error) => {
            log::error!("Page generation failed: {error}");
            notify_reload(reload_tx, Err(error.to_string()));
            return Err(error);
        }
    };
    notify_reload(reload_tx, Ok(()));
    Ok(html)
}

/// Sends one reload event to the dev server's clients.
///
/// # Arguments
///
/// - `Option<&broadcast::Sender<ReloadEvent>>` - The channel, when the caller has one.
/// - `Result<(), String>` - `Ok` to ask clients to reload, `Err` to tell them
///   why the rebuild failed.
fn notify_reload(reload_tx: Option<&broadcast::Sender<ReloadEvent>>, outcome: Result<(), String>) {
    let Some(sender) = reload_tx else {
        return;
    };
    let event: ReloadEvent = match outcome {
        Ok(()) => ReloadEvent::Reload,
        Err(message) => ReloadEvent::Error(message),
    };
    let _: Result<usize, tokio::sync::broadcast::error::SendError<ReloadEvent>> =
        sender.send(event);
}

/// Watches source files and triggers WASM builds.
///
/// # Arguments
///
/// - `Arc<AppState>` - The shared application state.
///
/// # Returns
///
/// - `Result<(), EuvError>` - Indicates success or failure of the file watcher.
pub(crate) async fn watch_and_build(state: Arc<AppState>) -> Result<(), EuvError> {
    let crate_path: PathBuf = state.get_args().get_crate_path().clone();
    let src_path: PathBuf = crate_path.join(SRC_DIR_NAME);
    let gitignore: Gitignore = build_gitignore(&crate_path).await;
    let (tx, mut rx): (Sender<Event>, Receiver<Event>) = channel(32);
    let mut watcher: RecommendedWatcher = RecommendedWatcher::new(
        move |result: Result<Event, notify::Error>| {
            if let Ok(event) = result {
                let _: Result<(), tokio::sync::mpsc::error::SendError<Event>> =
                    tx.blocking_send(event);
            }
        },
        Config::default(),
    )?;
    watcher.watch(&src_path, RecursiveMode::Recursive)?;
    log::info!("Watching {} for changes...", src_path.display());
    let mut debounce: Interval = interval(Duration::from_millis(500));
    debounce.tick().await;
    while let Some(event) = rx.recv().await {
        let filtered_paths: Vec<String> = event
            .paths
            .iter()
            .filter(|path: &&PathBuf| !gitignore.matched(*path, path.is_dir()).is_ignore())
            .map(|path: &PathBuf| path.display().to_string())
            .collect();
        if filtered_paths.is_empty() {
            continue;
        }
        log::warn!("File change detected: {}", filtered_paths.join(", "));
        debounce.reset();
        sleep(Duration::from_millis(300)).await;
        let mut building: RwLockWriteGuard<bool> = state.get_is_building().write().await;
        if *building {
            continue;
        }
        *building = true;
        drop(building);
        let state_for_build: Arc<AppState> = Arc::clone(&state);
        spawn(async move {
            let args: ModeArgs = state_for_build.get_args().clone();
            let reload_tx: broadcast::Sender<ReloadEvent> = state_for_build.get_reload_tx().clone();
            match run_build_pipeline(&args, Some(&reload_tx)).await {
                Ok(html) => {
                    let mut content: RwLockWriteGuard<String> =
                        state_for_build.get_html_content().write().await;
                    *content = html;
                }
                Err(error) => {
                    log::error!("Build pipeline error: {error}");
                }
            }
            let mut building: RwLockWriteGuard<bool> =
                state_for_build.get_is_building().write().await;
            *building = false;
        });
    }
    Ok(())
}

/// Runs wasm-pack build for the target crate.
///
/// All arguments in `args.wasm_pack_args` are transparently forwarded
/// to `wasm-pack build`. If `--out-dir` is not specified by the user,
/// `--out-dir {www_dir}/pkg` is automatically injected so that build artifacts
/// are placed inside the www directory served by the development server.
///
/// # Arguments
///
/// - `&ModeArgs` - The CLI arguments containing crate_path, www_dir, and wasm_pack_args.
///
/// # Returns
///
/// - `Result<(), EuvError>` - Indicates success or failure of the wasm-pack build.
pub async fn build_wasm(args: &ModeArgs) -> Result<(), EuvError> {
    let build_mode: BuildMode = resolve_build_mode(args);
    let build_mode_flag: &str = build_mode_to_flag(build_mode);
    let filtered_args: Vec<String> = filter_euv_args(args.get_wasm_pack_args());
    let has_existing_build_mode: bool = has_build_mode_flag(&filtered_args);
    let default_out_dir: String = format!("{}/{PKG_DIR_NAME}", args.get_www_dir());
    let mut command: Command = Command::new(WASM_PACK_COMMAND);
    command.arg(WASM_PACK_BUILD_SUBCOMMAND);
    if !has_existing_build_mode {
        command.arg(build_mode_flag);
    }
    command
        .args(&filtered_args)
        .env(RUST_MIN_STACK_ENV, RUST_MIN_STACK_VALUE);
    let has_out_dir: bool = extract_out_dir(&filtered_args).is_some();
    if !has_out_dir {
        command.arg(OUT_DIR_ARG).arg(&default_out_dir);
    }
    let has_target: bool = filtered_args
        .iter()
        .any(|arg: &String| arg == TARGET_ARG || arg.starts_with(&format!("{TARGET_ARG}=")));
    if !has_target {
        command.arg(TARGET_ARG).arg(TARGET_WEB);
    }
    command.current_dir(args.get_crate_path());
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
    let display_args: Vec<String> = (if has_existing_build_mode {
        filtered_args.to_vec()
    } else {
        once(build_mode_flag.to_string())
            .chain(filtered_args.iter().cloned())
            .collect::<Vec<String>>()
    })
    .into_iter()
    .chain(if has_out_dir {
        Vec::new()
    } else {
        vec![OUT_DIR_ARG.to_string(), default_out_dir.clone()]
    })
    .chain(if has_target {
        Vec::new()
    } else {
        vec![TARGET_ARG.to_string(), TARGET_WEB.to_string()]
    })
    .collect();
    let out_dir_absolute: PathBuf = resolve_out_dir(args);
    create_dir_all(&out_dir_absolute)
        .await
        .map_err(|error: io::Error| EuvError::IoPath {
            message: String::from("Failed to create output directory"),
            path: out_dir_absolute.clone(),
            error,
        })?;
    log::info!(
        "Running: {WASM_PACK_COMMAND} {WASM_PACK_BUILD_SUBCOMMAND} {} ...",
        display_args.join(" ")
    );
    let output: Output = command
        .output()
        .await
        .map_err(|error: io::Error| EuvError::Io {
            message: String::from("Failed to execute wasm-pack"),
            error,
        })?;
    let stdout: String = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr: String = String::from_utf8_lossy(&output.stderr).to_string();
    if args.get_no_gitignore() {
        let gitignore_path: PathBuf = out_dir_absolute.join(GITIGNORE_FILE_NAME);
        if gitignore_path.exists()
            && let Err(error) = remove_file(&gitignore_path).await
        {
            log::warn!("Failed to remove '{}': {error}", gitignore_path.display());
        }
    }
    for line in stdout.lines().filter(|line: &&str| !line.is_empty()) {
        log::info!("{line}");
    }
    if output.status.success() {
        for line in stderr.lines().filter(|line: &&str| !line.is_empty()) {
            log::info!("{line}");
        }
    } else {
        for line in stderr.lines().filter(|line: &&str| !line.is_empty()) {
            log::error!("{line}");
        }
        return Err(EuvError::Message(String::from("wasm-pack build failed")));
    }
    Ok(())
}

/// Prints the startup banner and command information.
///
/// # Arguments
///
/// - `Action` - The action to perform (run or build).
pub fn print_banner(action: Action) {
    let version: &str = env!("CARGO_PKG_VERSION");
    if version.is_empty() {
        log::warn!("Failed to parse version from root Cargo.toml");
    } else {
        log::info!("euv v{version}");
    }
    let action_name: &str = match action {
        Action::Run => ACTION_RUN,
        Action::Build => ACTION_BUILD,
    };
    log::info!("Mode: {action_name}");
    log::info!(
        "Use .gitignore to filter file change events; pass --no-gitignore to remove .gitignore from output"
    );
}

/// Enumerates all network interface IP addresses and prints each server URL
/// along with its corresponding QR code to the console.
///
/// Includes both loopback (127.0.0.1) and all private/public IPv4 addresses
/// bound to the host's network interfaces. Each address produces one URL line
/// followed by a Unicode QR code rendered with half-block characters,
/// where every line carries the standard log prefix (timestamp + level).
///
/// # Arguments
///
/// - `&ServerUrlConfig` - The server URL configuration.
pub(crate) fn print_server_urls(config: &ServerUrlConfig) {
    let port: u16 = config.get_port();
    let route_prefix: &str = config.get_route_prefix();
    let index_html_file_name: &str = config.get_index_html_file_name();
    let mut addresses: Vec<IpAddr> = Vec::new();
    match if_addrs::get_if_addrs() {
        Ok(interfaces) => {
            for interface in interfaces {
                let ip: IpAddr = interface.addr.ip();
                if !addresses.contains(&ip) {
                    addresses.push(ip);
                }
            }
        }
        Err(error) => {
            log::warn!("Failed to enumerate network interfaces: {error}");
        }
    }
    if addresses.is_empty() {
        addresses.push(IpAddr::V4(Ipv4Addr::LOCALHOST));
    }
    for ip in addresses {
        let host: String = match ip {
            IpAddr::V6(_) => format!("[{ip}]"),
            IpAddr::V4(_) => format!("{ip}"),
        };
        let url: String =
            format!("{HTTP_SCHEME}://{host}:{port}/{route_prefix}/{index_html_file_name}");
        log::info!("Server: {url}");
        match QrCode::new(url.as_str()) {
            Ok(code) => {
                let string: String = code.render::<Dense1x2>().quiet_zone(false).build();
                for line in string.lines() {
                    log::info!("{line}");
                }
            }
            Err(error) => {
                log::warn!("Failed to generate QR code: {error}");
            }
        }
    }
}

/// Reads the wasm-bindgen JS bridge produced by `wasm-pack build
/// --target web` and returns a self-contained IIFE that inlines the
/// bridge and every module it imports, fetches the wasm module from
/// `wasm_url`, then calls `main()` once the instance is ready.
///
/// The bridge's own top-level `import` lines (the snippet helpers under
/// `./snippets/`) are stripped and their bodies inlined, so the result
/// makes zero HTTP requests for JavaScript: the page carries the whole
/// module graph in one `<script>`.
///
/// Fails closed. A bridge whose bootstrap this cannot reproduce — one
/// whose `__wbg_init` uses `import.meta.url` on a line that carries
/// other statements, one that imports a non-relative or named-bound
/// module — returns an error naming the construct, never a bootstrap
/// that silently fails in the browser.
///
/// # Arguments
///
/// - `&Path` - Directory holding the wasm-pack output.
/// - `&str` - File name of the generated JS bridge inside it.
///
/// # Returns
///
/// - `Result<String, EuvError>` - The IIFE source, or an error naming the
///   exact unsupported construct when the bridge cannot be inlined.
pub(crate) async fn build_inline_bridge(
    pkg_dir: &Path,
    js_name: &str,
    wasm_url: &str,
) -> Result<String, EuvError> {
    let js_path: PathBuf = pkg_dir.join(js_name);
    let bridge_source: String =
        read_to_string(&js_path)
            .await
            .map_err(|error: io::Error| EuvError::IoPath {
                message: String::from("Failed to read wasm-bindgen JS bridge"),
                path: js_path.clone(),
                error,
            })?;
    let inlined: String = inline_module_graph(&bridge_source, pkg_dir).await?;
    let js_only: String = strip_trailing_module_exports(&inlined);
    if !js_only.contains("function main(") {
        return Err(EuvError::Message(format!(
            "wasm-bindgen bridge '{js_name}' declares no main() for the bootstrap to call"
        )));
    }
    if !js_only.contains("__wbg_get_imports") {
        return Err(EuvError::Message(format!(
            "wasm-bindgen bridge '{js_name}' declares no __wbg_get_imports() \
             for the bootstrap to instantiate the module with"
        )));
    }
    let wasm_url_json: String = encode_js_string(wasm_url);
    Ok(format!(
        "(function() {{\n\
         {js_only}\n\
         var __euv_wasm_url = {wasm_url_json};\n\
         var __euv_base = (document.querySelector('base[href]') && document.baseURI) || location.href;\n\
         var __euv_url = new URL(__euv_wasm_url, __euv_base).toString();\n\
         function __euv_main() {{ if (typeof main === 'function') {{ main(); }} }}\n\
         if (typeof __wbg_init === 'function') {{\n\
         __wbg_init(__euv_url).then(__euv_main).catch(function(e) {{\n\
         console.error('[euv] inline bridge init failed:', e);\n\
         }});\n\
         }} else if (typeof initSync === 'function') {{\n\
         fetch(__euv_url).then(function(response) {{ return response.arrayBuffer(); }})\n\
         .then(function(bytes) {{ initSync({{ module: bytes }}); __euv_main(); }})\n\
         .catch(function(e) {{ console.error('[euv] inline bridge init failed:', e); }});\n\
         }} else {{\n\
         console.error('[euv] inline bridge: no __wbg_init or initSync to instantiate the module');\n\
         }}\n\
         }})();\n"
    ))
}

/// Walks the bridge and every module it imports, returning their bodies
/// concatenated in dependency order with the bridge's own `import` lines
/// and namespace aliases materialised as plain `var` objects.
///
/// Fails closed: an import clause shape this inliner does not model, a
/// specifier that is not relative, a specifier that escapes the package
/// directory, a snippet that cannot be read, or a snippet that exports
/// nothing a namespace import can bind all produce an error instead of a
/// silently truncated bootstrap.
///
/// # Arguments
///
/// - `&str` - The bridge source.
/// - `&Path` - Directory holding the wasm-pack output.
///
/// # Returns
///
/// - `Result<String, EuvError>` - The concatenated module bodies.
async fn inline_module_graph(bridge_source: &str, pkg_dir: &Path) -> Result<String, EuvError> {
    let mut snippets: Vec<(PathBuf, String)> = Vec::new();
    let mut namespace_clauses: Vec<NamespaceImport> = Vec::new();
    for line in bridge_source.lines() {
        let trimmed: &str = line.trim();
        if !trimmed.starts_with("import ") {
            continue;
        }
        let rest: &str = trimmed.strip_prefix("import ").unwrap_or_default();
        let clause: ImportClause<'_> = parse_import_clause(rest)?;
        if clause.default_binding.is_some() || !clause.named.is_empty() {
            return Err(EuvError::Message(format!(
                "wasm-bindgen bridge imports a default or named binding from '{}', \
                 which a self-contained bootstrap cannot reproduce",
                clause.spec
            )));
        }
        if !(clause.spec.starts_with("./") || clause.spec.starts_with("../")) {
            return Err(EuvError::Message(format!(
                "wasm-bindgen bridge imports the bare module '{}', which a \
                 self-contained bootstrap cannot fetch",
                clause.spec
            )));
        }
        if let Some(alias) = clause.namespace {
            namespace_clauses.push(NamespaceImport {
                alias: alias.to_string(),
                spec: clause.spec.to_string(),
            });
        }
        let snippet_path: PathBuf = resolve_snippet_path(pkg_dir, clause.spec).await?;
        if snippets
            .iter()
            .any(|(path, _): &(PathBuf, String)| path == &snippet_path)
        {
            continue;
        }
        let raw: String = read_to_string(&snippet_path)
            .await
            .map_err(|error: io::Error| EuvError::IoPath {
                message: String::from("Failed to read wasm-pack snippet module"),
                path: snippet_path.clone(),
                error,
            })?;
        snippets.push((snippet_path, raw));
    }
    let mut out: String = String::new();
    for (snippet_path, raw) in &snippets {
        let body: String = strip_module_exports(raw).map_err(|error: EuvError| match error {
            EuvError::Message(message) => EuvError::Message(format!(
                "{} in snippet module '{}'",
                message,
                snippet_path.display()
            )),
            other => other,
        })?;
        if body.trim().is_empty() {
            return Err(EuvError::Message(format!(
                "snippet module '{}' has no inlinable body",
                snippet_path.display()
            )));
        }
        out.push_str(&body);
        out.push('\n');
    }
    for clause in &namespace_clauses {
        let snippet_path: PathBuf = resolve_snippet_path(pkg_dir, &clause.spec).await?;
        let raw: String = snippets
            .iter()
            .find(|(path, _): &&(PathBuf, String)| *path == snippet_path)
            .map(|(_, raw): &(PathBuf, String)| raw.clone())
            .unwrap_or_default();
        let names: Vec<String> = extract_exported_names(&raw);
        if names.is_empty() {
            return Err(EuvError::Message(format!(
                "snippet module '{}' exports nothing the namespace binding '{}' \
                 could reference",
                clause.spec, clause.alias
            )));
        }
        out.push_str(&render_namespace_object(&clause.alias, &names));
    }
    for line in bridge_source.lines() {
        let trimmed: &str = line.trim();
        if trimmed.starts_with("import ") {
            continue;
        }
        if trimmed.contains(IMPORT_META_URL_MARKER) {
            continue;
        }
        out.push_str(line);
        out.push('\n');
    }
    Ok(out)
}

/// Parses one `import` clause, rejecting shapes the inliner cannot model.
///
/// # Arguments
///
/// - `&str` - The static import clause after the `import` keyword.
///
/// # Returns
///
/// - `Result<ImportClause<'_>, EuvError>` - Borrowed bindings and module specifier.
pub fn parse_import_clause(rest: &str) -> Result<ImportClause<'_>, EuvError> {
    let rest: &str = rest.trim().trim_end_matches(';').trim();
    let from_idx: usize = rest.find(" from ").ok_or_else(|| {
        EuvError::Message(format!(
            "bridge import '{rest}' is not a static `from` clause, which a \
             self-contained bootstrap cannot reproduce"
        ))
    })?;
    let head: &str = rest[..from_idx].trim();
    let quoted: &str = rest[from_idx + " from ".len()..].trim();
    let quote: char = match quoted.chars().next() {
        Some(quote @ ('\'' | '"')) => quote,
        _ => {
            return Err(EuvError::Message(format!(
                "bridge import specifier '{quoted}' is not a quoted string"
            )));
        }
    };
    let after: &str = &quoted[quote.len_utf8()..];
    let spec: &str = after.split(quote).next().unwrap_or_default();
    if spec.is_empty() {
        return Err(EuvError::Message(format!(
            "bridge import '{rest}' names an empty specifier"
        )));
    }
    if head.starts_with('*') {
        let alias: &str = head
            .strip_prefix('*')
            .map(|value: &str| value.trim())
            .and_then(|value: &str| value.strip_prefix("as "))
            .map(|value: &str| value.trim())
            .unwrap_or_default();
        if alias.is_empty() {
            return Err(EuvError::Message(format!(
                "bridge import '{rest}' is a namespace import without a binding"
            )));
        }
        return Ok(ImportClause {
            spec,
            namespace: Some(alias),
            default_binding: None,
            named: Vec::new(),
        });
    }
    if let Some(inner) = head.strip_prefix('{') {
        let inner: &str = inner.strip_suffix('}').ok_or_else(|| {
            EuvError::Message(format!(
                "bridge import '{rest}' has an unbalanced `{{` clause"
            ))
        })?;
        return Ok(ImportClause {
            spec,
            namespace: None,
            default_binding: None,
            named: inner
                .split(',')
                .map(|entry: &str| entry.trim())
                .filter(|entry: &&str| !entry.is_empty())
                .collect(),
        });
    }
    Ok(ImportClause {
        spec,
        namespace: None,
        default_binding: Some(head),
        named: Vec::new(),
    })
}

/// Renders the `var <alias> = { a: a, ... };` statement a namespace import
/// becomes once the module graph is gone.
///
/// # Arguments
///
/// - `&str` - Local namespace binding.
/// - `&[String]` - Exported names to expose on the namespace object.
///
/// # Returns
///
/// - `String` - The JavaScript namespace declaration.
fn render_namespace_object(alias: &str, names: &[String]) -> String {
    let mut out: String = format!("var {alias} = {{");
    for (index, name) in names.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        out.push_str(name);
        out.push(':');
        out.push_str(name);
    }
    out.push_str("};\n");
    out
}

/// Strips the `export` keyword from a snippet module, turning it into bare
/// declarations, and rejects export forms a bare declaration cannot carry.
///
/// # Arguments
///
/// - `&str` - The snippet module source.
///
/// # Returns
///
/// - `Result<String, EuvError>` - The same source with its `export` keywords
///   removed, or an error naming the unsupported export form.
fn strip_module_exports(source: &str) -> Result<String, EuvError> {
    let mut out: String = String::with_capacity(source.len());
    for line in source.lines() {
        let trimmed: &str = line.trim();
        let Some(rest) = trimmed.strip_prefix("export ") else {
            out.push_str(line);
            out.push('\n');
            continue;
        };
        match classify_export(rest) {
            ExportForm::Aliases(clause) => {
                out.push_str(&render_export_aliases(clause)?);
            }
            ExportForm::Declaration(declaration) => {
                out.push_str(declaration);
                out.push('\n');
            }
            ExportForm::Unsupported => {
                return Err(EuvError::Message(format!(
                    "snippet exports '{rest}', a form a self-contained \
                     bootstrap cannot reproduce"
                )));
            }
        }
    }
    Ok(out)
}

/// Classifies the text following an `export ` keyword.
///
/// # Arguments
///
/// - `&str` - Source immediately after the export keyword.
///
/// # Returns
///
/// - `ExportForm<'_>` - A declaration, alias list, or unsupported export.
fn classify_export(rest: &str) -> ExportForm<'_> {
    let trimmed: &str = rest.trim_end().trim_end_matches(';');
    if let Some(inner) = trimmed.strip_prefix('{')
        && inner.trim_end().ends_with('}')
    {
        return ExportForm::Aliases(inner.trim_end().trim_end_matches('}'));
    }
    if EXPORT_DECLARATION_KEYWORDS
        .iter()
        .any(|keyword: &&str| trimmed.starts_with(keyword))
    {
        return ExportForm::Declaration(trimmed);
    }
    ExportForm::Unsupported
}

/// Renders an `{ a, b as c }` export clause as the assignments that keep
/// each local binding reachable under its exported name.
///
/// An export of `wasm`, or an alias whose two names match, is dropped:
/// the inlined bootstrap is a plain `<script>` with no module namespace
/// and no `wasm` binding in scope, and an identity alias needs no
/// assignment.
///
/// # Arguments
///
/// - `&str` - The clause body between the braces.
///
/// # Returns
///
/// - `Result<String, EuvError>` - The assignments, or an error naming a
///   non-identifier entry.
fn render_export_aliases(clause: &str) -> Result<String, EuvError> {
    let mut out: String = String::new();
    for entry in clause.split(',') {
        let entry: &str = entry.trim();
        if entry.is_empty() {
            continue;
        }
        let (original, exported): (&str, &str) = match entry.split_once(" as ") {
            Some((original, exported)) => (original.trim(), exported.trim()),
            None => (entry, entry),
        };
        if !is_identifier(original) || !is_identifier(exported) {
            return Err(EuvError::Message(format!(
                "snippet exports the non-identifier entry '{entry}'"
            )));
        }
        if original == exported || exported == "default" || original == "wasm" {
            continue;
        }
        out.push_str(exported);
        out.push_str(" = ");
        out.push_str(original);
        out.push_str(";\n");
    }
    Ok(out)
}

/// Returns `true` when `value` is a bare JavaScript identifier.
///
/// # Arguments
///
/// - `&str` - Candidate binding name.
///
/// # Returns
///
/// - `bool` - Whether the value contains only supported identifier characters.
fn is_identifier(value: &str) -> bool {
    !value.is_empty()
        && !value.starts_with(|c: char| c.is_ascii_digit())
        && value
            .chars()
            .all(|c: char| c.is_ascii_alphanumeric() || c == '_' || c == '$')
}

/// Scans a snippet module for every name it exports, so a namespace import
/// can be rebuilt as a plain object. `export function`, `export async
/// function`, `export class` and `export const|let|var` all count.
///
/// # Arguments
///
/// - `&str` - The snippet module source to scan.
///
/// # Returns
///
/// - `Vec<String>` - Every exported name, in source order.
pub fn extract_exported_names(source: &str) -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    for line in source.lines() {
        let trimmed: &str = line.trim();
        let Some(rest) = trimmed.strip_prefix("export ") else {
            continue;
        };
        let declaration: Option<&str> = EXPORT_DECLARATION_KEYWORDS
            .iter()
            .find_map(|keyword: &&str| rest.strip_prefix(keyword));
        if let Some(name) = declaration.and_then(leading_identifier)
            && !names.iter().any(|existing: &String| existing == name)
        {
            names.push(name.to_string());
        }
    }
    names
}

/// Returns the leading identifier of `value`, or `None` when it starts with
/// something that cannot be one.
///
/// # Arguments
///
/// - `&str` - Declaration text starting with the exported name.
///
/// # Returns
///
/// - `Option<&str>` - The leading identifier, when present.
fn leading_identifier(value: &str) -> Option<&str> {
    let end: usize = value
        .find(|c: char| !c.is_alphanumeric() && c != '_' && c != '$')
        .unwrap_or(value.len());
    if end == 0 { None } else { Some(&value[..end]) }
}

/// Resolves a relative specifier to a path inside `pkg_dir`.
///
/// # Arguments
///
/// - `&Path` - Directory holding the wasm-pack output.
/// - `&str` - Relative specifier taken from the bridge's import clause.
///
/// # Returns
///
/// - `Result<PathBuf, EuvError>` - The resolved path, or an error if the
///   specifier climbs above `pkg_dir`.
async fn resolve_snippet_path(pkg_dir: &Path, spec: &str) -> Result<PathBuf, EuvError> {
    let base: &str = spec.strip_prefix("./").unwrap_or(spec);
    let mut path: PathBuf = pkg_dir.to_path_buf();
    for segment in base.split('/') {
        if segment.is_empty() || segment == "." {
            continue;
        }
        if segment == ".." {
            return Err(EuvError::Message(String::from(
                "Snippet import spec escapes pkg directory",
            )));
        }
        path.push(segment);
    }
    let root: PathBuf =
        canonicalize(pkg_dir)
            .await
            .map_err(|error: io::Error| EuvError::IoPath {
                message: String::from("Failed to resolve the WASM output directory"),
                path: pkg_dir.to_path_buf(),
                error,
            })?;
    let resolved: PathBuf =
        canonicalize(&path)
            .await
            .map_err(|error: io::Error| EuvError::IoPath {
                message: String::from("Failed to resolve the imported snippet"),
                path: path.clone(),
                error,
            })?;
    if !resolved.starts_with(root) {
        return Err(EuvError::Message(format!(
            "Snippet import '{spec}' resolves outside the WASM output directory",
        )));
    }
    Ok(resolved)
}

/// Removes the bridge's trailing `export { ... }` clauses, turning each
/// exported alias into the assignment that keeps the local function
/// reachable under its exported name, and its `export function` lines into
/// plain declarations.
///
/// # Arguments
///
/// - `&str` - The bridge source, after its `import` lines were removed.
///
/// # Returns
///
/// - `String` - The source with its module-level export syntax removed.
fn strip_trailing_module_exports(source: &str) -> String {
    let mut out: String = String::with_capacity(source.len());
    for line in source.lines() {
        let trimmed: &str = line.trim();
        let Some(rest) = trimmed.strip_prefix("export ") else {
            out.push_str(line);
            out.push('\n');
            continue;
        };
        match classify_export(rest) {
            ExportForm::Aliases(clause) => {
                if let Ok(aliases) = render_export_aliases(clause) {
                    out.push_str(&aliases);
                }
            }
            ExportForm::Declaration(declaration) => {
                out.push_str(declaration);
                out.push('\n');
            }
            ExportForm::Unsupported => {
                out.push_str(line);
                out.push('\n');
            }
        }
    }
    out
}

/// Encodes a value as a JavaScript string literal, including the
/// surrounding quotes, escaping every character that would otherwise end
/// the literal, open a new line inside it, or close the enclosing
/// `<script>` element early.
///
/// # Arguments
///
/// - `&str` - The value to encode.
///
/// # Returns
///
/// - `String` - A quoted, escaped JavaScript string.
pub(crate) fn encode_js_string(value: &str) -> String {
    let mut out: String = String::with_capacity(value.len() + 2);
    out.push('"');
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{08}' => out.push_str("\\b"),
            '\u{0C}' => out.push_str("\\f"),
            '<' => out.push_str("\\u003c"),
            '>' => out.push_str("\\u003e"),
            '&' => out.push_str("\\u0026"),
            '\u{2028}' => out.push_str("\\u2028"),
            '\u{2029}' => out.push_str("\\u2029"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// Deletes every JavaScript artifact wasm-pack emitted for this build:
/// the bridge itself, its `.d.ts` declarations, its source maps and the
/// `snippets/` modules that were inlined into the HTML.
///
/// Runs only after the HTML carrying the inlined bridge has been written,
/// and only inside the wasm-pack output directory. The walk classifies each
/// entry with `symlink_metadata`, so a directory symlink is never followed:
/// a link pointing outside the output, or into a cycle, is left in place
/// and named, rather than extending the walk without bound. Every file
/// that is deleted is first confirmed to resolve inside the output.
///
/// A directory wasm-pack itself created is removed only once it is
/// genuinely empty — `remove_dir`, never `remove_dir_all` — so a
/// non-JavaScript asset an author left inside `snippets/` survives
/// together with its directory. Unreadable directories and failed
/// enumerations are errors, not silent skips: reporting success while
/// JavaScript remains is exactly the failure this guards against.
///
/// # Arguments
///
/// - `&Path` - The wasm-pack output directory.
///
/// # Returns
///
/// - `Result<Vec<PathBuf>, EuvError>` - The files removed, or an error naming
///   what could not be read or deleted.
pub(crate) async fn remove_generated_js(pkg_dir: &Path) -> Result<Vec<PathBuf>, EuvError> {
    let root: PathBuf = match canonicalize(pkg_dir).await {
        Ok(root) => root,
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            return Ok(Vec::new());
        }
        Err(error) => {
            return Err(EuvError::IoPath {
                message: String::from("Failed to resolve the wasm-pack output directory"),
                path: pkg_dir.to_path_buf(),
                error,
            });
        }
    };
    let mut removed: Vec<PathBuf> = Vec::new();
    let mut generated_dirs: Vec<PathBuf> = Vec::new();
    let mut pending: Vec<PathBuf> = vec![root.clone()];
    while let Some(dir) = pending.pop() {
        let mut entries: ReadDir = read_dir(&dir).await.map_err(|error: io::Error| {
            EuvError::IoPath {
                message: String::from(
                    "Failed to list the wasm-pack output directory while removing inlined JavaScript",
                ),
                path: dir.clone(),
                error,
            }
        })?;
        loop {
            let entry: DirEntry = match entries.next_entry().await {
                Ok(Some(entry)) => entry,
                Ok(None) => break,
                Err(error) => {
                    return Err(EuvError::IoPath {
                        message: String::from(
                            "Failed to read an entry of the wasm-pack output directory while \
                             removing inlined JavaScript",
                        ),
                        path: dir.clone(),
                        error,
                    });
                }
            };
            let path: PathBuf = entry.path();
            let file_type: FileType =
                entry
                    .file_type()
                    .await
                    .map_err(|error: io::Error| EuvError::IoPath {
                        message: String::from(
                            "Failed to stat an entry of the wasm-pack output directory while \
                         removing inlined JavaScript",
                        ),
                        path: path.clone(),
                        error,
                    })?;
            if file_type.is_symlink() {
                // Never followed: a link may leave the package or close a
                // cycle, and neither is this walk's to resolve.
                log::warn!(
                    "Leaving '{}' in place: it is a symlink, and the cleanup never follows one",
                    path.display()
                );
                continue;
            }
            if file_type.is_dir() {
                if path.starts_with(root.join(SNIPPET_DIR_NAME)) {
                    generated_dirs.push(path.clone());
                }
                pending.push(path);
                continue;
            }
            let canonical: PathBuf =
                canonicalize(&path)
                    .await
                    .map_err(|error: io::Error| EuvError::IoPath {
                        message: String::from(
                            "Failed to resolve an entry of the wasm-pack output directory before \
                         deleting it",
                        ),
                        path: path.clone(),
                        error,
                    })?;
            if !canonical.starts_with(&root) {
                log::warn!(
                    "Leaving '{}' in place: it resolves outside the wasm-pack output",
                    path.display()
                );
                continue;
            }
            if !is_generated_js_artifact(&path) {
                continue;
            }
            remove_file(&path)
                .await
                .map_err(|error: io::Error| EuvError::IoPath {
                    message: String::from(
                        "Failed to remove a generated JavaScript file after inlining it",
                    ),
                    path: path.clone(),
                    error,
                })?;
            removed.push(path);
        }
    }
    for dir in generated_dirs.iter().rev() {
        match remove_dir(dir).await {
            Ok(()) => {}
            Err(error) if error.kind() == io::ErrorKind::DirectoryNotEmpty => {}
            Err(error) => {
                return Err(EuvError::IoPath {
                    message: String::from("Failed to remove an emptied snippet directory"),
                    path: dir.clone(),
                    error,
                });
            }
        }
    }
    log::debug!(
        "Removed {} generated JavaScript file(s) and {} empty generated directory(ies)",
        removed.len(),
        generated_dirs.len()
    );
    Ok(removed)
}

/// Returns `true` for the JavaScript-adjacent files wasm-pack generates:
/// `.js`, `.mjs`, `.cjs`, their source maps and their `.d.ts` declarations.
///
/// # Arguments
///
/// - `&Path` - Output file path to classify.
///
/// # Returns
///
/// - `bool` - Whether the file belongs to the generated JavaScript artifacts.
fn is_generated_js_artifact(path: &Path) -> bool {
    let name: String = path
        .file_name()
        .and_then(|name: &ffi::OsStr| name.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    GENERATED_JS_SUFFIXES
        .iter()
        .any(|suffix: &&str| name.ends_with(suffix))
}
