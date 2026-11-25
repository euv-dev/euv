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
    /// Embedded AAA 3D open-world PBR + BVH ray-tracing game host.
    pub(crate) game3d_host: GameEngineHost,
    /// Shared mutable runtime 3D graphics/ray-tracing configuration adjusted live from UI controls.
    #[get(type(copy))]
    pub(crate) engine3d_quality: Signal<RenderQualityConfig>,
    /// Reactive 3D quality preset index (`0=Low, 1=Medium, 2=High, 3=Ultra, 4=Cinematic`).
    #[get(type(copy))]
    pub(crate) engine3d_preset_idx: Signal<usize>,
    /// Reactive 3D ray-tracing mode index (`0=Off, 1=Shadows, 2=Shadows+Refl, 3=Full GI`).
    #[get(type(copy))]
    pub(crate) engine3d_rt_idx: Signal<usize>,
    /// Reactive 3D internal resolution scale percentage (`40.0..=125.0`).
    #[get(type(copy))]
    pub(crate) engine3d_res_scale: Signal<f32>,
    /// Reactive 3D open-world time of day (`6.0..=23.0` hours).
    #[get(type(copy))]
    pub(crate) engine3d_time_of_day: Signal<f32>,
    /// Pending live 3D model import request `(format, label, source)` consumed on the next 3D tick.
    #[get(type(copy))]
    pub(crate) engine3d_import_queue: Signal<u64>,
    /// Human-readable status of the most recent 3D model import.
    #[get(type(copy))]
    pub(crate) engine3d_import_status: Signal<String>,
    /// Active 3D camera perspective mode (`0=Chase, 1=Hood, 2=Orbit, 3=Aerial`).
    #[get(type(copy))]
    pub(crate) engine3d_camera_mode: Signal<u8>,
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
    /// Reactive animated GIF playback state for the Media Studio showcase.
    #[get(type(copy))]
    pub(crate) media_gif: Signal<GifAnimation>,
    /// Reactive LIVE Photo motion state for the Media Studio showcase.
    #[get(type(copy))]
    pub(crate) media_live: Signal<LivePhotoAsset>,
    /// Reactive cross-platform video track state for the Media Studio showcase.
    #[get(type(copy))]
    pub(crate) media_video: Signal<VideoTrack>,
    /// Reactive WAV audio track and waveform state for the Media Studio showcase.
    #[get(type(copy))]
    pub(crate) media_audio: Signal<AudioTrack>,
    /// Reactive multi-page PDF document state for the Media Studio showcase.
    #[get(type(copy))]
    pub(crate) media_pdf: Signal<PdfDocument>,
    /// Most recently clicked native HTML hyperlink URI in the Media Studio showcase.
    #[get(type(copy))]
    pub(crate) media_html_link_log: Signal<String>,
    /// Simulated target platform index for cross-platform Dynamic Island & permissions matrix (`0..7`).
    #[get(type(copy))]
    pub(crate) os_target_platform_idx: Signal<usize>,
    /// Active Dynamic Island presentation mode index (`0..6`).
    #[get(type(copy))]
    pub(crate) island_mode_idx: Signal<usize>,
    /// Live notification delivery stack displayed via `EuvSonner`.
    #[get(type(copy))]
    pub(crate) system_notifications: Signal<Vec<(String, String)>>,
    /// Per-permission status map across all 14 `SystemPermission` categories.
    #[get(type(copy))]
    pub(crate) permission_statuses: Signal<Vec<PermissionStatus>>,
    /// Controlled search query for `EuvCombobox` in the UI & Edge Lab.
    #[get(type(copy))]
    pub(crate) lab_combobox_query: Signal<String>,
    /// Controlled selected index for `EuvCombobox`.
    #[get(type(copy))]
    pub(crate) lab_combobox_selected: Signal<usize>,
    /// Controlled open flag for `EuvCombobox`.
    #[get(type(copy))]
    pub(crate) lab_combobox_open: Signal<bool>,
    /// Controlled filter string for `EuvDataTable`.
    #[get(type(copy))]
    pub(crate) lab_table_filter: Signal<String>,
    /// Controlled sort order for `EuvDataTable`.
    #[get(type(copy))]
    pub(crate) lab_table_sort_asc: Signal<bool>,
    /// Controlled selected row index for `EuvDataTable`.
    #[get(type(copy))]
    pub(crate) lab_table_selected: Signal<Option<usize>>,
    /// Controlled selected day for `EuvDatePicker`.
    #[get(type(copy))]
    pub(crate) lab_date_day: Signal<u32>,
    /// Controlled open flag for `EuvDatePicker`.
    #[get(type(copy))]
    pub(crate) lab_date_open: Signal<bool>,
    /// Controlled domain input value for `EuvInputGroup`.
    #[get(type(copy))]
    pub(crate) lab_input_group_val: Signal<String>,
    /// Controlled selected index for `EuvNativeSelect`.
    #[get(type(copy))]
    pub(crate) lab_native_select_idx: Signal<usize>,
    /// Controlled active dropdown index for `EuvNavigationMenu`.
    #[get(type(copy))]
    pub(crate) lab_nav_menu_active: Signal<Option<usize>>,
    /// Controlled split ratio for `EuvResizable`.
    #[get(type(copy))]
    pub(crate) lab_split_ratio: Signal<f32>,
    /// Toggle controlling whether the nested edge-case box blocks scroll penetration (`true`) or allows chaining (`false`).
    #[get(type(copy))]
    pub(crate) lab_block_scroll_chain: Signal<bool>,
    /// Toggle controlling whether the nested edge-case box stops click event bubbling (`true`) or allows bubbling (`false`).
    #[get(type(copy))]
    pub(crate) lab_stop_click_bubble: Signal<bool>,
    /// Counter tracking clicks received by the outer parent card in the event bubbling test.
    #[get(type(copy))]
    pub(crate) lab_parent_clicks: Signal<u32>,
    /// Counter tracking clicks received by the inner button in the event bubbling test.
    #[get(type(copy))]
    pub(crate) lab_child_clicks: Signal<u32>,
    /// Persistent counter proving live resize never destroys or resets page state.
    #[get(type(copy))]
    pub(crate) lab_resize_preserve_count: Signal<u32>,
    /// Controlled direction mode toggle (`false=LTR, true=RTL`).
    #[get(type(copy))]
    pub(crate) lab_rtl_mode: Signal<bool>,
    /// Cross-platform in-memory WASM CDN registry serving real `.wasm` binaries and fault scenarios.
    pub(crate) wasm_registry: RegistryWasmFetcher,
    /// Dynamic WASM download, verification, sandbox execution, cache, and rollback manager.
    pub(crate) wasm_manager: WasmDynamicManager,
    /// Simulated offline network mode toggle for verifying verified offline WASM cache hits.
    #[get(type(copy))]
    pub(crate) wasm_offline: Signal<bool>,
    /// Live human-readable readout of the latest dynamic WASM download or fault verification case.
    #[get(type(copy))]
    pub(crate) wasm_fault_status: Signal<String>,
    /// Controlled global `⌘K` Command Palette modal visibility flag.
    #[get(type(copy))]
    pub(crate) cmd_palette_open: Signal<bool>,
    /// Controlled global `⌘K` Command Palette filter query.
    #[get(type(copy))]
    pub(crate) cmd_palette_query: Signal<String>,
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

