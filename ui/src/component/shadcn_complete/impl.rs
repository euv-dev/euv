use super::*;

/// Declarative `#[component]` implementation for `EuvChart`.
#[component]
/// Inherent implementation of [`EuvChart`].
impl EuvChart {
    /// Renders `EuvChart` into the ambient framework-managed `Context`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component descriptor reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted root scene node handle.
    fn render(&self) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let kind: EuvChartKind = self.get_kind();
        let Self { title, data, .. } = self.clone();
        let plot_data: Vec<EuvChartPoint> = data;
        let bar_color: Color = p.get_primary();
        let grid_color: Color = p.get_border();
        let label_color: Color = p.get_muted_foreground();
        let radius_sm: f32 = g.get_radius_sm();
        view! {
            column {
                width: Length::Percent(100.0)
                padding: Edges::all(SPACE_LG)
                gap: SPACE_SM
                background: p.get_card()
                border: (g.get_border_width(), p.get_border())
                radius: g.get_radius_lg()
                overflow: Overflow::Hidden
                text {
                    font_size: 14.0
                    font_weight: FontWeight::SemiBold
                    color: p.get_card_foreground()
                    { title }
                }
                canvas {
                    width: Length::Percent(100.0)
                    height: Length::Px(150.0)
                    painter: move |bounds: Rect, dl: &mut DisplayList| {
                        if plot_data.is_empty() {
                            return;
                        }
                        let max_val: f32 = plot_data
                        .iter()
                        .map(EuvChartPoint::get_value)
                        .fold(1.0_f32, f32::max);
                        let pad_x: f32 = 12.0;
                        let pad_top: f32 = 10.0;
                        let pad_bottom: f32 = 24.0;
                        let plot_w: f32 = (bounds.get_width() - pad_x * 2.0).max(40.0);
                        let plot_h: f32 = (bounds.get_height() - pad_top - pad_bottom).max(30.0);
                        for step in 0..=3 {
                            let gy: f32 = bounds.get_y() + pad_top + plot_h * (step as f32) / 3.0;
                            dl.push(DrawCommand::Line {
                                from: Point::new(bounds.get_x() + pad_x, gy),
                                to: Point::new(bounds.get_x() + pad_x + plot_w, gy),
                                color: grid_color.multiply_alpha(0.6),
                                width: 1.0,
                            });
                        }
                        let count: usize = plot_data.len();
                        let slot_w: f32 = plot_w / (count as f32);
                        let mut prev_pt: Option<Point> = None;
                        for (i, pt) in plot_data.iter().enumerate() {
                            let frac: f32 = (pt.get_value() / max_val).clamp(0.0, 1.0);
                            let cx: f32 = bounds.get_x() + pad_x + slot_w * ((i as f32) + 0.5);
                            let cy: f32 = bounds.get_y() + pad_top + plot_h * (1.0 - frac);
                            let base_y: f32 = bounds.get_y() + pad_top + plot_h;
                            match kind {
                                EuvChartKind::Bar => {
                                    let bw: f32 = (slot_w * 0.56).clamp(6.0, 36.0);
                                    let bh: f32 = (base_y - cy).max(3.0);
                                    dl.push(DrawCommand::FillRoundedRect {
                                        rect: Rect::new(cx - bw * 0.5, cy, bw, bh),
                                        color: bar_color,
                                        radius: CornerRadius::new(radius_sm, radius_sm, 0.0, 0.0),
                                        border: Border::default(),
                                    });
                                }
                                EuvChartKind::Line | EuvChartKind::Area => {
                                    let cur: Point = Point::new(cx, cy);
                                    if let Some(prev) = prev_pt {
                                        if kind == EuvChartKind::Area {
                                            let area_col: Color = bar_color.multiply_alpha(0.22);
                                            dl.push(DrawCommand::Triangle {
                                                p0: prev,
                                                p1: cur,
                                                p2: Point::new(cur.get_x(), base_y),
                                                color: area_col,
                                            });
                                            dl.push(DrawCommand::Triangle {
                                                p0: prev,
                                                p1: Point::new(cur.get_x(), base_y),
                                                p2: Point::new(prev.get_x(), base_y),
                                                color: area_col,
                                            });
                                        }
                                        dl.push(DrawCommand::Line {
                                            from: prev,
                                            to: cur,
                                            color: bar_color,
                                            width: 2.5,
                                        });
                                    }
                                    dl.push(DrawCommand::Circle {
                                        center: cur,
                                        radius: 3.5,
                                        color: bar_color,
                                    });
                                    prev_pt = Some(cur);
                                }
                            }
                            let mut lbl_st: TextStyle = TextStyle::default();
                            lbl_st.set_font_size(10.0).set_color(label_color);
                            dl.push(DrawCommand::TextRun {
                                rect: Rect::new(
                                    cx - slot_w * 0.45,
                                    base_y + 6.0,
                                    slot_w * 0.9,
                                    14.0,
                                ),
                                text: pt.get_label().clone(),
                                style: lbl_st,
                            });
                        }
                    }
                }
            }
        }
    }
}

/// Fluent search query and placeholder builders for `EuvCombobox`.
impl EuvCombobox {
    /// Sets the search input placeholder text fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source combobox.
    /// - `S` - Search input placeholder string.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated combobox.
    #[must_use]
    pub fn placeholder<S>(mut self, placeholder: S) -> Self
    where
        S: Into<String>,
    {
        self.set_placeholder(placeholder.into());
        self
    }

    /// Binds an external reactive query signal fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source combobox.
    /// - `Signal<String>` - Controlled search query signal.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated combobox.
    #[must_use]
    pub fn query(mut self, query: Signal<String>) -> Self {
        self.set_query(Some(query));
        self
    }
}

