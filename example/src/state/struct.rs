use super::*;

/// Reactive application state powering every interactive page of the cross-platform showcase.
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
    /// Controlled multi-line textarea value.
    #[get(type(copy))]
    pub(crate) bio: Signal<String>,
    /// Controlled numeric stepper value.
    #[get(type(copy))]
    pub(crate) stepper_value: Signal<i64>,
    /// Controlled radio group selection index.
    #[get(type(copy))]
    pub(crate) radio_index: Signal<usize>,
    /// Controlled continuous slider value.
    #[get(type(copy))]
    pub(crate) slider_value: Signal<f32>,
    /// Controlled select dropdown index.
    #[get(type(copy))]
    pub(crate) select_index: Signal<usize>,
    /// Controlled select dropdown open state.
    #[get(type(copy))]
    pub(crate) select_open: Signal<bool>,
    /// Controlled star rating value.
    #[get(type(copy))]
    pub(crate) rating_value: Signal<usize>,
    /// Controlled toggle pill pressed state.
    #[get(type(copy))]
    pub(crate) pin_toggled: Signal<bool>,
    /// Controlled toggle switch for hardware acceleration preference.
    #[get(type(copy))]
    pub(crate) hardware_accel: Signal<bool>,
    /// Controlled checkbox for terms acknowledgment.
    #[get(type(copy))]
    pub(crate) agree_terms: Signal<bool>,
    /// Controlled modal visibility flag.
    #[get(type(copy))]
    pub(crate) modal_open: Signal<bool>,
    /// Controlled drawer visibility flag.
    #[get(type(copy))]
    pub(crate) drawer_open: Signal<bool>,
    /// Controlled bottom sheet visibility flag.
    #[get(type(copy))]
    pub(crate) sheet_open: Signal<bool>,
    /// Controlled popover visibility flag.
    #[get(type(copy))]
    pub(crate) popover_open: Signal<bool>,
    /// Controlled dropdown menu open flag.
    #[get(type(copy))]
    pub(crate) dropdown_open: Signal<bool>,
    /// Controlled dropdown last selected index.
    #[get(type(copy))]
    pub(crate) dropdown_selected: Signal<Option<usize>>,
    /// Controlled toast banner visibility flag.
    #[get(type(copy))]
    pub(crate) toast_visible: Signal<bool>,
    /// Controlled corner notification visibility flag.
    #[get(type(copy))]
    pub(crate) notif_visible: Signal<bool>,
    /// Controlled loading overlay active flag.
    #[get(type(copy))]
    pub(crate) loading_active: Signal<bool>,
    /// Controlled tab bar index.
    #[get(type(copy))]
    pub(crate) active_tab: Signal<usize>,
    /// Controlled pagination page number.
    #[get(type(copy))]
    pub(crate) page_index: Signal<usize>,
    /// Controlled steps indicator active index.
    #[get(type(copy))]
    pub(crate) step_index: Signal<usize>,
    /// Controlled accordion expanded item index.
    #[get(type(copy))]
    pub(crate) accordion_open: Signal<Option<usize>>,
    /// Controlled fullscreen window toggle state.
    #[get(type(copy))]
    pub(crate) fullscreen: Signal<bool>,
    /// Reactive vertical scroll offset driving the 10,000-row virtualized list.
    #[get(type(copy))]
    pub(crate) list_scroll_y: Signal<f32>,
    /// Live Dynamic Island / Live Activity active state.
    #[get(type(copy))]
    pub(crate) island_active: Signal<bool>,
    /// Live Dynamic Island progress percentage (`0..=100`).
    #[get(type(copy))]
    pub(crate) island_progress: Signal<u8>,
    /// Live Camera system permission status text.
    #[get(type(copy))]
    pub(crate) camera_perm_status: Signal<String>,
    /// Live protocol registration and deep link telemetry text.
    #[get(type(copy))]
    pub(crate) protocol_status: Signal<String>,
    /// Total accessibility live announcements emitted interactively.
    #[get(type(copy))]
    pub(crate) a11y_announcements: Signal<u32>,
    /// Total haptic pulses emitted interactively.
    #[get(type(copy))]
    pub(crate) haptic_pulses: Signal<u32>,
    /// Embedded deterministic 2D physics game host rendered directly inside `euv`.
    pub(crate) game_host: GameEngineHost,
    /// Shared hub managing live component-level and page-level dynamic OTA updates.
    pub(crate) dynamic_hub: DynamicUpdateHub,
    /// Sibling counter preserved untouched during surgical component-level dynamic updates.
    #[get(type(copy))]
    pub(crate) sibling_counter: Signal<i32>,
    /// Human-readable log of the most recent action dispatched from a dynamically updated control.
    #[get(type(copy))]
    pub(crate) dynamic_action_log: Signal<String>,
    /// Controlled shadcn `EuvAlertDialog` open state.
    #[get(type(copy))]
    pub(crate) alert_dialog_open: Signal<bool>,
    /// Controlled shadcn `EuvAlertDialog` confirmed status.
    #[get(type(copy))]
    pub(crate) alert_dialog_confirmed: Signal<bool>,
    /// Controlled shadcn `EuvCollapsible` expanded state.
    #[get(type(copy))]
    pub(crate) collapsible_open: Signal<bool>,
    /// Controlled shadcn `EuvToggleGroup` active option index.
    #[get(type(copy))]
    pub(crate) toggle_group_index: Signal<usize>,
    /// Controlled shadcn `EuvInputOtp` passcode string.
    #[get(type(copy))]
    pub(crate) otp_code: Signal<String>,
    /// Controlled shadcn `EuvCommand` search filter text.
    #[get(type(copy))]
    pub(crate) command_query: Signal<String>,
    /// Controlled shadcn `EuvCommand` selected action index.
    #[get(type(copy))]
    pub(crate) command_selected: Signal<Option<usize>>,
    /// Controlled shadcn `EuvHoverCard` open flag.
    #[get(type(copy))]
    pub(crate) hover_card_open: Signal<bool>,
    /// Controlled shadcn `EuvCalendar` selected day of month.
    #[get(type(copy))]
    pub(crate) calendar_day: Signal<u32>,
    /// Controlled shadcn `EuvMenubar` active menu index.
    #[get(type(copy))]
    pub(crate) menubar_active: Signal<Option<usize>>,
    /// Controlled shadcn `EuvCarousel` active slide index.
    #[get(type(copy))]
    pub(crate) carousel_index: Signal<usize>,
}

