use super::*;

fn args(values: &[&str]) -> Vec<String> {
    values.iter().map(|v: &&str| (*v).to_string()).collect()
}

#[test]
fn an_empty_argv_has_no_build_mode_flag() {
    let observed: bool = has_build_mode_flag(&[] as &[String]);
    assert!(!observed, "nothing to find means nothing found");
}

#[test]
fn the_dev_flag_is_a_build_mode_flag() {
    let observed: bool = has_build_mode_flag(&args(&["--dev"]));
    assert!(observed, "--dev selects a build mode");
}

#[test]
fn the_release_flag_is_a_build_mode_flag() {
    let observed: bool = has_build_mode_flag(&args(&["--release"]));
    assert!(observed, "--release selects a build mode");
}

#[test]
fn the_profiling_flag_is_a_build_mode_flag() {
    let observed: bool = has_build_mode_flag(&args(&["--profiling"]));
    assert!(observed, "--profiling selects a build mode");
}

#[test]
fn a_build_mode_flag_is_found_among_other_args() {
    let observed: bool =
        has_build_mode_flag(&args(&["--target", "web", "--release", "--out-dir", "x"]));
    assert!(
        observed,
        "the search scans the whole list, not just the first entry"
    );
}

#[test]
fn an_unknown_flag_is_not_a_build_mode_flag() {
    let observed: bool = has_build_mode_flag(&args(&["--target", "web"]));
    assert!(!observed, "unrelated wasm-pack flags do not count");
}

#[test]
fn a_flag_that_merely_contains_a_build_mode_word_does_not_count() {
    let observed: bool = has_build_mode_flag(&args(&["--release-please"]));
    assert!(
        !observed,
        "the match is exact, so a longer flag name is a different flag"
    );
}

#[test]
fn build_mode_to_flag_round_trips_each_variant() {
    assert_eq!(build_mode_to_flag(BuildMode::Dev), "--dev");
    assert_eq!(build_mode_to_flag(BuildMode::Release), "--release");
    assert_eq!(build_mode_to_flag(BuildMode::Profiling), "--profiling");
}

#[test]
fn every_build_mode_produces_a_distinct_flag() {
    let dev: &str = build_mode_to_flag(BuildMode::Dev);
    let release: &str = build_mode_to_flag(BuildMode::Release);
    let profiling: &str = build_mode_to_flag(BuildMode::Profiling);
    assert!(
        dev != release && release != profiling && dev != profiling,
        "the three modes must not collapse onto one flag"
    );
}

#[test]
fn every_emitted_flag_is_recognised_as_a_build_mode_flag() {
    for mode in [BuildMode::Dev, BuildMode::Release, BuildMode::Profiling] {
        let flag: &str = build_mode_to_flag(mode);
        let observed: bool = has_build_mode_flag(&args(&[flag]));
        assert!(observed, "{flag} must be detectable as a build mode flag");
    }
}

#[test]
fn args_after_the_double_dash_are_the_ones_kept() {
    let observed: Vec<String> = filter_euv_args(&args(&["--dev", "--", "--target", "web"]));
    assert_eq!(
        observed,
        args(&["--target", "web"]),
        "wasm-pack flags before the separator are euv's own and are dropped"
    );
}

#[test]
fn a_valued_euv_flag_consumes_its_value() {
    let observed: Vec<String> = filter_euv_args(&args(&["--port", "3000", "--target", "web"]));
    assert_eq!(
        observed,
        args(&["--target", "web"]),
        "the flag and the argument after it are both removed"
    );
}

#[test]
fn an_equals_form_euv_flag_is_recognised_by_the_whitelist() {
    let observed: Vec<String> = filter_euv_args(&args(&["--port=3000", "--target", "web"]));
    assert_eq!(
        observed,
        args(&["--target", "web"]),
        "the whitelist compares the flag NAME, so the = form matches too and the \
         argument leaves the wasm-pack command line entirely. Got {observed:?}"
    );
}

#[test]
fn the_last_double_dash_wins_as_the_separator() {
    let observed: Vec<String> = filter_euv_args(&args(&["--", "build", "--", "--target", "web"]));
    assert_eq!(
        observed,
        args(&["--target", "web"]),
        "rposition finds the final separator, so earlier tokens are dropped"
    );
}

