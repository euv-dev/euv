use super::*;

thread_local! {
    /// Thread-local reactive runtime ensuring lock-free single-threaded performance and test isolation.
    pub(crate) static REACTIVE_RUNTIME: RefCell<ReactiveRuntime> = RefCell::new(ReactiveRuntime::default());
}
