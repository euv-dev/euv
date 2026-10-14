use super::*;

static LEAF: &[EuvSidebarItem] = &[EuvSidebarItem {
    text: "page",
    link: Some("/dir/page.html"),
    children: &[],
}];

static NESTED: &[EuvSidebarItem] = &[
    EuvSidebarItem {
        text: "outer",
        link: None,
        children: LEAF,
    },
    EuvSidebarItem {
        text: "inner",
        link: None,
        children: LEAF,
    },
];

static LEAF_SIBLING: &[EuvSidebarItem] = &[EuvSidebarItem {
    text: "loose",
    link: Some("/loose.html"),
    children: &[],
}];

static INDEX_GROUP: &[EuvSidebarItem] = &[EuvSidebarItem {
    text: "section",
    link: Some("/section/"),
    children: LEAF,
}];

static TREE: &[EuvSidebarItem] = &[
    EuvSidebarItem {
        text: "top",
        link: None,
        children: NESTED,
    },
    EuvSidebarItem {
        text: "bare",
        link: Some("/bare.html"),
        children: &[],
    },
    EuvSidebarItem {
        text: "withindex",
        link: Some("/withindex/"),
        children: LEAF,
    },
];

#[test]
fn a_top_level_group_is_keyed_under_a_single_separator() {
    assert_eq!(
        sidebar_group_key("", "guide"),
        String::from("/guide"),
        "the top level has no parent name, so the key is the separator \
         followed by the text"
    );
}

#[test]
fn a_nested_group_is_keyed_under_its_parent() {
    assert_eq!(
        sidebar_group_key("/guide", "install"),
        String::from("/guide/install"),
        "a nested key is its parent's key with the child text appended"
    );
}

#[test]
fn seeding_collapses_every_group_at_every_depth() {
    assert_eq!(
        all_group_keys(TREE),
        vec![
            String::from("/top"),
            String::from("/top/outer"),
            String::from("/top/inner"),
            String::from("/withindex"),
        ],
        "the default state has to fold the whole tree, or the sections the \
         reader is not in stay open and the sidebar overflows its column"
    );
}

#[test]
fn a_leaf_is_never_a_collapsible_key() {
    assert!(
        !all_group_keys(TREE).contains(&String::from("/bare")),
        "a childless entry has nothing to expand, so listing it would put a \
         dead key in the signal that no click can ever clear"
    );
}

#[test]
fn a_group_that_owns_an_index_page_still_collapses() {
    assert_eq!(
        all_group_keys(INDEX_GROUP),
        vec![String::from("/section")],
        "an index page does not make a group a leaf — it navigates, but its \
         children still fold"
    );
}

#[test]
fn a_tree_of_only_leaves_collapses_to_nothing() {
    assert!(
        all_group_keys(LEAF_SIBLING).is_empty(),
        "a sidebar with no groups has nothing to fold, and an empty default \
         is what keeps the leaves visible"
    );
}
