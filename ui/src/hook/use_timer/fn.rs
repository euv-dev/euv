use super::*;

/// Obtains the [`UseTimer`] registered against the current hook context
/// slot.
///
/// Behaves like `HookContext::use_hook` - the same handle is returned on
/// every render at the same hook index, so the tick count survives
/// re-renders.
///
/// The interval is created once, on the first render, and is cleared
/// automatically when the component unmounts. A period of zero is
/// treated as one millisecond: a zero-delay interval is a busy loop
/// that would starve the renderer.
///
/// # Arguments
///
/// - `u32` - The period in milliseconds.
///
/// # Returns
///
/// - `UseTimer` - The handle, with `get_ticks()` returning the reactive
///   `Signal<u64>`.
pub fn use_timer(interval_ms: u32) -> UseTimer {
    let handle: UseTimer = HookContext::use_hook(|| UseTimer::new(interval_ms));
    // The interval is a browser timer. `App::use_interval` reaches for
    // `window` to register it, and `window` does not exist on a host
    // build — the call would abort the process rather than return. The
    // timer therefore only runs in a browser, and a host build gets a
    // handle whose tick count simply never moves.
    #[cfg(target_arch = "wasm32")]
    {
        let ticks: Signal<u64> = handle.get_ticks();
        App::use_interval(interval_ms.max(1) as i32, move || {
            let current: u64 = ticks.get();
            ticks.set(current + 1);
        });
    }
    handle
}

/// Schedules `callback` to run once, `delay_ms` milliseconds from now.
///
/// The callback is dropped if the component unmounts before the delay
/// elapses, so a timeout cannot fire into a component that is gone.
///
/// # Arguments
///
/// - `u32` - The delay in milliseconds.
/// - `F` - The callback to run.
pub fn use_timeout<F>(delay_ms: u32, callback: F)
where
    F: FnOnce() + 'static,
{
    let cancelled: Signal<bool> = App::use_signal(|| false);
    schedule_after(delay_ms, move || {
        if cancelled.get() {
            return;
        }
        callback();
    });
    App::use_cleanup(move || {
        cancelled.set(true);
    });
}
