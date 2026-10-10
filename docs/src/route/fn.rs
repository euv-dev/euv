use super::*;

/// Reduces a requested route to its canonical form.
///
/// The normalisation is deliberately a *shape* fix, never a *lookup* fix:
/// it only removes the ways a URL can spell a route that `build.rs`
/// already generated, and never guesses at a different page. Every
/// rewrite is one a reader could reasonably have typed by hand:
///
/// - repeated separators collapse (`/zh//a` → `/zh/a`), so a link
///   assembled from a prefix plus a path that already starts with a
///   separator still resolves;
/// - a run of trailing separators collapses to the single one a
///   directory route ends with (`/zh/guide//` → `/zh/guide/`), so a
///   directory route survives a link helper that appends a slash while
///   `/zh/guide` — which names no page at all — stays distinguishable;
/// - the root stays the root, because trimming its only separator would
///   leave a string that matches nothing;
/// - everything else is preserved verbatim, including case and the
///   `.html` suffix — [`RouteSite::find_page`] owns the decision of which
///   of those a page can be addressed by, so this function only
///   guarantees the separator shape matches the generated routes.
///
/// A request with no leading separator (`zh/guide/`) gains one, because
/// every generated route is absolute.
///
/// # Arguments
///
/// - `&str` - The route as it arrived from the hash, after
///   [`parse_route`] removed any in-page anchor.
///
/// # Returns
///
/// - `String` - The canonical form of the route.
pub fn normalize_route(route: &str) -> String {
    let mut collapsed: String = String::with_capacity(route.len() + 1);
    let mut previous_separator: bool = false;
    for ch in route.chars() {
        if ch == ROUTE_SEPARATOR {
            if previous_separator {
                continue;
            }
            previous_separator = true;
        } else {
            previous_separator = false;
        }
        collapsed.push(ch);
    }
    // Whether the request was written as a directory is the one piece of
    // information the generated routes encode in the trailing separator,
    // so it is recorded before the run is trimmed. A `.html` route names
    // a file, and a file never carries a trailing separator — dropping it
    // here is what lets a slash-appending link helper still reach the
    // page it meant, instead of asking for a directory that was never
    // generated.
    let directory: bool =
        collapsed.ends_with(ROUTE_SEPARATOR) && !collapsed.ends_with(HTML_WITH_SEPARATOR);
    let segments: &str = collapsed
        .trim_end_matches(ROUTE_SEPARATOR)
        .trim_start_matches(ROUTE_SEPARATOR);
    if segments.is_empty() {
        return String::from(ROUTE_ROOT);
    }
    let mut out: String = String::with_capacity(segments.len() + 2);
    out.push(ROUTE_SEPARATOR);
    out.push_str(segments);
    if directory {
        out.push(ROUTE_SEPARATOR);
    }
    out
}

/// Returns the route with the `.html` leaf suffix removed, or the route
/// unchanged when it does not carry the suffix.
///
/// The locale prefix is never touched: `route_stem("/zh/a/b.html")` is
/// `/zh/a/b`, so the caller can re-append the suffix when it looks the
/// page back up.
///
/// # Arguments
///
/// - `&str` - The route to strip.
///
/// # Returns
///
/// - `String` - The route without its `.html` suffix.
pub fn route_stem(route: &str) -> String {
    match route.strip_suffix(ROUTE_HTML_SUFFIX) {
        Some(stem) if !stem.is_empty() => stem.to_string(),
        _ => route.to_string(),
    }
}

/// Returns the candidate spellings a request may stand for, in
/// descending priority.
///
/// This is the single source of truth for route spelling:
/// [`RouteSite::find_page`] walks it, so the ambiguity rule — prefer the
/// exact route `build.rs` emitted, never merge two distinct pages — is
/// expressed once instead of at every call site.
///
/// Every candidate is the same *path* written with the suffix question
/// answered differently: bare (`/zh/a/b`), as a directory
/// (`/zh/a/b/`), and as a file (`/zh/a/b.html`). That is the whole
/// vocabulary `build.rs` emits routes from — a markdown leaf becomes one
/// form, a `README.md` becomes another — so the three spellings cover
/// every way a link can name an existing page and nothing else.
///
/// It never merges two pages that both exist. The exact route is
/// consulted before any candidate is tried, so a site carrying both
/// `/a.html` and `/a/` resolves each to itself, and a request for a page
/// that does not exist at any spelling still returns nothing.
///
/// # Arguments
///
/// - `&str` - The normalised request path.
///
/// # Returns
///
/// - `Vec<String>` - The spellings to try, most specific first, each
///   present at most once.
pub fn route_candidates(path: &str) -> Vec<String> {
    let stem: String = route_stem(path);
    let body: &str = stem.trim_end_matches(ROUTE_SEPARATOR);
    let spellings: [String; ROUTE_CANDIDATE_COUNT] = [
        String::from(body),
        format!("{body}{ROUTE_SEPARATOR}"),
        format!("{body}{ROUTE_HTML_SUFFIX}"),
        String::from(path),
    ];
    let mut ordered: Vec<String> = Vec::with_capacity(ROUTE_CANDIDATE_CAPACITY);
    for spelling in spellings {
        if !ordered.contains(&spelling) {
            ordered.push(spelling);
        }
    }
    ordered
}

