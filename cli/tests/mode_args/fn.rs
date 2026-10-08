use super::*;

fn mode_args(argv: &[&str]) -> ModeArgs {
    let mut full: Vec<String> = vec!["euv".to_string()];
    for arg in argv {
        full.push((*arg).to_string());
    }
    ModeArgs::parse_from(full)
}

fn debug_of(args: &ModeArgs) -> String {
    format!("{args:?}")
}

fn field(args: &ModeArgs, name: &str) -> String {
    let rendered: String = debug_of(args);
    let needle: String = format!("{name}: ");
    let start: usize = match rendered.find(&needle) {
        Some(value) => value + needle.len(),
        None => return String::new(),
    };
    let rest: &str = &rendered[start..];
    if rest.starts_with('[') {
        return match rest.find(']') {
            Some(value) => rest[..=value].trim().to_string(),
            None => rest.trim().to_string(),
        };
    }
    let end: usize = match rest.find([',', '}']) {
        Some(value) => value,
        None => rest.len(),
    };
    rest[..end].trim().to_string()
}

#[test]
fn an_empty_argv_takes_every_documented_default() {
    let args: ModeArgs = mode_args(&[]);
    assert_eq!(
        field(&args, "crate_path"),
        "\".\"",
        "the crate path defaults to the working directory"
    );
    assert_eq!(
        field(&args, "port"),
        "80",
        "the dev server defaults to port 80"
    );
    assert_eq!(
        field(&args, "www_dir"),
        "\"www\"",
        "the asset dir defaults to www"
    );
    assert_eq!(
        field(&args, "index_html"),
        "None",
        "no index override by default"
    );
}

#[test]
fn no_build_flag_means_no_build_mode_requested() {
    let args: ModeArgs = mode_args(&[]);
    assert_eq!(field(&args, "dev"), "false", "dev is not implied");
    assert_eq!(field(&args, "release"), "false", "release is not implied");
    assert_eq!(
        field(&args, "profiling"),
        "false",
        "profiling is not implied"
    );
    assert_eq!(
        field(&args, "no_gitignore"),
        "false",
        "the gitignore is kept by default"
    );
}

#[test]
fn the_crate_path_flag_overrides_the_default() {
    let args: ModeArgs = mode_args(&["--crate-path", "app"]);
    assert_eq!(field(&args, "crate_path"), "\"app\"", "the long flag wins");
}

#[test]
fn the_short_crate_path_flag_is_accepted() {
    let args: ModeArgs = mode_args(&["-c", "app"]);
    assert_eq!(
        field(&args, "crate_path"),
        "\"app\"",
        "the short form reaches the same field"
    );
}

#[test]
fn the_port_flag_overrides_the_default() {
    let args: ModeArgs = mode_args(&["--port", "3000"]);
    assert_eq!(field(&args, "port"), "3000", "the port flag wins");
}

#[test]
fn the_www_dir_flag_overrides_the_default() {
    let args: ModeArgs = mode_args(&["--www-dir", "assets"]);
    assert_eq!(
        field(&args, "www_dir"),
        "\"assets\"",
        "the www dir flag wins"
    );
}

#[test]
fn the_index_html_flag_records_an_override() {
    let args: ModeArgs = mode_args(&["--index-html", "start.html"]);
    assert!(
        field(&args, "index_html").contains("start.html"),
        "the index override is recorded when supplied, got {}",
        field(&args, "index_html")
    );
}

#[test]
fn each_build_flag_sets_exactly_its_own_booleans() {
    let dev: ModeArgs = mode_args(&["--dev"]);
    let release: ModeArgs = mode_args(&["--release"]);
    let profiling: ModeArgs = mode_args(&["--profiling"]);
    assert_eq!(field(&dev, "dev"), "true", "--dev sets dev alone");
    assert_eq!(
        field(&dev, "release"),
        "false",
        "--dev leaves release alone"
    );
    assert_eq!(
        field(&release, "release"),
        "true",
        "--release sets release alone"
    );
    assert_eq!(
        field(&profiling, "profiling"),
        "true",
        "--profiling sets profiling alone"
    );
    assert_eq!(
        field(&profiling, "dev"),
        "false",
        "--profiling leaves dev alone"
    );
}

#[test]
fn the_no_gitignore_flag_is_recorded() {
    let args: ModeArgs = mode_args(&["--no-gitignore"]);
    assert_eq!(
        field(&args, "no_gitignore"),
        "true",
        "the flag turns the behaviour off"
    );
}

