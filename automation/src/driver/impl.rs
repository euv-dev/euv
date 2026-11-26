use super::*;

/// Human-readable error formatting for `AutomationError`.
impl fmt::Display for AutomationError {
    /// Formats `AutomationError` into a concise diagnostic string.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Error reference.
    /// - `&mut Formatter<'_>` - Output formatter.
    ///
    /// # Returns
    ///
    /// - `fmt::Result` - Formatting status.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::NodeNotFound(msg) => write!(f, "NodeNotFound: {msg}"),
            Self::AssertionFailed(msg) => write!(f, "AssertionFailed: {msg}"),
            Self::RuntimeError(msg) => write!(f, "RuntimeError: {msg}"),
            Self::Timeout(msg) => write!(f, "Timeout: {msg}"),
        }
    }
}

/// High-level querying, pointer/keyboard/scroll interaction, assertions, and script execution for `UiAutomator`.
impl<'a> UiAutomator<'a> {
    /// Creates a `UiAutomator` attached to `app`.
    ///
    /// # Arguments
    ///
    /// - `&'a mut App` - Mutable application under test.
    ///
    /// # Returns
    ///
    /// - `Self` - Ready UI automation driver.
    #[must_use]
    pub fn new(app: &'a mut App) -> Self {
        Self { app }
    }

    /// Returns a shared reference to the wrapped `App`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Automator reference.
    ///
    /// # Returns
    ///
    /// - `&App` - Wrapped application reference.
    #[must_use]
    pub fn app_ref(&self) -> &App {
        let Self { app } = self;
        app
    }

    /// Steps one frame and converts any `PlatformError` into `AutomationError::RuntimeError`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable automator.
    ///
    /// # Returns
    ///
    /// - `Result<FrameStats, AutomationError>` - Frame telemetry.
    pub fn step(&mut self) -> Result<FrameStats, AutomationError> {
        let Self { app } = self;
        app.step_frame()
            .map_err(|err: PlatformError| AutomationError::RuntimeError(err.to_string()))
    }

    /// Queries all currently visible layout nodes matching `selector`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Automator reference.
    /// - `&UiSelector` - Semantic UI selector.
    ///
    /// # Returns
    ///
    /// - `Vec<AiLayoutNode>` - Matching inspected nodes.
    #[must_use]
    pub fn query_all(&self, selector: &UiSelector) -> Vec<AiLayoutNode> {
        let snap: AiRuntimeSnapshot = AiSchemaProtocol::snapshot(self.app_ref(), true);
        snap.root_layout_node()
            .map(|r: &AiLayoutNode| r.find_matches(selector))
            .unwrap_or_default()
    }

    /// Queries the first visible layout node matching `selector`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Automator reference.
    /// - `&UiSelector` - Semantic UI selector.
    ///
    /// # Returns
    ///
    /// - `Result<AiLayoutNode, AutomationError>` - First matching node or `NodeNotFound`.
    pub fn query(&self, selector: &UiSelector) -> Result<AiLayoutNode, AutomationError> {
        self.query_all(selector)
            .into_iter()
            .next()
            .ok_or_else(|| AutomationError::NodeNotFound(format!("{selector:?}")))
    }

    /// Returns `true` when at least one visible node matches `selector`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Automator reference.
    /// - `&UiSelector` - Semantic UI selector.
    ///
    /// # Returns
    ///
    /// - `bool` - Existence flag.
    #[must_use]
    pub fn exists(&self, selector: &UiSelector) -> bool {
        !self.query_all(selector).is_empty()
    }

    /// Waits up to `max_frames` ticks until `selector` becomes visible.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable automator.
    /// - `&UiSelector` - Target selector.
    /// - `usize` - Maximum frames to step before timing out.
    ///
    /// # Returns
    ///
    /// - `Result<AiLayoutNode, AutomationError>` - Matched node or `Timeout`.
    pub fn wait_until(
        &mut self,
        selector: &UiSelector,
        max_frames: usize,
    ) -> Result<AiLayoutNode, AutomationError> {
        for _ in 0..=max_frames {
            if let Ok(node) = self.query(selector) {
                return Ok(node);
            }
            let _: FrameStats = self.step()?;
        }
        Err(AutomationError::Timeout(format!("{selector:?}")))
    }

