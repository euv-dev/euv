use super::*;
#[test]
fn css_debug_format_works() {
    let css: Css = Css::default();
    let formatted: String = format!("{:?}", css);
    assert!(formatted.contains("Css"));
}

#[test]
fn pseudo_rule_equality_same_values() {
    let a: PseudoRule = PseudoRule::new(String::from(":hover"), String::from("background: blue;"));
    let b: PseudoRule = PseudoRule::new(String::from(":hover"), String::from("background: blue;"));
    assert_eq!(a, b);
}

#[test]
fn pseudo_rule_equality_different_selectors() {
    let a: PseudoRule = PseudoRule::new(String::from(":hover"), String::from("background: blue;"));
    let b: PseudoRule = PseudoRule::new(String::from(":focus"), String::from("background: blue;"));
    assert_ne!(a, b);
}

#[test]
fn pseudo_rule_equality_different_styles() {
    let a: PseudoRule = PseudoRule::new(String::from(":hover"), String::from("background: blue;"));
    let b: PseudoRule = PseudoRule::new(String::from(":hover"), String::from("background: red;"));
    assert_ne!(a, b);
}

#[test]
fn pseudo_rule_hash_same_for_equal_values() {
    let a: PseudoRule = PseudoRule::new(String::from(":hover"), String::from("background: blue;"));
    let b: PseudoRule = PseudoRule::new(String::from(":hover"), String::from("background: blue;"));
    let mut h1: DefaultHasher = DefaultHasher::new();
    let mut h2: DefaultHasher = DefaultHasher::new();
    a.hash(&mut h1);
    b.hash(&mut h2);
    assert_eq!(h1.finish(), h2.finish());
}

#[test]
fn pseudo_rule_debug_format_works() {
    let rule: PseudoRule = PseudoRule::default();
    let formatted: String = format!("{:?}", rule);
    assert!(formatted.contains("PseudoRule"));
}

#[test]
fn media_rule_equality_same_values() {
    let a: MediaRule = MediaRule::new(
        String::from("(max-width: 767px)"),
        String::from("font-size: 14px;"),
        Vec::new(),
    );
    let b: MediaRule = MediaRule::new(
        String::from("(max-width: 767px)"),
        String::from("font-size: 14px;"),
        Vec::new(),
    );
    assert_eq!(a, b);
}

#[test]
fn media_rule_equality_different_queries() {
    let a: MediaRule = MediaRule::new(
        String::from("(max-width: 767px)"),
        String::from("font-size: 14px;"),
        Vec::new(),
    );
    let b: MediaRule = MediaRule::new(
        String::from("(min-width: 768px)"),
        String::from("font-size: 14px;"),
        Vec::new(),
    );
    assert_ne!(a, b);
}

#[test]
fn media_rule_hash_same_for_equal_values() {
    let a: MediaRule = MediaRule::new(
        String::from("(max-width: 767px)"),
        String::from("font-size: 14px;"),
        Vec::new(),
    );
    let b: MediaRule = MediaRule::new(
        String::from("(max-width: 767px)"),
        String::from("font-size: 14px;"),
        Vec::new(),
    );
    let mut h1: DefaultHasher = DefaultHasher::new();
    let mut h2: DefaultHasher = DefaultHasher::new();
    a.hash(&mut h1);
    b.hash(&mut h2);
    assert_eq!(h1.finish(), h2.finish());
}

#[test]
fn media_rule_debug_format_works() {
    let rule: MediaRule = MediaRule::default();
    let formatted: String = format!("{:?}", rule);
    assert!(formatted.contains("MediaRule"));
}

#[test]
fn attribute_entry_debug_format_works() {
    let entry: AttributeEntry = AttributeEntry::new(
        Cow::Borrowed("class"),
        AttributeValue::Text(String::from("btn")),
    );
    let formatted: String = format!("{:?}", entry);
    assert!(formatted.contains("AttributeEntry"));
}

#[test]
fn opt10_static_text_borrows_without_alloc() {
    let value: AttributeValue = AttributeValue::StaticText("color: red;");
    match value {
        AttributeValue::StaticText(s) => assert_eq!(s, "color: red;"),
        _ => panic!("expected AttributeValue::StaticText"),
    }
}

