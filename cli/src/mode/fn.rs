use super::*;

/// Executes the build-only pipeline.
///
/// # Arguments
///
/// - `ModeArgs` - The CLI arguments.
///
/// # Returns
///
/// - `Result<(), EuvError>` - Indicates success or failure.
pub async fn build_mode(mut args: ModeArgs) -> Result<(), EuvError> {
    reconcile_args(&mut args);
    args.set_crate_path(std::fs::canonicalize(args.get_crate_path()).map_err(
        |error: io::Error| EuvError::IoPath {
            message: String::from("Invalid crate-path"),
            path: args.get_crate_path().clone(),
            error,
        },
    )?);
    let crate_path_str: String = args.get_crate_path().to_string_lossy().to_string();
    if crate_path_str.starts_with(WINDOWS_UNC_PREFIX) {
        args.set_crate_path(PathBuf::from(
            crate_path_str
                .strip_prefix(WINDOWS_UNC_PREFIX)
                .unwrap_or(&crate_path_str),
        ));
    }
    print_banner(Action::Build);
    run_build_only_pipeline(&args).await?;
    log::info!("Build completed. Exiting (build-only mode).");
    Ok(())
}

/// Executes the format command.
///
/// # Arguments
///
/// - `FmtArgs` - The CLI arguments for the fmt command.
///
/// # Returns
///
/// - `Result<(), EuvError>` - Indicates success or failure.
pub async fn fmt_mode(args: FmtArgs) -> Result<(), EuvError> {
    let fmt_path: PathBuf = if args.get_path().is_absolute() {
        args.get_path().clone()
    } else {
        std::env::current_dir()
            .map_err(|error: io::Error| EuvError::Io {
                message: String::from("Failed to get current directory"),
                error,
            })?
            .join(args.get_path())
    };
    let mode: FmtMode = if args.get_check() {
        FmtMode::Check
    } else {
        FmtMode::Write
    };
    format_dir(&fmt_path, mode).await
}

/// Executes the run mode (build + dev server + hot reload).
///
/// # Arguments
///
/// - `ModeArgs` - The CLI arguments.
///
/// # Returns
///
/// - `Result<(), EuvError>` - Indicates success or failure.
pub async fn run_mode(mut args: ModeArgs) -> Result<(), EuvError> {
    reconcile_args(&mut args);
    args.set_crate_path(std::fs::canonicalize(args.get_crate_path()).map_err(
        |error: io::Error| EuvError::IoPath {
            message: String::from("Invalid crate-path"),
            path: args.get_crate_path().clone(),
            error,
        },
    )?);
    let crate_path_str: String = args.get_crate_path().to_string_lossy().to_string();
    if crate_path_str.starts_with(WINDOWS_UNC_PREFIX) {
        args.set_crate_path(PathBuf::from(
            crate_path_str
                .strip_prefix(WINDOWS_UNC_PREFIX)
                .unwrap_or(&crate_path_str),
        ));
    }
    let serving_route_prefix: String = resolve_serving_route_prefix(&args);
    let initial_html: String = match run_build_pipeline(&args, None).await {
        Ok(html) => html,
        Err(error) => {
            log::error!("Initial build pipeline failed: {error}");
            let html_config: HtmlConfig = HtmlConfig::new(
                resolve_serving_root(&args).await,
                resolve_import_path(&args),
                resolve_build_mode(&args) == BuildMode::Release,
                args.try_get_index_html().clone(),
            );
            generate_html(&html_config).await?
        }
    };
    print_banner(Action::Run);
    let (reload_tx, _): (
        broadcast::Sender<ReloadEvent>,
        broadcast::Receiver<ReloadEvent>,
    ) = broadcast::channel(16);
    let state: Arc<AppState> = Arc::new(AppState::new(
        RwLock::new(initial_html),
        reload_tx.clone(),
        RwLock::new(false),
        args.clone(),
    ));
    let state_for_watch: Arc<AppState> = Arc::clone(&state);
    tokio::spawn(async move {
        if let Err(error) = watch_and_build(state_for_watch).await {
            log::error!("Watch error: {error}");
        }
    });
    let pkg_dir: PathBuf = resolve_pkg_dir(&args);
    log::info!("Serving pkg from: {}", pkg_dir.display());
    let mut server: Server = Server::default();
    let mut server_config: ServerConfig = ServerConfig::default();
    server_config.set_nodelay(Some(false));
    server_config.set_address(Server::format_bind_address(DEFAULT_HOST, args.get_port()));
    server.server_config(server_config);
    server.request_middleware::<RequestMiddleware>();
    server.response_middleware::<ResponseMiddleware>();
    server.route::<RootRoute>("/");
    server.route::<RootRoute>(format!("/{INDEX_HTML_FILE_NAME}"));
    server.route::<IndexRoute>(format!("{serving_route_prefix}/{{path:.*}}"));
    server.route::<ReloadRoute>(RELOAD_ROUTE);
    if let Err(error) = set_global_state(Arc::clone(&state)) {
        log::error!("Failed to set global state: {error}");
    }
    print_server_urls(&ServerUrlConfig::new(
        args.get_port(),
        serving_route_prefix.clone(),
        INDEX_HTML_FILE_NAME.to_string(),
    ));
    let server_control_hook: ServerControlHook = server
        .run()
        .await
        .map_err(|error: ServerError| EuvError::Server(error.to_string()))?;
    server_control_hook.wait().await;
    Ok(())
}
