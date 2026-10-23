use super::*;

/// Obtains the [`UseMediaQuery`] registered against the current hook
/// context slot.
///
/// Behaves like `HookContext::use_hook` - the same handle is returned on
/// every render at the same hook index, so the media query listener is
/// attached once and the match result survives re-renders.
///
/// The match result is seeded with the browser's current answer when the
/// handle is created, and is then kept current by the query's `change`
/// event.
///
/// # Arguments
///
/// - `&'static str` - The media query text, for example
///   `"(max-width: 767px)"`.
///
/// # Returns
///
/// - `UseMediaQuery` - The handle, with `get_matches()` returning the
///   reactive `Signal<bool>`.
pub fn use_media_query(query: &'static str) -> UseMediaQuery {
    let handle: UseMediaQuery = HookContext::use_hook(|| {
        let handle: UseMediaQuery = UseMediaQuery::new(query);
        handle.attach();
        handle
    });
    App::use_cleanup({
        let handle: UseMediaQuery = handle.clone();
        move || handle.detach()
    });
    handle
}
