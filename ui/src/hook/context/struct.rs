use super::*;

/// Reactive UI state bundle shared across all `euv-ui` components.
#[derive(Clone, Data, Debug)]
pub struct UiThemeState {
    /// Reactive design scheme signal (supports live switching across all 17 schemes).
    #[get(type(copy))]
    pub(crate) scheme: Signal<DesignScheme>,
    /// Reactive light/dark mode signal.
    #[get(type(copy))]
    pub(crate) mode: Signal<ThemeMode>,
    /// Derived memoized `ThemeTokens` recomputed only when scheme or mode changes.
    #[get(type(copy))]
    pub(crate) tokens: Memo<ThemeTokens>,
}
