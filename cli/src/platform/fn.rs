use super::*;

/// Resolves the host's native `TargetArg`.
///
/// # Returns
///
/// - `TargetArg` - Host platform target argument.
#[must_use]
pub const fn host_native_target() -> TargetArg {
    if cfg!(target_os = "macos") {
        TargetArg::Macos
    } else if cfg!(target_os = "windows") {
        TargetArg::Windows
    } else if cfg!(target_os = "linux") {
        TargetArg::Linux
    } else if cfg!(target_os = "ios") {
        TargetArg::Ios
    } else if cfg!(target_os = "android") {
        TargetArg::Android
    } else {
        TargetArg::Web
    }
}

/// Resolves the preferred Rust target triple for `target`, adapting native desktop triples to host CPU architecture.
///
/// # Arguments
///
/// - `TargetArg` - Concrete platform target.
///
/// # Returns
///
/// - `&'static str` - Host-aware Rust target triple.
#[must_use]
pub const fn target_triple_for_host(target: TargetArg) -> &'static str {
    match target {
        TargetArg::Web => TRIPLE_WEB,
        TargetArg::Android => TRIPLE_ANDROID,
        TargetArg::Ios | TargetArg::Ipad => TRIPLE_IOS,
        TargetArg::Macos => HOST_MACOS_TRIPLE,
        TargetArg::Windows => HOST_WINDOWS_TRIPLE,
        TargetArg::Linux => HOST_LINUX_TRIPLE,
        TargetArg::Native | TargetArg::All => target_triple_for_host(host_native_target()),
    }
}

/// Returns the canonical `TargetSpec` list for a CLI `TargetArg`.
///
/// # Arguments
///
/// - `TargetArg` - Requested platform or `Native`/`All`.
///
/// # Returns
///
/// - `Vec<TargetSpec>` - One or seven resolved platform target specifications.
#[must_use]
pub fn resolve_target_specs(target: TargetArg) -> Vec<TargetSpec> {
    match target {
        TargetArg::All => vec![
            spec_for_platform(TargetArg::Web),
            spec_for_platform(TargetArg::Android),
            spec_for_platform(TargetArg::Ios),
            spec_for_platform(TargetArg::Ipad),
            spec_for_platform(TargetArg::Macos),
            spec_for_platform(TargetArg::Windows),
            spec_for_platform(TargetArg::Linux),
        ],
        TargetArg::Native => vec![spec_for_platform(host_native_target())],
        other => vec![spec_for_platform(other)],
    }
}

/// Returns the `TargetSpec` for a single concrete platform target.
///
/// # Arguments
///
/// - `TargetArg` - Concrete platform target.
///
/// # Returns
///
/// - `TargetSpec` - Platform specification with triple, system APIs, and GPU backend.
#[must_use]
pub fn spec_for_platform(target: TargetArg) -> TargetSpec {
    let triple: &str = target_triple_for_host(target);
    let (name, api, artifact, kind): (&str, &str, &str, PlatformKind) = match target {
        TargetArg::Web => (NAME_WEB, API_WEB, ARTIFACT_WEB, PlatformKind::Web),
        TargetArg::Android => (
            NAME_ANDROID,
            API_ANDROID,
            ARTIFACT_ANDROID,
            PlatformKind::Android,
        ),
        TargetArg::Ios => (NAME_IOS, API_IOS, ARTIFACT_IOS, PlatformKind::Ios),
        TargetArg::Ipad => (NAME_IPAD, API_IPAD, ARTIFACT_IPAD, PlatformKind::IpadOs),
        TargetArg::Macos => (NAME_MACOS, API_MACOS, ARTIFACT_MACOS, PlatformKind::MacOs),
        TargetArg::Windows => (
            NAME_WINDOWS,
            API_WINDOWS,
            ARTIFACT_WINDOWS,
            PlatformKind::Windows,
        ),
        TargetArg::Linux => (NAME_LINUX, API_LINUX, ARTIFACT_LINUX, PlatformKind::Linux),
        TargetArg::Native | TargetArg::All => return spec_for_platform(host_native_target()),
    };
    let caps: PlatformCapabilities = PlatformCapabilities::for_platform(kind);
    let gpu: GraphicsBackend =
        Renderer::new(kind, caps, 1, 1, RenderConfig::hardware()).get_active_backend();
    TargetSpec::new(
        name.to_string(),
        triple.to_string(),
        api.to_string(),
        artifact.to_string(),
        format!("{kind:?}"),
        format!("{gpu:?}"),
    )
}

/// Generates declarative cross-platform build plans for `target`.
///
/// # Arguments
///
/// - `TargetArg` - Target platform (or `Native` / `All`).
/// - `RenderArg` - Default render pipeline mode.
/// - `bool` - Whether release optimization is enabled.
///
/// # Returns
///
/// - `Vec<BuildPlan>` - Resolved build plans for every selected target.
#[must_use]
pub fn resolve_build_plans(target: TargetArg, render: RenderArg, release: bool) -> Vec<BuildPlan> {
    let specs: Vec<TargetSpec> = resolve_target_specs(target);
    let mode: RenderMode = render.to_render_mode();
    let mode_label: String = format!("{mode:?}");
    specs
        .into_iter()
        .map(|spec: TargetSpec| {
            let mut steps: Vec<BuildStep> = Vec::new();
            if spec.get_name() == NAME_WEB {
                let mut wp_args: Vec<String> = vec![
                    ARG_BUILD.to_string(),
                    ARG_TARGET.to_string(),
                    ARG_TARGET_WEB.to_string(),
                    ARG_OUT_DIR.to_string(),
                    OUT_DIR_WEB.to_string(),
                ];
                if release {
                    wp_args.push(ARG_RELEASE.to_string());
                }
                steps.push(BuildStep::new(
                    STEP_PACKAGE_DESC.to_string(),
                    BIN_WASM_PACK.to_string(),
                    wp_args,
                ));
            } else {
                let mut cargo_args: Vec<String> = vec![
                    ARG_BUILD.to_string(),
                    ARG_TARGET.to_string(),
                    spec.get_rust_triple().clone(),
                ];
                if release {
                    cargo_args.push(ARG_RELEASE.to_string());
                }
                steps.push(BuildStep::new(
                    STEP_COMPILE_DESC.to_string(),
                    BIN_CARGO.to_string(),
                    cargo_args,
                ));
            }
            BuildPlan::new(spec, mode_label.clone(), release, steps)
        })
        .collect()
}