#[test]
fn trailing_wasm_pack_args_are_captured_verbatim() {
    let args: ModeArgs = mode_args(&["--", "--target", "web"]);
    let rendered: String = field(&args, "wasm_pack_args");
    assert!(
        rendered.contains("--target") && rendered.contains("web"),
        "everything past the separator is forwarded, in order, got {rendered}"
    );
    assert!(
        !rendered.contains("--dev"),
        "euv's own flags never leak into the wasm-pack list, got {rendered}"
    );
}

#[test]
fn without_a_separator_the_wasm_pack_args_are_empty() {
    let args: ModeArgs = mode_args(&["--dev"]);
    assert_eq!(
        field(&args, "wasm_pack_args"),
        "[]",
        "with no separator there is nothing to forward"
    );
}

#[test]
fn no_build_flag_resolves_to_the_dev_build() {
    let args: ModeArgs = mode_args(&[]);
    assert_eq!(
        resolve_build_mode(&args),
        BuildMode::Dev,
        "an unqualified invocation builds in dev mode"
    );
}

#[test]
fn the_release_flag_resolves_to_the_release_build() {
    let args: ModeArgs = mode_args(&["--release"]);
    assert_eq!(
        resolve_build_mode(&args),
        BuildMode::Release,
        "--release wins"
    );
}

#[test]
fn the_profiling_flag_resolves_to_the_profiling_build() {
    let args: ModeArgs = mode_args(&["--profiling"]);
    assert_eq!(
        resolve_build_mode(&args),
        BuildMode::Profiling,
        "--profiling wins"
    );
}

#[test]
fn profiling_outranks_release_when_both_are_given() {
    let args: ModeArgs = mode_args(&["--release", "--profiling"]);
    assert_eq!(
        resolve_build_mode(&args),
        BuildMode::Profiling,
        "profiling is checked first, so it wins the tie"
    );
}

#[test]
fn release_outranks_dev_when_both_are_given() {
    let args: ModeArgs = mode_args(&["--dev", "--release"]);
    assert_eq!(
        resolve_build_mode(&args),
        BuildMode::Release,
        "release is checked before dev, so it wins the tie"
    );
}

#[test]
fn a_release_flag_inside_the_wasm_pack_args_still_resolves() {
    let args: ModeArgs = mode_args(&["--", "--release"]);
    assert_eq!(
        resolve_build_mode(&args),
        BuildMode::Release,
        "a build flag forwarded past the separator is still honoured"
    );
}

#[test]
fn a_profiling_flag_inside_the_wasm_pack_args_still_resolves() {
    let args: ModeArgs = mode_args(&["--", "--profiling"]);
    assert_eq!(
        resolve_build_mode(&args),
        BuildMode::Profiling,
        "the wasm-pack side is consulted too"
    );
}

#[test]
fn a_parsed_argv_round_trips_through_the_out_name_resolver() {
    let args: ModeArgs = mode_args(&["--crate-path", "."]);
    let observed: String = resolve_out_name(&args);
    assert!(
        observed.ends_with(".js"),
        "the out name is always a js bundle name, got {observed}"
    );
}

#[test]
fn the_out_dir_defaults_under_the_asset_dir() {
    let args: ModeArgs = mode_args(&[]);
    let observed: PathBuf = resolve_out_dir(&args);
    assert!(
        observed.ends_with("www/pkg"),
        "with no --out-dir the bundle lands under www/pkg, got {observed:?}"
    );
}

#[test]
fn the_out_dir_flag_moves_the_bundle() {
    let args: ModeArgs = mode_args(&["--", "--out-dir", "dist"]);
    let observed: PathBuf = resolve_out_dir(&args);
    assert!(
        observed.ends_with("dist"),
        "an explicit --out-dir is honoured, got {observed:?}"
    );
}

#[test]
fn a_relative_out_dir_is_anchored_to_the_crate_path() {
    let args: ModeArgs = mode_args(&["--crate-path", "app", "--", "--out-dir", "dist"]);
    let observed: PathBuf = resolve_out_dir(&args);
    assert_eq!(
        observed,
        PathBuf::from("app/dist"),
        "a relative out dir is resolved against the crate, not the cwd"
    );
}

#[test]
fn an_absolute_out_dir_is_left_alone() {
    let args: ModeArgs = mode_args(&["--crate-path", "app", "--", "--out-dir", "/tmp/bundle"]);
    let observed: PathBuf = resolve_out_dir(&args);
    assert_eq!(
        observed,
        PathBuf::from("/tmp/bundle"),
        "an absolute path is not re-anchored"
    );
}

