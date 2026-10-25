use super::*;

fn attribute_key(input: ParseStream) -> Result<bool> {
    let matched: bool = is_attr_key_pattern(input);
    while !input.is_empty() {
        let _: TokenTree = input.parse()?;
    }
    Ok(matched)
}

#[test]
fn internal_attribute_key_parser_accepts_keywords_but_not_namespace_paths() {
    assert!(attribute_key.parse_str("type: text").unwrap());
    assert!(attribute_key.parse_str("async-onclick: handler").unwrap());
    assert!(!attribute_key.parse_str("module::value").unwrap());
}

#[test]
fn internal_identifier_parser_preserves_css_names_and_cleans_raw_identifiers() {
    assert_eq!(
        parse_ident_name.parse_str("r#box-shadow").unwrap(),
        "box-shadow"
    );
    assert_eq!(
        parse_ident_name.parse_str("--custom-property").unwrap(),
        "--custom-property"
    );
    assert_eq!(
        parse_ident_name.parse_str("word-break").unwrap(),
        "word-break"
    );
}