    /// Moves the pointer over the center of `selector` and steps a frame.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable automator.
    /// - `&UiSelector` - Target selector.
    ///
    /// # Returns
    ///
    /// - `Result<FrameStats, AutomationError>` - Rendered frame telemetry.
    pub fn hover(&mut self, selector: &UiSelector) -> Result<FrameStats, AutomationError> {
        let target: AiLayoutNode = self.query(selector)?;
        let center: Point = target.get_screen_bounds().center();
        let Self { app } = self;
        let wid: WindowId = app.get_window_state().get_id();
        app.dispatch_window_event(WindowEvent::PointerMove(
            wid,
            PointerEvent::mouse(center.get_x(), center.get_y()),
        ));
        self.step()
    }

    /// Clicks exact logical coordinates `(x, y)` (move + down + up) and steps a frame.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable automator.
    /// - `f32` - Logical X coordinate.
    /// - `f32` - Logical Y coordinate.
    ///
    /// # Returns
    ///
    /// - `Result<FrameStats, AutomationError>` - Rendered frame telemetry.
    pub fn click_at(&mut self, x: f32, y: f32) -> Result<FrameStats, AutomationError> {
        let Self { app } = self;
        let wid: WindowId = app.get_window_state().get_id();
        let ev: PointerEvent = PointerEvent::mouse(x, y);
        app.dispatch_window_event(WindowEvent::PointerMove(wid, ev));
        app.dispatch_window_event(WindowEvent::PointerDown(wid, ev));
        app.dispatch_window_event(WindowEvent::PointerUp(wid, ev));
        self.step()
    }

    /// Clicks the center of `selector` (move + down + up) and steps a frame.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable automator.
    /// - `&UiSelector` - Target selector.
    ///
    /// # Returns
    ///
    /// - `Result<FrameStats, AutomationError>` - Rendered frame telemetry.
    pub fn click(&mut self, selector: &UiSelector) -> Result<FrameStats, AutomationError> {
        let target: AiLayoutNode = self.query(selector)?;
        let center: Point = target.get_screen_bounds().center();
        self.click_at(center.get_x(), center.get_y())
    }

    /// Moves focus onto the first visible node matching `selector` and steps a frame.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable automator.
    /// - `&UiSelector` - Target focusable node selector.
    ///
    /// # Returns
    ///
    /// - `Result<FrameStats, AutomationError>` - Rendered frame telemetry.
    pub fn focus(&mut self, selector: &UiSelector) -> Result<FrameStats, AutomationError> {
        self.click(selector)
    }

    /// Performs two consecutive clicks on `selector`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable automator.
    /// - `&UiSelector` - Target selector.
    ///
    /// # Returns
    ///
    /// - `Result<FrameStats, AutomationError>` - Rendered frame telemetry after the second click.
    pub fn double_click(&mut self, selector: &UiSelector) -> Result<FrameStats, AutomationError> {
        let _: FrameStats = self.click(selector)?;
        self.click(selector)
    }

    /// Clicks `selector` to focus it, injects `text`, and steps a frame.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable automator.
    /// - `&UiSelector` - Target input selector.
    /// - `&str` - UTF-8 text string to type.
    ///
    /// # Returns
    ///
    /// - `Result<FrameStats, AutomationError>` - Rendered frame telemetry.
    pub fn type_text(
        &mut self,
        selector: &UiSelector,
        text: &str,
    ) -> Result<FrameStats, AutomationError> {
        let _: FrameStats = self.click(selector)?;
        let Self { app } = self;
        let wid: WindowId = app.get_window_state().get_id();
        app.dispatch_window_event(WindowEvent::TextInput(wid, text.to_string()));
        self.step()
    }