#[test]
fn a_www_dir_inside_the_crate_yields_a_relative_route_prefix() {
    let args: ModeArgs = mode_args(&["--crate-path", "app"]);
    let observed: String = resolve_serving_route_prefix(&args);
    assert_eq!(
        observed, "www",
        "the default www bundle under the crate serves from the asset dir itself"
    );
}

#[test]
fn an_out_dir_outside_www_yields_its_parent_as_the_prefix() {
    let args: ModeArgs = mode_args(&["--crate-path", "app", "--", "--out-dir", "dist/pkg"]);
    let observed: String = resolve_serving_route_prefix(&args);
    assert_eq!(
        observed, "dist",
        "serving from outside www means serving the out dir's parent"
    );
}

#[test]
fn the_route_prefix_is_empty_when_the_out_dir_is_at_the_crate_root() {
    let args: ModeArgs = mode_args(&["--crate-path", "app", "--", "--out-dir", "pkg"]);
    let observed: String = resolve_serving_route_prefix(&args);
    assert_eq!(
        observed, "",
        "a bundle sitting at the crate root is served from the root itself"
    );
}

#[test]
fn reconcile_promotes_a_forwarded_crate_path_onto_the_field() {
    let mut args: ModeArgs = mode_args(&["--", "--crate-path", "app"]);
    reconcile_args(&mut args);
    assert_eq!(
        field(&args, "crate_path"),
        "\"app\"",
        "a value past the separator wins over the clap default"
    );
}

#[test]
fn reconcile_accepts_the_equals_form_of_a_forwarded_flag() {
    let mut args: ModeArgs = mode_args(&["--", "--crate-path=app"]);
    reconcile_args(&mut args);
    assert_eq!(
        field(&args, "crate_path"),
        "\"app\"",
        "the = form is recognised here, unlike in filter_euv_args"
    );
}

#[test]
fn reconcile_ignores_a_port_that_is_not_a_number() {
    let mut args: ModeArgs = mode_args(&["--", "--port", "not-a-port"]);
    reconcile_args(&mut args);
    assert_eq!(
        field(&args, "port"),
        "80",
        "an unparseable port leaves the default alone rather than zeroing it"
    );
}

#[test]
fn reconcile_ignores_a_equals_port_that_is_not_a_number() {
    let mut args: ModeArgs = mode_args(&["--", "--port=not-a-port"]);
    reconcile_args(&mut args);
    assert_eq!(
        field(&args, "port"),
        "80",
        "the = form is equally guarded against a bad value"
    );
}

#[test]
fn reconcile_promotes_the_forwarded_build_flags() {
    let mut args: ModeArgs = mode_args(&["--", "--release", "--no-gitignore"]);
    reconcile_args(&mut args);
    assert_eq!(field(&args, "release"), "true", "--release is promoted");
    assert_eq!(
        field(&args, "no_gitignore"),
        "true",
        "--no-gitignore is promoted"
    );
}

#[test]
fn reconcile_leaves_unmentioned_fields_at_their_clap_defaults() {
    let mut args: ModeArgs = mode_args(&["--", "--target", "web"]);
    reconcile_args(&mut args);
    assert_eq!(
        field(&args, "crate_path"),
        "\".\"",
        "an unrelated wasm-pack arg must not disturb the euv fields"
    );
    assert_eq!(field(&args, "port"), "80", "nor the port");
}

#[test]
fn reconcile_applies_the_last_value_when_a_flag_is_repeated() {
    let mut args: ModeArgs = mode_args(&["--", "--crate-path", "first", "--crate-path", "second"]);
    reconcile_args(&mut args);
    assert_eq!(
        field(&args, "crate_path"),
        "\"second\"",
        "a repeated flag resolves to its last occurrence"
    );
}

#[test]
fn a_reconciled_argv_drives_the_build_mode_the_same_way_as_the_direct_flag() {
    let mut via_reconcile: ModeArgs = mode_args(&["--", "--release"]);
    reconcile_args(&mut via_reconcile);
    let direct: ModeArgs = mode_args(&["--release"]);
    assert_eq!(
        resolve_build_mode(&via_reconcile),
        resolve_build_mode(&direct),
        "a forwarded build flag must resolve identically to a direct one"
    );
}

#[test]
fn a_reconciled_port_feeds_the_out_name_resolver() {
    let mut args: ModeArgs = mode_args(&["--crate-path", ".", "--", "--port", "9000"]);
    reconcile_args(&mut args);
    assert_eq!(
        field(&args, "port"),
        "9000",
        "the reconciled port is stored"
    );
    let observed: String = resolve_out_name(&args);
    assert!(
        observed.ends_with(".js"),
        "and the resolver still produces a bundle name, got {observed}"
    );
}
