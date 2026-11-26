use super::*;

/// Runs `operation` with `ctx` bound as the active ambient `Context`, restoring any previous context on exit.
///
/// # Arguments
///
/// - `Context` - Framework context to bind during `operation`.
/// - `F` - Closure executing declarative UI construction (`view!` or components).
///
/// # Returns
///
/// - `R` - Value returned by `operation`.
pub fn with_context<F, R>(ctx: Context, operation: F) -> R
where
    F: FnOnce() -> R,
{
    let previous: Option<Context> =
        ACTIVE_CONTEXT.with(|slot: &RefCell<Option<Context>>| slot.borrow_mut().replace(ctx));
    let _guard: ContextGuard = ContextGuard::new(previous);
    operation()
}

/// Returns a cloned handle to the active ambient `Context`.
///
/// # Returns
///
/// - `Context` - Currently bound ambient framework context.
#[must_use]
pub fn current_context() -> Context {
    ACTIVE_CONTEXT.with(|slot: &RefCell<Option<Context>>| {
        slot.borrow()
            .clone()
            .unwrap_or_else(|| Context::new(SceneHandle::new_tree(), PlatformKind::Headless))
    })
}

/// Returns `true` when an ambient `Context` is currently bound on this thread.
///
/// # Returns
///
/// - `bool` - Whether `with_context` or `App::mount` is active.
#[must_use]
pub fn has_context() -> bool {
    ACTIVE_CONTEXT.with(|slot: &RefCell<Option<Context>>| slot.borrow().is_some())
}

/// Returns the active `PlatformKind` from the ambient `Context`.
///
/// # Returns
///
/// - `PlatformKind` - Execution platform kind.
#[must_use]
pub fn current_platform() -> PlatformKind {
    current_context().get_platform()
}

/// Converts any string-like value into an owned `String` for declarative `view!` macro expansion.
///
/// # Arguments
///
/// - `T` - String-like text value implementing `Into<String>`.
///
/// # Returns
///
/// - `String` - Owned UTF-8 text string.
#[must_use]
pub fn into_text<T>(value: T) -> String
where
    T: Into<String>,
{
    value.into()
}