    /// Clicks `selector` to focus it, clears all existing characters via Backspace, and steps a frame.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable automator.
    /// - `&UiSelector` - Target input selector.
    ///
    /// # Returns
    ///
    /// - `Result<FrameStats, AutomationError>` - Rendered frame telemetry.
    pub fn clear_input(&mut self, selector: &UiSelector) -> Result<FrameStats, AutomationError> {
        let target: AiLayoutNode = self.query(selector)?;
        let char_count: usize = target
            .input_value()
            .map(|v: &str| v.chars().count())
            .unwrap_or(0);
        let _: FrameStats = self.click(selector)?;
        let Self { app } = self;
        let wid: WindowId = app.get_window_state().get_id();
        for _ in 0..char_count {
            app.dispatch_window_event(WindowEvent::KeyDown(
                wid,
                KeyEvent::new(KeyCode::Backspace, Modifiers::default(), false),
            ));
            app.dispatch_window_event(WindowEvent::KeyUp(
                wid,
                KeyEvent::new(KeyCode::Backspace, Modifiers::default(), false),
            ));
        }
        self.step()
    }

    /// Drags between explicit logical coordinates `(from_x, from_y) -> (to_x, to_y)` and steps a frame.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable automator.
    /// - `f32` - Start X coordinate.
    /// - `f32` - Start Y coordinate.
    /// - `f32` - End X coordinate.
    /// - `f32` - End Y coordinate.
    ///
    /// # Returns
    ///
    /// - `Result<FrameStats, AutomationError>` - Rendered frame telemetry.
    pub fn drag_points(
        &mut self,
        from_x: f32,
        from_y: f32,
        to_x: f32,
        to_y: f32,
    ) -> Result<FrameStats, AutomationError> {
        let Self { app } = self;
        let wid: WindowId = app.get_window_state().get_id();
        app.dispatch_window_event(WindowEvent::PointerMove(
            wid,
            PointerEvent::mouse(from_x, from_y),
        ));
        app.dispatch_window_event(WindowEvent::PointerDown(
            wid,
            PointerEvent::mouse(from_x, from_y),
        ));
        for step_idx in 1..=4 {
            let t: f32 = (step_idx as f32) * 0.25;
            let px: f32 = from_x + (to_x - from_x) * t;
            let py: f32 = from_y + (to_y - from_y) * t;
            app.dispatch_window_event(WindowEvent::PointerMove(wid, PointerEvent::mouse(px, py)));
        }
        app.dispatch_window_event(WindowEvent::PointerUp(wid, PointerEvent::mouse(to_x, to_y)));
        self.step()
    }

    /// Drags from the center of `from` to the center of `to` and steps a frame.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable automator.
    /// - `&UiSelector` - Drag start locator.
    /// - `&UiSelector` - Drag end locator.
    ///
    /// # Returns
    ///
    /// - `Result<FrameStats, AutomationError>` - Rendered frame telemetry.
    pub fn drag(
        &mut self,
        from: &UiSelector,
        to: &UiSelector,
    ) -> Result<FrameStats, AutomationError> {
        let start_pt: Point = self.query(from)?.get_screen_bounds().center();
        let end_pt: Point = self.query(to)?.get_screen_bounds().center();
        self.drag_points(
            start_pt.get_x(),
            start_pt.get_y(),
            end_pt.get_x(),
            end_pt.get_y(),
        )
    }

    /// Builds a touch `PointerEvent` for gesture synthesis.
    ///
    /// # Arguments
    ///
    /// - `u64` - Stable touch pointer identifier.
    /// - `f32` - Logical X coordinate.
    /// - `f32` - Logical Y coordinate.
    ///
    /// # Returns
    ///
    /// - `PointerEvent` - Constructed touch pointer event.
    #[must_use]
    fn make_touch_event(pid: u64, px: f32, py: f32) -> PointerEvent {
        PointerEvent::new(
            pid,
            PointerDevice::Touch,
            Point::new(px, py),
            PointerButton::Primary,
            1.0,
            Modifiers::default(),
        )
    }

