use super::*;

/// A generic breadcrumb trail aligned with common site frameworks.
///
/// Renders a `<nav>` holding one node per entry: ancestors become links
/// (`http`-prefixed targets open in a new tab, everything else goes through
/// the hash router), and the last entry renders as the current page span
/// instead of a link. A separator span is rendered between two entries.
/// An empty `items` list renders nothing.
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
            crumbs.push(separator_node(separator));
        }
        if index == last_index {
            crumbs.push(current_crumb(*item));
        } else {
            crumbs.push(link_crumb(*item));
        }
    }
    html! {
        nav {
            class: c_euv_breadcrumb()
            crumbs
        }
    }
}

/// Renders the separator span between two crumbs.
///
/// # Arguments
///
/// - `&'static str` - The separator text.
///
/// # Returns
///
/// - `VirtualNode` - The separator virtual DOM tree.
fn separator_node(separator: &'static str) -> VirtualNode {
    html! {
        span {
            class: c_euv_breadcrumb_sep()
            {
                separator
            }
        }
    }
}

/// Renders the current page crumb as a non-clickable span.
///
/// # Arguments
///
/// - `EuvBreadcrumbItem` - The current page entry.
///
/// # Returns
///
/// - `VirtualNode` - The current crumb virtual DOM tree.
fn current_crumb(item: EuvBreadcrumbItem) -> VirtualNode {
    html! {
        span {
            class: c_euv_breadcrumb_current()
            {
                item.label
            }
        }
    }
}

/// Renders an ancestor crumb as a link.
///
/// External (`http`-prefixed) targets open in a new tab; every other target
/// is treated as an internal hash route and prefixed with
/// `ROUTE_HASH_PREFIX` so the href matches what the router writes.
///
/// # Arguments
///
/// - `EuvBreadcrumbItem` - The ancestor entry.
///
/// # Returns
///
/// - `VirtualNode` - The link crumb virtual DOM tree.
fn link_crumb(item: EuvBreadcrumbItem) -> VirtualNode {
    if item.href.starts_with("http") {
        return html! {
            a {
                key: item.href
                class: c_euv_breadcrumb_item()
                href: item.href
                target: "_blank"
                onclick: Router::external_link_handler(item.href)
                {
                    item.label
                }
            }
        };
    }
    html! {
        a {
            key: item.href
            class: c_euv_breadcrumb_item()
            href: {
                let mut
                href: String =
                String::with_capacity(ROUTE_HASH_PREFIX.len() + item.href.len());
                href.push_str(ROUTE_HASH_PREFIX);
                href.push_str(item.href);
                href
            }
            onclick: Router::link_handler(item.href)
            {
                item.label
            }
        }
    }
}
