use super::super::*;

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
    if !encoded.as_bytes().contains(&b'%') {
        // Fast path: nothing to decode, skip the FFI roundtrip.
        return encoded.to_string();
    }
    match js_sys::decode_uri_component(encoded) {
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
pub(crate) fn parse_route(raw: &str) -> (String, Option<String>) {
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
    site.locales
        .iter()
        .filter(|locale| locale.prefix != "/")
        .find(|locale| route.starts_with(locale.prefix))
        .or_else(|| site.locales.iter().find(|locale| locale.prefix == "/"))
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
    site.pages
        .iter()
        .find(|page| page.route == route)
        .or_else(|| {
            // `/guide` → `/guide/`, `/guide/` stays as-is.
            if route.ends_with('/') || route.ends_with(".html") {
                None
            } else {
                let with_slash: String = format!("{route}/");
                site.pages.iter().find(|page| page.route == with_slash)
            }
        })
}

/// Flattens a sidebar tree into its ordered leaf links (for prev/next).
///
/// # Arguments
///
/// - `&'static [EuvSidebarItem]` - The sidebar tree.
///
/// # Returns
///
/// - `Vec<&'static EuvSidebarItem>` - Leaf items with links, in display order.
pub(crate) fn flat_sidebar_links(items: &'static [EuvSidebarItem]) -> Vec<&'static EuvSidebarItem> {
    let mut out: Vec<&'static EuvSidebarItem> = Vec::new();
    for item in items {
        if item.children.is_empty() {
            if item.link.is_some() {
                out.push(item);
            }
        } else {
            out.extend(flat_sidebar_links(item.children));
        }
    }
    out
}

/// Maps a route to the equivalent route in another locale.
///
/// Falls back to the target locale home when the page has no counterpart.
///
/// # Arguments
///
/// - `&str` - The current page route path.
/// - `&'static DocsLocale` - The target locale.
///
/// # Returns
///
/// - `String` - The target route.
pub(crate) fn route_in_locale(route: &str, target: &'static DocsLocale) -> String {
    let current: &DocsLocale = locale_of(route);
    let suffix: &str = route
        .strip_prefix(current.prefix.trim_end_matches('/'))
        .unwrap_or(route);
    let suffix: &str = if suffix.is_empty() { "/" } else { suffix };
    let candidate: String = if target.prefix == "/" {
        suffix.to_string()
    } else {
        format!("{}{}", target.prefix.trim_end_matches('/'), suffix)
    };
    if find_page(&candidate).is_some() {
        candidate
    } else {
        target.prefix.to_string()
    }
}
