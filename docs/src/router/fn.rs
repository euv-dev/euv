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

/// Flattens a sidebar scope into its ordered navigable items.
///
/// A navigable item is either a leaf (no children) with a link, or a
/// group/index page (children present) with its own link. Pure
/// container groups (no link, only children) recurse transparently so
/// the resulting pool contains every page the reader can actually
/// navigate to via prev/next, in sidebar order.
///
/// Flattening does **not** stop at a directory boundary: the whole
/// subtree is one ordered list, because that list is the prev/next
/// pager's universe. A directory is a presentation grouping, not a
/// boundary in the document, so the page after the last file of one
/// directory is the next directory's index rather than nothing.
///
/// # Arguments
///
/// - `&'static [EuvSidebarItem]` - The sidebar scope to flatten.
///
/// # Returns
///
/// - `Vec<&'static EuvSidebarItem>` - Navigable items, in display order.
pub fn flatten_links(items: &'static [EuvSidebarItem]) -> Vec<&'static EuvSidebarItem> {
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

/// Builds the collapse key of a sidebar group.
///
/// The key format is euv-ui's `euv_sidebar_item` recursion contract
/// (`{prefix}/{text}`, with the top level's prefix empty), and both
/// ends of the docs shell depend on it agreeing exactly: the seeded
/// default-collapsed state, the reveal pass that expands the active
/// page's ancestors, and the component that reads the list. Owning the
/// format here is what keeps the three from drifting apart.
///
/// # Arguments
///
/// - `&str` - The group key prefix of the parent level.
/// - `&str` - The group's display text.
///
/// # Returns
///
/// - `String` - The group's collapse key.
pub fn sidebar_group_key(prefix: &str, text: &str) -> String {
    let mut key: String = String::with_capacity(prefix.len() + 1 + text.len());
    key.push_str(prefix);
    key.push(SIDEBAR_GROUP_KEY_SEPARATOR);
    key.push_str(text);
    key
}

/// Collects every group key in a sidebar tree.
///
/// Seeding the collapsed-keys signal with this list folds the tree down
/// to its group titles, which is the state the docs shell mounts with:
/// a 240-directory sidebar opened to every leaf is a wall of links that
/// pushes the sections the reader is not in off the bottom.
///
/// Leaves are skipped — they carry no key and cannot collapse. Parents
/// are listed before the groups nested under them, so the order is
/// stable and matches the tree.
///
/// # Arguments
///
/// - `&'static [EuvSidebarItem]` - The sidebar tree to walk.
///
/// # Returns
///
/// - `Vec<String>` - One key per group, at any depth.
pub fn all_group_keys(items: &'static [EuvSidebarItem]) -> Vec<String> {
    let mut keys: Vec<String> = Vec::new();
    collect_group_keys(items, SIDEBAR_GROUP_KEY_ROOT, &mut keys);
    keys
}

/// Walks one tree level, appending every group key under `prefix` to
/// `keys`. Recurses depth-first so a parent key is always listed before
/// its children.
///
/// # Arguments
///
/// - `&'static [EuvSidebarItem]` - The items of the current tree level.
/// - `&str` - The group key prefix of the parent level.
/// - `&mut Vec<String>` - The collected group keys.
fn collect_group_keys(items: &'static [EuvSidebarItem], prefix: &str, keys: &mut Vec<String>) {
    for item in items {
        if item.children.is_empty() {
            continue;
        }
        let key: String = sidebar_group_key(prefix, item.text);
        keys.push(key.clone());
        collect_group_keys(item.children, &key, keys);
    }
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
