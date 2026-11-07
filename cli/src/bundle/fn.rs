use super::*;

/// Returns the host operating system this build runs on.
///
/// # Returns
///
/// - `HostOs` - The host operating system.
pub fn current_host() -> HostOs {
    match HOST_OS {
        HOST_MACOS => HostOs::Macos,
        HOST_LINUX => HostOs::Linux,
        HOST_WINDOWS => HostOs::Windows,
        _ => HostOs::Unsupported,
    }
}

/// Returns the host architecture this build runs on.
///
/// # Returns
///
/// - `String` - The host architecture identifier.
pub fn current_arch() -> String {
    HOST_ARCH.to_string()
}

/// Returns every platform the given host can build.
///
/// The order is the build order: the desktop clients first, then the
/// mobile deliverables, so a failure in the desktop client is reported
/// before the mobile toolchains are entered.
///
/// # Arguments
///
/// - `HostOs` - The host operating system.
///
/// # Returns
///
/// - `Vec<NativePlatform>` - The platforms this host builds.
pub fn host_platforms(host: HostOs) -> Vec<NativePlatform> {
    match host {
        HostOs::Macos => vec![
            NativePlatform::Macos,
            NativePlatform::Ios,
            NativePlatform::IosSimulator,
            NativePlatform::Android,
        ],
        HostOs::Linux => vec![NativePlatform::Linux, NativePlatform::Android],
        HostOs::Windows => vec![NativePlatform::Windows],
        HostOs::Unsupported => Vec::new(),
    }
}

/// Returns the kind of builder a platform needs.
///
/// # Arguments
///
/// - `NativePlatform` - The platform to classify.
///
/// # Returns
///
/// - `NativeTask` - The builder the platform needs.
pub fn native_task(platform: NativePlatform) -> NativeTask {
    match platform {
        NativePlatform::Android => NativeTask::Android,
        _ => NativeTask::Client,
    }
}

/// Reports whether a host can build a platform.
///
/// # Arguments
///
/// - `HostOs` - The host operating system.
/// - `NativePlatform` - The platform to test.
///
/// # Returns
///
/// - `bool` - Whether the host can build the platform.
pub fn host_supports(host: HostOs, platform: NativePlatform) -> bool {
    match platform {
        NativePlatform::Android => matches!(host, HostOs::Macos | HostOs::Linux),
        NativePlatform::Macos | NativePlatform::Ios | NativePlatform::IosSimulator => {
            matches!(host, HostOs::Macos)
        }
        NativePlatform::Linux => matches!(host, HostOs::Linux),
        NativePlatform::Windows => matches!(host, HostOs::Windows),
    }
}

/// Renders a platform list as a comma separated list of names.
///
/// # Arguments
///
/// - `&[NativePlatform]` - The platforms to render.
///
/// # Returns
///
/// - `String` - The rendered platform list.
pub fn describe_platforms(platforms: &[NativePlatform]) -> String {
    platforms
        .iter()
        .map(|platform: &NativePlatform| platform.platform_name())
        .collect::<Vec<&str>>()
        .join(", ")
}

/// Renders the host requirement of every platform in a list.
///
/// # Arguments
///
/// - `&[NativePlatform]` - The platforms to describe.
///
/// # Returns
///
/// - `String` - The rendered requirements.
pub fn describe_requirements(platforms: &[NativePlatform]) -> String {
    platforms
        .iter()
        .map(|platform: &NativePlatform| {
            format!(
                "{} needs a {} host",
                platform.platform_name(),
                platform.required_host()
            )
        })
        .collect::<Vec<String>>()
        .join("; ")
}

