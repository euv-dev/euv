use super::*;

/// Dispatches a parsed `Cli` command and returns formatted output text.
///
/// # Arguments
///
/// - `&Cli` - Parsed CLI invocation.
///
/// # Returns
///
/// - `io::Result<String>` - Command output text or I/O execution error.
pub fn execute_cli(cli: &Cli) -> io::Result<String> {
    match cli.get_command() {
        Commands::Doctor { json } => {
            let report: DoctorReport = collect_doctor_report();
            if *json {
                serialize_json(&report)
            } else {
                Ok(format_doctor_report(&report))
            }
        }
        Commands::Build {
            target,
            render,
            release,
            plan,
        } => {
            let plans: Vec<BuildPlan> = resolve_build_plans(*target, *render, *release);
            if *plan {
                serialize_json(&plans)
            } else {
                for item in plans.iter() {
                    for step in item.get_steps().iter() {
                        run_external_command(step.get_program(), step.get_args())?;
                    }
                }
                serialize_json(&plans)
            }
        }
        Commands::Check {
            all_targets,
            target,
            package,
            plan,
        } => {
            let resolved_target: TargetArg = if *all_targets {
                TargetArg::All
            } else {
                *target
            };
            let steps: Vec<BuildStep> = resolve_check_steps(resolved_target, package.as_deref());
            if *plan {
                serialize_json(&steps)
            } else {
                for step in steps.iter() {
                    run_external_command(step.get_program(), step.get_args())?;
                }
                serialize_json(&steps)
            }
        }
        Commands::Run {
            target,
            render,
            release,
            package,
            plan,
        } => {
            let step: BuildStep = resolve_run_step(*target, *render, *release, package.as_deref());
            if *plan {
                serialize_json(&step)
            } else {
                run_external_command(step.get_program(), step.get_args())?;
                serialize_json(&step)
            }
        }
        Commands::Fmt { check } => {
            let mut args: Vec<String> = vec![ARG_FMT.to_string()];
            if *check {
                args.push(ARG_FMT_CHECK.to_string());
            }
            run_external_command(BIN_CARGO, &args)?;
            Ok(args.join(" "))
        }
        Commands::New { name, path } => {
            let base: PathBuf = match path {
                Some(p) => p.clone(),
                None => env::current_dir()?,
            };
            let report: ScaffoldReport = scaffold_project(&base, name)?;
            serialize_json(&report)
        }
    }
}

/// Inspects the local Rust toolchain, installed 7-platform targets, and SDKs.
///
/// # Returns
///
/// - `DoctorReport` - Structured toolchain readiness report.
#[must_use]
pub fn collect_doctor_report() -> DoctorReport {
    let rustc_version: String = probe_command(BIN_RUSTC, &[ARG_VERSION]);
    let cargo_version: String = probe_command(BIN_CARGO, &[ARG_VERSION]);
    let wasm_pack_version: String = probe_command(BIN_WASM_PACK, &[ARG_VERSION]);
    let apple_sdk_path: String = probe_command(BIN_XCRUN, &[ARG_SHOW_SDK]);
    let android_ndk_home: String =
        env::var(ENV_ANDROID_NDK).unwrap_or_else(|_: VarError| STATUS_MISSING.to_string());
    let installed_raw: String =
        probe_command(BIN_RUSTUP, &[ARG_TARGET_SUB, ARG_LIST, ARG_INSTALLED]);
    let installed_set: Vec<&str> = installed_raw.lines().map(str::trim).collect();
    let specs: Vec<TargetSpec> = resolve_target_specs(TargetArg::All);
    let targets: Vec<TargetReadiness> = specs
        .into_iter()
        .map(|spec: TargetSpec| {
            let present: bool = installed_set
                .iter()
                .any(|line: &&str| *line == spec.get_rust_triple());
            TargetReadiness::new(
                spec.get_name().clone(),
                spec.get_rust_triple().clone(),
                present,
            )
        })
        .collect();
    DoctorReport::new(
        rustc_version,
        cargo_version,
        wasm_pack_version,
        apple_sdk_path,
        android_ndk_home,
        targets,
    )
}

/// Builds deduplicated `cargo check --target <triple>` steps for `target`.
///
/// # Arguments
///
/// - `TargetArg` - Target platform (or `Native` / `All`).
/// - `Option<&str>` - Optional Cargo package name.
///
/// # Returns
///
/// - `Vec<BuildStep>` - Ordered check steps with duplicate target triples removed.
#[must_use]
pub fn resolve_check_steps(target: TargetArg, package: Option<&str>) -> Vec<BuildStep> {
    let specs: Vec<TargetSpec> = resolve_target_specs(target);
    let mut seen: Vec<String> = Vec::new();
    let mut steps: Vec<BuildStep> = Vec::new();
    for spec in specs.into_iter() {
        let triple: String = spec.get_rust_triple().clone();
        if seen.contains(&triple) {
            continue;
        }
        seen.push(triple.clone());
        let mut args: Vec<String> = vec![
            ARG_CHECK.to_string(),
            ARG_TARGET.to_string(),
            triple.clone(),
        ];
        if let Some(pkg) = package {
            args.push(ARG_PACKAGE.to_string());
            args.push(pkg.to_string());
        }
        steps.push(BuildStep::new(
            format!("Check {} ({})", spec.get_name(), triple),
            BIN_CARGO.to_string(),
            args,
        ));
    }
    steps
}

