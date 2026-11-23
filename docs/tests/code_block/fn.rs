use super::*;

fn code_block_node(lang: &'static str, code: &'static str) -> VirtualNode<DocsCodeBlockProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(DocsCodeBlockProps { lang, code })),
    }
}

fn render_code_block(lang: &'static str, code: &'static str) -> VirtualNode {
    HookContext::with(HookContext::current(), || {
        docs_code_block(code_block_node(lang, code))
    })
}

fn own_classes(node: &VirtualNode) -> Vec<String> {
    match node {
        VirtualNode::Element { attributes, .. } => {
            let mut found: Vec<String> = Vec::new();
            for entry in attributes.iter() {
                if entry.get_name() == "class" {
                    found.extend(entry.class_names());
                }
            }
            found
        }
        _ => Vec::new(),
    }
}

fn has_class(node: &VirtualNode, name: &str) -> bool {
    own_classes(node).iter().any(|entry: &String| entry == name)
}

fn tree_has_class(node: &VirtualNode, name: &str) -> bool {
    if has_class(node, name) {
        return true;
    }
    children_of(node)
        .iter()
        .any(|child: &&VirtualNode| tree_has_class(child, name))
}

fn children_of(node: &VirtualNode) -> Vec<&VirtualNode> {
    match node {
        VirtualNode::Element { children, .. } => children.iter().collect(),
        VirtualNode::Fragment(children) => children.iter().collect(),
        _ => Vec::new(),
    }
}

fn find_by_class<'a>(node: &'a VirtualNode, name: &str) -> Option<&'a VirtualNode> {
    for child in children_of(node) {
        if has_class(child, name) {
            return Some(child);
        }
        if let Some(found) = find_by_class(child, name) {
            return Some(found);
        }
    }
    None
}

fn attribute_names(node: &VirtualNode) -> Vec<String> {
    match node {
        VirtualNode::Element { attributes, .. } => attributes
            .iter()
            .map(|entry: &AttributeEntry| entry.get_name().to_string())
            .collect(),
        _ => Vec::new(),
    }
}

fn child_count(node: &VirtualNode) -> usize {
    match node {
        VirtualNode::Element { children, .. } => children.len(),
        VirtualNode::Fragment(children) => children.len(),
        _ => 0,
    }
}

#[test]
fn the_code_block_renders_its_frame() {
    let rendered: VirtualNode = render_code_block("rust", "fn main() {}");
    assert!(tree_has_class(&rendered, "c_docs_code"));
}

#[test]
fn the_code_block_renders_a_toolbar() {
    let rendered: VirtualNode = render_code_block("rust", "fn main() {}");
    assert!(tree_has_class(&rendered, "c_docs_code_header"));
}

#[test]
fn the_toolbar_names_the_language() {
    let rendered: VirtualNode = render_code_block("rust", "fn main() {}");
    let label: Option<&VirtualNode> = find_by_class(&rendered, "c_docs_code_lang");
    assert!(label.is_some());
    assert!(child_count(label.unwrap_or(&rendered)) > 0);
}

#[test]
fn the_code_block_renders_a_copy_button() {
    let rendered: VirtualNode = render_code_block("rust", "fn main() {}");
    assert!(tree_has_class(&rendered, "c_docs_code_copy"));
}

#[test]
fn the_copy_button_is_a_button_element() {
    let rendered: VirtualNode = render_code_block("rust", "fn main() {}");
    let button: Option<&VirtualNode> = find_by_class(&rendered, "c_docs_code_copy");
    let button: &VirtualNode = button.unwrap_or(&rendered);
    assert!(format!("{button:?}").contains("Element(\"button\")"));
}

#[test]
fn the_copy_button_carries_a_click_handler() {
    let rendered: VirtualNode = render_code_block("rust", "fn main() {}");
    let button: Option<&VirtualNode> = find_by_class(&rendered, "c_docs_code_copy");
    let button: &VirtualNode = button.unwrap_or(&rendered);
    assert!(
        attribute_names(button)
            .iter()
            .any(|name: &String| name == "onclick")
    );
}

#[test]
fn the_copy_button_names_its_action() {
    let rendered: VirtualNode = render_code_block("rust", "fn main() {}");
    let button: Option<&VirtualNode> = find_by_class(&rendered, "c_docs_code_copy");
    let button: &VirtualNode = button.unwrap_or(&rendered);
    assert!(
        attribute_names(button)
            .iter()
            .any(|name: &String| name == "aria-label")
    );
}

