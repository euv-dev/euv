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

static GUIDE_CHILDREN: &[EuvSidebarItem] = &[
    EuvSidebarItem {
        text: "install",
        link: Some("/guide/install.html"),
        children: &[],
    },
    EuvSidebarItem {
        text: "usage",
        link: Some("/guide/usage.html"),
        children: &[],
    },
];

static OTHER_CHILDREN: &[EuvSidebarItem] = &[EuvSidebarItem {
    text: "config",
    link: Some("/other/config.html"),
    children: &[],
}];

static PAGER_TREE: &[EuvSidebarItem] = &[
    EuvSidebarItem {
        text: "Guide",
        link: Some("/guide/"),
        children: GUIDE_CHILDREN,
    },
    EuvSidebarItem {
        text: "Other",
        link: Some("/other/"),
        children: OTHER_CHILDREN,
    },
];

#[test]
fn the_pager_pool_spans_directory_boundaries() {
    assert_eq!(
        flatten_links(PAGER_TREE)
            .iter()
            .map(|item: &&EuvSidebarItem| item.link)
            .collect::<Vec<Option<&'static str>>>(),
        vec![
            Some("/guide/"),
            Some("/guide/install.html"),
            Some("/guide/usage.html"),
            Some("/other/"),
            Some("/other/config.html"),
        ],
        "the pool is one ordered list for the whole locale, so the reader \
         can walk from the last page of one directory into the next"
    );
}

#[test]
fn a_directorys_first_page_is_not_the_end_of_the_document() {
    let pool: Vec<&'static EuvSidebarItem> = flatten_links(PAGER_TREE);
    let index: usize = pool
        .iter()
        .position(|item: &&EuvSidebarItem| item.link == Some("/other/"))
        .expect("the second directory index is in the pool");
    assert!(
        index > 0,
        "the second directory index is preceded by the first directory's \
         pages, so it must have a previous entry"
    );
    assert_eq!(
        pool[index - 1].link,
        Some("/guide/usage.html"),
        "the previous button points at the last file of the previous \
         directory rather than being missing"
    );
}

#[test]
fn a_directorys_last_page_has_a_next_entry_in_the_following_directory() {
    let pool: Vec<&'static EuvSidebarItem> = flatten_links(PAGER_TREE);
    let index: usize = pool
        .iter()
        .position(|item: &&EuvSidebarItem| item.link == Some("/guide/usage.html"))
        .expect("the first directory's last page is in the pool");
    assert_eq!(
        pool.get(index + 1)
            .and_then(|item: &&'static EuvSidebarItem| item.link),
        Some("/other/"),
        "the next button crosses into the following directory instead of \
         being missing"
    );
}

#[test]
fn only_the_very_first_and_very_last_page_have_no_neighbour() {
    let pool: Vec<&'static EuvSidebarItem> = flatten_links(PAGER_TREE);
    for (index, item) in pool.iter().enumerate() {
        let has_prev: bool = index > 0;
        let has_next: bool = index + 1 < pool.len();
        assert!(
            has_prev || has_next,
            "only the document's endpoints lack a neighbour, but {:?} is \
             missing both",
            item.link
        );
    }
}
