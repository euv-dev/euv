use super::*;

/// Reactive application state powering every page of the cross-platform showcase.
#[derive(Clone, Getter)]
pub struct ShowcaseState {
    /// Active route path signal.
    #[get(type(copy))]
    pub(crate) route: Signal<String>,
    /// Interactive counter signal for fine-grained reactivity demos.
    #[get(type(copy))]
    pub(crate) counter: Signal<i32>,
    /// Derived memo computing `counter * 2` without redundant notifications.
    #[get(type(copy))]
    pub(crate) doubled: Memo<i32>,
    /// Active render mode selector index (`0 = Auto`, `1 = Software`, `2 = Hardware`, `3 = Hybrid`).
    #[get(type(copy))]
    pub(crate) render_mode_index: Signal<usize>,
    /// Active reactive theme state across all 17 design schemes and light/dark modes.
    pub(crate) theme: UiThemeState,
    /// Controlled form text input value.
    #[get(type(copy))]
    pub(crate) username: Signal<String>,
    /// Controlled toggle switch for hardware acceleration preference.
    #[get(type(copy))]
    pub(crate) hardware_accel: Signal<bool>,
    /// Controlled checkbox for terms acknowledgment.
    #[get(type(copy))]
    pub(crate) agree_terms: Signal<bool>,
    /// Controlled modal visibility flag.
    #[get(type(copy))]
    pub(crate) modal_open: Signal<bool>,
    /// Controlled toast banner visibility flag.
    #[get(type(copy))]
    pub(crate) toast_visible: Signal<bool>,
    /// Controlled tab bar index.
    #[get(type(copy))]
    pub(crate) active_tab: Signal<usize>,
    /// Controlled fullscreen window toggle state.
    #[get(type(copy))]
    pub(crate) fullscreen: Signal<bool>,
    /// Reactive vertical scroll offset driving the 10,000-row virtualized list.
    #[get(type(copy))]
    pub(crate) list_scroll_y: Signal<f32>,
    /// Embedded deterministic 2D physics game host rendered directly inside `euv`.
    pub(crate) game_host: GameEngineHost,
}

/// Deterministic 2D physics scene showcasing `GameScene`, `FrameClock`, `InputState`, and HUD overlay.
#[derive(Clone, Copy, Data, Debug, New, PartialEq)]
pub struct DemoPhysicsScene {
    /// Horizontal position of the bouncing physics orb.
    #[get(type(copy))]
    pub(crate) orb_x: f32,
    /// Vertical position of the bouncing physics orb.
    #[get(type(copy))]
    pub(crate) orb_y: f32,
    /// Horizontal velocity in logical pixels per second.
    #[get(type(copy))]
    pub(crate) vel_x: f32,
    /// Vertical velocity in logical pixels per second.
    #[get(type(copy))]
    pub(crate) vel_y: f32,
    /// Player-controlled paddle X coordinate.
    #[new(skip)]
    #[get(type(copy))]
    pub(crate) paddle_x: f32,
    /// Total fixed physics ticks simulated.
    #[new(skip)]
    #[get(type(copy))]
    pub(crate) ticks: u64,
}