/// Resolves the requested platforms from the command line.
///
/// `--all` selects every platform the host can build, and `--platforms`
/// selects an explicit list. The two are mutually exclusive, and neither
/// means the ordinary web-only build.
///
/// # Arguments
///
/// - `bool` - Whether `--all` was given.
/// - `&[NativePlatform]` - The platforms given by `--platforms`.
///
/// # Returns
///
/// - `Result<Vec<NativePlatform>, EuvError>` - The requested platforms.
pub fn resolve_requested_platforms(
    all: bool,
    platforms: &[NativePlatform],
) -> Result<Vec<NativePlatform>, EuvError> {
    if all && !platforms.is_empty() {
        return Err(EuvError::Message(format!(
            "{ALL_ARG} and {PLATFORMS_ARG} cannot be combined: {ALL_ARG} selects every \
             platform this host can build, and {PLATFORMS_ARG} names them explicitly."
        )));
    }
    if all {
        return Ok(host_platforms(current_host()));
    }
    if platforms.is_empty() {
        return Ok(Vec::new());
    }
    let mut unique: Vec<NativePlatform> = Vec::new();
    for platform in platforms {
        if !unique.contains(platform) {
            unique.push(*platform);
        }
    }
    Ok(unique)
}

/// Resolves the native build plan for the requested platforms.
///
/// Every requested platform that this host cannot build is reported as
/// unsupported, and the plan is only produced when nothing requested is
/// out of reach: an incompatible request must fail before any build
/// starts rather than silently skipping a deliverable.
///
/// # Arguments
///
/// - `&[NativePlatform]` - The requested platforms.
/// - `HostOs` - The host to resolve against.
///
/// # Returns
///
/// - `Result<NativePlan, EuvError>` - The resolved plan, or an error naming
///   every platform that cannot be built here.
pub fn resolve_native_plan(
    requested: &[NativePlatform],
    host: HostOs,
) -> Result<NativePlan, EuvError> {
    if host == HostOs::Unsupported || current_arch() == ARCH_UNSUPPORTED {
        return Err(EuvError::Message(
            "Native packaging is unsupported on this operating system or architecture".to_string(),
        ));
    }
    let mut unsupported: Vec<NativePlatform> = Vec::new();
    for platform in requested {
        if !host_supports(host, *platform) && !unsupported.contains(platform) {
            unsupported.push(*platform);
        }
    }
    if !unsupported.is_empty() {
        return Err(EuvError::Message(format!(
            "{} cannot be built on {}: {}. Build {} on a matching host, or drop it from \
             {PLATFORMS_ARG}.",
            describe_platforms(&unsupported),
            host.host_name(),
            describe_requirements(&unsupported),
            describe_platforms(&unsupported),
        )));
    }
    let mut targets: Vec<NativeTarget> = Vec::new();
    for platform in requested {
        targets.push(NativeTarget {
            platform: *platform,
            task: native_task(*platform),
        });
    }
    Ok(NativePlan {
        host,
        arch: current_arch(),
        targets,
        unsupported,
    })
}

/// Prints the resolved plan for a native build.
///
/// The plan names the host, the deliverables that will be built, and the
/// platforms that were requested but cannot be built here.
///
/// # Arguments
///
/// - `&NativePlan` - The resolved plan.
pub fn print_native_plan(plan: &NativePlan) {
    log::info!("Host: {} ({})", plan.host.host_name(), plan.arch);
    log::info!(
        "Native selection is host-specific; other operating systems require matching build hosts."
    );
    if plan.targets.is_empty() {
        log::info!("Native platforms: none");
    } else {
        for target in &plan.targets {
            log::info!(
                "Native platform: {} ({})",
                target.platform.platform_name(),
                target.task.task_name()
            );
        }
    }
    if !plan.unsupported.is_empty() {
        log::warn!(
            "Not buildable on this host: {} ({})",
            describe_platforms(&plan.unsupported),
            describe_requirements(&plan.unsupported)
        );
    }
}

