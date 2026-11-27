use super::*;

/// Default application configuration with light white background.
impl Default for AppConfig {
    /// Creates default `AppConfig`.
    ///
    /// # Returns
    ///
    /// - `Self` - Default application config.
    fn default() -> Self {
        Self {
            window: WindowConfig::default(),
            clear_color: COLOR_WHITE,
        }
    }
}

/// Core lifecycle, event dispatch, and frame rendering pipeline for `App`.
impl App {
    /// Creates an `App` bound to the active native OS/Web platform backend.
    ///
    /// # Arguments
    ///
    /// - `AppConfig` - Declarative application configuration.
    ///
    /// # Returns
    ///
    /// - `Result<Self, PlatformError>` - Initialized cross-platform app.
    pub fn new(config: AppConfig) -> Result<Self, PlatformError> {
        let backend: Box<dyn PlatformBackend> = create_native_platform();
        Self::with_platform(config, backend)
    }

    /// Creates an `App` backed by a deterministic offscreen `HeadlessPlatform`.
    ///
    /// # Arguments
    ///
    /// - `AppConfig` - Declarative application configuration.
    ///
    /// # Returns
    ///
    /// - `Result<Self, PlatformError>` - Initialized headless app.
    pub fn headless(config: AppConfig) -> Result<Self, PlatformError> {
        let backend: Box<dyn PlatformBackend> = Box::new(HeadlessPlatform::default());
        Self::with_platform(config, backend)
    }

    /// Creates an `App` backed by an explicit `PlatformBackend` implementation.
    ///
    /// # Arguments
    ///
    /// - `AppConfig` - Application configuration.
    /// - `Box<dyn PlatformBackend>` - Platform backend implementation.
    ///
    /// # Returns
    ///
    /// - `Result<Self, PlatformError>` - Initialized application runtime.
    pub fn with_platform(
        config: AppConfig,
        mut platform: Box<dyn PlatformBackend>,
    ) -> Result<Self, PlatformError> {
        let window_state: WindowState = platform.create_window(config.get_window())?;
        let kind: PlatformKind = platform.kind();
        let caps: PlatformCapabilities = platform.capabilities();
        let native_hz: f32 = platform.display_refresh_rate_hz().clamp(24.0, 480.0);
        let scene: SceneHandle = SceneHandle::new_tree();
        let context: Context = Context::new(scene.clone(), kind);
        let dynamic_hub: DynamicUpdateHub = context.get_dynamic_hub().clone();
        let _: bool = context.get_breakpoint().set(window_state.breakpoint());
        let _: bool = context.get_viewport_size().set(window_state.logical_size());
        let _: bool = context
            .get_theme_mode()
            .set(window_state.get_system_theme());
        let _: bool = context.get_safe_area().set(window_state.get_safe_area());
        let _: bool = context.get_orientation().set(ScreenOrientation::from_size(
            window_state.get_logical_width(),
            window_state.get_logical_height(),
        ));
        let _: bool = context.get_app_state().set(AppState::Active);
        let _: bool = context.get_window_focused().set(window_state.get_focused());
        let _: bool = context.get_display_refresh_rate().set(native_hz);
        let _: bool = context.get_live_fps().set(native_hz);
        let renderer: Renderer = Renderer::new(
            kind,
            caps,
            window_state.get_physical_width(),
            window_state.get_physical_height(),
            config.get_window().get_render(),
        );
        Ok(Self {
            config,
            platform,
            window_state,
            scene,
            context,
            layout_engine: LayoutEngine::default(),
            renderer,
            command_queue: WindowCommandQueue::default(),
            storage: StorageStore::default(),
            running: true,
            pending_damage: DamageRegion::full(),
            input_state: InputState::new(),
            frame_clock: FrameClock::default(),
            services: PlatformServiceHub::new(kind),
            deep_link: Signal::new(None),
            dynamic_hub,
            last_rendered_revision: 0,
            last_cursor: CursorIcon::Default,
            game_hosts: Vec::new(),
            touch_drag_pointer_id: None,
            touch_drag_origin: None,
            touch_drag_last: None,
            touch_dragging: false,
            touch_drag_travel: 0.0,
            touch_scroll_axis: 0,
            touch_vel_accum_delta: Point::default(),
            touch_vel_samples: Vec::with_capacity(16),
            touch_vel_idle_secs: 0.0,
            scrollbar_dragging: false,
            scrollbar_drag_axis: SCROLLBAR_AXIS_NONE,
            text_selecting_node: None,
            text_select_anchor_char: 0,
            text_click_count: 0,
            text_last_click_node: None,
            text_last_click_pos: None,
            text_last_click_age_secs: 10.0,
            active_pointers: HashMap::new(),
            active_pointer_devices: HashMap::new(),
            gesture_origin: None,
            pinch_base_dist: 0.0,
            gesture_hold_secs: 0.0,
            long_press_fired: false,
            last_tap_pos: None,
            last_tap_age_secs: 10.0,
            gesture_seq: 0,
            scroll_velocity: Point::default(),
            scroll_fling_pos: Point::default(),
            touch_scroll_target: None,
            smooth_scroll_pending: Point::default(),
            smooth_scroll_pos: Point::default(),
            smooth_scroll_target: None,
            in_step_frame: false,
            caret_blink_secs: 0.0,
            gesture_suppressed: false,
            fps_accum_secs: 0.0,
            fps_accum_frames: 0,
            fps_budget_accum: 0.0,
            last_ime_active: false,
            last_ime_node: None,
            last_frame_damage: DamageRegion::default(),
            display_refresh_rate_hz: native_hz,
            was_animating: false,
            system_event_normalizer: SystemEventNormalizer::new(),
        })
    }

    /// Returns the detected native hardware display refresh rate in Hertz (`120.0`, `144.0`, `180.0`, etc.).
    ///
    /// # Arguments
    ///
    /// - `&Self` - Application reference.
    ///
    /// # Returns
    ///
    /// - `f32` - Active display refresh rate in Hertz.
    #[must_use]
    pub fn display_refresh_rate_hz(&self) -> f32 {
        self.get_display_refresh_rate_hz()
    }

    /// Updates the detected native hardware display refresh rate reported by an OS or browser host.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable application.
    /// - `f32` - Measured or queried display refresh rate in Hertz.
    pub fn update_display_refresh_rate_hz(&mut self, hz: f32) {
        if !hz.is_finite() || hz < 24.0 {
            return;
        }
        let clamped: f32 = (hz.clamp(24.0, 480.0) * 10.0).round() * 0.1;
        self.set_display_refresh_rate_hz(clamped);
        let _: bool = self
            .get_context()
            .get_display_refresh_rate()
            .set_if_changed(clamped);
        if self.get_fps_accum_frames() == 0 {
            let idle_fps: f32 = match self.fps_limit() {
                Some(cap) => (cap as f32).min(clamped),
                None => clamped,
            };
            let _: bool = self.get_context().get_live_fps().set_if_changed(idle_fps);
        }
    }

    /// Reports live host VSync / `requestAnimationFrame` cadence when the scene is idle so telemetry reflects the real display rate.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable application.
    /// - `f32` - Measured host presentation cadence in Hertz.
    pub fn report_host_refresh_rate(&mut self, measured_hz: f32) {
        if !measured_hz.is_finite() || measured_hz < 24.0 {
            return;
        }
        let rounded: f32 = (measured_hz.clamp(24.0, 480.0) * 10.0).round() * 0.1;
        if (rounded - self.get_display_refresh_rate_hz()).abs() >= 1.5 {
            self.update_display_refresh_rate_hz(rounded);
        } else if self.get_fps_accum_frames() == 0 {
            let effective: f32 = match self.fps_limit() {
                Some(cap) => (cap as f32).min(rounded),
                None => rounded,
            };
            let _: bool = self.get_context().get_live_fps().set_if_changed(effective);
        }
    }

    /// Returns whether an editable input node currently requests the system soft keyboard / IME.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Application reference.
    ///
    /// # Returns
    ///
    /// - `bool` - True when an enabled `NodeKind::Input` holds keyboard focus.
    #[must_use]
    pub fn is_ime_active(&self) -> bool {
        self.get_last_ime_active()
    }

    /// Returns the damage region rasterized during the most recent rendered frame.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Application reference.
    ///
    /// # Returns
    ///
    /// - `DamageRegion` - Last rendered damage region.
    #[must_use]
    pub fn last_frame_damage(&self) -> DamageRegion {
        self.get_last_frame_damage()
    }

    /// Reads the current plain-text clipboard content from the active platform backend.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Application reference.
    ///
    /// # Returns
    ///
    /// - `Option<String>` - Current clipboard string if available.
    #[must_use]
    pub fn read_clipboard(&self) -> Option<String> {
        let Self { platform, .. } = self;
        platform.read_clipboard()
    }

    /// Writes `text` into the active platform backend clipboard.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable application.
    /// - `&str` - UTF-8 string to place in the system clipboard.
    ///
    /// # Returns
    ///
    /// - `Result<(), PlatformError>` - Clipboard write status.
    pub fn write_clipboard(&mut self, text: &str) -> Result<(), PlatformError> {
        let res: Result<(), PlatformError> = {
            let Self { platform, .. } = self;
            platform.write_clipboard(text)
        };
        if res.is_ok() {
            self.dispatch_system_event(SystemEvent::ClipboardChanged);
        }
        res
    }

    /// Returns the current normalized cross-platform `AppState`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Application reference.
    ///
    /// # Returns
    ///
    /// - `AppState` - Current canonical application lifecycle state.
    #[must_use]
    pub fn app_state(&self) -> AppState {
        self.get_system_event_normalizer().state()
    }

    /// Returns the reactive `Signal<AppState>` tracking canonical application lifecycle transitions.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Application reference.
    ///
    /// # Returns
    ///
    /// - `Signal<AppState>` - Reactive application state signal.
    #[must_use]
    pub fn app_state_signal(&self) -> Signal<AppState> {
        self.get_context().app_state_signal()
    }

    /// Normalizes a raw `SystemEvent` through `SystemEventNormalizer`, updates reactive `Context` signals, and notifies all active hooks and visible `Lifecycle` components.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable application.
    /// - `SystemEvent` - Incoming platform or synthetic system event.
    pub fn dispatch_system_event(&mut self, event: SystemEvent) {
        let canonical_events: Vec<SystemEvent> =
            self.get_mut_system_event_normalizer().normalize(event);
        let next_state: AppState = self.get_system_event_normalizer().state();
        let _: bool = self
            .get_context()
            .get_app_state()
            .set_if_changed(next_state);
        for ev in canonical_events.into_iter() {
            match &ev {
                SystemEvent::FocusChanged(focused) => {
                    self.get_mut_window_state().set_focused(*focused);
                    let _: bool = self
                        .get_context()
                        .get_window_focused()
                        .set_if_changed(*focused);
                    if !*focused {
                        self.cancel_all_active_pointers();
                    }
                }
                SystemEvent::Resize {
                    width,
                    height,
                    scale_factor,
                } => {
                    let prev_ws: WindowState = self.get_window_state();
                    if (prev_ws.get_logical_width() - *width).abs() > 0.5
                        || (prev_ws.get_logical_height() - *height).abs() > 0.5
                        || (prev_ws.get_scale_factor() - *scale_factor).abs() > 0.01
                    {
                        self.get_mut_window_state().set_logical_width(*width);
                        self.get_mut_window_state().set_logical_height(*height);
                        self.get_mut_window_state().set_scale_factor(*scale_factor);
                        self.get_mut_window_state().set_physical_width(
                            ((*width) * (*scale_factor)).round().max(1.0) as u32,
                        );
                        self.get_mut_window_state().set_physical_height(
                            ((*height) * (*scale_factor)).round().max(1.0) as u32,
                        );
                    }
                    let bp: Breakpoint = self.get_window_state().breakpoint();
                    let logical_size: Size = Size::new(*width, *height);
                    batch(|| {
                        let _: bool = self.get_context().get_breakpoint().set_if_changed(bp);
                        let _: bool = self
                            .get_context()
                            .get_viewport_size()
                            .set_if_changed(logical_size);
                    });
                }
                SystemEvent::SafeAreaChanged(insets) => {
                    self.get_mut_window_state().set_safe_area(*insets);
                    let _: bool = self.get_context().get_safe_area().set_if_changed(*insets);
                    self.set_pending_damage(DamageRegion::full());
                }
                SystemEvent::OrientationChanged(orient) => {
                    let _: bool = self.get_context().get_orientation().set_if_changed(*orient);
                }
                SystemEvent::ThemeChanged(mode) => {
                    self.get_mut_window_state().set_system_theme(*mode);
                    let _: bool = self.get_context().get_theme_mode().set_if_changed(*mode);
                    self.set_pending_damage(DamageRegion::full());
                }
                SystemEvent::LocaleChanged(locale) => {
                    let _: bool = self
                        .get_context()
                        .get_locale()
                        .set_if_changed(locale.clone());
                }
                SystemEvent::KeyboardVisibilityChanged { visible, height } => {
                    let _: bool = self
                        .get_context()
                        .get_keyboard_visible()
                        .set_if_changed(*visible);
                    let _: bool = self
                        .get_context()
                        .get_keyboard_height()
                        .set_if_changed(*height);
                    if !*visible {
                        self.set_last_ime_active(false);
                    }
                }
                SystemEvent::LowPowerModeChanged(enabled) => {
                    let _: bool = self
                        .get_context()
                        .get_low_power_mode()
                        .set_if_changed(*enabled);
                }
                SystemEvent::NetworkStatusChanged(status) => {
                    let _: bool = self
                        .get_context()
                        .get_network_status()
                        .set_if_changed(*status);
                }
                SystemEvent::BackPressed => {
                    let _: bool = self
                        .get_context()
                        .get_system_back()
                        .update(|v: &mut u64| *v = v.wrapping_add(1));
                    if self.get_context().get_exit_requested().get_untracked() && self.get_running()
                    {
                        let wid: WindowId = self.get_window_state().get_id();
                        self.queue_window_command(WindowCommand::Close(wid));
                        self.set_running(false);
                    }
                }
                SystemEvent::DeepLink(uri) => {
                    let already_same: bool = self
                        .get_deep_link()
                        .get_untracked()
                        .as_ref()
                        .map(|existing: &DeepLinkEvent| {
                            existing.get_raw_url().as_str() == uri.as_str()
                        })
                        .unwrap_or(false);
                    if !already_same && let Ok(parsed) = DeepLinkEvent::parse(uri.as_str()) {
                        let _: bool = self.get_deep_link().set(Some(parsed));
                    }
                }
                SystemEvent::Inactive | SystemEvent::Disappear | SystemEvent::Background => {
                    self.cancel_all_active_pointers();
                }
                SystemEvent::Terminate => {
                    self.cancel_all_active_pointers();
                    self.set_running(false);
                }
                SystemEvent::Appear | SystemEvent::Foreground => {
                    self.set_pending_damage(DamageRegion::full());
                }
                SystemEvent::MemoryWarning(_) | SystemEvent::ClipboardChanged => {}
            }
            let observers: Vec<(NodeId, Rc<dyn Lifecycle>)> = self
                .get_scene()
                .with_read(SceneTree::collect_visible_lifecycles, Vec::new());
            dispatch_lifecycle_system_event(&observers, &ev);
            dispatch_system_event_listeners(&ev);
        }
    }

    /// Copies the active scene text selection into the platform clipboard and returns the copied string.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable application.
    ///
    /// # Returns
    ///
    /// - `Option<String>` - Copied UTF-8 selection, or `None` when nothing is selected.
    pub fn copy_selection(&mut self) -> Option<String> {
        let selected: String = self.get_scene().get_selected_text()?;
        let _: Result<(), PlatformError> = self.write_clipboard(selected.as_str());
        Some(selected)
    }

    /// Hit-tests `pos` in logical screen coordinates against a `NodeKind::Text` node to resolve its character index.
    ///
    /// # Arguments
    ///
    /// - `&SceneTree` - Retained scene tree.
    /// - `NodeId` - Target text node handle.
    /// - `Point` - Screen-space logical pointer coordinates.
    ///
    /// # Returns
    ///
    /// - `usize` - Nearest character index in `0..=char_len`.
    #[must_use]
    fn hit_test_text_node_char(tree: &SceneTree, node_id: NodeId, pos: Point) -> usize {
        let Some(node) = tree.get_node(node_id) else {
            return 0;
        };
        let NodeKind::Text(content) = node.get_kind() else {
            return 0;
        };
        let lbox: LayoutBox = node.get_layout_box();
        let outer: Rect = lbox.get_bounds();
        let content_rect: Rect = lbox.get_content_bounds();
        let screen_outer: Rect = tree.node_screen_bounds(node_id).unwrap_or(outer);
        let screen_content: Rect = Rect::new(
            screen_outer.get_x() + (content_rect.get_x() - outer.get_x()),
            screen_outer.get_y() + (content_rect.get_y() - outer.get_y()),
            content_rect.get_width(),
            content_rect.get_height(),
        );
        hit_test_text_char(
            content.as_str(),
            node.get_style().get_text(),
            screen_content,
            pos,
        )
    }

