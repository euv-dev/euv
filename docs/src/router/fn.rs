use super::*;

/// Decodes a percent-encoded anchor slug from a hash route.
///
/// Browser URL hash for non-ASCII characters arrives as the
/// percent-encoded form (e.g. `%E7%8E%AF%E5%A2%83%E8%A6%81%E6%B1%82`
/// for `环境要求`), but the heading `<h2 id="…">` elements are
/// written in the raw UTF-8 form by `build.rs`. `getElementById`
/// needs the raw form to match, so decode before querying.
///
/// Falls back to the input verbatim if decoding fails or if
/// `js_sys` is unavailable (e.g. during a unit test outside the
/// browser), so a malformed anchor never panics the page.
///
/// # Arguments
///
/// - `&str` - The percent-encoded anchor slug (already stripped
///   of the leading `#`).
///
/// # Returns
///
/// - `String` - The decoded UTF-8 anchor slug, or the input
///   unchanged if decoding fails.
fn decode_anchor(encoded: &str) -> String {
    if !encoded.as_bytes().contains(&ANCHOR_ESCAPE) {
        // Fast path: nothing to decode, skip the FFI roundtrip.
        return encoded.to_string();
    }
    match decode_uri_component(encoded) {
        Ok(value) => value.as_string().unwrap_or_else(|| encoded.to_string()),
        Err(_) => encoded.to_string(),
    }
}

/// Splits a raw route into its page path and optional in-page anchor.
///
/// `/guide/a.html#install` → `("/guide/a.html", Some("install"))`.
///
/// The anchor is percent-decoded so that links to non-ASCII
/// headings (e.g. `/zh/guide/getting-started.html#环境要求`) match
/// the raw UTF-8 `id` attributes emitted by `build.rs`. Without
/// decoding, `getElementById` returns null and the in-page
/// scroll-to-anchor handler in `schedule_scroll` silently
/// regresses to "back to top".
///
/// # Arguments
///
/// - `&str` - The raw hash route.
///
/// # Returns
///
/// - `(String, Option<String>)` - The page path and optional anchor slug.
pub fn parse_route(raw: &str) -> (String, Option<String>) {
    match raw.split_once('#') {
        Some((path, anchor)) if !anchor.is_empty() => {
            (path.to_string(), Some(decode_anchor(anchor)))
        }
        Some((path, _)) => (path.to_string(), None),
        None => (raw.to_string(), None),
    }
}

/// Finds the locale owning a route (longest prefix match).
///
/// # Arguments
///
/// - `&str` - The page route path.
///
/// # Returns
///
/// - `&'static DocsLocale` - The matched locale (root locale as fallback).
pub(crate) fn locale_of(route: &str) -> &'static DocsLocale {
    let site: &DocsSite = &crate::generated::SITE;
    let owner: Option<&'static RouteLocale> = route_site().locale_of(route);
    let Some(owner) = owner else {
        return &site.locales[0];
    };
    site.locales
        .iter()
        .find(|locale: &&DocsLocale| locale.prefix == owner.prefix)
        .unwrap_or(&site.locales[0])
}

/// Looks up a page by route, normalizing missing trailing forms.
///
/// # Arguments
///
/// - `&str` - The page route path.
///
/// # Returns
///
/// - `Option<&'static DocsPage>` - The page when found.
pub(crate) fn find_page(route: &str) -> Option<&'static DocsPage> {
    let site: &DocsSite = &crate::generated::SITE;
    let found: &'static RoutePage = route_site().find_page(route)?;
    site.pages
        .iter()
        .find(|page: &&DocsPage| page.route == found.route)
}