/// Resolves the native client project directory.
///
/// The directory inside this repository is used unless the caller names
/// another one, so an external layout can be built without changing the
/// repository. The directory is checked when the build runs rather than
/// here, so a plan can be printed for a layout that is not present yet.
///
/// # Arguments
///
/// - `&Path` - The directory this build runs in.
/// - `Option<&PathBuf>` - The `--native-path` override, when one was given.
///
/// # Returns
///
/// - `PathBuf` - The native client project directory.
pub fn resolve_native_project(base: &Path, native_path: Option<&PathBuf>) -> PathBuf {
    match native_path {
        Some(path) if path.is_absolute() => path.clone(),
        Some(path) => base.join(path),
        None => base.join(NATIVE_PROJECT_RELATIVE_PATH),
    }
}

/// Rejects a native client project directory that is not there.
///
/// The check runs when the build starts rather than when the plan is
/// resolved, so a plan can be printed for a layout that is not present
/// yet, and so the failure names the directory that is missing.
///
/// # Arguments
///
/// - `&Path` - The native client project directory.
///
/// # Returns
///
/// - `Result<(), EuvError>` - `Ok` when the directory exists.
pub fn require_native_project(native_project: &Path) -> Result<(), EuvError> {
    if native_project.is_dir() {
        return Ok(());
    }
    Err(EuvError::Message(format!(
        "The native client project directory '{}' does not exist. Pass {NATIVE_PATH_ARG} to \
         point at it.",
        native_project.display()
    )))
}

/// Resolves the directory the web build output is published into.
///
/// The native builders embed the pages this build produced, so the
/// directory must be the one the web build actually wrote: the serving
/// root holds the page, and the wasm artifacts sit beside it.
///
/// # Arguments
///
/// - `&ModeArgs` - The build arguments carrying the crate path.
///
/// # Returns
///
/// - `PathBuf` - The web build output directory.
pub async fn resolve_web_dist(args: &ModeArgs) -> PathBuf {
    resolve_serving_root(args).await
}

/// Rejects the native-only flags when they reach the run mode.
///
/// The run mode forwards everything it does not own to `wasm-pack`, so a
/// misplaced `--all` would silently become a wasm-pack argument and the
/// native build the user asked for would never run. The run mode serves
/// the pages in a browser, so it has no native deliverable to produce.
///
/// # Arguments
///
/// - `&ModeArgs` - The run mode arguments.
///
/// # Returns
///
/// - `Result<(), EuvError>` - `Ok` when no native-only flag was given.
pub fn reject_native_flags_in_run(args: &ModeArgs) -> Result<(), EuvError> {
    for argument in args.get_wasm_pack_args() {
        let name: &str = argument.split('=').next().unwrap_or(argument.as_str());
        if name == ALL_ARG || name == PLATFORMS_ARG || name == NATIVE_PATH_ARG || name == PLAN_ARG {
            return Err(EuvError::Message(format!(
                "{name} selects a native build, which the run mode cannot perform. Use \
                 `euv build {name}` instead."
            )));
        }
    }
    Ok(())
}

/// Dispatches the build mode to the web-only pipeline or the native one.
///
/// A build that names no native platform keeps the ordinary web-only
/// behaviour, so the existing `euv build` invocations are unchanged.
/// A build that names one, or asks for the whole host plan, runs the
/// native pipeline as well.
///
/// # Arguments
///
/// - `BuildArgs` - The build arguments.
///
/// # Returns
///
/// - `Result<(), EuvError>` - Indicates success or failure.
pub async fn dispatch_build_mode(args: BuildArgs) -> Result<(), EuvError> {
    if args.try_get_native_path().is_some() && !args.get_all() && args.get_platforms().is_empty() {
        return Err(EuvError::Message(format!(
            "{NATIVE_PATH_ARG} requires {ALL_ARG} or {PLATFORMS_ARG}"
        )));
    }
    if args.get_all() || !args.get_platforms().is_empty() || args.get_plan() {
        build_all_mode(args).await
    } else {
        build_mode(args.get_mode().clone()).await
    }
}

