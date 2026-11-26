use super::*;

/// Creates a reactive `UiThemeState` hook initialized from `scheme` and `mode`.
///
/// # Arguments
///
/// - `DesignScheme` - Initial design scheme preset.
/// - `ThemeMode` - Initial light or dark mode.
///
/// # Returns
///
/// - `UiThemeState` - Reactive theme state handle.
#[must_use]
pub fn use_theme(scheme: DesignScheme, mode: ThemeMode) -> UiThemeState {
    UiThemeState::new(scheme, mode)
}

/// Creates an `I18nCatalog` hook initialized with `locale`.
///
/// # Arguments
///
/// - `String` - Initial BCP-47 locale identifier.
///
/// # Returns
///
/// - `I18nCatalog` - Reactive localization catalog.
#[must_use]
pub fn use_i18n(locale: String) -> I18nCatalog {
    I18nCatalog::new(locale)
}

/// Returns the reactive `Signal<Breakpoint>` from the ambient `Context`.
///
/// # Returns
///
/// - `Signal<Breakpoint>` - Live responsive breakpoint signal.
#[must_use]
pub fn use_breakpoint() -> Signal<Breakpoint> {
    current_context().get_breakpoint()
}

/// Returns the reactive `Signal<Size>` logical viewport dimensions from the ambient `Context`.
///
/// # Returns
///
/// - `Signal<Size>` - Live logical viewport size signal updated immediately on window resize.
#[must_use]
pub fn use_viewport_size() -> Signal<Size> {
    current_context().get_viewport_size()
}

/// Creates a cross-platform reactive `Router` hook initialized at `initial_path`.
///
/// # Arguments
///
/// - `String` - Starting route path.
///
/// # Returns
///
/// - `Router` - Reactive navigation router.
#[must_use]
pub fn use_router(initial_path: String) -> Router {
    Router::at(initial_path)
}

/// Creates a default `GestureRecognizer` hook for pointer and touch gestures.
///
/// # Returns
///
/// - `GestureRecognizer` - Fresh gesture recognizer.
#[must_use]
pub fn use_gesture() -> GestureRecognizer {
    GestureRecognizer::default()
}

/// Returns the reactive `Signal<Option<GestureEvent>>` stream from the ambient `Context`.
///
/// # Returns
///
/// - `Signal<Option<GestureEvent>>` - Live cross-platform gesture stream (`Tap`, `DoubleTap`, `LongPress`, `Swipe*`, `Pan`, `Pinch`).
#[must_use]
pub fn use_gesture_events() -> Signal<Option<GestureEvent>> {
    current_context().get_gesture()
}

/// Returns the reactive `(Signal<SafeAreaInsets>, Signal<ThemeMode>)` window-metrics hook pair from the ambient `Context`.
///
/// # Returns
///
/// - `(Signal<SafeAreaInsets>, Signal<ThemeMode>)` - Live window safe-area and OS theme signals.
#[must_use]
pub fn use_window() -> (Signal<SafeAreaInsets>, Signal<ThemeMode>) {
    let ctx: Context = current_context();
    (ctx.get_safe_area(), ctx.get_theme_mode())
}