/// Finds the sidebar scope that contains the given route.
///
/// Returns the level of the sidebar tree where the route appears as a
/// direct child. Group/index pages (items with both a link and children)
/// appear in the scope alongside leaf pages, so clicking a directory
/// README still gets a usable pager anchored inside their own directory.
///
/// When the direct scope has fewer than two navigable items (e.g. a
/// single-content directory whose only sidebar entry is its LICENSE),
/// the result bubbles up one level — the level whose array contains
/// the direct scope as a child. The pager pool then includes the
/// sibling group/index page and other leaves so prev/next can walk
/// instead of being empty. The bubble stops as soon as the parent
/// scope has at least two navigable items, or the recursion reaches
/// the top-level array (where further bubbling would have no parent
/// to consult).
///
/// Walks the tree depth-first. The recursive call does NOT bubble up —
/// the matched level is always the lowest one containing the route.
/// Sibling directories that do not contain the route are NOT pulled in
/// from a higher ancestor, so prev/next stays inside the same branch
/// of the sidebar instead of hopping to unrelated projects.
///
/// # Arguments
///
/// - `&'static [EuvSidebarItem]` - The sidebar tree to search.
/// - `&str` - The current route path.
///
/// # Returns
///
/// - `Option<&'static [EuvSidebarItem]>` - The sibling array that
///   contains the matched item as a direct child (possibly bubbled to a
///   higher level so the pool has at least two navigable items), or
///   `None` when the route is not present in this subtree.
pub(crate) fn scope_for(
    items: &'static [EuvSidebarItem],
    route: &str,
) -> Option<&'static [EuvSidebarItem]> {
    for item in items {
        if item.link == Some(route) {
            return Some(items);
        }
        if let Some(found) = scope_for(item.children, route) {
            // Bubble up one level when the deeper scope cannot supply both
            // prev and next: a single-item scope has nothing to walk
            // between, so the parent (this `items` array, which contains
            // `found` as a child) becomes the scope instead. This is a
            // one-step promotion, not a re-search: the route still lives
            // inside `items`, and the parent pool has at least one
            // sibling to give prev or next a real neighbour.
            if flatten_links(found).len() < 2 {
                return Some(items);
            }
            return Some(found);
        }
    }
    None
}