/// Declarative `#[component]` implementation for `EuvCombobox`.
#[component]
/// Inherent implementation of [`EuvCombobox`].
impl EuvCombobox {
    /// Renders `EuvCombobox` into the ambient framework-managed `Context`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component descriptor reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted root scene node handle.
    fn render(&self) -> NodeId {
        let ctx: Context = current_context();
        let Self {
            placeholder,
            options,
            query: query_opt,
            selected,
            open,
            tokens,
        } = self;
        let tokens: ThemeTokens = *tokens;
        let p: ThemePalette = tokens.get_palette();
        let g: ThemeGeometry = tokens.get_geometry();
        let selected: Signal<usize> = *selected;
        let open: Signal<bool> = *open;
        let query: Signal<String> = query_opt.unwrap_or_else(|| Signal::new(String::new()));
        let resolved_ph: String = if placeholder.is_empty() {
            COMBOBOX_DEFAULT_PLACEHOLDER.to_string()
        } else {
            placeholder.clone()
        };
        let opts_shared: Rc<[String]> = Rc::from(options.as_slice());
        let opts_trigger: Rc<[String]> = Rc::clone(&opts_shared);
        let opts_filter: Rc<[String]> = opts_shared;
        let init_open: bool = open.get_untracked();
        let mut panel: NodeId = NodeId::default();
        let chevron_color: Color = p.get_muted_foreground();
        let on_toggle = move |_: &UiEvent| {
            let _: bool = open.update(|v: &mut bool| *v = !*v);
        };
        let trigger_text = move || -> String {
            let idx: usize = selected.get();
            opts_trigger
                .get(idx)
                .cloned()
                .unwrap_or_else(|| COMBOBOX_EMPTY_SELECTION.to_string())
        };
        let paint_chevron = move |bounds: Rect, dl: &mut DisplayList| {
            let cx: f32 = bounds.get_x() + bounds.get_width() * 0.5;
            let cy: f32 = bounds.get_y() + bounds.get_height() * 0.5;
            dl.push(DrawCommand::Line {
                from: Point::new(cx - 4.0, cy - 2.0),
                to: Point::new(cx, cy + 2.5),
                color: chevron_color,
                width: 1.6,
            });
            dl.push(DrawCommand::Line {
                from: Point::new(cx, cy + 2.5),
                to: Point::new(cx + 4.0, cy - 2.0),
                color: chevron_color,
                width: 1.6,
            });
        };
        let build_filtered = move || -> Vec<NodeId> {
            let q: String = query.get();
            let cur_sel: usize = selected.get();
            let mut nodes: Vec<NodeId> = Vec::with_capacity(opts_filter.len());
            for (idx, opt) in opts_filter.iter().enumerate() {
                if !q.is_empty() && !contains_ascii_ignore_case(opt.as_str(), q.as_str()) {
                    continue;
                }
                let is_sel: bool = idx == cur_sel;
                let row_label: String = if is_sel {
                    format!("* {opt}")
                } else {
                    opt.clone()
                };
                let on_pick = move |_: &UiEvent| {
                    let _: bool = selected.set_if_changed(idx);
                    let _: bool = open.set_if_changed(false);
                };
                nodes.push(view! {
                    row {
                        width: Length::Percent(100.0)
                        height: Length::Px(g.get_control_sm())
                        padding: Edges::symmetric(0.0, SPACE_SM)
                        align: AlignItems::Center
                        radius: g.get_radius_sm()
                        overflow: Overflow::Hidden
                        background: if is_sel {
                            p.get_accent()
                        } else {
                            COLOR_TRANSPARENT
                        }
                        cursor: CursorIcon::Pointer
                        on_click: on_pick
                        text {
                            width: Length::Percent(100.0)
                            font_size: 13.0
                            color: p.get_foreground()
                            text_overflow: TextOverflow::Ellipsis
                            { row_label }
                        }
                    }
                });
            }
            nodes
        };
        let wrap: NodeId = view! {
            column {
                width: Length::Percent(100.0)
                gap: SPACE_XS
                row {
                    width: Length::Percent(100.0)
                    height: Length::Px(g.get_control_md())
                    padding: Edges::symmetric(0.0, SPACE_MD)
                    justify: JustifyContent::SpaceBetween
                    align: AlignItems::Center
                    gap: SPACE_SM
                    background: p.get_background()
                    border: (g.get_border_width(), p.get_border())
                    radius: g.get_radius_md()
                    overflow: Overflow::Hidden
                    cursor: CursorIcon::Pointer
                    on_click: on_toggle
                    text {
                        flex_grow: 1.0
                        flex_shrink: 1.0
                        min_width: Length::Px(0.0)
                        font_size: 14.0
                        color: p.get_foreground()
                        text_overflow: TextOverflow::Ellipsis
                        { trigger_text }
                    }
                    canvas {
                        width: Length::Px(12.0)
                        height: Length::Px(12.0)
                        flex_shrink: 0.0
                        painter: paint_chevron
                    }
                }
                column {
                    node_ref: panel
                    display: if init_open { Display::Flex } else { Display::None }
                    width: Length::Percent(100.0)
                    padding: Edges::all(SPACE_SM)
                    gap: SPACE_XS
                    background: p.get_card()
                    border: (g.get_border_width(), p.get_border())
                    radius: g.get_radius_md()
                    shadow: g.get_shadow_md()
                    overflow: Overflow::Hidden
                    stop_scroll_propagation: true
                    EuvInput {
                        value: query
                        placeholder: resolved_ph
                        tokens: tokens
                    }
                    column {
                        width: Length::Percent(100.0)
                        gap: 2.0
                        bind_children: build_filtered
                    }
                }
            }
        };
        let handle_panel: SceneHandle = ctx.get_scene().clone();
        let prev_open: Cell<bool> = Cell::new(init_open);
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            let _: Effect =
                EuvOverlayStack::bind_signal(open, COMBOBOX_OVERLAY_NAME, Z_INDEX_DRAWER - 10);
            Effect::new(move || {
                let is_open: bool = open.get();
                if prev_open.replace(is_open) == is_open {
                    return;
                }
                let next_disp: Display = if is_open {
                    Display::Flex
                } else {
                    Display::None
                };
                handle_panel.with_write(
                    |tree: &mut SceneTree| {
                        if let Some(n) = tree.get_node_mut(panel) {
                            n.get_mut_style().get_mut_layout().set_display(next_disp);
                            let dirty_l: u16 = DIRTY_LAYOUT | DIRTY_SUBTREE_LAYOUT;
                            tree.mark_dirty(panel, dirty_l | DIRTY_PAINT);
                        }
                    },
                    (),
                );
            })
        });
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                tree.attach_scope(panel, scope);
            },
            (),
        );
        wrap
    }
}

/// Fluent controlled-state builders for `EuvDataTable`.
impl EuvDataTable {
    /// Binds a controlled search filter signal fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source data table.
    /// - `Signal<String>` - Controlled filter text signal.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated data table.
    #[must_use]
    pub fn filter(mut self, filter: Signal<String>) -> Self {
        self.set_filter(Some(filter));
        self
    }

    /// Binds a controlled ascending/descending sort direction signal fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source data table.
    /// - `Signal<bool>` - Controlled ascending sort flag signal.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated data table.
    #[must_use]
    pub fn sort_ascending(mut self, sort_ascending: Signal<bool>) -> Self {
        self.set_sort_ascending(Some(sort_ascending));
        self
    }

    /// Binds a controlled selected row index signal fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source data table.
    /// - `Signal<Option<usize>>` - Controlled selected row signal.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated data table.
    #[must_use]
    pub fn selected_row(mut self, selected_row: Signal<Option<usize>>) -> Self {
        self.set_selected_row(Some(selected_row));
        self
    }
}

