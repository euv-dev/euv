use super::*;

/// Obtains the [`UseMounted`] registered against the current hook
/// context slot.
///
/// Behaves like `HookContext::use_hook` - the same handle is returned on
/// every render at the same hook index, so the flag survives re-renders.
///
/// The flag starts at `false` and is raised once the DOM has been
/// committed.
///
/// # Returns
///
/// - `UseMounted` - The handle, with `get_mounted()` returning the
///   reactive `Signal<bool>`.
pub fn use_mounted() -> UseMounted {
    let handle: UseMounted = HookContext::use_hook(UseMounted::new);
    // The second slot is the "has the flag been armed yet" marker. It
    // is what keeps `attach` from running on every render: the flag is
    // only raised once, after the first DOM commit.
    let armed: Signal<bool> = HookContext::use_hook(|| Signal::create(false));
    if !armed.get() {
        armed.set(true);
        handle.attach();
    }
    handle
}
