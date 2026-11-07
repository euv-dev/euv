use super::*;

fn build_args(argv: &[&str]) -> BuildArgs {
    let mut full: Vec<String> = vec!["euv".to_string(), "build".to_string()];
    for arg in argv {
        full.push((*arg).to_string());
    }
    let cli: Cli = Cli::parse_from(full);
    match cli.get_command() {
        Mode::Build(args) => args.clone(),
        other => panic!("expected Build, got {other:?}"),
    }
}

fn mode_args(argv: &[&str]) -> ModeArgs {
    let mut full: Vec<String> = vec!["euv".to_string()];
    for arg in argv {
        full.push((*arg).to_string());
    }
    ModeArgs::parse_from(full)
}

#[test]
fn a_plain_build_keeps_the_ordinary_web_only_arguments() {
    let args: BuildArgs = build_args(&["--release", "--crate-path", "./example"]);
    assert!(!args.get_all(), "the flag is false by default");
    assert!(
        args.get_platforms().is_empty(),
        "no platform is selected by default"
    );
    assert!(!args.get_plan(), "the plan is not requested by default");
    let rendered: String = format!("{:?}", args.get_mode());
    assert!(
        rendered.contains("./example"),
        "the crate path is kept, got {rendered}"
    );
    assert!(
        rendered.contains("release: true"),
        "the release flag is kept, got {rendered}"
    );
}

#[test]
fn the_all_flag_reaches_the_build_arguments() {
    let args: BuildArgs = build_args(&["--all"]);
    assert!(args.get_all(), "--all must select the whole host plan");
}

#[test]
fn the_platforms_flag_accepts_a_comma_separated_list() {
    let args: BuildArgs = build_args(&["--platforms", "macos,android"]);
    assert_eq!(
        args.get_platforms().as_slice(),
        &[NativePlatform::Macos, NativePlatform::Android],
        "both platforms are selected, in the order they were written"
    );
}

#[test]
fn the_platforms_flag_accepts_a_repeated_flag() {
    let args: BuildArgs = build_args(&["--platforms", "macos", "--platforms", "ios"]);
    assert_eq!(
        args.get_platforms().as_slice(),
        &[NativePlatform::Macos, NativePlatform::Ios],
        "a repeated flag extends the selection"
    );
}

#[test]
fn an_unknown_platform_name_is_rejected_by_the_parser() {
    let outcome: Result<Cli, clap::Error> =
        Cli::try_parse_from(["euv", "build", "--platforms", "bogus"]);
    assert!(outcome.is_err(), "an unknown platform must not be accepted");
}

#[test]
fn the_native_path_flag_reaches_the_build_arguments() {
    let args: BuildArgs = build_args(&["--native-path", "/tmp/client"]);
    assert_eq!(
        args.try_get_native_path().as_deref(),
        Some(Path::new("/tmp/client")),
        "an external native layout must be selectable"
    );
}

#[test]
fn the_plan_flag_reaches_the_build_arguments() {
    let args: BuildArgs = build_args(&["--plan"]);
    assert!(args.get_plan(), "--plan must reach the build arguments");
}

#[test]
fn the_all_and_platforms_flags_cannot_be_combined() {
    let outcome: Result<Vec<NativePlatform>, EuvError> =
        resolve_requested_platforms(true, &[NativePlatform::Macos]);
    assert!(
        outcome.is_err(),
        "the two selectors name the same thing twice"
    );
    let error: EuvError = outcome.expect_err("the combination is rejected");
    let rendered: String = format!("{error}");
    assert!(
        rendered.contains("--all") && rendered.contains("--platforms"),
        "the message must name both flags, got {rendered}"
    );
}

#[test]
fn no_native_flag_means_no_native_request() {
    let observed: Vec<NativePlatform> =
        resolve_requested_platforms(false, &[]).expect("an empty request resolves");
    assert!(
        observed.is_empty(),
        "an ordinary build asks for no native platform"
    );
}

#[test]
fn a_repeated_platform_is_selected_once() {
    let observed: Vec<NativePlatform> =
        resolve_requested_platforms(false, &[NativePlatform::Macos, NativePlatform::Macos])
            .expect("a repeated platform resolves");
    assert_eq!(
        observed,
        vec![NativePlatform::Macos],
        "the same platform twice is one deliverable, not two builds"
    );
}