/// Declarative `#[component]` implementation for `EuvDataTable`.
#[component]
/// Inherent implementation of [`EuvDataTable`].
impl EuvDataTable {
    /// Renders `EuvDataTable` into the ambient framework-managed `Context`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component descriptor reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted root scene node handle.
    fn render(&self) -> NodeId {
        let ctx: Context = current_context();
        let Self {
            headers,
            rows,
            filter: filter_opt,
            sort_ascending: sort_opt,
            selected_row: sel_opt,
            tokens,
        } = self;
        let tokens: ThemeTokens = *tokens;
        let p: ThemePalette = tokens.get_palette();
        let g: ThemeGeometry = tokens.get_geometry();
        let filter: Signal<String> = filter_opt.unwrap_or_else(|| Signal::new(String::new()));
        let sort_asc: Signal<bool> = sort_opt.unwrap_or_else(|| Signal::new(true));
        let sel_row: Signal<Option<usize>> = sel_opt.unwrap_or_else(|| Signal::new(None));
        let init_sel: Option<usize> = sel_row.get_untracked();
        let row_count: usize = rows.len();
        let mut sorted_asc: Vec<usize> = (0..row_count).collect();
        sorted_asc.sort_by(|a: &usize, b: &usize| {
            let ka: &str = rows[*a].first().map(String::as_str).unwrap_or("");
            let kb: &str = rows[*b].first().map(String::as_str).unwrap_or("");
            ka.cmp(kb)
        });
        let mut sorted_desc: Vec<usize> = (0..row_count).collect();
        sorted_desc.sort_by(|a: &usize, b: &usize| {
            let ka: &str = rows[*a].first().map(String::as_str).unwrap_or("");
            let kb: &str = rows[*b].first().map(String::as_str).unwrap_or("");
            kb.cmp(ka)
        });
        let total_search_bytes: usize = rows
            .iter()
            .map(|r: &Vec<String>| {
                r.iter().map(String::len).sum::<usize>() + r.len().saturating_sub(1)
            })
            .sum();
        let mut search_arena: Vec<u8> = Vec::with_capacity(total_search_bytes);
        let mut row_offsets: Vec<usize> = Vec::with_capacity(row_count + 1);
        row_offsets.push(0);
        for row_cells in rows.iter() {
            for (c_idx, cell) in row_cells.iter().enumerate() {
                if c_idx > 0 {
                    search_arena.push(0xFF);
                }
                for byte in cell.bytes() {
                    search_arena.push(byte.to_ascii_lowercase());
                }
            }
            row_offsets.push(search_arena.len());
        }
        let mut sort_btn: NodeId = NodeId::default();
        let mut table_box: NodeId = NodeId::default();
        let on_toggle_sort = move |_: &UiEvent| {
            let _: bool = sort_asc.update(|v: &mut bool| *v = !*v);
        };
        let wrap: NodeId = view! {
            column {
                width: Length::Percent(100.0)
                gap: SPACE_SM
                row {
                    flex_wrap: FlexWrap::Wrap
                    width: Length::Percent(100.0)
                    justify: JustifyContent::SpaceBetween
                    align: AlignItems::Center
                    gap: SPACE_SM
                    EuvInput {
                        value: filter
                        placeholder: DATATABLE_FILTER_PLACEHOLDER.to_string()
                        tokens: tokens
                    }
                    EuvButton {
                        label: DATATABLE_SORT_LABEL.to_string()
                        tokens: tokens
                        outline()
                        sm()
                        node_ref: sort_btn
                        on_click: on_toggle_sort
                    }
                }
                column {
                    node_ref: table_box
                    width: Length::Percent(100.0)
                    border: (g.get_border_width(), p.get_border())
                    radius: g.get_radius_md()
                    overflow: Overflow::Hidden
                }
            }
        };
        let card_bg: Color = p.get_card();
        let accent_bg: Color = p.get_accent();
        let mut header_row_style: Style = Style::default();
        header_row_style
            .get_mut_layout()
            .set_flex_direction(FlexDirection::Row)
            .set_width(Length::Percent(100.0))
            .set_padding(Edges::symmetric(SPACE_SM, SPACE_MD))
            .set_row_gap(SPACE_SM)
            .set_column_gap(SPACE_SM)
            .set_overflow(Overflow::Hidden);
        header_row_style
            .get_mut_paint()
            .set_background(Some(p.get_muted()));
        let mut cell_wrap_style: Style = Style::default();
        cell_wrap_style
            .get_mut_layout()
            .set_flex_grow(1.0)
            .set_min_width(Length::Px(0.0))
            .set_overflow(Overflow::Hidden);
        let mut header_text_style: Style = Style::default();
        header_text_style
            .get_mut_layout()
            .set_width(Length::Percent(100.0));
        header_text_style
            .get_mut_text()
            .set_font_size(12.0)
            .set_weight(FontWeight::SemiBold)
            .set_color(p.get_muted_foreground())
            .set_overflow(TextOverflow::Ellipsis);
        let mut data_row_style: Style = Style::default();
        data_row_style
            .get_mut_layout()
            .set_flex_direction(FlexDirection::Row)
            .set_width(Length::Percent(100.0))
            .set_padding(Edges::symmetric(SPACE_SM, SPACE_MD))
            .set_row_gap(SPACE_SM)
            .set_column_gap(SPACE_SM)
            .set_overflow(Overflow::Hidden);
        data_row_style
            .get_mut_paint()
            .set_background(Some(card_bg))
            .set_cursor(CursorIcon::Pointer);
        let mut data_text_style: Style = Style::default();
        data_text_style
            .get_mut_layout()
            .set_width(Length::Percent(100.0));
        data_text_style
            .get_mut_text()
            .set_font_size(13.0)
            .set_color(p.get_foreground())
            .set_overflow(TextOverflow::Ellipsis);
        let handle: SceneHandle = ctx.get_scene().clone();
        let (header_row, row_nodes_vec): (NodeId, Vec<NodeId>) = handle.with_write(
            |tree: &mut SceneTree| {
                if let Some(n) = tree.get_node_mut(sort_btn) {
                    n.get_mut_style()
                        .get_mut_layout()
                        .set_width(Length::Percent(100.0));
                }
                let header_id: NodeId = tree.create_view(header_row_style);
                let mut cell_buf: Vec<NodeId> = Vec::with_capacity(headers.len());
                for h in headers.iter() {
                    let cell_id: NodeId = tree.create_view(cell_wrap_style);
                    let txt_id: NodeId = tree.create_text(h.clone(), header_text_style);
                    tree.append_child(cell_id, txt_id);
                    cell_buf.push(cell_id);
                }
                tree.append_children(header_id, &cell_buf);
                let mut built_rows: Vec<NodeId> = Vec::with_capacity(row_count);
                for (orig_idx, r) in rows.iter().enumerate() {
                    let mut row_st: Style = data_row_style;
                    if init_sel == Some(orig_idx) {
                        row_st.get_mut_paint().set_background(Some(accent_bg));
                    }
                    let row_id: NodeId = tree.create_view(row_st);
                    let click_cb: UiEventHandler = Rc::new(RefCell::new(move |_: &UiEvent| {
                        let _: bool = sel_row.set_if_changed(Some(orig_idx));
                    }));
                    if let Some(row_node) = tree.get_node_mut(row_id) {
                        row_node.get_mut_handlers().set_on_click(Some(click_cb));
                        row_node.get_mut_interaction().set_focusable(true);
                    }
                    cell_buf.clear();
                    for cell_txt in r.iter() {
                        let cell_id: NodeId = tree.create_view(cell_wrap_style);
                        let txt_id: NodeId = tree.create_text(cell_txt.clone(), data_text_style);
                        tree.append_child(cell_id, txt_id);
                        cell_buf.push(cell_id);
                    }
                    tree.append_children(row_id, &cell_buf);
                    built_rows.push(row_id);
                }
                (header_id, built_rows)
            },
            (NodeId::default(), Vec::new()),
        );
        let row_nodes: Rc<Vec<NodeId>> = Rc::new(row_nodes_vec);
        let handle_rows: SceneHandle = handle.clone();
        let handle_sel: SceneHandle = handle.clone();
        let handle_cleanup: SceneHandle = handle.clone();
        let row_nodes_filter: Rc<Vec<NodeId>> = Rc::clone(&row_nodes);
        let row_nodes_sel: Rc<Vec<NodeId>> = Rc::clone(&row_nodes);
        let row_nodes_cleanup: Rc<Vec<NodeId>> = Rc::clone(&row_nodes);
        let prev_sel: Cell<Option<usize>> = Cell::new(init_sel);
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            on_cleanup(move || {
                handle_cleanup.with_write(
                    |tree: &mut SceneTree| {
                        for row_id in row_nodes_cleanup.iter().copied() {
                            if tree.get_node(row_id).is_some() {
                                tree.dispose_subtree(row_id);
                            }
                        }
                    },
                    (),
                );
            });
            let _: Effect = Effect::new(move || {
                let next_sel: Option<usize> = sel_row.get();
                let old_sel: Option<usize> = prev_sel.replace(next_sel);
                if old_sel == next_sel {
                    return;
                }
                handle_sel.with_write(
                    |tree: &mut SceneTree| {
                        if let Some(prev_idx) = old_sel
                            && let Some(&prev_node) = row_nodes_sel.get(prev_idx)
                            && let Some(n) = tree.get_node_mut(prev_node)
                        {
                            n.get_mut_style()
                                .get_mut_paint()
                                .set_background(Some(card_bg));
                            tree.mark_dirty(prev_node, DIRTY_PAINT);
                        }
                        if let Some(next_idx) = next_sel
                            && let Some(&next_node) = row_nodes_sel.get(next_idx)
                            && let Some(n) = tree.get_node_mut(next_node)
                        {
                            n.get_mut_style()
                                .get_mut_paint()
                                .set_background(Some(accent_bg));
                            tree.mark_dirty(next_node, DIRTY_PAINT);
                        }
                    },
                    (),
                );
            });
            let mut next_children: Vec<NodeId> = Vec::with_capacity(row_count + 1);
            let mut prev_children: Vec<NodeId> = Vec::with_capacity(row_count + 1);
            let mut q_heap: Vec<u8> = Vec::new();
            Effect::new(move || {
                let q_raw: String = filter.get();
                let asc: bool = sort_asc.get();
                let mut q_stack: [u8; 128] = [0u8; 128];
                let needle: &[u8] = if q_raw.len() <= q_stack.len() {
                    for (slot, byte) in q_stack.iter_mut().zip(q_raw.bytes()) {
                        *slot = byte.to_ascii_lowercase();
                    }
                    &q_stack[..q_raw.len()]
                } else {
                    q_heap.clear();
                    q_heap.reserve(q_raw.len());
                    for byte in q_raw.bytes() {
                        q_heap.push(byte.to_ascii_lowercase());
                    }
                    q_heap.as_slice()
                };
                let order: &[usize] = if asc {
                    sorted_asc.as_slice()
                } else {
                    sorted_desc.as_slice()
                };
                next_children.clear();
                next_children.push(header_row);
                if needle.is_empty() {
                    for &orig_idx in order.iter() {
                        next_children.push(row_nodes_filter[orig_idx]);
                    }
                } else {
                    for &orig_idx in order.iter() {
                        let start: usize = row_offsets[orig_idx];
                        let end: usize = row_offsets[orig_idx + 1];
                        if contains_ascii_lowered_needle(&search_arena[start..end], needle) {
                            next_children.push(row_nodes_filter[orig_idx]);
                        }
                    }
                }
                handle_rows.with_write(
                    |tree: &mut SceneTree| {
                        let unchanged: bool = tree
                            .get_node(table_box)
                            .map(|node: &SceneNode| {
                                node.get_children().as_slice() == next_children.as_slice()
                            })
                            .unwrap_or(true);
                        if unchanged {
                            return;
                        }
                        prev_children.clear();
                        if let Some(parent_node) = tree.get_node_mut(table_box) {
                            prev_children.extend_from_slice(parent_node.get_children().as_slice());
                            parent_node.get_mut_children().clear();
                        } else {
                            return;
                        }
                        for old_child in prev_children.iter().copied() {
                            if let Some(child_node) = tree.get_node_mut(old_child) {
                                child_node.set_parent(None);
                            }
                        }
                        for child in next_children.iter().copied() {
                            if let Some(child_node) = tree.get_node_mut(child) {
                                child_node.set_parent(Some(table_box));
                            }
                        }
                        if let Some(parent_node) = tree.get_node_mut(table_box) {
                            parent_node
                                .get_mut_children()
                                .extend_from_slice(next_children.as_slice());
                        }
                        let dirty_lay: u16 = DIRTY_LAYOUT | DIRTY_SUBTREE_LAYOUT;
                        let dirty_child: u16 = DIRTY_CHILDREN | DIRTY_PAINT;
                        tree.mark_dirty(table_box, dirty_lay | dirty_child);
                    },
                    (),
                );
            })
        });
        handle.with_write(
            |tree: &mut SceneTree| {
                tree.attach_scope(table_box, scope);
            },
            (),
        );
        wrap
    }
}