#[test]
fn args_with_no_separator_pass_through_unchanged() {
    let observed: Vec<String> = filter_euv_args(&args(&["--target", "web", "--release"]));
    assert_eq!(
        observed,
        args(&["--target", "web", "--release"]),
        "without a separator nothing is stripped"
    );
}

#[test]
fn filtering_an_empty_argv_yields_nothing() {
    let observed: Vec<String> = filter_euv_args(&[] as &[String]);
    assert!(observed.is_empty(), "nothing in, nothing out");
}

#[test]
fn a_trailing_euv_flag_does_not_eat_the_end_of_the_list() {
    let observed: Vec<String> = filter_euv_args(&args(&["--target", "web", "--port"]));
    assert_eq!(
        observed,
        args(&["--target", "web"]),
        "a value flag with no value after it simply drops itself"
    );
}

#[test]
fn a_short_euv_flag_is_stripped_as_well() {
    let observed: Vec<String> = filter_euv_args(&args(&["-p", "8080", "--target", "web"]));
    assert_eq!(
        observed,
        args(&["--target", "web"]),
        "the short forms are on the same whitelist as the long ones"
    );
}

#[test]
fn a_flag_outside_the_whitelist_is_left_for_wasm_pack() {
    let observed: Vec<String> = filter_euv_args(&args(&["--out-dir", "dist", "--target", "web"]));
    assert_eq!(
        observed,
        args(&["--out-dir", "dist", "--target", "web"]),
        "only the seven whitelisted euv args are stripped, wasm-pack keeps the rest"
    );
}

#[test]
fn exported_functions_are_collected_in_source_order() {
    let source: &str = "export function alpha() {}\nexport function beta() {}\n";
    let observed: Vec<String> = extract_exported_function_names(source);
    assert_eq!(observed, args(&["alpha", "beta"]), "both names, in order");
}

#[test]
fn leading_indentation_does_not_hide_an_exported_function() {
    let source: &str = "    export function indented() {}\n";
    let observed: Vec<String> = extract_exported_function_names(source);
    assert_eq!(observed, args(&["indented"]), "the line is trimmed first");
}

#[test]
fn a_private_function_is_not_collected() {
    let source: &str = "function hidden() {}\nexport function shown() {}\n";
    let observed: Vec<String> = extract_exported_function_names(source);
    assert_eq!(observed, args(&["shown"]), "only the exported one counts");
}

#[test]
fn an_arrow_function_export_is_not_a_function_declaration() {
    let source: &str = "export const arrow = () => {};\n";
    let observed: Vec<String> = extract_exported_function_names(source);
    assert!(
        observed.is_empty(),
        "the scanner looks for the function declaration form only"
    );
}

#[test]
fn dollar_signs_are_part_of_an_exported_name() {
    let source: &str = "export function $helper() {}\n";
    let observed: Vec<String> = extract_exported_function_names(source);
    assert_eq!(
        observed,
        args(&["$helper"]),
        "$ is a valid identifier character"
    );
}

#[test]
fn a_name_ends_at_the_first_non_identifier_character() {
    let source: &str = "export function typed(a: number) {}\n";
    let observed: Vec<String> = extract_exported_function_names(source);
    assert_eq!(observed, args(&["typed"]), "the parameter list is cut off");
}

#[test]
fn source_without_exports_yields_nothing() {
    let observed: Vec<String> = extract_exported_function_names("const x = 1;\n");
    assert!(observed.is_empty(), "no export means no names");
}

#[test]
fn a_star_import_is_a_namespace_import() {
    let observed: bool = is_namespace_import("* as helpers");
    assert!(observed, "a leading star marks a namespace import");
}

#[test]
fn a_named_import_is_not_a_namespace_import() {
    let observed: bool = is_namespace_import("{ a, b }");
    assert!(!observed, "a brace list is a named import");
}

#[test]
fn a_namespace_import_recovers_its_alias() {
    let observed: Option<&str> = extract_namespace_alias("* as helpers");
    assert_eq!(observed, Some("helpers"), "the alias after as is the name");
}

