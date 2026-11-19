use super::*;

#[test]
fn markdown_inline_code_class_is_attached_only_to_the_inline_render_branch() {
    let source: &str = include_str!("../../src/component/markdown/view/fn.rs");
    let inline: &str = source
        .split_once("EuvMdInline::Code(code) =>")
        .unwrap()
        .1
        .split_once("EuvMdInline::Link")
        .unwrap()
        .0;
    assert!(inline.contains("code {"));
    assert!(inline.contains("class: c_euv_markdown_inline_code()"));
    assert!(inline.contains("{ *code }"));
    let fence: &str = source
        .split_once("EuvMdBlock::CodeBlock { lang, code } =>")
        .unwrap()
        .1
        .split_once("EuvMdBlock::BlockQuote")
        .unwrap()
        .0;
    assert!(!fence.contains("c_euv_markdown_inline_code"));
}

#[test]
fn markdown_inline_code_obeys_theme_tokens_and_custom_isolation() {
    let css: &Css = c_euv_markdown_inline_code();
    assert!(css.get_pseudo_rules().is_empty());
    {
        assert_eq!(
            css.get_style().replace(' ', "").trim_end_matches(';'),
            "border-radius:var(--radius-sm)"
        );
    }
}