#[test]
fn opt10_static_text_matches_text_in_debug_layout() {
    let original: AttributeValue = AttributeValue::StaticText("color:red;");
    let cloned: AttributeValue = original.clone();
    let original_dbg: String = format!("{:?}", original);
    let cloned_dbg: String = format!("{:?}", cloned);
    assert!(
        original_dbg.contains("StaticText"),
        "expected original Debug to mention StaticText, got: {original_dbg}"
    );
    assert_eq!(
        original_dbg, cloned_dbg,
        "AttributeValue::clone changed the variant"
    );
    assert!(
        cloned_dbg.contains("color:red;"),
        "expected cloned value to retain the literal string, got: {cloned_dbg}"
    );
}

#[test]
fn opt11_from_static_css_yields_cssref_not_css() {
    use std::sync::LazyLock;
    static STATIC_CSS: LazyLock<Css> = LazyLock::new(|| {
        Css::new(
            String::from("opt11-fixture"),
            String::from("color: blue;"),
            Vec::new(),
            Vec::new(),
        )
    });
    let value: AttributeValue = (&*STATIC_CSS).into();
    match value {
        AttributeValue::CssRef(css_ref) => {
            assert_eq!(css_ref.get_name(), "opt11-fixture");
            assert_eq!(css_ref.get_style(), "color: blue;");
        }
        AttributeValue::Css(_) => panic!(
            "OPT 11 regression: `&'static Css` produced owned Css \
             variant; expected CssRef to skip the deep clone."
        ),
        other => panic!("expected AttributeValue::CssRef, got {other:?}"),
    }
}

#[test]
fn opt11_cssref_does_not_clone_inner_collections() {
    use std::sync::LazyLock;
    static STATIC_CSS: LazyLock<Css> = LazyLock::new(|| {
        Css::new(
            String::from("opt11-shared"),
            String::from("display: flex;"),
            Vec::new(),
            Vec::new(),
        )
    });
    let value: AttributeValue = AttributeValue::CssRef(&STATIC_CSS);
    let AttributeValue::CssRef(css_ref) = value else {
        panic!("expected AttributeValue::CssRef");
    };
    assert!(std::ptr::eq(
        css_ref as *const Css,
        &*STATIC_CSS as *const Css
    ));
}

#[test]
fn native_css_construct_does_not_panic() {
    let result: Result<(), String> = catch_unwind(AssertUnwindSafe(|| {
        let _: Css = Css::default();
        let _: PseudoRule = PseudoRule::default();
        let _: MediaRule = MediaRule::default();
    }))
    .map_err(|_| "panic".to_string());
    assert!(result.is_ok());
}

#[test]
fn native_pseudo_rule_clone_does_not_panic() {
    let result: Result<(), String> = catch_unwind(AssertUnwindSafe(|| {
        let rule: PseudoRule = PseudoRule::default();
        let cloned: PseudoRule = rule.clone();
        assert_eq!(rule, cloned);
    }))
    .map_err(|_| "panic".to_string());
    assert!(result.is_ok());
}

#[test]
fn native_media_rule_clone_does_not_panic() {
    let result: Result<(), String> = catch_unwind(AssertUnwindSafe(|| {
        let rule: MediaRule = MediaRule::default();
        let cloned: MediaRule = rule.clone();
        assert_eq!(rule, cloned);
    }))
    .map_err(|_| "panic".to_string());
    assert!(result.is_ok());
}

#[test]
fn merge_class_joins_text_segments_with_spaces() {
    let merged: AttributeValue = AttributeValue::merge_class(&[
        AttributeValue::Text(String::from("c_binding_slider")),
        AttributeValue::Text(String::from("c_slider_value-57289a1822494269")),
    ]);
    let AttributeValue::Text(merged_text) = merged else {
        panic!("merge_class should yield Text on the static path for Text inputs");
    };
    let parts: Vec<&str> = merged_text.split(' ').collect();
    assert_eq!(parts.len(), 2, "expected 2 segments, got `{merged_text}`");
    assert!(parts.contains(&"c_binding_slider"));
    assert!(parts.contains(&"c_slider_value-57289a1822494269"));
}