/// Declarative `#[component]` implementation for `EuvDatePicker`.
#[component]
/// Inherent implementation of [`EuvDatePicker`].
impl EuvDatePicker {
    /// Renders `EuvDatePicker` into the ambient framework-managed `Context`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component descriptor reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted root scene node handle.
    fn render(&self) -> NodeId {
        let ctx: Context = current_context();
        let tokens: ThemeTokens = self.get_tokens();
        let p: ThemePalette = tokens.get_palette();
        let g: ThemeGeometry = tokens.get_geometry();
        let selected_day: Signal<u32> = self.get_selected_day();
        let open: Signal<bool> = self.get_open();
        let Self { month_label, .. } = self.clone();
        let lbl_clone: String = month_label.clone();
        let mut cal_wrap: NodeId = NodeId::default();
        let chevron_color: Color = p.get_muted_foreground();
        let on_toggle = move |_: &UiEvent| {
            let _: bool = open.update(|v: &mut bool| *v = !*v);
        };
        let trigger_text = move || -> String {
            format!(
                "{DATEPICKER_TRIGGER_LABEL} · {lbl_clone} {}",
                selected_day.get()
            )
        };
        let paint_chevron = move |bounds: Rect, dl: &mut DisplayList| {
            let cx: f32 = bounds.get_x() + bounds.get_width() * 0.5;
            let cy: f32 = bounds.get_y() + bounds.get_height() * 0.5;
            dl.push(DrawCommand::Line {
                from: Point::new(cx - 4.0, cy - 2.0),
                to: Point::new(cx, cy + 2.5),
                color: chevron_color,
                width: 1.6,
            });
            dl.push(DrawCommand::Line {
                from: Point::new(cx, cy + 2.5),
                to: Point::new(cx + 4.0, cy - 2.0),
                color: chevron_color,
                width: 1.6,
            });
        };
        let cal_wrap_style = move || -> Style {
            Style::default()
                .width(Length::Percent(100.0))
                .display(if open.get() {
                    Display::Flex
                } else {
                    Display::None
                })
        };
        let wrap: NodeId = view! {
            column {
                width: Length::Percent(100.0)
                gap: SPACE_XS
                row {
                    width: Length::Percent(100.0)
                    height: Length::Px(g.get_control_md())
                    padding: Edges::symmetric(0.0, SPACE_MD)
                    justify: JustifyContent::SpaceBetween
                    align: AlignItems::Center
                    gap: SPACE_SM
                    background: p.get_background()
                    border: (g.get_border_width(), p.get_border())
                    radius: g.get_radius_md()
                    overflow: Overflow::Hidden
                    cursor: CursorIcon::Pointer
                    role: A11yRole::Button
                    a11y_label: DATEPICKER_TRIGGER_LABEL.to_string()
                    on_click: on_toggle
                    text {
                        flex_grow: 1.0
                        flex_shrink: 1.0
                        min_width: Length::Px(0.0)
                        font_size: 14.0
                        color: p.get_foreground()
                        text_overflow: TextOverflow::Ellipsis
                        { trigger_text }
                    }
                    canvas {
                        width: Length::Px(12.0)
                        height: Length::Px(12.0)
                        flex_shrink: 0.0
                        painter: paint_chevron
                    }
                }
                view {
                    node_ref: cal_wrap
                    bind_style: cal_wrap_style
                    EuvCalendar {
                        month_label: month_label
                        days_in_month: 30
                        selected_day: selected_day
                        tokens: tokens
                    }
                }
            }
        };
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            EuvOverlayStack::bind_signal(open, DATEPICKER_OVERLAY_NAME, Z_INDEX_DRAWER - 10)
        });
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                tree.attach_scope(cal_wrap, scope);
            },
            (),
        );
        wrap
    }
}

/// Fluent addon builders for `EuvInputGroup`.
impl EuvInputGroup {
    /// Sets the leading prefix addon text fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source input group.
    /// - `S` - Prefix text string.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated input group.
    #[must_use]
    pub fn prefix<S>(mut self, prefix: S) -> Self
    where
        S: Into<String>,
    {
        self.set_prefix(prefix.into());
        self
    }

    /// Sets the inner input placeholder text fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source input group.
    /// - `S` - Placeholder text string.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated input group.
    #[must_use]
    pub fn placeholder<S>(mut self, placeholder: S) -> Self
    where
        S: Into<String>,
    {
        self.set_placeholder(placeholder.into());
        self
    }

    /// Sets the trailing suffix addon text fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source input group.
    /// - `S` - Suffix text string.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated input group.
    #[must_use]
    pub fn suffix<S>(mut self, suffix: S) -> Self
    where
        S: Into<String>,
    {
        self.set_suffix(suffix.into());
        self
    }
}

/// Declarative `#[component]` implementation for `EuvInputGroup`.
#[component]
/// Inherent implementation of [`EuvInputGroup`].
impl EuvInputGroup {
    /// Renders `EuvInputGroup` into the ambient framework-managed `Context`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component descriptor reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted root scene node handle.
    fn render(&self) -> NodeId {
        let tokens: ThemeTokens = self.get_tokens();
        let p: ThemePalette = tokens.get_palette();
        let g: ThemeGeometry = tokens.get_geometry();
        let value: Signal<String> = self.get_value();
        let Self {
            prefix,
            placeholder,
            suffix,
            ..
        } = self.clone();
        view! {
            row {
                width: Length::Percent(100.0)
                align: AlignItems::Center
                background: p.get_card()
                border: (g.get_border_width(), p.get_border())
                radius: g.get_radius_md()
                overflow: Overflow::Hidden
                if !prefix.is_empty() {
                    row {
                        height: Length::Px(g.get_control_md())
                        padding: Edges::symmetric(0.0, SPACE_SM)
                        align: AlignItems::Center
                        background: p.get_muted()
                        text {
                            font_size: 12.0
                            color: p.get_muted_foreground()
                            { prefix }
                        }
                    }
                    view {
                        width: Length::Px(g.get_border_width().max(1.0))
                        height: Length::Px(g.get_control_md())
                        background: p.get_border()
                    }
                }
                column {
                    flex_grow: 1.0
                    min_width: Length::Px(40.0)
                    EuvInput {
                        value: value
                        placeholder: placeholder
                        tokens: tokens
                        borderless: true
                    }
                }
                if !suffix.is_empty() {
                    view {
                        width: Length::Px(g.get_border_width().max(1.0))
                        height: Length::Px(g.get_control_md())
                        background: p.get_border()
                    }
                    row {
                        height: Length::Px(g.get_control_md())
                        padding: Edges::symmetric(0.0, SPACE_SM)
                        align: AlignItems::Center
                        background: p.get_muted()
                        text {
                            font_size: 12.0
                            color: p.get_muted_foreground()
                            { suffix }
                        }
                    }
                }
            }
        }
    }
}

