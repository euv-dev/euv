use super::*;

/// Construction and switching methods for `UiThemeState`.
impl UiThemeState {
    /// Creates a reactive `UiThemeState` initialized to `(scheme, mode)`.
    ///
    /// # Arguments
    ///
    /// - `DesignScheme` - Initial design scheme.
    /// - `ThemeMode` - Initial appearance mode.
    ///
    /// # Returns
    ///
    /// - `Self` - Reactive theme controller.
    #[must_use]
    pub fn create(initial_scheme: DesignScheme, initial_mode: ThemeMode) -> Self {
        let scheme: Signal<DesignScheme> = Signal::new(initial_scheme);
        let mode: Signal<ThemeMode> = Signal::new(initial_mode);
        let tokens: Memo<ThemeTokens> =
            Memo::new(move || ThemeTokens::resolve(scheme.get(), mode.get()));
        Self {
            scheme,
            mode,
            tokens,
        }
    }

    /// Toggles between `ThemeMode::Light` and `ThemeMode::Dark`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Theme state reference.
    pub fn toggle_mode(&self) {
        let next: ThemeMode = match self.get_mode().get_untracked() {
            ThemeMode::Light => ThemeMode::Dark,
            ThemeMode::Dark => ThemeMode::Light,
        };
        let _: bool = self.get_mode().set(next);
    }

    /// Switches to `scheme` reactively.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Theme state reference.
    /// - `DesignScheme` - Target design language.
    pub fn select_scheme(&self, scheme: DesignScheme) {
        let _: bool = self.get_scheme().set_if_changed(scheme);
    }

    /// Reads the current `ThemeTokens` reactively.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Theme state reference.
    ///
    /// # Returns
    ///
    /// - `ThemeTokens` - Active design tokens.
    #[must_use]
    pub fn current_tokens(&self) -> ThemeTokens {
        self.get_tokens().get()
    }
}
