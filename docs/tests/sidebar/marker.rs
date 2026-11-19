use super::*;

fn strip_css_comments(css: &str) -> String {
    let mut kept: String = String::with_capacity(css.len());
    let mut rest: &str = css;
    while let Some(start) = rest.find("/*") {
        kept.push_str(&rest[..start]);
        let after: &str = &rest[start + 2..];
        match after.find("*/") {
            Some(end) => rest = &after[end + 2..],
            None => {
                rest = "";
                break;
            }
        }
    }
    kept.push_str(rest);
    kept
}

fn app_css() -> String {
    strip_css_comments(APP_SIDEBAR_CSS)
}

fn css_rules() -> Vec<(String, String)> {
    let mut rules: Vec<(String, String)> = Vec::new();
    for rule in app_css().split('}') {
        let Some((selector, body)) = rule.split_once('{') else {
            continue;
        };
        let selector: &str = selector.trim();
        if !selector.starts_with('.') && !selector.starts_with("#app .") {
            continue;
        }
        rules.push((selector.to_string(), body.to_string()));
    }
    rules
}

fn is_active_row_marker(selector: &str) -> bool {
    selector.contains("c_euv_sidebar_link_active_flush::before")
        || selector.contains("c_euv_sidebar_group_title_active::before")
}

#[test]
fn the_active_group_title_marker_is_not_dragged_off_its_own_fill() {
    let mut offenders: Vec<String> = Vec::new();
    for (selector, body) in css_rules() {
        if !selector.contains("c_euv_sidebar_group_title_active::before") {
            continue;
        }
        if body.contains("left: -") {
            offenders.push(format!("{selector} {{ {body} }}"));
        }
    }
    assert!(
        offenders.is_empty(),
        "the active group title is pulled back to the dashed guide, so its marker \
         already sits on its own accent fill; a negative offset drags the 5px bar \
         off the fill and paints a bold bar in the gutter beside the rail, which \
         the active leaf does not show:\n{}",
        offenders.join("\n")
    );
}

#[test]
fn the_active_rows_keep_their_marker_at_the_fill_edge() {
    if cfg!(feature = "theme-shadcn") {
        assert!(APP_SIDEBAR_CSS.is_empty());
        assert!(!APP_GLOBAL_CSS.contains("c_euv_sidebar_"));
        assert!(css_rules().is_empty());
        return;
    }
    let mut negative: Vec<String> = Vec::new();
    let mut pinned: Vec<String> = Vec::new();
    for (selector, body) in css_rules() {
        if !is_active_row_marker(&selector) {
            continue;
        }
        if body.contains("left: -") {
            negative.push(format!("{selector} {{ {body} }}"));
        }
        if body.contains("left: 0") {
            pinned.push(selector);
        }
    }
    assert!(
        negative.is_empty(),
        "an active row's marker belongs on its own accent fill, so no rule may \
         offset it to the left of that fill:\n{}",
        negative.join("\n")
    );
    assert!(
        pinned
            .iter()
            .any(|selector: &String| selector.contains("c_euv_sidebar_group_title_active::before")),
        "the active group title's marker must be pinned to its fill with \
         `left: 0`, the way the active leaf's is, or the bar floats in the \
         gutter beside the dashed rail: pinned by {pinned:?}"
    );
}
