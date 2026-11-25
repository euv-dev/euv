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

/// Optional layout tree accessor for `AiRuntimeSnapshot`.
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

    /// Infers a semantic AI role label from a `SceneNode`.
    ///
    /// # Arguments
    ///
    /// - `&SceneNode` - Retained scene node.
    ///
    /// # Returns
    ///
    /// - `&'static str` - Role string (`"Button"`, `"TextField"`, `"Text"`, etc.).
    fn infer_role(node: &SceneNode) -> &'static str {
        let clickable: bool = node
            .get_handlers()
            .handler_for(&UiEventKind::Click(Point::default()))
            .is_some()
            || node.get_style().get_paint().get_cursor() == CursorIcon::Pointer;
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
            text_color_hex: Self::color_to_hex(text_style.get_color()),
            font_size: text_style.get_font_size(),
            cursor: format!("{:?}", paint.get_cursor()),
        };
        Some(AiLayoutNode {
            node_index: node_id.get_index(),
            generation: node_id.get_generation(),
            kind: kind_str.to_string(),
            role: Self::infer_role(node).to_string(),
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

    /// Scans `node` recursively to detect horizontal viewport overflow and sibling text overlaps.
    ///
    /// # Arguments
    ///
    /// - `&AiLayoutNode` - Subtree root to audit.
    /// - `f32` - Active logical viewport width.
    /// - `&mut Vec<AiLayoutAnomaly>` - Output anomaly list.
    fn detect_anomalies(node: &AiLayoutNode, viewport_w: f32, out: &mut Vec<AiLayoutAnomaly>) {
        let b: AiRect = node.get_screen_bounds();
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
            Self::detect_anomalies(child, viewport_w, out);
        }
    }

    /// Captures a complete `AiRuntimeSnapshot` from a live `App`.
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
        let ws: WindowState = app.get_window_state();
        let stats: FrameStats = app.get_renderer().get_last_stats();
        let win_info: AiWindowInfo = AiWindowInfo {
            window_id: ws.get_id().get_value(),
            title: app.get_config().get_window().get_title().clone(),
            logical_width: ws.get_logical_width(),
            logical_height: ws.get_logical_height(),
            physical_width: ws.get_physical_width(),
            physical_height: ws.get_physical_height(),
            scale_factor: ws.get_scale_factor(),
            breakpoint: format!("{:?}", ws.breakpoint()),
            theme_mode: format!(
                "{:?}",
                app.get_build_context().get_theme_mode().get_untracked()
            ),
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
        let perf_info: AiPerfInfo = AiPerfInfo {
            frame_index: stats.get_frame_index(),
            active_mode: format!("{:?}", stats.get_active_mode()),
            active_backend: format!("{:?}", stats.get_active_backend()),
            layout_nodes: stats.get_layout_nodes(),
            layout_skipped: stats.get_layout_skipped(),
            draw_commands: stats.get_draw_commands(),
            draw_batches: stats.get_draw_batches(),
            pixels_shaded: stats.get_pixels_shaded(),
            frame_hash: stats.get_frame_hash(),
            live_node_count: live_count,
            scene_revision: rev,
        };
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
        let a11y: A11yTree = app.a11y_tree();
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
            a11y.get_nodes().len(),
        );
        let mut anomalies: Vec<AiLayoutAnomaly> = Vec::new();
        if let Some(root_ref) = root_tree.as_ref() {
            Self::detect_anomalies(root_ref, ws.get_logical_width(), &mut anomalies);
        }
        AiRuntimeSnapshot::new(
            AI_SCHEMA_PROTOCOL_VERSION.to_string(),
            win_info,
            perf_info,
            interaction,
            runtime_meta,
            if include_layout_tree { root_tree } else { None },
            anomalies,
        )
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
                            (Some(Self::infer_role(node).to_string()), txt)
                        }
                        None => (None, None),
                    };
                AiHitTestResult::new(hit.map(|n: NodeId| n.get_index()), role, text, ancestors)
            },
            AiHitTestResult::new(None, None, None, Vec::new()),
        )
    }

    /// Dispatches a typed `AiSchemaRequest` against `app` and produces an `AiSchemaResponse`.
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
        match request {
            AiSchemaRequest::DescribeSchema => {
                let spec: Value = from_str(AI_SCHEMA_JSON_SPEC).unwrap_or(Value::Null);
                AiSchemaResponse::success(spec)
            }
            AiSchemaRequest::Snapshot {
                include_layout_tree,
            } => {
                let snap: AiRuntimeSnapshot = Self::snapshot(app, include_layout_tree);
                match to_value(snap) {
                    Ok(v) => AiSchemaResponse::success(v),
                    Err(err) => AiSchemaResponse::failure(err.to_string()),
                }
            }
            AiSchemaRequest::QueryNodes { selector } => {
                let snap: AiRuntimeSnapshot = Self::snapshot(app, true);
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
                let mut auto: UiAutomator<'_> = UiAutomator::new(app);
                let exec_res: Result<FrameStats, AutomationError> = match action {
                    AiAction::Click { selector } => auto.click(&selector),
                    AiAction::Hover { selector } => auto.hover(&selector),
                    AiAction::TypeText { selector, text } => auto.type_text(&selector, &text),
                    AiAction::Scroll { selector, dx, dy } => auto.scroll(&selector, dx, dy),
                    AiAction::PressKey { key } => auto.press_key_name(&key),
                    AiAction::Resize {
                        width,
                        height,
                        scale,
                    } => auto.resize(width, height, scale),
                };
                match exec_res {
                    Ok(_) => {
                        let snap: AiRuntimeSnapshot = Self::snapshot(auto.app_ref(), false);
                        match to_value(snap) {
                            Ok(v) => AiSchemaResponse::success(v),
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
        let resp: AiSchemaResponse = match from_str::<AiSchemaRequest>(request_json) {
            Ok(req) => Self::handle(app, req),
            Err(err) => AiSchemaResponse::failure(format!("{ERR_INVALID_REQUEST_JSON}: {err}")),
        };
        to_string(&resp).unwrap_or_default()
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
        let snap: AiRuntimeSnapshot = Self::snapshot(app, true);
        to_string_pretty(&snap).unwrap_or_default()
    }
}
