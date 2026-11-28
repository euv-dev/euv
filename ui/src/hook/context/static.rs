use super::*;

thread_local! {
    pub(crate) static ACTIVE_THEME_STATE: RefCell<Option<UiThemeState>> = const { RefCell::new(None) };
    pub(crate) static ACTIVE_THEME_GEN: RefCell<Option<Signal<u64>>> = const { RefCell::new(None) };
    pub(crate) static FULLSCREEN_ACTIVE_SIG: RefCell<Option<Signal<bool>>> = const { RefCell::new(None) };
    pub(crate) static FULLSCREEN_DEPTH: Cell<usize> = const { Cell::new(0) };
    pub(crate) static FULLSCREEN_CTX_HANDLE: RefCell<Option<Context>> = const { RefCell::new(None) };
}