    /// Synthesizes a cross-platform touch or multi-touch gesture at `(x, y)` and steps a frame.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable automator.
    /// - `AiGestureKind` - Gesture kind (`Tap`, `DoubleTap`, `Swipe`, `Pinch`, `LongPress`).
    /// - `f32` - Anchor X coordinate.
    /// - `f32` - Anchor Y coordinate.
    /// - `f32` - Swipe X displacement.
    /// - `f32` - Swipe Y displacement.
    /// - `f32` - Pinch scale factor (`<= 0.0` defaults to `1.25`).
    ///
    /// # Returns
    ///
    /// - `Result<FrameStats, AutomationError>` - Rendered frame telemetry.
    pub fn gesture(
        &mut self,
        kind: AiGestureKind,
        x: f32,
        y: f32,
        dx: f32,
        dy: f32,
        scale: f32,
    ) -> Result<FrameStats, AutomationError> {
        let Self { app } = self;
        let wid: WindowId = app.get_window_state().get_id();
        match kind {
            AiGestureKind::Tap => {
                let ev: PointerEvent = Self::make_touch_event(1, x, y);
                app.dispatch_window_event(WindowEvent::PointerDown(wid, ev));
                app.dispatch_window_event(WindowEvent::PointerUp(wid, ev));
                self.step()
            }
            AiGestureKind::DoubleTap => {
                let ev: PointerEvent = Self::make_touch_event(1, x, y);
                app.dispatch_window_event(WindowEvent::PointerDown(wid, ev));
                app.dispatch_window_event(WindowEvent::PointerUp(wid, ev));
                app.dispatch_window_event(WindowEvent::PointerDown(wid, ev));
                app.dispatch_window_event(WindowEvent::PointerUp(wid, ev));
                self.step()
            }
            AiGestureKind::Swipe => {
                app.dispatch_window_event(WindowEvent::PointerDown(
                    wid,
                    Self::make_touch_event(1, x, y),
                ));
                for step_idx in 1..=4 {
                    let t: f32 = (step_idx as f32) * 0.25;
                    app.dispatch_window_event(WindowEvent::PointerMove(
                        wid,
                        Self::make_touch_event(1, x + dx * t, y + dy * t),
                    ));
                }
                app.dispatch_window_event(WindowEvent::PointerUp(
                    wid,
                    Self::make_touch_event(1, x + dx, y + dy),
                ));
                self.step()
            }
            AiGestureKind::Pinch => {
                let factor: f32 = if scale > 0.05 { scale } else { 1.25 };
                let half_start: f32 = 40.0;
                let half_end: f32 = (half_start * factor).clamp(8.0, 240.0);
                app.dispatch_window_event(WindowEvent::PointerDown(
                    wid,
                    Self::make_touch_event(1, x - half_start, y),
                ));
                app.dispatch_window_event(WindowEvent::PointerDown(
                    wid,
                    Self::make_touch_event(2, x + half_start, y),
                ));
                app.dispatch_window_event(WindowEvent::PointerMove(
                    wid,
                    Self::make_touch_event(1, x - half_end, y),
                ));
                app.dispatch_window_event(WindowEvent::PointerMove(
                    wid,
                    Self::make_touch_event(2, x + half_end, y),
                ));
                app.dispatch_window_event(WindowEvent::PointerUp(
                    wid,
                    Self::make_touch_event(1, x - half_end, y),
                ));
                app.dispatch_window_event(WindowEvent::PointerUp(
                    wid,
                    Self::make_touch_event(2, x + half_end, y),
                ));
                self.step()
            }
            AiGestureKind::LongPress => {
                let ev: PointerEvent = Self::make_touch_event(1, x, y);
                app.dispatch_window_event(WindowEvent::PointerDown(wid, ev));
                let _: FrameStats = app
                    .step_game_frame(0.5)
                    .map_err(|err: PlatformError| AutomationError::RuntimeError(err.to_string()))?;
                app.dispatch_window_event(WindowEvent::PointerUp(wid, ev));
                self.step()
            }
        }
    }

    /// Advances `count` UI render frames (`1..=120`) and returns the latest frame telemetry.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable automator.
    /// - `usize` - Number of frames to render.
    ///
    /// # Returns
    ///
    /// - `Result<FrameStats, AutomationError>` - Latest rendered frame telemetry.
    pub fn step_frames(&mut self, count: usize) -> Result<FrameStats, AutomationError> {
        let clamped: usize = count.clamp(1, 120);
        let mut last: FrameStats = FrameStats::default();
        for _ in 0..clamped {
            last = self.step()?;
        }
        Ok(last)
    }