/// Executes the build pipeline that produces the web output and every
/// requested native client.
///
/// The web build runs exactly once, and the directory it wrote is handed
/// to the native builders so every client embeds the same pages the web
/// build produced. The native deliverables are built one after another,
/// and a failing builder stops the pipeline with its own exit status
/// rather than being reported as a skipped deliverable.
///
/// # Arguments
///
/// - `BuildArgs` - The build arguments.
///
/// # Returns
///
/// - `Result<(), EuvError>` - Indicates success or failure.
pub async fn build_all_mode(args: BuildArgs) -> Result<(), EuvError> {
    let mut mode_args: ModeArgs = args.get_mode().clone();
    reconcile_args(&mut mode_args);
    mode_args.set_crate_path(sync_canonicalize(mode_args.get_crate_path()).map_err(
        |error: io::Error| EuvError::IoPath {
            message: ERROR_INVALID_CRATE_PATH.to_string(),
            path: mode_args.get_crate_path().clone(),
            error,
        },
    )?);
    let crate_path_str: String = mode_args.get_crate_path().to_string_lossy().to_string();
    if crate_path_str.starts_with(WINDOWS_UNC_PREFIX) {
        mode_args.set_crate_path(PathBuf::from(
            crate_path_str
                .strip_prefix(WINDOWS_UNC_PREFIX)
                .unwrap_or(&crate_path_str),
        ));
    }
    let requested: Vec<NativePlatform> =
        resolve_requested_platforms(args.get_all(), args.get_platforms())?;
    let plan: NativePlan = resolve_native_plan(&requested, current_host())?;
    print_banner(Action::Build);
    print_native_plan(&plan);
    if args.get_plan() {
        log::info!("Plan requested. Exiting without building.");
        return Ok(());
    }
    if plan.get_targets().is_empty() {
        run_build_only_pipeline(&mode_args).await?;
        log::info!("Build completed. Exiting (build-only mode).");
        return Ok(());
    }
    let base: PathBuf = current_dir().map_err(|error: io::Error| EuvError::Io {
        message: ERROR_CURRENT_DIRECTORY.to_string(),
        error,
    })?;
    let native_path: Option<&PathBuf> = args.try_get_native_path().as_ref();
    let native_project: PathBuf = resolve_native_project(&base, native_path);
    require_native_project(&native_project)?;
    prepare_native_profile(&mut mode_args)?;
    run_build_only_pipeline(&mode_args).await?;
    let web_dist: PathBuf = resolve_web_dist(&mode_args).await;
    log::info!("Web build output: {}", web_dist.display());
    run_native_builds(&plan, &native_project, &web_dist, NATIVE_MODE_RELEASE).await?;
    log::info!("Build completed. Exiting (build-all mode).");
    Ok(())
}

/// Defaults native packaging to release and rejects unsupported build profiles.
///
/// # Arguments
///
/// - `&mut ModeArgs` - Web arguments sharing the native release snapshot.
///
/// # Returns
///
/// - `Result<(), EuvError>` - Success for a release-compatible request.
pub(crate) fn prepare_native_profile(args: &mut ModeArgs) -> Result<(), EuvError> {
    if args.get_dev() || args.get_profiling() {
        return Err(EuvError::Message(
            "Native packages support release builds only; omit --dev and --profiling".to_string(),
        ));
    }
    args.set_release(true);
    Ok(())
}

