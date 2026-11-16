use super::*;

/// A page-navigation container that supports both prev/next docs links and
/// the composed shadcn pagination parts.
///
/// With no children it renders two bordered previous/next cards; when
/// children are supplied it renders them inside a navigation landmark, so
/// callers can compose numbered links and ellipses with
/// `euv_pagination_content`.
///
/// # Arguments
///
/// - `VirtualNode<EuvPaginationProps>` - The props node.
///
/// # Returns
///
/// - `VirtualNode` - The pagination virtual DOM tree.
#[component]
pub fn euv_pagination(node: VirtualNode<EuvPaginationProps>) -> VirtualNode {
    let EuvPaginationProps {
        prev_label,
        next_label,
        prev,
        next,
    }: EuvPaginationProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    if !matches!(&children, VirtualNode::Empty) {
        let shadcn_class: &Css = { c_euv_pagination() };
        return html! {
            nav {
                role: "navigation"
                aria-label: "pagination"
                "data-slot": "pagination"
                class: shadcn_class
                children
            }
        };
    }
    let prev_node: VirtualNode = pagination_side(prev, prev_label, false);
    let next_node: VirtualNode = pagination_side(next, next_label, true);
    html! {
        nav {
            role: "navigation"
            aria-label: "pagination"
            "data-slot": "pagination"
            class: c_euv_pagination()
            ul {
                "data-slot": "pagination-content"
                class: c_euv_pagination_auto_content()
                prev_node
                next_node
            }
        }
    }
}

/// Renders one side of the pagination footer.
///
/// # Arguments
///
/// - `Option<EuvPaginationItem>` - The entry; `None` renders a spacer.
/// - `&'static str` - The small uppercase label above the entry text.
/// - `bool` - Whether this is the right-aligned next side.
///
/// # Returns
///
/// - `VirtualNode` - The side virtual DOM tree.
fn pagination_side(
    item: Option<EuvPaginationItem>,
    label: &'static str,
    is_next: bool,
) -> VirtualNode {
    let Some(item) = item else {
        return html! {
            li {
                "data-slot": "pagination-item"
                class: c_euv_pagination_spacer()
            }
        };
    };
    html! {
        euv_pagination_item {
            label: label
            text: item.text
            link: item.link
            next: is_next
        }
    }
}

/// One entry of a pagination footer.
///
/// The item is the `pagination-item` part of the reference: it holds one
/// reachable page of the footer, and the link inside it is the
/// `pagination-link` part.
///
/// # Arguments
///
/// - `VirtualNode<EuvPaginationItemProps>` - The props node carrying the
///   label of the page, the route it points at and whether it is the current
///   page.
///
/// # Returns
///
/// - `VirtualNode` - The euv pagination item virtual DOM tree.
#[component]
pub fn euv_pagination_item(node: VirtualNode<EuvPaginationItemProps>) -> VirtualNode {
    let EuvPaginationItemProps {
        label,
        link,
        text,
        next,
    }: EuvPaginationItemProps = node.try_get_props().unwrap_or_default();
    let href: String = pagination_href(link);
    let href_ref: &str = href.as_str();
    let base_class: &Css = c_euv_pagination_link();
    let direction_class: &Css = if next {
        c_euv_pagination_next()
    } else {
        c_euv_pagination_no_next()
    };

    let direction_shadcn_class: &Css = {
        if next {
            c_euv_pagination_next()
        } else {
            c_euv_pagination_previous()
        }
    };
    html! {
        li {
            "data-slot": "pagination-item"
            class: c_euv_pagination_item()
            a {
                "data-slot": if next {
                    "pagination-next"
                } else {
                    "pagination-previous"
                }
                "aria-label": if next {
                    PAGINATION_NEXT_ARIA_LABEL
                } else {
                    PAGINATION_PREVIOUS_ARIA_LABEL
                }
                role: "button"
                tabindex: "0"
                class: base_class
                class: direction_class

                class: direction_shadcn_class
                href: href_ref
                onclick: Router::link_handler(link)
                if !next {
                    span {
                        "aria-hidden": "true"
                        class: c_euv_pagination_icon()
                        PAGINATION_PREVIOUS_ICON
                    }
                }
                span {
                    class: c_euv_pagination_label()
                    label
                }
                span {
                    class: c_euv_pagination_text()
                    text
                }
                if next {
                    span {
                        "aria-hidden": "true"
                        class: c_euv_pagination_icon()
                        PAGINATION_NEXT_ICON
                    }
                }
            }
        }
    }
}

