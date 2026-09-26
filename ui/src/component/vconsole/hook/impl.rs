use super::*;

/// Implements the Console struct providing web console API methods.
///
/// Each method outputs to both the browser developer console and the
/// vConsole panel signal, with appropriate log level classification.
/// Methods are associated functions that internally access the global
/// Console instance, so callers never need to hold a reference.
impl Console {
    /// Initializes the global Console log signal.
    ///
    /// Must be called once during application startup before any `Console::log`,
    /// `Console::warn`, `Console::error`, or `Console::push` calls.
    ///
    /// OPT-23: also populates the shared `RefCell` backing store that
    /// `Console::push` uses for in-place appends. The `RefCell` and the
    /// `Signal` always share the same `Vec` snapshot — every `push`
    /// writes to the `RefCell` first, then re-broadcasts via the signal.
    pub fn init() {
        let logs_ref: Rc<RefCell<Vec<ConsoleEntry>>> = Rc::new(RefCell::new(Vec::new()));
        install_console_log_ref(logs_ref);
        let signal: Signal<Vec<ConsoleEntry>> = Signal::create(Vec::new());
        CONSOLE_LOG_SIGNAL.set(signal);
    }

    /// Logs an informational message (equivalent to console.log).
    ///
    /// The vConsole panel entry is appended only when `Console::init` has
    /// been called; the browser console output always happens.
    ///
    /// # Arguments
    ///
    /// - `M: AsRef<str>` - The message to log.
    pub fn log<M>(message: M)
    where
        M: AsRef<str>,
    {
        let message_ref: &str = message.as_ref();
        console::log_1(&message_ref.into());
        Self::append_entry(ConsoleEntry::new(LogLevel::Log, message_ref.to_string()));
    }

    /// Logs a warning message (equivalent to console.warn).
    ///
    /// The vConsole panel entry is appended only when `Console::init` has
    /// been called; the browser console output always happens.
    ///
    /// # Arguments
    ///
    /// - `M: AsRef<str>` - The warning message to log.
    pub fn warn<M>(message: M)
    where
        M: AsRef<str>,
    {
        let message_ref: &str = message.as_ref();
        console::warn_1(&message_ref.into());
        Self::append_entry(ConsoleEntry::new(LogLevel::Warn, message_ref.to_string()));
    }

    /// Logs an error message (equivalent to console.error).
    ///
    /// The vConsole panel entry is appended only when `Console::init` has
    /// been called; the browser console output always happens.
    ///
    /// # Arguments
    ///
    /// - `M: AsRef<str>` - The error message to log.
    pub fn error<M>(message: M)
    where
        M: AsRef<str>,
    {
        let message_ref: &str = message.as_ref();
        console::error_1(&message_ref.into());
        Self::append_entry(ConsoleEntry::new(LogLevel::Error, message_ref.to_string()));
    }

    /// Clears all log entries from the vConsole panel signal.
    ///
    /// No-op when `Console::init` has not been called yet.
    ///
    /// OPT-23: when the shared `RefCell` backing store is installed,
    /// clears it in place before re-broadcasting an empty vec, so the
    /// two storage sites stay in sync.
    pub fn clear() {
        if let Some(logs_ref) = console_log_ref() {
            logs_ref.borrow_mut().clear();
        }
        let Some(log) = Self::get_signal() else {
            return;
        };
        log.set(Vec::new());
    }

    /// Returns the global vConsole log signal, if initialized.
    ///
    /// # Returns
    ///
    /// - `Option<Signal<Vec<ConsoleEntry>>>` - The console log signal, or
    ///   `None` when `Console::init` has not been called yet.
    pub(crate) fn get_signal() -> Option<Signal<Vec<ConsoleEntry>>> {
        CONSOLE_LOG_SIGNAL.loaded()
    }

    /// Creates a click event handler that opens the vConsole fab panel.
    ///
    /// Pushes an overlay state and sets the panel visibility signal to true.
    ///
    /// # Arguments
    ///
    /// - `Signal<bool>` - The signal controlling panel visibility.
    ///
    /// # Returns
    ///
    /// - `Option<Rc<dyn Fn(Event)>>` - A click handler that opens the panel.
    pub(crate) fn fab_on_click(panel_open: Signal<bool>) -> Option<Rc<dyn Fn(Event)>> {
        Some(Rc::new(move |_: Event| {
            let closer: Rc<dyn Fn()> = Rc::new(move || {
                panel_open.set(false);
            });
            Router::overlay_stack_push(closer);
            panel_open.set(true);
        }))
    }

