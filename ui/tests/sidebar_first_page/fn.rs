use super::*;

static LEAF_A: &[EuvSidebarItem] = &[EuvSidebarItem {
    text: "A",
    link: Some("/a.html"),
    children: &[],
}];
static DIR_ONE: &[EuvSidebarItem] = &[EuvSidebarItem {
    text: "one",
    link: None,
    children: LEAF_A,
}];

static DIR_DEEP: &[EuvSidebarItem] = &[EuvSidebarItem {
    text: "deep",
    link: None,
    children: DIR_ONE,
}];

static DIR_EMPTY: &[EuvSidebarItem] = &[EuvSidebarItem {
    text: "empty",
    link: None,
    children: &[],
}];

static GROUP_INDEX: &[EuvSidebarItem] = &[EuvSidebarItem {
    text: "grp",
    link: Some("/grp.html"),
    children: LEAF_A,
}];

#[test]
fn a_pure_directory_resolves_to_the_first_readable_page_below_it() {
    assert_eq!(
        first_navigable_route(DIR_ONE),
        Some("/a.html"),
        "a directory is not a destination, so the click has to land on the first page that actually has content"
    );
}

#[test]
fn an_empty_directory_resolves_to_nothing() {
    assert_eq!(
        first_navigable_route(DIR_EMPTY),
        None,
        "with nothing readable below it there is no honest destination, and the caller must fall back to folding"
    );
}

#[test]
fn nesting_is_followed_until_a_readable_page_is_found() {
    assert_eq!(
        first_navigable_route(DIR_DEEP),
        Some("/a.html"),
        "the rule repeats at every level, so a chain of empty directories still resolves to the page at the bottom"
    );
}

#[test]
fn a_group_index_is_preferred_over_its_children() {
    assert_eq!(
        first_navigable_route(GROUP_INDEX),
        Some("/grp.html"),
        "the index is the directory's own readable face and sits above its children in reading order"
    );
}

#[test]
fn sidebar_order_decides_which_page_wins() {
    let leaves: &[EuvSidebarItem] = &[
        EuvSidebarItem {
            text: "empty",
            link: None,
            children: DIR_EMPTY,
        },
        EuvSidebarItem {
            text: "real",
            link: Some("/real.html"),
            children: &[],
        },
    ];
    assert_eq!(
        first_navigable_route(leaves),
        Some("/real.html"),
        "an unreadable sibling must not stop the search, and the first readable entry in sidebar order is the destination"
    );
}

#[test]
fn an_empty_subtree_resolves_to_nothing() {
    assert_eq!(
        first_navigable_route(&[]),
        None,
        "a sidebar with no items at all has no first page"
    );
}
