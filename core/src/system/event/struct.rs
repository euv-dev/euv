use super::*;

/// Handle returned by scoped system event hooks allowing optional manual early unregistration.
#[derive(Clone, Copy, Data, Debug, Eq, Hash, New, PartialEq)]
pub struct SystemEventSubscription {
    /// Monotonic listener identifier inside the thread-local `SystemEventHub`.
    #[get(type(copy))]
    pub(crate) id: u64,
}

/// Deterministic state machine deduplicating noisy OS callbacks and enforcing canonical cross-platform event ordering.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct SystemEventNormalizer {
    /// Current canonical application lifecycle state.
    #[get(type(copy))]
    pub(crate) state: AppState,
    /// Whether the active application surface is currently visible (`Appear` emitted without matching `Disappear`).
    #[get(type(copy))]
    pub(crate) visible: bool,
    /// Most recently emitted window focus state for deduplication.
    #[get(skip)]
    pub(crate) last_focused: Option<bool>,
    /// Most recently emitted logical `(width, height, scale_factor)` resize tuple.
    #[get(skip)]
    pub(crate) last_resize: Option<(f32, f32, f32)>,
    /// Most recently emitted safe-area insets.
    #[get(skip)]
    pub(crate) last_safe_area: Option<SafeAreaInsets>,
    /// Most recently emitted screen orientation.
    #[get(skip)]
    pub(crate) last_orientation: Option<ScreenOrientation>,
    /// Most recently emitted OS light/dark theme.
    #[get(skip)]
    pub(crate) last_theme: Option<SystemTheme>,
    /// Most recently emitted OS locale string.
    #[get(skip)]
    pub(crate) last_locale: Option<String>,
    /// Most recently emitted virtual keyboard `(visible, height)` state.
    #[get(skip)]
    pub(crate) last_keyboard: Option<(bool, f32)>,
    /// Most recently emitted low-power mode flag.
    #[get(skip)]
    pub(crate) last_low_power: Option<bool>,
    /// Most recently emitted network reachability state.
    #[get(skip)]
    pub(crate) last_network: Option<NetworkStatus>,
}

/// Internal scope-bound system event listener entry stored in `SystemEventHub`.
#[derive(CustomDebug)]
pub(crate) struct SystemEventListener {
    /// Unique listener identifier.
    pub(crate) id: u64,
    /// Reactive scope owning this listener, if registered inside a scope.
    pub(crate) owner_scope: Option<ScopeId>,
    /// Type-erased callback invoked when matching system events arrive.
    #[debug(skip)]
    pub(crate) callback: SystemEventCallback,
}

/// Thread-local registry of active scoped system event listeners.
#[derive(CustomDebug, Default)]
pub(crate) struct SystemEventHub {
    /// Monotonic listener identifier counter.
    pub(crate) next_id: u64,
    /// Active registered listeners in registration order.
    pub(crate) listeners: Vec<SystemEventListener>,
}