    /// Synchronizes OS soft-keyboard / IME visibility with the currently focused `NodeKind::Input`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable application.
    fn sync_ime_focus(&mut self) {
        self.sync_ime_focus_forced(false);
    }

    /// Synchronizes OS soft-keyboard / IME visibility, optionally re-emitting `show` when re-tapping an already-focused input.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable application.
    /// - `bool` - When `true`, forces re-sending the OS IME show command even if the same input remained logically focused.
    fn sync_ime_focus_forced(&mut self, force_show: bool) {
        let (active_node, input_rect): (Option<NodeId>, Rect) = self.get_scene().with_read(
            |tree: &SceneTree| {
                let Some(fid) = tree.get_focus().focused_node() else {
                    return (None, Rect::default());
                };
                if !tree.is_node_visible(fid) {
                    return (None, Rect::default());
                }
                let Some(node) = tree.get_node(fid) else {
                    return (None, Rect::default());
                };
                if let NodeKind::Input(inp) = node.get_kind()
                    && !inp.get_disabled()
                {
                    let rect: Rect = tree
                        .node_screen_bounds(fid)
                        .unwrap_or_else(|| node.get_layout_box().get_bounds());
                    return (Some(fid), rect);
                }
                (None, Rect::default())
            },
            (None, Rect::default()),
        );
        self.apply_ime_focus_state(active_node, input_rect, force_show);
    }

    /// Applies pre-resolved IME focus state to the platform backend without re-borrowing `SceneTree`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable application.
    /// - `Option<NodeId>` - Active visible non-disabled focused input node, if any.
    /// - `Rect` - Screen-space logical bounds of the focused input node.
    /// - `bool` - When `true`, forces re-emitting the OS IME show command.
    fn apply_ime_focus_state(
        &mut self,
        active_node: Option<NodeId>,
        input_rect: Rect,
        force_show: bool,
    ) {
        let next_active: bool = active_node.is_some();
        let prev_active: bool = self.get_last_ime_active();
        let prev_node: Option<NodeId> = {
            let Self { last_ime_node, .. } = self;
            *last_ime_node
        };
        if !next_active {
            if !prev_active && prev_node.is_none() {
                return;
            }
            self.set_last_ime_active(false);
            self.set_last_ime_node(None);
            if !prev_active {
                return;
            }
            let wid: WindowId = self.get_window_state().get_id();
            let cmd: WindowCommand = WindowCommand::SetIme(wid, false, Rect::default());
            let Self { platform, .. } = self;
            let _: Result<(), PlatformError> = platform.apply_command(cmd);
            let _: bool = dispatch_native_action(NATIVE_ACTION_IME, IME_ACTION_HIDE, "");
            return;
        }
        let same_node: bool = active_node == prev_node;
        if same_node && !force_show {
            return;
        }
        self.set_last_ime_active(true);
        self.set_last_ime_node(active_node);
        let wid: WindowId = self.get_window_state().get_id();
        let cmd: WindowCommand = WindowCommand::SetIme(wid, true, input_rect);
        let Self { platform, .. } = self;
        let _: Result<(), PlatformError> = platform.apply_command(cmd);
        let rect_csv: String = format!(
            "{:.1},{:.1},{:.1},{:.1}",
            input_rect.get_x(),
            input_rect.get_y(),
            input_rect.get_width(),
            input_rect.get_height()
        );
        let _: bool = dispatch_native_action(NATIVE_ACTION_IME, IME_ACTION_SHOW, &rect_csv);
    }

    /// Configures the cross-platform maximum frame rate (`None` = unlimited maximum FPS, the default).
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable application.
    /// - `Option<u32>` - Target FPS cap (`Some(1..=1000)` or `None` for uncapped maximum FPS).
    pub fn set_fps_limit(&mut self, limit: Option<u32>) {
        let normalized: Option<u32> = limit.map(|v: u32| v.clamp(1, 1000));
        let _: bool = self
            .get_context()
            .get_fps_limit()
            .set_if_changed(normalized);
        self.set_fps_budget_accum(0.0);
    }

    /// Returns the active cross-platform FPS limit (`None` = unlimited maximum FPS).
    ///
    /// # Arguments
    ///
    /// - `&Self` - Application reference.
    ///
    /// # Returns
    ///
    /// - `Option<u32>` - Active frame rate cap, or `None` when uncapped.
    #[must_use]
    pub fn fps_limit(&self) -> Option<u32> {
        self.get_context().get_fps_limit().get_untracked()
    }

    /// Registers a `GameEngineHost` so all incoming OS/Web `WindowEvent`s are automatically forwarded to its `InputState`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable application.
    /// - `GameEngineHost` - Game engine host handle to attach.
    pub fn attach_game_host(&mut self, host: GameEngineHost) {
        self.get_mut_game_hosts().push(host);
    }

    /// Returns `true` when any node transition animation, inertial scroll, or blinking text caret is actively running.
    ///
    /// # Arguments
    ///
    /// - `&Self` - App reference.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether animations need continuous frame stepping.
    #[must_use]
    pub fn has_active_animations(&self) -> bool {
        if self.get_scene().has_active_transitions() {
            return true;
        }
        let vel: Point = self.get_scroll_velocity();
        if vel.get_x().hypot(vel.get_y()) > 1.5 {
            return true;
        }
        let smooth: Point = self.get_smooth_scroll_pending();
        if smooth.get_x().hypot(smooth.get_y()) > 0.05 {
            return true;
        }
        self.get_scene().with_read(
            |tree: &SceneTree| {
                tree.get_focus()
                    .focused_node()
                    .and_then(|id: NodeId| tree.get_node(id))
                    .map(|n: &SceneNode| matches!(n.get_kind(), NodeKind::Input(_)))
                    .unwrap_or(false)
            },
            false,
        )
    }

    /// Mounts a root `Component` or `NodeId` into the application's scene root with an ambient `Context`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable application.
    /// - `F` - Zero-argument root builder closure invoking `view!` or components.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted root content node handle.
    pub fn mount<F, C>(&mut self, builder: F) -> NodeId
    where
        F: FnOnce() -> C,
        C: Component,
    {
        let ctx: Context = self.get_context().clone();
        let root_child: NodeId = with_context(ctx, || {
            let component: C = builder();
            component.mount()
        });
        let scene: SceneHandle = self.get_scene().clone();
        let root: NodeId = scene.with_read(SceneTree::get_root, NodeId::default());
        scene.replace_children_safe(root, vec![root_child]);
        scene.activate_subtree(root_child);
        self.set_pending_damage(DamageRegion::full());
        root_child
    }

    /// Returns the active `PlatformKind`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - App reference.
    ///
    /// # Returns
    ///
    /// - `PlatformKind` - Active execution platform.
    #[must_use]
    pub fn platform_kind(&self) -> PlatformKind {
        let Self { platform, .. } = self;
        platform.kind()
    }

    /// Returns the active `PlatformCapabilities`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - App reference.
    ///
    /// # Returns
    ///
    /// - `PlatformCapabilities` - Hardware/OS capability matrix.
    #[must_use]
    pub fn platform_capabilities(&self) -> PlatformCapabilities {
        let Self { platform, .. } = self;
        platform.capabilities()
    }

    /// Returns the `RawWindowHandle` of the primary window.
    ///
    /// # Arguments
    ///
    /// - `&Self` - App reference.
    ///
    /// # Returns
    ///
    /// - `RawWindowHandle` - Native OS window pointer bundle.
    #[must_use]
    pub fn raw_window_handle(&self) -> RawWindowHandle {
        let Self {
            platform,
            window_state,
            ..
        } = self;
        platform.raw_window_handle(window_state.get_id())
    }

    /// Compiles a fresh cross-platform `A11yTree` snapshot from the active retained `SceneTree`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - App reference.
    ///
    /// # Returns
    ///
    /// - `A11yTree` - Compiled accessibility tree with laid-out bounds and focus state.
    #[must_use]
    pub fn a11y_tree(&self) -> A11yTree {
        self.get_scene()
            .with_read(A11yTree::from_scene, A11yTree::default())
    }

    /// Dynamically switches the rendering pipeline mode (`Auto`, `Software`, `Hardware`, `Hybrid`).
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable app.
    /// - `RenderMode` - Target render mode.
    pub fn set_render_mode(&mut self, mode: RenderMode) {
        let kind: PlatformKind = self.platform_kind();
        let caps: PlatformCapabilities = self.platform_capabilities();
        self.get_mut_renderer().switch_mode(kind, caps, mode);
        let id: WindowId = self.get_window_state().get_id();
        self.get_mut_command_queue()
            .push(WindowCommand::SetRenderMode(id, mode));
        self.set_pending_damage(DamageRegion::full());
    }

    /// Queues a `WindowCommand` for immediate dispatch on the next frame tick.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable app.
    /// - `WindowCommand` - Command to queue.
    pub fn queue_window_command(&mut self, command: WindowCommand) {
        self.get_mut_command_queue().push(command);
    }

    /// Sets exact physical pixel surface dimensions and DPI scale without truncating fractional logical sizes.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable app.
    /// - `u32` - Exact physical surface width in device pixels.
    /// - `u32` - Exact physical surface height in device pixels.
    /// - `f32` - Active display DPI scale factor.
    pub fn set_exact_physical_viewport(&mut self, phys_w: u32, phys_h: u32, scale: f32) {
        let safe_scale: f32 = if scale.is_finite() {
            scale.clamp(0.5, 4.5)
        } else {
            1.0
        };
        let pw: u32 = phys_w.max(1);
        let ph: u32 = phys_h.max(1);
        let lw: f32 = ((pw as f32) / safe_scale).max(1.0);
        let lh: f32 = ((ph as f32) / safe_scale).max(1.0);
        self.get_mut_window_state().set_logical_width(lw);
        self.get_mut_window_state().set_logical_height(lh);
        self.get_mut_window_state().set_scale_factor(safe_scale);
        self.get_mut_window_state().set_physical_width(pw);
        self.get_mut_window_state().set_physical_height(ph);
        let bp: Breakpoint = self.get_window_state().breakpoint();
        let next_size: Size = self.get_window_state().logical_size();
        batch(|| {
            let _: bool = self.get_context().get_breakpoint().set_if_changed(bp);
            let _: bool = self
                .get_context()
                .get_viewport_size()
                .set_if_changed(next_size);
        });
        let scene: SceneHandle = self.get_scene().clone();
        scene.with_write(
            |tree: &mut SceneTree| {
                let root: NodeId = tree.get_root();
                let layout_bits: u16 = DIRTY_LAYOUT | DIRTY_SUBTREE_LAYOUT;
                tree.mark_dirty(root, layout_bits | DIRTY_PAINT);
            },
            (),
        );
        self.set_pending_damage(DamageRegion::full());
        self.dispatch_system_event(SystemEvent::Resize {
            width: lw,
            height: lh,
            scale_factor: safe_scale,
        });
    }

    /// Processes a single incoming `WindowEvent`, updating state, hit-testing, and dispatching UI callbacks.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable app.
    /// - `WindowEvent` - Event received from the OS or test harness.
    pub fn dispatch_window_event(&mut self, event: WindowEvent) {
        self.get_mut_input_state().ingest(&event);
        for host in self.get_game_hosts().iter() {
            host.ingest_event(&event);
        }
        match event {
            WindowEvent::Resized(wid, width, height, scale) => {
                let safe_scale: f32 = scale.max(0.5);
                let exact_plat: Option<(u32, u32)> = {
                    let Self { platform, .. } = self;
                    platform.window_state(wid).and_then(|st: WindowState| {
                        let pw: u32 = st.get_physical_width();
                        let ph: u32 = st.get_physical_height();
                        let rw: u32 = ((pw as f32) / safe_scale).round().max(1.0) as u32;
                        let rh: u32 = ((ph as f32) / safe_scale).round().max(1.0) as u32;
                        if (st.get_scale_factor() - safe_scale).abs() < 0.001
                            && rw == width.max(1)
                            && rh == height.max(1)
                        {
                            Some((pw, ph))
                        } else {
                            None
                        }
                    })
                };
                if let Some((pw, ph)) = exact_plat {
                    self.set_exact_physical_viewport(pw, ph, safe_scale);
                } else {
                    self.get_mut_window_state()
                        .set_logical_width(width.max(1) as f32);
                    self.get_mut_window_state()
                        .set_logical_height(height.max(1) as f32);
                    self.get_mut_window_state().set_scale_factor(safe_scale);
                    self.get_mut_window_state()
                        .set_physical_width(((width.max(1) as f32) * safe_scale).round() as u32);
                    self.get_mut_window_state()
                        .set_physical_height(((height.max(1) as f32) * safe_scale).round() as u32);
                    let bp: Breakpoint = self.get_window_state().breakpoint();
                    let next_size: Size = self.get_window_state().logical_size();
                    batch(|| {
                        let _: bool = self.get_context().get_breakpoint().set_if_changed(bp);
                        let _: bool = self
                            .get_context()
                            .get_viewport_size()
                            .set_if_changed(next_size);
                    });
                    let scene: SceneHandle = self.get_scene().clone();
                    scene.with_write(
                        |tree: &mut SceneTree| {
                            let root: NodeId = tree.get_root();
                            let layout_bits: u16 = DIRTY_LAYOUT | DIRTY_SUBTREE_LAYOUT;
                            tree.mark_dirty(root, layout_bits | DIRTY_PAINT);
                        },
                        (),
                    );
                    self.set_pending_damage(DamageRegion::full());
                }
                let ws: WindowState = self.get_window_state();
                self.dispatch_system_event(SystemEvent::Resize {
                    width: ws.get_logical_width(),
                    height: ws.get_logical_height(),
                    scale_factor: ws.get_scale_factor(),
                });
            }
            WindowEvent::SafeAreaChanged(_, insets) => {
                self.dispatch_system_event(SystemEvent::SafeAreaChanged(insets));
            }
            WindowEvent::ThemeChanged(_, mode) => {
                self.dispatch_system_event(SystemEvent::ThemeChanged(mode));
            }
            WindowEvent::Focused(_, focused) => {
                self.dispatch_system_event(SystemEvent::FocusChanged(focused));
            }
            WindowEvent::CloseRequested(_) => {
                self.dispatch_system_event(SystemEvent::Terminate);
                self.set_running(false);
            }
            WindowEvent::PointerMove(_, ptr) => {
                self.handle_pointer_move_event(ptr);
            }
            WindowEvent::PointerDown(_, ptr) => {
                self.handle_pointer_down_event(ptr);
            }
            WindowEvent::PointerUp(_, ptr) => {
                self.handle_pointer_up_event(ptr);
            }
            WindowEvent::Scroll(_, delta) => {
                self.handle_scroll(delta);
            }
            WindowEvent::KeyDown(_, key) => {
                self.handle_key_down(key);
            }
            WindowEvent::TextInput(_, text) => {
                self.handle_text_input(text);
            }
            WindowEvent::Ime(_, ImeEvent::Commit(text)) => {
                self.handle_text_input(text);
            }
            WindowEvent::Ime(_, ImeEvent::Disabled) => {
                self.set_last_ime_active(false);
                self.dispatch_system_event(SystemEvent::KeyboardVisibilityChanged {
                    visible: false,
                    height: 0.0,
                });
            }
            WindowEvent::Lifecycle(lc) => match lc {
                LifecycleEvent::Resumed => {
                    self.dispatch_system_event(SystemEvent::Foreground);
                }
                LifecycleEvent::Paused => {
                    self.dispatch_system_event(SystemEvent::Background);
                }
                LifecycleEvent::LowMemory => {
                    self.dispatch_system_event(SystemEvent::MemoryWarning(
                        MemoryPressure::Critical,
                    ));
                }
                LifecycleEvent::Terminated => {
                    self.dispatch_system_event(SystemEvent::Terminate);
                    self.set_running(false);
                }
            },
            WindowEvent::RedrawRequested(_) => {
                self.set_pending_damage(DamageRegion::full());
            }
            WindowEvent::DeepLink(link) => {
                let uri: String = link.get_raw_url().clone();
                let _: bool = self.get_deep_link().set(Some(link));
                self.dispatch_system_event(SystemEvent::DeepLink(uri));
            }
            WindowEvent::System(sys_event) => {
                self.dispatch_system_event(sys_event);
            }
            WindowEvent::PointerCancel(_, pointer_id) => {
                if pointer_id == 0 {
                    self.cancel_all_active_pointers();
                } else {
                    self.handle_pointer_cancel_single(pointer_id);
                }
            }
            WindowEvent::KeyUp(_, _) | WindowEvent::Ime(_, _) => {}
        }
    }