#[test]
fn the_copy_button_carries_its_label() {
    let rendered: VirtualNode = render_code_block("rust", "fn main() {}");
    let button: Option<&VirtualNode> = find_by_class(&rendered, "c_docs_code_copy");
    let button: &VirtualNode = button.unwrap_or(&rendered);
    assert!(child_count(button) > 0);
}

#[test]
fn the_code_block_renders_the_code_in_a_pre() {
    let rendered: VirtualNode = render_code_block("rust", "fn main() {}");
    assert!(format!("{rendered:?}").contains("Element(\"pre\")"));
}

#[test]
fn the_code_block_renders_the_code_in_a_code_element() {
    let rendered: VirtualNode = render_code_block("rust", "fn main() {}");
    assert!(format!("{rendered:?}").contains("Element(\"code\")"));
}

#[test]
fn the_code_block_carries_the_highlighted_markup() {
    let rendered: VirtualNode = render_code_block("rust", "fn main() {}");
    assert!(format!("{rendered:?}").contains("inner_html"));
}

#[test]
fn a_code_block_of_an_unknown_language_still_renders() {
    let rendered: VirtualNode = render_code_block("brainfuck", "+[-]");
    assert!(tree_has_class(&rendered, "c_docs_code"));
}

fn scoped_rule(selector: &str, declarations: &[(&str, &str)]) -> PseudoRule {
    let style: String = declarations
        .iter()
        .map(|(property, value): &(&str, &str)| format!("{property}: {value}; "))
        .collect();
    PseudoRule::new(selector.to_string(), style)
}

fn attribute_text(node: &VirtualNode, name: &str) -> Option<String> {
    match node {
        VirtualNode::Element { attributes, .. } => attributes
            .iter()
            .find(|entry: &&AttributeEntry| entry.get_name() == name)
            .and_then(AttributeEntry::text_value),
        _ => None,
    }
}

#[test]
fn code_block_content_classes_are_attached_to_their_native_tags() {
    let rendered: VirtualNode = render_code_block("rust", "fn main() {}");
    let pre: &VirtualNode = find_by_class(&rendered, "c_docs_code_pre").expect("scoped pre");
    let code: &VirtualNode = find_by_class(pre, "c_docs_code_content").expect("scoped code");
    assert!(matches!(pre, VirtualNode::Element { tag: Tag::Element(tag), .. } if tag == "pre"));
    assert!(matches!(code, VirtualNode::Element { tag: Tag::Element(tag), .. } if tag == "code"));
    assert!(
        attribute_names(code)
            .iter()
            .any(|name: &String| name == "inner_html")
    );
}

#[test]
fn code_block_pre_rule_is_class_local_and_unframed() {
    let class: &Css = c_docs_code_pre();
    assert_eq!(class.get_name(), "c_docs_code_pre");
    assert!(class.get_style().is_empty());
    assert_eq!(
        class.get_pseudo_rules(),
        &vec![scoped_rule(
            ":is(pre.c_docs_code_pre)",
            &[
                ("margin", "0"),
                ("padding", "var(--space-lg)"),
                ("border", "none"),
                ("border-radius", "0"),
                ("box-shadow", "none"),
                ("background", "transparent"),
                ("color", "var(--docs-code-foreground)"),
                ("font-family", "var(--docs-code-font-family)"),
                ("font-size", "var(--font-xs)"),
                ("line-height", "1.6"),
                ("white-space", "pre"),
                ("overflow-wrap", "normal"),
                ("overflow-x", "auto"),
            ]
        )]
    );
}

#[test]
fn code_block_content_rule_removes_inline_markdown_geometry_without_touching_tokens() {
    let class: &Css = c_docs_code_content();
    assert_eq!(class.get_name(), "c_docs_code_content");
    assert!(class.get_style().is_empty());
    assert_eq!(
        class.get_pseudo_rules(),
        &vec![scoped_rule(
            ":is(code.c_docs_code_content)",
            &[
                ("display", "block"),
                ("margin", "0"),
                ("padding", "0"),
                ("border", "none"),
                ("border-radius", "0"),
                ("box-shadow", "none"),
                ("background", "transparent"),
                ("color", "inherit"),
                ("font-family", "inherit"),
                ("font-size", "inherit"),
                ("line-height", "inherit"),
                ("white-space", "pre"),
                ("overflow-wrap", "normal"),
            ]
        )]
    );
    assert!(
        CODE_CSS
            .contains(".c_docs_code_kw { color: var(--docs-code-keyword); font-weight: bold; }")
    );
    assert!(!CODE_CSS.contains("border:"));
}

