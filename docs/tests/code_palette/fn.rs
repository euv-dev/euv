use super::*;

const TOKEN_CLASSES: [(&str, &str); 9] = [
    ("c_docs_code_kw", "keyword"),
    ("c_docs_code_str", "string"),
    ("c_docs_code_num", "number"),
    ("c_docs_code_com", "comment"),
    ("c_docs_code_fn", "function"),
    ("c_docs_code_ty", "type"),
    ("c_docs_code_attr", "attribute"),
    ("c_docs_code_tag", "tag"),
    ("c_docs_code_var", "variable"),
];

const LIGHT_PALETTE: [(&str, &str); 11] = [
    ("keyword", "#7C4DFF"),
    ("string", "#53A053"),
    ("number", "#FF6D12"),
    ("comment", "#A9A9AA"),
    ("function", "#437AED"),
    ("type", "#00BEC4"),
    ("attribute", "#00BEC4"),
    ("tag", "#CD6069"),
    ("variable", "#5D5D5F"),
    ("surface", "#EBEEF5"),
    ("foreground", "#5D5D5F"),
];

const DARK_PALETTE: [(&str, &str); 11] = [
    ("keyword", "#A78CFA"),
    ("string", "#98C379"),
    ("number", "#FF9070"),
    ("comment", "#676E95"),
    ("function", "#6495EE"),
    ("type", "#56B7C3"),
    ("attribute", "#56B7C3"),
    ("tag", "#E06C75"),
    ("variable", "#B0B7C3"),
    ("surface", "#282c34"),
    ("foreground", "#9DA5B3"),
];

const EDITOR_FONT_FAMILY: &str = "Menlo, Monaco, 'Courier New', monospace";

fn rule_body(css: &str, selector: &str) -> String {
    let marker: String = format!("{selector} {{");
    let start: usize = css
        .find(&marker)
        .unwrap_or_else(|| panic!("the stylesheet has no rule for {selector}"));
    let body: &str = &css[start + marker.len()..];
    let end: usize = body
        .find('}')
        .unwrap_or_else(|| panic!("the rule for {selector} is never closed"));
    body[..end].trim().to_string()
}

fn declaration(body: &str, property: &str) -> String {
    for part in body.split(';') {
        let Some((name, value)) = part.split_once(':') else {
            continue;
        };
        if name.trim() == property {
            return value.trim().to_string();
        }
    }
    panic!("the rule declares no {property}");
}

fn palette_body(css: &str, selector: &str) -> String {
    let marker: String = format!("{selector} {{");
    let start: usize = css
        .find(&marker)
        .unwrap_or_else(|| panic!("the stylesheet has no {selector} block"));
    let body: &str = &css[start + marker.len()..];
    let end: usize = body
        .find('}')
        .unwrap_or_else(|| panic!("the {selector} block is never closed"));
    body[..end].to_string()
}

fn assert_palette(mode: &str, selector: &str, palette: &[(&str, &str)]) {
    let css: &str = CODE_CSS;
    let block: String = palette_body(css, selector);
    for (name, colour) in palette {
        let declaration: String = format!("--docs-code-{name}: {colour};");
        assert!(
            block.contains(&declaration),
            "the {mode} palette does not declare {declaration} — the block reads:\n{block}"
        );
    }
}

#[test]
fn the_light_palette_is_the_editor_themes_light_face() {
    assert_palette("light", ":root", &LIGHT_PALETTE);
}

#[test]
fn the_dark_palette_is_the_editor_themes_own_dark_face() {
    assert_palette("dark", ".c_theme_dark", &DARK_PALETTE);
}

#[test]
fn the_two_palettes_are_not_collapsed_into_one() {
    let light: String = palette_body(CODE_CSS, ":root");
    let dark: String = palette_body(CODE_CSS, ".c_theme_dark");
    let shared: Vec<&str> = LIGHT_PALETTE
        .iter()
        .map(|(name, _): &(&str, &str)| *name)
        .filter(|name: &&str| {
            let declaration: String = format!("--docs-code-{name}:");
            let light_value: Option<&str> = light.split(&declaration).nth(1);
            let dark_value: Option<&str> = dark.split(&declaration).nth(1);
            light_value == dark_value
        })
        .collect();
    assert!(
        shared.is_empty(),
        "these palette entries are the same in both modes — a shared colour is a colour \
         that reads on one surface and disappears on the other: {shared:?}"
    );
}

#[test]
fn every_token_class_reads_its_palette_entry() {
    for (class, name) in TOKEN_CLASSES {
        let body: String = rule_body(CODE_CSS, &format!(".{class}"));
        assert_eq!(
            declaration(&body, "color"),
            format!("var(--docs-code-{name})"),
            "the class {class} does not read its palette entry"
        );
    }
}

#[test]
fn the_keyword_and_function_tokens_are_bold() {
    for class in ["c_docs_code_kw", "c_docs_code_fn"] {
        let body: String = rule_body(CODE_CSS, &format!(".{class}"));
        assert_eq!(
            declaration(&body, "font-weight"),
            "bold",
            "the class {class} is not bold — the editor paints its keywords and \
             functions in a bold face"
        );
    }
}

#[test]
fn only_the_comment_token_is_italic() {
    for (class, _) in TOKEN_CLASSES {
        let body: String = rule_body(CODE_CSS, &format!(".{class}"));
        let italic: bool = body.contains("font-style: italic");
        assert_eq!(
            italic,
            class == "c_docs_code_com",
            "the class {class} carries the wrong italic state — only a comment is italic"
        );
    }
}

#[test]
fn the_code_surface_paints_the_editor_background_and_foreground_of_its_mode() {
    let frame: &Css = c_docs_code();
    assert!(
        frame
            .get_style()
            .contains("background: var(--docs-code-surface);"),
        "the code block does not paint the editor surface of its mode"
    );
    let pre: &Css = c_docs_code_pre();
    let rules: String = format!("{:?}", pre.get_pseudo_rules());
    assert!(
        rules.contains("var(--docs-code-foreground)"),
        "the code text does not paint the editor foreground of its mode"
    );
}

#[test]
fn the_code_text_uses_the_editors_own_font_stack() {
    let light: String = palette_body(CODE_CSS, ":root");
    assert!(
        light.contains(&format!("--docs-code-font-family: {EDITOR_FONT_FAMILY};")),
        "the code block does not read the editor's font stack"
    );
    let pre: &Css = c_docs_code_pre();
    let rules: String = format!("{:?}", pre.get_pseudo_rules());
    assert!(
        rules.contains("var(--docs-code-font-family)"),
        "the code text does not read the editor's font stack"
    );
}

#[test]
fn the_palette_declares_no_colour_outside_the_editor_theme() {
    let light: String = palette_body(CODE_CSS, ":root");
    let dark: String = palette_body(CODE_CSS, ".c_theme_dark");
    let allowed: Vec<String> = LIGHT_PALETTE
        .iter()
        .chain(DARK_PALETTE.iter())
        .map(|(_, colour): &(&str, &str)| colour.to_uppercase())
        .collect();
    for block in [light, dark] {
        for part in block.split(';') {
            let Some((name, value)) = part.split_once(':') else {
                continue;
            };
            if !name.trim().starts_with("--docs-code-") {
                continue;
            }
            if name.trim() == "--docs-code-font-family" {
                continue;
            }
            let colour: String = value.trim().to_uppercase();
            assert!(
                allowed.contains(&colour),
                "the palette declares {colour}, which is not a colour of the editor theme"
            );
        }
    }
}
