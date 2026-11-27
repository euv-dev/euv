use super::*;

/// Conversion and center-point helpers for `AiRect`.
impl AiRect {
    /// Converts a core `Rect` into a serializable `AiRect`.
    ///
    /// # Arguments
    ///
    /// - `Rect` - Core layout rectangle.
    ///
    /// # Returns
    ///
    /// - `Self` - Serializable rectangle.
    #[must_use]
    pub fn from_rect(rect: Rect) -> Self {
        Self::new(
            rect.get_x(),
            rect.get_y(),
            rect.get_width(),
            rect.get_height(),
        )
    }

    /// Computes the logical center point `(x + width / 2, y + height / 2)` of this rectangle.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Rectangle reference.
    ///
    /// # Returns
    ///
    /// - `Point` - Center point in logical pixels.
    #[must_use]
    pub fn center(&self) -> Point {
        Point::new(
            self.get_x() + self.get_width() * 0.5,
            self.get_y() + self.get_height() * 0.5,
        )
    }

    /// Returns the right edge coordinate `x + width`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Rectangle reference.
    ///
    /// # Returns
    ///
    /// - `f32` - Right edge coordinate.
    #[must_use]
    pub fn right(&self) -> f32 {
        self.get_x() + self.get_width()
    }

    /// Returns the bottom edge coordinate `y + height`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Rectangle reference.
    ///
    /// # Returns
    ///
    /// - `f32` - Bottom edge coordinate.
    #[must_use]
    pub fn bottom(&self) -> f32 {
        self.get_y() + self.get_height()
    }
}

/// Optional text/value/placeholder/identity accessors and flattening helpers for `AiLayoutNode`.
impl AiLayoutNode {
    /// Returns the static text content if this node is a `Text` node.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Inspected layout node reference.
    ///
    /// # Returns
    ///
    /// - `Option<&str>` - Text slice when present.
    #[must_use]
    pub fn text_value(&self) -> Option<&str> {
        let Self { text, .. } = self;
        text.as_deref()
    }

    /// Returns the placeholder text if this node is an `Input` node.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Inspected layout node reference.
    ///
    /// # Returns
    ///
    /// - `Option<&str>` - Placeholder slice when present.
    #[must_use]
    pub fn placeholder_value(&self) -> Option<&str> {
        let Self { placeholder, .. } = self;
        placeholder.as_deref()
    }

    /// Returns the current input value if this node is an `Input` node.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Inspected layout node reference.
    ///
    /// # Returns
    ///
    /// - `Option<&str>` - Input value slice when present.
    #[must_use]
    pub fn input_value(&self) -> Option<&str> {
        let Self { value, .. } = self;
        value.as_deref()
    }

    /// Returns the explicit accessibility label when set on this node.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Inspected layout node reference.
    ///
    /// # Returns
    ///
    /// - `Option<&str>` - Accessibility label slice when present.
    #[must_use]
    pub fn a11y_label_value(&self) -> Option<&str> {
        let Self { a11y_label, .. } = self;
        a11y_label.as_deref()
    }

    /// Returns the explicit accessibility hint when set on this node.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Inspected layout node reference.
    ///
    /// # Returns
    ///
    /// - `Option<&str>` - Accessibility hint slice when present.
    #[must_use]
    pub fn a11y_hint_value(&self) -> Option<&str> {
        let Self { a11y_hint, .. } = self;
        a11y_hint.as_deref()
    }

    /// Returns the resolved arena-independent `stable_key` if available.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Inspected layout node reference.
    ///
    /// # Returns
    ///
    /// - `Option<&str>` - Stable semantic key slice when present.
    #[must_use]
    pub fn stable_key_value(&self) -> Option<&str> {
        let Self { stable_key, .. } = self;
        stable_key.as_deref()
    }

    /// Returns the author-declared `test_id` if available.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Inspected layout node reference.
    ///
    /// # Returns
    ///
    /// - `Option<&str>` - Author test identifier slice when present.
    #[must_use]
    pub fn test_id_value(&self) -> Option<&str> {
        let Self { test_id, .. } = self;
        test_id.as_deref()
    }

    /// Collects all visible nodes matching `selector` in pre-order traversal.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Root layout node.
    /// - `&UiSelector` - Semantic selector.
    ///
    /// # Returns
    ///
    /// - `Vec<AiLayoutNode>` - Matching inspected nodes (honoring `UiSelector::Nth`).
    #[must_use]
    pub fn find_matches(&self, selector: &UiSelector) -> Vec<AiLayoutNode> {
        let mut all: Vec<AiLayoutNode> = Vec::new();
        self.collect_matches(selector, &mut all);
        if let UiSelector::Nth { index, .. } = selector {
            return all.into_iter().nth(*index).into_iter().collect();
        }
        all
    }

    /// Recursively collects nodes matching `selector` into `out`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Current layout node.
    /// - `&UiSelector` - Semantic selector.
    /// - `&mut Vec<AiLayoutNode>` - Output match accumulator.
    fn collect_matches(&self, selector: &UiSelector, out: &mut Vec<AiLayoutNode>) {
        if self.get_visible() && selector.matches_node(self) {
            out.push(self.clone());
        }
        for child in self.get_children().iter() {
            child.collect_matches(selector, out);
        }
    }
}

/// Optional telemetry accessors for `AiWindowInfo`.
impl AiWindowInfo {
    /// Returns the active application route string if registered via `AiAppHooks`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Window info reference.
    ///
    /// # Returns
    ///
    /// - `Option<&str>` - Active route string slice.
    #[must_use]
    pub fn active_route_value(&self) -> Option<&str> {
        let Self { active_route, .. } = self;
        active_route.as_deref()
    }

    /// Returns the active design scheme label if registered via `AiAppHooks`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Window info reference.
    ///
    /// # Returns
    ///
    /// - `Option<&str>` - Active scheme name slice.
    #[must_use]
    pub fn active_scheme_value(&self) -> Option<&str> {
        let Self { active_scheme, .. } = self;
        active_scheme.as_deref()
    }
}

/// Optional layout tree and game telemetry accessors for `AiRuntimeSnapshot`.
impl AiRuntimeSnapshot {
    /// Returns a reference to the root `AiLayoutNode` when `include_layout_tree` was requested.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Runtime snapshot reference.
    ///
    /// # Returns
    ///
    /// - `Option<&AiLayoutNode>` - Root layout node reference.
    #[must_use]
    pub fn root_layout_node(&self) -> Option<&AiLayoutNode> {
        let Self { layout_tree, .. } = self;
        layout_tree.as_ref()
    }

    /// Returns a reference to structured 3D/2D game telemetry JSON when reported by `AiAppHooks`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Runtime snapshot reference.
    ///
    /// # Returns
    ///
    /// - `Option<&Value>` - Game telemetry payload reference.
    #[must_use]
    pub fn game_stats_value(&self) -> Option<&Value> {
        let Self { game_stats, .. } = self;
        game_stats.as_ref()
    }
}

/// Optional RGBA FNV-1a digest accessor for `AiSameFrameCapture`.
impl AiSameFrameCapture {
    /// Returns the deterministic FNV-1a RGBA framebuffer hash if captured on this frame boundary.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Same-frame capture reference.
    ///
    /// # Returns
    ///
    /// - `Option<u64>` - Framebuffer RGBA FNV-1a digest when available.
    #[must_use]
    pub fn rgba_fnv1a_value(&self) -> Option<u64> {
        let Self { rgba_fnv1a, .. } = self;
        *rgba_fnv1a
    }
}

/// Fluent registration and invocation helpers for `AiAppHooks`.
impl AiAppHooks {
    /// Registers a getter returning the active application route string.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source hooks builder.
    /// - `F` - Closure returning current route string.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated hooks builder.
    #[must_use]
    pub fn on_route<F>(mut self, getter: F) -> Self
    where
        F: Fn() -> String + 'static,
    {
        let Self { route_getter, .. } = &mut self;
        *route_getter = Some(Rc::new(getter));
        self
    }

    /// Registers a getter returning the active design scheme name.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source hooks builder.
    /// - `F` - Closure returning active scheme name.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated hooks builder.
    #[must_use]
    pub fn on_scheme<F>(mut self, getter: F) -> Self
    where
        F: Fn() -> String + 'static,
    {
        let Self { scheme_getter, .. } = &mut self;
        *scheme_getter = Some(Rc::new(getter));
        self
    }

    /// Registers a getter returning structured 3D/2D game engine telemetry JSON.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source hooks builder.
    /// - `F` - Closure returning structured JSON value.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated hooks builder.
    #[must_use]
    pub fn on_game_stats<F>(mut self, getter: F) -> Self
    where
        F: Fn() -> Value + 'static,
    {
        let Self {
            game_stats_getter, ..
        } = &mut self;
        *game_stats_getter = Some(Rc::new(getter));
        self
    }

    /// Registers a navigation handler invoked by `AiAction::Navigate`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source hooks builder.
    /// - `F` - Closure accepting route string and returning `Result<(), String>`.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated hooks builder.
    #[must_use]
    pub fn on_navigate<F>(mut self, handler: F) -> Self
    where
        F: FnMut(&str) -> Result<(), String> + 'static,
    {
        let Self {
            navigate_handler, ..
        } = &mut self;
        *navigate_handler = Some(Rc::new(RefCell::new(handler)));
        self
    }

    /// Registers a theme switcher invoked by `AiAction::SetTheme`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source hooks builder.
    /// - `F` - Closure accepting `(Option<&str>, Option<&str>)` scheme and mode.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated hooks builder.
    #[must_use]
    pub fn on_set_theme<F>(mut self, handler: F) -> Self
    where
        F: FnMut(Option<&str>, Option<&str>) -> Result<(), String> + 'static,
    {
        let Self { theme_handler, .. } = &mut self;
        *theme_handler = Some(Rc::new(RefCell::new(handler)));
        self
    }

    /// Registers a custom game engine command handler invoked by `AiAction::GameControl`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source hooks builder.
    /// - `F` - Closure accepting `(command, payload)` and returning `Result<Value, String>`.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated hooks builder.
    #[must_use]
    pub fn on_game_control<F>(mut self, handler: F) -> Self
    where
        F: FnMut(&str, &Value) -> Result<Value, String> + 'static,
    {
        let Self {
            game_control_handler,
            ..
        } = &mut self;
        *game_control_handler = Some(Rc::new(RefCell::new(handler)));
        self
    }

    /// Invokes the route getter if registered.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Hooks reference.
    ///
    /// # Returns
    ///
    /// - `Option<String>` - Active route path when available.
    #[must_use]
    pub(crate) fn read_route(&self) -> Option<String> {
        let Self { route_getter, .. } = self;
        route_getter.as_ref().map(|f: &AiRouteGetter| f())
    }

    /// Invokes the scheme getter if registered.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Hooks reference.
    ///
    /// # Returns
    ///
    /// - `Option<String>` - Active design scheme name when available.
    #[must_use]
    pub(crate) fn read_scheme(&self) -> Option<String> {
        let Self { scheme_getter, .. } = self;
        scheme_getter.as_ref().map(|f: &AiSchemeGetter| f())
    }

    /// Invokes the game telemetry getter if registered.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Hooks reference.
    ///
    /// # Returns
    ///
    /// - `Option<Value>` - Structured 3D/2D game stats JSON when available.
    #[must_use]
    pub(crate) fn read_game_stats(&self) -> Option<Value> {
        let Self {
            game_stats_getter, ..
        } = self;
        game_stats_getter.as_ref().map(|f: &AiGameStatsGetter| f())
    }

    /// Executes the registered route navigation hook.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Hooks reference.
    /// - `&str` - Target route string.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>` - Navigation status or error.
    pub(crate) fn invoke_navigate(&self, route: &str) -> Result<(), String> {
        let Self {
            navigate_handler, ..
        } = self;
        let Some(handler) = navigate_handler.as_ref() else {
            return Err(ERR_NO_NAVIGATE_HOOK.to_string());
        };
        handler
            .try_borrow_mut()
            .map_err(|err: BorrowMutError| err.to_string())?(route)
    }

