use super::*;

thread_local! {
    /// Thread-local Win32 event queue populated synchronously by `win32_wnd_proc`.
    pub(crate) static WIN32_EVENT_QUEUE: RefCell<Vec<Win32QueuedEvent>> = const { RefCell::new(Vec::new()) };
}
