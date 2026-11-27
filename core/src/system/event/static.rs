use super::*;

thread_local! {
    /// Thread-local hub storing scoped `SystemEvent` hook subscriptions.
    pub(crate) static SYSTEM_EVENT_HUB: RefCell<SystemEventHub> = RefCell::new(SystemEventHub::default());
}