    /// Executes one complete frame tick: poll events, flush commands, layout, paint, render, and present.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable app.
    ///
    /// # Returns
    ///
    /// - `Result<FrameStats, PlatformError>` - Performance and pixel telemetry for this frame.
    pub fn step_frame(&mut self) -> Result<FrameStats, PlatformError> {
        if !self.get_command_queue().is_empty() {
            let commands: Vec<WindowCommand> = self.get_mut_command_queue().drain();
            for cmd in commands.into_iter() {
                let Self { platform, .. } = self;
                platform.apply_command(cmd)?;
            }
        }
        let events: Vec<WindowEvent> = {
            let Self { platform, .. } = self;
            platform.poll_events()
        };
        if !events.is_empty() {
            for ev in events.into_iter() {
                self.dispatch_window_event(ev);
            }
        }
        flush_effects();
        let raw_dt: f32 = self.get_frame_clock().get_delta_secs();
        let native_hz: f32 = self.get_display_refresh_rate_hz().clamp(24.0, 480.0);
        let anim_dt: f32 = if raw_dt > 0.0001 {
            raw_dt.clamp(1.0 / 480.0, 0.05)
        } else {
            1.0 / native_hz
        };
        if let Some(fps_cap) = self.fps_limit() {
            let min_dt: f32 = 1.0 / (fps_cap.max(1) as f32);
            let accum: f32 = self.get_fps_budget_accum() + anim_dt;
            if accum + 0.0005 < min_dt
                && self.get_last_rendered_revision() > 0
                && !self.get_pending_damage().get_full_damage()
            {
                self.set_fps_budget_accum(accum);
                return Ok(self.get_mut_renderer().record_skipped_frame());
            }
            self.set_fps_budget_accum(0.0);
        } else if self.get_fps_budget_accum() != 0.0 {
            self.set_fps_budget_accum(0.0);
        }
        let next_caret_phase: f32 = (self.get_caret_blink_secs() + anim_dt) % 1.06;
        let next_caret_vis: bool = next_caret_phase < 0.56;
        let vel_before: Point = self.get_scroll_velocity();
        let smooth_before: Point = self.get_smooth_scroll_pending();
        let vel_moving_before: bool = vel_before.get_x().hypot(vel_before.get_y()) > 1.5;
        let smooth_moving_before: bool = smooth_before.get_x().hypot(smooth_before.get_y()) > 0.05;
        let scroll_motion_before: bool = vel_moving_before || smooth_moving_before;
        let clamp_first_anim_frame: bool =
            self.platform_kind() != PlatformKind::Headless && !self.get_was_animating();
        let FramePreTick {
            has_focused_input,
            has_ime_node,
            ime_node_id,
            ime_rect,
            game_damage_rects,
            visible_portals,
            active_host_indices,
            step_dt,
            mut anim_ticked,
            pending_game_transitions,
        } = {
            let Self {
                scene, game_hosts, ..
            } = self;
            scene.with_write(
                |tree: &mut SceneTree| {
                    let mut focused_input: bool = false;
                    let mut ime_found: bool = false;
                    let mut ime_target_id: NodeId = NodeId::default();
                    let mut ime_bounds: Rect = Rect::default();
                    if let Some(fid) = tree.get_focus().focused_node()
                        && let Some(node) = tree.get_node(fid)
                        && let NodeKind::Input(inp) = node.get_kind()
                    {
                        focused_input = true;
                        let is_enabled: bool = !inp.get_disabled();
                        if is_enabled && tree.is_node_visible(fid) {
                            ime_bounds = tree
                                .node_screen_bounds(fid)
                                .unwrap_or_else(|| node.get_layout_box().get_bounds());
                            ime_target_id = fid;
                            ime_found = true;
                        }
                        if tree.get_caret_visible() != next_caret_vis {
                            tree.set_caret_visible(next_caret_vis);
                            tree.mark_dirty(fid, DIRTY_PAINT);
                        }
                    }
                    let registered_portals: &[NodeId] = tree.portal_nodes();
                    let mut portals: Vec<(bool, Rect)> = Vec::new();
                    if !registered_portals.is_empty() {
                        portals.reserve(registered_portals.len());
                        for &nid in registered_portals {
                            if let Some(n) = tree.get_node(nid)
                                && tree.is_node_visible(nid)
                            {
                                let is_dirty: bool = n.get_dirty().get_bits() != DIRTY_NONE;
                                let bounds: Rect = tree
                                    .node_screen_bounds(nid)
                                    .unwrap_or_else(|| n.get_layout_box().get_bounds());
                                portals.push((is_dirty, bounds));
                            }
                        }
                    }
                    let mut rects: Vec<Rect> = Vec::new();
                    let mut active_hosts: Vec<usize> = Vec::new();
                    if !game_hosts.is_empty() {
                        for (host_idx, host) in game_hosts.iter().enumerate() {
                            let host_rects: Vec<Rect> = host.active_viewport_bounds(tree);
                            if !host_rects.is_empty() {
                                active_hosts.push(host_idx);
                                rects.extend(host_rects);
                            }
                        }
                    }
                    let has_transitions: bool = tree.has_active_transitions();
                    let has_game_or_trans: bool = !active_hosts.is_empty() || has_transitions;
                    let motion_before: bool = has_game_or_trans || scroll_motion_before;
                    let resolved_dt: f32 = if clamp_first_anim_frame && motion_before {
                        anim_dt.min(1.0 / native_hz)
                    } else {
                        anim_dt
                    };
                    let (ticked, deferred_trans): (bool, bool) = if active_hosts.is_empty() {
                        if has_transitions {
                            (tree.tick_transitions(resolved_dt), false)
                        } else {
                            (false, false)
                        }
                    } else {
                        (false, has_transitions)
                    };
                    FramePreTick {
                        has_focused_input: focused_input,
                        has_ime_node: ime_found,
                        ime_node_id: ime_target_id,
                        ime_rect: ime_bounds,
                        game_damage_rects: rects,
                        visible_portals: portals,
                        active_host_indices: active_hosts,
                        step_dt: resolved_dt,
                        anim_ticked: ticked,
                        pending_game_transitions: deferred_trans,
                    }
                },
                FramePreTick {
                    step_dt: anim_dt,
                    ..FramePreTick::default()
                },
            )
        };
        if has_focused_input {
            self.set_caret_blink_secs(next_caret_phase);
        } else if self.get_caret_blink_secs() > 0.0 {
            self.set_caret_blink_secs(0.0);
        }
        let ime_node: Option<NodeId> = if has_ime_node {
            Some(ime_node_id)
        } else {
            None
        };
        self.apply_ime_focus_state(ime_node, ime_rect, false);
        let has_active_game: bool = !active_host_indices.is_empty();
        if has_active_game {
            for host_idx in active_host_indices.into_iter() {
                if let Some(host) = self.get_game_hosts().get(host_idx) {
                    let _: u32 = host.step(step_dt);
                }
            }
            if pending_game_transitions {
                let Self { scene, .. } = self;
                anim_ticked =
                    scene.with_write(|tree: &mut SceneTree| tree.tick_transitions(step_dt), false);
            }
        }
        self.get_context()
            .advance_clock_ms(((step_dt.max(0.0)) * 1000.0).round() as u64);
        if self.get_context().get_exit_requested().get_untracked() && self.get_running() {
            let wid: WindowId = self.get_window_state().get_id();
            self.queue_window_command(WindowCommand::Close(wid));
            self.set_running(false);
        }
        self.tick_gesture_timers(step_dt);
        self.sample_touch_drag_velocity(step_dt);
        self.tick_smooth_scroll(step_dt);
        self.tick_scroll_inertia(step_dt);
        let viewport: Size = self.get_window_state().logical_size();
        let scale: f32 = self.get_window_state().get_scale_factor();
        let clear: Color = self.get_config().get_clear_color();
        let mut damage: DamageRegion = self.get_pending_damage();
        let (scene_rev, tree_damage, has_transitions_after): (u64, DamageRegion, bool) = {
            let Self { scene, .. } = self;
            scene.with_write(
                |tree: &mut SceneTree| {
                    let d: DamageRegion = tree.collect_paint_damage(scale);
                    (tree.get_revision(), d, tree.has_active_transitions())
                },
                (0, DamageRegion::full(), false),
            )
        };
        let vel_after: Point = self.get_scroll_velocity();
        let smooth_after: Point = self.get_smooth_scroll_pending();
        let vel_moving_after: bool = vel_after.get_x().hypot(vel_after.get_y()) > 1.5;
        let smooth_moving_after: bool = smooth_after.get_x().hypot(smooth_after.get_y()) > 0.05;
        let active_scroll_after: bool = vel_moving_after || smooth_moving_after;
        let active_anim_after: bool = has_active_game || has_transitions_after;
        self.set_was_animating(active_anim_after || active_scroll_after);
        if tree_damage.get_full_damage() || scene_rev != self.get_last_rendered_revision() {
            if tree_damage.get_full_damage() {
                damage = DamageRegion::full();
            } else if let Some(rect) = tree_damage.bounds_rect() {
                damage.add_rect(rect);
            } else if damage.resolve(1, 1).is_none() {
                damage = DamageRegion::full();
            }
        } else if let Some(rect) = tree_damage.bounds_rect() {
            damage.add_rect(rect);
        }
        if anim_ticked
            && !damage.get_full_damage()
            && let Some(prev_rect) = self.get_last_frame_damage().bounds_rect()
        {
            damage.add_rect(prev_rect);
        }
        if has_active_game && !damage.get_full_damage() {
            for r in game_damage_rects.iter() {
                damage.add_rect(Rect::new(
                    (r.get_x() * scale).floor(),
                    (r.get_y() * scale).floor(),
                    (r.get_width() * scale).ceil() + 2.0,
                    (r.get_height() * scale).ceil() + 2.0,
                ));
            }
        }
        let phys_w: u32 = ((viewport.get_width() * scale).round() as u32).max(1);
        let phys_h: u32 = ((viewport.get_height() * scale).round() as u32).max(1);
        let damage_rect_opt: Option<Rect> = damage.resolve(phys_w, phys_h);
        if damage_rect_opt.is_none() {
            return Ok(self.get_mut_renderer().record_skipped_frame());
        }
        let animated_or_game: bool = anim_ticked || has_active_game;
        let portal_dirty: bool = !visible_portals.is_empty()
            && visible_portals
                .iter()
                .any(|(is_dirty, logical_rect): &(bool, Rect)| {
                    if *is_dirty || animated_or_game {
                        return true;
                    }
                    if let Some(dmg) = damage_rect_opt {
                        let phys_portal: Rect = Rect::new(
                            logical_rect.get_x() * scale,
                            logical_rect.get_y() * scale,
                            logical_rect.get_width() * scale,
                            logical_rect.get_height() * scale,
                        );
                        dmg.intersect(phys_portal).is_some()
                    } else {
                        false
                    }
                });
        if portal_dirty
            || !self
                .get_renderer()
                .get_config()
                .get_dirty_rect_optimization()
        {
            damage = DamageRegion::full();
        }
        let layout_stats: LayoutStats = {
            let Self {
                scene,
                layout_engine,
                ..
            } = self;
            scene.with_write(
                |tree: &mut SceneTree| layout_engine.compute(tree, viewport),
                LayoutStats::default(),
            )
        };
        if layout_stats.get_nodes_computed() > 0 {
            damage = DamageRegion::full();
        }
        let frame_stats: FrameStats = {
            let Self {
                scene, renderer, ..
            } = self;
            scene.with_read(
                |tree: &SceneTree| {
                    renderer.render_frame(tree, viewport, scale, clear, layout_stats, damage)
                },
                Ok(FrameStats::default()),
            )?
        };
        let window_id: WindowId = self.get_window_state().get_id();
        {
            let Self {
                platform, renderer, ..
            } = self;
            if let Some(fb) = renderer.framebuffer() {
                platform.present_framebuffer_region(window_id, fb, damage)?;
            }
        }
        self.set_last_frame_damage(damage);
        self.set_last_rendered_revision(scene_rev);
        self.set_pending_damage(DamageRegion::default());
        let next_fps_secs: f32 = self.get_fps_accum_secs() + anim_dt;
        let next_fps_frames: u32 = self.get_fps_accum_frames().saturating_add(1);
        if next_fps_secs >= 0.35 {
            let measured: f32 = ((next_fps_frames as f32) / next_fps_secs).clamp(1.0, 999.0);
            let _: bool = self
                .get_context()
                .get_live_fps()
                .set_if_changed((measured * 2.0).round() * 0.5);
            self.set_fps_accum_secs(0.0);
            self.set_fps_accum_frames(0);
        } else {
            self.set_fps_accum_secs(next_fps_secs);
            self.set_fps_accum_frames(next_fps_frames);
        }
        Ok(frame_stats)
    }

    /// Advances the internal `FrameClock` by `dt_secs`, executes one full UI/game frame, and resets transient input deltas.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable app.
    /// - `f32` - Elapsed seconds since the previous frame.
    ///
    /// # Returns
    ///
    /// - `Result<FrameStats, PlatformError>` - Frame telemetry for the rendered frame.
    pub fn step_game_frame(&mut self, dt_secs: f32) -> Result<FrameStats, PlatformError> {
        let _: u32 = self.get_mut_frame_clock().tick(dt_secs);
        let stats: FrameStats = self.step_frame()?;
        self.get_mut_input_state().begin_frame();
        Ok(stats)
    }

    /// CoreFoundation run-loop timer callback keeping macOS live window border dragging repainting smoothly at 60Hz.
    ///
    /// # Arguments
    ///
    /// - `*mut c_void` - Unused `CFRunLoopTimerRef`.
    /// - `*mut c_void` - Pointer to the active `App`.
    #[cfg(target_os = "macos")]
    unsafe extern "C" fn macos_live_resize_tick(_timer: *mut c_void, info: *mut c_void) {
        if info.is_null() {
            return;
        }
        // SAFETY: `info` points to the live `App` on the main thread for the exact duration of `run_frames`.
        let app: &mut Self = unsafe { &mut *info.cast::<Self>() };
        if !app.get_running() || app.get_in_step_frame() {
            return;
        }
        app.set_in_step_frame(true);
        let _: Result<FrameStats, PlatformError> = app.step_frame();
        app.set_in_step_frame(false);
    }

    /// Runs the interactive frame loop until `max_frames` is reached or the window closes.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable app.
    /// - `Option<u64>` - Optional frame limit (`None` loops until `CloseRequested` or termination).
    ///
    /// # Returns
    ///
    /// - `Result<FrameStats, PlatformError>` - Telemetry from the final rendered frame.
    pub fn run_frames(&mut self, max_frames: Option<u64>) -> Result<FrameStats, PlatformError> {
        #[cfg(target_os = "macos")]
        let live_timer: *mut c_void =
            if max_frames.is_none() && self.platform_kind() == PlatformKind::MacOs {
                macos_install_live_resize_timer(
                    Self::macos_live_resize_tick,
                    (self as *mut Self).cast(),
                )
            } else {
                null_mut()
            };
        let mut last_stats: FrameStats = FrameStats::default();
        let mut rendered: u64 = 0;
        let mut loop_err: Option<PlatformError> = None;
        #[cfg(not(target_arch = "wasm32"))]
        let mut prev_instant: Instant = Instant::now();
        while self.get_running() {
            #[cfg(not(target_arch = "wasm32"))]
            let frame_start: Instant = Instant::now();
            #[cfg(not(target_arch = "wasm32"))]
            if max_frames.is_none() {
                let real_dt: f32 = frame_start
                    .duration_since(prev_instant)
                    .as_secs_f32()
                    .clamp(1.0 / 480.0, 0.05);
                prev_instant = frame_start;
                let _: u32 = self.get_mut_frame_clock().tick(real_dt);
            }
            self.set_in_step_frame(true);
            let step_res: Result<FrameStats, PlatformError> = self.step_frame();
            self.set_in_step_frame(false);
            match step_res {
                Ok(stats) => last_stats = stats,
                Err(err) => {
                    loop_err = Some(err);
                    break;
                }
            }
            rendered = rendered.saturating_add(1);
            if let Some(limit) = max_frames
                && rendered >= limit
            {
                break;
            }
            if !self.get_running() {
                break;
            }
            #[cfg(not(target_arch = "wasm32"))]
            if max_frames.is_none() {
                let effective_hz: f64 = self
                    .fps_limit()
                    .map(|cap: u32| f64::from(cap.max(1)))
                    .unwrap_or_else(|| {
                        f64::from(self.get_display_refresh_rate_hz().clamp(24.0, 480.0))
                    });
                let target_budget: Duration = Duration::from_secs_f64(1.0 / effective_hz);
                let elapsed: Duration = frame_start.elapsed();
                if let Some(remaining) = target_budget.checked_sub(elapsed) {
                    if self.get_was_animating() {
                        let spin_guard: Duration = Duration::from_micros(1500);
                        if let Some(coarse) = remaining.checked_sub(spin_guard) {
                            sleep(coarse);
                        }
                        while frame_start.elapsed() < target_budget {
                            spin_loop();
                        }
                    } else {
                        sleep(remaining);
                    }
                }
            }
        }
        #[cfg(target_os = "macos")]
        if !live_timer.is_null() {
            macos_remove_live_resize_timer(live_timer);
        }
        if let Some(err) = loop_err {
            return Err(err);
        }
        Ok(last_stats)
    }

