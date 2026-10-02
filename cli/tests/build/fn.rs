use super::*;

use clap::Parser;

use std::path::PathBuf;

fn args_from(extra: &[&str]) -> ModeArgs {
    let mut argv: Vec<&str> = vec!["euv"];
    argv.extend_from_slice(extra);
    ModeArgs::parse_from(argv)
}

fn owned(items: &[&str]) -> Vec<String> {
    items.iter().map(|s: &&str| s.to_string()).collect()
}

#[test]
fn a_build_mode_flag_is_detected_by_any_of_its_three_spellings() {
    assert!(has_build_mode_flag(&owned(&[DEV_FLAG])));
    assert!(has_build_mode_flag(&owned(&[RELEASE_FLAG])));
    assert!(has_build_mode_flag(&owned(&[PROFILING_FLAG])));
}

#[test]
fn a_list_without_a_build_mode_flag_reports_none() {
    assert!(!has_build_mode_flag(&owned(&[])));
    assert!(!has_build_mode_flag(&owned(&["--out-dir", "pkg", "--target", "web"])));
    assert!(
        !has_build_mode_flag(&owned(&["--developer"])),
        "a flag that merely starts with --dev is not --dev"
    );
}

#[test]
fn filtering_drops_each_euv_flag_together_with_the_token_after_it() {
    let filtered: Vec<String> = filter_euv_args(&owned(&[
        "--target",
        "web",
        PORT_ARG,
        "8080",
        CRATE_PATH_ARG,
        "/tmp/crate",
    ]));
    assert_eq!(
        filtered,
        vec!["--target".to_string(), "web".to_string()],
        "a detached euv flag takes the next token with it"
    );
}

#[test]
fn filtering_lets_the_wasm_pack_flags_that_look_similar_through() {
    let filtered: Vec<String> = filter_euv_args(&owned(&[OUT_DIR_ARG, "pkg", OUT_NAME_ARG, "game"]));
    assert_eq!(
        filtered,
        vec![
            OUT_DIR_ARG.to_string(),
            "pkg".to_string(),
            OUT_NAME_ARG.to_string(),
            "game".to_string()
        ],
        "--out-dir and --out-name are wasm-pack's own, not euv's, so they must pass through"
    );
}

#[test]
fn filtering_skips_the_token_after_an_euv_flag_even_when_it_looks_like_another_flag() {
    let filtered: Vec<String> = filter_euv_args(&owned(&["--features", PORT_ARG, RELEASE_FLAG]));
    assert_eq!(
        filtered,
        vec!["--features".to_string()],
        "the skip is unconditional: a flag following an euv flag is eaten as its value"
    );
}

#[test]
fn filtering_keeps_an_inline_assignment_verbatim_because_the_match_is_exact() {
    let filtered: Vec<String> = filter_euv_args(&owned(&[&format!("{PORT_ARG}=8080"), RELEASE_FLAG]));
    assert_eq!(
        filtered,
        vec![format!("{PORT_ARG}=8080"), RELEASE_FLAG.to_string()],
        "EUV_ARGS holds the bare flag, so `--port=8080` never matches and reaches wasm-pack"
    );
}

#[test]
fn filtering_only_considers_the_arguments_after_the_last_separator() {
    let filtered: Vec<String> = filter_euv_args(&owned(&[
        "--target",
        "web",
        DOUBLE_DASH,
        "--no-opt",
        "--features",
        "a,b",
    ]));
    assert_eq!(
        filtered,
        vec![
            "--no-opt".to_string(),
            "--features".to_string(),
            "a,b".to_string()
        ],
        "everything before the last -- is not part of the forwarded set"
    );
}

#[test]
fn filtering_with_no_separator_and_no_euv_args_returns_the_input_unchanged() {
    let filtered: Vec<String> = filter_euv_args(&owned(&["--target", "web", "--release"]));
    assert_eq!(
        filtered,
        vec!["--target".to_string(), "web".to_string(), "--release".to_string()]
    );
}

#[test]
fn a_build_mode_flag_maps_to_its_own_wasm_pack_spelling() {
    assert_eq!(build_mode_to_flag(BuildMode::Dev), DEV_FLAG);
    assert_eq!(build_mode_to_flag(BuildMode::Release), RELEASE_FLAG);
    assert_eq!(build_mode_to_flag(BuildMode::Profiling), PROFILING_FLAG);
}

