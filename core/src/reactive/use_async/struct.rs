// `super::*` is intentionally NOT imported here — this module
// defines its own `HasLoadingHint` trait (no dependency on the
// parent module) and the rest of the file uses fully-qualified
// `core::...` paths so the type stays self-contained.

/// Discriminant for the three states an `use_async` future can be in.
///
/// Re-exported as `UseAsyncState` via [`super`]. Kept as a separate
/// enum (rather than a `Result`-shaped type) so the `match` arms
/// produced by users can name the `Loading` case separately and
/// reach for `LoadingHint` (the previous "in flight" payload) when
/// available.
///
/// The default initial state is `Loading(LoadingHint::DEFAULT)`,
/// because until the first `.await` resolves there is no `T` to
/// report. Users that need a distinct pre-fetch state can supply
/// `LoadingHint` via `UseAsyncHandle::loading_hint()`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AsyncState<T, L = ()> {
    /// The future has not yet produced a value (initial state).
    Loading(L),
    /// The future completed successfully with `T`.
    Ok(T),
    /// The future rejected with an error of type `E`.
    ///
    /// We intentionally use a free-form `String` for the error
    /// payload so the same enum works whether the future's error
    /// type is `JsValue`, `String`, `serde_json::Value`, or a custom
    /// domain type — `use_async` normalises everything into
    /// `String::to_string` at the await boundary. Callers that need
    /// typed errors should `String::parse` or use a richer future
    /// combinator on top.
    Err(String),
}

/// Optional payload for the `Loading` state.
///
/// Most users will leave this at the default `()` — the variant
/// then carries no information beyond "we're fetching". Implement
/// this trait for your loading-hint type to attach stale-while-
/// revalidate metadata (last-known data, fetched-at timestamp, …)
/// to the `Loading` arm.
///
/// The trait has a single associated constant — a zero-sized marker
/// so the type can be used as a default type parameter — and one
/// method that produces the "no prior data, no hint" sentinel.
///
/// # Examples
///
/// ```ignore
/// #[derive(Clone)]
/// struct Hint { previous: Option<MyData>, last_fetched_ms: u64 }
///
/// impl HasLoadingHint for Hint {
///     fn empty() -> Self { Hint { previous: None, last_fetched_ms: 0 } }
/// }
/// ```
pub trait HasLoadingHint: Clone + 'static {
    /// The sentinel value representing "no prior data is available".
    /// `use_async` slots this in for the first render so that
    /// `.loading_hint()` always returns a usable value, even before
    /// the future has a chance to produce one.
    fn empty() -> Self;
}

impl HasLoadingHint for () {
    #[inline]
    fn empty() -> Self {}
}

/// `use_async`'s reactive handle, stored in the hook context slot and
/// returned to the user on each render.
///
/// The handle exposes three fields:
///
/// - `state`: the current `AsyncState<T, L>` (matches what the user
///   should `match` on in `html!`).
/// - `refetch`: triggers the future to run again, regardless of
///   whether the previous attempt completed or is still in flight.
/// - `cancel`: drops the in-flight future (if any) and prevents its
///   `Ok`/`Err` branches from mutating the state. Subsequent renders
///   will still call the future again on the next mount.
///
/// Cloning a handle is cheap — `UseAsyncHandle` is `Copy` if its
/// generic parameters are. Use it from event handlers the same way
/// you'd use a `Signal<T>`.
#[derive(Clone, Copy)]
pub struct UseAsyncHandle<T, L = ()>
where
    T: Clone + PartialEq + 'static,
    L: Clone + PartialEq + HasLoadingHint + 'static,
{
    /// Address of the heap-allocated `UseAsyncInner<T, L>` state.
    pub(crate) inner: usize,
    /// `Copy` marker so `UseAsyncHandle` itself is `Copy`.
    pub(crate) _marker: core::marker::PhantomData<fn() -> (T, L)>,
}

impl<T, L> core::fmt::Debug for UseAsyncHandle<T, L>
where
    T: Clone + PartialEq + 'static,
    L: Clone + PartialEq + HasLoadingHint + 'static,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Avoid touching the inner pointer in `Debug` output — the
        // address is meaningless to users and could collide with
        // string-formatted `AsyncState` payloads.
        f.debug_struct("UseAsyncHandle")
            .field("inner", &format_args!("<opaque 0x{:x}>", self.inner))
            .finish()
    }
}

impl<T, L> Default for UseAsyncHandle<T, L>
where
    T: Clone + PartialEq + 'static,
    L: Clone + PartialEq + HasLoadingHint + 'static,
{
    fn default() -> Self {
        // Same fallback path as `App::use_signal` when the hook
        // context is unavailable: a fresh state handle that points
        // at a stand-alone `UseAsyncInner`. This means
        // `UseAsyncHandle::default()` always gives the caller
        // something they can `match` on, but the state will stay
        // stuck in `Loading` because no future is wired up.
        Self::new_for_fallback()
    }
}
