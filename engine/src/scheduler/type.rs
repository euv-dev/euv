use super::*;

/// A reference-counted shared-mutable container for a `dyn TickHandler`.
///
/// Holds a `dyn TickHandler` behind `EngineCell` (an `UnsafeCell`-backed
/// `Sync` newtype), so the multi-owner shared-mut pattern that
/// `Rc<RefCell<T>>` used to express is now expressed without `RefCell` -
/// matching the rest of the engine's `static mut` convention.
///
/// # Access
///
/// Use [`EngineCell::get_mut`] for unconditional mutable access.
pub type TickHandlerRc = Rc<EngineCell<dyn TickHandler>>;

/// A reference-counted `requestAnimationFrame` closure cell.
///
/// Holds an `Option<Closure<dyn FnMut()>>` behind `MaybeEngineCell` so
/// the cell may legitimately be empty (no closure installed) before
/// `spawn` is called and after cleanup tears it down. Mirrors
/// `core::reactive::schedule::static::CURRENT_HOOK_CONTEXT` in shape.
///
/// # Access
///
/// Use [`MaybeEngineCell::try_get_mut`] for absent-or-present read,
/// [`MaybeEngineCell::try_set`] to install, [`MaybeEngineCell::try_take`]
/// to remove.
pub type RafClosureCell = Rc<MaybeEngineCell<Closure<dyn FnMut()>>>;

/// A reference-counted shared-mutable [`TaskRegistry`].
///
/// The registry is owned by the engine handle but must also be reachable
/// from the `requestAnimationFrame` closure, which captures a clone and
/// drives `update_all` on every fixed step. An `Rc<EngineCell<..>>` lets
/// both owners mutate the same task list without `RefCell` borrow checks,
/// matching `TickHandlerRc`.
pub type TaskRegistryRc = Rc<EngineCell<TaskRegistry>>;
