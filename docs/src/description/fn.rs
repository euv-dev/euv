use super::*;

/// Returns the reference description of a page, addressed by its slug.
///
/// A slug that is not in the table has no description: the line is then
/// omitted rather than filled with an invented sentence. The reference
/// site only prints a description on the pages it has one for, so the
/// omission is the faithful behaviour.
///
/// # Arguments
///
/// - `&str` - The page slug.
///
/// # Returns
///
/// - `Option<&'static str>` - The description, or `None` when the slug
///   has no entry.
pub(crate) fn description_for_slug(slug: &str) -> Option<&'static str> {
    for (key, description) in REF_DESCRIPTIONS {
        if *key == slug {
            return Some(*description);
        }
    }
    None
}

/// Derives the slug of a route: its last path segment, without the
/// `.html` suffix and without the trailing slash.
///
/// The slug is what the reference table is keyed by, so it has to be
/// the *page* name and not the whole route: `/guide/getting-started.html`
/// is the `getting-started` page, and the site root has no slug at all.
///
/// # Arguments
///
/// - `&str` - The page route.
///
/// # Returns
///
/// - `&str` - The slug, empty for the site root.
pub(crate) fn route_slug(route: &str) -> &str {
    let trimmed: &str = route.trim_end_matches('/');
    let segment: &str = trimmed.rsplit('/').next().unwrap_or("");
    segment.strip_suffix(".html").unwrap_or(segment)
}

/// Returns the description of the page a route points at.
///
/// # Arguments
///
/// - `&str` - The page route.
///
/// # Returns
///
/// - `Option<&'static str>` - The description, or `None` when the page
///   has no entry in the reference table.
pub(crate) fn description_for_route(route: &str) -> Option<&'static str> {
    description_for_slug(route_slug(route))
}
