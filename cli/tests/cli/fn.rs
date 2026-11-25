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

#[test]
fn cli_web_bundler_emits_zero_js_files_and_valid_html_wasm_bundle() {
    let temp_dir: PathBuf = env::temp_dir().join("euv_cli_zero_js_web_test");
    drop(fs::remove_dir_all(&temp_dir));
    fs::create_dir_all(&temp_dir).unwrap();
    fs::write(temp_dir.join("app_bg.wasm"), b"\0asm\x01\0\0\0").unwrap();
    fs::write(
        temp_dir.join("app.js"),
        "export function helper() {}\nasync function __wbg_init(p) { const u = new URL('app_bg.wasm', import.meta.url); return u; }\nexport { __wbg_init as default };\n",
    )
    .unwrap();
    fs::write(
        temp_dir.join("app.d.ts"),
        "export function helper(): void;\n",
    )
    .unwrap();
    fs::write(temp_dir.join("package.json"), "{}").unwrap();
    let index_path: PathBuf = package_zero_js_web_bundle(&temp_dir, "Zero JS App").unwrap();
    assert!(index_path.exists());
    let html: String = fs::read_to_string(&index_path).unwrap();
    assert!(html.contains("<title>Zero JS App</title>"));
    assert!(html.contains("__wbg_init('./app_bg.wasm')"));
    assert!(!html.contains("import.meta.url"));
    assert!(!html.contains("export {"));
    let mut files: Vec<String> = Vec::new();
    for entry_res in fs::read_dir(&temp_dir).unwrap() {
        let entry: fs::DirEntry = entry_res.unwrap();
        files.push(entry.file_name().to_string_lossy().into_owned());
    }
    assert!(
        files
            .iter()
            .all(|f: &String| !f.ends_with(".js") && !f.ends_with(".d.ts"))
    );
    assert!(files.contains(&"index.html".to_string()));
    assert!(files.contains(&"app_bg.wasm".to_string()));
    assert_eq!(files.len(), 2);
    drop(fs::remove_dir_all(&temp_dir));
}
