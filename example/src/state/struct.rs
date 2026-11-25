use super::*;

/// Reactive application state powering every page of the cross-platform showcase.
#[derive(Clone, Getter)]
pub struct ShowcaseState {
    /// Active route path signal.
    #[get(type(copy))]
    pub(crate) route: Signal<String>,
    /// Interactive counter signal for fine-grained reactivity demos.
    #[get(type(copy))]
    pub(crate) counter: Signal<i32>,
    /// Derived memo computing `counter * 2` without redundant notifications.
    #[get(type(copy))]
    pub(crate) doubled: Memo<i32>,
    /// Active render mode selector index (`0 = Auto`, `1 = Software`, `2 = Hardware`, `3 = Hybrid`).
    #[get(type(copy))]
    pub(crate) render_mode_index: Signal<usize>,
    /// Active reactive theme state across all 17 design schemes and light/dark modes.
    pub(crate) theme: UiThemeState,
    /// Controlled form text input value.
    #[get(type(copy))]
    pub(crate) username: Signal<String>,
    /// Controlled toggle switch for hardware acceleration preference.
    #[get(type(copy))]
    pub(crate) hardware_accel: Signal<bool>,
    /// Controlled checkbox for terms acknowledgment.
    #[get(type(copy))]
    pub(crate) agree_terms: Signal<bool>,
    /// Controlled modal visibility flag.
    #[get(type(copy))]
    pub(crate) modal_open: Signal<bool>,
    /// Controlled toast banner visibility flag.
    #[get(type(copy))]
    pub(crate) toast_visible: Signal<bool>,
    /// Controlled tab bar index.
    #[get(type(copy))]
    pub(crate) active_tab: Signal<usize>,
    /// Controlled fullscreen window toggle state.
    #[get(type(copy))]
    pub(crate) fullscreen: Signal<bool>,
}
