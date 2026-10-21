use super::*;

fn active_row_styles() -> Vec<(String, String)> {
    let group: &'static Css = c_euv_sidebar_group_title_active();
    let leaf: &'static Css = c_euv_sidebar_link_active_flush();
    vec![
        (group.get_name().to_string(), group.get_style().to_string()),
        (leaf.get_name().to_string(), leaf.get_style().to_string()),
    ]
}

fn declaration(body: &str, property: &str) -> Option<String> {
    for part in body.split(';') {
        let Some((name, value)) = part.split_once(':') else {
            continue;
        };
        if name.trim() == property {
            return Some(value.trim().to_string());
        }
    }
    None
}

#[test]
fn an_active_sidebar_row_pulls_its_fill_back_to_the_dashed_rail() {
    for (name, style) in active_row_styles() {
        assert_eq!(
            declaration(&style, "margin-left"),
            Some("calc(-1 * var(--side-indent))".to_string()),
            "{name} must pull its fill back to the dashed rail, or the accent \
             block floats inside the gutter instead of reaching it"
        );
    }
}

#[test]
fn no_emitted_sidebar_css_glues_a_unit_onto_a_function() {
    for (name, style) in active_row_styles() {
        assert!(
            !style.contains(")px"),
            "{name} glues a unit onto the end of a function: {style} — the \
             substitution leaves a number token next to an ident, the parser \
             drops the whole declaration, and the property silently keeps its \
             initial value"
        );
    }
}
