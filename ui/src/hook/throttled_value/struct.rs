use super::*;

/// A value that emits the most recent input at most once
/// per `interval_ms`.
///
/// Constructed via `ThrottledValue::new(interval_ms)`
/// (Lombok `New`); the emitted value starts at
/// `T::default()` and the throttle state starts at
/// `Idle`. Use [`ThrottledValue::set`] to seed the
/// emitted value.
///
/// Unlike [`DebouncedValue`], which waits for a quiet
/// period, a throttled value commits a snapshot every
/// `interval_ms` regardless of how often `set` was called.
///
/// Pair with `App::use_interval` — the interval callback
/// calls `tick(now_ms)` every `interval_ms`, where
/// `now_ms` comes from `performance.now()` on the web.
/// The caller picks the time source (plain `u64` millis)
/// so the hook stays free of browser / timer dependencies
/// and works on `wasm32-unknown-unknown` (where
/// `std::time::Instant::now()` panics).
#[derive(Clone, Data, Debug, New)]
pub struct ThrottledValue<T: Clone + PartialEq + Default + 'static> {
    /// The emitted value signal. Defaults to
    /// `Signal::create(T::default())` via
    /// `#[new(skip)]`.
    ///
    /// Public because `example/src/page/hooks_timing/view/fn.rs`
    /// reads it through `get_value()`.
    #[new(skip)]
    #[get(type(copy))]
    pub value: Signal<T>,
    /// The latest input waiting for the next commit.
    /// Defaults to `Signal::create(None)` via
    /// `#[new(skip)]`.
    ///
    /// `get_pending()` is only called by `impl.rs`, so the
    /// three generated accessors are all `pub(crate)`.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    pub(crate) pending: Signal<Option<T>>,
    /// The internal idle/cooldown state. Defaults to
    /// `Signal::create(ThrottleState::Idle)` via
    /// `#[new(skip)]`.
    ///
    /// `get_state()` is only called by `impl.rs`, so the three
    /// generated accessors are all `pub(crate)`.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    pub(crate) state: Signal<ThrottleState>,
    /// The throttle window in milliseconds.
    ///
    /// `get_interval_ms()` is only called by `impl.rs`, so the
    /// three generated accessors are all `pub(crate)`.
    #[get(type(copy), pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) interval_ms: u32,
}

/// `ThrottledValue<T>` is `Copy` when `T` is — every field
/// is itself `Copy` (`Signal<T>`, `Signal<Option<T>>`, `u32`)
/// or a simple `enum`, so the blanket impl is sound.
impl<T> Copy for ThrottledValue<T> where T: Clone + PartialEq + Default + 'static {}