    /// Cancels all active pointer/touch contacts and resets transient gesture, drag, and pressed UI state.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable app.
    pub fn cancel_all_active_pointers(&mut self) {
        let had_pointers: bool = !self.get_active_pointers().is_empty();
        self.get_mut_active_pointers().clear();
        self.get_mut_active_pointer_devices().clear();
        if had_pointers {
            self.publish_active_touches();
        }
        self.set_gesture_origin(None);
        self.set_gesture_suppressed(false);
        self.set_pinch_base_dist(0.0);
        self.set_gesture_hold_secs(0.0);
        self.set_long_press_fired(false);
        self.set_touch_drag_pointer_id(None);
        self.set_touch_drag_origin(None);
        self.set_touch_drag_last(None);
        self.set_touch_dragging(false);
        self.set_touch_drag_travel(0.0);
        self.set_touch_scroll_axis(0);
        self.set_touch_vel_accum_delta(Point::default());
        self.get_mut_touch_vel_samples().clear();
        self.set_touch_vel_idle_secs(0.0);
        self.set_scrollbar_dragging(false);
        self.set_scrollbar_drag_axis(SCROLLBAR_AXIS_NONE);
        self.set_text_selecting_node(None);
        self.set_smooth_scroll_pending(Point::default());
        self.set_smooth_scroll_target(None);
        self.clear_pressed_node();
    }

    /// Reconciles active touch contacts against the OS ground-truth live pointer ID list, pruning any stale/missed touches.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable app.
    /// - `&[u64]` - Currently active OS touch pointer identifiers.
    pub fn reconcile_active_touch_ids(&mut self, live_ids: &[u64]) {
        let stale_ids: Vec<u64> = self
            .get_active_pointers()
            .keys()
            .copied()
            .filter(|id: &u64| {
                self.get_active_pointer_devices().get(id) == Some(&PointerDevice::Touch)
                    && !live_ids.contains(id)
            })
            .collect();
        if stale_ids.is_empty() {
            return;
        }
        for stale_id in stale_ids.into_iter() {
            let _: Option<Point> = self.get_mut_active_pointers().remove(&stale_id);
            let _: Option<PointerDevice> = self.get_mut_active_pointer_devices().remove(&stale_id);
        }
        self.publish_active_touches();
        if self.get_active_pointers().is_empty() {
            self.set_gesture_origin(None);
            self.set_gesture_suppressed(false);
            self.set_pinch_base_dist(0.0);
            self.set_gesture_hold_secs(0.0);
            self.set_long_press_fired(false);
            self.set_touch_drag_pointer_id(None);
            self.set_touch_drag_origin(None);
            self.set_touch_drag_last(None);
            self.set_touch_dragging(false);
            self.set_touch_drag_travel(0.0);
            self.set_touch_scroll_axis(0);
            self.set_touch_vel_accum_delta(Point::default());
            self.get_mut_touch_vel_samples().clear();
            self.set_touch_vel_idle_secs(0.0);
            self.set_scrollbar_dragging(false);
            self.set_scrollbar_drag_axis(SCROLLBAR_AXIS_NONE);
            self.set_text_selecting_node(None);
            self.clear_pressed_node();
        } else {
            self.sync_remaining_pointer_state();
        }
    }

    /// Clears any currently pressed scene node and marks it dirty so visual press states never stick.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable app.
    fn clear_pressed_node(&mut self) {
        let scene: SceneHandle = self.get_scene().clone();
        scene.with_write(
            |tree: &mut SceneTree| {
                if let Some(prev_id) = tree.get_focus().pressed_node() {
                    tree.get_mut_focus().set_pressed(None);
                    if let Some(node) = tree.get_node_mut(prev_id) {
                        node.get_mut_interaction().set_pressed(false);
                    }
                    tree.mark_dirty(prev_id, DIRTY_PAINT);
                }
            },
            (),
        );
    }

    /// Cancels a single active pointer identifier and cleanly hands off remaining contacts without freezing.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable app.
    /// - `u64` - Cancelled pointer identifier.
    fn handle_pointer_cancel_single(&mut self, pointer_id: u64) {
        let removed_pt: bool = self.get_mut_active_pointers().remove(&pointer_id).is_some();
        let _: Option<PointerDevice> = self.get_mut_active_pointer_devices().remove(&pointer_id);
        if removed_pt {
            self.publish_active_touches();
        }
        if self.get_active_pointers().is_empty() {
            self.set_gesture_origin(None);
            self.set_gesture_suppressed(false);
            self.set_pinch_base_dist(0.0);
            self.set_gesture_hold_secs(0.0);
            self.set_long_press_fired(false);
            self.set_touch_drag_pointer_id(None);
            self.set_touch_drag_origin(None);
            self.set_touch_drag_last(None);
            self.set_touch_dragging(false);
            self.set_touch_drag_travel(0.0);
            self.set_touch_scroll_axis(0);
            self.set_touch_vel_accum_delta(Point::default());
            self.get_mut_touch_vel_samples().clear();
            self.set_touch_vel_idle_secs(0.0);
            self.set_scrollbar_dragging(false);
            self.set_scrollbar_drag_axis(SCROLLBAR_AXIS_NONE);
            self.set_text_selecting_node(None);
            self.clear_pressed_node();
        } else {
            self.sync_remaining_pointer_state();
        }
    }

    /// Re-anchors pinch baseline or single-finger touch scroll state when transitioning from N pointers to N-1.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable app.
    fn sync_remaining_pointer_state(&mut self) {
        if self.get_active_pointers().len() >= 2 {
            let next_base: f32 = self
                .active_pinch_span()
                .map(|(dist, _): (f32, Point)| dist)
                .unwrap_or(0.0);
            self.set_pinch_base_dist(next_base);
            self.set_touch_drag_pointer_id(None);
            self.set_touch_drag_origin(None);
            self.set_touch_drag_last(None);
            self.set_touch_dragging(false);
            self.set_touch_drag_travel(0.0);
            self.set_touch_scroll_axis(0);
            self.get_mut_touch_vel_samples().clear();
            return;
        }
        self.set_pinch_base_dist(0.0);
        let remaining: Option<(u64, Point, PointerDevice)> = self
            .get_active_pointers()
            .iter()
            .next()
            .map(|(id, pt): (&u64, &Point)| {
                let dev: PointerDevice = self
                    .get_active_pointer_devices()
                    .get(id)
                    .copied()
                    .unwrap_or(PointerDevice::Touch);
                (*id, *pt, dev)
            });
        if let Some((rem_id, rem_pos, PointerDevice::Touch)) = remaining {
            let cur_drag_id: Option<u64> = {
                let Self {
                    touch_drag_pointer_id,
                    ..
                } = self;
                *touch_drag_pointer_id
            };
            if cur_drag_id != Some(rem_id) {
                let locked: Option<NodeId> = self.resolve_touch_scroll_target(rem_pos);
                self.set_touch_drag_pointer_id(Some(rem_id));
                self.set_touch_drag_origin(Some(rem_pos));
                self.set_touch_drag_last(Some(rem_pos));
                self.set_touch_dragging(false);
                self.set_touch_scroll_axis(0);
                self.set_touch_vel_accum_delta(Point::default());
                self.get_mut_touch_vel_samples().clear();
                self.set_touch_vel_idle_secs(0.0);
                self.set_scroll_velocity(Point::default());
                self.set_scroll_fling_pos(rem_pos);
                self.set_touch_scroll_target(locked);
            }
        }
    }

    /// Resolves the innermost scrollable or scroll-isolating node under `pos`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - App reference.
    /// - `Point` - Logical pointer coordinates.
    ///
    /// # Returns
    ///
    /// - `Option<NodeId>` - Locked scroll or isolation target under `pos`.
    #[must_use]
    fn resolve_touch_scroll_target(&self, pos: Point) -> Option<NodeId> {
        self.get_scene().with_read(
            |tree: &SceneTree| {
                let hit: Option<NodeId> = hit_test(tree, pos);
                for aid in Self::ancestor_chain(tree, hit).into_iter() {
                    if let Some(node) = tree.get_node(aid) {
                        if node.get_style().get_layout().get_overflow() == Overflow::Scroll {
                            return Some(aid);
                        }
                        if node.get_style().get_paint().get_stop_scroll_propagation()
                            || matches!(node.get_kind(), NodeKind::Portal)
                            || node.get_style().get_layout().get_position() == Position::Fixed
                        {
                            return Some(aid);
                        }
                    }
                }
                None
            },
            None,
        )
    }

    /// Projects a raw touch scroll delta onto the active gesture-locked scroll axis.
    ///
    /// # Arguments
    ///
    /// - `&Self` - App reference.
    /// - `f32` - Raw horizontal scroll delta.
    /// - `f32` - Raw vertical scroll delta.
    ///
    /// # Returns
    ///
    /// - `(f32, f32)` - Axis-filtered scroll delta `(scroll_dx, scroll_dy)`.
    #[must_use]
    fn project_touch_scroll_delta(&self, raw_dx: f32, raw_dy: f32) -> (f32, f32) {
        match self.get_touch_scroll_axis() {
            1 => (0.0, raw_dy),
            2 => (raw_dx, 0.0),
            _ => (raw_dx, raw_dy),
        }
    }

    /// Computes a clean, reversal-free inertial release velocity from recent touch-pan samples and pending in-frame displacement.
    ///
    /// # Arguments
    ///
    /// - `&Self` - App reference.
    /// - `Point` - Unsampled displacement accumulated since the last frame tick.
    ///
    /// # Returns
    ///
    /// - `Point` - Reversal-safe release velocity in logical pixels per second.
    #[must_use]
    fn compute_touch_release_velocity(&self, pending_accum: Point) -> Point {
        if self.get_touch_vel_idle_secs() >= 0.06 {
            return Point::default();
        }
        let samples: &[(f32, f32, f32)] = self.get_touch_vel_samples().as_slice();
        let mut ref_dx: f32 = 0.0;
        let mut ref_dy: f32 = 0.0;
        for &(_, dx, dy) in samples.iter() {
            ref_dx += dx;
            ref_dy += dy;
        }
        if ref_dx.abs() < 1.0 && ref_dy.abs() < 1.0 {
            ref_dx += pending_accum.get_x();
            ref_dy += pending_accum.get_y();
        }
        let mut trimmed_len: usize = samples.len();
        let mut tail_dt: f32 = 0.0;
        while trimmed_len > 1 && tail_dt < 0.028 {
            let (sdt, sdx, sdy): (f32, f32, f32) = samples[trimmed_len - 1];
            let rev_x: bool = ref_dx.abs() >= 4.0
                && sdx.abs() > 0.01
                && sdx.signum() != ref_dx.signum()
                && sdx.abs() < 12.0;
            let rev_y: bool = ref_dy.abs() >= 4.0
                && sdy.abs() > 0.01
                && sdy.signum() != ref_dy.signum()
                && sdy.abs() < 12.0;
            if rev_x || rev_y {
                trimmed_len -= 1;
                tail_dt += sdt;
            } else {
                break;
            }
        }
        let mut sum_dt: f32 = 0.0;
        let mut sum_dx: f32 = 0.0;
        let mut sum_dy: f32 = 0.0;
        for &(sdt, sdx, sdy) in samples[..trimmed_len].iter().rev() {
            if sum_dt >= 0.085 {
                break;
            }
            sum_dt += sdt;
            sum_dx += sdx;
            sum_dy += sdy;
        }
        let pend_x: f32 = if ref_dx.abs() >= 3.0
            && pending_accum.get_x().abs() > 0.01
            && pending_accum.get_x().signum() != ref_dx.signum()
            && pending_accum.get_x().abs() < 12.0
        {
            0.0
        } else {
            pending_accum.get_x()
        };
        let pend_y: f32 = if ref_dy.abs() >= 3.0
            && pending_accum.get_y().abs() > 0.01
            && pending_accum.get_y().signum() != ref_dy.signum()
            && pending_accum.get_y().abs() < 12.0
        {
            0.0
        } else {
            pending_accum.get_y()
        };
        if pend_x.hypot(pend_y) > 0.25 {
            let dt_est: f32 = self
                .get_frame_clock()
                .get_delta_secs()
                .clamp(1.0 / 144.0, 0.02);
            sum_dx += pend_x;
            sum_dy += pend_y;
            sum_dt += dt_est;
        }
        if sum_dt <= 0.002 {
            return Point::default();
        }
        let mut vx: f32 = (sum_dx / sum_dt).clamp(-4200.0, 4200.0);
        let mut vy: f32 = (sum_dy / sum_dt).clamp(-4200.0, 4200.0);
        if ref_dx.abs() >= 3.0 && vx.signum() != ref_dx.signum() {
            vx = 0.0;
        }
        if ref_dy.abs() >= 3.0 && vy.signum() != ref_dy.signum() {
            vy = 0.0;
        }
        let (ax_vx, ax_vy): (f32, f32) = self.project_touch_scroll_delta(vx, vy);
        let speed: f32 = ax_vx.hypot(ax_vy);
        if speed < TOUCH_MIN_FLING_SPEED_DP_S {
            Point::default()
        } else if speed >= 220.0 {
            Point::new(
                (ax_vx * 1.06).clamp(-4400.0, 4400.0),
                (ax_vy * 1.06).clamp(-4400.0, 4400.0),
            )
        } else {
            Point::new(ax_vx, ax_vy)
        }
    }

    /// Samples touch drag displacement once per frame into a timestamped window to prevent jitter and backward fling reversals.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable app.
    /// - `f32` - Elapsed frame delta in seconds.
    fn sample_touch_drag_velocity(&mut self, dt_secs: f32) {
        if !self.get_touch_dragging() {
            return;
        }
        let accum: Point = self.get_touch_vel_accum_delta();
        self.set_touch_vel_accum_delta(Point::default());
        let safe_dt: f32 = dt_secs.clamp(1.0 / 480.0, 0.05);
        let dist: f32 = accum.get_x().hypot(accum.get_y());
        if dist > 0.2 {
            self.set_touch_vel_idle_secs(0.0);
        } else {
            let idle: f32 = self.get_touch_vel_idle_secs() + safe_dt;
            self.set_touch_vel_idle_secs(idle);
        }
        let samples: &mut Vec<(f32, f32, f32)> = self.get_mut_touch_vel_samples();
        samples.push((safe_dt, accum.get_x(), accum.get_y()));
        let mut total_dt: f32 = samples
            .iter()
            .map(|item: &(f32, f32, f32)| item.0)
            .sum::<f32>();
        while samples.len() > 2 && (total_dt > 0.11 || samples.len() > 16) {
            let removed: (f32, f32, f32) = samples.remove(0);
            total_dt -= removed.0;
        }
    }

    /// Advances smooth exponential interpolation for discrete desktop/web mouse-wheel scrolling.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable app.
    /// - `f32` - Elapsed frame delta in seconds.
    fn tick_smooth_scroll(&mut self, dt_secs: f32) {
        let pending: Point = self.get_smooth_scroll_pending();
        let mag: f32 = pending.get_x().hypot(pending.get_y());
        if mag <= 0.05 {
            if mag > 0.0 {
                let locked: Option<NodeId> = {
                    let Self {
                        smooth_scroll_target,
                        ..
                    } = self;
                    *smooth_scroll_target
                };
                let prev_ptr: Point = self.get_input_state().get_pointer_position();
                let target_pos: Point = self.get_smooth_scroll_pos();
                self.get_mut_input_state().set_pointer_position(target_pos);
                let _: bool = self.handle_scroll_on_target(pending, locked);
                self.get_mut_input_state().set_pointer_position(prev_ptr);
                self.set_smooth_scroll_pending(Point::default());
                self.set_smooth_scroll_target(None);
            }
            return;
        }
        let frac: f32 = (1.0 - (-15.0 * dt_secs).exp()).clamp(0.01, 1.0);
        let step_dx: f32 = pending.get_x() * frac;
        let step_dy: f32 = pending.get_y() * frac;
        let locked: Option<NodeId> = {
            let Self {
                smooth_scroll_target,
                ..
            } = self;
            *smooth_scroll_target
        };
        let prev_ptr: Point = self.get_input_state().get_pointer_position();
        let target_pos: Point = self.get_smooth_scroll_pos();
        self.get_mut_input_state().set_pointer_position(target_pos);
        let moved: bool = self.handle_scroll_on_target(Point::new(step_dx, step_dy), locked);
        self.get_mut_input_state().set_pointer_position(prev_ptr);
        if !moved {
            self.set_smooth_scroll_pending(Point::default());
            self.set_smooth_scroll_target(None);
        } else {
            self.set_smooth_scroll_pending(Point::new(
                pending.get_x() - step_dx,
                pending.get_y() - step_dy,
            ));
        }
    }

