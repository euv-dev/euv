use super::*;

/// Construction and helper methods for `ShowcaseState`.
impl ShowcaseState {
    /// Initializes a new reactive `ShowcaseState` with default values and derived memos.
    ///
    /// # Returns
    ///
    /// - `Self` - Initialized reactive showcase state.
    #[must_use]
    pub fn create() -> Self {
        let counter: Signal<i32> = Signal::new(1);
        let doubled: Memo<i32> = Memo::new(move || counter.get() * 2);
        Self {
            route: Signal::new(ROUTE_OVERVIEW.to_string()),
            counter,
            doubled,
            render_mode_index: Signal::new(0),
            theme: UiThemeState::create(active_feature_scheme(), ThemeMode::Light),
            username: Signal::new(DEFAULT_USERNAME.to_string()),
            hardware_accel: Signal::new(true),
            agree_terms: Signal::new(true),
            modal_open: Signal::new(false),
            toast_visible: Signal::new(false),
            active_tab: Signal::new(0),
            fullscreen: Signal::new(false),
        }
    }

    /// Returns the ordered sidebar navigation items for the showcase.
    ///
    /// # Returns
    ///
    /// - `Vec<EuvNavItem>` - Complete list of showcase routes.
    #[must_use]
    pub fn nav_items() -> Vec<EuvNavItem> {
        vec![
            EuvNavItem::new(ROUTE_OVERVIEW.to_string(), NAV_LABEL_OVERVIEW.to_string()),
            EuvNavItem::new(ROUTE_REACTIVE.to_string(), NAV_LABEL_REACTIVE.to_string()),
            EuvNavItem::new(ROUTE_RENDER.to_string(), NAV_LABEL_RENDER.to_string()),
            EuvNavItem::new(ROUTE_WINDOW.to_string(), NAV_LABEL_WINDOW.to_string()),
            EuvNavItem::new(ROUTE_FORMS.to_string(), NAV_LABEL_FORMS.to_string()),
            EuvNavItem::new(ROUTE_THEMES.to_string(), NAV_LABEL_THEMES.to_string()),
            EuvNavItem::new(ROUTE_DATA.to_string(), NAV_LABEL_DATA.to_string()),
            EuvNavItem::new(ROUTE_OVERLAYS.to_string(), NAV_LABEL_OVERLAYS.to_string()),
        ]
    }

    /// Resolves the currently selected `RenderMode` from `render_mode_index`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - State reference.
    ///
    /// # Returns
    ///
    /// - `RenderMode` - Active render pipeline mode.
    #[must_use]
    pub fn selected_render_mode(&self) -> RenderMode {
        match self.get_render_mode_index().get_untracked() {
            1 => RenderMode::Software,
            2 => RenderMode::Hardware,
            3 => RenderMode::Hybrid,
            _ => RenderMode::Auto,
        }
    }

    /// Cycles to the next `DesignScheme` in `ALL_DESIGN_SCHEMES`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - State reference.
    pub fn next_scheme(&self) {
        let all: [DesignScheme; 17] = DesignScheme::all();
        let current: DesignScheme = self.get_theme().get_scheme().get_untracked();
        let pos: usize = all
            .iter()
            .position(|s: &DesignScheme| *s == current)
            .unwrap_or(0);
        let next: DesignScheme = all[(pos + 1) % all.len()];
        let _: bool = self.get_theme().get_scheme().set_if_changed(next);
    }

    /// Toggles between light and dark theme modes.
    ///
    /// # Arguments
    ///
    /// - `&Self` - State reference.
    pub fn toggle_theme_mode(&self) {
        let cur: ThemeMode = self.get_theme().get_mode().get_untracked();
        let next: ThemeMode = match cur {
            ThemeMode::Light => ThemeMode::Dark,
            ThemeMode::Dark => ThemeMode::Light,
        };
        let _: bool = self.get_theme().get_mode().set_if_changed(next);
    }
}
