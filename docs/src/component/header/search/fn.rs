use super::*;

/// Matches only public page metadata, preserving title-first source order.
///
/// # Arguments
///
/// - `&[DocsPage]` - Pages compiled into the current locale bundle.
/// - `&str` - User query, trimmed and Unicode lowercased here.
///
/// # Returns
///
/// - `Vec<DocsPage>` - Deduplicated, bounded previews without inspecting content.
pub(crate) fn header_search_matches(pages: &[DocsPage], query: &str) -> Vec<DocsPage> {
    let needle: String = query.trim().to_lowercase();
    let mut matches: Vec<DocsPage> = Vec::new();
    if needle.is_empty() {
        return matches;
    }
    for title_pass in [true, false] {
        for page in pages {
            let title_match: bool = page.title.to_lowercase().contains(&needle);
            let matched: bool = if title_pass {
                title_match
            } else {
                !title_match && page.route.to_lowercase().contains(&needle)
            };
            if matched
                && !matches
                    .iter()
                    .any(|found: &DocsPage| found.route == page.route)
            {
                matches.push(*page);
                if matches.len() == HEADER_SEARCH_LIMIT {
                    return matches;
                }
            }
        }
    }
    matches
}

/// Opens previews and resets an earlier keyboard selection.
///
/// # Arguments
///
/// - `Signal<HeaderSearchState>` - Interaction state owned by the search hook.
pub(crate) fn header_search_focus(state: Signal<HeaderSearchState>) {
    let mut snapshot: HeaderSearchState = state.get();
    snapshot.focus();
    state.set(snapshot);
}

/// Commits an explicitly chosen preview route.
///
/// # Arguments
///
/// - `Signal<String>` - Current documentation route.
/// - `Signal<String>` - Query cleared when committing the selection.
/// - `Signal<HeaderSearchState>` - Search interaction state.
/// - `&'static str` - Chosen compiled-page route.
pub(crate) fn header_search_select(
    route_signal: Signal<String>,
    query: Signal<String>,
    state: Signal<HeaderSearchState>,
    route: &'static str,
) {
    state.set(HeaderSearchState::default());
    query.set(String::new());
    route_signal.set(route.to_string());
    Router::navigate(route);
}
