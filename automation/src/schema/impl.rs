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

/// Optional text/value/placeholder accessors and flattening helpers for `AiLayoutNode`.
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
}

/// Runtime snapshot capture, anomaly detection, and JSON-RPC request dispatch for `AiSchemaProtocol`.
impl AiSchemaProtocol {
    /// Formats a `Color` as an `#RRGGBBAA` hex string.
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
        format!(
            "#{:02x}{:02x}{:02x}{:02x}",
            color.get_r(),
            color.get_g(),
            color.get_b(),
            color.get_a()
        )
    }

    /// Infers a semantic AI role label from explicit `A11yProps` or fallback `SceneNode` heuristics.
    ///
    /// # Arguments
    ///
    /// - `&SceneNode` - Retained scene node.
    ///
    /// # Returns
    ///
    /// - `String` - Role string (`"Button"`, `"Switch"`, `"Checkbox"`, `"TextInput"`, etc.).
    fn infer_role(node: &SceneNode) -> String {
        if let Some(a11y) = node.a11y_props() {
            return format!("{:?}", a11y.get_role());
        }
        let clickable: bool = node
            .get_handlers()
            .handler_for(&UiEventKind::Click(Point::default()))
            .is_some()
            || node.get_style().get_paint().get_cursor() == CursorIcon::Pointer;
        let fallback: &'static str = match node.get_kind() {
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
        };
        fallback.to_string()
    }

    /// Exports the full compiled semantic accessibility tree from `app`.
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
        let a11y: A11yTree = app.a11y_tree();
        a11y.get_nodes()
            .iter()
            .map(|n: &A11yNode| {
                let props: &A11yProps = n.get_props();
                let child_indices: Vec<u32> = n
                    .get_children()
                    .iter()
                    .map(|cid: &NodeId| cid.get_index())
                    .collect();
                AiA11yNode {
                    node_index: n.get_id().get_index(),
                    role: format!("{:?}", props.get_role()),
                    label: props.get_label().clone(),
                    hint: props.get_hint().clone(),
                    value: props.get_value().clone(),
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

    /// Recursively compiles `node_id` into an `AiLayoutNode` preserving scroll-adjusted screen coordinates.
    ///
    /// # Arguments
    ///
    /// - `&SceneTree` - Retained scene tree.
    /// - `NodeId` - Current node handle.
    /// - `Point` - Accumulated ancestor scroll offset.
    /// - `bool` - Whether all ancestors are visible.
    ///
    /// # Returns
    ///
    /// - `Option<AiLayoutNode>` - Serialized layout node if alive.
    fn build_layout_node(
        tree: &SceneTree,
        node_id: NodeId,
        scroll_offset: Point,
        ancestor_visible: bool,
    ) -> Option<AiLayoutNode> {
        let node: &SceneNode = tree.get_node(node_id)?;
        let style: Style = node.get_style();
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
        let raw_b: Rect = node.get_layout_box().get_bounds();
        let screen_b: Rect = Rect::new(
            raw_b.get_x() - eff_scroll.get_x(),
            raw_b.get_y() - eff_scroll.get_y(),
            raw_b.get_width(),
            raw_b.get_height(),
        );
        let lbox: LayoutBox = node.get_layout_box();
        let max_scroll_x: f32 =
            (lbox.get_content_size().get_width() - lbox.get_content_bounds().get_width()).max(0.0);
        let max_scroll_y: f32 = (lbox.get_content_size().get_height()
            - lbox.get_content_bounds().get_height())
        .max(0.0);
        let self_visible: bool =
            ancestor_visible && layout.get_display() != Display::None && paint.get_opacity() > 0.0;
        let visible: bool =
            self_visible && screen_b.get_width() > 0.0 && screen_b.get_height() > 0.0;
        let interactive: bool = paint.get_cursor() == CursorIcon::Pointer
            || node.get_interaction().get_focusable()
            || matches!(node.get_kind(), NodeKind::Input(_))
            || node
                .get_handlers()
                .handler_for(&UiEventKind::Click(Point::default()))
                .is_some();
        let a11y_opt: Option<A11yProps> = node.a11y_props();
        let a11y_label: Option<String> = a11y_opt
            .as_ref()
            .map(|a: &A11yProps| a.get_label().clone())
            .filter(|s: &String| !s.is_empty());
        let a11y_hint: Option<String> = a11y_opt
            .as_ref()
            .map(|a: &A11yProps| a.get_hint().clone())
            .filter(|s: &String| !s.is_empty());
        let input_disabled: bool = match node.get_kind() {
            NodeKind::Input(inp) => inp.get_disabled(),
            _ => false,
        };
        let disabled: bool = input_disabled
            || a11y_opt
                .as_ref()
                .map(A11yProps::get_disabled)
                .unwrap_or(false);
        let (kind_str, text, placeholder, value): (
            &'static str,
            Option<String>,
            Option<String>,
            Option<String>,
        ) = match node.get_kind() {
            NodeKind::View => (KIND_VIEW, None, None, None),
            NodeKind::Text(t) => (KIND_TEXT, Some(t.clone()), None, None),
            NodeKind::Input(inp) => (
                KIND_INPUT,
                None,
                Some(inp.get_placeholder().clone()),
                Some(inp.get_value().clone()),
            ),
            NodeKind::Image(_) => (KIND_IMAGE, None, None, None),
            NodeKind::ScrollView => (KIND_SCROLL_VIEW, None, None, None),
            NodeKind::Canvas => (KIND_CANVAS, None, None, None),
            NodeKind::Portal => (KIND_PORTAL, None, None, None),
            NodeKind::Fragment => (KIND_FRAGMENT, None, None, None),
        };
        let child_scroll: Point = Point::new(
            eff_scroll.get_x() + lbox.get_scroll_offset().get_x(),
            eff_scroll.get_y() + lbox.get_scroll_offset().get_y(),
        );
        let mut children: Vec<AiLayoutNode> = Vec::with_capacity(node.get_children().len());
        for child_id in node.get_children().iter().copied() {
            if let Some(child_node) =
                Self::build_layout_node(tree, child_id, child_scroll, self_visible)
            {
                children.push(child_node);
            }
        }
        let style_summary: AiStyleSummary = AiStyleSummary {
            display: format!("{:?}", layout.get_display()),
            position: format!("{:?}", layout.get_position()),
            overflow: format!("{:?}", layout.get_overflow()),
            z_index: layout.get_z_index(),
            opacity: paint.get_opacity(),
            background_hex: paint.background_color().map(Self::color_to_hex),
            border_width: paint.get_border().get_width(),
            border_hex: Self::color_to_hex(paint.get_border().get_color()),
            radius: paint.get_radius().get_top_left(),
            text_color_hex: Self::color_to_hex(text_style.get_color()),
            font_size: text_style.get_font_size(),
            font_weight: format!("{:?}", text_style.get_weight()),
            cursor: format!("{:?}", paint.get_cursor()),
        };
        Some(AiLayoutNode {
            node_index: node_id.get_index(),
            generation: node_id.get_generation(),
            kind: kind_str.to_string(),
            role: Self::infer_role(node),
            a11y_label,
            a11y_hint,
            disabled,
            text,
            placeholder,
            value,
            screen_bounds: AiRect::from_rect(screen_b),
            layout_bounds: AiRect::from_rect(raw_b),
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

    /// Scans `node` recursively to detect viewport overflow, negative bounds, zero-size visible content, and sibling text overlaps.
    ///
    /// # Arguments
    ///
    /// - `&AiLayoutNode` - Subtree root to audit.
    /// - `f32` - Active logical viewport width.
    /// - `bool` - Whether all ancestors are displayed with non-zero opacity.
    /// - `&mut Vec<AiLayoutAnomaly>` - Output anomaly list.
    fn detect_anomalies(
        node: &AiLayoutNode,
        viewport_w: f32,
        ancestor_displayed: bool,
        out: &mut Vec<AiLayoutAnomaly>,
    ) {
        let b: AiRect = node.get_screen_bounds();
        let style: &AiStyleSummary = node.get_style();
        let displayed: bool =
            ancestor_displayed && style.get_display() != "None" && style.get_opacity() > 0.0;
        if node.get_visible() && b.right() > viewport_w + 2.0 {
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
        if node.get_visible() && b.get_x() < -2.0 {
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
        for child in node.get_children().iter() {
            Self::detect_anomalies(child, viewport_w, displayed, out);
        }
    }

    /// Captures a complete `AiRuntimeSnapshot` from a live `App` without custom host hooks.
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

    /// Captures a complete `AiRuntimeSnapshot` enriched with optional host `AiAppHooks` telemetry.
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
            breakpoint: format!("{:?}", ws.breakpoint()),
            theme_mode: format!("{:?}", app.get_context().get_theme_mode().get_untracked()),
            active_route,
            active_scheme,
            focused: ws.get_focused(),
        };
        let (live_count, rev, focus_mgr, root_tree): (
            usize,
            u64,
            FocusManager,
            Option<AiLayoutNode>,
        ) = app.get_scene().with_read(
            |tree: &SceneTree| {
                let root_node: Option<AiLayoutNode> =
                    Self::build_layout_node(tree, tree.get_root(), Point::new(0.0, 0.0), true);
                (
                    tree.live_node_count(),
                    tree.get_revision(),
                    tree.get_focus(),
                    root_node,
                )
            },
            (0, 0, FocusManager::default(), None),
        );
        let measured_fps: f32 = app.get_context().get_live_fps().get_untracked();
        let frame_time_ms: f32 = if measured_fps > 0.1 {
            (1000.0 / measured_fps * 100.0).round() * 0.01
        } else {
            (app.get_frame_clock().get_delta_secs() * 1000.0 * 100.0).round() * 0.01
        };
        let perf_info: AiPerfInfo = AiPerfInfo {
            frame_index: stats.get_frame_index(),
            active_mode: format!("{:?}", stats.get_active_mode()),
            active_backend: format!("{:?}", stats.get_active_backend()),
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
            format!("{:?}", app.get_last_cursor()),
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
            format!("{:?}", app.platform_kind()),
            caps.get_hardware_gpu(),
            caps.get_touch(),
            protocols,
            a11y_nodes.len(),
        );
        let mut anomalies: Vec<AiLayoutAnomaly> = Vec::new();
        if let Some(root_ref) = root_tree.as_ref() {
            Self::detect_anomalies(root_ref, ws.get_logical_width(), true, &mut anomalies);
        }
        AiRuntimeSnapshot {
            protocol_version: AI_SCHEMA_PROTOCOL_VERSION.to_string(),
            window: win_info,
            performance: perf_info,
            reactive: reactive_info,
            interaction,
            runtime: runtime_meta,
            game_stats,
            accessibility_tree: a11y_nodes,
            layout_tree: if include_layout_tree { root_tree } else { None },
            anomalies,
        }
    }

    /// Executes a live hit test at logical coordinates `(x, y)` and returns the leaf + ancestor chain.
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
        app.get_scene().with_read(
            |tree: &SceneTree| {
                let hit: Option<NodeId> = hit_test(tree, Point::new(x, y));
                let mut ancestors: Vec<u32> = Vec::new();
                let mut cursor: Option<NodeId> = hit;
                while let Some(nid) = cursor {
                    ancestors.push(nid.get_index());
                    cursor = tree.get_node(nid).and_then(SceneNode::parent_id);
                }
                let (role, text): (Option<String>, Option<String>) =
                    match hit.and_then(|nid: NodeId| tree.get_node(nid)) {
                        Some(node) => {
                            let txt: Option<String> = match node.get_kind() {
                                NodeKind::Text(t) => Some(t.clone()),
                                NodeKind::Input(i) => Some(i.get_value().clone()),
                                _ => None,
                            };
                            (Some(Self::infer_role(node)), txt)
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
    /// - `AiSchemaRequest` - AI Schema v1 command.
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
    /// - `AiSchemaRequest` - AI Schema v1 command.
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
            AiSchemaRequest::AccessibilityTree => {
                let nodes: Vec<AiA11yNode> = Self::accessibility_tree(app);
                match to_value(nodes) {
                    Ok(v) => AiSchemaResponse::success(v),
                    Err(err) => AiSchemaResponse::failure(err.to_string()),
                }
            }
            AiSchemaRequest::QueryNodes { selector } => {
                let snap: AiRuntimeSnapshot = Self::snapshot_with_hooks(app, true, hooks);
                let matches: Vec<AiLayoutNode> = snap
                    .root_layout_node()
                    .map(|r: &AiLayoutNode| r.find_matches(&selector))
                    .unwrap_or_default();
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
                                let parsed: ThemeMode = if m.eq_ignore_ascii_case("Dark") {
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
                                        obj.insert("control_result".to_string(), extra);
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
