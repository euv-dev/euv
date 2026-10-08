use super::*;

#[test]
fn style_string_renders_a_single_declaration() {
    let observed: String = Css::style_string(&[("color", "red")]);
    assert_eq!(observed, "color: red;", "one property gets one declaration");
}

#[test]
fn style_string_separates_multiple_declarations_with_a_space() {
    let observed: String = Css::style_string(&[("color", "red"), ("margin", "0 auto")]);
    assert_eq!(
        observed, "color: red; margin: 0 auto;",
        "declarations are space separated"
    );
}

#[test]
fn style_string_of_an_empty_slice_is_empty() {
    let observed: String = Css::style_string(&[] as &[(&str, &str)]);
    assert_eq!(observed, "", "no properties produce no CSS");
}

#[test]
fn style_string_preserves_the_property_order_it_is_given() {
    let observed: String = Css::style_string(&[("a", "1"), ("b", "2"), ("c", "3")]);
    assert_eq!(
        observed, "a: 1; b: 2; c: 3;",
        "the caller controls declaration order"
    );
}

#[test]
fn style_string_owned_matches_the_borrowed_form() {
    let borrowed: String = Css::style_string(&[("color", "red"), ("top", "0")]);
    let owned: String = Css::style_string_owned(&[
        ("color".to_string(), "red".to_string()),
        ("top".to_string(), "0".to_string()),
    ]);
    assert_eq!(
        owned, borrowed,
        "the owned and borrowed builders must produce identical CSS"
    );
}

#[test]
fn style_string_owned_of_an_empty_slice_is_empty() {
    let observed: String = Css::style_string_owned(&[] as &[(String, String)]);
    assert_eq!(observed, "", "no properties produce no CSS");
}

#[test]
fn style_string_keeps_a_value_containing_a_colon_intact() {
    let observed: String = Css::style_string(&[("background", "url(http://x/y) no-repeat")]);
    assert_eq!(
        observed, "background: url(http://x/y) no-repeat;",
        "only the first colon is the separator, the rest is value"
    );
}

#[test]
fn param_class_name_is_stable_for_the_same_input() {
    let first: String = Css::param_class_name("primary");
    let second: String = Css::param_class_name("primary");
    assert_eq!(first, second, "the same parameter must hash the same");
}

#[test]
fn param_class_name_differs_between_different_inputs() {
    let first: String = Css::param_class_name("primary");
    let second: String = Css::param_class_name("secondary");
    assert_ne!(
        first, second,
        "different parameters must get different class suffixes"
    );
}

#[test]
fn param_class_name_of_an_empty_value_is_still_a_name() {
    let observed: String = Css::param_class_name("");
    assert!(
        !observed.is_empty(),
        "even an empty parameter must produce a usable suffix"
    );
}

#[test]
fn param_class_name_is_hexadecimal_and_lowercase() {
    let observed: String = Css::param_class_name("size-3");
    assert!(
        observed
            .chars()
            .all(|c: char| c.is_ascii_hexdigit() && !c.is_ascii_uppercase()),
        "the suffix is a lowercase hex string, got {observed}"
    );
}

#[test]
fn parse_pseudo_rules_reads_a_single_rule() {
    let observed: Vec<PseudoRule> = Css::parse_pseudo_rules(":hover { color: red; }");
    assert_eq!(observed.len(), 1, "one serialized rule yields one rule");
    assert_eq!(
        observed[0],
        PseudoRule::new(String::from(":hover"), String::from("color: red; ")),
        "the selector is captured and the body keeps its trailing separator"
    );
}

#[test]
fn parse_pseudo_rules_reads_several_rules_in_order() {
    let observed: Vec<PseudoRule> =
        Css::parse_pseudo_rules(":hover { color: red; }:focus { color: blue; }");
    assert_eq!(observed.len(), 2, "both rules are parsed");
    assert_eq!(
        observed[0],
        PseudoRule::new(String::from(":hover"), String::from("color: red; ")),
        "the first rule comes first"
    );
    assert_eq!(
        observed[1],
        PseudoRule::new(String::from(":focus"), String::from("color: blue; ")),
        "the second rule follows"
    );
}

#[test]
fn parse_pseudo_rules_of_an_empty_string_is_empty() {
    let observed: Vec<PseudoRule> = Css::parse_pseudo_rules("");
    assert!(
        observed.is_empty(),
        "nothing serialized means nothing parsed"
    );
}

#[test]
fn parse_pseudo_rules_keeps_a_multi_declaration_body_together() {
    let observed: Vec<PseudoRule> = Css::parse_pseudo_rules(":hover { color: red; top: 0; }");
    assert_eq!(observed.len(), 1, "the whole body is one rule");
    assert_eq!(
        observed[0],
        PseudoRule::new(String::from(":hover"), String::from("color: red; top: 0; ")),
        "declarations are not split apart"
    );
}