#[test]
fn merge_class_signal_path_preserves_text_siblings() {
    let signal_value: Signal<String> =
        Signal::create(String::from("c_binding_slider_label_accent"));
    let merged: AttributeValue = AttributeValue::merge_class(&[
        AttributeValue::Text(String::from("c_binding_slider_label")),
        AttributeValue::Signal(signal_value),
    ]);
    let AttributeValue::Signal(merged_signal) = merged else {
        panic!("merge_class should yield Signal when any input is Signal");
    };
    let resolved: String = merged_signal.get();
    let parts: Vec<&str> = resolved
        .split(' ')
        .filter(|s: &&str| !s.is_empty())
        .collect();
    assert!(
        parts.contains(&"c_binding_slider_label"),
        "Text sibling dropped on signal path: `{resolved}` is missing `c_binding_slider_label`"
    );
    assert!(
        parts.contains(&"c_binding_slider_label_accent"),
        "Signal value dropped: `{resolved}` is missing the accent class"
    );
}

fn pseudo(selector: &str, style: &str) -> PseudoRule {
    PseudoRule::new(selector.to_string(), style.to_string())
}

fn media(query: &str, style: &str, rules: Vec<PseudoRule>) -> MediaRule {
    MediaRule::new(query.to_string(), style.to_string(), rules)
}

#[test]
fn a_pseudo_rule_block_keeps_the_space_that_precedes_its_closing_brace() {
    let parsed: Vec<PseudoRule> = Css::parse_pseudo_rules(":hover { color: red; }");
    assert_eq!(
        parsed,
        vec![pseudo(":hover", "color: red; ")],
        "the open delimiter already swallowed the space before the brace, \
         so the body runs right up to the closing one"
    );
}

#[test]
fn several_pseudo_blocks_in_one_string_are_parsed_in_order() {
    let parsed: Vec<PseudoRule> =
        Css::parse_pseudo_rules(":hover { color: red; }:focus { color: blue; }");
    assert_eq!(
        parsed,
        vec![
            pseudo(":hover", "color: red; "),
            pseudo(":focus", "color: blue; ")
        ]
    );
}

#[test]
fn a_pseudo_block_body_keeps_its_own_semicolons_intact() {
    let parsed: Vec<PseudoRule> =
        Css::parse_pseudo_rules("::before { content: 'a;b'; margin: 0 auto; }");
    assert_eq!(
        parsed,
        vec![pseudo("::before", "content: 'a;b'; margin: 0 auto; ")],
        "only the first closing brace ends the block"
    );
}

#[test]
fn a_pseudo_block_needs_the_exact_space_brace_space_delimiter() {
    let spaced: Vec<PseudoRule> = Css::parse_pseudo_rules(":active { outline: none; }");
    assert_eq!(spaced, vec![pseudo(":active", "outline: none; ")]);
    assert!(
        Css::parse_pseudo_rules(":active{outline: none;}").is_empty(),
        "without the surrounding spaces the block is not a serialized rule at all"
    );
}

#[test]
fn a_pseudo_block_with_an_empty_selector_or_body_is_dropped() {
    assert!(
        Css::parse_pseudo_rules(" { color: red; }").is_empty(),
        "a rule with no selector cannot become a CSS rule"
    );
    assert!(
        Css::parse_pseudo_rules(":hover { }").is_empty(),
        "a rule with no declarations carries no style"
    );
    assert!(
        Css::parse_pseudo_rules(":hover {  }").len() == 1,
        "one space of body is not empty: the delimiter already consumed one"
    );
}

#[test]
fn an_unterminated_pseudo_block_yields_nothing_rather_than_panicking() {
    assert!(Css::parse_pseudo_rules(":hover { color: red;").is_empty());
    assert!(Css::parse_pseudo_rules("").is_empty());
    assert!(Css::parse_pseudo_rules("no braces at all").is_empty());
}

