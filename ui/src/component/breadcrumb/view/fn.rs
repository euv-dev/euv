use super::*;

/// A generic breadcrumb trail aligned with common site frameworks.
///
/// Renders a `<nav>` holding one node per entry: ancestors become links
/// (`http`-prefixed targets open in a new tab, everything else goes through
/// the hash router), and the last entry renders as the current page span
/// instead of a link. A separator span is rendered between two entries.
/// An empty `items` list renders nothing.
///
/// The trail is the surface the reference publishes: the list, the item,
/// the link, the page, the separator and the ellipsis are its parts, so
/// each of them is a component of its own.
///
/// # Arguments
///
/// - `VirtualNode<EuvBreadcrumbProps>` - The props node containing items and
///   separator.
///
/// # Returns
///
/// - `VirtualNode` - The breadcrumb virtual DOM tree.
#[component]
pub fn euv_breadcrumb(node: VirtualNode<EuvBreadcrumbProps>) -> VirtualNode {
    let EuvBreadcrumbProps { items, separator }: EuvBreadcrumbProps =
        node.try_get_props().unwrap_or_default();
    if items.is_empty() {
        return html! {
            ""
        };
    }
    let last_index: usize = items.len() - 1;
    let mut crumbs: Vec<VirtualNode> = Vec::with_capacity(items.len() * 2);
    for (index, item) in items.iter().enumerate() {
        if index > 0 {
            crumbs.push(breadcrumb_separator_node(separator));
        }
        let crumb: VirtualNode = if index == last_index {
            breadcrumb_page_node(item.label)
        } else {
            breadcrumb_link_node_short(item.label, item.href)
        };
        crumbs.push(breadcrumb_item_node(crumb));
    }
    html! {
        nav {
            class: c_euv_breadcrumb()
            "data-slot": "breadcrumb"
            euv_breadcrumb_list {
                crumbs
            }
        }
    }
}

/// The row of crumbs of a trail.
///
/// The list holds the crumbs in order, so it takes no settings of its
/// own.
///
/// # Arguments
///
/// - `VirtualNode<EuvBreadcrumbListProps>` - The props node carrying the
///   component settings.
///
/// # Returns
///
/// - `VirtualNode` - The breadcrumb list virtual DOM tree.
#[component]
pub fn euv_breadcrumb_list(node: VirtualNode<EuvBreadcrumbListProps>) -> VirtualNode {
    let EuvBreadcrumbListProps {}: EuvBreadcrumbListProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        ol {
            class: c_euv_breadcrumb_list()
            "data-slot": "breadcrumb-list"
            children
        }
    }
}

/// One crumb of a trail.
///
/// The item holds the crumb, so it takes no settings of its own.
///
/// # Arguments
///
/// - `VirtualNode<EuvBreadcrumbItemProps>` - The props node carrying the
///   component settings.
///
/// # Returns
///
/// - `VirtualNode` - The breadcrumb item virtual DOM tree.
#[component]
pub fn euv_breadcrumb_item(node: VirtualNode<EuvBreadcrumbItemProps>) -> VirtualNode {
    let EuvBreadcrumbItemProps {}: EuvBreadcrumbItemProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        li {
            class: c_euv_breadcrumb_item()
            "data-slot": "breadcrumb-item"
            children
        }
    }
}

/// An ancestor crumb, rendered as a link.
///
/// External (`http`-prefixed) targets open in a new tab; every other target
/// is treated as an internal hash route and prefixed with
/// `ROUTE_HASH_PREFIX` so the href matches what the router writes.
///
/// # Arguments
///
/// - `VirtualNode<EuvBreadcrumbLinkProps>` - The props node carrying the
///   component settings.
///
/// # Returns
///
/// - `VirtualNode` - The breadcrumb link virtual DOM tree.
#[component]
pub fn euv_breadcrumb_link(node: VirtualNode<EuvBreadcrumbLinkProps>) -> VirtualNode {
    let EuvBreadcrumbLinkProps { label, href }: EuvBreadcrumbLinkProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    breadcrumb_link_node(label, href, children)
}

/// The crumb that names the page the reader is on.
///
/// The page is not clickable: it is where the trail ends, so it is
/// written in the foreground colour rather than as a link.
///
/// # Arguments
///
/// - `VirtualNode<EuvBreadcrumbPageProps>` - The props node carrying the
///   component settings.
///
/// # Returns
///
/// - `VirtualNode` - The breadcrumb page virtual DOM tree.
#[component]
pub fn euv_breadcrumb_page(node: VirtualNode<EuvBreadcrumbPageProps>) -> VirtualNode {
    let EuvBreadcrumbPageProps { label }: EuvBreadcrumbPageProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    breadcrumb_page_node_with(label, children)
}

/// The glyph that divides two crumbs.
///
/// The separator is drawn in the muted colour and is not selectable, so
/// copying a trail copies the crumbs and not the marks between them.
///
/// # Arguments
///
/// - `VirtualNode<EuvBreadcrumbSeparatorProps>` - The props node carrying
///   the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The breadcrumb separator virtual DOM tree.
#[component]
pub fn euv_breadcrumb_separator(node: VirtualNode<EuvBreadcrumbSeparatorProps>) -> VirtualNode {
    let EuvBreadcrumbSeparatorProps { text }: EuvBreadcrumbSeparatorProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    breadcrumb_separator_node_with(text, children)
}