    /// Appends an entry to the vConsole log signal, trimming if over capacity.
    ///
    /// No-op when `Console::init` has not been called yet.
    ///
    /// OPT-23: when the shared `RefCell` backing store is available
    /// (the common case after `Console::init`), this appends in place
    /// to the `RefCell` and re-broadcasts via the signal in a single
    /// `set` call. The previous signal-only path had to clone the
    /// entire log vec via `Signal::get` before pushing — the new path
    /// mutates in place and clones only the snapshot it forwards to
    /// `set`.
    ///
    /// # Arguments
    ///
    /// - `ConsoleEntry` - The console entry to append.
    fn append_entry(entry: ConsoleEntry) {
        if let Some(logs_ref) = console_log_ref() {
            let mut logs: std::cell::RefMut<'_, Vec<ConsoleEntry>> = logs_ref.borrow_mut();
            logs.push(entry);
            if logs.len() > MAX_CONSOLE_LOG_ENTRIES {
                let excess: usize = logs.len() - MAX_CONSOLE_LOG_ENTRIES;
                logs.drain(0..excess);
            }
            let snapshot: Vec<ConsoleEntry> = logs.clone();
            drop(logs);
            Self::replace_signal(snapshot);
            return;
        }
        let Some(log) = Self::get_signal() else {
            return;
        };
        let mut current: Vec<ConsoleEntry> = log.get();
        current.push(entry);
        if current.len() > MAX_CONSOLE_LOG_ENTRIES {
            let excess: usize = current.len() - MAX_CONSOLE_LOG_ENTRIES;
            current.drain(0..excess);
        }
        log.set(current);
    }

    /// OPT-23: append-only mutation API for the vConsole log signal.
    ///
    /// Public escape hatch for callers (and tests) that want to push
    /// a `ConsoleEntry` without going through the `log`/`warn`/`error`
    /// helpers. Uses the shared `RefCell` backing store for an
    /// in-place append, then re-broadcasts via the signal so existing
    /// reactive subscribers re-render.
    ///
    /// Falls back to the signal-only path when `Console::init` has
    /// not yet installed the shared `RefCell`.
    ///
    /// # Arguments
    ///
    /// - `ConsoleEntry` - The console entry to append.
    pub fn push(entry: ConsoleEntry) {
        Self::append_entry(entry);
    }

    /// OPT-23: replaces the public log signal value with the given
    /// snapshot. Used by `append_entry` after mutating the shared
    /// `RefCell`, so subscribers receive the latest snapshot without
    /// the `RefCell` borrow aliasing the signal listener registry.
    fn replace_signal(next: Vec<ConsoleEntry>) {
        if let Some(log) = Self::get_signal() {
            log.set(next);
        }
    }
}

/// Implements the Display trait for LogFilter to render filter button labels.
impl Display for LogFilter {
    /// Formats the [`LogFilter`] via the supplied formatter.
    ///
    /// # Arguments
    ///
    /// - `&mut Formatter<'_>` - The formatter receiving the formatted output.
    ///
    /// # Returns
    ///
    /// - `fmt::Result` - Result of the formatting operation.
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        let label: &str = match self {
            LogFilter::All => "All",
            LogFilter::Log => "Log",
            LogFilter::Warn => "Warn",
            LogFilter::Error => "Error",
        };
        write!(formatter, "{}", label)
    }
}

/// Implementation of log level badge rendering.
impl LogLevel {
    /// Returns the short badge label for a log level.
    ///
    /// # Returns
    ///
    /// - `&str` - The badge label string ("LOG", "WRN", "ERR").
    pub(crate) fn badge(self) -> &'static str {
        match self {
            LogLevel::Log => "LOG",
            LogLevel::Warn => "WRN",
            LogLevel::Error => "ERR",
        }
    }
}

/// Implementation of log filter event handlers.
impl LogFilter {
    /// Creates a click event handler that sets the log filter to "All".
    ///
    /// # Arguments
    ///
    /// - `Signal<LogFilter>` - The signal controlling the active log filter.
    ///
    /// # Returns
    ///
    /// - `Option<Rc<dyn Fn(Event)>>` - A click handler that sets filter to All.
    pub(crate) fn on_filter_all(filter_signal: Signal<LogFilter>) -> Option<Rc<dyn Fn(Event)>> {
        Some(Rc::new(move |_: Event| {
            filter_signal.set(LogFilter::All);
        }))
    }

    /// Creates a click event handler that sets the log filter to "Log".
    ///
    /// # Arguments
    ///
    /// - `Signal<LogFilter>` - The signal controlling the active log filter.
    ///
    /// # Returns
    ///
    /// - `Option<Rc<dyn Fn(Event)>>` - A click handler that sets filter to Log.
    pub(crate) fn on_filter_log(filter_signal: Signal<LogFilter>) -> Option<Rc<dyn Fn(Event)>> {
        Some(Rc::new(move |_: Event| {
            filter_signal.set(LogFilter::Log);
        }))
    }

    /// Creates a click event handler that sets the log filter to "Warn".
    ///
    /// # Arguments
    ///
    /// - `Signal<LogFilter>` - The signal controlling the active log filter.
    ///
    /// # Returns
    ///
    /// - `Option<Rc<dyn Fn(Event)>>` - A click handler that sets filter to Warn.
    pub(crate) fn on_filter_warn(filter_signal: Signal<LogFilter>) -> Option<Rc<dyn Fn(Event)>> {
        Some(Rc::new(move |_: Event| {
            filter_signal.set(LogFilter::Warn);
        }))
    }

    /// Creates a click event handler that sets the log filter to "Error".
    ///
    /// # Arguments
    ///
    /// - `Signal<LogFilter>` - The signal controlling the active log filter.
    ///
    /// # Returns
    ///
    /// - `Option<Rc<dyn Fn(Event)>>` - A click handler that sets filter to Error.
    pub(crate) fn on_filter_error(filter_signal: Signal<LogFilter>) -> Option<Rc<dyn Fn(Event)>> {
        Some(Rc::new(move |_: Event| {
            filter_signal.set(LogFilter::Error);
        }))
    }
}