/// Interactive AAA 3D open-world city scene with PBR shading, BVH ray tracing, model importing, and runtime settings.
#[derive(Clone, Data, Debug)]
pub struct DemoOpenWorld3DScene {
    /// Open-world 3D city scene graph with instanced towers, reflective wet boulevard, and drivable GT car.
    pub(crate) world: World3D,
    /// Zero-allocation PBR + BVH ray-tracing renderer.
    pub(crate) renderer: Renderer3D,
    /// Shared arcade action bindings for driving/flying the 3D scene.
    pub(crate) actions: ActionMap,
    /// Reactive runtime quality, resolution scale, and ray-tracing configuration.
    #[get(type(copy))]
    pub(crate) quality_signal: Signal<RenderQualityConfig>,
    /// Reactive time-of-day sun slider signal.
    #[get(type(copy))]
    pub(crate) time_signal: Signal<f32>,
    /// Monotonic import trigger signal (`odd = OBJ`, `even = glTF`).
    #[get(type(copy))]
    pub(crate) import_signal: Signal<u64>,
    /// Last processed import counter.
    #[get(type(copy))]
    pub(crate) last_import_seq: u64,
    /// Status text signal reporting imported vertices/triangles/BVH count.
    #[get(type(copy))]
    pub(crate) status_signal: Signal<String>,
    /// Reactive 3D camera mode signal (`0=Chase, 1=Hood, 2=Orbit, 3=Aerial`).
    #[get(type(copy))]
    pub(crate) camera_mode_signal: Signal<u8>,
    /// Smoothed real-time FPS telemetry.
    #[get(type(copy))]
    pub(crate) smoothed_fps: f32,
}
