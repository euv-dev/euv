use super::*;

/// Implementation of theme state functionality.
///
/// Provides methods for creating theme state, detecting system theme changes,
/// and managing theme-related reactive state.
impl ThemeState {
    /// Creates theme state for managing light/dark mode and root CSS classes.
    ///
    /// Detects the system color scheme preference at startup and initializes the
    /// theme signal accordingly. Uses `watch!` to reactively update the root class
    /// whenever the theme or mobile signal changes.
    ///
    /// # Arguments
    ///
    /// - `Signal<bool>` - The reactive signal indicating whether the viewport is mobile-sized.
    ///
    /// # Returns
    ///
    /// - `ThemeState` - The reactive theme state containing the theme signal and root class signal.
    pub fn use_theme_state(mobile_signal: Signal<bool>) -> ThemeState {
        let theme: Signal<String> = App::use_signal(Self::detect_system_theme);
        Self::use_system_theme_change(theme);
        let initial_theme: String = theme.get();
        let initial_mobile: bool = mobile_signal.get();
        let initial_root: &'static str = if initial_mobile {
            c_mobile_app_root().get_name()
        } else {
            c_app_root().get_name()
        };
        let root_class: Signal<String> = App::use_signal(|| {
            format!(
                "{initial_root} {theme_class}",
                theme_class = Self::theme_class_name(&initial_theme)
            )
        });
        watch!(mobile_signal, theme, |mobile: bool, theme_value: String| {
            let root: &'static str = if mobile {
                c_mobile_app_root().get_name()
            } else {
                c_app_root().get_name()
            };
            root_class.set(format!(
                "{root} {theme_class}",
                theme_class = Self::theme_class_name(&theme_value)
            ));
        });
        ThemeState { theme, root_class }
    }

    /// Subscribes to system color scheme changes and updates the theme signal.
    ///
    /// Creates a `MediaQueryList` for `prefers-color-scheme: dark` and listens
    /// for `change` events. When the system theme changes, the theme signal is
    /// updated accordingly. The listener is automatically cleaned up when the
    /// hook context is cleared.
    ///
    /// # Arguments
    ///
    /// - `Signal<String>` - The theme signal to update when the system theme changes.
    pub fn use_system_theme_change(theme_signal: Signal<String>) {
        let Some(window) = window() else {
            return;
        };
        let media_query: Option<MediaQueryList> = window
            .match_media(THEME_DARK_SCHEME_MEDIA_QUERY)
            .ok()
            .flatten();
        let Some(mql) = media_query else {
            return;
        };
        let closure: Closure<dyn FnMut(Event)> = Closure::wrap(Box::new(move |_: Event| {
            let detected: String = Self::detect_system_theme();
            let current: String = theme_signal.get();
            if current != detected {
                theme_signal.set(detected);
            }
        }));
        let _: Result<(), JsValue> = mql.add_event_listener_with_callback(
            THEME_MEDIA_QUERY_CHANGE_EVENT,
            closure.as_ref().unchecked_ref(),
        );
        closure.forget();
    }

    /// Detects the current system color scheme preference.
    ///
    /// Uses `window.matchMedia("(prefers-color-scheme: dark)")` to check whether
    /// the operating system is in dark mode. Falls back to light theme if the
    /// detection fails.
    ///
    /// # Returns
    ///
    /// - `String` - The detected system theme name ("light" or "dark").
    pub fn detect_system_theme() -> String {
        let Some(window) = window() else {
            return THEME_LIGHT.to_string();
        };
        let is_dark: bool = window
            .match_media(THEME_DARK_SCHEME_MEDIA_QUERY)
            .ok()
            .flatten()
            .map(|mql: MediaQueryList| mql.matches())
            .unwrap_or_default();
        if is_dark {
            THEME_DARK.to_string()
        } else {
            THEME_LIGHT.to_string()
        }
    }

    /// Returns the CSS class name for the given theme value.
    ///
    /// # Arguments
    ///
    /// - `&str` - The theme name ("light" or "dark").
    ///
    /// # Returns
    ///
    /// - `&'static str` - The CSS class name for the theme.
    pub(crate) fn theme_class_name(theme: &str) -> &'static str {
        if theme == THEME_DARK {
            c_theme_dark().get_name()
        } else {
            c_theme_light().get_name()
        }
    }

    /// Creates a click event handler that toggles the theme between "light" and "dark".
    ///
    /// The switch stays smooth even on pages with many elements (e.g. the list
    /// page) because individual components no longer carry their own colour
    /// (`background` / `color` / `border-color`) transitions — only the app root
    /// container animates its colour change, so the page fades as a whole while
    /// each component snaps to the new theme instantly. This avoids hundreds of
    /// elements running simultaneous colour transitions, which previously caused
    /// dropped frames.
    ///
    /// # Arguments
    ///
    /// - `Signal<String>` - The theme signal to toggle.
    ///
    /// # Returns
    ///
    /// - `Option<Rc<dyn Fn(Event)>>` - A click event handler that flips the theme value.
    pub fn toggle(theme_signal: Signal<String>) -> Option<Rc<dyn Fn(Event)>> {
        Some(Rc::new(move |_: Event| {
            let current: String = theme_signal.get();
            if current == THEME_LIGHT {
                theme_signal.set(THEME_DARK.to_string());
            } else {
                theme_signal.set(THEME_LIGHT.to_string());
            }
        }))
    }
}
