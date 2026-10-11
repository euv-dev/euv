use super::*;

fn const_source() -> String {
    let path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "src", "build", "const.rs"]
        .iter()
        .collect();
    fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("cannot read {}: {error}", path.display()))
}

fn raw_string_constants(source: &str) -> Vec<(String, String)> {
    let mut found: Vec<(String, String)> = Vec::new();
    let mut cursor: usize = 0;
    while let Some(open) = source[cursor..].find("r#\"") {
        let body_start: usize = cursor + open + 3;
        let close: usize = match source[body_start..].find("\"#;") {
            Some(offset) => body_start + offset,
            None => break,
        };
        let head: &str = &source[cursor..body_start];
        let name: String = head
            .rsplit("const ")
            .next()
            .and_then(|rest| rest.split(':').next())
            .unwrap_or_default()
            .trim()
            .to_string();
        found.push((name, source[body_start..close].to_string()));
        cursor = close + 3;
    }
    found
}

fn loader_style() -> String {
    raw_string_constants(&const_source())
        .into_iter()
        .find(|(name, _)| name == "LOADING_STYLE")
        .map(|(_, value)| value)
        .expect("LOADING_STYLE is missing from const.rs")
}

#[test]
fn a_stray_quote_on_a_raw_string_edge_renders_as_visible_text_in_the_page() {
    let source: String = const_source();
    let constants: Vec<(String, String)> = raw_string_constants(&source);
    assert!(
        !constants.is_empty(),
        "no raw string constants parsed out of const.rs — the scanner is broken"
    );
    let offenders: Vec<String> = constants
        .iter()
        .filter(|(_, value)| value.starts_with('"') || value.ends_with('"'))
        .map(|(name, value)| format!("{name}: starts/ends with a bare quote ({value:?})"))
        .collect();
    assert!(
        offenders.is_empty(),
        "a raw string literal has a stray quote at its edge, which renders as \
         visible text in the generated page:\n{}",
        offenders.join("\n")
    );
}

#[test]
fn a_truncated_html_template_would_swallow_the_rest_of_the_document() {
    let source: String = const_source();
    for (name, value) in raw_string_constants(&source) {
        if !name.contains("HTML") {
            continue;
        }
        assert!(
            value.trim_end().ends_with("</html>"),
            "{name} does not end with </html>: {:?}",
            &value[value.len().saturating_sub(40)..]
        );
    }
}

#[test]
fn an_unbalanced_style_tag_lets_the_rest_of_head_leak_into_the_body() {
    let style: String = loader_style();
    assert_eq!(
        style.matches("<style>").count(),
        1,
        "expected exactly one <style> opener in {style:?}"
    );
    assert_eq!(
        style.matches("</style>").count(),
        1,
        "expected exactly one </style> closer in {style:?}"
    );
}

#[test]
fn the_loader_style_closes_on_its_own_closing_tag() {
    let style: String = loader_style();
    assert!(
        !style.trim_end().ends_with('"'),
        "LOADING_STYLE ends with a bare quote: {:?}",
        &style[style.len().saturating_sub(20)..]
    );
}
