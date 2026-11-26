/// Marker trait implemented by strongly-typed cross-component custom events dispatched through `EventBus` and `#[event]`/`emit!`/`on_event!`.
pub trait CustomEvent: Clone + 'static {}