    /// Applies exact frame-rate-independent exponential friction decay for touch inertial fling scrolling.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable app.
    /// - `f32` - Elapsed frame delta in seconds.
    fn tick_scroll_inertia(&mut self, dt_secs: f32) {
        if self.get_touch_dragging() {
            return;
        }
        let vel: Point = self.get_scroll_velocity();
        let speed: f32 = vel.get_x().hypot(vel.get_y());
        if speed < 1.5 {
            if speed > 0.0 {
                self.set_scroll_velocity(Point::default());
                self.set_touch_scroll_target(None);
            }
            return;
        }
        let drag_k: f32 = 3.2;
        let decay: f32 = (-drag_k * dt_secs).exp();
        let integral_scale: f32 = (1.0 - decay) / drag_k;
        let dx: f32 = vel.get_x() * integral_scale;
        let dy: f32 = vel.get_y() * integral_scale;
        let locked_target: Option<NodeId> = {
            let Self {
                touch_scroll_target,
                ..
            } = self;
            *touch_scroll_target
        };
        let prev_ptr: Point = self.get_input_state().get_pointer_position();
        let fling_pos: Point = self.get_scroll_fling_pos();
        self.get_mut_input_state().set_pointer_position(fling_pos);
        let moved: bool = self.handle_scroll_on_target(Point::new(dx, dy), locked_target);
        self.get_mut_input_state().set_pointer_position(prev_ptr);
        let next_vx: f32 = vel.get_x() * decay;
        let next_vy: f32 = vel.get_y() * decay;
        if !moved || next_vx.hypot(next_vy) < 1.5 {
            self.set_scroll_velocity(Point::default());
            self.set_touch_scroll_target(None);
        } else {
            self.set_scroll_velocity(Point::new(next_vx, next_vy));
        }
    }

    /// Collects the ancestor chain starting from `start` up to the root node.
    ///
    /// # Arguments
    ///
    /// - `&SceneTree` - Retained scene tree.
    /// - `Option<NodeId>` - Starting leaf node.
    ///
    /// # Returns
    ///
    /// - `Vec<NodeId>` - Ordered ancestor chain (leaf first).
    fn ancestor_chain(tree: &SceneTree, start: Option<NodeId>) -> Vec<NodeId> {
        let mut chain: Vec<NodeId> = Vec::new();
        let mut cursor: Option<NodeId> = start;
        while let Some(id) = cursor {
            chain.push(id);
            cursor = tree.get_node(id).and_then(SceneNode::parent_id);
        }
        chain
    }

    /// Synchronizes hovered state across old and new ancestor chains, preventing stuck hover backgrounds on clicked items.
    ///
    /// # Arguments
    ///
    /// - `&mut SceneTree` - Active scene tree.
    /// - `Option<NodeId>` - Newly hovered leaf node (`None` clears all hover states).
    /// - `&mut Vec<(UiEventHandler, UiEvent)>` - Output enter/leave callback queue.
    fn sync_hover_chain(
        tree: &mut SceneTree,
        next_hit: Option<NodeId>,
        out: &mut Vec<(UiEventHandler, UiEvent)>,
    ) {
        let prev: Option<NodeId> = tree.get_focus().hovered_node();
        if prev == next_hit {
            return;
        }
        let old_chain: Vec<NodeId> = Self::ancestor_chain(tree, prev);
        let new_chain: Vec<NodeId> = Self::ancestor_chain(tree, next_hit);
        for old_id in old_chain.iter().copied() {
            if new_chain.contains(&old_id) {
                continue;
            }
            let mut visual_hover: bool = false;
            if let Some(node) = tree.get_node_mut(old_id) {
                node.get_mut_interaction().set_hovered(false);
                visual_hover = node.hover_paint().is_some()
                    || node.get_style().get_paint().get_cursor() == CursorIcon::Pointer
                    || node.get_style().get_layout().get_overflow() == Overflow::Scroll
                    || node
                        .get_handlers()
                        .handler_for(&UiEventKind::Click(Point::default()))
                        .is_some();
                if let Some(h) = node.get_handlers().handler_for(&UiEventKind::PointerLeave) {
                    out.push((h, UiEvent::new(old_id, UiEventKind::PointerLeave)));
                }
            }
            if visual_hover {
                tree.mark_dirty(old_id, DIRTY_PAINT);
            }
        }
        tree.get_mut_focus().set_hovered(next_hit);
        for new_id in new_chain.iter().copied() {
            if old_chain.contains(&new_id) {
                continue;
            }
            let mut visual_hover: bool = false;
            if let Some(node) = tree.get_node_mut(new_id) {
                node.get_mut_interaction().set_hovered(true);
                visual_hover = node.hover_paint().is_some()
                    || node.get_style().get_paint().get_cursor() == CursorIcon::Pointer
                    || node.get_style().get_layout().get_overflow() == Overflow::Scroll
                    || node
                        .get_handlers()
                        .handler_for(&UiEventKind::Click(Point::default()))
                        .is_some();
                if let Some(h) = node.get_handlers().handler_for(&UiEventKind::PointerEnter) {
                    out.push((h, UiEvent::new(new_id, UiEventKind::PointerEnter)));
                }
            }
            if visual_hover {
                tree.mark_dirty(new_id, DIRTY_PAINT);
            }
        }
    }

    /// Emits a recognized cross-platform `GestureEvent` into the reactive `Context::get_gesture()` signal.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable app.
    /// - `GestureEventKind` - Classified gesture variant.
    /// - `Point` - Starting gesture coordinates.
    /// - `Point` - Current/ending gesture coordinates.
    /// - `f32` - Relative pinch scale factor (`1.0` for non-pinch gestures).
    /// - `u8` - Number of active pointers involved.
    /// - `PointerDevice` - Originating input device class.
    fn emit_gesture(
        &mut self,
        kind: GestureEventKind,
        origin: Point,
        position: Point,
        scale: f32,
        pointer_count: u8,
        device: PointerDevice,
    ) {
        let next_seq: u64 = self.get_gesture_seq().wrapping_add(1);
        self.set_gesture_seq(next_seq);
        let ev: GestureEvent = GestureEvent {
            sequence: next_seq,
            kind,
            origin,
            position,
            delta: Point::new(
                position.get_x() - origin.get_x(),
                position.get_y() - origin.get_y(),
            ),
            scale,
            pointer_count,
            device,
        };
        let _: bool = self.get_context().get_gesture().set(Some(ev));
    }

    /// Advances double-tap aging and stationary long-press gesture detection on every frame tick.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable app.
    /// - `f32` - Elapsed frame delta in seconds.
    fn tick_gesture_timers(&mut self, dt_secs: f32) {
        let age: f32 = (self.get_last_tap_age_secs() + dt_secs).min(10.0);
        self.set_last_tap_age_secs(age);
        let click_age: f32 = (self.get_text_last_click_age_secs() + dt_secs).min(10.0);
        self.set_text_last_click_age_secs(click_age);
        let origin_opt: Option<Point> = {
            let Self { gesture_origin, .. } = self;
            *gesture_origin
        };
        if let Some(origin) = origin_opt
            && !self.get_long_press_fired()
            && self.get_active_pointers().len() == 1
        {
            let cur: Point = self.get_input_state().get_pointer_position();
            let moved: f32 = (cur.get_x() - origin.get_x()).hypot(cur.get_y() - origin.get_y());
            if moved <= 12.0 {
                let held: f32 = self.get_gesture_hold_secs() + dt_secs;
                self.set_gesture_hold_secs(held);
                if held >= 0.45 {
                    self.set_long_press_fired(true);
                    self.emit_gesture(
                        GestureEventKind::LongPress,
                        origin,
                        cur,
                        1.0,
                        1,
                        PointerDevice::Touch,
                    );
                }
            }
        }
    }

    /// Publishes the live multi-touch contact map into `Context::get_active_touches()`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable app.
    fn publish_active_touches(&mut self) {
        let mut pts: Vec<(u64, Point)> = self
            .get_active_pointers()
            .iter()
            .map(|(id, pt): (&u64, &Point)| (*id, *pt))
            .collect();
        pts.sort_by_key(|item: &(u64, Point)| item.0);
        let _: bool = self.get_context().get_active_touches().set(pts);
    }

    /// Computes deterministic distance between the first two active touch pointers for pinch-to-zoom gestures.
    ///
    /// # Arguments
    ///
    /// - `&Self` - App reference.
    ///
    /// # Returns
    ///
    /// - `Option<(f32, Point)>` - Distance and midpoint when `>= 2` pointers are active.
    #[must_use]
    fn active_pinch_span(&self) -> Option<(f32, Point)> {
        let mut pts: Vec<(u64, Point)> = self
            .get_active_pointers()
            .iter()
            .filter(|(id, _): &(&u64, &Point)| {
                self.get_active_pointer_devices().get(id) == Some(&PointerDevice::Touch)
            })
            .map(|(id, pt): (&u64, &Point)| (*id, *pt))
            .collect();
        if pts.len() < 2 {
            pts = self
                .get_active_pointers()
                .iter()
                .map(|(id, pt): (&u64, &Point)| (*id, *pt))
                .collect();
        }
        if pts.len() < 2 {
            return None;
        }
        pts.sort_by_key(|item: &(u64, Point)| item.0);
        let a: Point = pts[0].1;
        let b: Point = pts[1].1;
        let dist: f32 = (b.get_x() - a.get_x()).hypot(b.get_y() - a.get_y());
        let mid: Point = Point::new((a.get_x() + b.get_x()) * 0.5, (a.get_y() + b.get_y()) * 0.5);
        Some((dist, mid))
    }

    /// Handles pointer motion with 1:1 touch pan-to-scroll, deterministic multi-touch pinch, stale-mouse cleanup, pointer capture for sliders, hover bubbling, cursor updates, and scrollbar dragging.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable app.
    /// - `PointerEvent` - Incoming pointer move event.
    fn handle_pointer_move_event(&mut self, ptr: PointerEvent) {
        let pos: Point = ptr.get_position();
        let pid: u64 = ptr.get_pointer_id();
        if ptr.get_device() == PointerDevice::Mouse && ptr.get_pressure() <= 0.0 {
            if self.get_mut_active_pointers().remove(&pid).is_some() {
                let _: Option<PointerDevice> = self.get_mut_active_pointer_devices().remove(&pid);
                self.publish_active_touches();
                if self.get_active_pointers().is_empty() {
                    self.set_scrollbar_dragging(false);
                    self.set_scrollbar_drag_axis(SCROLLBAR_AXIS_NONE);
                    self.set_text_selecting_node(None);
                    self.clear_pressed_node();
                }
            }
        } else if let Some(prev_pt) = self.get_active_pointers().get(&pid).copied() {
            let moved_enough: bool =
                (pos.get_x() - prev_pt.get_x()).hypot(pos.get_y() - prev_pt.get_y()) > 0.05;
            let _: Option<Point> = self.get_mut_active_pointers().insert(pid, pos);
            if moved_enough {
                self.publish_active_touches();
            }
            if self.get_active_pointers().len() >= 2 {
                if let Some((cur_dist, mid)) = self.active_pinch_span() {
                    let base: f32 = self.get_pinch_base_dist();
                    if base > 8.0 && (cur_dist - base).abs() > 6.0 {
                        let scale: f32 = (cur_dist / base).clamp(0.4, 3.0);
                        self.set_pinch_base_dist(cur_dist);
                        self.emit_gesture(
                            GestureEventKind::Pinch,
                            mid,
                            mid,
                            scale,
                            self.get_active_pointers().len().min(8) as u8,
                            ptr.get_device(),
                        );
                    }
                }
                return;
            }
        }
        let is_touch: bool = ptr.get_device() == PointerDevice::Touch;
        let drag_pid_opt: Option<u64> = {
            let Self {
                touch_drag_pointer_id,
                ..
            } = self;
            *touch_drag_pointer_id
        };
        let origin_opt: Option<Point> = {
            let Self {
                touch_drag_origin, ..
            } = self;
            *touch_drag_origin
        };
        let is_scrollbar_drag: bool = self.get_scrollbar_dragging();
        if is_touch
            && self.get_active_pointers().len() == 1
            && drag_pid_opt == Some(pid)
            && let Some(origin) = origin_opt
        {
            let captured_drag_control: bool = is_scrollbar_drag
                || self.get_scene().with_read(
                    |tree: &SceneTree| {
                        tree.get_focus()
                            .pressed_node()
                            .and_then(|pressed_id: NodeId| tree.get_node(pressed_id))
                            .map(|n: &SceneNode| {
                                n.get_handlers()
                                    .handler_for(&UiEventKind::PointerMove(pos))
                                    .is_some()
                            })
                            .unwrap_or(false)
                    },
                    false,
                );
            if !captured_drag_control {
                let total_dx: f32 = pos.get_x() - origin.get_x();
                let total_dy: f32 = pos.get_y() - origin.get_y();
                let total_dist: f32 = total_dx.hypot(total_dy);
                if !self.get_touch_dragging() && total_dist > TOUCH_SLOP_DP {
                    let axis: u8 = if total_dy.abs() >= total_dx.abs() * 1.15 {
                        1
                    } else if total_dx.abs() >= total_dy.abs() * 1.75 {
                        2
                    } else {
                        3
                    };
                    self.set_touch_dragging(true);
                    self.set_touch_drag_travel(0.0);
                    self.set_touch_scroll_axis(axis);
                    if axis == 2 && !self.get_gesture_suppressed() {
                        let _: bool = self
                            .get_context()
                            .get_swipe_drag_offset()
                            .set_if_changed(total_dx);
                    }
                    self.set_touch_vel_accum_delta(Point::default());
                    self.get_mut_touch_vel_samples().clear();
                    self.set_touch_vel_idle_secs(0.0);
                    self.clear_pressed_node();
                    self.set_scroll_fling_pos(pos);
                    let slop_scale: f32 = TOUCH_SLOP_DP / total_dist;
                    let anchor_x: f32 = origin.get_x() + total_dx * slop_scale;
                    let anchor_y: f32 = origin.get_y() + total_dy * slop_scale;
                    let raw_dx: f32 = anchor_x - pos.get_x();
                    let raw_dy: f32 = anchor_y - pos.get_y();
                    let (scroll_dx, scroll_dy): (f32, f32) =
                        self.project_touch_scroll_delta(raw_dx, raw_dy);
                    let excess_dist: f32 = scroll_dx.hypot(scroll_dy);
                    if excess_dist >= 0.05 {
                        let locked_target: Option<NodeId> = {
                            let Self {
                                touch_scroll_target,
                                ..
                            } = self;
                            *touch_scroll_target
                        };
                        let _: bool = self.handle_scroll_on_target(
                            Point::new(scroll_dx, scroll_dy),
                            locked_target,
                        );
                        self.set_touch_drag_travel(excess_dist);
                        self.set_touch_vel_accum_delta(Point::new(scroll_dx, scroll_dy));
                    }
                    self.set_touch_drag_last(Some(pos));
                    return;
                }
                if self.get_touch_dragging() {
                    if self.get_touch_scroll_axis() == 2 && !self.get_gesture_suppressed() {
                        let _: bool = self
                            .get_context()
                            .get_swipe_drag_offset()
                            .set_if_changed(total_dx);
                    }
                    let prev_opt: Option<Point> = {
                        let Self {
                            touch_drag_last, ..
                        } = self;
                        *touch_drag_last
                    };
                    if let Some(prev) = prev_opt {
                        let raw_dx: f32 = prev.get_x() - pos.get_x();
                        let raw_dy: f32 = prev.get_y() - pos.get_y();
                        let (scroll_dx, scroll_dy): (f32, f32) =
                            self.project_touch_scroll_delta(raw_dx, raw_dy);
                        let step_dist: f32 = scroll_dx.hypot(scroll_dy);
                        if step_dist < 0.05 {
                            return;
                        }
                        let locked_target: Option<NodeId> = {
                            let Self {
                                touch_scroll_target,
                                ..
                            } = self;
                            *touch_scroll_target
                        };
                        let _: bool = self.handle_scroll_on_target(
                            Point::new(scroll_dx, scroll_dy),
                            locked_target,
                        );
                        self.set_touch_drag_travel(self.get_touch_drag_travel() + step_dist);
                        let cur_accum: Point = self.get_touch_vel_accum_delta();
                        self.set_touch_vel_accum_delta(Point::new(
                            cur_accum.get_x() + scroll_dx,
                            cur_accum.get_y() + scroll_dy,
                        ));
                        self.set_touch_vel_idle_secs(0.0);
                        self.set_scroll_fling_pos(pos);
                    }
                    self.set_touch_drag_last(Some(pos));
                    return;
                }
            }
            self.set_touch_drag_last(Some(pos));
        }
        let drag_axis: u8 = self.get_scrollbar_drag_axis();
        let text_sel_node: Option<NodeId> = {
            let Self {
                text_selecting_node,
                ..
            } = self;
            *text_selecting_node
        };
        let text_sel_anchor: usize = self.get_text_select_anchor_char();
        let scene: SceneHandle = self.get_scene().clone();
        let (callbacks, next_cursor): (Vec<(UiEventHandler, UiEvent)>, CursorIcon) = scene
            .with_write(
                |tree: &mut SceneTree| {
                    let mut out: Vec<(UiEventHandler, UiEvent)> = Vec::new();
                    let pressed_opt: Option<NodeId> = tree.get_focus().pressed_node();
                    if is_scrollbar_drag
                        && let Some(pressed_id) = pressed_opt
                        && let Some(node) = tree.get_node(pressed_id)
                        && node.get_style().get_layout().get_overflow() == Overflow::Scroll
                    {
                        let lbox: LayoutBox = node.get_layout_box();
                        let border_w: f32 = node.get_style().get_paint().get_border().get_width();
                        let screen_outer: Rect = tree
                            .node_screen_bounds(pressed_id)
                            .unwrap_or_else(|| lbox.get_bounds());
                        let view_w: f32 = lbox.get_content_bounds().get_width().max(1.0);
                        let view_h: f32 = lbox.get_content_bounds().get_height().max(1.0);
                        let inner_w: f32 = (screen_outer.get_width() - border_w * 2.0).max(1.0);
                        let inner_h: f32 = (screen_outer.get_height() - border_w * 2.0).max(1.0);
                        let total_w: f32 = lbox.get_content_size().get_width();
                        let max_x: f32 = (total_w - view_w).max(0.0);
                        let total_h_ref: f32 = node
                            .virtual_scroll_metrics()
                            .map(|(_, vh): (f32, f32)| vh)
                            .unwrap_or_else(|| lbox.get_content_size().get_height());
                        let max_y_ref: f32 = (total_h_ref - view_h).max(0.0);
                        let corner_inset: f32 = if max_y_ref > 1.0 && max_x > 1.0 {
                            8.0
                        } else {
                            0.0
                        };
                        let track_margin: f32 = 3.0;
                        if drag_axis == SCROLLBAR_AXIS_HORIZONTAL {
                            let track_x: f32 = screen_outer.get_x() + border_w + track_margin;
                            let track_w: f32 =
                                (inner_w - track_margin * 2.0 - corner_inset).max(16.0);
                            if max_x > 1.0 && track_w > 16.0 {
                                let ratio: f32 = (view_w / total_w.max(view_w)).clamp(0.0, 1.0);
                                let thumb_w: f32 =
                                    (track_w * ratio).clamp(20.0_f32.min(track_w), track_w);
                                let travel: f32 = (track_w - thumb_w).max(1.0);
                                let frac: f32 = ((pos.get_x() - track_x - thumb_w * 0.5) / travel)
                                    .clamp(0.0, 1.0);
                                tree.set_scroll_offset(
                                    pressed_id,
                                    Point::new(frac * max_x, lbox.get_scroll_offset().get_y()),
                                );
                            }
                        } else {
                            let track_y: f32 = screen_outer.get_y() + border_w + track_margin;
                            let track_h: f32 =
                                (inner_h - track_margin * 2.0 - corner_inset).max(16.0);
                            if let Some((cur_vy, total_h)) = node.virtual_scroll_metrics() {
                                let max_y: f32 = (total_h - view_h).max(0.0);
                                if max_y > 1.0 && track_h > 16.0 {
                                    let ratio: f32 = (view_h / total_h.max(view_h)).clamp(0.0, 1.0);
                                    let thumb_h: f32 =
                                        (track_h * ratio).clamp(20.0_f32.min(track_h), track_h);
                                    let travel: f32 = (track_h - thumb_h).max(1.0);
                                    let frac: f32 = ((pos.get_y() - track_y - thumb_h * 0.5)
                                        / travel)
                                        .clamp(0.0, 1.0);
                                    let delta_y: f32 = frac * max_y - cur_vy;
                                    if delta_y.abs() > 0.1
                                        && let Some(h) = node.get_handlers().handler_for(
                                            &UiEventKind::Scroll(Point::new(0.0, delta_y)),
                                        )
                                    {
                                        out.push((
                                            h,
                                            UiEvent::new(
                                                pressed_id,
                                                UiEventKind::Scroll(Point::new(0.0, delta_y)),
                                            ),
                                        ));
                                    }
                                }
                            } else {
                                let total_h: f32 = lbox.get_content_size().get_height();
                                let max_y: f32 = (total_h - view_h).max(0.0);
                                if max_y > 1.0 && track_h > 16.0 {
                                    let ratio: f32 = (view_h / total_h.max(view_h)).clamp(0.0, 1.0);
                                    let thumb_h: f32 =
                                        (track_h * ratio).clamp(20.0_f32.min(track_h), track_h);
                                    let travel: f32 = (track_h - thumb_h).max(1.0);
                                    let frac: f32 = ((pos.get_y() - track_y - thumb_h * 0.5)
                                        / travel)
                                        .clamp(0.0, 1.0);
                                    tree.set_scroll_offset(
                                        pressed_id,
                                        Point::new(lbox.get_scroll_offset().get_x(), frac * max_y),
                                    );
                                }
                            }
                        }
                    }
                    if let Some(sel_id) = text_sel_node {
                        let focus_char: usize = Self::hit_test_text_node_char(tree, sel_id, pos);
                        tree.set_text_selection(Some(SelectionState::new(
                            sel_id,
                            text_sel_anchor,
                            focus_char,
                        )));
                    }
                    let hit: Option<NodeId> = hit_test(tree, pos);
                    if !is_touch {
                        Self::sync_hover_chain(tree, hit, &mut out);
                    }
                    let mut resolved_cursor: CursorIcon = CursorIcon::Default;
                    let mut has_selectable_text: bool = false;
                    for aid in Self::ancestor_chain(tree, hit).into_iter() {
                        if let Some(node) = tree.get_node(aid) {
                            if matches!(node.get_kind(), NodeKind::Input(_)) {
                                resolved_cursor = CursorIcon::Text;
                                break;
                            }
                            let c: CursorIcon = node.get_style().get_paint().get_cursor();
                            if c != CursorIcon::Default {
                                resolved_cursor = c;
                                break;
                            }
                            if matches!(node.get_kind(), NodeKind::Text(_))
                                && tree.is_node_selectable(aid)
                            {
                                has_selectable_text = true;
                            }
                        }
                    }
                    if resolved_cursor == CursorIcon::Default && has_selectable_text {
                        resolved_cursor = CursorIcon::Text;
                    }
                    if let Some(pid) = pressed_opt
                        && let Some(node) = tree.get_node(pid)
                        && let Some(h) = node
                            .get_handlers()
                            .handler_for(&UiEventKind::PointerMove(pos))
                    {
                        out.push((h, UiEvent::new(pid, UiEventKind::PointerMove(pos))));
                    }
                    if let Some(target_id) = hit
                        && pressed_opt != Some(target_id)
                        && let Some(node) = tree.get_node(target_id)
                        && let Some(h) = node
                            .get_handlers()
                            .handler_for(&UiEventKind::PointerMove(pos))
                    {
                        out.push((h, UiEvent::new(target_id, UiEventKind::PointerMove(pos))));
                    }
                    (out, resolved_cursor)
                },
                (Vec::new(), CursorIcon::Default),
            );
        if next_cursor != self.get_last_cursor() {
            self.set_last_cursor(next_cursor);
            let wid: WindowId = self.get_window_state().get_id();
            self.queue_window_command(WindowCommand::SetCursor(wid, next_cursor));
        }
        Self::invoke_callbacks(callbacks);
    }