/// The row of entries of a pagination footer.
///
/// The content is the `pagination-content` part of the reference: it is the
/// row the items sit in, so the items keep their order and their spacing.
///
/// # Arguments
///
/// - `VirtualNode<EuvPaginationContentProps>` - The props node carrying the
///   component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv pagination content virtual DOM tree.
#[component]
pub fn euv_pagination_content(node: VirtualNode<EuvPaginationContentProps>) -> VirtualNode {
    let EuvPaginationContentProps {}: EuvPaginationContentProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let base_class: &Css = c_euv_pagination_content();

    html! {
        ul {
            "data-slot": "pagination-content"
            class: base_class

            children
        }
    }
}

/// The mark that stands for the pages between two reachable ones.
///
/// The ellipsis is the `pagination-ellipsis` part of the reference: it
/// tells the reader that the page list continues without naming every
/// page in it.
///
/// # Arguments
///
/// - `VirtualNode<EuvPaginationEllipsisProps>` - The props node carrying the
///   component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv pagination ellipsis virtual DOM tree.
#[component]
pub fn euv_pagination_ellipsis(node: VirtualNode<EuvPaginationEllipsisProps>) -> VirtualNode {
    let EuvPaginationEllipsisProps {}: EuvPaginationEllipsisProps =
        node.try_get_props().unwrap_or_default();

    html! {
        li {
            "data-slot": "pagination-item"
            class: c_euv_pagination_item()
            span {
                "data-slot": "pagination-ellipsis"
                role: "img"
                "aria-label": PAGINATION_ELLIPSIS_ARIA_LABEL
                class: c_euv_pagination_ellipsis()

                PAGINATION_ELLIPSIS_GLYPH
            }
        }
    }
}

/// Builds the hash route a pagination link points at.
///
/// # Arguments
///
/// - `&'static str` - The route the link points at.
///
/// # Returns
///
/// - `String` - The hash route of the link.
fn pagination_href(link: &'static str) -> String {
    let mut href: String = String::with_capacity(ROUTE_HASH_PREFIX.len() + link.len());
    href.push_str(ROUTE_HASH_PREFIX);
    href.push_str(link);
    href
}

/// One page of a page list.
///
/// The link points at one page and tells whether that page is the one the
/// reader is on. The current page carries the active state so it can be told
/// apart from the pages the reader can jump to.
///
/// # Arguments
///
/// - `VirtualNode<EuvPaginationLinkProps>` - The props node carrying the
///   label of the page, the route it points at and whether it is the current
///   page.
///
/// # Returns
///
/// - `VirtualNode` - The page link element.
#[component]
pub fn euv_pagination_link(node: VirtualNode<EuvPaginationLinkProps>) -> VirtualNode {
    let EuvPaginationLinkProps {
        label,
        link,
        active,
    }: EuvPaginationLinkProps = node.try_get_props().unwrap_or_default();
    let href: String = pagination_href(link);
    let href_ref: &str = href.as_str();
    let base_class: &Css = c_euv_pagination_page();
    let active_class: &Css = if active {
        c_euv_pagination_page_active()
    } else {
        c_euv_pagination_page_inactive()
    };

    let active_shadcn_class: &Css = if active {
        c_euv_pagination_page_active()
    } else {
        &EMPTY_CLASS
    };
    html! {
        li {
            "data-slot": "pagination-item"
            class: c_euv_pagination_item()
            a {
                "data-slot": "pagination-link"
                role: "button"
                tabindex: "0"
                "data-active": if active { "true" } else { "false" }
                aria-current: if active { "page" } else { "false" }
                class: base_class
                class: active_class

                class: active_shadcn_class
                href: href_ref
                onclick: Router::link_handler(link)
                label
            }
        }
    }
}
