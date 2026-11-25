use super::*;

/// Construction and helper methods for `ShowcaseState`.
impl ShowcaseState {
    /// Initializes a new reactive `ShowcaseState` with default values, derived `computed!` memos, and game host.
    ///
    /// # Returns
    ///
    /// - `Self` - Initialized reactive showcase state.
    #[must_use]
    pub fn new() -> Self {
        let counter: Signal<i32> = Signal::new(1);
        let doubled: Memo<i32> = computed!((counter) -> i32 => move |n: i32| n * 2);
        let mut physics: DemoPhysicsScene = DemoPhysicsScene::new(96.0, 64.0, 140.0, 95.0);
        physics.set_paddle_x(180.0);
        let game_host: GameEngineHost = GameEngineHost::new(physics, 1.0 / 60.0);
        Self {
            route: Signal::new(ROUTE_OVERVIEW.to_string()),
            counter,
            doubled,
            render_mode_index: Signal::new(0),
            theme: UiThemeState::new(active_feature_scheme(), ThemeMode::Light),
            username: Signal::new(DEFAULT_USERNAME.to_string()),
            hardware_accel: Signal::new(true),
            agree_terms: Signal::new(true),
            modal_open: Signal::new(false),
            toast_visible: Signal::new(false),
            active_tab: Signal::new(0),
            fullscreen: Signal::new(false),
            list_scroll_y: Signal::new(640.0),
            game_host,
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
            EuvNavItem::new(ROUTE_GAME.to_string(), NAV_LABEL_GAME.to_string()),
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

/// Default `ShowcaseState` delegating to `ShowcaseState::new`.
impl Default for ShowcaseState {
    /// Creates a default `ShowcaseState`.
    ///
    /// # Returns
    ///
    /// - `Self` - Initialized state.
    fn default() -> Self {
        Self::new()
    }
}

/// Deterministic fixed-timestep 2D physics simulation and HUD rendering for `DemoPhysicsScene`.
impl GameScene for DemoPhysicsScene {
    /// Steps orb kinematics and player paddle input at a fixed simulation cadence.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable physics scene.
    /// - `f32` - Fixed simulation delta time in seconds.
    /// - `&InputState` - Real-time polled keyboard and pointer state.
    fn fixed_update(&mut self, dt: f32, input: &InputState) {
        let move_x: f32 = input.axis(KeyCode::ArrowLeft, KeyCode::ArrowRight);
        let next_paddle: f32 = (self.get_paddle_x() + move_x * 240.0 * dt).clamp(24.0, 420.0);
        self.set_paddle_x(next_paddle);
        let mut x: f32 = self.get_orb_x() + self.get_vel_x() * dt;
        let mut y: f32 = self.get_orb_y() + self.get_vel_y() * dt;
        if x <= 18.0 || x >= 460.0 {
            self.set_vel_x(-self.get_vel_x());
            x = x.clamp(18.0, 460.0);
        }
        if y <= 18.0 || y >= 190.0 {
            self.set_vel_y(-self.get_vel_y());
            y = y.clamp(18.0, 190.0);
        }
        self.set_orb_x(x).set_orb_y(y);
        self.set_ticks(self.get_ticks().saturating_add(1));
    }

    /// Renders the 2D physics arena, bouncing orb, player paddle, and frame HUD telemetry.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable physics scene.
    /// - `f32` - Interpolation alpha in `[0.0, 1.0]`.
    /// - `&mut GameRenderContext<'_>` - Immediate-mode game draw surface.
    fn render(&mut self, alpha: f32, ctx: &mut GameRenderContext<'_>) {
        let size: Size = ctx.viewport_size();
        ctx.clear(Color::hex(0x0B1120));
        ctx.stroke_rect(
            Rect::new(
                4.0,
                4.0,
                (size.get_width() - 8.0).max(1.0),
                (size.get_height() - 8.0).max(1.0),
            ),
            1.5,
            Color::hex(0x1E293B),
        );
        ctx.fill_circle(
            Point::new(self.get_orb_x(), self.get_orb_y()),
            14.0,
            Color::hex(0x38BDF8),
        );
        ctx.fill_rounded_rect(
            Rect::new(self.get_paddle_x(), 184.0, 96.0, 12.0),
            Color::hex(0x34D399),
            6.0,
        );
        ctx.draw_text(
            Point::new(14.0, 12.0),
            format!(
                "60Hz Fixed Physics | ticks={} | alpha={alpha:.2}",
                self.get_ticks()
            ),
            12.0,
            Color::hex(0xE2E8F0),
        );
    }
}