#[test]
fn a_namespace_alias_tolerates_trailing_syntax() {
    let observed: Option<&str> = extract_namespace_alias("* as helpers;");
    assert_eq!(observed, Some("helpers"), "a trailing semicolon is trimmed");
}

#[test]
fn a_namespace_alias_of_a_bare_star_is_none() {
    let observed: Option<&str> = extract_namespace_alias("*");
    assert_eq!(observed, None, "there is no alias to report");
}

#[test]
fn a_named_import_has_no_namespace_alias() {
    let observed: Option<&str> = extract_namespace_alias("{ a }");
    assert_eq!(observed, None, "a brace list is not a namespace import");
}

#[test]
fn a_relative_import_reports_its_specifier() {
    let observed: Option<&str> = extract_import_spec("{ a } from './local.js'");
    assert_eq!(observed, Some("./local.js"), "the quoted path is unwrapped");
}

#[test]
fn a_parent_relative_import_reports_its_specifier() {
    let observed: Option<&str> = extract_import_spec("{ a } from '../up.js'");
    assert_eq!(
        observed,
        Some("../up.js"),
        "parent-relative paths count too"
    );
}

#[test]
fn a_bare_package_import_is_not_reported() {
    let observed: Option<&str> = extract_import_spec("{ a } from 'react'");
    assert_eq!(observed, None, "a package specifier is not a local path");
}

#[test]
fn an_import_without_a_from_clause_is_not_reported() {
    let observed: Option<&str> = extract_import_spec("import './side-effect.js'");
    assert_eq!(observed, None, "a bare side-effect import names nothing");
}

#[test]
fn double_quoted_specifiers_are_unwrapped_too() {
    let observed: Option<&str> = extract_import_spec("{ a } from \"./local.js\"");
    assert_eq!(
        observed,
        Some("./local.js"),
        "both quote styles are accepted"
    );
}

fn argv(items: &[&str]) -> Vec<String> {
    items
        .iter()
        .map(|item: &&str| (*item).to_string())
        .collect()
}

#[test]
fn a_space_separated_euv_flag_drops_itself_and_its_value() {
    let observed: Vec<String> = filter_euv_args(&argv(&["--port", "3000", "--target", "web"]));
    assert_eq!(
        observed,
        vec!["--target".to_string(), "web".to_string()],
        "both the euv flag and the value it owns must leave the wasm-pack command line"
    );
}

#[test]
fn an_equals_form_euv_flag_is_dropped_whole() {
    let observed: Vec<String> = filter_euv_args(&argv(&["--port=3000", "--target=web"]));
    assert_eq!(
        observed,
        vec!["--target=web".to_string()],
        "`--port=3000` carries its own value, so dropping the flag means dropping the whole \
         argument — forwarding it would hand wasm-pack a flag it does not know"
    );
}

#[test]
fn every_euv_flag_is_dropped_in_both_spellings() {
    for (spaced, equals) in [
        (vec!["--crate-path", "/tmp/app"], "--crate-path=/tmp/app"),
        (vec!["-c", "/tmp/app"], "-c=/tmp/app"),
        (vec!["--www-dir", "static"], "--www-dir=static"),
        (
            vec!["--index-html", "custom.html"],
            "--index-html=custom.html",
        ),
        (vec!["--no-gitignore"], "--no-gitignore=true"),
    ] {
        let mut spaced_args: Vec<String> = spaced.iter().map(|s: &&str| (*s).to_string()).collect();
        spaced_args.push(String::from("--target"));
        spaced_args.push(String::from("web"));
        let spaced_out: Vec<String> = filter_euv_args(&spaced_args);
        assert_eq!(
            spaced_out,
            vec!["--target".to_string(), "web".to_string()],
            "{spaced:?} must be removed in its spaced spelling"
        );

        let mut equals_args: Vec<String> = vec![equals.to_string()];
        equals_args.push(String::from("--target"));
        equals_args.push(String::from("web"));
        let equals_out: Vec<String> = filter_euv_args(&equals_args);
        assert_eq!(
            equals_out,
            vec!["--target".to_string(), "web".to_string()],
            "{equals} must be removed in its equals spelling too"
        );
    }
}

