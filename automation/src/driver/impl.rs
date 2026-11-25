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
        let Self { app } = self;
        let wid: WindowId = app.get_window_state().get_id();
        let ev: PointerEvent = PointerEvent::mouse(center.get_x(), center.get_y());
        app.dispatch_window_event(WindowEvent::PointerMove(wid, ev));
        app.dispatch_window_event(WindowEvent::PointerDown(wid, ev));
        app.dispatch_window_event(WindowEvent::PointerUp(wid, ev));
        self.step()
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
        let Self { app } = self;
        let wid: WindowId = app.get_window_state().get_id();
        app.dispatch_window_event(WindowEvent::PointerMove(
            wid,
            PointerEvent::mouse(start_pt.get_x(), start_pt.get_y()),
        ));
        app.dispatch_window_event(WindowEvent::PointerDown(
            wid,
            PointerEvent::mouse(start_pt.get_x(), start_pt.get_y()),
        ));
        app.dispatch_window_event(WindowEvent::PointerMove(
            wid,
            PointerEvent::mouse(end_pt.get_x(), end_pt.get_y()),
        ));
        app.dispatch_window_event(WindowEvent::PointerUp(
            wid,
            PointerEvent::mouse(end_pt.get_x(), end_pt.get_y()),
        ));
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
                UiTestStep::Hover { selector } => self.hover(selector).map(|_: FrameStats| ()),
                UiTestStep::TypeText { selector, text } => {
                    self.type_text(selector, text).map(|_: FrameStats| ())
                }
                UiTestStep::PressKey { key } => self.press_key_name(key).map(|_: FrameStats| ()),
                UiTestStep::Scroll { selector, dx, dy } => {
                    self.scroll(selector, *dx, *dy).map(|_: FrameStats| ())
                }
                UiTestStep::Resize {
                    width,
                    height,
                    scale,
                } => self.resize(*width, *height, *scale).map(|_: FrameStats| ()),
                UiTestStep::AssertVisible { selector } => self.assert_visible(selector),
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
