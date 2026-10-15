/// The dark theme name used by euv-ui theme hooks.
pub(crate) const THEME_DARK: &str = "dark";
/// Selector of the scrollable sidebar container (the desktop nav column and
/// the mobile drawer both use `c_nav_items_scroll`).
pub(crate) const SIDEBAR_SCROLL_SELECTOR: &str = ".c_nav_items_scroll";
/// Selector matching the active sidebar entry (leaf link or group title).
///
/// The `_flush` and `_root_active` variants are listed too: a nested active
/// leaf and a first-level active group are the same "you are here" state as
/// the plain active class, and the scroll-into-view pass has to find all of
/// them or it leaves the current page off-screen.
pub(crate) const SIDEBAR_ACTIVE_SELECTOR: &str = ".c_euv_sidebar_link_active, \
     .c_euv_sidebar_link_active_flush, .c_euv_sidebar_group_title_active, \
     .c_euv_sidebar_group_title_root_active";
/// Breathing room left between the active entry and the container edge it
/// was scrolled to reveal, in CSS pixels.
///
/// Zero would park the entry flush against the edge, which reads as clipped
/// rather than revealed; a large value reintroduces the drift the minimum
/// move was meant to remove.
pub(crate) const SIDEBAR_SCROLL_GAP: f64 = 12.0;
/// Selector of a locale dropdown root (trigger plus its menu), so an
/// outside-click can tell an inside click from an outside one.
pub(crate) const LOCALE_DROPDOWN_SELECTOR: &str = ".c_euv_dropdown";
/// The document event an outside click is delivered on.
///
/// `pointerdown` rather than `click`: `click` fires only after the browser
/// has finished deciding what the press means, so a press that starts
/// outside and ends on a row (or is cancelled by a drag) still closes the
/// menu. Capturing the press is what makes "click away to dismiss" feel
/// immediate rather than lagging a frame or two behind the finger.
pub(crate) const DOCUMENT_OUTSIDE_POINTER_EVENT: &str = "pointerdown";
