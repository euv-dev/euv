use super::*;

thread_local! {
    /// Active ambient `Context` managed automatically by `App::mount`, `with_context`, and `view!`.
    pub(crate) static ACTIVE_CONTEXT: RefCell<Option<Context>> = const { RefCell::new(None) };
}
