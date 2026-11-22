use super::*;

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

fn nav_column_style() -> (String, String) {
    let column: &'static Css = c_nav_items_scroll();
    (
        column.get_name().to_string(),
        column.get_style().to_string(),
    )
}

#[test]
fn the_nav_column_reserves_its_scrollbar_gutter_on_the_right_only() {
    let (name, style) = nav_column_style();
    assert_eq!(
        declaration(&style, "scrollbar-gutter"),
        Some("stable".to_string()),
        "{name} must reserve the gutter on the right only"
    );
    assert!(
        !style.contains("both-edges"),
        "{name} must not reserve a gutter on the left edge as well: it inflates \
         the sidebar's left inset by a full scrollbar width"
    );
}

#[test]
fn the_nav_column_still_scrolls_vertically() {
    let (name, style) = nav_column_style();
    assert_eq!(
        declaration(&style, "overflow-y"),
        Some("auto".to_string()),
        "{name} must keep its vertical overflow"
    );
}

#[test]
fn the_nav_column_keeps_only_its_own_small_left_padding() {
    let (name, style) = nav_column_style();
    let padding: String =
        declaration(&style, "padding").unwrap_or_else(|| panic!("{name} declares its padding"));
    assert!(
        padding.contains("var(--nav-item-inset)"),
        "{name} pads with the spacing tokens, got {padding}"
    );
    assert!(
        padding.contains("var(--space-sm)"),
        "{name} pads with the spacing tokens, got {padding}"
    );
}
