//! Tests for the `Tag::Portal(String)` variant and the macro's
//! `portal { target: "..." }` lowering.
//!
//! These tests cover the pure-Rust surface that does not require a
//! live browser to resolve `query_selector`:
//!
//! - `Tag::Portal` round-trips through `Clone`, `PartialEq`, `Hash`,
//!   and `Debug` — the four derives the `Tag` enum ships with.
//! - `Tag::Portal("body") != Tag::Portal("#x")` — the inner selector
//!   contributes to equality, so a portal that flips target is
//!   recognised as a different tag by the renderer's patch logic.
//! - `try_get_tag_name` returns `None` for portal nodes so callers
//!   that ask "what tag is this?" never see the internal marker
//!   name.

use super::*;

/// `Tag` derives `Clone`, which the renderer's patch loop relies on
/// when comparing old/new tag pointers.
#[test]
fn portal_tag_is_clone() {
    let tag: Tag = Tag::Portal(String::from("#modal-root"));
    let cloned: Tag = tag.clone();
    assert_eq!(tag, cloned);
}

/// `Tag` derives `PartialEq`. Portals with different selectors must
/// compare unequal so the renderer's `if old_tag != new_tag` branch
/// triggers a remount when the user changes `target`.
#[test]
fn portal_tag_partial_eq_distinguishes_selectors() {
    assert_eq!(
        Tag::Portal(String::from("#a")),
        Tag::Portal(String::from("#a")),
    );
    assert_ne!(
        Tag::Portal(String::from("#a")),
        Tag::Portal(String::from("#b")),
    );
    assert_ne!(
        Tag::Portal(String::from("body")),
        Tag::Element(String::from("body")),
        "Portal(\"body\") must not equal Element(\"body\")",
    );
}

/// `Tag` derives `Hash`. The renderer's keyed patch path builds a
/// `HashMap` keyed by tag, so any hash-trait breakage here would
/// silently disable the keyed diff.
#[test]
fn portal_tag_is_hashable() {
    use std::collections::HashSet;
    let mut set: HashSet<Tag> = HashSet::new();
    set.insert(Tag::Portal(String::from("#a")));
    set.insert(Tag::Portal(String::from("#a")));
    set.insert(Tag::Portal(String::from("#b")));
    assert_eq!(set.len(), 2, "selectors collapse by equality, not identity");
}

/// `Tag` derives `Debug`. The dev-tools inspector prints tag names
/// for element nodes; portals need a stable string representation.
#[test]
fn portal_tag_debug_names_variant() {
    let formatted: String = format!("{:?}", Tag::Portal(String::from("#x")));
    assert!(
        formatted.contains("Portal"),
        "Debug output must name the variant, got: {formatted}",
    );
    assert!(
        formatted.contains("#x"),
        "Debug output must include the selector for traceability, got: {formatted}",
    );
}

/// `try_get_tag_name` returns `None` for portal nodes. The
/// renderer treats the portal marker as an internal detail — the
/// public-facing tag name is meaningless because the real content
/// lives in a separate DOM subtree.
#[test]
fn virtual_node_try_get_tag_name_returns_none_for_portal() {
    let node: VirtualNode = VirtualNode::Element {
        tag: Tag::Portal(String::from("#toast-host")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    assert_eq!(
        node.try_get_tag_name(),
        None,
        "Portal must not expose a tag name through the public API",
    );
}

/// `try_get_tag_name` for non-portal Element nodes still returns
/// the inner name string. The Portal exception must not regress
/// existing behaviour.
#[test]
fn virtual_node_try_get_tag_name_still_works_for_element() {
    let node: VirtualNode = VirtualNode::Element {
        tag: Tag::Element(String::from("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    assert_eq!(
        node.try_get_tag_name(),
        Some(String::from("div")),
        "Element tag name must still resolve",
    );
}

/// `try_get_tag_name` for Component nodes returns the component
/// name (the renderer needs it to look up the `#[component]`
/// function). The Portal exception must not regress this either.
#[test]
fn virtual_node_try_get_tag_name_still_works_for_component() {
    let node: VirtualNode = VirtualNode::Element {
        tag: Tag::Component(String::from("euv_button")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    assert_eq!(
        node.try_get_tag_name(),
        Some(String::from("euv_button")),
        "Component tag name must still resolve",
    );
}

/// Portal nodes with the same selector are `Eq` — this is what
/// `if old_tag != new_tag` relies on in the patch loop. Without
/// this, every render would unnecessarily remount the portal
/// subtree.
#[test]
fn portal_tag_partial_eq_same_selector_equals() {
    let a: Tag = Tag::Portal(String::from("#modal-root"));
    let b: Tag = Tag::Portal(String::from("#modal-root"));
    assert!(a == b, "same-selector portals must compare equal");
}

/// Portal nodes with empty selector still work — useful as a
/// sentinel during component unwrapping (see `unwrap_component`'s
/// pass-through arm).
#[test]
fn portal_tag_supports_empty_selector() {
    let tag: Tag = Tag::Portal(String::new());
    assert_eq!(tag.clone(), tag, "empty selector must round-trip");
}