#[test]
fn the_whole_host_plan_names_every_deliverable_that_host_can_build() {
    let macos: Vec<NativePlatform> = host_platforms(HostOs::Macos);
    assert_eq!(
        macos,
        vec![
            NativePlatform::Macos,
            NativePlatform::Ios,
            NativePlatform::IosSimulator,
            NativePlatform::Android
        ],
        "a macOS host builds the Apple clients and the Android package"
    );
    let linux: Vec<NativePlatform> = host_platforms(HostOs::Linux);
    assert_eq!(
        linux,
        vec![NativePlatform::Linux, NativePlatform::Android],
        "a Linux host builds the Linux client and the Android package"
    );
    let windows: Vec<NativePlatform> = host_platforms(HostOs::Windows);
    assert_eq!(
        windows,
        vec![NativePlatform::Windows],
        "a Windows host builds the Windows client only"
    );
}

#[test]
fn the_android_builder_runs_on_both_hosts_that_support_it() {
    assert!(host_supports(HostOs::Macos, NativePlatform::Android));
    assert!(host_supports(HostOs::Linux, NativePlatform::Android));
    assert!(
        !host_supports(HostOs::Windows, NativePlatform::Android),
        "the Android builder has no Windows host"
    );
}

#[test]
fn every_apple_deliverable_requires_a_macos_host() {
    for platform in [
        NativePlatform::Macos,
        NativePlatform::Ios,
        NativePlatform::IosSimulator,
    ] {
        assert!(
            host_supports(HostOs::Macos, platform),
            "{platform:?} must be buildable on macOS"
        );
        assert!(
            !host_supports(HostOs::Linux, platform),
            "{platform:?} must not be buildable on Linux"
        );
        assert!(
            !host_supports(HostOs::Windows, platform),
            "{platform:?} must not be buildable on Windows"
        );
    }
}

#[test]
fn the_desktop_clients_each_require_their_own_host() {
    assert!(host_supports(HostOs::Linux, NativePlatform::Linux));
    assert!(!host_supports(HostOs::Macos, NativePlatform::Linux));
    assert!(host_supports(HostOs::Windows, NativePlatform::Windows));
    assert!(!host_supports(HostOs::Macos, NativePlatform::Windows));
}

#[test]
fn a_host_compatible_plan_names_its_builders_in_build_order() {
    let plan: NativePlan = resolve_native_plan(&host_platforms(HostOs::Macos), HostOs::Macos)
        .expect("a macOS host builds every requested platform");
    assert_eq!(plan.get_targets().len(), 4, "every deliverable is planned");
    assert_eq!(
        plan.get_targets()[0].platform,
        NativePlatform::Macos,
        "the desktop client is built first"
    );
    assert_eq!(
        plan.get_targets()[3].platform,
        NativePlatform::Android,
        "the Android package is built last"
    );
    assert_eq!(
        plan.get_targets()[3].task,
        NativeTask::Android,
        "the Android package is built by the Android builder"
    );
    assert_eq!(
        plan.get_targets()[0].task,
        NativeTask::Client,
        "the desktop client is built by the client builder"
    );
    assert!(plan.get_unsupported().is_empty(), "nothing is out of reach");
}

#[test]
fn an_incompatible_platform_fails_the_plan_before_any_build() {
    let outcome: Result<NativePlan, EuvError> =
        resolve_native_plan(&[NativePlatform::Windows], HostOs::Macos);
    let error: EuvError = outcome.expect_err("a Windows client cannot be built on macOS");
    let rendered: String = format!("{error}");
    assert!(
        rendered.contains("windows") && rendered.contains("macos"),
        "the message must name the platform and the host, got {rendered}"
    );
    assert!(
        rendered.contains("windows host"),
        "the message must say which host the platform needs, got {rendered}"
    );
}

#[test]
fn every_incompatible_platform_is_reported_together() {
    let outcome: Result<NativePlan, EuvError> = resolve_native_plan(
        &[NativePlatform::Linux, NativePlatform::Windows],
        HostOs::Macos,
    );
    let error: EuvError = outcome.expect_err("neither platform can be built here");
    let rendered: String = format!("{error}");
    assert!(
        rendered.contains("linux") && rendered.contains("windows"),
        "both unreachable platforms must be named, got {rendered}"
    );
}

#[test]
fn the_run_mode_rejects_the_native_only_flags() {
    for flag in ["--all", "--platforms", "--native-path", "--plan"] {
        let args: ModeArgs = mode_args(&["--", flag]);
        let outcome: Result<(), EuvError> = reject_native_flags_in_run(&args);
        assert!(
            outcome.is_err(),
            "{flag} selects a native build the run mode cannot do"
        );
        let error: EuvError = outcome.expect_err("the flag is rejected");
        let rendered: String = format!("{error}");
        assert!(
            rendered.contains(flag),
            "the message must name the flag the user typed, got {rendered}"
        );
    }
}