/// Full-featured 2D arcade arena scene powered by `GameWorld`, `PhysicsWorld2D`, `Camera2D`, `ParticleSystem`, `TileMap`, `ActionMap`, and `AudioBus`.
#[derive(Clone, Data, Debug)]
pub struct DemoPhysicsScene {
    /// Generational ECS entity arena holding player, drones, projectiles, and obstacles.
    pub(crate) world: GameWorld,
    /// Spatial-hash 2D rigid-body physics and raycast solver.
    pub(crate) physics: PhysicsWorld2D,
    /// 2D follow camera with trauma-based screen shake and frustum culling.
    #[get(type(copy))]
    pub(crate) camera: Camera2D,
    /// Zero-allocation pooled neon particle engine.
    pub(crate) particles: ParticleSystem,
    /// Frustum-culled tiled arena map with solid corner pillars.
    pub(crate) tilemap: TileMap,
    /// Declarative arcade input bindings (WASD, Arrows, Space/Click fire, E dash, R wave).
    pub(crate) actions: ActionMap,
    /// Deterministic cross-platform sound cue bus.
    pub(crate) audio: AudioBus,
    /// Player ship entity handle.
    #[get(type(copy))]
    pub(crate) player_id: EntityId,
    /// Remaining weapon cooldown in seconds.
    #[get(type(copy))]
    pub(crate) fire_cooldown: f32,
    /// Remaining dash cooldown in seconds.
    #[get(type(copy))]
    pub(crate) dash_cooldown: f32,
    /// Current player arcade score.
    #[get(type(copy))]
    pub(crate) score: u32,
    /// Current wave counter.
    #[get(type(copy))]
    pub(crate) wave: u32,
    /// Toggle for wireframe collider debug visualization.
    #[get(type(copy))]
    pub(crate) show_colliders: bool,
    /// Smoothed real-time frames-per-second telemetry.
    #[get(type(copy))]
    pub(crate) smoothed_fps: f32,
    /// Total fixed physics ticks simulated.
    #[get(type(copy))]
    pub(crate) ticks: u64,
}
