use super::*;

thread_local! {
    /// Active ambient `BuildContext` managed automatically by `App::mount` and `view!`.
    pub(crate) static ACTIVE_BUILD_CONTEXT: RefCell<Option<BuildContext>> = const { RefCell::new(None) };
}
