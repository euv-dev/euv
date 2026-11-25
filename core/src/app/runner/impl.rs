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
        let scene: SceneHandle = SceneHandle::new_tree();
        let build_context: BuildContext = BuildContext::new(scene.clone(), kind);
        let dynamic_hub: DynamicUpdateHub = build_context.get_dynamic_hub().clone();
        let _: bool = build_context
            .get_breakpoint()
            .set(window_state.breakpoint());
        let _: bool = build_context
            .get_viewport_size()
            .set(window_state.logical_size());
        let _: bool = build_context
            .get_theme_mode()
            .set(window_state.get_system_theme());
        let _: bool = build_context
            .get_safe_area()
            .set(window_state.get_safe_area());
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
            build_context,
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
            touch_drag_origin: None,
            touch_drag_last: None,
            touch_dragging: false,
            scrollbar_dragging: false,
            active_pointers: HashMap::new(),
            gesture_origin: None,
            pinch_base_dist: 0.0,
            gesture_hold_secs: 0.0,
            long_press_fired: false,
            last_tap_pos: None,
            last_tap_age_secs: 10.0,
            gesture_seq: 0,
        })
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

    /// Returns `true` when any node transition animation is actively running.
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
        self.get_scene().has_active_transitions()
    }

    /// Mounts a root `Mountable` component or `NodeId` into the application's scene root with an ambient `BuildContext`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable application.
    /// - `F` - Zero-argument root builder closure invoking `view!` or components.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted root content node handle.
    pub fn mount<F, M>(&mut self, builder: F) -> NodeId
    where
        F: FnOnce() -> M,
        M: Mountable,
    {
        let ctx: BuildContext = self.get_build_context().clone();
        let root_child: NodeId = with_build_context(ctx.clone(), || {
            let mountable: M = builder();
            let mut mount_ctx: BuildContext = ctx;
            mountable.mount(&mut mount_ctx)
        });
        let scene: SceneHandle = self.get_scene().clone();
        scene.with_write(
            |tree: &mut SceneTree| {
                let root: NodeId = tree.get_root();
                tree.replace_children(root, vec![root_child]);
            },
            (),
        );
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
            WindowEvent::Resized(_, width, height, scale) => {
                let safe_scale: f32 = scale.max(0.5);
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
                    let _: bool = self.get_build_context().get_breakpoint().set_if_changed(bp);
                    let _: bool = self
                        .get_build_context()
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
            WindowEvent::SafeAreaChanged(_, insets) => {
                self.get_mut_window_state().set_safe_area(insets);
                let _: bool = self
                    .get_build_context()
                    .get_safe_area()
                    .set_if_changed(insets);
                self.set_pending_damage(DamageRegion::full());
            }
            WindowEvent::ThemeChanged(_, mode) => {
                self.get_mut_window_state().set_system_theme(mode);
                let _: bool = self
                    .get_build_context()
                    .get_theme_mode()
                    .set_if_changed(mode);
                self.set_pending_damage(DamageRegion::full());
            }
            WindowEvent::Focused(_, focused) => {
                self.get_mut_window_state().set_focused(focused);
            }
            WindowEvent::CloseRequested(_) => {
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
            WindowEvent::Lifecycle(LifecycleEvent::Terminated) => {
                self.set_running(false);
            }
            WindowEvent::RedrawRequested(_) => {
                self.set_pending_damage(DamageRegion::full());
            }
            WindowEvent::DeepLink(link) => {
                let _: bool = self.get_deep_link().set(Some(link));
            }
            WindowEvent::PointerCancel(_, _)
            | WindowEvent::KeyUp(_, _)
            | WindowEvent::Ime(_, _)
            | WindowEvent::Lifecycle(_) => {}
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
        let commands: Vec<WindowCommand> = self.get_mut_command_queue().drain();
        for cmd in commands.into_iter() {
            let Self { platform, .. } = self;
            platform.apply_command(cmd)?;
        }
        let events: Vec<WindowEvent> = {
            let Self { platform, .. } = self;
            platform.poll_events()
        };
        for ev in events.into_iter() {
            self.dispatch_window_event(ev);
        }
        flush_effects();
        let raw_dt: f32 = self.get_frame_clock().get_delta_secs();
        let anim_dt: f32 = if raw_dt > 0.0001 {
            raw_dt.clamp(1.0 / 240.0, 0.05)
        } else {
            1.0 / 60.0
        };
        let scene: SceneHandle = self.get_scene().clone();
        let has_canvas: bool = scene.with_read(
            |tree: &SceneTree| {
                tree.get_nodes().iter().any(|n: &SceneNode| {
                    n.get_alive()
                        && matches!(n.get_kind(), NodeKind::Canvas)
                        && n.get_style().get_layout().get_display() != Display::None
                })
            },
            false,
        );
        if has_canvas && !self.get_game_hosts().is_empty() {
            for host in self.get_game_hosts().iter() {
                let _: u32 = host.step(anim_dt);
            }
        }
        let _: bool =
            scene.with_write(|tree: &mut SceneTree| tree.tick_transitions(anim_dt), false);
        self.tick_gesture_timers(anim_dt);
        let viewport: Size = self.get_window_state().logical_size();
        let scale: f32 = self.get_window_state().get_scale_factor();
        let clear: Color = self.get_config().get_clear_color();
        let mut damage: DamageRegion = self.get_pending_damage();
        let (scene_rev, tree_damage): (u64, DamageRegion) = scene.with_write(
            |tree: &mut SceneTree| {
                let d: DamageRegion = tree.collect_paint_damage(scale);
                (tree.get_revision(), d)
            },
            (0, DamageRegion::full()),
        );
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
        let phys_w: u32 = ((viewport.get_width() * scale).round() as u32).max(1);
        let phys_h: u32 = ((viewport.get_height() * scale).round() as u32).max(1);
        if damage.resolve(phys_w, phys_h).is_none() {
            return Ok(self.get_mut_renderer().record_skipped_frame());
        }
        if !self
            .get_renderer()
            .get_config()
            .get_dirty_rect_optimization()
        {
            damage = DamageRegion::full();
        }
        let layout_stats: LayoutStats = scene.with_write(
            |tree: &mut SceneTree| self.get_mut_layout_engine().compute(tree, viewport),
            LayoutStats::default(),
        );
        if layout_stats.get_nodes_computed() > 0 {
            damage = DamageRegion::full();
        }
        let frame_stats: FrameStats = scene.with_read(
            |tree: &SceneTree| {
                self.get_mut_renderer().render_frame(
                    tree,
                    viewport,
                    scale,
                    clear,
                    layout_stats,
                    damage,
                )
            },
            Ok(FrameStats::default()),
        )?;
        let window_id: WindowId = self.get_window_state().get_id();
        {
            let Self {
                platform, renderer, ..
            } = self;
            if let Some(fb) = renderer.framebuffer() {
                platform.present_framebuffer_region(window_id, fb, damage)?;
            }
        }
        self.set_last_rendered_revision(scene_rev);
        self.set_pending_damage(DamageRegion::default());
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
        let mut last_stats: FrameStats = FrameStats::default();
        let mut rendered: u64 = 0;
        while self.get_running() {
            last_stats = self.step_frame()?;
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
                sleep(Duration::from_millis(16));
            }
        }
        Ok(last_stats)
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

    /// Emits a recognized cross-platform `GestureEvent` into the reactive `BuildContext::get_gesture()` signal.
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
        let _: bool = self.get_build_context().get_gesture().set(Some(ev));
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

    /// Computes distance between the first two active multi-touch pointers for pinch-to-zoom gestures.
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
        let pts: Vec<Point> = self.get_active_pointers().values().copied().collect();
        if pts.len() < 2 {
            return None;
        }
        let a: Point = pts[0];
        let b: Point = pts[1];
        let dist: f32 = (b.get_x() - a.get_x()).hypot(b.get_y() - a.get_y());
        let mid: Point = Point::new((a.get_x() + b.get_x()) * 0.5, (a.get_y() + b.get_y()) * 0.5);
        Some((dist, mid))
    }

    /// Handles pointer motion with touch pan-to-scroll, multi-touch pinch, pointer capture for sliders, hover bubbling, cursor updates, and scrollbar dragging.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable app.
    /// - `PointerEvent` - Incoming pointer move event.
    fn handle_pointer_move_event(&mut self, ptr: PointerEvent) {
        let pos: Point = ptr.get_position();
        if self
            .get_active_pointers()
            .contains_key(&ptr.get_pointer_id())
        {
            let _: Option<Point> = self
                .get_mut_active_pointers()
                .insert(ptr.get_pointer_id(), pos);
            if self.get_active_pointers().len() >= 2
                && let Some((cur_dist, mid)) = self.active_pinch_span()
            {
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
                    return;
                }
            }
        }
        let is_touch: bool = ptr.get_device() == PointerDevice::Touch;
        let origin_opt: Option<Point> = {
            let Self {
                touch_drag_origin, ..
            } = self;
            *touch_drag_origin
        };
        let is_scrollbar_drag: bool = self.get_scrollbar_dragging();
        if is_touch && let Some(origin) = origin_opt {
            let captured_drag_control: bool = is_scrollbar_drag
                || self.get_scene().with_read(
                    |tree: &SceneTree| {
                        tree.get_focus()
                            .pressed_node()
                            .and_then(|pid: NodeId| tree.get_node(pid))
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
                if !self.get_touch_dragging() && (total_dx.abs() > 10.0 || total_dy.abs() > 10.0) {
                    self.set_touch_dragging(true);
                }
                if self.get_touch_dragging() {
                    let prev_opt: Option<Point> = {
                        let Self {
                            touch_drag_last, ..
                        } = self;
                        *touch_drag_last
                    };
                    if let Some(prev) = prev_opt {
                        let scroll_dx: f32 = prev.get_x() - pos.get_x();
                        let scroll_dy: f32 = prev.get_y() - pos.get_y();
                        self.handle_scroll(Point::new(scroll_dx, scroll_dy));
                    }
                    self.set_touch_drag_last(Some(pos));
                    return;
                }
            }
            self.set_touch_drag_last(Some(pos));
        }
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
                        let view_h: f32 = lbox.get_content_bounds().get_height().max(1.0);
                        let inner_h: f32 = (screen_outer.get_height() - border_w * 2.0).max(1.0);
                        let track_margin: f32 = 3.0;
                        let track_y: f32 = screen_outer.get_y() + border_w + track_margin;
                        let track_h: f32 = (inner_h - track_margin * 2.0).max(16.0);
                        if let Some((cur_vy, total_h)) = node.virtual_scroll_metrics() {
                            let max_y: f32 = (total_h - view_h).max(0.0);
                            if max_y > 1.0 && track_h > 16.0 {
                                let ratio: f32 = (view_h / total_h.max(view_h)).clamp(0.0, 1.0);
                                let thumb_h: f32 =
                                    (track_h * ratio).clamp(20.0_f32.min(track_h), track_h);
                                let travel: f32 = (track_h - thumb_h).max(1.0);
                                let frac: f32 = ((pos.get_y() - track_y - thumb_h * 0.5) / travel)
                                    .clamp(0.0, 1.0);
                                let delta_y: f32 = frac * max_y - cur_vy;
                                if delta_y.abs() > 0.1
                                    && let Some(h) = node
                                        .get_handlers()
                                        .handler_for(&UiEventKind::Scroll(Point::new(0.0, delta_y)))
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
                                let frac: f32 = ((pos.get_y() - track_y - thumb_h * 0.5) / travel)
                                    .clamp(0.0, 1.0);
                                tree.set_scroll_offset(
                                    pressed_id,
                                    Point::new(lbox.get_scroll_offset().get_x(), frac * max_y),
                                );
                            }
                        }
                    }
                    let hit: Option<NodeId> = hit_test(tree, pos);
                    let prev: Option<NodeId> = tree.get_focus().hovered_node();
                    if prev != hit {
                        let old_chain: Vec<NodeId> = Self::ancestor_chain(tree, prev);
                        let new_chain: Vec<NodeId> = Self::ancestor_chain(tree, hit);
                        for old_id in old_chain.iter().copied() {
                            if new_chain.contains(&old_id) {
                                continue;
                            }
                            let mut visual_hover: bool = false;
                            if let Some(node) = tree.get_node_mut(old_id) {
                                node.get_mut_interaction().set_hovered(false);
                                visual_hover = node.hover_paint().is_some()
                                    || node.get_style().get_paint().get_cursor()
                                        == CursorIcon::Pointer
                                    || node.get_style().get_layout().get_overflow()
                                        == Overflow::Scroll
                                    || node
                                        .get_handlers()
                                        .handler_for(&UiEventKind::Click(Point::default()))
                                        .is_some();
                                if let Some(h) =
                                    node.get_handlers().handler_for(&UiEventKind::PointerLeave)
                                {
                                    out.push((h, UiEvent::new(old_id, UiEventKind::PointerLeave)));
                                }
                            }
                            if visual_hover {
                                tree.mark_dirty(old_id, DIRTY_PAINT);
                            }
                        }
                        tree.get_mut_focus().set_hovered(hit);
                        for new_id in new_chain.iter().copied() {
                            if old_chain.contains(&new_id) {
                                continue;
                            }
                            let mut visual_hover: bool = false;
                            if let Some(node) = tree.get_node_mut(new_id) {
                                node.get_mut_interaction().set_hovered(true);
                                visual_hover = node.hover_paint().is_some()
                                    || node.get_style().get_paint().get_cursor()
                                        == CursorIcon::Pointer
                                    || node.get_style().get_layout().get_overflow()
                                        == Overflow::Scroll
                                    || node
                                        .get_handlers()
                                        .handler_for(&UiEventKind::Click(Point::default()))
                                        .is_some();
                                if let Some(h) =
                                    node.get_handlers().handler_for(&UiEventKind::PointerEnter)
                                {
                                    out.push((h, UiEvent::new(new_id, UiEventKind::PointerEnter)));
                                }
                            }
                            if visual_hover {
                                tree.mark_dirty(new_id, DIRTY_PAINT);
                            }
                        }
                    }
                    let mut resolved_cursor: CursorIcon = CursorIcon::Default;
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
                        }
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
        let _: Option<Point> = self
            .get_mut_active_pointers()
            .insert(ptr.get_pointer_id(), pos);
        if self.get_active_pointers().len() == 1 {
            self.set_gesture_origin(Some(pos));
            self.set_gesture_hold_secs(0.0);
            self.set_long_press_fired(false);
        } else if let Some((dist, _)) = self.active_pinch_span() {
            self.set_gesture_origin(None);
            self.set_pinch_base_dist(dist);
        }
        if ptr.get_device() == PointerDevice::Touch {
            self.set_touch_drag_origin(Some(pos));
            self.set_touch_drag_last(Some(pos));
            self.set_touch_dragging(false);
        } else {
            self.set_touch_drag_origin(None);
            self.set_touch_drag_last(None);
            self.set_touch_dragging(false);
        }
        let scene: SceneHandle = self.get_scene().clone();
        let (callbacks, grabbed_scrollbar): (Vec<(UiEventHandler, UiEvent)>, bool) = scene
            .with_write(
                |tree: &mut SceneTree| {
                    let hit: Option<NodeId> = hit_test(tree, pos);
                    tree.get_mut_focus().set_hovered(hit);
                    let mut scrollbar_target: Option<NodeId> = None;
                    let mut sb_callbacks: Vec<(UiEventHandler, UiEvent)> = Vec::new();
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
                            let inner_h: f32 =
                                (screen_outer.get_height() - border_w * 2.0).max(1.0);
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
                            let max_y: f32 = (total_h - view_h).max(0.0);
                            if max_y > 1.0
                                && pos.get_x() >= inner_right - 14.0
                                && pos.get_x() <= screen_outer.right()
                            {
                                let track_margin: f32 = 3.0;
                                let track_y: f32 = screen_outer.get_y() + border_w + track_margin;
                                let track_h: f32 = (inner_h - track_margin * 2.0).max(16.0);
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
                                scrollbar_target = Some(aid);
                                break;
                            }
                        }
                    }
                    if let Some(sb_id) = scrollbar_target {
                        tree.get_mut_focus().set_pressed(Some(sb_id));
                        return (sb_callbacks, true);
                    }
                    let mut focus_target: Option<NodeId> = None;
                    let mut press_target: Option<NodeId> = hit;
                    for aid in Self::ancestor_chain(tree, hit).into_iter() {
                        if let Some(node) = tree.get_node(aid) {
                            if focus_target.is_none() && node.get_interaction().get_focusable() {
                                focus_target = Some(aid);
                            }
                            if node
                                .get_handlers()
                                .handler_for(&UiEventKind::PointerDown(pos))
                                .is_some()
                                || node
                                    .get_handlers()
                                    .handler_for(&UiEventKind::Click(pos))
                                    .is_some()
                                || node.get_style().get_paint().get_cursor() == CursorIcon::Pointer
                            {
                                press_target = Some(aid);
                                break;
                            }
                        }
                    }
                    let mut out: Vec<(UiEventHandler, UiEvent)> = tree.update_focus(focus_target);
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
                    (out, false)
                },
                (Vec::new(), false),
            );
        self.set_scrollbar_dragging(grabbed_scrollbar);
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
        let _: Option<Point> = self.get_mut_active_pointers().remove(&ptr.get_pointer_id());
        if self.get_active_pointers().is_empty() {
            let origin_opt: Option<Point> = {
                let Self { gesture_origin, .. } = self;
                *gesture_origin
            };
            self.set_gesture_origin(None);
            self.set_pinch_base_dist(0.0);
            self.set_gesture_hold_secs(0.0);
            let long_fired: bool = self.get_long_press_fired();
            self.set_long_press_fired(false);
            if let Some(origin) = origin_opt
                && !long_fired
            {
                let dx: f32 = pos.get_x() - origin.get_x();
                let dy: f32 = pos.get_y() - origin.get_y();
                let dist: f32 = dx.hypot(dy);
                if dist <= 12.0 {
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
                        self.emit_gesture(
                            GestureEventKind::Tap,
                            origin,
                            pos,
                            1.0,
                            1,
                            ptr.get_device(),
                        );
                    }
                } else if dist >= 44.0 {
                    let swipe_kind: GestureEventKind = if dx.abs() >= dy.abs() {
                        if dx > 0.0 {
                            GestureEventKind::SwipeRight
                        } else {
                            GestureEventKind::SwipeLeft
                        }
                    } else if dy > 0.0 {
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
        let suppress_click: bool = self.get_touch_dragging() || self.get_scrollbar_dragging();
        self.set_touch_drag_origin(None);
        self.set_touch_drag_last(None);
        self.set_touch_dragging(false);
        self.set_scrollbar_dragging(false);
        let scene: SceneHandle = self.get_scene().clone();
        let callbacks: Vec<(UiEventHandler, UiEvent)> = scene.with_write(
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
                if let Some(target_id) = hit {
                    if pressed != Some(target_id)
                        && let Some(node) = tree.get_node(target_id)
                        && let Some(h) = node
                            .get_handlers()
                            .handler_for(&UiEventKind::PointerUp(pos))
                    {
                        out.push((h, UiEvent::new(target_id, UiEventKind::PointerUp(pos))));
                    }
                    if !suppress_click {
                        let mut bubble: Option<NodeId> = Some(target_id);
                        while let Some(id) = bubble {
                            if let Some(node) = tree.get_node(id) {
                                if let Some(h) =
                                    node.get_handlers().handler_for(&UiEventKind::Click(pos))
                                {
                                    out.push((h, UiEvent::new(target_id, UiEventKind::Click(pos))));
                                    break;
                                }
                                bubble = node.parent_id();
                            } else {
                                break;
                            }
                        }
                    }
                }
                out
            },
            Vec::new(),
        );
        Self::invoke_callbacks(callbacks);
    }

    /// Handles wheel/pan scroll deltas with live re-hit-testing (fixing fast virtual-list scroll freeze), clamping, and scroll chaining.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable app.
    /// - `Point` - Logical scroll delta `(dx, dy)`.
    fn handle_scroll(&mut self, delta: Point) {
        let pointer_pos: Point = self.get_input_state().get_pointer_position();
        let scene: SceneHandle = self.get_scene().clone();
        let callbacks: Vec<(UiEventHandler, UiEvent)> = scene.with_write(
            |tree: &mut SceneTree| {
                let mut cursor: Option<NodeId> = tree
                    .get_focus()
                    .hovered_node()
                    .filter(|id: &NodeId| tree.get_node(*id).is_some());
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
                            let lbox: LayoutBox = node.get_layout_box();
                            let scrollable: bool = node.virtual_scroll_metrics().is_some()
                                || lbox.get_content_size().get_height()
                                    > lbox.get_content_bounds().get_height() + 1.0;
                            if scrollable {
                                cursor = Some(NodeId::new(idx as u32, node.get_generation()));
                                break;
                            }
                        }
                    }
                }
                let mut out: Vec<(UiEventHandler, UiEvent)> = Vec::new();
                while let Some(id) = cursor {
                    let next_parent: Option<NodeId> = match tree.get_node(id) {
                        Some(node) => {
                            if node.get_style().get_layout().get_overflow() == Overflow::Scroll {
                                let lbox: LayoutBox = node.get_layout_box();
                                let cur: Point = lbox.get_scroll_offset();
                                let max_x: f32 = (lbox.get_content_size().get_width()
                                    - lbox.get_content_bounds().get_width())
                                .max(0.0);
                                let max_y: f32 = (lbox.get_content_size().get_height()
                                    - lbox.get_content_bounds().get_height())
                                .max(0.0);
                                let next: Point = Point::new(
                                    (cur.get_x() + delta.get_x()).clamp(0.0, max_x),
                                    (cur.get_y() + delta.get_y()).clamp(0.0, max_y),
                                );
                                let has_custom: bool = node
                                    .get_handlers()
                                    .handler_for(&UiEventKind::Scroll(delta))
                                    .is_some();
                                if let Some(h) =
                                    node.get_handlers().handler_for(&UiEventKind::Scroll(delta))
                                {
                                    out.push((h, UiEvent::new(id, UiEventKind::Scroll(delta))));
                                }
                                if has_custom {
                                    return out;
                                }
                                let moved: bool = (next.get_x() - cur.get_x()).abs() > 0.01
                                    || (next.get_y() - cur.get_y()).abs() > 0.01;
                                if moved {
                                    tree.set_scroll_offset(id, next);
                                    return out;
                                }
                            }
                            node.parent_id()
                        }
                        None => None,
                    };
                    cursor = next_parent;
                }
                out
            },
            Vec::new(),
        );
        Self::invoke_callbacks(callbacks);
    }

    /// Handles keyboard presses on the focused node, including Backspace on `NodeKind::Input`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable app.
    /// - `KeyEvent` - Keyboard event payload.
    fn handle_key_down(&mut self, key: KeyEvent) {
        let scene: SceneHandle = self.get_scene().clone();
        let callbacks: Vec<(UiEventHandler, UiEvent)> = scene.with_write(
            |tree: &mut SceneTree| {
                let Some(focused_id) = tree.get_focus().focused_node() else {
                    return Vec::new();
                };
                let mut out: Vec<(UiEventHandler, UiEvent)> = Vec::new();
                let mut dirty_input: bool = false;
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
                    }
                }
                if dirty_input {
                    let mask: u16 = DIRTY_LAYOUT | DIRTY_PAINT;
                    tree.mark_dirty(focused_id, mask | DIRTY_TEXT);
                }
                out
            },
            Vec::new(),
        );
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
        let scene: SceneHandle = self.get_scene().clone();
        let callbacks: Vec<(UiEventHandler, UiEvent)> = scene.with_write(
            |tree: &mut SceneTree| {
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
