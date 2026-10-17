use super::*;

/// Tracks the previously observed value of some external
/// reactive source.
///
/// Typical use: in a render closure, call
/// `previous.record(current)` at the top, then read
/// `previous.get_previous().get()` to find out what the
/// value was on the previous render. The two reads are
/// decoupled so callers can record and read independently.
///
/// # Why a `Signal<Option<T>>`?
///
/// Because the very first call to `record` has no
/// "previous" to report. The signal starts at `None`
/// and flips to `Some(value)` after the first record.
/// Render code can branch on the `Option` for
/// "first render vs subsequent".
///
/// # Lombok caveat
///
/// `Previous` cannot use Lombok `New` because the
/// `previous: Signal<Option<T>>` field would require
/// `T: Default` to satisfy `Signal::default()`. The
/// struct intentionally keeps `T: Clone + PartialEq +
/// 'static` (no `Default`), and the constructor is
/// hand-written in `impl.rs` to wrap the field with
/// `Signal::create(None)`.
#[derive(Clone, Data, Debug)]
pub struct Previous<T: Clone + PartialEq + 'static> {
    /// The previous-value signal. `None` until the first
    /// `record` call.
    ///
    /// Public because `ui/tests/previous/fn.rs` reads it
    /// through `get_previous()`.
    pub previous: Signal<Option<T>>,
}

/// `Previous<T>` is `Copy` when `T` is — `Signal<Option<T>>` is
/// already `Copy` (the signal registry hands out cheap `usize`
/// addresses), so this blanket impl is sound.
impl<T> Copy for Previous<T> where T: Clone + PartialEq + 'static {}
