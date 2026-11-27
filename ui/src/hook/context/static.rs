use super::*;

thread_local! {
    pub(crate) static ACTIVE_THEME_STATE: RefCell<Option<UiThemeState>> = const { RefCell::new(None) };
}