#[test]
fn code_block_qualified_selectors_outrank_the_injected_markdown_rules() {
    let markdown: &str = euv_md_css();
    assert!(markdown.contains(".md-body pre{"));
    assert!(markdown.contains(".md-body code{"));
    assert!(markdown.contains(".md-body pre code{"));
    for (class, suffix) in [
        (c_docs_code_pre(), ":is(pre.c_docs_code_pre)"),
        (c_docs_code_content(), ":is(code.c_docs_code_content)"),
    ] {
        let selector: String = format!(".{}{suffix}", class.get_name());
        let classes: usize = selector.matches(".c_docs_code_").count();
        assert_eq!(classes, 2);
        assert!((0, classes, 1) > (0, 1, 2));
        assert!(!selector.contains(' '));
        assert!(!format!("{class:?}").contains("!important"));
    }
    assert!(c_docs_code().get_pseudo_rules().is_empty());
}

#[test]
fn code_block_has_one_exterior_frame_and_a_ghost_copy_action() {
    let frame: &Css = c_docs_code();
    assert!(
        frame
            .get_style()
            .contains("border: var(--border-width) solid var(--border);")
    );
    assert!(
        frame
            .get_style()
            .contains("border-radius: var(--radius-lg);")
    );
    for class in [c_docs_code_header(), c_docs_code_copy()] {
        assert!(class.get_style().contains("border: none;"));
        assert!(!class.get_style().contains("border-bottom:"));
        assert!(!class.get_style().contains("box-shadow:"));
        assert!(!class.get_style().contains("transition:"));
        assert!(!class.get_style().contains("animation:"));
    }
    let copy: &Css = c_docs_code_copy();
    assert!(copy.get_style().contains("background: transparent;"));
    assert_eq!(
        copy.get_pseudo_rules(),
        &vec![scoped_rule(
            ":hover",
            &[
                ("background", "var(--accent)"),
                ("color", "var(--accent-foreground)"),
            ]
        )]
    );
}

#[test]
fn code_block_keeps_long_plain_lines_intact_for_horizontal_scrolling() {
    let source: &'static str = "\t<unknown>&abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz\n    indented\n";
    let rendered: VirtualNode = render_code_block("unknown", source);
    let pre: &VirtualNode = find_by_class(&rendered, "c_docs_code_pre").expect("scroll container");
    let code: &VirtualNode =
        find_by_class(pre, "c_docs_code_content").expect("preformatted content");
    assert!(
        attribute_names(code)
            .iter()
            .any(|name: &String| name == "inner_html")
    );
    let html: String = highlighted_html("unknown", source);
    assert_eq!(
        html,
        source
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
    );
    assert!(!html.contains("<br"));
    assert!(!html.contains("<span"));
    assert!(html.lines().next().expect("long line").len() > 200);
}

#[test]
fn code_block_preserves_escaped_markup_whitespace_and_copy_feedback_labels() {
    let source: &'static str = "\tlet tag = \"<script>&\";\n    // keep spaces\n";
    let rendered: VirtualNode = render_code_block("rust", source);
    let code: &VirtualNode = find_by_class(&rendered, "c_docs_code_content").expect("scoped code");
    assert!(
        attribute_names(code)
            .iter()
            .any(|name: &String| name == "inner_html")
    );
    let html: String = highlighted_html("rust", source);
    assert!(html.starts_with('\t'));
    assert!(html.contains("&lt;script&gt;&amp;"));
    assert!(html.contains("\n    "));
    assert!(html.ends_with('\n'));
    assert!(html.contains("c_docs_code_kw"));
    assert!(!html.contains("<script>"));
    let button: &VirtualNode = find_by_class(&rendered, "c_docs_code_copy").expect("copy action");
    assert_eq!(
        attribute_text(button, "aria-label").as_deref(),
        Some("Copy code to clipboard")
    );
    assert!(
        attribute_names(button)
            .iter()
            .any(|name: &String| name == "onclick")
    );
    assert_eq!(child_count(button), 3);
    assert_eq!(
        [COPY_LABEL, COPIED_LABEL, FAILED_LABEL],
        ["Copy", "Copied", "Failed"]
    );
    for span in children_of(button) {
        assert_eq!(child_count(span), 1);
        assert!(matches!(children_of(span)[0], VirtualNode::Dynamic(_)));
    }
    assert_eq!(
        own_classes(children_of(button)[0]),
        vec!["c_docs_code_label"]
    );
    for label in &children_of(button)[1..] {
        assert_eq!(own_classes(label), vec!["c_docs_code_label_hidden"]);
    }
}