/// Builds a single `cargo run` step for `target` and `render`.
///
/// # Arguments
///
/// - `TargetArg` - Target platform to launch.
/// - `RenderArg` - Selected render pipeline mode.
/// - `bool` - Whether release mode is requested.
/// - `Option<&str>` - Optional Cargo package name.
///
/// # Returns
///
/// - `BuildStep` - Concrete run command step.
#[must_use]
pub fn resolve_run_step(
    target: TargetArg,
    render: RenderArg,
    release: bool,
    package: Option<&str>,
) -> BuildStep {
    let spec: TargetSpec = spec_for_platform(target);
    let mut args: Vec<String> = vec![ARG_RUN.to_string()];
    if let Some(pkg) = package {
        args.push(ARG_PACKAGE.to_string());
        args.push(pkg.to_string());
    }
    if release {
        args.push(ARG_RELEASE.to_string());
    }
    args.push(ARG_DOUBLE_DASH.to_string());
    args.push(render.as_flag().to_string());
    BuildStep::new(
        format!("Run {} ({})", spec.get_name(), spec.get_rust_triple()),
        BIN_CARGO.to_string(),
        args,
    )
}

/// Scaffolds a lean cross-platform `euv` project inside `parent_dir/<name>`.
///
/// # Arguments
///
/// - `&Path` - Parent directory where the project folder will be created.
/// - `&str` - Crate and folder name.
///
/// # Returns
///
/// - `io::Result<ScaffoldReport>` - List of created project files or I/O error.
pub fn scaffold_project(parent_dir: &Path, name: &str) -> io::Result<ScaffoldReport> {
    let project_dir: PathBuf = parent_dir.join(name);
    let src_dir: PathBuf = project_dir.join(SRC_DIR_NAME);
    fs::create_dir_all(&src_dir)?;
    let cargo_toml_path: PathBuf = project_dir.join(CARGO_TOML_NAME);
    let main_rs_path: PathBuf = src_dir.join(MAIN_RS_NAME);
    let readme_path: PathBuf = project_dir.join(README_NAME);
    let cargo_toml: String = format!(
        "[package]\nname = \"{name}\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\neuv = \"0.38.0\"\neuv-ui = \"0.38.0\"\n"
    );
    let readme: String = format!("# {name}{TEMPLATE_README_SUFFIX}");
    fs::write(&cargo_toml_path, cargo_toml)?;
    fs::write(&main_rs_path, TEMPLATE_MAIN_RS)?;
    fs::write(&readme_path, readme)?;
    Ok(ScaffoldReport::new(
        project_dir.display().to_string(),
        vec![
            cargo_toml_path.display().to_string(),
            main_rs_path.display().to_string(),
            readme_path.display().to_string(),
        ],
    ))
}

/// Formats a `DoctorReport` as human-readable terminal lines.
///
/// # Arguments
///
/// - `&DoctorReport` - Collected environment report.
///
/// # Returns
///
/// - `String` - Multi-line formatted summary.
#[must_use]
pub fn format_doctor_report(report: &DoctorReport) -> String {
    let mut out: String = format!(
        "rustc: {}\ncargo: {}\nwasm-pack: {}\napple-sdk: {}\nandroid-ndk: {}\ntargets:\n",
        report.get_rustc_version(),
        report.get_cargo_version(),
        report.get_wasm_pack_version(),
        report.get_apple_sdk_path(),
        report.get_android_ndk_home()
    );
    for t in report.get_targets().iter() {
        let mark: &str = if t.get_installed() {
            "ok"
        } else {
            STATUS_MISSING
        };
        out.push_str(&format!(
            "  - {:8} {:28} [{}]\n",
            t.get_name(),
            t.get_triple(),
            mark
        ));
    }
    out
}

/// Executes an external command and returns an I/O error if it exits non-zero.
///
/// # Arguments
///
/// - `&str` - Executable name.
/// - `&[String]` - Argument list.
///
/// # Returns
///
/// - `io::Result<()>` - Execution status.
fn run_external_command(program: &str, args: &[String]) -> io::Result<()> {
    let output: Output = Command::new(program).args(args).output()?;
    if output.status.success() {
        Ok(())
    } else {
        let stderr: String = String::from_utf8_lossy(&output.stderr).into_owned();
        Err(Error::other(format!("{ERR_STEP_FAILED}: {stderr}")))
    }
}

/// Probes an external CLI tool and returns its trimmed stdout or `"missing"`.
///
/// # Arguments
///
/// - `&str` - Executable name.
/// - `&[&str]` - Probe arguments.
///
/// # Returns
///
/// - `String` - Tool stdout or `"missing"`.
fn probe_command(program: &str, args: &[&str]) -> String {
    match Command::new(program).args(args).output() {
        Ok(out) if out.status.success() => String::from_utf8_lossy(&out.stdout).trim().to_string(),
        _ => STATUS_MISSING.to_string(),
    }
}

/// Pretty-prints any `Serialize` value to JSON.
///
/// # Arguments
///
/// - `&T` - Serializable report or plan.
///
/// # Returns
///
/// - `io::Result<String>` - Formatted JSON string.
fn serialize_json<T>(value: &T) -> io::Result<String>
where
    T: Serialize,
{
    match to_string_pretty(value) {
        Ok(text) => Ok(text),
        Err(err) => Err(Error::other(err.to_string())),
    }
}
