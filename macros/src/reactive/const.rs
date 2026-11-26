/// Keyword prefix for `watch!(global StateType => |prev, next| { ... })`.
pub(crate) const WATCH_KW_GLOBAL: &str = "global";

/// Error message when `watch!` receives an empty explicit signal tuple.
pub(crate) const ERR_WATCH_EMPTY_SIGNALS: &str = "watch! requires at least one signal, e.g. `watch!((count) => move |n: i32| { ... })` or auto-tracked `watch!(move || { ... })`";

/// Error message when `computed!` receives an empty explicit signal tuple.
pub(crate) const ERR_COMPUTED_EMPTY_SIGNALS: &str = "computed! requires at least one signal, e.g. `computed!((count) -> i32 => move |n: i32| n * 2)` or auto-tracked `computed!(move || count.get() * 2)`";