#[test]
fn a_wasm_pack_flag_containing_an_equals_sign_survives() {
    let observed: Vec<String> = filter_euv_args(&argv(&["--out-dir=dist", "--scope=my-pkg"]));
    assert_eq!(
        observed,
        vec!["--out-dir=dist".to_string(), "--scope=my-pkg".to_string()],
        "a wasm-pack flag is not an euv flag just because it uses the same syntax"
    );
}

#[test]
fn a_value_that_merely_contains_an_equals_sign_is_not_dropped() {
    let observed: Vec<String> = filter_euv_args(&argv(&["--scope", "a=b", "--target", "web"]));
    assert_eq!(
        observed,
        vec![
            "--scope".to_string(),
            "a=b".to_string(),
            "--target".to_string(),
            "web".to_string()
        ],
        "the value of a wasm-pack flag is data, not a flag, and must survive"
    );
}

#[test]
fn everything_after_the_last_separator_is_the_only_candidate_list() {
    let observed: Vec<String> = filter_euv_args(&argv(&[
        "--port", "3000", "--", "--port", "4000", "--target", "web",
    ]));
    assert_eq!(
        observed,
        vec!["--target".to_string(), "web".to_string()],
        "the earlier euv flags were already parsed by clap; only the post-`--` run is filtered"
    );
}

#[test]
fn a_non_euv_flag_before_the_separator_is_not_filtered() {
    let observed: Vec<String> = filter_euv_args(&argv(&["--port", "3000", "--release"]));
    assert_eq!(
        observed,
        vec!["--release".to_string()],
        "with no separator the whole list is filtered, so a wasm-pack flag is untouched"
    );
}

#[test]
fn filtering_an_empty_list_yields_an_empty_list() {
    let observed: Vec<String> = filter_euv_args(&[]);
    assert!(observed.is_empty());
}

#[test]
fn a_lone_separator_contributes_nothing() {
    let observed: Vec<String> = filter_euv_args(&argv(&["--", "--target", "web"]));
    assert_eq!(observed, vec!["--target".to_string(), "web".to_string()]);
}

#[test]
fn reconcile_reads_the_equals_form_into_the_typed_field() {
    let mut args: ModeArgs = ModeArgs::parse_from(["euv", "--", "--port=4321"]);
    reconcile_args(&mut args);
    let rendered: String = format!("{args:?}");
    assert!(
        rendered.contains("port: 4321"),
        "the equals spelling must reach the typed field, got: {rendered}"
    );
}

#[test]
fn a_filtered_equals_form_leaves_nothing_for_wasm_pack_to_misread() {
    let passthrough: Vec<String> = argv(&["--www-dir=static", "--target=web"]);
    let remaining: Vec<String> = filter_euv_args(&passthrough);
    assert_eq!(
        remaining,
        vec!["--target=web".to_string()],
        "after filtering the euv flag must be gone from the wasm-pack command line"
    );
}

#[test]
fn a_boolean_euv_flag_does_not_swallow_the_argument_after_it() {
    let observed: Vec<String> = filter_euv_args(&argv(&["--no-gitignore", "--target", "web"]));
    assert_eq!(
        observed,
        vec!["--target".to_string(), "web".to_string()],
        "`--no-gitignore` is a bare boolean with no value, so skipping the next \
         argument would silently drop a real wasm-pack flag"
    );
}

#[test]
fn a_boolean_euv_flag_at_the_end_of_the_list_is_harmless() {
    let observed: Vec<String> = filter_euv_args(&argv(&["--target", "web", "--no-gitignore"]));
    assert_eq!(
        observed,
        vec!["--target".to_string(), "web".to_string()],
        "a trailing boolean has nothing after it to consume"
    );
}

#[test]
fn two_boolean_flags_in_a_row_both_leave() {
    let observed: Vec<String> = filter_euv_args(&argv(&["--no-gitignore", "--release"]));
    assert_eq!(
        observed,
        vec!["--release".to_string()],
        "each boolean is independent; neither may consume the other"
    );
}
