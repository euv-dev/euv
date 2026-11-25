use super::*;

thread_local! {
    /// Active ambient `BuildContext` managed automatically by `App::mount`, `view!`, and `ui!`.
    pub(crate) static ACTIVE_BUILD_CONTEXT: RefCell<Option<BuildContext>> = const { RefCell::new(None) };
}
