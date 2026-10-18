use super::*;

#[test]
fn an_import_clause_reports_its_relative_specifier() {
    let observed: ImportClause<'_> = parse_import_clause(
        "{ euv_event_collect_id_chain } from './snippets/euv-core-abc/inline1.js';",
    )
    .expect("a static from clause parses");
    assert_eq!(observed.spec, "./snippets/euv-core-abc/inline1.js");
}

#[test]
fn a_double_quoted_specifier_is_unwrapped_too() {
    let observed: ImportClause<'_> =
        parse_import_clause("* as import1 from \"./snippets/euv-core-abc/inline0.js\"")
            .expect("a double-quoted clause parses");
    assert_eq!(observed.spec, "./snippets/euv-core-abc/inline0.js");
    assert_eq!(observed.namespace, Some("import1"));
}

#[test]
fn a_bare_package_clause_parses_its_specifier_but_binds_a_named_import() {
    let observed: ImportClause<'_> =
        parse_import_clause("{ a } from 'react'").expect("parsing reports the clause's shape");
    assert_eq!(
        observed.spec, "react",
        "the quoted specifier is unwrapped verbatim"
    );
    assert_eq!(observed.named, vec!["a"]);
    assert!(
        observed.namespace.is_none() && observed.default_binding.is_none(),
        "a bare package specifier is a parse-level fact, not an import shape this \
         bootstrap accepts: the inliner, not this parser, rejects a specifier it \
         cannot fetch"
    );
}

#[test]
fn an_import_without_a_from_clause_is_rejected() {
    let observed: Result<ImportClause<'_>, EuvError> = parse_import_clause("'./side-effect.js'");
    assert!(
        observed.is_err(),
        "a bare side-effect import names no specifier, so there is nothing to inline"
    );
}

#[test]
fn an_unquoted_specifier_is_rejected() {
    let observed: Result<ImportClause<'_>, EuvError> = parse_import_clause("{ a } from ./local.js");
    assert!(
        observed.is_err(),
        "an unquoted specifier is not a static import this parser may read"
    );
}

#[test]
fn an_empty_specifier_is_rejected() {
    let observed: Result<ImportClause<'_>, EuvError> = parse_import_clause("{ a } from ''");
    assert!(observed.is_err(), "an empty specifier names no module");
}

#[test]
fn a_star_as_clause_is_the_only_shape_that_carries_a_namespace() {
    let observed: ImportClause<'_> =
        parse_import_clause("* as import1 from './snippets/x.js';").expect("namespace parses");
    assert_eq!(observed.namespace, Some("import1"));
}

#[test]
fn a_brace_clause_carries_no_namespace_binding() {
    let observed: ImportClause<'_> =
        parse_import_clause("{ a, b } from './snippets/x.js';").expect("named clause parses");
    assert!(
        observed.namespace.is_none(),
        "a brace list is a named import, not a namespace binding"
    );
    assert_eq!(
        observed.named,
        vec!["a", "b"],
        "both bindings, in source order"
    );
}

#[test]
fn a_default_clause_carries_the_head_as_its_default_binding() {
    let observed: ImportClause<'_> =
        parse_import_clause("init from './x.js';").expect("default clause parses");
    assert_eq!(observed.default_binding, Some("init"));
    assert!(
        observed.namespace.is_none() && observed.named.is_empty(),
        "a default import binds exactly one form"
    );
}

#[test]
fn a_star_without_an_alias_is_rejected() {
    let observed: Result<ImportClause<'_>, EuvError> = parse_import_clause("* from './x.js'");
    assert!(
        observed.is_err(),
        "a namespace import with no binding leaves nothing for the inliner to rebuild"
    );
}

#[test]
fn an_unbalanced_brace_clause_is_rejected() {
    let observed: Result<ImportClause<'_>, EuvError> = parse_import_clause("{ a, b from './x.js'");
    assert!(
        observed.is_err(),
        "an unclosed brace list is not a named import"
    );
}

#[test]
fn exported_names_collects_every_declaration_form() {
    let source: &str = r#"
export function euv_collect_subtree_ids(root) {
    return [];
}

export async function euv_event_collect_id_chain(event, max_depth) {
    return ids;
}

function _private_helper() {}

export const VERSION = 1;
export class Widget {}
export { something };
"#;
    let names: Vec<String> = extract_exported_names(source);
    assert_eq!(
        names,
        vec![
            "euv_collect_subtree_ids".to_string(),
            "euv_event_collect_id_chain".to_string(),
            "VERSION".to_string(),
            "Widget".to_string()
        ],
        "every declaration form the scanner accepts counts, in source order; \
         a bare `export {{ }}` clause names no declaration and `export const arrow` \
         is covered separately"
    );
}

#[test]
fn exported_names_of_empty_input_is_empty() {
    let names: Vec<String> = extract_exported_names("");
    assert!(names.is_empty());
}

#[test]
fn exported_names_supports_dollar_identifiers() {
    let source: &str = "export function foo$bar() {}\n";
    let names: Vec<String> = extract_exported_names(source);
    assert_eq!(names, vec!["foo$bar".to_string()]);
}