#[test]
fn profiling_outranks_release_and_release_outranks_dev() {
    let both: ModeArgs = args_from(&[DEV_FLAG, RELEASE_FLAG, PROFILING_FLAG]);
    assert_eq!(resolve_build_mode(&both), BuildMode::Profiling);
    let dev_and_release: ModeArgs = args_from(&[DEV_FLAG, RELEASE_FLAG]);
    assert_eq!(resolve_build_mode(&dev_and_release), BuildMode::Release);
    let dev_only: ModeArgs = args_from(&[DEV_FLAG]);
    assert_eq!(resolve_build_mode(&dev_only), BuildMode::Dev);
}

#[test]
fn a_build_mode_forwarded_inside_wasm_pack_args_still_resolves() {
    let forwarded: ModeArgs = args_from(&[RELEASE_FLAG]);
    assert_eq!(
        resolve_build_mode(&forwarded),
        BuildMode::Release,
        "trailing_var_arg swallows the flag, but the mode must still be found"
    );
    let forwarded_profiling: ModeArgs = args_from(&[PROFILING_FLAG]);
    assert_eq!(resolve_build_mode(&forwarded_profiling), BuildMode::Profiling);
}

#[test]
fn with_no_mode_flag_anywhere_the_build_defaults_to_dev() {
    let plain: ModeArgs = args_from(&["--target", "web"]);
    assert_eq!(resolve_build_mode(&plain), BuildMode::Dev);
    let bare: ModeArgs = args_from(&[]);
    assert_eq!(resolve_build_mode(&bare), BuildMode::Dev);
}

#[test]
fn the_output_dir_defaults_to_a_pkg_folder_inside_the_www_dir() {
    let args: ModeArgs = args_from(&[]);
    let out_dir: PathBuf = resolve_out_dir(&args);
    assert!(out_dir.ends_with("www/pkg"), "got {out_dir:?}");
}

#[test]
fn an_explicit_out_dir_is_honoured_in_both_spellings() {
    let spaced: ModeArgs = args_from(&[OUT_DIR_ARG, "dist"]);
    let out_dir: PathBuf = resolve_out_dir(&spaced);
    assert!(out_dir.ends_with("dist"), "got {out_dir:?}");
    let inline: ModeArgs = args_from(&[&format!("{OUT_DIR_ARG}=dist2")]);
    assert!(resolve_out_dir(&inline).ends_with("dist2"));
}

#[test]
fn an_absolute_out_dir_is_not_joined_onto_the_crate_path() {
    let args: ModeArgs = args_from(&[OUT_DIR_ARG, "/tmp/euv-pkg"]);
    let out_dir: PathBuf = resolve_out_dir(&args);
    assert!(out_dir.is_absolute(), "got {out_dir:?}");
    assert_eq!(out_dir, PathBuf::from("/tmp/euv-pkg"));
}

#[test]
fn an_out_name_becomes_a_js_filename_with_hyphens_swapped_for_underscores() {
    let args: ModeArgs = args_from(&[OUT_NAME_ARG, "my-game"]);
    assert_eq!(resolve_out_name(&args), "my_game.js");
    let inline: ModeArgs = args_from(&[&format!("{OUT_NAME_ARG}=other-game")]);
    assert_eq!(resolve_out_name(&inline), "other_game.js");
}

#[test]
fn without_an_out_name_the_crate_directory_name_becomes_the_filename() {
    let args: ModeArgs = args_from(&["--crate-path", "/tmp/some-crate"]);
    assert_eq!(
        resolve_out_name(&args),
        "some_crate.js",
        "with no Cargo.toml to read, the directory name is the fallback"
    );
}

#[test]
fn the_import_path_points_at_the_js_file_relative_to_the_serving_root() {
    let args: ModeArgs = args_from(&[]);
    let import: String = resolve_import_path(&args);
    assert!(
        import.starts_with("./") || import.starts_with("../"),
        "an import path is always relative, got {import}"
    );
    assert!(
        import.ends_with(".js"),
        "and it must land on the generated script, got {import}"
    );
    assert!(
        !import.contains('\\'),
        "the separator is always forward slashes, got {import}"
    );
}

#[test]
fn the_serving_route_prefix_is_the_served_directory_relative_to_the_crate() {
    let args: ModeArgs = args_from(&[]);
    let prefix: String = resolve_serving_route_prefix(&args);
    assert_eq!(
        prefix, "www",
        "with the default layout the served root is the www directory itself"
    );
}
