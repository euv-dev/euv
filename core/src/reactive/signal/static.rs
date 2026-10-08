use super::*;

thread_local! {
    /// Global typed signal slab for this thread. Lives for the thread's
    /// lifetime.
    ///
    /// The slab is append-only: slots are never recycled, so a stale `Signal<T>`
    /// handle always resolves to its original (deactivated) slot.
    ///
    /// Storage is thread-local rather than the previous `static mut` +
    /// `UnsafeCell`. The old form handed out `&'static mut SignalSlab` from a
    /// global that was never actually `Sync` — the slab holds `Box<dyn FnMut()>`
    /// listeners, so it is `!Send` — which is aliasing undefined behaviour the
    /// moment two threads touch it. Parallel test runs pushed into the same
    /// `Vec<Box<dyn AnySignalInner>>` concurrently, corrupting the heap and
    /// killing the test binary with a silent SIGSEGV or a poisoned `LazyLock`.
    ///
    /// Thread-local storage is the sound choice for a second reason: a `Signal`
    /// handle is just a slot index, and an index is only meaningful inside the
    /// slab that issued it. Thread A's slot 3 and thread B's slot 3 are
    /// different signals, so a process-wide slab handed each thread a view of
    /// the other's signals. Per-thread slabs make a handle unambiguous by
    /// construction, and `RefCell` restores the aliasing check that the old
    /// `unsafe` bypassed.

    pub(crate) static SIGNAL_SLAB: RefCell<SignalSlab> = RefCell::new(SignalSlab::new());
}