/// Flattens a sidebar scope into its ordered navigable items.
///
/// A navigable item is either a leaf (no children) with a link, or a
/// group/index page (children present) with its own link. Pure
/// container groups (no link, only children) recurse transparently so
/// the resulting pool contains every page the reader can actually
/// navigate to via prev/next, in sidebar order.
///
/// # Arguments
///
/// - `&'static [EuvSidebarItem]` - The sidebar scope to flatten.
///
/// # Returns
///
/// - `Vec<&'static EuvSidebarItem>` - Navigable items, in display order.
pub(crate) fn flatten_links(items: &'static [EuvSidebarItem]) -> Vec<&'static EuvSidebarItem> {
    let mut out: Vec<&'static EuvSidebarItem> = Vec::new();
    for item in items {
        if item.children.is_empty() {
            if item.link.is_some() {
                out.push(item);
            }
        } else {
            if item.link.is_some() {
                out.push(item);
            }
            out.extend(flatten_links(item.children));
        }
    }
    out
}

/// Computes the site root URL path from the current location.
///
/// Every locale bundle is served from its own directory under the site
/// root (the default locale at the root itself, `/en/` from `<root>/en/`),
/// so stripping this bundle's directory (`SITE_LOCALE_DIR`) off the
/// current pathname yields the site root the language switcher and the
/// boot redirect build cross-locale URLs from.
///
/// # Returns
///
/// - `String` - The site root path with a trailing slash (e.g. `/pages/`).
pub(crate) fn site_root() -> String {
    let Some(window) = window() else {
        return URL_PATH_ROOT.to_string();
    };
    let Ok(mut path) = window.location().pathname() else {
        return URL_PATH_ROOT.to_string();
    };
    if !path.ends_with('/') {
        path.push('/');
    }
    if !crate::generated::SITE_LOCALE_DIR.is_empty()
        && let Some(root) = path.strip_suffix(crate::generated::SITE_LOCALE_DIR)
    {
        return root.to_string();
    }
    path
}

/// Redirects boot when the hash names a route this bundle does not serve.
///
/// Per-locale bundling gives every locale its own directory, but links
/// written against the old single-bundle site (`<root>/#/en/…`) still
/// arrive at the default bundle, and alias prefixes (`/zh/…`) arrive at
/// their canonical bundle. `SITE_REDIRECTS` maps those foreign prefixes:
/// an entry with an empty `to_dir` rewrites the hash in place, any other
/// entry replaces the location with the owning bundle's directory.
///
/// # Returns
///
/// - `bool` - `true` when a cross-directory navigation was started and the
///   caller must skip mounting (the page is being replaced).
pub(crate) fn redirect_foreign_route() -> bool {
    let Some(window) = window() else {
        return false;
    };
    let location: Location = window.location();
    let Ok(hash) = location.hash() else {
        return false;
    };
    let path: &str = hash.strip_prefix('#').unwrap_or(&hash);
    for redirect in crate::generated::SITE_REDIRECTS {
        let head: &str = redirect.from;
        let boundary: bool = path == head
            || path
                .strip_prefix(head)
                .is_some_and(|rest: &str| rest.starts_with('/'));
        if !boundary {
            continue;
        }
        let bare: &str = &path[head.len()..];
        let bare: &str = if bare.is_empty() { URL_PATH_ROOT } else { bare };
        if redirect.to_dir.is_empty() {
            let _: Result<(), JsValue> = location.set_hash(bare);
            return false;
        }
        let url: String = format!("{}{}#{}", site_root(), redirect.to_dir, bare);
        let _: Result<(), JsValue> = location.replace(&url);
        return true;
    }
    false
}

/// Builds the route layer's view of the generated site.
///
/// # Returns
///
/// - `RouteSite` - The site's locale prefixes and page routes.
pub(crate) fn build_route_site() -> RouteSite {
    let site: &'static DocsSite = &crate::generated::SITE;
    let locales: &'static [RouteLocale] = leak_locales(site.locales);
    let pages: &'static [RoutePage] = leak_pages(site.pages);
    RouteSite::new(locales, pages)
}

/// Projects the generated locale list onto the route layer's view.
///
/// # Arguments
///
/// - `&'static [DocsLocale]` - The generated locales.
///
/// # Returns
///
/// - `&'static [RouteLocale]` - The same locales, reduced to prefixes.
pub(crate) fn leak_locales(locales: &'static [DocsLocale]) -> &'static [RouteLocale] {
    Box::leak(
        locales
            .iter()
            .map(|locale: &DocsLocale| RouteLocale {
                prefix: locale.prefix,
            })
            .collect::<Vec<RouteLocale>>()
            .into_boxed_slice(),
    )
}

/// Projects the generated page list onto the route layer's view.
///
/// # Arguments
///
/// - `&'static [DocsPage]` - The generated pages.
///
/// # Returns
///
/// - `&'static [RoutePage]` - The same pages, reduced to routes.
pub(crate) fn leak_pages(pages: &'static [DocsPage]) -> &'static [RoutePage] {
    Box::leak(
        pages
            .iter()
            .map(|page: &DocsPage| RoutePage { route: page.route })
            .collect::<Vec<RoutePage>>()
            .into_boxed_slice(),
    )
}

/// Returns the generated site projected onto the route layer's view.
///
/// # Returns
///
/// - `RouteSite` - The site's locale prefixes and page routes.
pub fn route_site() -> RouteSite {
    ROUTE_SITE.with(RouteSite::clone)
}

/// Returns the route prefix the locale owning `route` is served at.
///
/// # Arguments
///
/// - `&str` - The page route path.
///
/// # Returns
///
/// - `&'static str` - The owning locale's prefix.
pub fn locale_prefix_for(route: &str) -> &'static str {
    locale_of(route).prefix
}

/// Returns the display label the locale owning `route` is declared
/// under.
///
/// The label is matched by route prefix against the site's language
/// list, which is where `build.rs` puts every label once per-locale
/// bundling moved them out of `DocsLocale` and into the cross-bundle
/// switcher. A prefix with no matching entry (the root bundle serving
/// routes it does not declare) falls back to the first declared label,
/// so the caller always gets a real string rather than an empty one.
///
/// # Arguments
///
/// - `&str` - The page route path.
///
/// # Returns
///
/// - `&'static str` - The owning locale's label, as `build.rs` emitted
///   it from the content repository's `[[locales]]` frontmatter.
pub fn locale_label_for(route: &str) -> &'static str {
    let prefix: &str = locale_of(route).prefix;
    for language in crate::generated::SITE_LANGUAGES {
        if language.dir.is_empty() && prefix == "/" {
            return language.label;
        }
        if language.dir == prefix.trim_start_matches('/') {
            return language.label;
        }
    }
    crate::generated::SITE_LANGUAGES
        .first()
        .map_or("", |language: &'static DocsLanguageLink| language.label)
}
