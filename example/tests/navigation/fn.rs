use super::*;

#[test]
fn navigation_excludes_standalone_component_gallery_routes() {
    assert!(
        NAV_ITEMS
            .iter()
            .all(|(_, _, route): &(&str, &str, &str)| !route.starts_with("/ui-"))
    );
}

#[test]
fn page_registry_and_router_exclude_standalone_component_galleries() {
    let pages: &str = include_str!("../../src/page/mod.rs");
    let router: &str = include_str!("../../src/component/router/view/fn.rs");
    let routes: &str = include_str!("../../src/component/router/view/const.rs");
    assert!(
        pages
            .lines()
            .all(|line: &str| !line.trim_start().starts_with("mod ui_"))
    );
    assert!(!router.contains("ROUTE_UI_"));
    assert!(!routes.contains("ROUTE_UI_"));
    assert!(router.contains("page_not_found"));
}