/// Enumerates the misspellings of a served route that a link author is
/// likely to produce, so the route audit can check each one against the
/// site.
///
/// The list is exhaustive over *shape* mistakes rather than over content,
/// and every entry is derived mechanically from `route`, so running it
/// over every generated page stays cheap and misses no shape:
///
/// - the route itself (the control);
/// - the `.html` suffix dropped (`/zh/a/b.html` → `/zh/a/b`);
/// - the suffix dropped and a separator kept (`/zh/a/b.html` →
///   `/zh/a/b/`);
/// - a separator appended to a leaf route (`/zh/a/b.html` →
///   `/zh/a/b.html/`), the trailing slash a link helper adds;
/// - the `.html` suffix doubled onto a leaf route (`/zh/a/b.html` →
///   `/zh/a/b.html.html`), the same mistake one level down;
/// - every separator doubled (`/zh/a/b.html` → `//zh//a//b.html`);
/// - a separator prepended (`//zh/a/b.html`).
///
/// Every one of these names the same page as the control, so
/// [`RouteSite::find_page`] is expected to resolve all of them. The
/// locale prefix is deliberately not varied here: dropping it is a
/// question about *which page exists*, not about spelling, and
/// [`route_without_locale_prefix`] is the probe for it.
///
/// # Arguments
///
/// - `&str` - A route the site serves.
///
/// # Returns
///
/// - `Vec<String>` - The variants to feed through
///   [`RouteSite::find_page`], in a stable order, each present once.
pub fn route_variants(route: &str) -> Vec<String> {
    let stem: String = route_stem(route);
    let mut out: Vec<String> = Vec::with_capacity(ROUTE_VARIANT_CAPACITY);
    out.push(String::from(route));
    if stem != route {
        out.push(stem.clone());
        out.push(format!("{stem}{ROUTE_SEPARATOR}"));
    }
    out.push(format!("{route}{ROUTE_SEPARATOR}"));
    out.push(format!("{route}{ROUTE_HTML_SUFFIX}"));
    out.push(route.replace(ROUTE_SEPARATOR, ROUTE_SEPARATOR_PAIR));
    out.push(format!("{ROUTE_SEPARATOR}{route}"));
    let mut ordered: Vec<String> = Vec::with_capacity(ROUTE_VARIANT_CAPACITY);
    for variant in out {
        if !ordered.contains(&variant) && variant != ROUTE_ROOT {
            ordered.push(variant);
        }
    }
    ordered
}

/// Returns the route with its locale prefix removed.
///
/// Locale prefixes always end in a separator (`/zh/`), so matching one
/// is inherently a segment-boundary match: `/zh/guide/a` yields
/// `/guide/a` while `/zh-legacy/guide/a` is returned unchanged. That is
/// the same boundary rule [`RouteSite::locale_of`] relies on when
/// deciding which locale owns a route, kept in one place so the two
/// cannot disagree.
///
/// Whether the result resolves is a question about the site's contents,
/// not about spelling: the root locale may well serve the page, and when
/// it does not, the link is wrong rather than the router.
///
/// # Arguments
///
/// - `&str` - A route the site serves.
/// - `&str` - The prefix of the locale that owns it.
///
/// # Returns
///
/// - `String` - The route as it would be written without the prefix, or
///   `route` unchanged when it does not carry `locale_prefix`.
pub fn route_without_locale_prefix(route: &str, locale_prefix: &str) -> String {
    if locale_prefix == ROUTE_ROOT {
        return String::from(route);
    }
    match route.strip_prefix(locale_prefix) {
        Some(rest) if !rest.is_empty() => format!("{ROUTE_SEPARATOR}{rest}"),
        _ => String::from(route),
    }
}

/// Returns the root route every request collapses onto when it carries
/// no path at all.
///
/// Exposed as a function because a `pub const` is never public in this
/// project, and a test comparing routes needs the root spelled the same
/// way the normaliser spells it.
///
/// # Returns
///
/// - `&'static str` - The root route.
pub fn route_root() -> &'static str {
    ROUTE_ROOT
}

/// Returns how many spelling variants the audit enumerates per route.
///
/// # Returns
///
/// - `usize` - The variant count, or zero when the route has no
///   distinct spelling to try.
pub fn route_variant_capacity() -> usize {
    ROUTE_VARIANT_CAPACITY
}
