use super::*;

#[test]
fn unsafe_no_inline_macro_emits_raw_html_from_string_literal() {
    let raw: RawHtml = unsafe_no_inline!("<b>bold</b>");
    assert_eq!(raw.get_content(), "<b>bold</b>");
}

#[test]
fn unsafe_no_inline_macro_preserves_empty_string() {
    let raw: RawHtml = unsafe_no_inline!("");
    assert_eq!(raw.get_content(), "");
}

#[test]
fn unsafe_no_inline_macro_preserves_html_with_attributes() {
    let raw: RawHtml = unsafe_no_inline!(r#"<span class=tag data-kind=note>text</span>"#);
    assert!(raw.get_content().contains("span"));
    assert!(raw.get_content().contains("class=tag"));
    assert!(raw.get_content().contains("data-kind=note"));
}

#[test]
fn unsafe_no_inline_macro_default_creates_empty_raw_html() {
    let empty: RawHtml = RawHtml::default();
    assert_eq!(empty.get_content(), "");
}
