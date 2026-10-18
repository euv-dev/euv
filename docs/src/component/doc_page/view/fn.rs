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
/// - `VirtualNode<DocsPageProps>` - The props node carrying the route signal.
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
                class: c_euv_display_contents()
                docs_home_page {
                    route_signal
                }
            }
        },
        Some(page) if page.private && !is_unlocked(&path) => html! {
            div {
                key: path.clone()
                class: c_euv_display_contents()
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
                class: c_euv_display_contents()
                docs_doc_page {
                    route_signal
                }
            }
        },
        None => html! {
            div {
                key: path.clone()
                class: c_euv_display_contents()
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
/// - `VirtualNode<DocsPageProps>` - The props node carrying the route signal.
///
/// # Returns
///
/// - `VirtualNode` - The doc page virtual DOM tree.
#[component]
pub(crate) fn docs_doc_page(node: VirtualNode<DocsPageProps>) -> VirtualNode {
    let DocsPageProps { route_signal }: DocsPageProps = node.try_get_props().unwrap_or_default();
    // The spy lives here rather than in the shell because the TOC it
    // feeds is per-page state: a new page means a new entry list, and
    // the spy re-marks the first heading against that list on mount.
    let toc_active: Signal<String> = App::use_signal(String::new);
    use_toc_scroll_spy(toc_active);
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
            toc_active: toc_active
            prev_label: locale.prev_label
            next_label: locale.next_label
            prev: prev
            next: next
            footer: footer_text
            if { !page.title.is_empty() } {
                h1 {
                    class: CLASS_DOCS_PAGE_TITLE
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

/// Keeps the active heading marked in the table of contents as the
/// reader scrolls, and keeps the URL hash naming that same heading.
///
/// A reactive `Signal<String>` holds the active entry's `href` and the
/// TOC paints its `_active` class straight off it, so the highlight is
/// ordinary declarative state instead of script mutating class lists
/// behind the renderer's back. The spy only reads geometry, writes one
/// signal, and mirrors the result into `location.hash`.
///
/// Geometry is polled rather than listened to, and the reason is
/// specific: this shell locks the viewport and scrolls an inner
/// container, so a delegated `scroll` listener on `document` never sees
/// that scroll, while `requestAnimationFrame` stops outright in a
/// hidden tab. A short interval reads one property per tick and
/// early-outs when it has not moved, so it is effectively free and
/// behaves the same in a background tab.
///
/// The URL hash is the single source of truth for the section intent,
/// and the flows around it are deliberately asymmetric so no feedback
/// loop can exist:
///
/// - **Click / back / paste (hashchange)** — the fragment is an intent:
///   it wins the highlight while the scroll position has not moved since
///   the hash changed (the anchor scroll lands within one tick), and the
///   route signal's own subscriber performs the scroll to the anchor.
/// - **Scroll (reader)** — geometry decides the active entry and the
///   result is written back with `history.replaceState`, which fires no
///   `hashchange`: the URL follows the reader without re-triggering the
///   anchor scroll or flooding the history stack. Because scroll never
///   calls `set_hash`, and the hashchange path never writes the hash,
///   neither direction can re-enter the other.
///
/// Once the reader scrolls away from the position a hash change pointed
/// at, the fragment intent is spent and geometry takes over — so a
/// clicked entry can never pin the highlight forever.
///
/// # Arguments
///
/// - `Signal<String>` - The active TOC entry href, written by the spy.
fn use_toc_scroll_spy(active: Signal<String>) {
    // The hash the spy last saw (or wrote itself) and the scroll position
    // at that moment. A hash that differs from `seen_hash` arrived from
    // outside (click, back/forward, paste); while the scroll position has
    // not moved since, its fragment is treated as the reader's intent.
    let seen_hash: Signal<String> = App::use_signal(|| {
        window()
            .and_then(|window: Window| window.location().hash().ok())
            .unwrap_or_default()
    });
    let scroll_at_hash: Signal<i32> = App::use_signal(|| 0);
    App::use_interval(TOC_SPY_INTERVAL_MILLIS, move || {
        let Some(window) = window() else {
            return;
        };
        let Some(document) = window.document() else {
            return;
        };
        let Ok(Some(container)) = document.query_selector(TOC_SPY_CONTAINER_SELECTOR) else {
            return;
        };
        let hash: String = window.location().hash().unwrap_or_default();
        let scroll_top: i32 = container.scroll_top();
        // Compared through [`same_route_anchor`], never as raw strings:
        // `seen_hash` holds the raw slug this spy wrote, while
        // `location.hash` reads it back percent-encoded. A raw `!=`
        // therefore mistakes the spy's own previous write for an
        // external change on the next tick, re-arming `scroll_at_hash`
        // and letting the click/deeplink fragment keep winning the
        // highlight over the geometry the reader is scrolling into.
        if !same_route_anchor(&hash, &seen_hash.get()) {
            seen_hash.set(hash.clone());
            scroll_at_hash.set(scroll_top);
        }
        let Some(entries) = toc_entry_hrefs(&document) else {
            return;
        };
        // The fragment decides only while the scroll has not moved since
        // the hash last changed; after that, geometry owns the answer.
        let fragment_pick: Option<String> = if scroll_top == scroll_at_hash.get() {
            fragment_target(&window, &entries)
        } else {
            None
        };
        let Some(next) = fragment_pick.or_else(|| scroll_target(&container, &document, &entries))
        else {
            return;
        };
        if next != active.get() {
            active.set(next.clone());
        }
        if !same_route_anchor(&hash, &next) {
            // Scroll-driven half of the sync: replaceState fires no
            // hashchange, so this write cannot re-enter the anchor scroll.
            sync_hash(&window, &next);
            seen_hash.set(next);
            scroll_at_hash.set(scroll_top);
        }
    });
}

/// Collects every TOC entry's `href` in display order.
///
/// Returns `None` when the page has no TOC at all — the caller treats
/// that as "nothing to mark", leaving the previous highlight alone so a
/// re-render does not flicker it off and back on.
///
/// # Arguments
///
/// - `&Document` - The document to query.
///
/// # Returns
///
/// - `Option<Vec<String>>` - The entry hrefs, or `None` when there are none.
fn toc_entry_hrefs(document: &Document) -> Option<Vec<String>> {
    let Ok(links) = document.query_selector_all(TOC_SPY_LINK_SELECTOR) else {
        return None;
    };
    let mut entries: Vec<String> = Vec::new();
    for index in 0..links.length() {
        let Some(node) = links.item(index) else {
            continue;
        };
        let element: Element = node.unchecked_into();
        let Some(href) = element.get_attribute(HREF_ATTR) else {
            continue;
        };
        entries.push(href);
    }
    if entries.is_empty() {
        None
    } else {
        Some(entries)
    }
}

/// Mirrors the active entry into the URL fragment.
///
/// `history.replaceState` — unlike `location.set_hash` — fires no
/// `hashchange`, which is the structural reason the scroll-driven hash
/// update cannot loop back into the anchor scroll.
///
/// # Arguments
///
/// - `&Window` - The window whose history is rewritten.
/// - `&str` - The entry href (`#<route>#<slug>`) to place in the URL.
fn sync_hash(window: &Window, href: &str) {
    let Ok(history) = window.history() else {
        return;
    };
    let _: Result<(), JsValue> = history.replace_state_with_url(&JsValue::NULL, "", Some(href));
}

/// Extracts the heading slug an anchor's href points at.
///
/// # Arguments
///
/// - `&str` - The anchor href (`#<route>#<slug>`).
///
/// # Returns
///
/// - `Option<String>` - The slug, empty when the href carries none.
fn href_anchor(href: &str) -> Option<String> {
    let index: Option<usize> = href.rfind('#').filter(|index: &usize| *index > 0);
    index
        .map(|index: usize| href[index + 1..].to_string())
        .filter(|anchor: &String| !anchor.is_empty())
}

/// Matches the URL fragment against the TOC entries.
///
/// The fragment arrives percent-encoded while heading ids are raw
/// UTF-8, so it is decoded before comparison; a fragment that fails to
/// decode is compared verbatim rather than dropped. Prefix matching
/// backs the longest-id-wins case, so a fragment naming a nested
/// heading still resolves when both it and its parent section are in
/// the TOC.
///
/// # Arguments
///
/// - `&Window` - The window, read for `location.hash`.
/// - `&[String]` - Every TOC entry href.
///
/// # Returns
///
/// - `Option<String>` - The matching entry's href.
fn fragment_target(window: &Window, entries: &[String]) -> Option<String> {
    let Ok(hash) = window.location().hash() else {
        return None;
    };
    let anchor_start: Option<usize> = hash.rfind('#').filter(|index: &usize| *index > 0);
    let index: usize = anchor_start?;
    let raw: &str = &hash[index + 1..];
    if raw.is_empty() {
        return None;
    }
    let anchor: String = decode_anchor(raw);
    let mut best: Option<String> = None;
    let mut best_len: i32 = -1;
    for href in entries {
        let Some(slug) = href_anchor(href) else {
            continue;
        };
        if slug == anchor {
            best = Some(href.clone());
            break;
        }
        if anchor.starts_with(&slug) && slug.len() as i32 > best_len {
            best_len = slug.len() as i32;
            best = Some(href.clone());
        }
    }
    best
}

/// Picks the entry for the heading sitting at the reader's reading line.
///
/// The last heading wins once the container is scrolled to its end,
/// because there the final section is the one being read and the first
/// still-visible heading is often the very first on the page.
///
/// # Arguments
///
/// - `&Element` - The scrolling container the spy measures against.
/// - `&Document` - The document to query.
/// - `&[String]` - Every TOC entry href.
///
/// # Returns
///
/// - `Option<String>` - The active entry's `href`.
fn scroll_target(container: &Element, document: &Document, entries: &[String]) -> Option<String> {
    let rect: DomRect = container.get_bounding_client_rect();
    let line: f64 = rect.top() + TOC_SPY_READING_LINE_PX;
    let at_end: bool = container.scroll_top() + container.client_height()
        >= container.scroll_height() - TOC_SPY_END_TOLERANCE_PX as i32;
    if at_end && entries.len() > 1 {
        return entries.last().cloned();
    }
    let mut best: Option<String> = None;
    let mut best_top: f64 = f64::NEG_INFINITY;
    for href in entries {
        let Some(slug) = href_anchor(href) else {
            continue;
        };
        let Some(heading) = document.get_element_by_id(slug.as_str()) else {
            continue;
        };
        let top: f64 = heading.get_bounding_client_rect().top();
        if top <= line && top > best_top {
            best_top = top;
            best = Some(href.clone());
        }
    }
    best.or_else(|| entries.first().cloned())
}

/// Computes the prev/next pagination entries around the current route.
///
/// The pager pool is the **whole locale sidebar flattened in display
/// order**, not the current directory's own sibling list. A directory's
/// index page sits at the head of its own subtree, so a per-directory
/// pool left its `prev` empty even when the last page of the previous
/// directory was right there in the tree; the same held for the tail of
/// every directory. Flattening across directory boundaries makes the
/// pager a property of *the file's position in the document*, which is
/// what a reader expects: "the page before this one" and "the page after
/// this one", crossing into the neighbouring directory when this file
/// is the first or last of its own.
///
/// Sidebar order is the pool order, so the pager always agrees with what
/// the sidebar shows — including its explicit `sidebar_order` / `order`
/// frontmatter pinning. A route with no sidebar entry (a page marked
/// `sidebar: false`, or one reached only by its home link) has no
/// position in that order, so it gets no pager rather than a guess.
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
    let pool: Vec<&'static EuvSidebarItem> = flatten_links(locale.sidebar);
    let pool_index: Option<usize> = pool
        .iter()
        .position(|item: &&'static EuvSidebarItem| item.link == Some(route));
    let Some(pool_index) = pool_index else {
        return (None, None);
    };
    let to_item: fn(Option<&&'static EuvSidebarItem>) -> Option<EuvPaginationItem> =
        |item: Option<&&'static EuvSidebarItem>| {
            item.and_then(|item: &&'static EuvSidebarItem| {
                item.link.map(|link: &'static str| EuvPaginationItem {
                    text: item.text,
                    link,
                })
            })
        };
    let prev: Option<EuvPaginationItem> =
        to_item(pool_index.checked_sub(1).and_then(|i: usize| pool.get(i)));
    let next: Option<EuvPaginationItem> = to_item(pool.get(pool_index + 1));
    (prev, next)
}
