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
