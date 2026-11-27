use super::*;

thread_local! {
    pub(crate) static ACTIVE_THEME_STATE: RefCell<Option<UiThemeState>> = const { RefCell::new(None) };
    pub(crate) static ACTIVE_THEME_GEN: RefCell<Option<Signal<u64>>> = const { RefCell::new(None) };
}
