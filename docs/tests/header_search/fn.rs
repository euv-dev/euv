use super::*;

fn page(title: &'static str, route: &'static str) -> DocsPage {
    DocsPage {
        title,
        route,
        ..crate::generated::SITE.pages[0]
    }
}

fn fixture() -> Vec<DocsPage> {
    vec![
        page("Other", "/search-route"),
        page("Search first", "/first"),
        page("SEARCH second", "/second"),
        page("Duplicate", "/first"),
    ]
}

fn attr(node: &VirtualNode, name: &str) -> Option<String> {
    match node {
        VirtualNode::Element { attributes, .. } => attributes
            .iter()
            .find(|entry: &&AttributeEntry| entry.get_name() == name)
            .and_then(AttributeEntry::text_value),
        _ => None,
    }
}

fn find_role<'a>(node: &'a VirtualNode, role: &str) -> Option<&'a VirtualNode> {
    if attr(node, "role").as_deref() == Some(role) {
        return Some(node);
    }
    node.get_children()
        .iter()
        .find_map(|child: &VirtualNode| find_role(child, role))
}

fn find_id<'a>(node: &'a VirtualNode, id: &str) -> Option<&'a VirtualNode> {
    if attr(node, "id").as_deref() == Some(id) {
        return Some(node);
    }
    node.get_children()
        .iter()
        .find_map(|child: &VirtualNode| find_id(child, id))
}

fn render(query: &str, snapshot: HeaderSearchState, id: &str) -> VirtualNode {
    header_search_preview(
        Signal::create("/unchanged".to_string()),
        Signal::create(query.to_string()),
        Signal::create(snapshot),
        id.to_string(),
        NodeRef::new(),
    )
}

#[test]
fn title_matches_precede_route_matches_in_stable_order() {
    let results: Vec<DocsPage> = header_search_matches(&fixture(), "  sEaRcH  ");
    let routes: Vec<&str> = results.iter().map(|page: &DocsPage| page.route).collect();
    assert_eq!(routes, vec!["/first", "/second", "/search-route"]);
}

#[test]
fn unicode_case_and_whitespace_are_normalized() {
    let pages: Vec<DocsPage> = vec![page("ÉCOLE 文档", "/guide")];
    assert_eq!(header_search_matches(&pages, " éCoLe ").len(), 1);
    assert_eq!(header_search_matches(&pages, "文档").len(), 1);
    assert!(header_search_matches(&pages, "  \t  ").is_empty());
}

