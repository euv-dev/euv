use super::*;

#[test]
fn cli_parses_all_subcommands_and_resolves_seven_platform_plans() {
    let parsed_build: Cli = Cli::try_parse_from([
        "euv",
        "build",
        "--target",
        "all",
        "--render",
        "hybrid",
        "--release",
        "--plan",
    ])
    .unwrap();
    let json: String = execute_cli(&parsed_build).unwrap();
    assert!(json.contains("wasm32-unknown-unknown"));
    assert!(json.contains("aarch64-linux-android"));
    assert!(json.contains("aarch64-apple-ios"));
    assert!(json.contains("aarch64-apple-darwin"));
    assert!(json.contains("x86_64-pc-windows-msvc"));
    assert!(json.contains("x86_64-unknown-linux-gnu"));
    let plans: Vec<BuildPlan> = resolve_build_plans(TargetArg::All, RenderArg::Hybrid, true);
    assert_eq!(plans.len(), 7);
    let check_steps: Vec<BuildStep> = resolve_check_steps(TargetArg::All, Some("euv-core"));
    assert_eq!(check_steps.len(), 6);
    let run_step: BuildStep = resolve_run_step(
        TargetArg::Macos,
        RenderArg::Software,
        false,
        Some("euv-example"),
    );
    assert!(
        run_step
            .get_args()
            .iter()
            .any(|a: &String| a == "--software")
    );
}

#[test]
fn cli_doctor_and_project_scaffolding_work_end_to_end() {
    let report: DoctorReport = collect_doctor_report();
    assert_eq!(report.get_targets().len(), 7);
    assert!(report.get_rustc_version().contains("rustc"));
    let formatted: String = format_doctor_report(&report);
    assert!(formatted.contains("macos"));
    let temp_root: PathBuf = env::temp_dir().join("euv_cli_scaffold_test");
    drop(fs::remove_dir_all(&temp_root));
    fs::create_dir_all(&temp_root).unwrap();
    let scaffold: ScaffoldReport = scaffold_project(&temp_root, "demo-app").unwrap();
    assert_eq!(scaffold.get_files().len(), 3);
    for file in scaffold.get_files().iter() {
        assert!(Path::new(file).exists());
    }
    drop(fs::remove_dir_all(&temp_root));
}