#[test]
fn the_run_mode_accepts_the_ordinary_arguments() {
    let args: ModeArgs = mode_args(&["--", "--target", "web", "--out-dir", "dist"]);
    let outcome: Result<(), EuvError> = reject_native_flags_in_run(&args);
    assert!(
        outcome.is_ok(),
        "a wasm-pack passthrough is not a native flag"
    );
}

#[test]
fn the_native_project_directory_defaults_to_the_repository_layout() {
    let base: PathBuf = PathBuf::from("/repo");
    let observed: PathBuf = resolve_native_project(&base, None);
    assert_eq!(
        observed,
        PathBuf::from("/repo/native/tauri"),
        "the default layout is the native client project inside the repository"
    );
}

#[test]
fn an_explicit_native_path_overrides_the_repository_layout() {
    let base: PathBuf = PathBuf::from("/repo");
    let override_path: PathBuf = PathBuf::from("/elsewhere/client");
    let observed: PathBuf = resolve_native_project(&base, Some(&override_path));
    assert_eq!(
        observed, override_path,
        "an external layout must be selectable without changing the repository"
    );
}

#[test]
fn a_relative_native_path_is_resolved_against_the_invocation_directory() {
    let base: PathBuf = PathBuf::from("/repo");
    let relative: PathBuf = PathBuf::from("clients/app");
    assert_eq!(
        resolve_native_project(&base, Some(&relative)),
        base.join(relative)
    );
}

#[test]
fn native_packaging_defaults_to_release() {
    let mut args: ModeArgs = mode_args(&[]);
    prepare_native_profile(&mut args).expect("default profile is valid");
    assert!(args.get_release());
}

#[test]
fn native_packaging_rejects_debug_and_profiling_before_building() {
    for flag in ["--dev", "--profiling"] {
        let mut args: ModeArgs = mode_args(&[flag]);
        assert!(prepare_native_profile(&mut args).is_err());
    }
}

#[test]
fn unsupported_hosts_are_not_treated_as_macos() {
    assert!(resolve_native_plan(&[], HostOs::Unsupported).is_err());
    assert!(!host_supports(HostOs::Unsupported, NativePlatform::Macos));
}

#[cfg(unix)]
#[tokio::test]
async fn a_native_builder_failure_stops_later_targets() {
    let root: PathBuf = temp_dir().join(format!("euv-native-failure-{}", process::id()));
    fs::create_dir_all(root.join("scripts")).expect("create fake builder directory");
    fs::write(root.join("build.sh"), "exit 17").expect("write failing test builder");
    fs::write(
        root.join("scripts/build-client.js"),
        r#"require('node:fs').writeFileSync('unexpected', 'ran')"#,
    )
    .expect("write later test builder");
    let plan: NativePlan = resolve_native_plan(
        &[NativePlatform::Android, NativePlatform::Macos],
        HostOs::Macos,
    )
    .expect("resolve test plan");
    let error: EuvError = run_native_builds(&plan, &root, &root.join("web"), "release")
        .await
        .expect_err("native failure must propagate");
    assert!(error.to_string().contains("17"));
    assert!(!root.join("unexpected").exists());
    fs::remove_dir_all(root).expect("remove test builders");
}

#[cfg(unix)]
#[tokio::test]
async fn the_native_builder_receives_the_exact_web_snapshot_and_release_mode() {
    let root: PathBuf = temp_dir().join(format!("euv-native-snapshot-{}", process::id()));
    fs::create_dir_all(&root).expect("create test builder directory");
    fs::write(
        root.join("build.sh"),
        r#"printf '%s\n' "$EUV_WEB_DIST" "$1" "$2" > received"#,
    )
    .expect("write snapshot test builder");
    let web: PathBuf = root.join("selected web output");
    let plan: NativePlan = resolve_native_plan(&[NativePlatform::Android], HostOs::Macos)
        .expect("resolve snapshot plan");
    run_native_builds(&plan, &root, &web, "release")
        .await
        .expect("run test builder");
    assert_eq!(
        fs::read_to_string(root.join("received")).expect("read builder input"),
        format!("{}\nandroid\nrelease\n", web.display())
    );
    fs::remove_dir_all(root).expect("remove test builder");
}

#[test]
fn a_missing_native_project_directory_is_reported_when_the_build_starts() {
    let missing: PathBuf = PathBuf::from("/definitely/not/a/repository");
    let outcome: Result<(), EuvError> = require_native_project(&missing);
    assert!(
        outcome.is_err(),
        "a directory that does not exist cannot be built in"
    );
    let error: EuvError = outcome.expect_err("the missing directory is reported");
    let rendered: String = format!("{error}");
    assert!(
        rendered.contains("--native-path"),
        "the message must point at the flag that fixes it, got {rendered}"
    );
}