    /// Handles pointer press, propagating pressed state, capturing draggable controls, and supporting scrollbar thumb clicks.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable app.
    /// - `PointerEvent` - Incoming pointer press event.
    fn handle_pointer_down_event(&mut self, ptr: PointerEvent) {
        let pos: Point = ptr.get_position();
        let pid: u64 = ptr.get_pointer_id();
        self.set_caret_blink_secs(0.0);
        self.set_smooth_scroll_pending(Point::default());
        self.set_smooth_scroll_target(None);
        self.get_scene().with_write(
            |tree: &mut SceneTree| {
                tree.set_caret_visible(true);
            },
            (),
        );
        let _: Option<Point> = self.get_mut_active_pointers().insert(pid, pos);
        let _: Option<PointerDevice> = self
            .get_mut_active_pointer_devices()
            .insert(pid, ptr.get_device());
        self.publish_active_touches();
        if self.get_active_pointers().len() == 1 {
            let suppress_gest: bool = self.get_scene().with_read(
                |tree: &SceneTree| {
                    let hit: Option<NodeId> = hit_test(tree, pos);
                    for aid in Self::ancestor_chain(tree, hit).into_iter() {
                        if let Some(node) = tree.get_node(aid) {
                            let is_canvas: bool = matches!(node.get_kind(), NodeKind::Canvas);
                            let has_drag_move: bool = node
                                .get_handlers()
                                .handler_for(&UiEventKind::PointerMove(pos))
                                .is_some();
                            let has_horiz_scroll: bool =
                                node.get_style().get_layout().get_overflow() == Overflow::Scroll
                                    && node.get_layout_box().get_content_size().get_width()
                                        > node.get_layout_box().get_content_bounds().get_width()
                                            + 2.0;
                            if is_canvas
                                || has_drag_move
                                || has_horiz_scroll
                                || node.get_style().get_paint().get_stop_event_propagation()
                            {
                                return true;
                            }
                        }
                    }
                    false
                },
                false,
            );
            self.set_gesture_suppressed(suppress_gest);
            self.set_gesture_origin(Some(pos));
            self.set_gesture_hold_secs(0.0);
            self.set_long_press_fired(false);
        } else {
            self.set_gesture_origin(None);
            self.set_gesture_suppressed(true);
            let next_base: f32 = self
                .active_pinch_span()
                .map(|(dist, _): (f32, Point)| dist)
                .unwrap_or(0.0);
            self.set_pinch_base_dist(next_base);
            self.set_touch_drag_pointer_id(None);
            self.set_touch_drag_origin(None);
            self.set_touch_drag_last(None);
            self.set_touch_dragging(false);
            self.set_touch_drag_travel(0.0);
            self.set_touch_scroll_axis(0);
            self.set_touch_vel_accum_delta(Point::default());
            self.get_mut_touch_vel_samples().clear();
            self.set_touch_vel_idle_secs(0.0);
            self.set_scroll_velocity(Point::default());
            self.set_touch_scroll_target(None);
            let _: bool = self
                .get_context()
                .get_swipe_drag_offset()
                .set_if_changed(0.0);
            return;
        }
        if ptr.get_device() == PointerDevice::Touch {
            self.set_touch_drag_pointer_id(Some(pid));
            self.set_touch_drag_origin(Some(pos));
            self.set_touch_drag_last(Some(pos));
            self.set_touch_dragging(false);
            self.set_touch_drag_travel(0.0);
            self.set_touch_scroll_axis(0);
            self.set_touch_vel_accum_delta(Point::default());
            self.get_mut_touch_vel_samples().clear();
            self.set_touch_vel_idle_secs(0.0);
            self.set_scroll_velocity(Point::default());
            self.set_scroll_fling_pos(pos);
            let locked: Option<NodeId> = self.resolve_touch_scroll_target(pos);
            self.set_touch_scroll_target(locked);
        } else {
            self.set_touch_drag_pointer_id(None);
            self.set_touch_drag_origin(None);
            self.set_touch_drag_last(None);
            self.set_touch_dragging(false);
            self.set_touch_drag_travel(0.0);
            self.set_touch_scroll_axis(0);
            self.set_touch_vel_accum_delta(Point::default());
            self.get_mut_touch_vel_samples().clear();
            self.set_touch_vel_idle_secs(0.0);
            self.set_scroll_velocity(Point::default());
            self.set_touch_scroll_target(None);
        }
        let prev_click_node: Option<NodeId> = {
            let Self {
                text_last_click_node,
                ..
            } = self;
            *text_last_click_node
        };
        let prev_click_pos: Option<Point> = {
            let Self {
                text_last_click_pos,
                ..
            } = self;
            *text_last_click_pos
        };
        let prev_click_age: f32 = self.get_text_last_click_age_secs();
        let prev_click_count: u8 = self.get_text_click_count();
        let scene: SceneHandle = self.get_scene().clone();
        let (callbacks, grabbed_scrollbar_axis, tapped_enabled_input, text_sel_started): PointerDownOutcome =
            scene.with_write(
            |tree: &mut SceneTree| {
                let hit: Option<NodeId> = hit_test(tree, pos);
                let mut hover_out: Vec<(UiEventHandler, UiEvent)> = Vec::new();
                Self::sync_hover_chain(
                    tree,
                    if ptr.get_device() == PointerDevice::Touch {
                        None
                    } else {
                        hit
                    },
                    &mut hover_out,
                );
                let mut scrollbar_target: Option<(NodeId, u8)> = None;
                let mut sb_callbacks: Vec<(UiEventHandler, UiEvent)> = hover_out;
                if ptr.get_device() != PointerDevice::Touch {
                    for aid in Self::ancestor_chain(tree, hit).into_iter() {
                        if let Some(node) = tree.get_node(aid)
                            && node.get_style().get_layout().get_overflow() == Overflow::Scroll
                        {
                            let lbox: LayoutBox = node.get_layout_box();
                            let border_w: f32 =
                                node.get_style().get_paint().get_border().get_width();
                            let screen_outer: Rect = tree
                                .node_screen_bounds(aid)
                                .unwrap_or_else(|| lbox.get_bounds());
                            let inner_right: f32 = screen_outer.right() - border_w;
                            let inner_bottom: f32 = screen_outer.bottom() - border_w;
                            let inner_w: f32 =
                                (screen_outer.get_width() - border_w * 2.0).max(1.0);
                            let inner_h: f32 =
                                (screen_outer.get_height() - border_w * 2.0).max(1.0);
                            let view_w: f32 = lbox.get_content_bounds().get_width().max(1.0);
                            let view_h: f32 = lbox.get_content_bounds().get_height().max(1.0);
                            let (cur_vy, total_h, is_virtual): (f32, f32, bool) =
                                match node.virtual_scroll_metrics() {
                                    Some((vy, vh)) => (vy, vh, true),
                                    None => (
                                        lbox.get_scroll_offset().get_y(),
                                        lbox.get_content_size().get_height(),
                                        false,
                                    ),
                                };
                            let total_w: f32 = lbox.get_content_size().get_width();
                            let max_x: f32 = (total_w - view_w).max(0.0);
                            let max_y: f32 = (total_h - view_h).max(0.0);
                            let has_v_bar: bool = max_y > 1.0;
                            let has_h_bar: bool = max_x > 1.0 && screen_outer.get_width() > 24.0;
                            let corner_inset: f32 = if has_v_bar && has_h_bar { 8.0 } else { 0.0 };
                            if has_v_bar
                                && pos.get_x() >= inner_right - 14.0
                                && pos.get_x() <= screen_outer.right()
                                && (!has_h_bar || pos.get_y() < inner_bottom - 14.0)
                            {
                                let track_margin: f32 = 3.0;
                                let track_y: f32 = screen_outer.get_y() + border_w + track_margin;
                                let track_h: f32 =
                                    (inner_h - track_margin * 2.0 - corner_inset).max(16.0);
                                let ratio: f32 = (view_h / total_h.max(view_h)).clamp(0.0, 1.0);
                                let thumb_h: f32 =
                                    (track_h * ratio).clamp(20.0_f32.min(track_h), track_h);
                                let travel: f32 = (track_h - thumb_h).max(1.0);
                                let frac: f32 = ((pos.get_y() - track_y - thumb_h * 0.5) / travel)
                                    .clamp(0.0, 1.0);
                                if is_virtual {
                                    let delta_y: f32 = frac * max_y - cur_vy;
                                    if let Some(h) = node
                                        .get_handlers()
                                        .handler_for(&UiEventKind::Scroll(Point::new(0.0, delta_y)))
                                    {
                                        sb_callbacks.push((
                                            h,
                                            UiEvent::new(
                                                aid,
                                                UiEventKind::Scroll(Point::new(0.0, delta_y)),
                                            ),
                                        ));
                                    }
                                } else {
                                    tree.set_scroll_offset(
                                        aid,
                                        Point::new(lbox.get_scroll_offset().get_x(), frac * max_y),
                                    );
                                }
                                scrollbar_target = Some((aid, SCROLLBAR_AXIS_VERTICAL));
                                break;
                            }
                            if has_h_bar
                                && pos.get_y() >= inner_bottom - 14.0
                                && pos.get_y() <= screen_outer.bottom()
                            {
                                let track_margin: f32 = 3.0;
                                let track_x: f32 = screen_outer.get_x() + border_w + track_margin;
                                let track_w: f32 =
                                    (inner_w - track_margin * 2.0 - corner_inset).max(16.0);
                                let ratio: f32 = (view_w / total_w.max(view_w)).clamp(0.0, 1.0);
                                let thumb_w: f32 =
                                    (track_w * ratio).clamp(20.0_f32.min(track_w), track_w);
                                let travel: f32 = (track_w - thumb_w).max(1.0);
                                let frac: f32 = ((pos.get_x() - track_x - thumb_w * 0.5) / travel)
                                    .clamp(0.0, 1.0);
                                tree.set_scroll_offset(
                                    aid,
                                    Point::new(frac * max_x, lbox.get_scroll_offset().get_y()),
                                );
                                scrollbar_target = Some((aid, SCROLLBAR_AXIS_HORIZONTAL));
                                break;
                            }
                        }
                    }
                }
                if let Some((sb_id, sb_axis)) = scrollbar_target {
                    tree.get_mut_focus().set_pressed(Some(sb_id));
                    return (sb_callbacks, sb_axis, false, None);
                }
                let mut focus_target: Option<NodeId> = None;
                let mut press_target: Option<NodeId> = None;
                for aid in Self::ancestor_chain(tree, hit).into_iter() {
                    if let Some(node) = tree.get_node(aid) {
                        let is_scrim: bool = matches!(node.get_kind(), NodeKind::Portal)
                            || node.get_style().get_layout().get_position() == Position::Fixed;
                        if focus_target.is_none()
                            && node.get_interaction().get_focusable()
                            && (!is_scrim || hit == Some(aid))
                        {
                            focus_target = Some(aid);
                        }
                        if (!is_scrim || hit == Some(aid))
                            && (node
                                .get_handlers()
                                .handler_for(&UiEventKind::PointerDown(pos))
                                .is_some()
                                || node
                                    .get_handlers()
                                    .handler_for(&UiEventKind::Click(pos))
                                    .is_some()
                                || node.get_style().get_paint().get_cursor() == CursorIcon::Pointer)
                        {
                            press_target = Some(aid);
                            break;
                        }
                    }
                }
                let mut started_sel: Option<StartedTextSelection> = None;
                if ptr.get_button() == PointerButton::Primary
                    && ptr.get_device() != PointerDevice::Touch
                    && let Some(hit_id) = hit
                    && tree.is_node_selectable(hit_id)
                {
                    let text_meta: Option<(String, bool)> =
                        tree.get_node(hit_id).and_then(|n: &SceneNode| {
                            if let NodeKind::Text(content) = n.get_kind() {
                                Some((content.clone(), n.get_style().get_text().get_selectable()))
                            } else {
                                None
                            }
                        });
                    if let Some((content, explicit_self)) = text_meta
                        && (explicit_self || press_target.is_none())
                    {
                        let char_idx: usize = Self::hit_test_text_node_char(tree, hit_id, pos);
                        let same_spot: bool = prev_click_node == Some(hit_id)
                            && prev_click_age <= TEXT_MULTI_CLICK_MAX_SECS
                            && prev_click_pos
                                .map(|p: Point| {
                                    (p.get_x() - pos.get_x()).hypot(p.get_y() - pos.get_y())
                                        <= TEXT_MULTI_CLICK_MAX_DIST_DP
                                })
                                .unwrap_or(false);
                        let click_count: u8 = if same_spot {
                            if prev_click_count >= 3 {
                                1
                            } else {
                                prev_click_count + 1
                            }
                        } else {
                            1
                        };
                        if click_count == 1 {
                            tree.set_text_selection(Some(SelectionState::new(
                                hit_id, char_idx, char_idx,
                            )));
                            started_sel = Some((hit_id, char_idx, 1, true));
                        } else if click_count == 2 {
                            let (ws, we): (usize, usize) =
                                word_bounds_at_char(content.as_str(), char_idx);
                            tree.set_text_selection(Some(SelectionState::new(hit_id, ws, we)));
                            started_sel = Some((hit_id, ws, 2, false));
                        } else {
                            let total_chars: usize = content.chars().count();
                            tree.set_text_selection(Some(SelectionState::new(
                                hit_id,
                                0,
                                total_chars,
                            )));
                            started_sel = Some((hit_id, 0, 3, false));
                        }
                    } else {
                        tree.clear_text_selection();
                    }
                } else {
                    tree.clear_text_selection();
                }
                let hit_enabled_input: bool = focus_target
                        .and_then(|fid: NodeId| tree.get_node(fid))
                        .map(|n: &SceneNode| {
                            matches!(n.get_kind(), NodeKind::Input(inp) if !inp.get_disabled())
                        })
                        .unwrap_or(false);
                let mut out: Vec<(UiEventHandler, UiEvent)> = sb_callbacks;
                out.extend(tree.update_focus(focus_target));
                tree.get_mut_focus().set_pressed(press_target);
                if let Some(id) = press_target {
                    if let Some(node) = tree.get_node_mut(id) {
                        node.get_mut_interaction().set_pressed(true);
                        if let Some(h) = node
                            .get_handlers()
                            .handler_for(&UiEventKind::PointerDown(pos))
                        {
                            out.push((h, UiEvent::new(id, UiEventKind::PointerDown(pos))));
                        }
                    }
                    tree.mark_dirty(id, DIRTY_PAINT);
                }
                (out, SCROLLBAR_AXIS_NONE, hit_enabled_input, started_sel)
            },
            (Vec::new(), SCROLLBAR_AXIS_NONE, false, None),
        );
        self.set_scrollbar_dragging(grabbed_scrollbar_axis != SCROLLBAR_AXIS_NONE);
        self.set_scrollbar_drag_axis(grabbed_scrollbar_axis);
        if let Some((sel_id, anchor_char, clicks, drag_active)) = text_sel_started {
            self.set_text_selecting_node(if drag_active { Some(sel_id) } else { None });
            self.set_text_select_anchor_char(anchor_char);
            self.set_text_click_count(clicks);
            self.set_text_last_click_node(Some(sel_id));
            self.set_text_last_click_pos(Some(pos));
            self.set_text_last_click_age_secs(0.0);
            self.set_gesture_suppressed(true);
        } else {
            self.set_text_selecting_node(None);
            self.set_text_click_count(0);
            self.set_text_last_click_node(None);
            self.set_text_last_click_pos(None);
        }
        if ptr.get_device() == PointerDevice::Touch {
            if !tapped_enabled_input {
                self.sync_ime_focus();
            }
        } else {
            self.sync_ime_focus_forced(tapped_enabled_input);
        }
        Self::invoke_callbacks(callbacks);
    }