/// Builder helpers for `EuvItem`.
impl EuvItem {
    /// Sets the leading monogram or short tag fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source item.
    /// - `S` - Leading text string.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated item.
    #[must_use]
    pub fn leading<S>(mut self, leading: S) -> Self
    where
        S: Into<String>,
    {
        self.set_leading(leading.into());
        self
    }

    /// Sets the secondary muted description line fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source item.
    /// - `S` - Description string.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated item.
    #[must_use]
    pub fn description<S>(mut self, description: S) -> Self
    where
        S: Into<String>,
    {
        self.set_description(description.into());
        self
    }

    /// Sets the trailing metadata caption fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source item.
    /// - `S` - Trailing label string.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated item.
    #[must_use]
    pub fn trailing<S>(mut self, trailing: S) -> Self
    where
        S: Into<String>,
    {
        self.set_trailing(trailing.into());
        self
    }

    /// Attaches an interactive click handler to this item row.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source item.
    /// - `F` - Click callback.
    ///
    /// # Returns
    ///
    /// - `Self` - Interactive item.
    #[must_use]
    pub fn on_click<F>(mut self, handler: F) -> Self
    where
        F: FnMut(&UiEvent) + 'static,
    {
        self.set_on_click(Some(Rc::new(RefCell::new(handler))));
        self
    }
}

/// Declarative `#[component]` implementation for `EuvItem`.
#[component]
/// Inherent implementation of [`EuvItem`].
impl EuvItem {
    /// Renders `EuvItem` into the ambient framework-managed `Context`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component descriptor reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted root scene node handle.
    fn render(&self) -> NodeId {
        let mut ctx: Context = current_context();
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let Self {
            leading,
            title,
            description,
            trailing,
            on_click,
            ..
        } = self.clone();
        let row: NodeId = view! {
            row {
                width: Length::Percent(100.0)
                padding: Edges::symmetric(SPACE_SM, SPACE_MD)
                gap: SPACE_MD
                align: AlignItems::Center
                background: p.get_card()
                border: (g.get_border_width(), p.get_border())
                radius: g.get_radius_md()
                overflow: Overflow::Hidden
                cursor: CursorIcon::Pointer
                if !leading.is_empty() {
                    row {
                        width: Length::Px(32.0)
                        height: Length::Px(32.0)
                        justify: JustifyContent::Center
                        align: AlignItems::Center
                        background: p.get_secondary()
                        radius: g.get_radius_md()
                        text {
                            font_size: 12.0
                            font_weight: FontWeight::Bold
                            color: p.get_secondary_foreground()
                            { leading }
                        }
                    }
                }
                column {
                    flex_grow: 1.0
                    min_width: Length::Px(0.0)
                    gap: 2.0
                    overflow: Overflow::Hidden
                    text {
                        width: Length::Percent(100.0)
                        font_size: 13.0
                        font_weight: FontWeight::SemiBold
                        color: p.get_foreground()
                        { title }
                    }
                    if !description.is_empty() {
                        text {
                            width: Length::Percent(100.0)
                            font_size: 12.0
                            color: p.get_muted_foreground()
                            { description }
                        }
                    }
                }
                if !trailing.is_empty() {
                    text {
                        font_size: 12.0
                        font_weight: FontWeight::Medium
                        color: p.get_muted_foreground()
                        { trailing }
                    }
                }
            }
        };
        if let Some(cb) = on_click {
            ctx.on_click(row, move |ev: &UiEvent| {
                if let Ok(mut f) = cb.try_borrow_mut() {
                    f(ev);
                }
            });
        }
        row
    }
}

/// Declarative `#[component]` implementation for `EuvNativeSelect`.
#[component]
/// Inherent implementation of [`EuvNativeSelect`].
impl EuvNativeSelect {
    /// Renders `EuvNativeSelect` into the ambient framework-managed `Context`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component descriptor reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted root scene node handle.
    fn render(&self) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let selected: Signal<usize> = self.get_selected();
        let Self { options, .. } = self.clone();
        let total: usize = options.len().max(1);
        let chevron_color: Color = p.get_muted_foreground();
        let on_cycle = move |_: &UiEvent| {
            let _: bool = selected.update(|v: &mut usize| *v = (*v + 1) % total);
        };
        let label_text = move || -> String {
            let idx: usize = selected.get() % total;
            options
                .get(idx)
                .cloned()
                .unwrap_or_else(|| NATIVE_SELECT_DEFAULT_LABEL.to_string())
        };
        let paint_chevrons = move |bounds: Rect, dl: &mut DisplayList| {
            let cx: f32 = bounds.get_x() + bounds.get_width() * 0.5;
            let cy: f32 = bounds.get_y() + bounds.get_height() * 0.5;
            dl.push(DrawCommand::Line {
                from: Point::new(cx - 3.5, cy - 2.0),
                to: Point::new(cx, cy - 5.0),
                color: chevron_color,
                width: 1.5,
            });
            dl.push(DrawCommand::Line {
                from: Point::new(cx, cy - 5.0),
                to: Point::new(cx + 3.5, cy - 2.0),
                color: chevron_color,
                width: 1.5,
            });
            dl.push(DrawCommand::Line {
                from: Point::new(cx - 3.5, cy + 2.0),
                to: Point::new(cx, cy + 5.0),
                color: chevron_color,
                width: 1.5,
            });
            dl.push(DrawCommand::Line {
                from: Point::new(cx, cy + 5.0),
                to: Point::new(cx + 3.5, cy + 2.0),
                color: chevron_color,
                width: 1.5,
            });
        };
        view! {
            row {
                width: Length::Percent(100.0)
                height: Length::Px(g.get_control_md())
                padding: Edges::symmetric(0.0, SPACE_MD)
                justify: JustifyContent::SpaceBetween
                align: AlignItems::Center
                gap: SPACE_SM
                background: p.get_background()
                border: (g.get_border_width(), p.get_border())
                radius: g.get_radius_md()
                overflow: Overflow::Hidden
                cursor: CursorIcon::Pointer
                on_click: on_cycle
                text {
                    flex_grow: 1.0
                    flex_shrink: 1.0
                    min_width: Length::Px(0.0)
                    font_size: 14.0
                    color: p.get_foreground()
                    text_overflow: TextOverflow::Ellipsis
                    { label_text }
                }
                canvas {
                    width: Length::Px(12.0)
                    height: Length::Px(14.0)
                    flex_shrink: 0.0
                    painter: paint_chevrons
                }
            }
        }
    }
}