/// Runs the native builders for every deliverable in the plan.
///
/// The deliverables are built one after another: the Android build and
/// the Apple builds both drive the same Cargo target directory and the
/// same Gradle and Xcode state, so a parallel run would have them fight
/// over the same files. Every step's exit status is propagated: a
/// builder that fails stops the pipeline instead of being reported as
/// a skipped deliverable.
///
/// # Arguments
///
/// - `&NativePlan` - The resolved plan.
/// - `&Path` - The native client project directory.
/// - `&Path` - The web build output directory the clients must embed.
/// - `&str` - The native build mode.
///
/// # Returns
///
/// - `Result<(), EuvError>` - `Ok` when every deliverable was built.
pub(crate) async fn run_native_builds(
    plan: &NativePlan,
    native_project: &Path,
    web_dist: &Path,
    mode: &str,
) -> Result<(), EuvError> {
    for target in plan.get_targets() {
        let platform: &str = target.platform.platform_name();
        log::info!("Building native client: {platform}");
        match target.task {
            NativeTask::Android => {
                run_android_build(native_project, web_dist, mode).await?;
            }
            NativeTask::Client => {
                run_client_build(native_project, platform, web_dist, mode).await?;
            }
        }
    }
    Ok(())
}

/// Runs the Android build script in the native client project.
///
/// The Android builder reads the web pages from the environment, so the
/// directory this pipeline built is passed in rather than left to the
/// builder's own default.
///
/// # Arguments
///
/// - `&Path` - The native client project directory.
/// - `&Path` - The web build output directory the client must embed.
/// - `&str` - The native build mode.
///
/// # Returns
///
/// - `Result<(), EuvError>` - `Ok` when the build succeeded.
async fn run_android_build(
    native_project: &Path,
    web_dist: &Path,
    mode: &str,
) -> Result<(), EuvError> {
    let script: PathBuf = native_project.join(ANDROID_BUILD_SCRIPT_NAME);
    if !script.is_file() {
        return Err(EuvError::Message(format!(
            "The Android build script '{}' does not exist.",
            script.display()
        )));
    }
    let mut command: Command = Command::new(BASH_COMMAND);
    command
        .arg(&script)
        .arg(PLATFORM_ANDROID)
        .arg(mode)
        .current_dir(native_project)
        .env(ENV_WEB_DIST, web_dist)
        .env(ENV_NATIVE_MODE, mode)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());
    run_command(command, "Android build").await
}

/// Runs the per-platform client builder in the native client project.
///
/// The client builders read the web pages from the environment, so the
/// directory this pipeline built is passed in rather than left to the
/// builder's own default.
///
/// # Arguments
///
/// - `&Path` - The native client project directory.
/// - `&str` - The platform to build.
/// - `&Path` - The web build output directory the client must embed.
/// - `&str` - The native build mode.
///
/// # Returns
///
/// - `Result<(), EuvError>` - `Ok` when the build succeeded.
async fn run_client_build(
    native_project: &Path,
    platform: &str,
    web_dist: &Path,
    mode: &str,
) -> Result<(), EuvError> {
    let script: PathBuf = native_project
        .join("scripts")
        .join(BUILD_CLIENT_SCRIPT_NAME);
    if !script.is_file() {
        return Err(EuvError::Message(format!(
            "The native client builder '{}' does not exist.",
            script.display()
        )));
    }
    let mut command: Command = Command::new(NODE_COMMAND);
    command
        .arg(&script)
        .arg(platform)
        .current_dir(native_project)
        .env(ENV_WEB_DIST, web_dist)
        .env(ENV_NATIVE_MODE, mode)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());
    run_command(command, "native client build").await
}

/// Runs a native build command and propagates its exit status.
///
/// The command's output is forwarded as it happens, so a failing build
/// shows the same diagnostics the builder printed on its own.
///
/// # Arguments
///
/// - `Command` - The command to run.
/// - `&str` - The name of the step, used in the failure message.
///
/// # Returns
///
/// - `Result<(), EuvError>` - `Ok` when the command succeeded.
async fn run_command(mut command: Command, step: &str) -> Result<(), EuvError> {
    let status: ExitStatus = command
        .status()
        .await
        .map_err(|error: io::Error| EuvError::Io {
            message: format!("Failed to execute the {step}"),
            error,
        })?;
    if status.success() {
        return Ok(());
    }
    Err(EuvError::Message(format!(
        "The {step} failed with exit code {}",
        status.code().unwrap_or(-1)
    )))
}
