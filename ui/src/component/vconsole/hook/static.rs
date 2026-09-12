use super::*;

thread_local! {
    /// OPT-23: shared mutable storage backing the vConsole log signal.
    ///
    /// `Console::push` appends to this `RefCell` in place, then re-broadcasts
    /// the snapshot via the public `Signal` so existing reactive subscribers
    /// re-render. The `RefCell` is `thread_local` so it sidesteps the
    /// `Rc` / `RefCell` `Sync` requirement that a plain `OnceLock` would
    /// hit (we are single-threaded WASM, but the Rust checker doesn't know).
    pub(crate) static CONSOLE_LOG_REF: RefCell<Option<Rc<RefCell<Vec<ConsoleEntry>>>>>
        = const { RefCell::new(None) };
}

/// Global storage for the Console log signal.
///
/// Initialized via `init_console` and accessed through `get_console_signal`.

pub(crate) static CONSOLE_LOG_SIGNAL: SignalCell<Vec<ConsoleEntry>> = SignalCell::none();