    /// Executes the registered theme switching hook.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Hooks reference.
    /// - `Option<&str>` - Optional design scheme name.
    /// - `Option<&str>` - Optional theme mode label (`"Light"` / `"Dark"`).
    ///
    /// # Returns
    ///
    /// - `Result<(), String>` - Theme switch status or error.
    pub(crate) fn invoke_set_theme(
        &self,
        scheme: Option<&str>,
        mode: Option<&str>,
    ) -> Result<(), String> {
        let Self { theme_handler, .. } = self;
        let Some(handler) = theme_handler.as_ref() else {
            return Err(ERR_NO_THEME_HOOK.to_string());
        };
        handler
            .try_borrow_mut()
            .map_err(|err: BorrowMutError| err.to_string())?(scheme, mode)
    }

    /// Executes the registered game control command hook.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Hooks reference.
    /// - `&str` - Command name.
    /// - `&Value` - Structured JSON payload.
    ///
    /// # Returns
    ///
    /// - `Result<Value, String>` - Structured command result JSON or error.
    pub(crate) fn invoke_game_control(
        &self,
        command: &str,
        payload: &Value,
    ) -> Result<Value, String> {
        let Self {
            game_control_handler,
            ..
        } = self;
        let Some(handler) = game_control_handler.as_ref() else {
            return Err(ERR_NO_GAME_HOOK.to_string());
        };
        handler
            .try_borrow_mut()
            .map_err(|err: BorrowMutError| err.to_string())?(command, payload)
    }
}

/// Convenience response constructors for `AiSchemaResponse`.
impl AiSchemaResponse {
    /// Builds a successful `AiSchemaResponse` wrapping `data`.
    ///
    /// # Arguments
    ///
    /// - `Value` - Structured JSON payload.
    ///
    /// # Returns
    ///
    /// - `Self` - Success response envelope.
    #[must_use]
    pub fn success(data: Value) -> Self {
        Self::new(AI_SCHEMA_PROTOCOL_VERSION.to_string(), true, None, data)
    }

    /// Builds a successful AI Schema v2 `AiSchemaResponse` wrapping `data`.
    ///
    /// # Arguments
    ///
    /// - `Value` - Structured JSON payload.
    ///
    /// # Returns
    ///
    /// - `Self` - Success v2 response envelope.
    #[must_use]
    pub fn success_v2(data: Value) -> Self {
        Self::new(AI_SCHEMA_V2_PROTOCOL_VERSION.to_string(), true, None, data)
    }

    /// Builds a failed `AiSchemaResponse` carrying `message`.
    ///
    /// # Arguments
    ///
    /// - `String` - Error diagnostic message.
    ///
    /// # Returns
    ///
    /// - `Self` - Error response envelope.
    #[must_use]
    pub fn failure(message: String) -> Self {
        Self::new(
            AI_SCHEMA_PROTOCOL_VERSION.to_string(),
            false,
            Some(message),
            Value::Null,
        )
    }

    /// Builds a failed `AiSchemaResponse` carrying both a stable error `code` and diagnostic `message`.
    ///
    /// # Arguments
    ///
    /// - `&str` - Stable machine-readable error code (e.g. `"BACKPRESSURE"`).
    /// - `String` - Human-readable diagnostic message.
    ///
    /// # Returns
    ///
    /// - `Self` - Structured error response envelope.
    #[must_use]
    pub fn failure_with_code(code: &str, message: String) -> Self {
        let mut resp: Self = Self::new(
            AI_SCHEMA_V2_PROTOCOL_VERSION.to_string(),
            false,
            Some(message),
            Value::Null,
        );
        resp.set_code(Some(code.to_string()));
        resp
    }

    /// Returns the optional error message when `ok` is `false`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Response reference.
    ///
    /// # Returns
    ///
    /// - `Option<&str>` - Error message slice.
    #[must_use]
    pub fn error_message(&self) -> Option<&str> {
        let Self { error, .. } = self;
        error.as_deref()
    }

    /// Returns the optional machine-readable error code when present.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Response reference.
    ///
    /// # Returns
    ///
    /// - `Option<&str>` - Error code slice.
    #[must_use]
    pub fn code_value(&self) -> Option<&str> {
        let Self { code, .. } = self;
        code.as_deref()
    }
}

/// Runtime snapshot capture, direct scene query, privacy redaction, anomaly detection, and JSON-RPC request dispatch for `AiSchemaProtocol`.
impl AiSchemaProtocol {
    /// Formats a `Color` as an `#RRGGBBAA` hex string using a stack `[u8; 9]` buffer.
    ///
    /// # Arguments
    ///
    /// - `Color` - Source RGBA color.
    ///
    /// # Returns
    ///
    /// - `String` - Hex RGBA string.
    #[must_use]
    fn color_to_hex(color: Color) -> String {
        let r: u8 = color.get_r();
        let g: u8 = color.get_g();
        let b: u8 = color.get_b();
        let a: u8 = color.get_a();
        let buf: [u8; 9] = [
            b'#',
            HEX_LOWER_DIGITS[(r >> 4) as usize],
            HEX_LOWER_DIGITS[(r & 0x0f) as usize],
            HEX_LOWER_DIGITS[(g >> 4) as usize],
            HEX_LOWER_DIGITS[(g & 0x0f) as usize],
            HEX_LOWER_DIGITS[(b >> 4) as usize],
            HEX_LOWER_DIGITS[(b & 0x0f) as usize],
            HEX_LOWER_DIGITS[(a >> 4) as usize],
            HEX_LOWER_DIGITS[(a & 0x0f) as usize],
        ];
        from_utf8(&buf).unwrap_or_default().to_string()
    }