/// The mark that stands in for the crumbs a collapsed trail hides.
///
/// The ellipsis is a single dot, so the reader sees that the trail
/// continues without the trail taking the room it would need.
///
/// # Arguments
///
/// - `VirtualNode<EuvBreadcrumbEllipsisProps>` - The props node carrying
///   the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The breadcrumb ellipsis virtual DOM tree.
#[component]
pub fn euv_breadcrumb_ellipsis(node: VirtualNode<EuvBreadcrumbEllipsisProps>) -> VirtualNode {
    let EuvBreadcrumbEllipsisProps {}: EuvBreadcrumbEllipsisProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        span {
            class: c_euv_breadcrumb_ellipsis()
            role: "presentation"
            "aria-hidden": "true"
            "data-slot": "breadcrumb-ellipsis"
            children
        }
    }
}

/// Builds the list row that holds one crumb.
///
/// The reference wraps every crumb in a list row, so the row is the part
/// that holds the link or the current page.
///
/// # Arguments
///
/// - `VirtualNode` - The crumb the row holds.
///
/// # Returns
///
/// - `VirtualNode` - The breadcrumb item virtual DOM tree.
fn breadcrumb_item_node(crumb: VirtualNode) -> VirtualNode {
    html! {
        li {
            class: c_euv_breadcrumb_item()
            "data-slot": "breadcrumb-item"
            crumb
        }
    }
}

/// Builds the link of an ancestor crumb.
///
/// External (`http`-prefixed) targets open in a new tab; every other
/// target is treated as an internal hash route and prefixed with
/// `ROUTE_HASH_PREFIX` so the href matches what the router writes.
///
/// # Arguments
///
/// - `&'static str` - The text shown inside the link.
/// - `&'static str` - The link target.
/// - `VirtualNode` - The children the link carries, if any.
///
/// # Returns
///
/// - `VirtualNode` - The breadcrumb link virtual DOM tree.
fn breadcrumb_link_node(
    label: &'static str,
    href: &'static str,
    children: VirtualNode,
) -> VirtualNode {
    let content: VirtualNode = match children {
        VirtualNode::Empty => VirtualNode::Text(TextNode::new(Cow::Owned(label.to_string()), None)),
        other => other,
    };
    if href.starts_with(BREADCRUMB_LINK_SCHEME_HTTP_PREFIX) {
        return html! {
            a {
                key: href
                class: c_euv_breadcrumb_link()
                href: href
                target: "_blank"
                onclick: Router::external_link_handler(href)
                "data-slot": "breadcrumb-link"
                content
            }
        };
    }
    html! {
        a {
            key: href
            class: c_euv_breadcrumb_link()
            href: {
                let mut
                target: String =
                String::with_capacity(ROUTE_HASH_PREFIX.len() + href.len());
                target.push_str(ROUTE_HASH_PREFIX);
                target.push_str(href);
                target
            }
            onclick: Router::link_handler(href)
            "data-slot": "breadcrumb-link"
            content
        }
    }
}

/// Builds the link of an ancestor crumb from its item.
///
/// # Arguments
///
/// - `&'static str` - The text shown inside the link.
/// - `&'static str` - The link target.
///
/// # Returns
///
/// - `VirtualNode` - The breadcrumb link virtual DOM tree.
fn breadcrumb_link_node_short(label: &'static str, href: &'static str) -> VirtualNode {
    breadcrumb_link_node(label, href, VirtualNode::Empty)
}

/// Builds the crumb that names the page the reader is on.
///
/// # Arguments
///
/// - `&'static str` - The text shown inside the crumb.
///
/// # Returns
///
/// - `VirtualNode` - The breadcrumb page virtual DOM tree.
fn breadcrumb_page_node(label: &'static str) -> VirtualNode {
    breadcrumb_page_node_with(label, VirtualNode::Empty)
}

/// Builds the crumb that names the page the reader is on.
///
/// # Arguments
///
/// - `&'static str` - The text shown inside the crumb.
/// - `VirtualNode` - The children the crumb carries, if any.
///
/// # Returns
///
/// - `VirtualNode` - The breadcrumb page virtual DOM tree.
fn breadcrumb_page_node_with(label: &'static str, children: VirtualNode) -> VirtualNode {
    let content: VirtualNode = match children {
        VirtualNode::Empty => VirtualNode::Text(TextNode::new(Cow::Owned(label.to_string()), None)),
        other => other,
    };
    html! {
        span {
            class: c_euv_breadcrumb_current()
            "data-slot": "breadcrumb-page"
            content
        }
    }
}

/// Builds the glyph that divides two crumbs.
///
/// # Arguments
///
/// - `&'static str` - The separator text.
///
/// # Returns
///
/// - `VirtualNode` - The breadcrumb separator virtual DOM tree.
fn breadcrumb_separator_node(separator: &'static str) -> VirtualNode {
    breadcrumb_separator_node_with(separator, VirtualNode::Empty)
}

/// Builds the glyph that divides two crumbs.
///
/// # Arguments
///
/// - `&'static str` - The separator text.
/// - `VirtualNode` - The children the separator carries, if any.
///
/// # Returns
///
/// - `VirtualNode` - The breadcrumb separator virtual DOM tree.
fn breadcrumb_separator_node_with(separator: &'static str, children: VirtualNode) -> VirtualNode {
    let content: VirtualNode = match children {
        VirtualNode::Empty => {
            VirtualNode::Text(TextNode::new(Cow::Owned(separator.to_string()), None))
        }
        other => other,
    };
    html! {
        li {
            class: c_euv_breadcrumb_sep()
            role: "presentation"
            "aria-hidden": "true"
            "data-slot": "breadcrumb-separator"
            content
        }
    }
}
