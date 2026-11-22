use super::*;

fn mermaid_node(source: &'static str, ordinal: usize) -> VirtualNode<DocsMermaidProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(DocsMermaidProps { source, ordinal })),
    }
}

fn render_mermaid_diagram(source: &'static str, ordinal: usize) -> VirtualNode {
    HookContext::with(HookContext::current(), || {
        docs_mermaid(mermaid_node(source, ordinal))
    })
}

fn children_of(node: &VirtualNode) -> Vec<&VirtualNode> {
    match node {
        VirtualNode::Element { children, .. } => children.iter().collect(),
        VirtualNode::Fragment(children) => children.iter().collect(),
        _ => Vec::new(),
    }
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

fn find_by_class<'a>(node: &'a VirtualNode, name: &str) -> Option<&'a VirtualNode> {
    if has_class(node, name) {
        return Some(node);
    }
    for child in children_of(node) {
        if let Some(found) = find_by_class(child, name) {
            return Some(found);
        }
    }
    None
}

fn inner_html_payload(node: &VirtualNode) -> Option<String> {
    match node {
        VirtualNode::Element { attributes, .. } => {
            attributes.iter().find_map(|entry: &AttributeEntry| {
                if entry.get_name() != "inner_html" {
                    return None;
                }
                entry.inner_html_value()
            })
        }
        _ => None,
    }
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

fn scoped_sheet(svg: &str) -> String {
    let marker: &str = "<style data-merman-postprocess=\"scoped-css\">";
    let start: usize = svg.find(marker).expect("the scoped sheet");
    let tail: &str = &svg[start + marker.len()..];
    let end: usize = tail.find("</style>").expect("the scoped sheet ends");
    tail[..end].to_string()
}

const THEME_RULES: &[&str] = &[
    ".actor",
    ".actor-line",
    ".messageLine0",
    ".messageLine1",
    "rect.note[data-look]",
    "text.actor>tspan",
    ".noteText",
    ".messageText",
    ".labelBox",
    ".labelText",
    ".loopText",
    ".sectionTitle",
    ".loopLabel",
];

const THEME_TOKENS: &[&str] = &["var(--foreground)", "var(--muted)", "var(--border)"];

const SEQUENCE: &str = "sequenceDiagram\n    participant Client as \"客户端\"\n    participant Server as \"Server\"\n    Client->>Server: \"TCP 连接请求\"\n";

static BLOCKS: &[EuvMdBlock] = &[EuvMdBlock::CodeBlock {
    lang: "mermaid",
    code: SEQUENCE,
}];

#[test]
fn a_mermaid_fence_renders_an_svg_node() {
    let svg: Option<String> = render_mermaid(SEQUENCE, 0);
    let svg: String = svg.expect("the sequence diagram must render");
    let head: String = svg.chars().take(200).collect();
    assert!(
        svg.contains("<svg"),
        "the rendered diagram must be an inline svg, got {head}"
    );
    assert!(
        svg.contains("</svg>"),
        "the rendered diagram must be a closed svg element"
    );
}

#[test]
fn a_mermaid_fence_carries_its_labels() {
    let svg: Option<String> = render_mermaid(SEQUENCE, 0);
    let svg: String = svg.expect("the sequence diagram must render");
    assert!(
        svg.contains("客户端"),
        "the diagram must keep the CJK participant label"
    );
    assert!(
        svg.contains("TCP 连接请求"),
        "the diagram must keep the CJK message label"
    );
}

#[test]
fn a_mermaid_diagram_is_inlined_in_the_page() {
    let rendered: VirtualNode = render_mermaid_diagram(SEQUENCE, 0);
    let figure: &VirtualNode = find_by_class(&rendered, "c_docs_mermaid").expect("the figure");
    let frame: &VirtualNode = find_by_class(figure, "c_docs_mermaid_frame").expect("the frame");
    let payload: String = inner_html_payload(frame).expect("the frame must carry the svg inline");
    assert!(
        payload.contains("<svg"),
        "the frame must carry the svg itself, not an empty payload"
    );
    assert!(
        !attribute_names(frame)
            .iter()
            .any(|name: &String| name == "src"),
        "the frame must not reference an external asset"
    );
}

#[test]
fn a_mermaid_diagram_emits_no_javascript_or_stylesheet_asset() {
    let svg: Option<String> = render_mermaid(SEQUENCE, 0);
    let svg: String = svg.expect("the sequence diagram must render");
    for forbidden in [".js", ".css", "mermaid.min.js", "mermaid.js", "<script"] {
        assert!(
            !svg.contains(forbidden),
            "the rendered diagram must not carry {forbidden}"
        );
    }
    let rendered: VirtualNode = render_mermaid_diagram(SEQUENCE, 0);
    let names: Vec<String> = attribute_names(&rendered);
    for forbidden in ["src", "href"] {
        assert!(
            !names.iter().any(|name: &String| name == forbidden),
            "the diagram must not carry an external {forbidden} reference"
        );
    }
}

#[test]
fn a_mermaid_diagram_follows_the_theme_tokens() {
    let svg: Option<String> = render_mermaid(SEQUENCE, 0);
    let svg: String = svg.expect("the sequence diagram must render");
    for rule in THEME_RULES {
        assert!(
            svg.contains(rule),
            "the diagram must style {rule} with a theme token"
        );
    }
    for token in THEME_TOKENS {
        assert!(
            svg.contains(token),
            "the diagram must paint with the docs theme token {token}"
        );
    }
    assert!(
        !svg.contains("!important"),
        "the diagram must not need an important override to follow the theme"
    );
    let sheet: String = scoped_sheet(&svg);
    for literal in ["#000000", "#ffffff", "#28253D"] {
        assert!(
            !sheet.contains(literal),
            "the scoped sheet must not paint the diagram with {literal}"
        );
    }
    assert!(
        !scoped_sheet(&svg).contains("!important"),
        "the scoped sheet must not need an important override"
    );
}

fn render_page_block(index: usize) -> VirtualNode {
    HookContext::with(HookContext::current(), || {
        render_docs_block(BLOCKS, index, &BLOCKS[index])
    })
}

#[test]
fn a_mermaid_fence_on_a_page_renders_the_diagram() {
    let rendered: VirtualNode = render_page_block(0);
    assert!(
        find_by_class(&rendered, "c_docs_mermaid").is_some(),
        "a mermaid fence on a page must render the diagram"
    );
    assert!(
        find_by_class(&rendered, "c_docs_code").is_none(),
        "a mermaid fence on a page must not render the code block"
    );
}

#[test]
fn an_ordinary_fence_on_a_page_renders_the_code_block() {
    static RUST_BLOCKS: &[EuvMdBlock] = &[EuvMdBlock::CodeBlock {
        lang: "rust",
        code: "fn main() {}",
    }];
    let rendered: VirtualNode = HookContext::with(HookContext::current(), || {
        render_docs_block(RUST_BLOCKS, 0, &RUST_BLOCKS[0])
    });
    assert!(
        find_by_class(&rendered, "c_docs_code").is_some(),
        "an ordinary fence on a page must render the code block"
    );
    assert!(
        find_by_class(&rendered, "c_docs_mermaid").is_none(),
        "an ordinary fence on a page must not render a diagram"
    );
}

#[test]
fn a_mermaid_fence_is_routed_to_the_diagram() {
    assert_eq!(
        code_block_route("mermaid"),
        CodeBlockRoute::Diagram,
        "a mermaid fence must be routed to the diagram"
    );
}

#[test]
fn an_ordinary_fence_is_not_routed_to_the_diagram() {
    for language in ["rust", "shell", "json", "text", ""] {
        assert_eq!(
            code_block_route(language),
            CodeBlockRoute::Code,
            "a {language} fence must not be routed to the diagram"
        );
    }
}

#[test]
fn a_mermaid_fence_is_not_confused_with_a_longer_language() {
    assert_eq!(
        code_block_route("mermaidjs"),
        CodeBlockRoute::Code,
        "a language that merely starts with mermaid is not a mermaid fence"
    );
    assert_eq!(
        code_block_route("mermaid-lang"),
        CodeBlockRoute::Code,
        "a language that merely starts with mermaid is not a mermaid fence"
    );
}

#[test]
fn an_unrenderable_mermaid_fence_falls_back_to_the_code_block() {
    let rendered: VirtualNode = render_mermaid_diagram("this is not a diagram", 0);
    assert!(
        find_by_class(&rendered, "c_docs_code").is_some(),
        "a diagram the engine cannot lay out must degrade to the code block"
    );
    assert!(
        find_by_class(&rendered, "c_docs_mermaid").is_none(),
        "a diagram the engine cannot lay out must not render a broken figure"
    );
}

#[test]
fn a_mermaid_diagram_carries_no_inline_colour() {
    let svg: Option<String> = render_mermaid(SEQUENCE, 0);
    let svg: String = svg.expect("the sequence diagram must render");
    for inline in ["style=\"stroke:", "style=\"color:"] {
        assert!(
            !svg.contains(inline),
            "the diagram must not paint with an inline {inline}, or the page's theme cannot reach it"
        );
    }
    for inline in ["style=\"fill: rgb", "style=\"fill: #", "style=\"fill: hsl"] {
        assert!(
            !svg.contains(inline),
            "the diagram must not paint with an inline {inline}, or the page's theme cannot reach it"
        );
    }
}

#[test]
fn a_mermaid_diagram_paints_its_text_with_the_theme_token() {
    let svg: Option<String> = render_mermaid(SEQUENCE, 0);
    let svg: String = svg.expect("the sequence diagram must render");
    assert!(
        scoped_sheet(&svg).contains("text.actor>tspan"),
        "the scoped sheet must style the actor labels"
    );
    assert!(
        scoped_sheet(&svg).contains("var(--foreground)"),
        "the actor labels must be painted with the theme token"
    );
}

#[test]
fn the_theme_rules_and_the_stylesheet_agree() {
    let sheet: String = SCOPED_CSS.to_string();
    for rule in THEME_RULES {
        assert!(
            sheet.contains(rule),
            "the stylesheet must carry the rule {rule}"
        );
    }
    for token in THEME_TOKENS {
        assert!(
            sheet.contains(token),
            "the stylesheet must paint with the theme token {token}"
        );
    }
    let selectors: Vec<&str> = THEME_RULES.to_vec();
    assert_eq!(
        selectors.len(),
        13,
        "the rule list must name every rule the stylesheet declares"
    );
}