#[test]
fn a_media_rule_is_split_into_its_query_and_its_declarations() {
    let parsed: Vec<MediaRule> =
        Css::parse_media_rules("@media (max-width: 767px) { font-size: 14px; }");
    assert_eq!(parsed.len(), 1);
    assert_eq!(parsed[0], media("(max-width: 767px)", "font-size: 14px;", vec![]));
}

#[test]
fn a_nested_pseudo_block_is_extracted_only_when_it_comes_before_the_declarations() {
    let pseudo_first: Vec<MediaRule> = Css::parse_media_rules(
        "@media (max-width: 767px) { ::-webkit-scrollbar { width: 0px; } font-size: 14px; }",
    );
    assert_eq!(pseudo_first.len(), 1);
    assert_eq!(
        pseudo_first[0],
        media(
            "(max-width: 767px)",
            "font-size: 14px;",
            vec![pseudo("::-webkit-scrollbar", "width: 0px;")]
        ),
        "a body that opens with a selector is split into style plus pseudo rules"
    );
    let declaration_first: Vec<MediaRule> = Css::parse_media_rules(
        "@media (max-width: 767px) { font-size: 14px; ::-webkit-scrollbar { width: 0px; } }",
    );
    assert_eq!(declaration_first.len(), 1);
    assert_eq!(
        declaration_first[0],
        media(
            "(max-width: 767px)",
            "font-size: 14px; ::-webkit-scrollbar width: 0px;",
            vec![]
        ),
        "once a declaration precedes it the block loses its braces and is inlined \
         into the style string, keeping no pseudo rules of its own"
    );
}

#[test]
fn two_media_rules_in_one_string_are_parsed_in_order() {
    let parsed: Vec<MediaRule> = Css::parse_media_rules(
        "@media (max-width: 767px) { font-size: 14px; }@media (min-width: 1200px) { font-size: 20px; }",
    );
    assert_eq!(parsed.len(), 2);
    assert_eq!(parsed[0], media("(max-width: 767px)", "font-size: 14px;", vec![]));
    assert_eq!(parsed[1], media("(min-width: 1200px)", "font-size: 20px;", vec![]));
}

#[test]
fn a_nested_block_keeps_the_outer_media_rule_from_ending_early() {
    let parsed: Vec<MediaRule> = Css::parse_media_rules(
        "@media print { ::before { content: x; } font-size: 10px; }@media screen { font-size: 12px; }",
    );
    assert_eq!(
        parsed.len(),
        2,
        "the brace counter must count the nested block before closing the media rule"
    );
    assert_eq!(parsed[1], media("screen", "font-size: 12px;", vec![]));
}

#[test]
fn an_unbalanced_brace_inside_a_media_value_swallows_the_rest_instead_of_emitting_garbage() {
    let parsed: Vec<MediaRule> =
        Css::parse_media_rules("@media print { content: '{'; font-size: 10px; }");
    assert!(
        parsed.is_empty(),
        "the parser is not string-aware, so a literal brace leaves the block unterminated \
         and the rule is dropped rather than half-parsed"
    );
}

#[test]
fn a_media_rule_with_an_empty_body_or_query_is_dropped() {
    assert!(Css::parse_media_rules("@media print { }").is_empty());
    assert!(Css::parse_media_rules("@media  { font-size: 10px; }").is_empty());
}

#[test]
fn media_input_that_does_not_start_with_the_media_prefix_yields_nothing() {
    assert!(Css::parse_media_rules("font-size: 10px;").is_empty());
    assert!(Css::parse_media_rules("@medi { font-size: 10px; }").is_empty());
    assert!(Css::parse_media_rules("@media print { font-size: 10px;").is_empty());
}

#[test]
fn a_style_string_terminates_every_declaration_and_separates_them_with_spaces() {
    let styled: String = Css::style_string(&[("color", "red"), ("margin", "0 auto")]);
    assert_eq!(styled, "color: red; margin: 0 auto;");
}

#[test]
fn a_style_string_of_no_properties_is_empty() {
    let styled: String = Css::style_string::<&str, &str>(&[]);
    assert!(styled.is_empty());
}