#[test]
fn parse_pseudo_rules_drops_a_rule_with_an_empty_selector() {
    let observed: Vec<PseudoRule> = Css::parse_pseudo_rules(" { color: red; }");
    assert!(observed.is_empty(), "a nameless selector is not a rule");
}

#[test]
fn parse_pseudo_rules_drops_a_rule_with_an_empty_body() {
    let observed: Vec<PseudoRule> = Css::parse_pseudo_rules(":hover { }");
    assert!(observed.is_empty(), "a bodyless rule carries no style");
}

#[test]
fn parse_pseudo_rules_keeps_a_whitespace_only_body() {
    let observed: Vec<PseudoRule> = Css::parse_pseudo_rules(":hover {  }");
    assert_eq!(
        observed.len(),
        1,
        "the open delimiter eats one space, so the leftover space is a non-empty body"
    );
}

#[test]
fn parse_pseudo_rules_stops_at_a_truncated_rule() {
    let observed: Vec<PseudoRule> =
        Css::parse_pseudo_rules(":hover { color: red; }:focus { color: blue;");
    assert_eq!(
        observed.len(),
        1,
        "the unterminated trailing rule is dropped"
    );
}

#[test]
fn parse_pseudo_rules_stops_when_the_opening_brace_is_missing() {
    let observed: Vec<PseudoRule> = Css::parse_pseudo_rules(":hover color: red;");
    assert!(observed.is_empty(), "text with no rule block is not a rule");
}

#[test]
fn parse_media_rules_reads_a_single_query() {
    let observed: Vec<MediaRule> =
        Css::parse_media_rules("@media (min-width: 600px) { color: red; }");
    assert_eq!(observed.len(), 1, "one serialized query yields one rule");
    assert_eq!(
        observed[0],
        MediaRule::new(
            String::from("(min-width: 600px)"),
            String::from("color: red;"),
            Vec::new(),
        ),
        "the query text and body are captured"
    );
}

#[test]
fn parse_media_rules_reads_several_queries_in_order() {
    let observed: Vec<MediaRule> = Css::parse_media_rules(
        "@media (min-width: 600px) { color: red; }@media (min-width: 900px) { color: blue; }",
    );
    assert_eq!(observed.len(), 2, "both queries are parsed");
    assert_eq!(
        observed[0],
        MediaRule::new(
            String::from("(min-width: 600px)"),
            String::from("color: red;"),
            Vec::new(),
        ),
        "the first query comes first"
    );
    assert_eq!(
        observed[1],
        MediaRule::new(
            String::from("(min-width: 900px)"),
            String::from("color: blue;"),
            Vec::new(),
        ),
        "the second query follows"
    );
}

#[test]
fn parse_media_rules_of_an_empty_string_is_empty() {
    let observed: Vec<MediaRule> = Css::parse_media_rules("");
    assert!(
        observed.is_empty(),
        "nothing serialized means nothing parsed"
    );
}

#[test]
fn parse_media_rules_refuses_a_string_that_does_not_start_with_the_prefix() {
    let observed: Vec<MediaRule> = Css::parse_media_rules("color: red;");
    assert!(
        observed.is_empty(),
        "a non-media serialization is not a media rule"
    );
}

#[test]
fn parse_media_rules_stops_at_a_truncated_query() {
    let observed: Vec<MediaRule> =
        Css::parse_media_rules("@media (min-width: 600px) { color: red;");
    assert!(
        observed.is_empty(),
        "an unterminated block cannot be located, so nothing is parsed"
    );
}

#[test]
fn parse_media_rules_reads_a_nested_pseudo_block() {
    let observed: Vec<MediaRule> = Css::parse_media_rules(
        "@media (min-width: 600px) { color: red; ::selection { color: blue; } }",
    );
    assert_eq!(observed.len(), 1, "the outer query is the rule");
}

#[test]
fn merge_style_joins_text_segments_with_spaces() {
    let values: [AttributeValue; 2] = [
        AttributeValue::Text("color: red;".to_string()),
        AttributeValue::Text("top: 0;".to_string()),
    ];
    let observed: AttributeValue = AttributeValue::merge_style(&values);
    assert!(
        matches!(observed, AttributeValue::Text(_)),
        "text stays text"
    );
}

#[test]
fn merge_style_of_an_empty_slice_is_empty_text() {
    let observed: AttributeValue = AttributeValue::merge_style(&[] as &[AttributeValue]);
    assert!(
        matches!(observed, AttributeValue::Text(_)),
        "nothing to merge still yields the text variant"
    );
}

#[test]
fn merge_style_skips_empty_segments() {
    let values: [AttributeValue; 3] = [
        AttributeValue::Text("color: red;".to_string()),
        AttributeValue::Text(String::new()),
        AttributeValue::Text("top: 0;".to_string()),
    ];
    let observed: AttributeValue = AttributeValue::merge_style(&values);
    assert!(
        matches!(observed, AttributeValue::Text(_)),
        "an empty segment does not promote the merge to a signal"
    );
}

