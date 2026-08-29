use super::*;

/// A value that only emits after a quiet period of
/// `delay` since its most recent `set`.
///
/// Constructed via `DebouncedValue::new(delay_ms)`
/// (Lombok `New`); the emitted value starts at
/// `T::default()` and the throttle state starts at
/// `Idle`. Use [`DebouncedValue::set`] (or
/// [`DebouncedValue::tick`] with a backdated timestamp)
/// to seed the emitted value.
///
/// Typical use: pair with `App::use_interval` — the
/// interval callback calls `tick(now_ms)` every
/// N milliseconds, where `now_ms` comes from
/// `performance.now()` on the web. After `delay_ms`
/// without a fresh `set`, the pending value is committed.
///
/// This shape keeps the hook free of any browser /
/// timer dependency so the same code runs in
/// `cargo test` and in `wasm32-unknown-unknown` — the
/// caller supplies the time source as plain milliseconds
/// (`std::time::Instant::now()` panics on wasm, so the
/// hook API deliberately takes `u64` millis).
#[derive(Clone, Data, Debug, New)]
pub struct DebouncedValue<T: Clone + PartialEq + Default + 'static> {
    /// The emitted value signal. Defaults to
    /// `Signal::create(T::default())` via
    /// `#[new(skip)]`.
    #[new(skip)]
    #[get(type(copy))]
    pub(crate) value: Signal<T>,
    /// The internal pending/empty state. Defaults to
    /// `Signal::create(DebounceState::Idle)` via
    /// `#[new(skip)]`.
    #[new(skip)]
    pub(crate) state: Signal<DebounceState<T>>,
    /// The quiet period in milliseconds.
    pub(crate) delay_ms: u32,
}

/// `DebouncedValue<T>` is `Copy` when `T` is — every field
/// (`Signal<T>`, `Signal<DebounceState<T>>`, `u32`) is itself
/// `Copy`, so the blanket impl is sound.
impl<T> Copy for DebouncedValue<T> where T: Clone + PartialEq + Default + 'static {}