/// Declarative `#[component]` implementation for `EuvNavigationMenu`.
#[component]
/// Inherent implementation of [`EuvNavigationMenu`].
impl EuvNavigationMenu {
    /// Renders `EuvNavigationMenu` into the ambient framework-managed `Context`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component descriptor reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted root scene node handle.
    fn render(&self) -> NodeId {
        let tokens: ThemeTokens = self.get_tokens();
        let p: ThemePalette = tokens.get_palette();
        let g: ThemeGeometry = tokens.get_geometry();
        let active: Signal<Option<usize>> = self.get_active();
        let Self { items, .. } = self.clone();
        let summaries: Vec<String> = items
            .iter()
            .map(|(_, s): &(String, String)| s.clone())
            .collect();
        view! {
            column {
                width: Length::Percent(100.0)
                gap: SPACE_XS
                row {
                    flex_wrap: FlexWrap::Wrap
                    width: Length::Percent(100.0)
                    gap: SPACE_XS
                    for (idx, (label, _)) in items.into_iter().enumerate() {
                        EuvButton {
                            label: label
                            tokens: tokens
                            ghost()
                            sm()
                            on_click: move |_: &UiEvent| {
                                let cur: Option<usize> = active.get_untracked();
                                let next: Option<usize> = if cur == Some(idx) { None } else { Some(idx) };
                                let _: bool = active.set_if_changed(next);
                            }
                        }
                    }
                }
                column {
                    bind_style: move || {
                        Style::default()
                        .column()
                        .display(if active.get().is_some() {
                            Display::Flex
                            } else {
                                Display::None
                        })
                        .width(Length::Percent(100.0))
                        .padding(Edges::all(SPACE_MD))
                        .background(p.get_card())
                        .border(g.get_border_width(), p.get_border())
                        .radius(g.get_radius_md())
                        .overflow(Overflow::Hidden)
                    }
                    text {
                        width: Length::Percent(100.0)
                        font_size: 13.0
                        color: p.get_muted_foreground()
                        { move || {
                            active
                            .get()
                            .and_then(|i: usize| summaries.get(i).cloned())
                            .unwrap_or_default()
                        } }
                    }
                }
            }
        }
    }
}

/// Declarative `#[component]` implementation for `EuvResizable`.
#[component]
/// Inherent implementation of [`EuvResizable`].
impl EuvResizable {
    /// Renders `EuvResizable` into the ambient framework-managed `Context`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component descriptor reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted root scene node handle.
    fn render(&self) -> NodeId {
        let mut ctx: Context = current_context();
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let ratio: Signal<f32> = self.get_split_ratio();
        let left: NodeId = self.get_left();
        let right: NodeId = self.get_right();
        let mut grip: NodeId = NodeId::default();
        let container: NodeId = view! {
            row {
                width: Length::Percent(100.0)
                height: Length::Px(132.0)
                background: p.get_card()
                border: (g.get_border_width(), p.get_border())
                radius: g.get_radius_md()
                overflow: Overflow::Hidden
                column {
                    bind_style: move || {
                        let pct: f32 = (ratio.get().clamp(0.25, 0.75)) * 100.0;
                        Style::default()
                        .column()
                        .width(Length::Percent(pct))
                        .height(Length::Percent(100.0))
                        .padding(Edges::all(SPACE_MD))
                        .overflow(Overflow::Hidden)
                    }
                    { left }
                }
                view {
                    node_ref: grip
                    width: Length::Px(8.0)
                    height: Length::Percent(100.0)
                    background: p.get_border()
                    cursor: CursorIcon::EwResize
                }
                column {
                    flex_grow: 1.0
                    height: Length::Percent(100.0)
                    padding: Edges::all(SPACE_MD)
                    overflow: Overflow::Hidden
                    { right }
                }
            }
        };
        let dragging: Rc<Cell<bool>> = Rc::new(Cell::new(false));
        let drag_down: Rc<Cell<bool>> = Rc::clone(&dragging);
        ctx.on_pointer_down(grip, move |_: &UiEvent| {
            drag_down.set(true);
        });
        let drag_up: Rc<Cell<bool>> = Rc::clone(&dragging);
        ctx.on_pointer_up(grip, move |_: &UiEvent| {
            drag_up.set(false);
        });
        let scene_move: SceneHandle = ctx.get_scene().clone();
        ctx.on_pointer_move(grip, move |ev: &UiEvent| {
            if !dragging.get() {
                return;
            }
            if let UiEventKind::PointerMove(pt) = ev.get_kind() {
                let bounds: Rect = scene_move.with_read(
                    |tree: &SceneTree| {
                        tree.node_screen_bounds(container).unwrap_or_else(|| {
                            tree.get_node(container)
                                .map(|n: &SceneNode| n.get_layout_box().get_bounds())
                                .unwrap_or_default()
                        })
                    },
                    Rect::default(),
                );
                if bounds.get_width() > 40.0 {
                    let next: f32 =
                        ((pt.get_x() - bounds.get_x()) / bounds.get_width()).clamp(0.25, 0.75);
                    let _: bool = ratio.set_if_changed(next);
                }
            }
        });
        container
    }
}

/// Declarative `#[component]` implementation for `EuvSeparator`.
#[component]
/// Inherent implementation of [`EuvSeparator`].
impl EuvSeparator {
    /// Renders `EuvSeparator` into the ambient framework-managed `Context`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component descriptor reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted root scene node handle.
    fn render(&self) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let Self { label, .. } = self.clone();
        if label.is_empty() {
            return view! {
                view {
                    width: Length::Percent(100.0)
                    height: Length::Px(1.0)
                    background: p.get_border()
                }
            };
        }
        view! {
            row {
                width: Length::Percent(100.0)
                align: AlignItems::Center
                gap: SPACE_SM
                view {
                    flex_grow: 1.0
                    height: Length::Px(1.0)
                    background: p.get_border()
                }
                text {
                    font_size: 11.0
                    color: p.get_muted_foreground()
                    { label }
                }
                view {
                    flex_grow: 1.0
                    height: Length::Px(1.0)
                    background: p.get_border()
                }
            }
        }
    }
}

/// Declarative `#[component]` implementation for `EuvSonner`.
#[component]
/// Inherent implementation of [`EuvSonner`].
impl EuvSonner {
    /// Renders `EuvSonner` into the ambient framework-managed `Context`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component descriptor reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted root scene node handle.
    fn render(&self) -> NodeId {
        let tokens: ThemeTokens = self.get_tokens();
        let p: ThemePalette = tokens.get_palette();
        let g: ThemeGeometry = tokens.get_geometry();
        let messages: Signal<Vec<(String, String)>> = self.get_messages();
        view! {
            column {
                width: Length::Percent(100.0)
                gap: SPACE_XS
                bind_children: move || {
                    messages
                    .get()
                    .into_iter()
                    .enumerate()
                    .map(|(idx, (title, desc)): (usize, (String, String))| {
                        view! {
                            row {
                                width: Length::Percent(100.0)
                                padding: Edges::symmetric(SPACE_SM, SPACE_MD)
                                justify: JustifyContent::SpaceBetween
                                align: AlignItems::Center
                                gap: SPACE_SM
                                background: p.get_card()
                                border: (g.get_border_width(), p.get_border())
                                radius: g.get_radius_md()
                                shadow: g.get_shadow_md()
                                overflow: Overflow::Hidden
                                column {
                                    flex_grow: 1.0
                                    gap: 2.0
                                    overflow: Overflow::Hidden
                                    text {
                                        width: Length::Percent(100.0)
                                        font_size: 13.0
                                        font_weight: FontWeight::SemiBold
                                        color: p.get_card_foreground()
                                        { title }
                                    }
                                    text {
                                        width: Length::Percent(100.0)
                                        font_size: 12.0
                                        color: p.get_muted_foreground()
                                        { desc }
                                    }
                                }
                                EuvButton {
                                    label: SONNER_DISMISS_LABEL.to_string()
                                    tokens: tokens
                                    ghost()
                                    sm()
                                    on_click: move |_: &UiEvent| {
                                        let _: bool =
                                        messages.update(|v: &mut Vec<(String, String)>| {
                                            if idx < v.len() {
                                                let _: (String, String) = v.remove(idx);
                                            }
                                        });
                                    }
                                }
                            }
                        }
                    })
                    .collect()
                }
            }
        }
    }
}

/// Declarative `#[component]` implementation for `EuvDirection`.
#[component]
/// Inherent implementation of [`EuvDirection`].
impl EuvDirection {
    /// Renders `EuvDirection` into the ambient framework-managed `Context`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component descriptor reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted root scene node handle.
    fn render(&self) -> NodeId {
        let align: AlignItems = match self.get_direction() {
            EuvTextDirection::Ltr => AlignItems::Start,
            EuvTextDirection::Rtl => AlignItems::End,
        };
        let child: NodeId = self.get_child();
        view! {
            column {
                width: Length::Percent(100.0)
                align: align
                { child }
            }
        }
    }
}

