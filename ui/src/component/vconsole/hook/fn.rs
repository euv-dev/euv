use super::*;

/// OPT-23: returns a clone of the shared `Rc<RefCell<Vec<ConsoleEntry>>>`
/// that backs the vConsole log signal, or `None` if `Console::init` has
/// not yet installed it.
///
/// # Returns
///
/// - `Option<Rc<RefCell<Vec<ConsoleEntry>>>>` - A clone of the shared log
///   store, or `None` before `Console::init` has installed it.
pub(crate) fn console_log_ref() -> Option<Rc<RefCell<Vec<ConsoleEntry>>>> {
    CONSOLE_LOG_REF
        .with(|cell: &RefCell<Option<Rc<RefCell<Vec<ConsoleEntry>>>>>| cell.borrow().clone())
}

/// OPT-23: installs the shared `Rc<RefCell<Vec<ConsoleEntry>>>` backing
/// store for the vConsole log signal. Called from `Console::init`.
///
/// # Arguments
///
/// - `Rc<RefCell<Vec<ConsoleEntry>>>` - The shared store to install; the
///   same handle is later returned by [`console_log_ref`].
pub(crate) fn install_console_log_ref(logs_ref: Rc<RefCell<Vec<ConsoleEntry>>>) {
    CONSOLE_LOG_REF.with(|cell: &RefCell<Option<Rc<RefCell<Vec<ConsoleEntry>>>>>| {
        *cell.borrow_mut() = Some(logs_ref);
    });
}