    /// Returns the static canonical label for an `A11yRole` without `format!("{:?}")`.
    ///
    /// # Arguments
    ///
    /// - `A11yRole` - Semantic accessibility role.
    ///
    /// # Returns
    ///
    /// - `&'static str` - Canonical role label.
    #[must_use]
    fn a11y_role_label(role: A11yRole) -> &'static str {
        match role {
            A11yRole::Group => A11Y_ROLE_GROUP,
            A11yRole::Window => A11Y_ROLE_WINDOW,
            A11yRole::Region => A11Y_ROLE_REGION,
            A11yRole::Navigation => A11Y_ROLE_NAVIGATION,
            A11yRole::Heading => A11Y_ROLE_HEADING,
            A11yRole::Text => ROLE_TEXT,
            A11yRole::Button => ROLE_BUTTON,
            A11yRole::TextInput => A11Y_ROLE_TEXT_INPUT,
            A11yRole::Checkbox => A11Y_ROLE_CHECKBOX,
            A11yRole::Switch => A11Y_ROLE_SWITCH,
            A11yRole::Slider => A11Y_ROLE_SLIDER,
            A11yRole::Tab => A11Y_ROLE_TAB,
            A11yRole::TabList => A11Y_ROLE_TAB_LIST,
            A11yRole::List => A11Y_ROLE_LIST,
            A11yRole::ListItem => A11Y_ROLE_LIST_ITEM,
            A11yRole::Dialog => ROLE_DIALOG,
            A11yRole::Alert => A11Y_ROLE_ALERT,
            A11yRole::ProgressBar => A11Y_ROLE_PROGRESS_BAR,
            A11yRole::Image => ROLE_IMAGE,
            A11yRole::ScrollView => A11Y_ROLE_SCROLL_VIEW,
            A11yRole::Link => A11Y_ROLE_LINK,
            A11yRole::Menu => A11Y_ROLE_MENU,
            A11yRole::MenuItem => A11Y_ROLE_MENU_ITEM,
            A11yRole::Canvas => ROLE_CANVAS,
        }
    }

    /// Returns the static canonical label for a `Display` mode.
    ///
    /// # Arguments
    ///
    /// - `Display` - Box display mode.
    ///
    /// # Returns
    ///
    /// - `&'static str` - Canonical display label.
    #[must_use]
    fn display_label(display: Display) -> &'static str {
        match display {
            Display::Flex => DISPLAY_FLEX,
            Display::Block => DISPLAY_BLOCK,
            Display::Grid => DISPLAY_GRID,
            Display::None => DISPLAY_NONE,
        }
    }

    /// Returns the static canonical label for a `Position` mode.
    ///
    /// # Arguments
    ///
    /// - `Position` - Positioning mode.
    ///
    /// # Returns
    ///
    /// - `&'static str` - Canonical position label.
    #[must_use]
    fn position_label(position: Position) -> &'static str {
        match position {
            Position::Relative => POSITION_RELATIVE,
            Position::Absolute => POSITION_ABSOLUTE,
            Position::Fixed => POSITION_FIXED,
        }
    }

    /// Returns the static canonical label for an `Overflow` policy.
    ///
    /// # Arguments
    ///
    /// - `Overflow` - Overflow clipping policy.
    ///
    /// # Returns
    ///
    /// - `&'static str` - Canonical overflow label.
    #[must_use]
    fn overflow_label(overflow: Overflow) -> &'static str {
        match overflow {
            Overflow::Visible => OVERFLOW_VISIBLE,
            Overflow::Hidden => OVERFLOW_HIDDEN,
            Overflow::Scroll => OVERFLOW_SCROLL,
        }
    }

    /// Returns the static canonical label for a `FontWeight`.
    ///
    /// # Arguments
    ///
    /// - `FontWeight` - Typography weight.
    ///
    /// # Returns
    ///
    /// - `&'static str` - Canonical font weight label.
    #[must_use]
    fn font_weight_label(weight: FontWeight) -> &'static str {
        match weight {
            FontWeight::Light => WEIGHT_LIGHT,
            FontWeight::Normal => WEIGHT_NORMAL,
            FontWeight::Medium => WEIGHT_MEDIUM,
            FontWeight::SemiBold => WEIGHT_SEMI_BOLD,
            FontWeight::Bold => WEIGHT_BOLD,
        }
    }

    /// Returns the static canonical label for a `CursorIcon`.
    ///
    /// # Arguments
    ///
    /// - `CursorIcon` - Pointer cursor icon hint.
    ///
    /// # Returns
    ///
    /// - `&'static str` - Canonical cursor icon label.
    #[must_use]
    fn cursor_icon_label(cursor: CursorIcon) -> &'static str {
        match cursor {
            CursorIcon::Default => CURSOR_DEFAULT,
            CursorIcon::Pointer => CURSOR_POINTER,
            CursorIcon::Text => CURSOR_TEXT,
            CursorIcon::Crosshair => CURSOR_CROSSHAIR,
            CursorIcon::Move => CURSOR_MOVE,
            CursorIcon::NotAllowed => CURSOR_NOT_ALLOWED,
            CursorIcon::EwResize => CURSOR_EW_RESIZE,
            CursorIcon::NsResize => CURSOR_NS_RESIZE,
            CursorIcon::Grab => CURSOR_GRAB,
            CursorIcon::Grabbing => CURSOR_GRABBING,
        }
    }

    /// Returns the static canonical label for a `Breakpoint`.
    ///
    /// # Arguments
    ///
    /// - `Breakpoint` - Responsive layout breakpoint.
    ///
    /// # Returns
    ///
    /// - `&'static str` - Canonical breakpoint label.
    #[must_use]
    fn breakpoint_label(bp: Breakpoint) -> &'static str {
        match bp {
            Breakpoint::Compact => BREAKPOINT_COMPACT,
            Breakpoint::Medium => BREAKPOINT_MEDIUM,
            Breakpoint::Expanded => BREAKPOINT_EXPANDED,
        }
    }

    /// Returns the static canonical label for a `ThemeMode`.
    ///
    /// # Arguments
    ///
    /// - `ThemeMode` - Active appearance mode.
    ///
    /// # Returns
    ///
    /// - `&'static str` - Canonical theme mode label.
    #[must_use]
    fn theme_mode_label(mode: ThemeMode) -> &'static str {
        match mode {
            ThemeMode::Light => THEME_MODE_LIGHT,
            ThemeMode::Dark => THEME_MODE_DARK,
        }
    }

    /// Returns the static canonical label for a `RenderMode`.
    ///
    /// # Arguments
    ///
    /// - `RenderMode` - Active render strategy.
    ///
    /// # Returns
    ///
    /// - `&'static str` - Canonical render mode label.
    #[must_use]
    fn render_mode_label(mode: RenderMode) -> &'static str {
        match mode {
            RenderMode::Auto => RENDER_MODE_AUTO,
            RenderMode::Software => RENDER_MODE_SOFTWARE,
            RenderMode::Hardware => RENDER_MODE_HARDWARE,
            RenderMode::Hybrid => RENDER_MODE_HYBRID,
        }
    }

    /// Returns the static canonical label for a `GraphicsBackend`.
    ///
    /// # Arguments
    ///
    /// - `GraphicsBackend` - Active low-level graphics backend.
    ///
    /// # Returns
    ///
    /// - `&'static str` - Canonical backend label.
    #[must_use]
    fn graphics_backend_label(backend: GraphicsBackend) -> &'static str {
        match backend {
            GraphicsBackend::SoftwareCpu => BACKEND_SOFTWARE_CPU,
            GraphicsBackend::Metal => BACKEND_METAL,
            GraphicsBackend::Vulkan => BACKEND_VULKAN,
            GraphicsBackend::Direct3D12 => BACKEND_DIRECT3D12,
            GraphicsBackend::Direct3D11 => BACKEND_DIRECT3D11,
            GraphicsBackend::OpenGlEs => BACKEND_OPEN_GL_ES,
            GraphicsBackend::WebGpu => BACKEND_WEB_GPU,
            GraphicsBackend::WebGl2 => BACKEND_WEB_GL2,
            GraphicsBackend::Canvas2d => BACKEND_CANVAS2D,
            GraphicsBackend::Headless => BACKEND_HEADLESS,
        }
    }

    /// Returns the static canonical label for a `PlatformKind`.
    ///
    /// # Arguments
    ///
    /// - `PlatformKind` - Active host execution platform.
    ///
    /// # Returns
    ///
    /// - `&'static str` - Canonical platform label.
    #[must_use]
    fn platform_kind_label(kind: PlatformKind) -> &'static str {
        match kind {
            PlatformKind::Web => PLATFORM_WEB,
            PlatformKind::Android => PLATFORM_ANDROID,
            PlatformKind::Ios => PLATFORM_IOS,
            PlatformKind::IpadOs => PLATFORM_IPAD_OS,
            PlatformKind::MacOs => PLATFORM_MAC_OS,
            PlatformKind::Windows => PLATFORM_WINDOWS,
            PlatformKind::Linux => PLATFORM_LINUX,
            PlatformKind::Headless => PLATFORM_HEADLESS,
        }
    }

    /// Returns `true` when `haystack` contains lowercase ASCII `needle` ignoring ASCII case without allocating.
    ///
    /// # Arguments
    ///
    /// - `&str` - Source identifier slice.
    /// - `&str` - Lowercase ASCII keyword to search for.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether `needle` occurs case-insensitively inside `haystack`.
    #[must_use]
    fn contains_ascii_ignore_case(haystack: &str, needle: &str) -> bool {
        let h: &[u8] = haystack.as_bytes();
        let n: &[u8] = needle.as_bytes();
        if n.is_empty() {
            return true;
        }
        if h.len() < n.len() {
            return false;
        }
        let first: u8 = n[0];
        let limit: usize = h.len() - n.len();
        let mut idx: usize = 0;
        while idx <= limit {
            if h[idx].to_ascii_lowercase() == first
                && h[idx..(idx + n.len())].eq_ignore_ascii_case(n)
            {
                return true;
            }
            idx += 1;
        }
        false
    }

    /// Returns `true` when `identifier` contains a sensitive keyword indicating secret, password, or token data.
    ///
    /// # Arguments
    ///
    /// - `&str` - Candidate key, test identifier, label, or placeholder string.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the identifier denotes sensitive data.
    #[must_use]
    pub(crate) fn is_sensitive_identifier(identifier: &str) -> bool {
        if identifier.len() < 3 {
            return false;
        }
        SENSITIVE_KEY_TOKENS
            .iter()
            .any(|kw: &&str| Self::contains_ascii_ignore_case(identifier, kw))
    }

    /// Extracts borrowed `(stable_key, test_id)` slices from optional explicit node key and `A11yProps` with zero allocation.
    ///
    /// # Arguments
    ///
    /// - `Option<&'a str>` - Optional reconciliation key stored on the `SceneNode`.
    /// - `Option<&'a A11yProps>` - Optional explicit accessibility metadata.
    ///
    /// # Returns
    ///
    /// - `(Option<&'a str>, Option<&'a str>)` - Borrowed `(stable_key, test_id)` slices.
    #[must_use]
    fn resolve_node_identity_refs<'a>(
        raw_key: Option<&'a str>,
        a11y_opt: Option<&'a A11yProps>,
    ) -> (Option<&'a str>, Option<&'a str>) {
        let clean_key: Option<&'a str> = raw_key.map(str::trim).filter(|s: &&str| !s.is_empty());
        let hint_str: Option<&'a str> = a11y_opt
            .map(|a: &'a A11yProps| a.get_hint().trim())
            .filter(|s: &&str| !s.is_empty());
        let hint_test_id: Option<&'a str> = hint_str.and_then(|h: &'a str| {
            h.strip_prefix(TEST_ID_HINT_PREFIX)
                .map(str::trim)
                .filter(|s: &&str| !s.is_empty())
        });
        let hint_key: Option<&'a str> = hint_str.and_then(|h: &'a str| {
            h.strip_prefix(KEY_HINT_PREFIX)
                .map(str::trim)
                .filter(|s: &&str| !s.is_empty())
        });
        let key_as_test_id: Option<&'a str> = clean_key.map(|k: &'a str| {
            k.strip_prefix(TEST_ID_HINT_PREFIX)
                .map(str::trim)
                .filter(|s: &&str| !s.is_empty())
                .unwrap_or(k)
        });
        let test_id: Option<&'a str> = hint_test_id.or(key_as_test_id);
        let a11y_label: Option<&'a str> = a11y_opt
            .map(|a: &'a A11yProps| a.get_label().trim())
            .filter(|s: &&str| !s.is_empty());
        let stable_key: Option<&'a str> = clean_key.or(hint_key).or(hint_test_id).or(a11y_label);
        (stable_key, test_id)
    }

    /// Extracts owned `(stable_key, test_id)` from optional explicit node key and `A11yProps`.
    ///
    /// # Arguments
    ///
    /// - `Option<&str>` - Optional reconciliation key stored on the `SceneNode`.
    /// - `Option<&A11yProps>` - Optional explicit accessibility metadata.
    ///
    /// # Returns
    ///
    /// - `(Option<String>, Option<String>)` - Resolved `(stable_key, test_id)`.
    #[must_use]
    fn resolve_node_identity(
        raw_key: Option<&str>,
        a11y_opt: Option<&A11yProps>,
    ) -> (Option<String>, Option<String>) {
        let (stable_key, test_id): (Option<&str>, Option<&str>) =
            Self::resolve_node_identity_refs(raw_key, a11y_opt);
        (stable_key.map(str::to_string), test_id.map(str::to_string))
    }

    /// Collects non-empty reconciliation keys from all live scene nodes into an index map.
    ///
    /// # Arguments
    ///
    /// - `&mut SceneTree` - Retained scene tree.
    ///
    /// # Returns
    ///
    /// - `HashMap<u32, String>` - Mapping from `node_index` to non-empty node key.
    #[must_use]
    fn collect_live_node_keys(tree: &mut SceneTree) -> HashMap<u32, String> {
        let mut keys: HashMap<u32, String> = HashMap::new();
        let nodes: &mut Vec<SceneNode> = tree.get_mut_nodes();
        for (idx, node) in nodes.iter_mut().enumerate() {
            if !node.get_alive() {
                continue;
            }
            if let Some(k) = node.get_mut_key().as_deref()
                && !k.trim().is_empty()
            {
                let _: Option<String> = keys.insert(idx as u32, k.to_string());
            }
        }
        keys
    }

    /// Determines whether `node` holds sensitive/secret content that must be redacted in snapshots and queries.
    ///
    /// # Arguments
    ///
    /// - `&SceneNode` - Candidate scene node.
    /// - `Option<&str>` - Optional explicit `SceneNode` reconciliation key.
    /// - `Option<&str>` - Optional resolved `test_id`.
    /// - `Option<&str>` - Optional explicit accessibility label.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the node's text or input value must be masked with `[REDACTED]`.
    #[must_use]
    fn is_sensitive_node(
        node: &SceneNode,
        raw_key: Option<&str>,
        test_id: Option<&str>,
        a11y_label: Option<&str>,
    ) -> bool {
        let key_sensitive: bool = raw_key.is_some_and(Self::is_sensitive_identifier)
            || test_id.is_some_and(Self::is_sensitive_identifier);
        match node.get_kind() {
            NodeKind::Input(inp) => {
                inp.get_password()
                    || key_sensitive
                    || a11y_label.is_some_and(Self::is_sensitive_identifier)
                    || Self::is_sensitive_identifier(inp.get_placeholder())
            }
            NodeKind::Text(_) => key_sensitive,
            _ => false,
        }
    }

    /// Infers a static semantic AI role label from explicit `A11yProps` or fallback `SceneNode` heuristics.
    ///
    /// # Arguments
    ///
    /// - `&SceneNode` - Retained scene node.
    ///
    /// # Returns
    ///
    /// - `&'static str` - Role label (`"Button"`, `"Switch"`, `"Checkbox"`, `"TextInput"`, etc.).
    #[must_use]
    fn infer_role_label(node: &SceneNode) -> &'static str {
        if let Some(a11y) = node.a11y_props_ref() {
            return Self::a11y_role_label(a11y.get_role());
        }
        let clickable: bool = node
            .get_handlers()
            .handler_for(&UiEventKind::Click(Point::default()))
            .is_some()
            || node.style_ref().get_paint().get_cursor() == CursorIcon::Pointer;
        match node.get_kind() {
            NodeKind::Input(_) => ROLE_TEXT_FIELD,
            NodeKind::Text(_) => {
                if clickable {
                    ROLE_BUTTON
                } else {
                    ROLE_TEXT
                }
            }
            NodeKind::ScrollView => ROLE_SCROLL_CONTAINER,
            NodeKind::Canvas => ROLE_CANVAS,
            NodeKind::Portal => ROLE_DIALOG,
            NodeKind::Image(_) => ROLE_IMAGE,
            NodeKind::View | NodeKind::Fragment => {
                if clickable {
                    ROLE_BUTTON
                } else {
                    ROLE_CONTAINER
                }
            }
        }
    }

    /// Returns the static node kind label for `node`.
    ///
    /// # Arguments
    ///
    /// - `&SceneNode` - Retained scene node.
    ///
    /// # Returns
    ///
    /// - `&'static str` - Node kind label.
    #[must_use]
    fn node_kind_label(node: &SceneNode) -> &'static str {
        match node.get_kind() {
            NodeKind::View => KIND_VIEW,
            NodeKind::Text(_) => KIND_TEXT,
            NodeKind::Input(_) => KIND_INPUT,
            NodeKind::Image(_) => KIND_IMAGE,
            NodeKind::ScrollView => KIND_SCROLL_VIEW,
            NodeKind::Canvas => KIND_CANVAS,
            NodeKind::Portal => KIND_PORTAL,
            NodeKind::Fragment => KIND_FRAGMENT,
        }
    }

    /// Exports the full compiled semantic accessibility tree from `app` with sensitive values redacted.
    ///
    /// # Arguments
    ///
    /// - `&App` - Active application runtime.
    ///
    /// # Returns
    ///
    /// - `Vec<AiA11yNode>` - Flattened preorder list of serializable A11y nodes.
    #[must_use]
    pub fn accessibility_tree(app: &App) -> Vec<AiA11yNode> {
        let sensitive_nodes: HashMap<u32, bool> = app.get_scene().with_write(
            |tree: &mut SceneTree| {
                let keys: HashMap<u32, String> = Self::collect_live_node_keys(tree);
                let mut map: HashMap<u32, bool> = HashMap::new();
                for (idx, node) in tree.get_nodes().iter().enumerate() {
                    if !node.get_alive() {
                        continue;
                    }
                    let u_idx: u32 = idx as u32;
                    let raw_key: Option<&str> = keys.get(&u_idx).map(String::as_str);
                    let a11y_opt: Option<&A11yProps> = node.a11y_props_ref();
                    let (_, test_id): (Option<&str>, Option<&str>) =
                        Self::resolve_node_identity_refs(raw_key, a11y_opt);
                    let a11y_label: Option<&str> = a11y_opt
                        .map(|a: &A11yProps| a.get_label().as_str())
                        .filter(|s: &&str| !s.is_empty());
                    if Self::is_sensitive_node(node, raw_key, test_id, a11y_label) {
                        let is_text: bool = matches!(node.get_kind(), NodeKind::Text(_));
                        let _: Option<bool> = map.insert(u_idx, is_text);
                    }
                }
                map
            },
            HashMap::new(),
        );
        let a11y: A11yTree = app.a11y_tree();
        a11y.get_nodes()
            .iter()
            .map(|n: &A11yNode| {
                let props: &A11yProps = n.get_props();
                let node_idx: u32 = n.get_id().get_index();
                let sensitive_kind: Option<bool> = sensitive_nodes.get(&node_idx).copied();
                let safe_label: String =
                    if sensitive_kind == Some(true) && !props.get_label().is_empty() {
                        AI_SCHEMA_REDACTED_VALUE.to_string()
                    } else {
                        props.get_label().clone()
                    };
                let safe_value: String =
                    if sensitive_kind.is_some() && !props.get_value().is_empty() {
                        AI_SCHEMA_REDACTED_VALUE.to_string()
                    } else {
                        props.get_value().clone()
                    };
                let child_indices: Vec<u32> = n
                    .get_children()
                    .iter()
                    .map(|cid: &NodeId| cid.get_index())
                    .collect();
                AiA11yNode {
                    node_index: node_idx,
                    role: Self::a11y_role_label(props.get_role()).to_string(),
                    label: safe_label,
                    hint: props.get_hint().clone(),
                    value: safe_value,
                    checked: props.checked_state(),
                    disabled: props.get_disabled(),
                    focused: n.get_focused(),
                    heading_level: props.get_heading_level(),
                    bounds: AiRect::from_rect(n.get_bounds()),
                    children: child_indices,
                }
            })
            .collect()
    }

    /// Compiles a single `SceneNode` into an `AiLayoutNode` with optional recursive child compilation.
    ///
    /// # Arguments
    ///
    /// - `&SceneTree` - Retained scene tree.
    /// - `&HashMap<u32, String>` - Pre-collected reconciliation key lookup map.
    /// - `NodeId` - Target node handle.
    /// - `Point` - Accumulated ancestor scroll offset.
    /// - `bool` - Whether all ancestors are visible.
    /// - `bool` - Whether to recursively compile child `AiLayoutNode`s.
    /// - `&mut usize` - Running counter of redacted sensitive nodes.
    ///
    /// # Returns
    ///
    /// - `Option<AiLayoutNode>` - Compiled layout node if alive.
    fn build_layout_node_inner(
        tree: &SceneTree,
        keys: &HashMap<u32, String>,
        node_id: NodeId,
        scroll_offset: Point,
        ancestor_visible: bool,
        include_children: bool,
        redacted_count: &mut usize,
    ) -> Option<AiLayoutNode> {
        let node: &SceneNode = tree.get_node(node_id)?;
        let style: &Style = node.style_ref();
        let layout: LayoutStyle = style.get_layout();
        let paint: PaintStyle = style.get_paint();
        let text_style: TextStyle = style.get_text();
        let is_top_layer: bool =
            matches!(node.get_kind(), NodeKind::Portal) || layout.get_position() == Position::Fixed;
        let eff_scroll: Point = if is_top_layer {
            Point::new(0.0, 0.0)
        } else {
            scroll_offset
        };
        let lbox: LayoutBox = node.get_layout_box();
        let raw_b: Rect = lbox.get_bounds();
        let self_displayed: bool = ancestor_visible && layout.get_display() != Display::None;
        let layout_state: AiLayoutState = if !self_displayed
            || (raw_b.get_width() <= 0.0
                && raw_b.get_height() <= 0.0
                && node.get_dirty().intersects(DIRTY_LAYOUT))
        {
            AiLayoutState::Unmeasured
        } else {
            AiLayoutState::Measured
        };
        let (eff_raw_b, screen_b): (Rect, Rect) = if layout_state == AiLayoutState::Unmeasured {
            (Rect::default(), Rect::default())
        } else {
            (
                raw_b,
                Rect::new(
                    raw_b.get_x() - eff_scroll.get_x(),
                    raw_b.get_y() - eff_scroll.get_y(),
                    raw_b.get_width(),
                    raw_b.get_height(),
                ),
            )
        };
        let max_scroll_x: f32 =
            (lbox.get_content_size().get_width() - lbox.get_content_bounds().get_width()).max(0.0);
        let max_scroll_y: f32 = (lbox.get_content_size().get_height()
            - lbox.get_content_bounds().get_height())
        .max(0.0);
        let self_visible: bool = self_displayed && paint.get_opacity() > 0.0;
        let visible: bool =
            self_visible && screen_b.get_width() > 0.0 && screen_b.get_height() > 0.0;
        let interactive: bool = paint.get_cursor() == CursorIcon::Pointer
            || node.get_interaction().get_focusable()
            || matches!(node.get_kind(), NodeKind::Input(_))
            || node
                .get_handlers()
                .handler_for(&UiEventKind::Click(Point::default()))
                .is_some();
        let a11y_opt: Option<&A11yProps> = node.a11y_props_ref();
        let a11y_label_ref: Option<&str> = a11y_opt
            .map(|a: &A11yProps| a.get_label().as_str())
            .filter(|s: &&str| !s.is_empty());
        let a11y_label: Option<String> = a11y_label_ref.map(str::to_string);
        let a11y_hint: Option<String> = a11y_opt
            .map(|a: &A11yProps| a.get_hint().as_str())
            .filter(|s: &&str| !s.is_empty())
            .map(str::to_string);
        let raw_key: Option<&str> = keys.get(&node_id.get_index()).map(String::as_str);
        let (stable_key, test_id): (Option<String>, Option<String>) =
            Self::resolve_node_identity(raw_key, a11y_opt);
        let sensitive: bool =
            Self::is_sensitive_node(node, raw_key, test_id.as_deref(), a11y_label_ref);
        let input_disabled: bool = match node.get_kind() {
            NodeKind::Input(inp) => inp.get_disabled(),
            _ => false,
        };
        let disabled: bool =
            input_disabled || a11y_opt.map(A11yProps::get_disabled).unwrap_or(false);
        let mut was_redacted: bool = false;
        let (kind_str, text, placeholder, value): (
            &'static str,
            Option<String>,
            Option<String>,
            Option<String>,
        ) = match node.get_kind() {
            NodeKind::View => (KIND_VIEW, None, None, None),
            NodeKind::Text(t) => {
                if sensitive && !t.is_empty() {
                    was_redacted = true;
                    (
                        KIND_TEXT,
                        Some(AI_SCHEMA_REDACTED_VALUE.to_string()),
                        None,
                        None,
                    )
                } else {
                    (KIND_TEXT, Some(t.clone()), None, None)
                }
            }
            NodeKind::Input(inp) => {
                let safe_val: String = if sensitive && !inp.get_value().is_empty() {
                    was_redacted = true;
                    AI_SCHEMA_REDACTED_VALUE.to_string()
                } else {
                    inp.get_value().clone()
                };
                if sensitive {
                    was_redacted = true;
                }
                (
                    KIND_INPUT,
                    None,
                    Some(inp.get_placeholder().clone()),
                    Some(safe_val),
                )
            }
            NodeKind::Image(_) => (KIND_IMAGE, None, None, None),
            NodeKind::ScrollView => (KIND_SCROLL_VIEW, None, None, None),
            NodeKind::Canvas => (KIND_CANVAS, None, None, None),
            NodeKind::Portal => (KIND_PORTAL, None, None, None),
            NodeKind::Fragment => (KIND_FRAGMENT, None, None, None),
        };
        if was_redacted {
            *redacted_count = redacted_count.saturating_add(1);
        }
        let child_scroll: Point = Point::new(
            eff_scroll.get_x() + lbox.get_scroll_offset().get_x(),
            eff_scroll.get_y() + lbox.get_scroll_offset().get_y(),
        );
        let mut children: Vec<AiLayoutNode> = if include_children {
            Vec::with_capacity(node.get_children().len())
        } else {
            Vec::new()
        };
        if include_children {
            for child_id in node.get_children().iter().copied() {
                if let Some(child_node) = Self::build_layout_node_inner(
                    tree,
                    keys,
                    child_id,
                    child_scroll,
                    self_visible,
                    true,
                    redacted_count,
                ) {
                    children.push(child_node);
                }
            }
        }
        let style_summary: AiStyleSummary = AiStyleSummary {
            display: Self::display_label(layout.get_display()).to_string(),
            position: Self::position_label(layout.get_position()).to_string(),
            overflow: Self::overflow_label(layout.get_overflow()).to_string(),
            z_index: layout.get_z_index(),
            opacity: paint.get_opacity(),
            background_hex: paint.background_color().map(Self::color_to_hex),
            border_width: paint.get_border().get_width(),
            border_hex: Self::color_to_hex(paint.get_border().get_color()),
            radius: paint.get_radius().get_top_left(),
            text_color_hex: Self::color_to_hex(text_style.get_color()),
            font_size: text_style.get_font_size(),
            font_weight: Self::font_weight_label(text_style.get_weight()).to_string(),
            cursor: Self::cursor_icon_label(paint.get_cursor()).to_string(),
        };
        Some(AiLayoutNode {
            node_index: node_id.get_index(),
            generation: node_id.get_generation(),
            kind: kind_str.to_string(),
            role: Self::infer_role_label(node).to_string(),
            layout_state,
            stable_key,
            test_id,
            redacted: was_redacted,
            a11y_label,
            a11y_hint,
            disabled,
            text,
            placeholder,
            value,
            screen_bounds: AiRect::from_rect(screen_b),
            layout_bounds: AiRect::from_rect(eff_raw_b),
            content_size: AiRect::new(
                0.0,
                0.0,
                lbox.get_content_size().get_width(),
                lbox.get_content_size().get_height(),
            ),
            scroll_x: lbox.get_scroll_offset().get_x(),
            scroll_y: lbox.get_scroll_offset().get_y(),
            max_scroll_x,
            max_scroll_y,
            visible,
            interactive,
            hovered: node.get_interaction().get_hovered(),
            focused: node.get_interaction().get_focused(),
            pressed: node.get_interaction().get_pressed(),
            style: style_summary,
            children,
        })
    }

    /// Evaluates whether a raw `SceneNode` matches `selector` without allocating an `AiLayoutNode`.
    ///
    /// # Arguments
    ///
    /// - `&UiSelector` - Semantic selector to test.
    /// - `&SceneNode` - Live retained scene node.
    /// - `NodeId` - Handle of `node`.
    /// - `Option<&str>` - Optional explicit node reconciliation key.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the raw scene node matches `selector`.
    #[must_use]
    fn matches_scene_node(
        selector: &UiSelector,
        node: &SceneNode,
        node_id: NodeId,
        raw_key: Option<&str>,
    ) -> bool {
        let a11y_opt: Option<&A11yProps> = node.a11y_props_ref();
        let a11y_label: Option<&str> = a11y_opt
            .map(|a: &A11yProps| a.get_label().as_str())
            .filter(|s: &&str| !s.is_empty());
        match selector {
            UiSelector::ByText(expected) => {
                let (_, test_id): (Option<&str>, Option<&str>) =
                    Self::resolve_node_identity_refs(raw_key, a11y_opt);
                let sensitive: bool = Self::is_sensitive_node(node, raw_key, test_id, a11y_label);
                match node.get_kind() {
                    NodeKind::Text(t) => {
                        if sensitive && !t.is_empty() {
                            expected.as_str() == AI_SCHEMA_REDACTED_VALUE
                        } else {
                            t == expected
                        }
                    }
                    NodeKind::Input(inp) => {
                        if sensitive && !inp.get_value().is_empty() {
                            expected.as_str() == AI_SCHEMA_REDACTED_VALUE
                        } else {
                            inp.get_value() == expected
                        }
                    }
                    _ => false,
                }
            }
            UiSelector::ByTextContains(sub) => {
                let (_, test_id): (Option<&str>, Option<&str>) =
                    Self::resolve_node_identity_refs(raw_key, a11y_opt);
                let sensitive: bool = Self::is_sensitive_node(node, raw_key, test_id, a11y_label);
                let text_match: bool = match node.get_kind() {
                    NodeKind::Text(t) => {
                        if sensitive && !t.is_empty() {
                            AI_SCHEMA_REDACTED_VALUE.contains(sub.as_str())
                        } else {
                            t.contains(sub.as_str())
                        }
                    }
                    NodeKind::Input(inp) => {
                        let val_match: bool = if sensitive && !inp.get_value().is_empty() {
                            AI_SCHEMA_REDACTED_VALUE.contains(sub.as_str())
                        } else {
                            inp.get_value().contains(sub.as_str())
                        };
                        val_match || inp.get_placeholder().contains(sub.as_str())
                    }
                    _ => false,
                };
                text_match
                    || a11y_label
                        .map(|lbl: &str| lbl.contains(sub.as_str()))
                        .unwrap_or(false)
            }
            UiSelector::ByPlaceholder(expected) => match node.get_kind() {
                NodeKind::Input(inp) => inp.get_placeholder() == expected,
                _ => false,
            },
            UiSelector::ByInputValue(expected) => match node.get_kind() {
                NodeKind::Input(inp) => {
                    let (_, test_id): (Option<&str>, Option<&str>) =
                        Self::resolve_node_identity_refs(raw_key, a11y_opt);
                    let sensitive: bool =
                        Self::is_sensitive_node(node, raw_key, test_id, a11y_label);
                    if sensitive && !inp.get_value().is_empty() {
                        expected.as_str() == AI_SCHEMA_REDACTED_VALUE
                    } else {
                        inp.get_value() == expected
                    }
                }
                _ => false,
            },
            UiSelector::ByA11yLabel(expected) => a11y_label
                .map(|lbl: &str| lbl.eq_ignore_ascii_case(expected))
                .unwrap_or(false),
            UiSelector::ByRole(role_name) => {
                Self::infer_role_label(node).eq_ignore_ascii_case(role_name)
            }
            UiSelector::ByKind(kind_name) => {
                Self::node_kind_label(node).eq_ignore_ascii_case(kind_name)
            }
            UiSelector::ByNodeIndex(idx) => node_id.get_index() == *idx,
            UiSelector::ByTestId(expected) => {
                let (_, test_id): (Option<&str>, Option<&str>) =
                    Self::resolve_node_identity_refs(raw_key, a11y_opt);
                test_id == Some(expected.as_str())
            }
            UiSelector::ByStableKey(expected) => {
                let (stable_key, _): (Option<&str>, Option<&str>) =
                    Self::resolve_node_identity_refs(raw_key, a11y_opt);
                stable_key == Some(expected.as_str())
            }
            UiSelector::Interactive => {
                let paint: PaintStyle = node.style_ref().get_paint();
                paint.get_cursor() == CursorIcon::Pointer
                    || node.get_interaction().get_focusable()
                    || matches!(node.get_kind(), NodeKind::Input(_))
                    || node
                        .get_handlers()
                        .handler_for(&UiEventKind::Click(Point::default()))
                        .is_some()
            }
            UiSelector::Nth { inner, .. } => {
                Self::matches_scene_node(inner, node, node_id, raw_key)
            }
            UiSelector::All(preds) => preds
                .iter()
                .all(|p: &UiSelector| Self::matches_scene_node(p, node, node_id, raw_key)),
        }
    }

    /// Recursively traverses `SceneTree` directly and constructs `AiLayoutNode` only for matching visible nodes.
    ///
    /// # Arguments
    ///
    /// - `&SceneTree` - Retained scene tree.
    /// - `&HashMap<u32, String>` - Pre-collected reconciliation key map.
    /// - `&UiSelector` - Semantic selector to match.
    /// - `NodeId` - Current node handle.
    /// - `Point` - Accumulated ancestor scroll offset.
    /// - `Option<usize>` - Optional maximum number of matches to collect before stopping early.
    /// - `&mut Vec<AiLayoutNode>` - Output matched node buffer.
    fn collect_direct_scene_matches(
        tree: &SceneTree,
        keys: &HashMap<u32, String>,
        selector: &UiSelector,
        node_id: NodeId,
        scroll_offset: Point,
        max_matches: Option<usize>,
        out: &mut Vec<AiLayoutNode>,
    ) {
        if let Some(limit) = max_matches
            && out.len() >= limit
        {
            return;
        }
        let Some(node) = tree.get_node(node_id) else {
            return;
        };
        let style: &Style = node.style_ref();
        let layout: LayoutStyle = style.get_layout();
        let paint: PaintStyle = style.get_paint();
        let self_visible: bool = layout.get_display() != Display::None && paint.get_opacity() > 0.0;
        if !self_visible {
            return;
        }
        let is_top_layer: bool =
            matches!(node.get_kind(), NodeKind::Portal) || layout.get_position() == Position::Fixed;
        let eff_scroll: Point = if is_top_layer {
            Point::new(0.0, 0.0)
        } else {
            scroll_offset
        };
        let lbox: LayoutBox = node.get_layout_box();
        let raw_b: Rect = lbox.get_bounds();
        let visible: bool = raw_b.get_width() > 0.0 && raw_b.get_height() > 0.0;
        let raw_key: Option<&str> = keys.get(&node_id.get_index()).map(String::as_str);
        if visible && Self::matches_scene_node(selector, node, node_id, raw_key) {
            let mut dummy_redacted: usize = 0;
            if let Some(matched) = Self::build_layout_node_inner(
                tree,
                keys,
                node_id,
                eff_scroll,
                true,
                true,
                &mut dummy_redacted,
            ) {
                out.push(matched);
            }
            if let Some(limit) = max_matches
                && out.len() >= limit
            {
                return;
            }
        }
        let child_scroll: Point = Point::new(
            eff_scroll.get_x() + lbox.get_scroll_offset().get_x(),
            eff_scroll.get_y() + lbox.get_scroll_offset().get_y(),
        );
        for child_id in node.get_children().iter().copied() {
            Self::collect_direct_scene_matches(
                tree,
                keys,
                selector,
                child_id,
                child_scroll,
                max_matches,
                out,
            );
            if let Some(limit) = max_matches
                && out.len() >= limit
            {
                return;
            }
        }
    }

    /// Recursively checks whether at least `target_count` visible nodes match `selector` without allocating `AiLayoutNode`s.
    ///
    /// # Arguments
    ///
    /// - `&SceneTree` - Retained scene tree.
    /// - `&HashMap<u32, String>` - Pre-collected reconciliation key map.
    /// - `&UiSelector` - Semantic selector to match.
    /// - `NodeId` - Current node handle.
    /// - `usize` - Number of matches required before early exit.
    /// - `&mut usize` - Running match counter.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` as soon as `*found >= target_count`.
    fn has_direct_scene_match(
        tree: &SceneTree,
        keys: &HashMap<u32, String>,
        selector: &UiSelector,
        node_id: NodeId,
        target_count: usize,
        found: &mut usize,
    ) -> bool {
        let Some(node) = tree.get_node(node_id) else {
            return false;
        };
        let style: &Style = node.style_ref();
        let layout: LayoutStyle = style.get_layout();
        let paint: PaintStyle = style.get_paint();
        if layout.get_display() == Display::None || paint.get_opacity() <= 0.0 {
            return false;
        }
        let raw_b: Rect = node.get_layout_box().get_bounds();
        let visible: bool = raw_b.get_width() > 0.0 && raw_b.get_height() > 0.0;
        let raw_key: Option<&str> = keys.get(&node_id.get_index()).map(String::as_str);
        if visible && Self::matches_scene_node(selector, node, node_id, raw_key) {
            *found += 1;
            if *found >= target_count {
                return true;
            }
        }
        for child_id in node.get_children().iter().copied() {
            if Self::has_direct_scene_match(tree, keys, selector, child_id, target_count, found) {
                return true;
            }
        }
        false
    }

    /// Returns `true` when at least one visible node matches `selector` without constructing any `AiLayoutNode`.
    ///
    /// # Arguments
    ///
    /// - `&App` - Active application runtime.
    /// - `&UiSelector` - Semantic UI selector.
    ///
    /// # Returns
    ///
    /// - `bool` - Existence flag honoring `UiSelector::Nth` and privacy redaction.
    #[must_use]
    pub fn exists_direct(app: &App, selector: &UiSelector) -> bool {
        let target_count: usize = match selector {
            UiSelector::Nth { index, .. } => index.saturating_add(1),
            _ => 1,
        };
        app.get_scene().with_write(
            |tree: &mut SceneTree| {
                let keys: HashMap<u32, String> = Self::collect_live_node_keys(tree);
                let mut found: usize = 0;
                Self::has_direct_scene_match(
                    tree,
                    &keys,
                    selector,
                    tree.get_root(),
                    target_count,
                    &mut found,
                )
            },
            false,
        )
    }

    /// Queries only the first visible node matching `selector` directly over `SceneTree`, stopping immediately on match.
    ///
    /// # Arguments
    ///
    /// - `&App` - Active application runtime.
    /// - `&UiSelector` - Semantic UI selector.
    ///
    /// # Returns
    ///
    /// - `Option<AiLayoutNode>` - First matching inspected node honoring `UiSelector::Nth`.
    #[must_use]
    pub fn query_first_direct(app: &App, selector: &UiSelector) -> Option<AiLayoutNode> {
        let limit: usize = match selector {
            UiSelector::Nth { index, .. } => index.saturating_add(1),
            _ => 1,
        };
        let mut all: Vec<AiLayoutNode> = app.get_scene().with_write(
            |tree: &mut SceneTree| {
                let keys: HashMap<u32, String> = Self::collect_live_node_keys(tree);
                let mut matches: Vec<AiLayoutNode> = Vec::with_capacity(limit);
                Self::collect_direct_scene_matches(
                    tree,
                    &keys,
                    selector,
                    tree.get_root(),
                    Point::new(0.0, 0.0),
                    Some(limit),
                    &mut matches,
                );
                matches
            },
            Vec::new(),
        );
        if let UiSelector::Nth { index, .. } = selector {
            all.drain(..).nth(*index)
        } else {
            all.into_iter().next()
        }
    }

    /// Queries all visible nodes matching `selector` directly over `SceneTree` without building a full `AiRuntimeSnapshot`.
    ///
    /// # Arguments
    ///
    /// - `&App` - Active application runtime.
    /// - `&UiSelector` - Semantic UI selector.
    ///
    /// # Returns
    ///
    /// - `Vec<AiLayoutNode>` - Matching inspected nodes honoring `UiSelector::Nth` and privacy redaction.
    #[must_use]
    pub fn query_nodes_direct(app: &App, selector: &UiSelector) -> Vec<AiLayoutNode> {
        let max_matches: Option<usize> = match selector {
            UiSelector::Nth { index, .. } => Some(index.saturating_add(1)),
            _ => None,
        };
        let mut all: Vec<AiLayoutNode> = app.get_scene().with_write(
            |tree: &mut SceneTree| {
                let keys: HashMap<u32, String> = Self::collect_live_node_keys(tree);
                let mut matches: Vec<AiLayoutNode> = Vec::new();
                Self::collect_direct_scene_matches(
                    tree,
                    &keys,
                    selector,
                    tree.get_root(),
                    Point::new(0.0, 0.0),
                    max_matches,
                    &mut matches,
                );
                matches
            },
            Vec::new(),
        );
        if let UiSelector::Nth { index, .. } = selector {
            return all.drain(..).nth(*index).into_iter().collect();
        }
        all
    }

    /// Scans `SceneTree` directly for layout anomalies without allocating an `AiLayoutNode` tree or per-node scratch vectors.
    ///
    /// # Arguments
    ///
    /// - `&SceneTree` - Retained scene tree.
    /// - `NodeId` - Current node handle.
    /// - `Point` - Accumulated ancestor scroll offset.
    /// - `f32` - Active logical viewport width.
    /// - `(bool, bool)` - `(ancestor_displayed, ancestor_scroll_clipped)` traversal flags.
    /// - `&mut Vec<(u32, AiRect)>` - Reusable sibling text-bounds scratch stack.
    /// - `&mut Vec<AiLayoutAnomaly>` - Output anomaly list.
    fn detect_anomalies_direct(
        tree: &SceneTree,
        node_id: NodeId,
        scroll_offset: Point,
        viewport_w: f32,
        ancestor_flags: (bool, bool),
        text_scratch: &mut Vec<(u32, AiRect)>,
        out: &mut Vec<AiLayoutAnomaly>,
    ) {
        let (ancestor_displayed, ancestor_scroll_clipped): (bool, bool) = ancestor_flags;
        let Some(node) = tree.get_node(node_id) else {
            return;
        };
        let style: &Style = node.style_ref();
        let layout: LayoutStyle = style.get_layout();
        let paint: PaintStyle = style.get_paint();
        let displayed: bool = ancestor_displayed
            && layout.get_display() != Display::None
            && paint.get_opacity() > 0.0;
        let is_top_layer: bool =
            matches!(node.get_kind(), NodeKind::Portal) || layout.get_position() == Position::Fixed;
        let eff_scroll: Point = if is_top_layer {
            Point::new(0.0, 0.0)
        } else {
            scroll_offset
        };
        let lbox: LayoutBox = node.get_layout_box();
        let raw_b: Rect = lbox.get_bounds();
        let b: AiRect = AiRect::new(
            raw_b.get_x() - eff_scroll.get_x(),
            raw_b.get_y() - eff_scroll.get_y(),
            raw_b.get_width(),
            raw_b.get_height(),
        );
        let visible: bool = displayed && b.get_width() > 0.0 && b.get_height() > 0.0;
        let kind_str: &'static str = Self::node_kind_label(node);
        if visible && !ancestor_scroll_clipped && b.right() > viewport_w + 2.0 {
            out.push(AiLayoutAnomaly::new(
                AiAnomalyKind::HorizontalOverflow,
                node_id.get_index(),
                format!(
                    "Node #{} ({}) overflows viewport right edge ({:.1} > {:.1})",
                    node_id.get_index(),
                    kind_str,
                    b.right(),
                    viewport_w
                ),
                b,
            ));
        }
        if visible && !ancestor_scroll_clipped && b.get_x() < -2.0 {
            out.push(AiLayoutAnomaly::new(
                AiAnomalyKind::NegativeBounds,
                node_id.get_index(),
                format!(
                    "Node #{} ({}) extends past viewport left edge (x={:.1})",
                    node_id.get_index(),
                    kind_str,
                    b.get_x()
                ),
                b,
            ));
        }
        let is_content_node: bool = match node.get_kind() {
            NodeKind::Input(_) | NodeKind::Canvas | NodeKind::Image(_) => true,
            NodeKind::Text(t) => !t.trim().is_empty(),
            _ => false,
        };
        if displayed && is_content_node && (b.get_width() <= 0.0 || b.get_height() <= 0.0) {
            out.push(AiLayoutAnomaly::new(
                AiAnomalyKind::ZeroSizeVisible,
                node_id.get_index(),
                format!(
                    "Displayed {} node #{} collapsed to {:.1}x{:.1}px",
                    kind_str,
                    node_id.get_index(),
                    b.get_width(),
                    b.get_height()
                ),
                b,
            ));
        }
        let children: &[NodeId] = node.get_children().as_slice();
        if children.is_empty() {
            return;
        }
        let child_scroll: Point = Point::new(
            eff_scroll.get_x() + lbox.get_scroll_offset().get_x(),
            eff_scroll.get_y() + lbox.get_scroll_offset().get_y(),
        );
        if children.len() >= 2 {
            let mark: usize = text_scratch.len();
            for child_id in children.iter().copied() {
                if let Some(child) = tree.get_node(child_id)
                    && matches!(child.get_kind(), NodeKind::Text(_))
                {
                    let cstyle: &Style = child.style_ref();
                    let clayout: LayoutStyle = cstyle.get_layout();
                    let cdisp: bool = displayed
                        && clayout.get_display() != Display::None
                        && cstyle.get_paint().get_opacity() > 0.0;
                    if cdisp {
                        let cb_raw: Rect = child.get_layout_box().get_bounds();
                        let cs: Point = if clayout.get_position() == Position::Fixed {
                            Point::new(0.0, 0.0)
                        } else {
                            child_scroll
                        };
                        let cb: AiRect = AiRect::new(
                            cb_raw.get_x() - cs.get_x(),
                            cb_raw.get_y() - cs.get_y(),
                            cb_raw.get_width(),
                            cb_raw.get_height(),
                        );
                        if cb.get_width() > 0.0 && cb.get_height() > 0.0 {
                            text_scratch.push((child_id.get_index(), cb));
                        }
                    }
                }
            }
            let end: usize = text_scratch.len();
            for i in mark..end {
                let (idx_a, a): (u32, AiRect) = text_scratch[i];
                for &(idx_c, c) in text_scratch[(i + 1)..end].iter() {
                    let ix0: f32 = a.get_x().max(c.get_x());
                    let iy0: f32 = a.get_y().max(c.get_y());
                    let ix1: f32 = a.right().min(c.right());
                    let iy1: f32 = a.bottom().min(c.bottom());
                    if (ix1 - ix0) > 2.0 && (iy1 - iy0) > 2.0 {
                        out.push(AiLayoutAnomaly::new(
                            AiAnomalyKind::TextOverlap,
                            idx_c,
                            format!(
                                "Sibling text nodes #{} and #{} overlap by {:.1}x{:.1}px",
                                idx_a,
                                idx_c,
                                ix1 - ix0,
                                iy1 - iy0
                            ),
                            c,
                        ));
                    }
                }
            }
            text_scratch.truncate(mark);
        }
        let next_scroll_clipped: bool = ancestor_scroll_clipped
            || (layout.get_overflow() == Overflow::Scroll
                && b.get_width() > 0.0
                && b.right() <= viewport_w + 2.0
                && b.get_x() >= -2.0);
        for child_id in children.iter().copied() {
            Self::detect_anomalies_direct(
                tree,
                child_id,
                child_scroll,
                viewport_w,
                (displayed, next_scroll_clipped),
                text_scratch,
                out,
            );
        }
    }

    /// Scans `node` recursively to detect viewport overflow, negative bounds, zero-size visible content, and sibling text overlaps.
    ///
    /// # Arguments
    ///
    /// - `&AiLayoutNode` - Subtree root to audit.
    /// - `f32` - Active logical viewport width.
    /// - `bool` - Whether all ancestors are displayed with non-zero opacity.
    /// - `bool` - Whether an in-bounds `Overflow::Scroll` ancestor clips horizontal overflow.
    /// - `&mut Vec<AiLayoutAnomaly>` - Output anomaly list.
    fn detect_anomalies(
        node: &AiLayoutNode,
        viewport_w: f32,
        ancestor_displayed: bool,
        ancestor_scroll_clipped: bool,
        out: &mut Vec<AiLayoutAnomaly>,
    ) {
        let b: AiRect = node.get_screen_bounds();
        let style: &AiStyleSummary = node.get_style();
        let displayed: bool =
            ancestor_displayed && style.get_display() != DISPLAY_NONE && style.get_opacity() > 0.0;
        if node.get_visible() && !ancestor_scroll_clipped && b.right() > viewport_w + 2.0 {
            out.push(AiLayoutAnomaly::new(
                AiAnomalyKind::HorizontalOverflow,
                node.get_node_index(),
                format!(
                    "Node #{} ({}) overflows viewport right edge ({:.1} > {:.1})",
                    node.get_node_index(),
                    node.get_kind(),
                    b.right(),
                    viewport_w
                ),
                b,
            ));
        }
        if node.get_visible() && !ancestor_scroll_clipped && b.get_x() < -2.0 {
            out.push(AiLayoutAnomaly::new(
                AiAnomalyKind::NegativeBounds,
                node.get_node_index(),
                format!(
                    "Node #{} ({}) extends past viewport left edge (x={:.1})",
                    node.get_node_index(),
                    node.get_kind(),
                    b.get_x()
                ),
                b,
            ));
        }
        let is_content_node: bool = node.get_kind() == KIND_INPUT
            || node.get_kind() == KIND_CANVAS
            || node.get_kind() == KIND_IMAGE
            || (node.get_kind() == KIND_TEXT
                && node
                    .text_value()
                    .map(|t: &str| !t.trim().is_empty())
                    .unwrap_or(false));
        if displayed && is_content_node && (b.get_width() <= 0.0 || b.get_height() <= 0.0) {
            out.push(AiLayoutAnomaly::new(
                AiAnomalyKind::ZeroSizeVisible,
                node.get_node_index(),
                format!(
                    "Displayed {} node #{} collapsed to {:.1}x{:.1}px",
                    node.get_kind(),
                    node.get_node_index(),
                    b.get_width(),
                    b.get_height()
                ),
                b,
            ));
        }
        if node.get_children().len() >= 2 {
            let text_children: Vec<&AiLayoutNode> = node
                .get_children()
                .iter()
                .filter(|c: &&AiLayoutNode| c.get_visible() && c.get_kind() == KIND_TEXT)
                .collect();
            for i in 0..text_children.len() {
                let a: AiRect = text_children[i].get_screen_bounds();
                for j in (i + 1)..text_children.len() {
                    let c: AiRect = text_children[j].get_screen_bounds();
                    let ix0: f32 = a.get_x().max(c.get_x());
                    let iy0: f32 = a.get_y().max(c.get_y());
                    let ix1: f32 = a.right().min(c.right());
                    let iy1: f32 = a.bottom().min(c.bottom());
                    if (ix1 - ix0) > 2.0 && (iy1 - iy0) > 2.0 {
                        out.push(AiLayoutAnomaly::new(
                            AiAnomalyKind::TextOverlap,
                            text_children[j].get_node_index(),
                            format!(
                                "Sibling text nodes #{} and #{} overlap by {:.1}x{:.1}px",
                                text_children[i].get_node_index(),
                                text_children[j].get_node_index(),
                                ix1 - ix0,
                                iy1 - iy0
                            ),
                            c,
                        ));
                    }
                }
            }
        }
        let next_scroll_clipped: bool = ancestor_scroll_clipped
            || (style.get_overflow() == OVERFLOW_SCROLL
                && b.get_width() > 0.0
                && b.right() <= viewport_w + 2.0
                && b.get_x() >= -2.0);
        for child in node.get_children().iter() {
            Self::detect_anomalies(child, viewport_w, displayed, next_scroll_clipped, out);
        }
    }

    /// Captures a snapshot using an explicit schema version tag (`euv.ai.schema.v1` or `euv.ai.schema.v2`).
    ///
    /// # Arguments
    ///
    /// - `&App` - Active application runtime.
    /// - `bool` - Whether to include the full recursive `layout_tree`.
    /// - `Option<&AiAppHooks>` - Optional host route/theme/game telemetry hooks.
    /// - `&str` - Schema version string (`AI_SCHEMA_PROTOCOL_VERSION` or `AI_SCHEMA_V2_PROTOCOL_VERSION`).
    ///
    /// # Returns
    ///
    /// - `AiRuntimeSnapshot` - Complete serializable runtime & layout snapshot.
    #[must_use]
    fn snapshot_versioned(
        app: &App,
        include_layout_tree: bool,
        hooks: Option<&AiAppHooks>,
        schema_ver: &str,
    ) -> AiRuntimeSnapshot {
        let ws: WindowState = app.get_window_state();
        let stats: FrameStats = app.get_renderer().get_last_stats();
        let active_route: Option<String> = hooks.and_then(AiAppHooks::read_route);
        let active_scheme: Option<String> = hooks.and_then(AiAppHooks::read_scheme);
        let game_stats: Option<Value> = hooks.and_then(AiAppHooks::read_game_stats);
        let win_info: AiWindowInfo = AiWindowInfo {
            window_id: ws.get_id().get_value(),
            title: app.get_config().get_window().get_title().clone(),
            logical_width: ws.get_logical_width(),
            logical_height: ws.get_logical_height(),
            physical_width: ws.get_physical_width(),
            physical_height: ws.get_physical_height(),
            scale_factor: ws.get_scale_factor(),
            breakpoint: Self::breakpoint_label(ws.breakpoint()).to_string(),
            theme_mode: Self::theme_mode_label(app.get_context().get_theme_mode().get_untracked())
                .to_string(),
            active_route,
            active_scheme,
            focused: ws.get_focused(),
        };
        let (live_count, rev, focus_mgr, root_tree, redacted_count, direct_anomalies): (
            usize,
            u64,
            FocusManager,
            Option<AiLayoutNode>,
            usize,
            Vec<AiLayoutAnomaly>,
        ) = app.get_scene().with_write(
            |tree: &mut SceneTree| {
                let mut redacted: usize = 0;
                let mut anomalies: Vec<AiLayoutAnomaly> = Vec::new();
                let root_node: Option<AiLayoutNode> = if include_layout_tree {
                    let keys: HashMap<u32, String> = Self::collect_live_node_keys(tree);
                    Self::build_layout_node_inner(
                        tree,
                        &keys,
                        tree.get_root(),
                        Point::new(0.0, 0.0),
                        true,
                        true,
                        &mut redacted,
                    )
                } else {
                    let mut text_scratch: Vec<(u32, AiRect)> = Vec::with_capacity(16);
                    Self::detect_anomalies_direct(
                        tree,
                        tree.get_root(),
                        Point::new(0.0, 0.0),
                        ws.get_logical_width(),
                        (true, false),
                        &mut text_scratch,
                        &mut anomalies,
                    );
                    None
                };
                (
                    tree.live_node_count(),
                    tree.get_revision(),
                    tree.get_focus(),
                    root_node,
                    redacted,
                    anomalies,
                )
            },
            (0, 0, FocusManager::default(), None, 0, Vec::new()),
        );
        let measured_fps: f32 = app.get_context().get_live_fps().get_untracked();
        let frame_time_ms: f32 =
            (app.get_frame_clock().get_delta_secs() * 1000.0 * 100.0).round() * 0.01;
        let frame_id: u64 = stats.get_frame_index();
        let perf_info: AiPerfInfo = AiPerfInfo {
            frame_index: frame_id,
            active_mode: Self::render_mode_label(stats.get_active_mode()).to_string(),
            active_backend: Self::graphics_backend_label(stats.get_active_backend()).to_string(),
            measured_fps,
            frame_time_ms,
            fps_limit: app.fps_limit(),
            has_active_animations: app.has_active_animations(),
            layout_nodes: stats.get_layout_nodes(),
            layout_skipped: stats.get_layout_skipped(),
            draw_commands: stats.get_draw_commands(),
            draw_batches: stats.get_draw_batches(),
            pixels_shaded: stats.get_pixels_shaded(),
            frame_hash: stats.get_frame_hash(),
            live_node_count: live_count,
            scene_revision: rev,
        };
        let rstats: ReactiveStats = reactive_stats();
        let reactive_info: AiReactiveInfo = AiReactiveInfo::new(
            rstats.get_live_signals(),
            rstats.get_live_memos(),
            rstats.get_live_effects(),
            rstats.get_live_scopes(),
            rstats.get_clock(),
        );
        let ptr_pos: Point = app.get_input_state().get_pointer_position();
        let interaction: AiInteractionInfo = AiInteractionInfo::new(
            focus_mgr.hovered_node().map(|n: NodeId| n.get_index()),
            focus_mgr.focused_node().map(|n: NodeId| n.get_index()),
            focus_mgr.pressed_node().map(|n: NodeId| n.get_index()),
            Self::cursor_icon_label(app.get_last_cursor()).to_string(),
            ptr_pos.get_x(),
            ptr_pos.get_y(),
        );
        let caps: PlatformCapabilities = app.platform_capabilities();
        let a11y_nodes: Vec<AiA11yNode> = Self::accessibility_tree(app);
        let protocols: Vec<String> = app
            .get_services()
            .get_protocols()
            .get_registrations()
            .iter()
            .map(|r: &ProtocolRegistration| r.get_scheme().clone())
            .collect();
        let runtime_meta: AiRuntimeMeta = AiRuntimeMeta::new(
            Self::platform_kind_label(app.platform_kind()).to_string(),
            caps.get_hardware_gpu(),
            caps.get_touch(),
            protocols,
            a11y_nodes.len(),
        );
        let mut anomalies: Vec<AiLayoutAnomaly> = direct_anomalies;
        if let Some(root_ref) = root_tree.as_ref() {
            Self::detect_anomalies(
                root_ref,
                ws.get_logical_width(),
                true,
                false,
                &mut anomalies,
            );
        }
        AiRuntimeSnapshot {
            protocol_version: schema_ver.to_string(),
            schema_version: schema_ver.to_string(),
            frame_id,
            scene_revision: rev,
            redacted_field_count: redacted_count,
            window: win_info,
            performance: perf_info,
            reactive: reactive_info,
            interaction,
            runtime: runtime_meta,
            game_stats,
            accessibility_tree: a11y_nodes,
            layout_tree: root_tree,
            anomalies,
        }
    }

    /// Captures a complete v1 `AiRuntimeSnapshot` from a live `App` without custom host hooks.
    ///
    /// # Arguments
    ///
    /// - `&App` - Active application runtime.
    /// - `bool` - Whether to include the full recursive `layout_tree`.
    ///
    /// # Returns
    ///
    /// - `AiRuntimeSnapshot` - Complete serializable runtime & layout snapshot.
    #[must_use]
    pub fn snapshot(app: &App, include_layout_tree: bool) -> AiRuntimeSnapshot {
        Self::snapshot_with_hooks(app, include_layout_tree, None)
    }

    /// Captures a complete v1 `AiRuntimeSnapshot` enriched with optional host `AiAppHooks` telemetry.
    ///
    /// # Arguments
    ///
    /// - `&App` - Active application runtime.
    /// - `bool` - Whether to include the full recursive `layout_tree`.
    /// - `Option<&AiAppHooks>` - Optional host route/theme/game telemetry hooks.
    ///
    /// # Returns
    ///
    /// - `AiRuntimeSnapshot` - Complete serializable runtime, reactive, A11y, game, & layout snapshot.
    #[must_use]
    pub fn snapshot_with_hooks(
        app: &App,
        include_layout_tree: bool,
        hooks: Option<&AiAppHooks>,
    ) -> AiRuntimeSnapshot {
        Self::snapshot_versioned(app, include_layout_tree, hooks, AI_SCHEMA_PROTOCOL_VERSION)
    }

    /// Captures a complete AI Schema v2 (`euv.ai.schema.v2`) runtime & layout snapshot.
    ///
    /// # Arguments
    ///
    /// - `&App` - Active application runtime.
    /// - `bool` - Whether to include the full recursive `layout_tree`.
    /// - `Option<&AiAppHooks>` - Optional host route/theme/game telemetry hooks.
    ///
    /// # Returns
    ///
    /// - `AiRuntimeSnapshot` - Complete v2 runtime snapshot with `frame_id`, `scene_revision`, `layout_state`, and redaction counts.
    #[must_use]
    pub fn snapshot_v2(
        app: &App,
        include_layout_tree: bool,
        hooks: Option<&AiAppHooks>,
    ) -> AiRuntimeSnapshot {
        Self::snapshot_versioned(
            app,
            include_layout_tree,
            hooks,
            AI_SCHEMA_V2_PROTOCOL_VERSION,
        )
    }

    /// Captures a deterministic same-frame AI Schema v2 diagnostic bundle pairing `(frame_id, scene_revision, snapshot, optional_rgba_fnv1a)`.
    ///
    /// # Arguments
    ///
    /// - `&App` - Active application runtime.
    /// - `bool` - Whether to include the full recursive `layout_tree`.
    /// - `bool` - Whether to compute an FNV-1a digest of the matching frame's CPU RGBA8 framebuffer.
    /// - `Option<&AiAppHooks>` - Optional host route/theme/game telemetry hooks.
    ///
    /// # Returns
    ///
    /// - `AiSameFrameCapture` - Synchronized same-frame snapshot and optional pixel digest.
    #[must_use]
    pub fn capture_same_frame(
        app: &App,
        include_layout_tree: bool,
        include_rgba_hash: bool,
        hooks: Option<&AiAppHooks>,
    ) -> AiSameFrameCapture {
        let last_rendered_rev: u64 = app.get_last_rendered_revision();
        let snapshot: AiRuntimeSnapshot = Self::snapshot_v2(app, include_layout_tree, hooks);
        let frame_id: u64 = snapshot.get_frame_id();
        let scene_revision: u64 = snapshot.get_scene_revision();
        let (rgba_fnv1a, capture_source): (Option<u64>, &'static str) = if include_rgba_hash {
            match app.get_renderer().framebuffer() {
                Some(fb) => {
                    let source: &'static str = match app.get_renderer().get_active_mode() {
                        RenderMode::Software => CAPTURE_SOURCE_SOFTWARE,
                        RenderMode::Hybrid => CAPTURE_SOURCE_HYBRID,
                        RenderMode::Hardware | RenderMode::Auto => CAPTURE_SOURCE_HARDWARE,
                    };
                    (Some(fb.fnv1a_hash()), source)
                }
                None => (None, CAPTURE_SOURCE_NONE),
            }
        } else {
            (None, CAPTURE_SOURCE_NONE)
        };
        let synced: bool = if include_rgba_hash {
            rgba_fnv1a.is_some() && last_rendered_rev == scene_revision && frame_id > 0
        } else {
            true
        };
        AiSameFrameCapture {
            frame_id,
            scene_revision,
            snapshot,
            rgba_fnv1a,
            capture_source: capture_source.to_string(),
            synced,
        }
    }

    /// Executes a live hit test at logical coordinates `(x, y)` and returns the leaf + ancestor chain with sensitive content redacted.
    ///
    /// # Arguments
    ///
    /// - `&App` - Active application runtime.
    /// - `f32` - Logical X coordinate.
    /// - `f32` - Logical Y coordinate.
    ///
    /// # Returns
    ///
    /// - `AiHitTestResult` - Hit node metadata and ancestor chain.
    #[must_use]
    pub fn hit_test_at(app: &App, x: f32, y: f32) -> AiHitTestResult {
        app.get_scene().with_write(
            |tree: &mut SceneTree| {
                let hit: Option<NodeId> = hit_test(tree, Point::new(x, y));
                let mut ancestors: Vec<u32> = Vec::new();
                let mut cursor: Option<NodeId> = hit;
                while let Some(nid) = cursor {
                    ancestors.push(nid.get_index());
                    cursor = tree.get_node(nid).and_then(SceneNode::parent_id);
                }
                let raw_key: Option<String> = hit
                    .and_then(|nid: NodeId| tree.get_node_mut(nid))
                    .and_then(|node: &mut SceneNode| node.get_mut_key().clone());
                let (role, text): (Option<String>, Option<String>) = match hit
                    .and_then(|nid: NodeId| tree.get_node(nid))
                {
                    Some(node) => {
                        let a11y_opt: Option<&A11yProps> = node.a11y_props_ref();
                        let a11y_label: Option<&str> = a11y_opt
                            .map(|a: &A11yProps| a.get_label().as_str())
                            .filter(|s: &&str| !s.is_empty());
                        let (_, test_id): (Option<&str>, Option<&str>) =
                            Self::resolve_node_identity_refs(raw_key.as_deref(), a11y_opt);
                        let sensitive: bool =
                            Self::is_sensitive_node(node, raw_key.as_deref(), test_id, a11y_label);
                        let txt: Option<String> = match node.get_kind() {
                            NodeKind::Text(t) => {
                                if sensitive && !t.is_empty() {
                                    Some(AI_SCHEMA_REDACTED_VALUE.to_string())
                                } else {
                                    Some(t.clone())
                                }
                            }
                            NodeKind::Input(i) => {
                                if sensitive && !i.get_value().is_empty() {
                                    Some(AI_SCHEMA_REDACTED_VALUE.to_string())
                                } else {
                                    Some(i.get_value().clone())
                                }
                            }
                            _ => None,
                        };
                        (Some(Self::infer_role_label(node).to_string()), txt)
                    }
                    None => (None, None),
                };
                AiHitTestResult::new(hit.map(|n: NodeId| n.get_index()), role, text, ancestors)
            },
            AiHitTestResult::new(None, None, None, Vec::new()),
        )
    }

    /// Dispatches a typed `AiSchemaRequest` against `app` without custom host hooks.
    ///
    /// # Arguments
    ///
    /// - `&mut App` - Mutable application runtime.
    /// - `AiSchemaRequest` - AI Schema v1/v2 command.
    ///
    /// # Returns
    ///
    /// - `AiSchemaResponse` - Structured protocol response.
    pub fn handle(app: &mut App, request: AiSchemaRequest) -> AiSchemaResponse {
        Self::handle_with_hooks(app, request, None)
    }

    /// Dispatches a typed `AiSchemaRequest` against `app` with optional host `AiAppHooks`.
    ///
    /// # Arguments
    ///
    /// - `&mut App` - Mutable application runtime.
    /// - `AiSchemaRequest` - AI Schema v1/v2 command.
    /// - `Option<&AiAppHooks>` - Optional host route/theme/game control hooks.
    ///
    /// # Returns
    ///
    /// - `AiSchemaResponse` - Structured protocol response.
    pub fn handle_with_hooks(
        app: &mut App,
        request: AiSchemaRequest,
        hooks: Option<&AiAppHooks>,
    ) -> AiSchemaResponse {
        match request {
            AiSchemaRequest::DescribeSchema => {
                let spec: Value = from_str(AI_SCHEMA_JSON_SPEC).unwrap_or(Value::Null);
                AiSchemaResponse::success(spec)
            }
            AiSchemaRequest::Snapshot {
                include_layout_tree,
            } => {
                let snap: AiRuntimeSnapshot =
                    Self::snapshot_with_hooks(app, include_layout_tree, hooks);
                match to_value(snap) {
                    Ok(v) => AiSchemaResponse::success(v),
                    Err(err) => AiSchemaResponse::failure(err.to_string()),
                }
            }
            AiSchemaRequest::SnapshotV2 {
                include_layout_tree,
                include_rgba_hash,
            } => {
                let capture: AiSameFrameCapture =
                    Self::capture_same_frame(app, include_layout_tree, include_rgba_hash, hooks);
                match to_value(capture) {
                    Ok(v) => AiSchemaResponse::success_v2(v),
                    Err(err) => AiSchemaResponse::failure(err.to_string()),
                }
            }
            AiSchemaRequest::AccessibilityTree => {
                let nodes: Vec<AiA11yNode> = Self::accessibility_tree(app);
                match to_value(nodes) {
                    Ok(v) => AiSchemaResponse::success(v),
                    Err(err) => AiSchemaResponse::failure(err.to_string()),
                }
            }
            AiSchemaRequest::QueryNodes { selector } => {
                let matches: Vec<AiLayoutNode> = Self::query_nodes_direct(app, &selector);
                match to_value(matches) {
                    Ok(v) => AiSchemaResponse::success(v),
                    Err(err) => AiSchemaResponse::failure(err.to_string()),
                }
            }
            AiSchemaRequest::HitTest { x, y } => {
                let res: AiHitTestResult = Self::hit_test_at(app, x, y);
                match to_value(res) {
                    Ok(v) => AiSchemaResponse::success(v),
                    Err(err) => AiSchemaResponse::failure(err.to_string()),
                }
            }
            AiSchemaRequest::ExecuteAction { action } => {
                let mut custom_result: Option<Value> = None;
                let exec_res: Result<(), AutomationError> = match action {
                    AiAction::Click { selector } => UiAutomator::new(app)
                        .click(&selector)
                        .map(|_: FrameStats| ()),
                    AiAction::ClickAt { x, y } => {
                        UiAutomator::new(app).click_at(x, y).map(|_: FrameStats| ())
                    }
                    AiAction::DoubleClick { selector } => UiAutomator::new(app)
                        .double_click(&selector)
                        .map(|_: FrameStats| ()),
                    AiAction::Hover { selector } => UiAutomator::new(app)
                        .hover(&selector)
                        .map(|_: FrameStats| ()),
                    AiAction::Focus { selector } => UiAutomator::new(app)
                        .focus(&selector)
                        .map(|_: FrameStats| ()),
                    AiAction::TypeText { selector, text } => UiAutomator::new(app)
                        .type_text(&selector, &text)
                        .map(|_: FrameStats| ()),
                    AiAction::ClearInput { selector } => UiAutomator::new(app)
                        .clear_input(&selector)
                        .map(|_: FrameStats| ()),
                    AiAction::Drag { from, to } => UiAutomator::new(app)
                        .drag(&from, &to)
                        .map(|_: FrameStats| ()),
                    AiAction::DragPoints {
                        from_x,
                        from_y,
                        to_x,
                        to_y,
                    } => UiAutomator::new(app)
                        .drag_points(from_x, from_y, to_x, to_y)
                        .map(|_: FrameStats| ()),
                    AiAction::Gesture {
                        gesture,
                        x,
                        y,
                        dx,
                        dy,
                        scale,
                    } => UiAutomator::new(app)
                        .gesture(gesture, x, y, dx, dy, scale)
                        .map(|_: FrameStats| ()),
                    AiAction::Scroll { selector, dx, dy } => UiAutomator::new(app)
                        .scroll(&selector, dx, dy)
                        .map(|_: FrameStats| ()),
                    AiAction::PressKey { key } => UiAutomator::new(app)
                        .press_key_name(&key)
                        .map(|_: FrameStats| ()),
                    AiAction::Resize {
                        width,
                        height,
                        scale,
                    } => UiAutomator::new(app)
                        .resize(width, height, scale)
                        .map(|_: FrameStats| ()),
                    AiAction::StepFrames { count } => UiAutomator::new(app)
                        .step_frames(count)
                        .map(|_: FrameStats| ()),
                    AiAction::StepGame { dt_secs, frames } => UiAutomator::new(app)
                        .step_game(dt_secs, frames)
                        .map(|_: FrameStats| ()),
                    AiAction::SetFpsLimit { fps } => UiAutomator::new(app)
                        .set_fps_limit(fps)
                        .map(|_: FrameStats| ()),
                    AiAction::Navigate { route } => match hooks {
                        Some(h) => h
                            .invoke_navigate(&route)
                            .map_err(AutomationError::RuntimeError)
                            .and_then(|()| UiAutomator::new(app).step().map(|_: FrameStats| ())),
                        None => Err(AutomationError::RuntimeError(
                            ERR_NO_NAVIGATE_HOOK.to_string(),
                        )),
                    },
                    AiAction::SetTheme { scheme, mode } => match hooks {
                        Some(h) => h
                            .invoke_set_theme(scheme.as_deref(), mode.as_deref())
                            .map_err(AutomationError::RuntimeError)
                            .and_then(|()| UiAutomator::new(app).step().map(|_: FrameStats| ())),
                        None => {
                            if let Some(m) = mode.as_deref() {
                                let wid: WindowId = app.get_window_state().get_id();
                                let parsed: ThemeMode = if m.eq_ignore_ascii_case(THEME_MODE_DARK) {
                                    ThemeMode::Dark
                                } else {
                                    ThemeMode::Light
                                };
                                app.dispatch_window_event(WindowEvent::ThemeChanged(wid, parsed));
                                UiAutomator::new(app).step().map(|_: FrameStats| ())
                            } else {
                                Err(AutomationError::RuntimeError(ERR_NO_THEME_HOOK.to_string()))
                            }
                        }
                    },
                    AiAction::GameControl { command, payload } => match hooks {
                        Some(h) => h
                            .invoke_game_control(&command, &payload)
                            .map_err(AutomationError::RuntimeError)
                            .and_then(|val: Value| {
                                custom_result = Some(val);
                                UiAutomator::new(app)
                                    .step_game(1.0 / 60.0, 1)
                                    .map(|_: FrameStats| ())
                            }),
                        None => Err(AutomationError::RuntimeError(ERR_NO_GAME_HOOK.to_string())),
                    },
                };
                match exec_res {
                    Ok(()) => {
                        let snap: AiRuntimeSnapshot = Self::snapshot_with_hooks(app, false, hooks);
                        match to_value(snap) {
                            Ok(mut v) => {
                                if let Some(extra) = custom_result
                                    && let Some(obj) = v.as_object_mut()
                                {
                                    let _: Option<Value> =
                                        obj.insert(CONTROL_RESULT_FIELD.to_string(), extra);
                                }
                                AiSchemaResponse::success(v)
                            }
                            Err(err) => AiSchemaResponse::failure(err.to_string()),
                        }
                    }
                    Err(err) => AiSchemaResponse::failure(err.to_string()),
                }
            }
            AiSchemaRequest::RunTestScript { script } => {
                let mut auto: UiAutomator<'_> = UiAutomator::new(app);
                let report: UiTestReport = auto.run_script(&script);
                match to_value(report) {
                    Ok(v) => AiSchemaResponse::success(v),
                    Err(err) => AiSchemaResponse::failure(err.to_string()),
                }
            }
            AiSchemaRequest::RunE2eFlow {
                flow,
                run_id,
                app_build,
            } => {
                let eff_run_id: &str = if run_id.trim().is_empty() {
                    DEFAULT_E2E_RUN_ID
                } else {
                    run_id.as_str()
                };
                let eff_build: &str = if app_build.trim().is_empty() {
                    DEFAULT_E2E_APP_BUILD
                } else {
                    app_build.as_str()
                };
                let replayer: UiE2eReplayer = UiE2eReplayer::for_build(eff_build);
                let result: UiE2eResultV1 = replayer.replay_once(app, &flow, eff_run_id);
                match to_value(result) {
                    Ok(v) => AiSchemaResponse::success_v2(v),
                    Err(err) => AiSchemaResponse::failure(err.to_string()),
                }
            }
        }
    }

    /// Parses a raw JSON `AiSchemaRequest`, executes it against `app`, and returns JSON output.
    ///
    /// # Arguments
    ///
    /// - `&mut App` - Mutable application runtime.
    /// - `&str` - Raw JSON request string.
    ///
    /// # Returns
    ///
    /// - `String` - Serialized `AiSchemaResponse` JSON string.
    pub fn handle_json(app: &mut App, request_json: &str) -> String {
        Self::handle_json_with_hooks(app, request_json, None)
    }

    /// Parses a raw JSON `AiSchemaRequest`, executes it against `app` with optional `AiAppHooks`, and returns JSON output.
    ///
    /// # Arguments
    ///
    /// - `&mut App` - Mutable application runtime.
    /// - `&str` - Raw JSON request string.
    /// - `Option<&AiAppHooks>` - Optional host telemetry and control hooks.
    ///
    /// # Returns
    ///
    /// - `String` - Serialized `AiSchemaResponse` JSON string.
    pub fn handle_json_with_hooks(
        app: &mut App,
        request_json: &str,
        hooks: Option<&AiAppHooks>,
    ) -> String {
        let resp: AiSchemaResponse = match from_str::<AiSchemaRequest>(request_json) {
            Ok(req) => Self::handle_with_hooks(app, req, hooks),
            Err(err) => AiSchemaResponse::failure(format!("{ERR_INVALID_REQUEST_JSON}: {err}")),
        };
        to_string(&resp).unwrap_or_default()
    }

    /// Parses a JSON string into a `serde_json::Value` (falling back to `Value::Null` on error).
    ///
    /// # Arguments
    ///
    /// - `&str` - JSON payload string.
    ///
    /// # Returns
    ///
    /// - `Value` - Parsed JSON value.
    #[must_use]
    pub fn parse_json_value(raw: &str) -> Value {
        from_str(raw).unwrap_or(Value::Null)
    }

    /// Serializes a full pretty-printed `AiRuntimeSnapshot` JSON string for AI inspection.
    ///
    /// # Arguments
    ///
    /// - `&App` - Active application runtime.
    ///
    /// # Returns
    ///
    /// - `String` - Pretty-printed JSON snapshot string.
    #[must_use]
    pub fn snapshot_pretty_json(app: &App) -> String {
        Self::snapshot_pretty_json_with_hooks(app, None)
    }

    /// Serializes a full pretty-printed `AiRuntimeSnapshot` JSON string enriched with optional `AiAppHooks`.
    ///
    /// # Arguments
    ///
    /// - `&App` - Active application runtime.
    /// - `Option<&AiAppHooks>` - Optional host telemetry hooks.
    ///
    /// # Returns
    ///
    /// - `String` - Pretty-printed JSON snapshot string.
    #[must_use]
    pub fn snapshot_pretty_json_with_hooks(app: &App, hooks: Option<&AiAppHooks>) -> String {
        let snap: AiRuntimeSnapshot = Self::snapshot_with_hooks(app, true, hooks);
        to_string_pretty(&snap).unwrap_or_default()
    }
}