#[test]
fn a_style_string_accepts_anything_that_is_as_ref_str() {
    let from_strings: String = Css::style_string(&[
        (String::from("color"), String::from("red")),
        (String::from("margin"), String::from("0")),
    ]);
    assert_eq!(from_strings, "color: red; margin: 0;");
    let from_strs: String = Css::style_string(&[("color", "red"), ("margin", "0")]);
    assert_eq!(from_strs, "color: red; margin: 0;");
}

#[test]
fn a_parameter_class_name_is_stable_and_distinguishes_different_values() {
    let first: String = Css::param_class_name("sm");
    let same: String = Css::param_class_name("sm");
    let other: String = Css::param_class_name("lg");
    assert_eq!(first, same, "the same value must always yield the same class");
    assert_ne!(first, other, "different values must not collide");
    assert!(
        first.chars().all(|c: char| c.is_ascii_alphanumeric()),
        "a hex suffix keeps the class name a valid identifier, got {first}"
    );
    assert!(!first.is_empty());
}

#[test]
fn merging_plain_text_styles_joins_them_with_single_spaces() {
    let merged: AttributeValue = AttributeValue::merge_style(&[
        AttributeValue::Text("color: red;".to_string()),
        AttributeValue::Text("margin: 0;".to_string()),
    ]);
    assert_eq!(
        merged,
        AttributeValue::Text("color: red; margin: 0;".to_string())
    );
}

#[test]
fn merging_skips_empty_segments_so_no_double_space_appears() {
    let merged: AttributeValue = AttributeValue::merge_style(&[
        AttributeValue::Text("color: red;".to_string()),
        AttributeValue::Text(String::new()),
        AttributeValue::Text("margin: 0;".to_string()),
    ]);
    assert_eq!(
        merged,
        AttributeValue::Text("color: red; margin: 0;".to_string())
    );
}

#[test]
fn merging_no_styles_at_all_yields_an_empty_text_value() {
    let merged: AttributeValue = AttributeValue::merge_style(&[]);
    assert_eq!(merged, AttributeValue::Text(String::new()));
}

#[test]
fn a_reactive_attribute_starts_at_the_value_its_closure_produces() {
    let calls: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let counter: Rc<Cell<u32>> = calls.clone();
    let attribute: AttributeValue = AttributeValue::reactive(move || {
        counter.set(counter.get() + 1);
        String::from("computed")
    });
    assert_eq!(calls.get(), 1, "the closure runs once to seed the signal");
    match attribute {
        AttributeValue::Signal(signal) => assert_eq!(signal.get(), "computed"),
        other => panic!("expected a signal-backed attribute, got {other:?}"),
    }
}

#[test]
fn a_reactive_attribute_is_distinct_from_a_literal_one() {
    let reactive: AttributeValue = AttributeValue::reactive(|| String::from("same"));
    let literal: AttributeValue = AttributeValue::Text(String::from("same"));
    assert!(
        matches!(reactive, AttributeValue::Signal(_)),
        "a closure-backed attribute must stay signal-backed"
    );
    assert!(matches!(literal, AttributeValue::Text(_)));
}

#[test]
fn an_event_adapter_becomes_an_event_attribute() {
    let attribute: AttributeValue = EventAdapter::new(|_event: Event| {}).into_attribute("click");
    assert!(
        matches!(attribute, AttributeValue::Event(_)),
        "an event handler must land in the Event variant, not as text"
    );
}

#[test]
fn the_owned_style_string_matches_the_borrowed_form() {
    let borrowed: String = Css::style_string(&[("color", "red"), ("margin", "0")]);
    let owned: String = Css::style_string_owned(&[
        (String::from("color"), String::from("red")),
        (String::from("margin"), String::from("0")),
    ]);
    assert_eq!(
        owned, borrowed,
        "the owned variant exists to avoid borrowing, not to format differently"
    );
}

#[test]
fn the_owned_style_string_of_nothing_is_empty() {
    let owned: String = Css::style_string_owned(&[]);
    assert!(owned.is_empty());
}
