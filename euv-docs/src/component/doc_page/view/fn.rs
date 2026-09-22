use super::*;

/// Routes the current path to the home page, a doc page, or the 404 page.
///
/// For pages marked `private: true` in their markdown frontmatter, the
/// doc body is replaced with a [`docs_password_gate`] prompt until the
/// correct password has been entered in this browser (and the unlock
/// record has been written into `localStorage`). Both the initial URL
/// paste case and the in-app route switch case go through this branch
/// — the gate checks `is_unlocked(route)` on every render, so a refresh
/// of a previously-unlocked route skips the prompt automatically, and
/// a successful submit writes the unlock record + forces a re-render
/// that drops the gate in favour of the page body.
///
/// # Arguments
///
/// - `DocsPageProps` - The typed props containing the route signal.
///
/// # Returns
///
/// - `VirtualNode` - The matched page virtual DOM tree.
#[component]
pub(crate) fn docs_main(node: VirtualNode<DocsPageProps>) -> VirtualNode {
    let DocsPageProps { route_signal }: DocsPageProps = node.try_get_props().unwrap_or_default();
    let (path, _anchor) = parse_route(&route_signal.get());
    match find_page(&path) {
        Some(page) if page.home => html! {
            div {
                key: path.clone()
                style: "display: contents"
                docs_home_page {
                    route_signal
                }
            }
        },
        Some(page) if page.private && !is_unlocked(&path) => html! {
            div {
                key: path.clone()
                style: "display: contents"
                docs_password_gate {
                    route: page.route
                    expected_hash: page.password_hash
                    title: page.title
                }
            }
        },
        Some(_) => html! {
            div {
                key: path.clone()
                style: "display: contents"
                docs_doc_page {
                    route_signal
                }
            }
        },
        None => html! {
            div {
                key: path.clone()
                style: "display: contents"
                docs_not_found {
                    route_signal
                }
            }
        },
    }
}

/// Renders one documentation page out of euv-ui components: `euv_markdown`
/// for the body, `euv_pagination` for prev/next links, the footer and the
/// right `euv_toc` anchor column.
///
/// # Arguments
///
/// - `DocsPageProps` - The typed props containing the route signal.
///
/// # Returns
///
/// - `VirtualNode` - The doc page virtual DOM tree.
#[component]
pub(crate) fn docs_doc_page(node: VirtualNode<DocsPageProps>) -> VirtualNode {
    let DocsPageProps { route_signal }: DocsPageProps = node.try_get_props().unwrap_or_default();
    let (path, _anchor) = parse_route(&route_signal.get());
    let locale: &DocsLocale = locale_of(&path);
    let Some(page) = find_page(&path) else {
        return html! {
            ""
        };
    };
    let (prev, next) = prev_next(locale, page.route);
    let footer_text: &str = if page.footer.is_empty() {
        locale.footer
    } else {
        page.footer
    };
    html! {
        euv_doc_layout {
            toc_title: locale.toc_label
            toc_items: page.headings
            prev_label: locale.prev_label
            next_label: locale.next_label
            prev: prev
            next: next
            footer: footer_text
            if { !page.title.is_empty() } {
                h1 {
                    class: "c_docs_page_title"
                    {
                        page.title
                    }
                }
            }
            euv_markdown {
                blocks: page.blocks
            }
        }
    }
}

/// Computes the prev/next pagination entries around the current route.
///
/// Locates the current page in the sidebar tree, then walks the sibling
/// scope (the array containing the matched item as a direct child). For
/// leaf pages the scope is the immediate sibling array; for directory
/// READMEs (group with link) the scope is the parent array so the pager
/// stays anchored inside the directory instead of hopping to an unrelated
/// project.
///
/// # Arguments
///
/// - `&'static DocsLocale` - The current locale.
/// - `&str` - The current page route.
///
/// # Returns
///
/// - `(Option<EuvPaginationItem>, Option<EuvPaginationItem>)` - Prev and next.
fn prev_next(
    locale: &'static DocsLocale,
    route: &str,
) -> (Option<EuvPaginationItem>, Option<EuvPaginationItem>) {
    let to_item = |item: &'static EuvSidebarItem| -> Option<EuvPaginationItem> {
        item.link.map(|link: &'static str| EuvPaginationItem {
            text: item.text,
            link,
        })
    };
    let Some(scope) = scope_for(locale.sidebar, route) else {
        return (None, None);
    };
    let pool: Vec<&'static EuvSidebarItem> = flatten_links(scope);
    let pool_index: Option<usize> = pool
        .iter()
        .position(|item: &&'static EuvSidebarItem| item.link == Some(route));
    let Some(pool_index) = pool_index else {
        return (None, None);
    };
    let prev: Option<EuvPaginationItem> = pool_index
        .checked_sub(1)
        .and_then(|i: usize| to_item(pool[i]));
    let next: Option<EuvPaginationItem> = pool
        .get(pool_index + 1)
        .and_then(|item: &&'static EuvSidebarItem| to_item(item));
    (prev, next)
}