/// Declarative `#[component]` implementation for `EuvTrace`.
#[component]
/// Inherent implementation of [`EuvTrace`].
impl EuvTrace {
    /// Renders `EuvTrace` into the ambient framework-managed `Context`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component descriptor reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted root scene node handle.
    fn render(&self) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let Self { title, spans, .. } = self.clone();
        let rows_data: Vec<(String, f32, Color, f32, f32)> = spans
            .into_iter()
            .map(|span: EuvTraceSpan| {
                let (bar_bg, _): (Color, Color) = span.get_tone().colors(p);
                let left_pct: f32 = (span.get_offset_frac().clamp(0.0, 0.9)) * 100.0;
                let bar_pct: f32 = (span
                    .get_width_frac()
                    .clamp(0.05, 1.0 - span.get_offset_frac()))
                    * 100.0;
                (
                    span.get_name().clone(),
                    span.get_duration_ms(),
                    bar_bg,
                    left_pct,
                    bar_pct,
                )
            })
            .collect();
        view! {
            column {
                width: Length::Percent(100.0)
                padding: Edges::all(SPACE_MD)
                gap: SPACE_SM
                background: p.get_card()
                border: (g.get_border_width(), p.get_border())
                radius: g.get_radius_md()
                text {
                    font_size: 13.5
                    font_weight: FontWeight::SemiBold
                    color: p.get_foreground()
                    { title }
                }
                for (name, dur_ms, bar_bg, left_pct, bar_pct) in rows_data.into_iter() {
                    column {
                        width: Length::Percent(100.0)
                        gap: 4.0
                        row {
                            width: Length::Percent(100.0)
                            justify: JustifyContent::SpaceBetween
                            align: AlignItems::Center
                            text {
                                font_size: 12.0
                                font_weight: FontWeight::Medium
                                color: p.get_foreground()
                                { name }
                            }
                            text {
                                font_size: 11.5
                                color: p.get_muted_foreground()
                                { format!("{dur_ms:.2}ms") }
                            }
                        }
                        row {
                            width: Length::Percent(100.0)
                            height: Length::Px(8.0)
                            background: p.get_muted()
                            radius: 4.0
                            overflow: Overflow::Hidden
                            if left_pct > 0.5 {
                                view {
                                    width: Length::Percent(left_pct)
                                    height: Length::Percent(100.0)
                                }
                            }
                            view {
                                width: Length::Percent(bar_pct)
                                height: Length::Percent(100.0)
                                background: bar_bg
                                radius: 4.0
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Declarative `#[component]` implementation for `EuvAttachment`.
#[component]
/// Inherent implementation of [`EuvAttachment`].
impl EuvAttachment {
    /// Renders `EuvAttachment` into the ambient framework-managed `Context`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component descriptor reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted root scene node handle.
    fn render(&self) -> NodeId {
        let tokens: ThemeTokens = self.get_tokens();
        let p: ThemePalette = tokens.get_palette();
        let g: ThemeGeometry = tokens.get_geometry();
        let visible: Signal<bool> = self.get_visible();
        let Self { filename, meta, .. } = self.clone();
        view! {
            row {
                bind_style: move || {
                    Style::default()
                    .row()
                    .display(if visible.get() {
                        Display::Flex
                        } else {
                            Display::None
                    })
                    .width(Length::Percent(100.0))
                    .justify(JustifyContent::SpaceBetween)
                    .align(AlignItems::Center)
                    .padding(Edges::symmetric(SPACE_SM, SPACE_MD))
                    .gap(SPACE_SM)
                    .background(p.get_muted())
                    .border(g.get_border_width(), p.get_border())
                    .radius(g.get_radius_md())
                }
                column {
                    flex_grow: 1.0
                    flex_shrink: 1.0
                    min_width: Length::Px(0.0)
                    gap: 2.0
                    overflow: Overflow::Hidden
                    text {
                        width: Length::Percent(100.0)
                        font_size: 13.0
                        font_weight: FontWeight::Medium
                        color: p.get_foreground()
                        { filename }
                    }
                    text {
                        width: Length::Percent(100.0)
                        font_size: 11.5
                        color: p.get_muted_foreground()
                        { meta }
                    }
                }
                EuvButton {
                    label: ATTACHMENT_REMOVE_LABEL.to_string()
                    tokens: tokens
                    outline()
                    sm()
                    on_click: move |_: &UiEvent| {
                        let _: bool = visible.set_if_changed(false);
                    }
                }
            }
        }
    }
}

/// Fluent metadata and alignment builders for `EuvBubble`.
impl EuvBubble {
    /// Sets the message timestamp caption fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source bubble.
    /// - `S` - Timestamp caption string.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated bubble.
    #[must_use]
    pub fn timestamp<S>(mut self, timestamp: S) -> Self
    where
        S: Into<String>,
    {
        self.set_timestamp(timestamp.into());
        self
    }

    /// Marks the bubble as outgoing (right-aligned primary surface) or incoming fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source bubble.
    /// - `bool` - Whether this bubble represents an outgoing message.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated bubble.
    #[must_use]
    pub fn outgoing(mut self, outgoing: bool) -> Self {
        self.set_outgoing(outgoing);
        self
    }
}

/// Declarative `#[component]` implementation for `EuvBubble`.
#[component]
/// Inherent implementation of [`EuvBubble`].
impl EuvBubble {
    /// Renders `EuvBubble` into the ambient framework-managed `Context`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component descriptor reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted root scene node handle.
    fn render(&self) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let outgoing: bool = self.get_outgoing();
        let Self {
            author,
            content,
            timestamp,
            ..
        } = self.clone();
        let fg: Color = if outgoing {
            p.get_primary_foreground()
        } else {
            p.get_foreground()
        };
        let meta_fg: Color = if outgoing {
            p.get_primary_foreground().multiply_alpha(0.75)
        } else {
            p.get_muted_foreground()
        };
        let header_line: String = if timestamp.is_empty() {
            author
        } else {
            format!("{author} · {timestamp}")
        };
        view! {
            row {
                width: Length::Percent(100.0)
                justify: if outgoing { JustifyContent::End } else { JustifyContent::Start }
                overflow: Overflow::Hidden
                column {
                    width: Length::Percent(86.0)
                    min_width: Length::Px(0.0)
                    flex_shrink: 1.0
                    padding: Edges::symmetric(SPACE_SM, SPACE_MD)
                    gap: 4.0
                    background: if outgoing { p.get_primary() } else { p.get_muted() }
                    border: (
                        if outgoing { 0.0 } else { g.get_border_width() },
                        p.get_border(),
                    )
                    radius: g.get_radius_lg()
                    overflow: Overflow::Hidden
                    text {
                        width: Length::Percent(100.0)
                        font_size: 11.0
                        font_weight: FontWeight::SemiBold
                        color: meta_fg
                        { header_line }
                    }
                    text {
                        width: Length::Percent(100.0)
                        font_size: 13.0
                        color: fg
                        { content }
                    }
                }
            }
        }
    }
}

/// Declarative `#[component]` implementation for `EuvMarker`.
#[component]
/// Inherent implementation of [`EuvMarker`].
impl EuvMarker {
    /// Renders `EuvMarker` into the ambient framework-managed `Context`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component descriptor reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted root scene node handle.
    fn render(&self) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let (dot_bg, dot_fg): (Color, Color) = self.get_tone().colors(p);
        let Self { code, label, .. } = self.clone();
        view! {
            row {
                align: AlignItems::Center
                padding: Edges::symmetric(SPACE_XS, SPACE_SM)
                gap: SPACE_XS
                background: p.get_card()
                border: (g.get_border_width(), p.get_border())
                radius: g.get_radius_md()
                row {
                    width: Length::Px(20.0)
                    height: Length::Px(20.0)
                    justify: JustifyContent::Center
                    align: AlignItems::Center
                    background: dot_bg
                    radius: 10.0
                    text {
                        font_size: 10.5
                        font_weight: FontWeight::Bold
                        color: dot_fg
                        text_align: TextAlign::Center
                        { code }
                    }
                }
                text {
                    font_size: 12.0
                    font_weight: FontWeight::Medium
                    color: p.get_foreground()
                    { label }
                }
            }
        }
    }
}

/// Declarative `#[component]` implementation for `EuvQuestionnaire`.
#[component]
/// Inherent implementation of [`EuvQuestionnaire`].
impl EuvQuestionnaire {
    /// Renders `EuvQuestionnaire` into the ambient framework-managed `Context`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component descriptor reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted root scene node handle.
    fn render(&self) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let selected: Signal<Option<usize>> = self.get_selected();
        let Self {
            prompt, options, ..
        } = self.clone();
        view! {
            column {
                width: Length::Percent(100.0)
                padding: Edges::all(SPACE_MD)
                gap: SPACE_SM
                background: p.get_card()
                border: (g.get_border_width(), p.get_border())
                radius: g.get_radius_md()
                overflow: Overflow::Hidden
                text {
                    width: Length::Percent(100.0)
                    font_size: 13.5
                    font_weight: FontWeight::SemiBold
                    color: p.get_foreground()
                    { prompt }
                }
                for (idx, opt) in options.into_iter().enumerate() {
                    row {
                        bind_style: move || {
                            let is_sel: bool = selected.get() == Some(idx);
                            Style::default()
                            .row()
                            .width(Length::Percent(100.0))
                            .padding(Edges::symmetric(SPACE_SM, SPACE_MD))
                            .align(AlignItems::Center)
                            .overflow(Overflow::Hidden)
                            .background(if is_sel {
                                p.get_accent()
                                } else {
                                    p.get_background()
                            })
                            .border(
                                g.get_border_width(),
                                if is_sel {
                                    p.get_primary()
                                } else {
                                    p.get_border()
                                },
                            )
                            .radius(g.get_radius_sm())
                            .cursor(CursorIcon::Pointer)
                        }
                        on_click: move |_: &UiEvent| {
                            let _: bool = selected.set_if_changed(Some(idx));
                        }
                        text {
                            width: Length::Percent(100.0)
                            font_size: 13.0
                            font_weight: FontWeight::Medium
                            color: p.get_foreground()
                            { opt }
                        }
                    }
                }
            }
        }
    }
}

/// Fluent controlled-state builders for `EuvContextMenu`.
impl EuvContextMenu {
    /// Binds a controlled menu open state signal fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source context menu.
    /// - `Signal<bool>` - Controlled open flag signal.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated context menu.
    #[must_use]
    pub fn open(mut self, open: Signal<bool>) -> Self {
        self.set_open(Some(open));
        self
    }

    /// Binds a controlled selected action index signal fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source context menu.
    /// - `Signal<Option<usize>>` - Controlled selected item index signal.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated context menu.
    #[must_use]
    pub fn selected(mut self, selected: Signal<Option<usize>>) -> Self {
        self.set_selected(Some(selected));
        self
    }
}

/// Declarative `#[component]` implementation for `EuvContextMenu`.
#[component]
/// Inherent implementation of [`EuvContextMenu`].
impl EuvContextMenu {
    /// Renders `EuvContextMenu` into the ambient framework-managed `Context`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component descriptor reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted root scene node handle.
    fn render(&self) -> NodeId {
        let ctx: Context = current_context();
        let tokens: ThemeTokens = self.get_tokens();
        let p: ThemePalette = tokens.get_palette();
        let g: ThemeGeometry = tokens.get_geometry();
        let gesture_sig: Signal<Option<GestureEvent>> = ctx.get_gesture();
        let Self {
            trigger_label,
            items,
            open: open_opt,
            selected: sel_opt,
            ..
        } = self.clone();
        let open: Signal<bool> = open_opt.unwrap_or_else(|| Signal::new(false));
        let selected: Signal<Option<usize>> = sel_opt.unwrap_or_else(|| Signal::new(None));
        let init_open: bool = open.get_untracked();
        let mut trigger: NodeId = NodeId::default();
        let mut menu: NodeId = NodeId::default();
        let wrap: NodeId = view! {
            column {
                width: Length::Percent(100.0)
                gap: SPACE_XS
                column {
                    node_ref: trigger
                    width: Length::Percent(100.0)
                    padding: Edges::symmetric(SPACE_MD, SPACE_LG)
                    justify: JustifyContent::Center
                    align: AlignItems::Center
                    gap: SPACE_XS
                    background: p.get_muted()
                    border: (g.get_border_width(), p.get_border())
                    radius: g.get_radius_md()
                    cursor: CursorIcon::Pointer
                    role: A11yRole::Button
                    a11y_label: trigger_label.clone()
                    on_click: move |_: &UiEvent| {
                        let _: bool = open.update(|v: &mut bool| *v = !*v);
                    }
                    text {
                        font_size: 13.0
                        font_weight: FontWeight::SemiBold
                        color: p.get_foreground()
                        text_align: TextAlign::Center
                        { trigger_label }
                    }
                    text {
                        font_size: 11.5
                        color: p.get_muted_foreground()
                        text_align: TextAlign::Center
                        { CONTEXT_MENU_HINT_LABEL }
                    }
                }
                column {
                    node_ref: menu
                    display: if init_open { Display::Flex } else { Display::None }
                    width: Length::Percent(100.0)
                    padding: Edges::all(SPACE_XS)
                    gap: 2.0
                    background: p.get_card()
                    border: (g.get_border_width(), p.get_border())
                    radius: g.get_radius_md()
                    shadow: g.get_shadow_md()
                    z_index: Z_INDEX_FLOATING
                    role: A11yRole::Menu
                    for (idx, item_label) in items.into_iter().enumerate() {
                        row {
                            width: Length::Percent(100.0)
                            padding: Edges::symmetric(SPACE_SM, SPACE_MD)
                            radius: g.get_radius_sm()
                            cursor: CursorIcon::Pointer
                            role: A11yRole::MenuItem
                            a11y_label: item_label.clone()
                            on_click: move |_: &UiEvent| {
                                let _: bool = selected.set_if_changed(Some(idx));
                                let _: bool = open.set_if_changed(false);
                            }
                            text {
                                font_size: 13.0
                                color: p.get_foreground()
                                { item_label }
                            }
                        }
                    }
                }
            }
        };
        let handle: SceneHandle = ctx.get_scene().clone();
        let prev_open: Rc<Cell<bool>> = Rc::new(Cell::new(init_open));
        let last_seq: Rc<Cell<u64>> = Rc::new(Cell::new(
            gesture_sig
                .get_untracked()
                .map(|ev: GestureEvent| ev.get_sequence())
                .unwrap_or(0),
        ));
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            let _: Effect =
                EuvOverlayStack::bind_signal(open, CONTEXT_MENU_OVERLAY_NAME, Z_INDEX_DRAWER - 10);
            let _: Effect = Effect::new(move || {
                if let Some(ev) = gesture_sig.get()
                    && ev.get_sequence() != last_seq.replace(ev.get_sequence())
                    && ev.get_kind() == GestureEventKind::LongPress
                    && let Some(bounds) = handle.node_screen_bounds(trigger)
                {
                    let pos: Point = ev.get_position();
                    if pos.get_x() >= bounds.get_x()
                        && pos.get_x() <= bounds.right()
                        && pos.get_y() >= bounds.get_y()
                        && pos.get_y() <= bounds.bottom()
                    {
                        let _: bool = open.set_if_changed(true);
                    }
                }
            });
            let handle_anim: SceneHandle = ctx.get_scene().clone();
            Effect::new(move || {
                let is_open: bool = open.get();
                let was_open: bool = prev_open.replace(is_open);
                if is_open == was_open {
                    return;
                }
                if is_open {
                    handle_anim.start_transition(NodeTransition::slide_fade(
                        menu,
                        Point::new(0.0, -6.0),
                        Point::new(0.0, 0.0),
                        0.0,
                        1.0,
                        0.16,
                        false,
                    ));
                } else {
                    handle_anim.start_transition(NodeTransition::slide_fade(
                        menu,
                        Point::new(0.0, 0.0),
                        Point::new(0.0, -4.0),
                        1.0,
                        0.0,
                        0.14,
                        true,
                    ));
                }
            })
        });
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                tree.attach_scope(menu, scope);
            },
            (),
        );
        wrap
    }
}