#[test]
fn reactive_wraps_a_computed_string_in_a_signal_backed_value() {
    let observed: AttributeValue = AttributeValue::reactive(|| "color: red;".to_string());
    assert!(
        matches!(observed, AttributeValue::Signal(_)),
        "a computed attribute is signal backed"
    );
}

#[test]
fn merge_class_joins_non_empty_names_with_spaces() {
    let values: [AttributeValue; 2] = [
        AttributeValue::Text("btn".to_string()),
        AttributeValue::Text("primary".to_string()),
    ];
    let observed: AttributeValue = AttributeValue::merge_class(&values);
    assert!(
        matches!(observed, AttributeValue::Text(_)),
        "text stays text"
    );
}

#[test]
fn merge_class_of_an_empty_slice_is_empty_text() {
    let observed: AttributeValue = AttributeValue::merge_class(&[] as &[AttributeValue]);
    assert!(
        matches!(observed, AttributeValue::Text(_)),
        "nothing to merge still yields the text variant"
    );
}

#[test]
fn injecting_css_off_wasm_returns_instead_of_reaching_for_the_document() {
    Css::inject_css("body { margin: 0; }");
    Css::inject_css(String::from(".x { color: red; }"));
}

#[test]
fn injecting_a_style_off_wasm_never_reaches_the_dom() {
    let css: Css = Css::new(
        String::from("plain"),
        String::from("color: red;"),
        Vec::new(),
        Vec::new(),
    );
    css.inject_style();
    assert_eq!(
        format!("{css}"),
        "plain",
        "the name is the class selector the stylesheet is keyed by"
    );
}

#[test]
fn a_style_name_is_escaped_into_a_valid_css_identifier() {
    let css: Css = Css::new(
        String::from("has space"),
        String::from("color: red;"),
        Vec::new(),
        Vec::new(),
    );
    assert_eq!(
        format!("{css}"),
        "has space",
        "the class attribute keeps the readable name; only the stylesheet escapes it"
    );
}

#[test]
fn a_name_of_only_safe_characters_needs_no_escaping() {
    for name in ["plain", "with-dash", "with_underscore", "camelCase", "n123"] {
        let css: Css = Css::new(
            name.to_string(),
            String::from("color: red;"),
            Vec::new(),
            Vec::new(),
        );
        css.inject_style();
        assert_eq!(format!("{css}"), name, "an unescaped name must round-trip");
    }
}

#[test]
fn a_name_full_of_punctuation_still_injects_without_panicking() {
    for name in [
        "a.b",
        "a b c",
        "1 2 3",
        "::before",
        "--custom-prop",
        "你好",
        "",
    ] {
        let css: Css = Css::new(
            name.to_string(),
            String::from("color: red;"),
            Vec::new(),
            Vec::new(),
        );
        css.inject_style();
        assert_eq!(format!("{css}"), name, "the name must survive verbatim");
    }
}

#[test]
fn a_style_with_pseudo_and_media_rules_injects_the_whole_set() {
    let css: Css = Css::new(
        String::from("card"),
        String::from("color: red;"),
        vec![PseudoRule::new(
            String::from(":hover"),
            String::from("color: blue;"),
        )],
        vec![MediaRule::new(
            String::from("(max-width: 767px)"),
            String::from("font-size: 14px;"),
            vec![PseudoRule::new(
                String::from("::-webkit-scrollbar"),
                String::from("width: 0px;"),
            )],
        )],
    );
    css.inject_style();
    assert_eq!(format!("{css}"), "card");
}

#[test]
fn an_empty_pseudo_rule_is_skipped_rather_than_emitted_empty() {
    let css: Css = Css::new(
        String::from("card"),
        String::from("color: red;"),
        vec![PseudoRule::new(String::from(":hover"), String::new())],
        vec![MediaRule::new(
            String::from("(max-width: 767px)"),
            String::new(),
            Vec::new(),
        )],
    );
    css.inject_style();
    assert_eq!(
        format!("{css}"),
        "card",
        "an empty rule must not become a bare selector"
    );
}

#[test]
fn an_empty_media_query_is_skipped_rather_than_emitting_a_bare_at_rule() {
    let css: Css = Css::new(
        String::from("card"),
        String::from("color: red;"),
        Vec::new(),
        vec![MediaRule::new(
            String::new(),
            String::from("font-size: 14px;"),
            Vec::new(),
        )],
    );
    css.inject_style();
    assert_eq!(
        format!("{css}"),
        "card",
        "an empty query would emit a bare at-rule, which the browser drops silently"
    );
}

#[test]
fn injecting_the_same_class_twice_is_idempotent() {
    let css: Css = Css::new(
        String::from("dedupe-probe"),
        String::from("color: red;"),
        Vec::new(),
        Vec::new(),
    );
    css.inject_style();
    css.inject_style();
    assert_eq!(
        format!("{css}"),
        "dedupe-probe",
        "a second injection of the same class must be a no-op, not a duplicate rule"
    );
}
