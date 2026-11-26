use super::*;

/// Runs `operation` with `ctx` bound as the active ambient `BuildContext`, restoring any previous context on exit.
///
/// # Arguments
///
/// - `BuildContext` - Build context to bind during `operation`.
/// - `F` - Closure executing declarative UI construction (`view!` or components).
///
/// # Returns
///
/// - `R` - Value returned by `operation`.
pub fn with_build_context<F, R>(ctx: BuildContext, operation: F) -> R
where
    F: FnOnce() -> R,
{
    let previous: Option<BuildContext> = ACTIVE_BUILD_CONTEXT
        .with(|slot: &RefCell<Option<BuildContext>>| slot.borrow_mut().replace(ctx));
    let _guard: BuildContextGuard = BuildContextGuard::new(previous);
    operation()
}

/// Returns a cloned handle to the active ambient `BuildContext`.
///
/// # Returns
///
/// - `BuildContext` - Currently bound ambient build context.
#[must_use]
pub fn current_build_context() -> BuildContext {
    ACTIVE_BUILD_CONTEXT.with(|slot: &RefCell<Option<BuildContext>>| {
        slot.borrow()
            .clone()
            .unwrap_or_else(|| BuildContext::new(SceneHandle::new_tree(), PlatformKind::Headless))
    })
}

/// Returns `true` when an ambient `BuildContext` is currently bound on this thread.
///
/// # Returns
///
/// - `bool` - Whether `with_build_context` or `App::mount` is active.
#[must_use]
pub fn has_build_context() -> bool {
    ACTIVE_BUILD_CONTEXT.with(|slot: &RefCell<Option<BuildContext>>| slot.borrow().is_some())
}

/// Returns the active `PlatformKind` from the ambient `BuildContext`.
///
/// # Returns
///
/// - `PlatformKind` - Execution platform kind.
#[must_use]
pub fn current_platform() -> PlatformKind {
    current_build_context().get_platform()
}

/// Mounts any `Mountable` component or node into the active ambient `BuildContext`.
///
/// # Arguments
///
/// - `M` - Mountable UI component or node handle.
///
/// # Returns
///
/// - `NodeId` - Mounted root scene node handle.
pub fn mount_current<M>(mountable: M) -> NodeId
where
    M: Mountable,
{
    let mut ctx: BuildContext = current_build_context();
    mountable.mount(&mut ctx)
}