    /// Advances game physics and animation simulation for `frames` ticks of `dt_secs`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable automator.
    /// - `f32` - Per-frame delta time in seconds.
    /// - `usize` - Number of simulation frames (`1..=120`).
    ///
    /// # Returns
    ///
    /// - `Result<FrameStats, AutomationError>` - Latest rendered frame telemetry.
    pub fn step_game(
        &mut self,
        dt_secs: f32,
        frames: usize,
    ) -> Result<FrameStats, AutomationError> {
        let clamped: usize = frames.clamp(1, 120);
        let dt: f32 = dt_secs.clamp(0.001, 0.1);
        let Self { app } = self;
        let mut last: FrameStats = FrameStats::default();
        for _ in 0..clamped {
            last = app
                .step_game_frame(dt)
                .map_err(|err: PlatformError| AutomationError::RuntimeError(err.to_string()))?;
        }
        Ok(last)
    }

    /// Configures the active runtime FPS cap (`None` = unlimited) and steps a frame.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable automator.
    /// - `Option<u32>` - Target FPS cap.
    ///
    /// # Returns
    ///
    /// - `Result<FrameStats, AutomationError>` - Rendered frame telemetry.
    pub fn set_fps_limit(&mut self, fps: Option<u32>) -> Result<FrameStats, AutomationError> {
        let Self { app } = self;
        app.set_fps_limit(fps);
        self.step()
    }

    /// Dispatches a keyboard press by key name (`"Enter"`, `"Backspace"`, `"Space"`, `"ArrowRight"`, or single char).
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable automator.
    /// - `&str` - Key name string.
    ///
    /// # Returns
    ///
    /// - `Result<FrameStats, AutomationError>` - Rendered frame telemetry.
    pub fn press_key_name(&mut self, key: &str) -> Result<FrameStats, AutomationError> {
        let code: KeyCode = match key {
            KEY_NAME_ENTER => KeyCode::Enter,
            KEY_NAME_BACKSPACE => KeyCode::Backspace,
            KEY_NAME_DELETE => KeyCode::Delete,
            KEY_NAME_ESCAPE => KeyCode::Escape,
            KEY_NAME_TAB => KeyCode::Tab,
            KEY_NAME_SPACE | " " => KeyCode::Space,
            KEY_NAME_ARROW_LEFT => KeyCode::ArrowLeft,
            KEY_NAME_ARROW_RIGHT => KeyCode::ArrowRight,
            KEY_NAME_ARROW_UP => KeyCode::ArrowUp,
            KEY_NAME_ARROW_DOWN => KeyCode::ArrowDown,
            other => other
                .chars()
                .next()
                .map(KeyCode::Character)
                .unwrap_or(KeyCode::Unknown),
        };
        self.press_key(code)
    }

    /// Dispatches a `KeyDown` and `KeyUp` pair for `code` and steps a frame.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable automator.
    /// - `KeyCode` - Normalized key code.
    ///
    /// # Returns
    ///
    /// - `Result<FrameStats, AutomationError>` - Rendered frame telemetry.
    pub fn press_key(&mut self, code: KeyCode) -> Result<FrameStats, AutomationError> {
        let Self { app } = self;
        let wid: WindowId = app.get_window_state().get_id();
        app.dispatch_window_event(WindowEvent::KeyDown(
            wid,
            KeyEvent::new(code, Modifiers::default(), false),
        ));
        app.dispatch_window_event(WindowEvent::KeyUp(
            wid,
            KeyEvent::new(code, Modifiers::default(), false),
        ));
        self.step()
    }

    /// Scrolls at the center of `selector` by `(dx, dy)` and steps a frame.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable automator.
    /// - `&UiSelector` - Target scrollable node or descendant.
    /// - `f32` - Horizontal scroll delta.
    /// - `f32` - Vertical scroll delta.
    ///
    /// # Returns
    ///
    /// - `Result<FrameStats, AutomationError>` - Rendered frame telemetry.
    pub fn scroll(
        &mut self,
        selector: &UiSelector,
        dx: f32,
        dy: f32,
    ) -> Result<FrameStats, AutomationError> {
        let center: Point = self.query(selector)?.get_screen_bounds().center();
        let Self { app } = self;
        let wid: WindowId = app.get_window_state().get_id();
        app.dispatch_window_event(WindowEvent::PointerMove(
            wid,
            PointerEvent::mouse(center.get_x(), center.get_y()),
        ));
        app.dispatch_window_event(WindowEvent::Scroll(wid, Point::new(dx, dy)));
        self.step()
    }

