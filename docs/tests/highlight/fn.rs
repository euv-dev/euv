use super::*;

fn kinds(code: &str) -> Vec<CodeTokenKind> {
    tokenize("rust", code)
        .iter()
        .map(|token: &CodeToken| token.kind)
        .collect()
}

fn texts(code: &str) -> Vec<String> {
    tokenize("rust", code)
        .iter()
        .map(|token: &CodeToken| token.text.clone())
        .collect()
}

#[test]
fn a_rust_keyword_is_a_keyword() {
    assert!(kinds("fn").contains(&CodeTokenKind::Keyword));
}

#[test]
fn a_rust_function_name_is_a_function() {
    let tokens: Vec<CodeToken> = tokenize("rust", "fn main() {}");
    assert!(
        tokens
            .iter()
            .any(|token: &CodeToken| token.kind == CodeTokenKind::Function && token.text == "main")
    );
}

#[test]
fn a_rust_type_name_is_a_type() {
    let tokens: Vec<CodeToken> = tokenize("rust", "struct Holder {}");
    assert!(
        tokens
            .iter()
            .any(|token: &CodeToken| token.kind == CodeTokenKind::Type && token.text == "Holder")
    );
}

#[test]
fn a_rust_line_comment_is_a_comment() {
    assert!(kinds("// note").contains(&CodeTokenKind::Comment));
}

#[test]
fn a_rust_block_comment_is_a_comment() {
    assert!(kinds("/* note */").contains(&CodeTokenKind::Comment));
}

#[test]
fn a_rust_string_is_a_string() {
    assert!(kinds("\"text\"").contains(&CodeTokenKind::String));
}

#[test]
fn a_rust_attribute_is_an_attribute() {
    assert!(kinds("#[derive(Debug)]").contains(&CodeTokenKind::Attribute));
}

#[test]
fn a_rust_number_is_a_number() {
    assert!(kinds("42").contains(&CodeTokenKind::Number));
}

#[test]
fn the_segments_keep_the_source_order() {
    assert_eq!(texts("fn main() {}"), vec!["fn", " ", "main", "() {}"]);
}

#[test]
fn a_shell_command_is_a_function() {
    let tokens: Vec<CodeToken> = tokenize("bash", "cargo build");
    assert!(tokens
        .iter()
        .any(|token: &CodeToken| token.kind == CodeTokenKind::Function && token.text == "cargo"));
}

#[test]
fn a_shell_variable_is_a_variable() {
    assert!(
        tokenize("bash", "echo $HOME")
            .iter()
            .any(|token: &CodeToken| token.kind == CodeTokenKind::Variable)
    );
}

#[test]
fn a_json_key_is_an_attribute() {
    let tokens: Vec<CodeToken> = tokenize("json", "{\"name\": \"euv\"}");
    assert!(tokens.iter().any(
        |token: &CodeToken| token.kind == CodeTokenKind::Attribute && token.text == "\"name\""
    ));
}

#[test]
fn a_json_value_is_a_string() {
    let tokens: Vec<CodeToken> = tokenize("json", "{\"name\": \"euv\"}");
    assert!(tokens
        .iter()
        .any(|token: &CodeToken| token.kind == CodeTokenKind::String && token.text == "\"euv\""));
}

#[test]
fn a_json_literal_is_a_keyword() {
    assert!(
        tokenize("json", "true")
            .iter()
            .any(|token: &CodeToken| token.kind == CodeTokenKind::Keyword)
    );
}

#[test]
fn an_html_tag_is_a_tag() {
    assert!(
        tokenize("html", "<div>")
            .iter()
            .any(|token: &CodeToken| token.kind == CodeTokenKind::Tag && token.text == "div")
    );
}

#[test]
fn an_html_attribute_is_an_attribute() {
    assert!(
        tokenize("html", "<div id=\"x\">")
            .iter()
            .any(|token: &CodeToken| token.kind == CodeTokenKind::Attribute && token.text == "id")
    );
}

#[test]
fn a_css_property_is_an_attribute() {
    assert!(
        tokenize("css", "a { color: red; }")
            .iter()
            .any(
                |token: &CodeToken| token.kind == CodeTokenKind::Attribute && token.text == "color"
            )
    );
}

#[test]
fn a_css_selector_is_a_tag() {
    assert!(
        tokenize("css", "a { color: red; }")
            .iter()
            .any(|token: &CodeToken| token.kind == CodeTokenKind::Tag && token.text == "a")
    );
}

#[test]
fn an_unknown_language_is_left_plain() {
    let tokens: Vec<CodeToken> = tokenize("brainfuck", "+[-]");
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, CodeTokenKind::Plain);
    assert_eq!(tokens[0].text, "+[-]");
}

#[test]
fn empty_code_yields_no_segment() {
    assert!(tokenize("rust", "").is_empty());
}

#[test]
fn the_markup_escapes_the_code() {
    let html: String = highlighted_html("rust", "a < b && c > d");
    assert!(html.contains("&lt;"));
    assert!(html.contains("&gt;"));
    assert!(html.contains("&amp;"));
    assert!(!html.contains("< b"));
}

#[test]
fn the_markup_wraps_a_keyword_in_its_class() {
    let html: String = highlighted_html("rust", "fn");
    assert_eq!(html, "<span class=\"c_docs_code_kw\">fn</span>");
}

#[test]
fn the_markup_leaves_plain_text_unwrapped() {
    let html: String = highlighted_html("rust", "x");
    assert_eq!(html, "x");
}

#[test]
fn the_markup_of_an_empty_block_is_empty() {
    assert_eq!(highlighted_html("rust", ""), "");
}
