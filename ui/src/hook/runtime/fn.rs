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
    UiThemeState::create(scheme, mode)
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
    I18nCatalog::create(locale)
}

/// Returns the reactive `Signal<Breakpoint>` from `ctx`.
///
/// # Arguments
///
/// - `&BuildContext` - Active build context.
///
/// # Returns
///
/// - `Signal<Breakpoint>` - Live responsive breakpoint signal.
#[must_use]
pub fn use_breakpoint(ctx: &BuildContext) -> Signal<Breakpoint> {
    ctx.get_breakpoint()
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

/// Creates a reactive `(Signal<SafeAreaInsets>, Signal<ThemeMode>)` window-metrics hook pair from `ctx`.
///
/// # Arguments
///
/// - `&BuildContext` - Active build context.
///
/// # Returns
///
/// - `(Signal<SafeAreaInsets>, Signal<ThemeMode>)` - Live window safe-area and OS theme signals.
#[must_use]
pub fn use_window(ctx: &BuildContext) -> (Signal<SafeAreaInsets>, Signal<ThemeMode>) {
    (ctx.get_safe_area(), ctx.get_theme_mode())
}