#[test]
fn duplicate_routes_are_removed_and_results_are_bounded() {
    let base: DocsPage = page("Search", "/duplicate");
    assert_eq!(header_search_matches(&[base; 20], "search").len(), 1);
    let pages: Vec<DocsPage> = ["/a", "/b", "/c", "/d", "/e", "/f", "/g", "/h", "/i"]
        .iter()
        .map(|route: &&'static str| page("Search", route))
        .collect();
    let results: Vec<DocsPage> = header_search_matches(&pages, "search");
    assert_eq!(results.len(), HEADER_SEARCH_LIMIT);
    assert_eq!(results.last().map(|page: &DocsPage| page.route), Some("/h"));
}

#[test]
fn private_body_metadata_is_not_searched() {
    let private: DocsPage = DocsPage {
        private: true,
        hero_text: "confidential needle",
        tagline: "confidential needle",
        password_hash: "confidential needle",
        ..page("Public title", "/public-route")
    };
    assert!(header_search_matches(&[private], "confidential").is_empty());
    assert_eq!(header_search_matches(&[private], "public").len(), 1);
}

#[test]
fn focus_typing_and_unselected_enter_do_not_choose_first_match() {
    let results: Vec<DocsPage> = header_search_matches(&fixture(), "search");
    let state: Signal<HeaderSearchState> = Signal::create(HeaderSearchState::default());
    header_search_focus(state);
    assert!(state.get().get_open());
    assert_eq!(state.get().get_active(), None);
    let mut snapshot: HeaderSearchState = state.get();
    assert_eq!(snapshot.key(KEY_ENTER, false, &results), (false, None));
    assert_eq!(snapshot.key("a", false, &results), (false, None));
    assert_eq!(
        snapshot.key(HEADER_SEARCH_DOWN, false, &results),
        (true, None)
    );
    state.set(snapshot);
    header_search_focus(state);
    assert_eq!(state.get().get_active(), None);
    assert!(state.get().get_open());
}

#[test]
fn arrows_home_end_choose_only_a_preview_and_enter_emits_explicit_route() {
    let results: Vec<DocsPage> = header_search_matches(&fixture(), "search");
    let mut state: HeaderSearchState = HeaderSearchState::default();
    state.focus();
    assert_eq!(state.key(HEADER_SEARCH_DOWN, false, &results), (true, None));
    assert_eq!(state.get_active(), Some(0));
    assert_eq!(state.key(HEADER_SEARCH_UP, false, &results), (true, None));
    assert_eq!(state.get_active(), Some(results.len() - 1));
    assert_eq!(state.key(HEADER_SEARCH_HOME, false, &results), (true, None));
    assert_eq!(state.get_active(), Some(0));
    assert_eq!(state.key(HEADER_SEARCH_END, false, &results), (true, None));
    assert_eq!(state.get_active(), Some(results.len() - 1));
    assert_eq!(
        state.key(KEY_ENTER, false, &results),
        (true, Some("/search-route"))
    );
    assert!(!state.get_open());
    assert_eq!(state.get_active(), None);
    assert_eq!(state.key(KEY_ENTER, false, &results), (false, None));
}

#[test]
fn composition_preserves_preview_without_navigating_or_consuming_keys() {
    let results: Vec<DocsPage> = header_search_matches(&fixture(), "search");
    let mut state: HeaderSearchState = HeaderSearchState::default();
    state.focus();
    assert_eq!(state.key(HEADER_SEARCH_DOWN, false, &results), (true, None));
    let before: HeaderSearchState = state.clone();
    for key in [KEY_ENTER, HEADER_SEARCH_DOWN, HEADER_SEARCH_ESCAPE] {
        assert_eq!(state.key(key, true, &results), (false, None));
        assert_eq!(state, before);
    }
}

#[test]
fn no_matches_and_dismissal_never_emit_a_route() {
    let mut state: HeaderSearchState = HeaderSearchState::default();
    state.focus();
    for key in [
        HEADER_SEARCH_DOWN,
        HEADER_SEARCH_UP,
        HEADER_SEARCH_HOME,
        HEADER_SEARCH_END,
        KEY_ENTER,
    ] {
        assert_eq!(state.key(key, false, &[]), (false, None));
    }
    assert_eq!(state.key(HEADER_SEARCH_ESCAPE, false, &[]), (true, None));
    assert_eq!(state.get_active(), None);
    assert!(!state.get_open());
    assert_eq!(state.key(HEADER_SEARCH_ESCAPE, false, &[]), (false, None));
    state.focus();
    assert_eq!(state.key(HEADER_SEARCH_TAB, false, &[]), (false, None));
    assert!(!state.get_open());
    state.focus();
    state.dismiss();
    assert!(!state.get_open());
}

#[test]
fn preview_has_real_combobox_listbox_options_with_stable_aria_links() {
    let query: &str = crate::generated::SITE.pages[0].title;
    let results: Vec<DocsPage> = header_search_matches(crate::generated::SITE.pages, query);
    assert!(!results.is_empty());
    let mut state: HeaderSearchState = HeaderSearchState::default();
    state.focus();
    assert_eq!(state.key(HEADER_SEARCH_DOWN, false, &results), (true, None));
    let node: VirtualNode = render(query, state.clone(), "search-one");
    let input: &VirtualNode = find_role(&node, "combobox").expect("native combobox");
    let list: &VirtualNode = find_role(&node, "listbox").expect("real listbox");
    assert_eq!(input.try_get_tag_name().as_deref(), Some("input"));
    assert_eq!(attr(input, "aria-expanded").as_deref(), Some("true"));
    assert_eq!(attr(input, "aria-controls"), attr(list, "id"));
    let active_id: String = attr(input, "aria-activedescendant").expect("active result id");
    let active: &VirtualNode = find_id(&node, &active_id).expect("existing option");
    assert_eq!(attr(active, "role").as_deref(), Some("option"));
    assert_eq!(attr(active, "aria-selected").as_deref(), Some("true"));
    assert_eq!(active.get_children().len(), 2);
    assert!(format!("{active:?}").contains(results[0].title));
    assert!(format!("{active:?}").contains(results[0].route));
    let second: VirtualNode = render(query, state, "search-two");
    assert!(find_id(&second, &active_id).is_none());
}

#[test]
fn option_click_is_explicit_and_mouse_down_preserves_input_focus() {
    let query: &str = crate::generated::SITE.pages[0].title;
    let mut state: HeaderSearchState = HeaderSearchState::default();
    state.focus();
    let node: VirtualNode = render(query, state, "search-click");
    let option: &VirtualNode = find_role(&node, "option").expect("preview row");
    let VirtualNode::Element { attributes, .. } = option else {
        panic!("native option");
    };
    for event in ["onclick", "onmousedown"] {
        assert!(
            attributes
                .iter()
                .any(|entry: &AttributeEntry| entry.get_name() == event)
        );
    }
    let input: &VirtualNode = find_role(&node, "combobox").expect("input");
    assert_eq!(attr(input, "aria-activedescendant").as_deref(), Some(""));
}

#[test]
fn empty_and_missing_queries_announce_state_and_never_change_route() {
    for (query, message) in [
        ("   ", HEADER_SEARCH_EMPTY),
        ("no-such-page-928364", HEADER_SEARCH_NONE),
    ] {
        let route: Signal<String> = Signal::create("/unchanged".to_string());
        let mut snapshot: HeaderSearchState = HeaderSearchState::default();
        snapshot.focus();
        let node: VirtualNode = header_search_preview(
            route,
            Signal::create(query.to_string()),
            Signal::create(snapshot),
            "empty-search".to_string(),
            NodeRef::new(),
        );
        assert_eq!(route.get(), "/unchanged");
        assert!(find_role(&node, "option").is_none());
        let status: &VirtualNode = find_role(&node, "status").expect("live status");
        assert_eq!(attr(status, "aria-live").as_deref(), Some("polite"));
        assert!(format!("{status:?}").contains(message));
    }
}

#[test]
fn closed_panel_clears_aria_active_state_and_uses_shared_tokens() {
    let node: VirtualNode = render("guide", HeaderSearchState::default(), "closed-search");
    let input: &VirtualNode = find_role(&node, "combobox").expect("input");
    assert_eq!(attr(input, "aria-expanded").as_deref(), Some("false"));
    assert_eq!(attr(input, "aria-activedescendant").as_deref(), Some(""));
    let css: String = c_docs_header_search_panel().get_style().to_string();
    assert!(css.contains("position: absolute"));
    assert!(css.contains("var(--popover)"));
    assert!(css.contains("var(--border)"));
    assert!(
        c_docs_header_search_panel_closed()
            .get_style()
            .to_string()
            .contains("display: none")
    );
}

#[test]
fn committed_search_selection_clears_the_query_for_click_and_enter() {
    let source: &str = include_str!("../../src/component/header/search/fn.rs");
    let selection: &str = source
        .split_once("pub(crate) fn header_search_select(")
        .unwrap()
        .1;
    let cleared: usize = selection
        .find("query.set(String::new());")
        .expect("clear query");
    let navigated: usize = selection
        .find("Router::navigate(route);")
        .expect("commit route");
    assert!(cleared < navigated);
    let preview: &str = include_str!("../../src/component/header/preview/fn.rs");
    assert_eq!(
        preview
            .matches("header_search_select(route_signal, query, state, route);")
            .count(),
        2
    );
}