    /// Resizes the window to `(width, height, scale)` immediately and steps a frame.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable automator.
    /// - `u32` - Target logical width.
    /// - `u32` - Target logical height.
    /// - `f32` - Target display scale factor.
    ///
    /// # Returns
    ///
    /// - `Result<FrameStats, AutomationError>` - Rendered frame telemetry.
    pub fn resize(
        &mut self,
        width: u32,
        height: u32,
        scale: f32,
    ) -> Result<FrameStats, AutomationError> {
        let Self { app } = self;
        let wid: WindowId = app.get_window_state().get_id();
        app.queue_window_command(WindowCommand::SetSize(wid, width, height));
        app.dispatch_window_event(WindowEvent::Resized(wid, width, height, scale));
        self.step()
    }

    /// Asserts that at least one visible node matches `selector`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Automator reference.
    /// - `&UiSelector` - Expected visible selector.
    ///
    /// # Returns
    ///
    /// - `Result<(), AutomationError>` - Ok when visible, `AssertionFailed` otherwise.
    pub fn assert_visible(&self, selector: &UiSelector) -> Result<(), AutomationError> {
        if self.exists(selector) {
            Ok(())
        } else {
            Err(AutomationError::AssertionFailed(format!(
                "Expected visible node matching {selector:?}"
            )))
        }
    }

    /// Asserts that no visible node overflows the viewport right edge.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Automator reference.
    ///
    /// # Returns
    ///
    /// - `Result<(), AutomationError>` - Ok when zero horizontal overflows exist.
    pub fn assert_no_horizontal_overflow(&self) -> Result<(), AutomationError> {
        let snap: AiRuntimeSnapshot = AiSchemaProtocol::snapshot(self.app_ref(), false);
        let overflow: Option<&AiLayoutAnomaly> = snap
            .get_anomalies()
            .iter()
            .find(|a: &&AiLayoutAnomaly| a.get_kind() == AiAnomalyKind::HorizontalOverflow);
        match overflow {
            Some(a) => Err(AutomationError::AssertionFailed(a.get_message().clone())),
            None => Ok(()),
        }
    }

    /// Asserts that no visible sibling text nodes overlap each other.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Automator reference.
    ///
    /// # Returns
    ///
    /// - `Result<(), AutomationError>` - Ok when zero sibling text overlaps exist.
    pub fn assert_no_text_overlap(&self) -> Result<(), AutomationError> {
        let snap: AiRuntimeSnapshot = AiSchemaProtocol::snapshot(self.app_ref(), false);
        let overlap: Option<&AiLayoutAnomaly> = snap
            .get_anomalies()
            .iter()
            .find(|a: &&AiLayoutAnomaly| a.get_kind() == AiAnomalyKind::TextOverlap);
        match overlap {
            Some(a) => Err(AutomationError::AssertionFailed(a.get_message().clone())),
            None => Ok(()),
        }
    }

    /// Asserts that no visible node matches `selector`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Automator reference.
    /// - `&UiSelector` - Selector expected to be absent.
    ///
    /// # Returns
    ///
    /// - `Result<(), AutomationError>` - Ok when absent, `AssertionFailed` otherwise.
    pub fn assert_not_visible(&self, selector: &UiSelector) -> Result<(), AutomationError> {
        if !self.exists(selector) {
            Ok(())
        } else {
            Err(AutomationError::AssertionFailed(format!(
                "Expected no visible node matching {selector:?}"
            )))
        }
    }

    /// Asserts that the first visible node matching `selector` has text or input value equal to `expected`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Automator reference.
    /// - `&UiSelector` - Target node selector.
    /// - `&str` - Expected exact string value.
    ///
    /// # Returns
    ///
    /// - `Result<(), AutomationError>` - Ok when text matches, `AssertionFailed` otherwise.
    pub fn assert_text_equals(
        &self,
        selector: &UiSelector,
        expected: &str,
    ) -> Result<(), AutomationError> {
        let node: AiLayoutNode = self.query(selector)?;
        let actual: Option<&str> = node.text_value().or_else(|| node.input_value());
        if actual == Some(expected) {
            Ok(())
        } else {
            Err(AutomationError::AssertionFailed(format!(
                "Expected text {:?}, got {:?}",
                expected, actual
            )))
        }
    }