    /// Handles pointer release, firing `PointerUp` on captured targets and bubbling `Click` when not touch-scrolling.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable app.
    /// - `PointerEvent` - Incoming pointer release event.
    fn handle_pointer_up_event(&mut self, ptr: PointerEvent) {
        let pos: Point = ptr.get_position();
        let pid: u64 = ptr.get_pointer_id();
        let had_active_pointer: bool = self.get_mut_active_pointers().remove(&pid).is_some();
        let _: Option<PointerDevice> = self.get_mut_active_pointer_devices().remove(&pid);
        self.publish_active_touches();
        if !self.get_active_pointers().is_empty() {
            self.sync_remaining_pointer_state();
            return;
        }
        let origin_opt: Option<Point> = {
            let Self { gesture_origin, .. } = self;
            *gesture_origin
        };
        let suppressed: bool = self.get_gesture_suppressed();
        self.set_gesture_origin(None);
        self.set_gesture_suppressed(false);
        self.set_pinch_base_dist(0.0);
        self.set_gesture_hold_secs(0.0);
        let long_fired: bool = self.get_long_press_fired();
        self.set_long_press_fired(false);
        let locked_axis: u8 = self.get_touch_scroll_axis();
        let _: bool = self
            .get_context()
            .get_swipe_drag_offset()
            .set_if_changed(0.0);
        if let Some(origin) = origin_opt
            && !long_fired
            && !suppressed
        {
            let dx: f32 = pos.get_x() - origin.get_x();
            let dy: f32 = pos.get_y() - origin.get_y();
            let dist: f32 = dx.hypot(dy);
            if dist <= TOUCH_SLOP_DP {
                let prev_tap: Option<Point> = {
                    let Self { last_tap_pos, .. } = self;
                    *last_tap_pos
                };
                let is_double: bool = self.get_last_tap_age_secs() <= 0.35
                    && prev_tap
                        .map(|p: Point| {
                            (p.get_x() - pos.get_x()).hypot(p.get_y() - pos.get_y()) <= 28.0
                        })
                        .unwrap_or(false);
                if is_double {
                    self.set_last_tap_pos(None);
                    self.set_last_tap_age_secs(10.0);
                    self.emit_gesture(
                        GestureEventKind::DoubleTap,
                        origin,
                        pos,
                        1.0,
                        1,
                        ptr.get_device(),
                    );
                } else {
                    self.set_last_tap_pos(Some(pos));
                    self.set_last_tap_age_secs(0.0);
                    self.emit_gesture(GestureEventKind::Tap, origin, pos, 1.0, 1, ptr.get_device());
                }
            } else {
                let is_horiz_swipe: bool = locked_axis != 1
                    && dx.abs() >= SWIPE_MIN_DISTANCE_DP
                    && dx.abs() >= dy.abs() * SWIPE_DIRECTION_RATIO
                    && dy.abs() <= SWIPE_MAX_CROSS_AXIS_DP;
                let is_vert_swipe: bool = locked_axis != 2
                    && dy.abs() >= SWIPE_MIN_DISTANCE_DP
                    && dy.abs() >= dx.abs() * SWIPE_DIRECTION_RATIO
                    && dx.abs() <= SWIPE_MAX_CROSS_AXIS_DP;
                if is_horiz_swipe {
                    let swipe_kind: GestureEventKind = if dx > 0.0 {
                        GestureEventKind::SwipeRight
                    } else {
                        GestureEventKind::SwipeLeft
                    };
                    self.emit_gesture(swipe_kind, origin, pos, 1.0, 1, ptr.get_device());
                } else if is_vert_swipe {
                    let swipe_kind: GestureEventKind = if dy > 0.0 {
                        GestureEventKind::SwipeDown
                    } else {
                        GestureEventKind::SwipeUp
                    };
                    self.emit_gesture(swipe_kind, origin, pos, 1.0, 1, ptr.get_device());
                } else {
                    self.emit_gesture(GestureEventKind::Pan, origin, pos, 1.0, 1, ptr.get_device());
                }
            }
        }
        let was_touch_dragging: bool = self.get_touch_dragging();
        let drag_travel: f32 = self.get_touch_drag_travel();
        if was_touch_dragging && drag_travel >= TOUCH_MIN_FLING_DRAG_DP {
            let pending_accum: Point = self.get_touch_vel_accum_delta();
            let release_vel: Point = self.compute_touch_release_velocity(pending_accum);
            self.set_scroll_velocity(release_vel);
            if release_vel.get_x().hypot(release_vel.get_y()) <= 0.0 {
                self.set_touch_scroll_target(None);
            }
        } else {
            self.set_scroll_velocity(Point::default());
            self.set_touch_scroll_target(None);
        }
        let was_text_selecting: bool = {
            let Self {
                text_selecting_node,
                ..
            } = self;
            text_selecting_node.is_some()
        };
        let selected_non_empty: bool = self.get_scene().get_selected_text().is_some();
        let text_sel_active: bool = was_text_selecting || self.get_text_click_count() >= 2;
        let suppress_click: bool = was_touch_dragging
            || self.get_scrollbar_dragging()
            || (text_sel_active && selected_non_empty);
        self.set_touch_drag_pointer_id(None);
        self.set_touch_drag_origin(None);
        self.set_touch_drag_last(None);
        self.set_touch_dragging(false);
        self.set_touch_drag_travel(0.0);
        self.set_touch_scroll_axis(0);
        self.set_touch_vel_accum_delta(Point::default());
        self.get_mut_touch_vel_samples().clear();
        self.set_touch_vel_idle_secs(0.0);
        self.set_scrollbar_dragging(false);
        self.set_scrollbar_drag_axis(SCROLLBAR_AXIS_NONE);
        self.set_text_selecting_node(None);
        let scene: SceneHandle = self.get_scene().clone();
        let (callbacks, tapped_focused_input): (Vec<(UiEventHandler, UiEvent)>, bool) = scene
            .with_write(
                |tree: &mut SceneTree| {
                    let hit: Option<NodeId> = hit_test(tree, pos);
                    let pressed: Option<NodeId> = tree.get_focus().pressed_node();
                    tree.get_mut_focus().set_pressed(None);
                    let mut out: Vec<(UiEventHandler, UiEvent)> = Vec::new();
                    if let Some(prev_id) = pressed {
                        if let Some(node) = tree.get_node_mut(prev_id) {
                            node.get_mut_interaction().set_pressed(false);
                            if let Some(h) = node
                                .get_handlers()
                                .handler_for(&UiEventKind::PointerUp(pos))
                            {
                                out.push((h, UiEvent::new(prev_id, UiEventKind::PointerUp(pos))));
                            }
                        }
                        tree.mark_dirty(prev_id, DIRTY_PAINT);
                    }
                    let mut retapped_input: bool = false;
                    if let Some(target_id) = hit {
                        if pressed != Some(target_id)
                            && let Some(node) = tree.get_node(target_id)
                            && let Some(h) = node
                                .get_handlers()
                                .handler_for(&UiEventKind::PointerUp(pos))
                        {
                            out.push((h, UiEvent::new(target_id, UiEventKind::PointerUp(pos))));
                        }
                        if !suppress_click && (had_active_pointer || pressed.is_some()) {
                            let focused_now: Option<NodeId> = tree.get_focus().focused_node();
                            if let Some(fid) = focused_now
                                && Self::ancestor_chain(tree, Some(target_id)).contains(&fid)
                                && let Some(fnode) = tree.get_node(fid)
                                && matches!(fnode.get_kind(), NodeKind::Input(inp) if !inp.get_disabled())
                            {
                                retapped_input = true;
                            }
                            let mut bubble: Option<NodeId> = Some(target_id);
                            while let Some(id) = bubble {
                                if let Some(node) = tree.get_node(id) {
                                    if let Some(h) =
                                        node.get_handlers().handler_for(&UiEventKind::Click(pos))
                                    {
                                        out.push((
                                            h,
                                            UiEvent::new(target_id, UiEventKind::Click(pos)),
                                        ));
                                        break;
                                    }
                                    if node.get_style().get_paint().get_stop_event_propagation() {
                                        break;
                                    }
                                    bubble = node.parent_id();
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                    if ptr.get_device() == PointerDevice::Touch {
                        Self::sync_hover_chain(tree, None, &mut out);
                    }
                    (out, retapped_input)
                },
                (Vec::new(), false),
            );
        if tapped_focused_input {
            self.sync_ime_focus_forced(true);
        } else if !suppress_click {
            self.sync_ime_focus();
        }
        Self::invoke_callbacks(callbacks);
    }

    /// Handles wheel/pan scroll deltas with smooth exponential interpolation for discrete mouse wheels and 1:1 immediate response for precise trackpads and headless runs.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable app.
    /// - `Point` - Logical scroll delta `(dx, dy)`.
    fn handle_scroll(&mut self, raw_delta: Point) {
        let delta: Point = if self.get_input_state().get_modifiers().get_shift()
            && raw_delta.get_x().abs() <= 0.01
            && raw_delta.get_y().abs() > 0.01
        {
            Point::new(raw_delta.get_y(), 0.0)
        } else {
            raw_delta
        };
        let mag: f32 = delta.get_x().hypot(delta.get_y());
        if self.platform_kind() == PlatformKind::Headless {
            self.set_smooth_scroll_pending(Point::default());
            self.set_smooth_scroll_target(None);
            let _: bool = self.handle_scroll_on_target(delta, None);
            return;
        }
        let cur_pending: Point = self.get_smooth_scroll_pending();
        let cur_mag: f32 = cur_pending.get_x().hypot(cur_pending.get_y());
        if mag <= 24.0 && cur_mag <= 0.05 {
            self.set_scroll_velocity(Point::default());
            self.set_touch_scroll_target(None);
            let _: bool = self.handle_scroll_on_target(delta, None);
            return;
        }
        self.set_scroll_velocity(Point::default());
        self.set_touch_scroll_target(None);
        let pos: Point = self.get_input_state().get_pointer_position();
        let locked: Option<NodeId> = self.resolve_touch_scroll_target(pos);
        let base_x: f32 = if cur_pending.get_x() * delta.get_x() < 0.0 {
            0.0
        } else {
            cur_pending.get_x()
        };
        let base_y: f32 = if cur_pending.get_y() * delta.get_y() < 0.0 {
            0.0
        } else {
            cur_pending.get_y()
        };
        let next_x: f32 = (base_x + delta.get_x()).clamp(-2400.0, 2400.0);
        let next_y: f32 = (base_y + delta.get_y()).clamp(-2400.0, 2400.0);
        self.set_smooth_scroll_pending(Point::new(next_x, next_y));
        self.set_smooth_scroll_pos(pos);
        self.set_smooth_scroll_target(locked);
    }

    /// Routes scroll delta to an optional locked touch target or hovered node with strict scroll-isolation support.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable app.
    /// - `Point` - Logical scroll delta `(dx, dy)`.
    /// - `Option<NodeId>` - Optional locked scroll target captured at touch-down.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether a scroll container or virtual list consumed and advanced the scroll offset.
    fn handle_scroll_on_target(&mut self, delta: Point, locked_target: Option<NodeId>) -> bool {
        let pointer_pos: Point = self.get_input_state().get_pointer_position();
        let scene: SceneHandle = self.get_scene().clone();
        let (callbacks, moved): (Vec<(UiEventHandler, UiEvent)>, bool) = scene.with_write(
            |tree: &mut SceneTree| {
                let mut cursor: Option<NodeId> = locked_target
                    .filter(|id: &NodeId| tree.get_node(*id).is_some() && tree.is_node_visible(*id))
                    .or_else(|| {
                        tree.get_focus().hovered_node().filter(|id: &NodeId| {
                            tree.get_node(*id).is_some() && tree.is_node_visible(*id)
                        })
                    });
                if cursor.is_none() {
                    let fresh: Option<NodeId> = hit_test(tree, pointer_pos);
                    if fresh.is_some() {
                        tree.get_mut_focus().set_hovered(fresh);
                        cursor = fresh;
                    }
                }
                if cursor.is_none() {
                    for (idx, node) in tree.get_nodes().iter().enumerate() {
                        if node.get_alive()
                            && node.get_style().get_layout().get_overflow() == Overflow::Scroll
                        {
                            let cand_id: NodeId = NodeId::new(idx as u32, node.get_generation());
                            if !tree.is_node_visible(cand_id) {
                                continue;
                            }
                            let lbox: LayoutBox = node.get_layout_box();
                            let scrollable: bool = node.virtual_scroll_metrics().is_some()
                                || lbox.get_content_size().get_height()
                                    > lbox.get_content_bounds().get_height() + 1.0
                                || lbox.get_content_size().get_width()
                                    > lbox.get_content_bounds().get_width() + 1.0;
                            if scrollable {
                                cursor = Some(cand_id);
                                break;
                            }
                        }
                    }
                }
                let mut out: Vec<(UiEventHandler, UiEvent)> = Vec::new();
                let mut rem_dx: f32 = delta.get_x();
                let mut rem_dy: f32 = delta.get_y();
                let mut any_moved: bool = false;
                while let Some(id) = cursor {
                    let (scroll_update, next_parent, blocks_scroll, virtual_stop): ScrollStepOutcome =
                        match tree.get_node(id) {
                        Some(node) => {
                            let blocks: bool =
                                node.get_style().get_paint().get_stop_scroll_propagation()
                                    || matches!(node.get_kind(), NodeKind::Portal)
                                    || node.get_style().get_layout().get_position()
                                        == Position::Fixed;
                            let parent_id: Option<NodeId> = node.parent_id();
                            let is_scroll_box: bool =
                                node.get_style().get_layout().get_overflow() == Overflow::Scroll;
                            if is_scroll_box {
                                let lbox: LayoutBox = node.get_layout_box();
                                let cur: Point = lbox.get_scroll_offset();
                                let max_x: f32 = (lbox.get_content_size().get_width()
                                    - lbox.get_content_bounds().get_width())
                                .max(0.0);
                                let max_y: f32 = (lbox.get_content_size().get_height()
                                    - lbox.get_content_bounds().get_height())
                                .max(0.0);
                                let allow_x: bool = max_x > 1.0
                                    && rem_dx.abs() > 0.01
                                    && (max_y <= 1.0 || rem_dx.abs() >= rem_dy.abs() * 0.25);
                                let next_x: f32 = if allow_x {
                                    (cur.get_x() + rem_dx).clamp(0.0, max_x)
                                } else {
                                    cur.get_x().min(max_x)
                                };
                                let next_y: f32 = if max_y > 1.0 && rem_dy.abs() > 0.01 {
                                    (cur.get_y() + rem_dy).clamp(0.0, max_y)
                                } else {
                                    cur.get_y().min(max_y)
                                };
                                let next: Point = Point::new(next_x, next_y);
                                let is_virtual: bool = node.virtual_scroll_metrics().is_some();
                                let virtual_moved: bool = if let Some((vy, total_h)) =
                                    node.virtual_scroll_metrics()
                                {
                                    let vmax: f32 =
                                        (total_h - lbox.get_content_bounds().get_height()).max(0.0);
                                    let vnext: f32 = (vy + rem_dy).clamp(0.0, vmax);
                                    (vnext - vy).abs() > 0.01
                                } else {
                                    false
                                };
                                let step_delta: Point = Point::new(rem_dx, rem_dy);
                                if let Some(h) = node
                                    .get_handlers()
                                    .handler_for(&UiEventKind::Scroll(step_delta))
                                {
                                    out.push((h, UiEvent::new(id, UiEventKind::Scroll(step_delta))));
                                }
                                if is_virtual {
                                    (None, parent_id, blocks, Some(any_moved || virtual_moved))
                                } else {
                                    let consumed_x: f32 = next_x - cur.get_x();
                                    let consumed_y: f32 = next_y - cur.get_y();
                                    let upd: Option<(Point, f32, f32)> =
                                        if consumed_x.abs() > 0.01 || consumed_y.abs() > 0.01 {
                                            Some((next, consumed_x, consumed_y))
                                        } else {
                                            None
                                        };
                                    (upd, parent_id, blocks, None)
                                }
                            } else {
                                (None, parent_id, blocks, None)
                            }
                        }
                        None => (None, None, false, None),
                    };
                    if let Some(v_res) = virtual_stop {
                        return (out, v_res);
                    }
                    if let Some((next, consumed_x, consumed_y)) = scroll_update {
                        tree.set_scroll_offset(id, next);
                        rem_dx -= consumed_x;
                        rem_dy -= consumed_y;
                        any_moved = true;
                    }
                    if rem_dx.abs() <= 0.01 && rem_dy.abs() <= 0.01 {
                        return (out, true);
                    }
                    if blocks_scroll {
                        return (out, any_moved);
                    }
                    cursor = next_parent;
                }
                if any_moved {
                    return (out, true);
                }
                for (idx, node) in tree.get_nodes().iter().enumerate() {
                    if !node.get_alive()
                        || node.get_style().get_layout().get_overflow() != Overflow::Scroll
                    {
                        continue;
                    }
                    let id: NodeId = NodeId::new(idx as u32, node.get_generation());
                    if !tree.is_node_visible(id) {
                        continue;
                    }
                    let lbox: LayoutBox = node.get_layout_box();
                    let cur: Point = lbox.get_scroll_offset();
                    let max_x: f32 = (lbox.get_content_size().get_width()
                        - lbox.get_content_bounds().get_width())
                    .max(0.0);
                    let max_y: f32 = (lbox.get_content_size().get_height()
                        - lbox.get_content_bounds().get_height())
                    .max(0.0);
                    let next_x: f32 = if max_x > 1.0 && rem_dx.abs() > 0.01 {
                        (cur.get_x() + rem_dx).clamp(0.0, max_x)
                    } else {
                        cur.get_x()
                    };
                    let next_y: f32 = if max_y > 1.0 && rem_dy.abs() > 0.01 {
                        (cur.get_y() + rem_dy).clamp(0.0, max_y)
                    } else {
                        cur.get_y()
                    };
                    if (next_x - cur.get_x()).abs() > 0.01 || (next_y - cur.get_y()).abs() > 0.01 {
                        tree.set_scroll_offset(id, Point::new(next_x, next_y));
                        return (out, true);
                    }
                }
                (out, false)
            },
            (Vec::new(), false),
        );
        Self::invoke_callbacks(callbacks);
        moved
    }

    /// Collects visible focusable nodes in pre-order DFS traversal for keyboard Tab focus cycling.
    ///
    /// # Arguments
    ///
    /// - `&SceneTree` - Retained scene tree.
    /// - `NodeId` - Current traversal node.
    /// - `&mut Vec<NodeId>` - Collected visible focusable node handles.
    fn collect_focusable_nodes(tree: &SceneTree, node_id: NodeId, out: &mut Vec<NodeId>) {
        let Some(node) = tree.get_node(node_id) else {
            return;
        };
        if node.get_style().get_layout().get_display() == Display::None
            || node.get_style().get_paint().get_opacity() <= 0.0
        {
            return;
        }
        let is_scrim: bool = matches!(node.get_kind(), NodeKind::Portal)
            || node.get_style().get_layout().get_position() == Position::Fixed;
        if node.get_interaction().get_focusable() && !is_scrim {
            out.push(node_id);
        }
        for child_id in node.get_children().iter().copied() {
            Self::collect_focusable_nodes(tree, child_id, out);
        }
    }

    /// Handles keyboard presses including Tab focus cycling, Enter/Space activation, scroll keys, and Input editing.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable app.
    /// - `KeyEvent` - Keyboard event payload.
    fn handle_key_down(&mut self, key: KeyEvent) {
        self.set_caret_blink_secs(0.0);
        let scene: SceneHandle = self.get_scene().clone();
        if key.get_code() == KeyCode::Tab {
            let focus_callbacks: Vec<(UiEventHandler, UiEvent)> = scene.with_write(
                |tree: &mut SceneTree| {
                    let mut focusables: Vec<NodeId> = Vec::new();
                    Self::collect_focusable_nodes(tree, tree.get_root(), &mut focusables);
                    if focusables.is_empty() {
                        return Vec::new();
                    }
                    let cur: Option<NodeId> = tree.get_focus().focused_node();
                    let next_idx: usize = match cur.and_then(|id: NodeId| {
                        focusables.iter().position(|cand: &NodeId| *cand == id)
                    }) {
                        Some(idx) if key.get_modifiers().get_shift() => {
                            if idx == 0 {
                                focusables.len() - 1
                            } else {
                                idx - 1
                            }
                        }
                        Some(idx) => (idx + 1) % focusables.len(),
                        None if key.get_modifiers().get_shift() => focusables.len() - 1,
                        None => 0,
                    };
                    tree.update_focus(Some(focusables[next_idx]))
                },
                Vec::new(),
            );
            self.sync_ime_focus();
            Self::invoke_callbacks(focus_callbacks);
            return;
        }
        let focused_is_input: bool = scene.with_read(
            |tree: &SceneTree| {
                tree.get_focus()
                    .focused_node()
                    .and_then(|id: NodeId| tree.get_node(id))
                    .map(|n: &SceneNode| matches!(n.get_kind(), NodeKind::Input(_)))
                    .unwrap_or(false)
            },
            false,
        );
        let cmd_or_ctrl: bool = key.get_modifiers().get_meta() || key.get_modifiers().get_ctrl();
        if cmd_or_ctrl
            && matches!(key.get_code(), KeyCode::Character('c' | 'C'))
            && self.copy_selection().is_some()
        {
            return;
        }
        if !focused_is_input
            && cmd_or_ctrl
            && matches!(key.get_code(), KeyCode::Character('a' | 'A'))
        {
            let selected_all: bool = scene.with_write(
                |tree: &mut SceneTree| {
                    let target: Option<NodeId> = tree
                        .text_selection()
                        .map(|s: SelectionState| s.get_node_id())
                        .or_else(|| {
                            tree.get_focus().focused_node().filter(|id: &NodeId| {
                                tree.get_node(*id)
                                    .map(|n: &SceneNode| matches!(n.get_kind(), NodeKind::Text(_)))
                                    .unwrap_or(false)
                            })
                        })
                        .or_else(|| {
                            tree.get_focus().hovered_node().filter(|id: &NodeId| {
                                tree.get_node(*id)
                                    .map(|n: &SceneNode| matches!(n.get_kind(), NodeKind::Text(_)))
                                    .unwrap_or(false)
                            })
                        });
                    target
                        .map(|id: NodeId| tree.select_all_text(id))
                        .unwrap_or(false)
                },
                false,
            );
            if selected_all {
                return;
            }
        }
        if focused_is_input && matches!(key.get_code(), KeyCode::Back | KeyCode::Escape) {
            let blur_cbs: Vec<(UiEventHandler, UiEvent)> =
                scene.with_write(|tree: &mut SceneTree| tree.update_focus(None), Vec::new());
            self.sync_ime_focus();
            Self::invoke_callbacks(blur_cbs);
            return;
        }
        if key.get_code() == KeyCode::Escape {
            let cleared_selection: bool = scene.with_write(
                |tree: &mut SceneTree| {
                    if tree.text_selection().is_some() {
                        tree.clear_text_selection();
                        true
                    } else {
                        false
                    }
                },
                false,
            );
            if cleared_selection {
                return;
            }
        }
        if matches!(key.get_code(), KeyCode::Back | KeyCode::Escape) {
            self.dispatch_system_event(SystemEvent::BackPressed);
        }
        if !focused_is_input {
            let key_scroll_y: f32 = match key.get_code() {
                KeyCode::ArrowDown => 48.0,
                KeyCode::ArrowUp => -48.0,
                KeyCode::PageDown => 280.0,
                KeyCode::PageUp => -280.0,
                _ => 0.0,
            };
            if key_scroll_y.abs() > 0.0 {
                self.handle_scroll(Point::new(0.0, key_scroll_y));
            }
        }
        let callbacks: Vec<(UiEventHandler, UiEvent)> = scene.with_write(
            |tree: &mut SceneTree| {
                tree.set_caret_visible(true);
                let Some(focused_id) = tree.get_focus().focused_node() else {
                    return Vec::new();
                };
                let mut out: Vec<(UiEventHandler, UiEvent)> = Vec::new();
                let mut dirty_input: bool = false;
                let mut blur_single_line: bool = false;
                if let Some(node) = tree.get_node_mut(focused_id) {
                    if let Some(h) = node
                        .get_handlers()
                        .handler_for(&UiEventKind::KeyDown(key.get_code(), key.get_modifiers()))
                    {
                        out.push((
                            h,
                            UiEvent::new(
                                focused_id,
                                UiEventKind::KeyDown(key.get_code(), key.get_modifiers()),
                            ),
                        ));
                    }
                    let is_input: bool = matches!(node.get_kind(), NodeKind::Input(_));
                    if key.get_code() == KeyCode::Backspace
                        && let NodeKind::Input(state) = node.get_mut_kind()
                    {
                        state.backspace();
                        let updated: String = state.get_value().clone();
                        dirty_input = true;
                        if let Some(h) = node
                            .get_handlers()
                            .handler_for(&UiEventKind::Change(String::new()))
                        {
                            out.push((h, UiEvent::new(focused_id, UiEventKind::Change(updated))));
                        }
                    } else if key.get_code() == KeyCode::Enter
                        && let NodeKind::Input(state) = node.get_mut_kind()
                    {
                        if state.get_multiline() {
                            state.insert_str("\n");
                            let updated: String = state.get_value().clone();
                            dirty_input = true;
                            if let Some(h) = node
                                .get_handlers()
                                .handler_for(&UiEventKind::Change(String::new()))
                            {
                                out.push((
                                    h,
                                    UiEvent::new(focused_id, UiEventKind::Change(updated)),
                                ));
                            }
                        } else {
                            blur_single_line = true;
                        }
                    } else if !is_input
                        && matches!(key.get_code(), KeyCode::Enter | KeyCode::Space)
                        && let Some(h) = node
                            .get_handlers()
                            .handler_for(&UiEventKind::Click(Point::default()))
                    {
                        let b: Rect = node.get_layout_box().get_bounds();
                        let center: Point = Point::new(
                            b.get_x() + b.get_width() * 0.5,
                            b.get_y() + b.get_height() * 0.5,
                        );
                        out.push((h, UiEvent::new(focused_id, UiEventKind::Click(center))));
                    }
                }
                if blur_single_line {
                    out.extend(tree.update_focus(None));
                } else if dirty_input {
                    let mask: u16 = DIRTY_LAYOUT | DIRTY_PAINT;
                    tree.mark_dirty(focused_id, mask | DIRTY_TEXT);
                }
                out
            },
            Vec::new(),
        );
        self.sync_ime_focus();
        Self::invoke_callbacks(callbacks);
    }

    /// Commits text into the focused `NodeKind::Input` and fires `UiEventKind::Change`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable app.
    /// - `String` - Committed UTF-8 text.
    fn handle_text_input(&mut self, text: String) {
        if text.is_empty() {
            return;
        }
        self.set_caret_blink_secs(0.0);
        let scene: SceneHandle = self.get_scene().clone();
        let callbacks: Vec<(UiEventHandler, UiEvent)> = scene.with_write(
            |tree: &mut SceneTree| {
                tree.set_caret_visible(true);
                let Some(focused_id) = tree.get_focus().focused_node() else {
                    return Vec::new();
                };
                let mut out: Vec<(UiEventHandler, UiEvent)> = Vec::new();
                let mut changed: bool = false;
                if let Some(node) = tree.get_node_mut(focused_id)
                    && let NodeKind::Input(state) = node.get_mut_kind()
                {
                    state.insert_str(&text);
                    let updated: String = state.get_value().clone();
                    changed = true;
                    if let Some(h) = node
                        .get_handlers()
                        .handler_for(&UiEventKind::Change(String::new()))
                    {
                        out.push((h, UiEvent::new(focused_id, UiEventKind::Change(updated))));
                    }
                }
                if changed {
                    let mask: u16 = DIRTY_LAYOUT | DIRTY_PAINT;
                    tree.mark_dirty(focused_id, mask | DIRTY_TEXT);
                }
                out
            },
            Vec::new(),
        );
        self.sync_ime_focus();
        Self::invoke_callbacks(callbacks);
    }

    /// Invokes collected UI event callbacks outside any `SceneTree` borrow.
    ///
    /// # Arguments
    ///
    /// - `Vec<(UiEventHandler, UiEvent)>` - Collected callback/event pairs.
    fn invoke_callbacks(callbacks: Vec<(UiEventHandler, UiEvent)>) {
        for (handler, event) in callbacks.into_iter() {
            if let Ok(mut func) = handler.try_borrow_mut() {
                func(&event);
            }
        }
    }
}
