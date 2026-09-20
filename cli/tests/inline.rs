//! Unit tests for the inline-bridge parser helpers.
//!
//! These exercise the pure-function half of `cli/src/build/inline.rs` so that
//! the bridge-inlining pipeline has regression coverage without needing a
//! `wasm-pack` build or a running browser. End-to-end browser verification
//! still happens in CI via the example app, but pure-parser regressions
//! (import-spec extraction, namespace detection, export-name scanning) are
//! caught by these tests.

use euv_cli::{
    extract_exported_function_names, extract_import_spec, extract_namespace_alias,
    is_namespace_import,
};

#[test]
fn extract_import_spec_strips_quotes_and_semicolon() {
    let rest: &str = "{ euv_event_collect_id_chain } from './snippets/euv-core-abc/inline1.js';";
    assert_eq!(
        extract_import_spec(rest),
        Some("./snippets/euv-core-abc/inline1.js")
    );
}

#[test]
fn extract_import_spec_handles_double_quotes() {
    let rest: &str = "* as import1 from \"./snippets/euv-core-abc/inline0.js\"";
    assert_eq!(
        extract_import_spec(rest),
        Some("./snippets/euv-core-abc/inline0.js")
    );
}

#[test]
fn extract_import_spec_rejects_non_relative_specs() {
    // Bare module specifiers (e.g. `lodash`) must not be inlined.
    let rest: &str = "default from 'lodash';";
    assert_eq!(extract_import_spec(rest), None);
}

#[test]
fn is_namespace_import_detects_star_as() {
    assert!(is_namespace_import("* as import1 from './snippets/x.js';"));
    assert!(is_namespace_import("* as foo from \"./x.js\""));
    assert!(!is_namespace_import("{ a, b } from './snippets/x.js';"));
    assert!(!is_namespace_import("default from './x.js';"));
}

#[test]
fn extract_namespace_alias_returns_alias_name() {
    assert_eq!(
        extract_namespace_alias("* as import1 from \"./snippets/x.js\";"),
        Some("import1")
    );
    assert_eq!(
        extract_namespace_alias("* as foo from './x.js'"),
        Some("foo")
    );
    assert_eq!(extract_namespace_alias("{ a } from './x.js';"), None);
}

#[test]
fn extract_exported_function_names_finds_all_declarations() {
    let source: &str = r#"
export function euv_collect_subtree_ids(root) {
    return [];
}

export function euv_event_collect_id_chain(event, max_depth) {
    return ids;
}

function _private_helper() {}

// Re-exports / non-function exports are ignored.
export const VERSION = 1;
export { something };
"#;
    let names: Vec<String> = extract_exported_function_names(source);
    assert_eq!(
        names,
        vec![
            "euv_collect_subtree_ids".to_string(),
            "euv_event_collect_id_chain".to_string()
        ]
    );
}

#[test]
fn extract_exported_function_names_handles_empty_input() {
    let names: Vec<String> = extract_exported_function_names("");
    assert!(names.is_empty());
}

#[test]
fn extract_exported_function_names_supports_dollar_identifiers() {
    // wasm-bindgen occasionally emits $ in function names.
    let source: &str = "export function foo$bar() {}\n";
    let names: Vec<String> = extract_exported_function_names(source);
    assert_eq!(names, vec!["foo$bar".to_string()]);
}