    /// Executes every step in `script` sequentially and produces a structured `UiTestReport`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable automator.
    /// - `&UiTestScript` - Declarative automation test script.
    ///
    /// # Returns
    ///
    /// - `UiTestReport` - Full pass/fail execution report and final anomaly list.
    pub fn run_script(&mut self, script: &UiTestScript) -> UiTestReport {
        let mut results: Vec<UiStepResult> = Vec::with_capacity(script.get_steps().len());
        let mut passed_steps: usize = 0;
        let mut failed_steps: usize = 0;
        for (idx, step) in script.get_steps().iter().enumerate() {
            #[cfg(not(target_arch = "wasm32"))]
            let started: Instant = Instant::now();
            let outcome: Result<(), AutomationError> = match step {
                UiTestStep::Click { selector } => self.click(selector).map(|_: FrameStats| ()),
                UiTestStep::ClickAt { x, y } => self.click_at(*x, *y).map(|_: FrameStats| ()),
                UiTestStep::DoubleClick { selector } => {
                    self.double_click(selector).map(|_: FrameStats| ())
                }
                UiTestStep::Hover { selector } => self.hover(selector).map(|_: FrameStats| ()),
                UiTestStep::Focus { selector } => self.focus(selector).map(|_: FrameStats| ()),
                UiTestStep::TypeText { selector, text } => {
                    self.type_text(selector, text).map(|_: FrameStats| ())
                }
                UiTestStep::ClearInput { selector } => {
                    self.clear_input(selector).map(|_: FrameStats| ())
                }
                UiTestStep::Drag { from, to } => self.drag(from, to).map(|_: FrameStats| ()),
                UiTestStep::Gesture {
                    gesture,
                    x,
                    y,
                    dx,
                    dy,
                    scale,
                } => self
                    .gesture(*gesture, *x, *y, *dx, *dy, *scale)
                    .map(|_: FrameStats| ()),
                UiTestStep::PressKey { key } => self.press_key_name(key).map(|_: FrameStats| ()),
                UiTestStep::Scroll { selector, dx, dy } => {
                    self.scroll(selector, *dx, *dy).map(|_: FrameStats| ())
                }
                UiTestStep::Resize {
                    width,
                    height,
                    scale,
                } => self.resize(*width, *height, *scale).map(|_: FrameStats| ()),
                UiTestStep::StepFrames { count } => {
                    self.step_frames(*count).map(|_: FrameStats| ())
                }
                UiTestStep::StepGame { dt_secs, frames } => {
                    self.step_game(*dt_secs, *frames).map(|_: FrameStats| ())
                }
                UiTestStep::SetFpsLimit { fps } => self.set_fps_limit(*fps).map(|_: FrameStats| ()),
                UiTestStep::AssertVisible { selector } => self.assert_visible(selector),
                UiTestStep::AssertNotVisible { selector } => self.assert_not_visible(selector),
                UiTestStep::AssertTextEquals { selector, expected } => {
                    self.assert_text_equals(selector, expected)
                }
                UiTestStep::AssertNoHorizontalOverflow => self.assert_no_horizontal_overflow(),
                UiTestStep::AssertNoTextOverlap => self.assert_no_text_overlap(),
            };
            #[cfg(not(target_arch = "wasm32"))]
            let elapsed_us: u64 = started.elapsed().as_micros() as u64;
            #[cfg(target_arch = "wasm32")]
            let elapsed_us: u64 = 0;
            let (ok, err_msg): (bool, Option<String>) = match outcome {
                Ok(()) => {
                    passed_steps += 1;
                    (true, None)
                }
                Err(err) => {
                    failed_steps += 1;
                    (false, Some(err.to_string()))
                }
            };
            results.push(UiStepResult::new(
                idx,
                format!("{step:?}"),
                ok,
                elapsed_us,
                err_msg,
            ));
        }
        let final_snap: AiRuntimeSnapshot = AiSchemaProtocol::snapshot(self.app_ref(), false);
        UiTestReport::new(
            script.get_name().clone(),
            failed_steps == 0,
            passed_steps,
            failed_steps,
            results,
            final_snap.get_anomalies().clone(),
        )
    }
}
