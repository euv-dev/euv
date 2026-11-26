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
        let tokens: ThemeTokens = self.get_tokens();
        let p: ThemePalette = tokens.get_palette();
        let g: ThemeGeometry = tokens.get_geometry();
        let selected: Signal<usize> = self.get_selected();
        let open: Signal<bool> = self.get_open();
        let Self {
            placeholder,
            options,
            query: query_opt,
            ..
        } = self.clone();
        let query: Signal<String> = query_opt.unwrap_or_else(|| Signal::new(String::new()));
        let resolved_ph: String = if placeholder.is_empty() {
            COMBOBOX_DEFAULT_PLACEHOLDER.to_string()
        } else {
            placeholder
        };
        let opts_trigger: Vec<String> = options.clone();
        let opts_filter: Vec<String> = options;
        let mut panel: NodeId = NodeId::default();
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
                    background: p.get_card()
                    border: (g.get_border_width(), p.get_border())
                    radius: g.get_radius_md()
                    cursor: CursorIcon::Pointer
                    on_click: move |_: &UiEvent| {
                        let _: bool = open.update(|v: &mut bool| *v = !*v);
                    }
                    text {
                        font_size: 13.0
                        color: p.get_foreground()
                        { move || {
                                let idx: usize = selected.get();
                                let lbl: String = opts_trigger
                                .get(idx)
                                .cloned()
                                .unwrap_or_else(|| COMBOBOX_EMPTY_SELECTION.to_string());
                                format!("{lbl}  v")
                        } }
                    }
                }
                column {
                    node_ref: panel
                    bind_style: move || {
                        Style::default()
                        .column()
                        .display(if open.get() {
                                Display::Flex
                            } else {
                                Display::None
                        })
                        .width(Length::Percent(100.0))
                        .padding(Edges::all(SPACE_SM))
                        .gap(SPACE_XS)
                        .background(p.get_card())
                        .border(g.get_border_width(), p.get_border())
                        .radius(g.get_radius_md())
                        .shadow(g.get_shadow_md())
                        .stop_scroll_propagation(true)
                    }
                    EuvInput(query, resolved_ph, tokens)
                    column {
                        width: Length::Percent(100.0)
                        gap: 2.0
                        bind_children: move || {
                            let q: String = query.get().to_ascii_lowercase();
                            let cur_sel: usize = selected.get();
                            let mut nodes: Vec<NodeId> = Vec::new();
                            for (idx, opt) in opts_filter.iter().enumerate() {
                                if !q.is_empty() && !opt.to_ascii_lowercase().contains(&q) {
                                    continue;
                                }
                                let is_sel: bool = idx == cur_sel;
                                nodes.push(view! {
                                        row {
                                            width: Length::Percent(100.0)
                                            height: Length::Px(g.get_control_sm())
                                            padding: Edges::symmetric(0.0, SPACE_SM)
                                            align: AlignItems::Center
                                            radius: g.get_radius_sm()
                                            background: if is_sel {
                                                p.get_accent()
                                            } else {
                                                COLOR_TRANSPARENT
                                            }
                                            cursor: CursorIcon::Pointer
                                            on_click: move |_: &UiEvent| {
                                                let _: bool = selected.set_if_changed(idx);
                                                let _: bool = open.set_if_changed(false);
                                            }
                                            text {
                                                font_size: 13.0
                                                color: p.get_foreground()
                                                { if is_sel {
                                                        format!("* {opt}")
                                                    } else {
                                                        opt.clone()
                                                } }
                                            }
                                        }
                                });
                            }
                            nodes
                        }
                    }
                }
            }
        };
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            EuvOverlayStack::bind_signal(open, COMBOBOX_OVERLAY_NAME, Z_INDEX_DRAWER - 10)
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
        let tokens: ThemeTokens = self.get_tokens();
        let p: ThemePalette = tokens.get_palette();
        let g: ThemeGeometry = tokens.get_geometry();
        let Self {
            headers,
            rows,
            filter: filter_opt,
            sort_ascending: sort_opt,
            selected_row: sel_opt,
            ..
        } = self.clone();
        let filter: Signal<String> = filter_opt.unwrap_or_else(|| Signal::new(String::new()));
        let sort_asc: Signal<bool> = sort_opt.unwrap_or_else(|| Signal::new(true));
        let sel_row: Signal<Option<usize>> = sel_opt.unwrap_or_else(|| Signal::new(None));
        let mut sort_btn: NodeId = NodeId::default();
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
                    EuvInput(filter, DATATABLE_FILTER_PLACEHOLDER.to_string(), tokens)
                    EuvButton(DATATABLE_SORT_LABEL.to_string(), tokens) {
                        outline()
                        sm()
                        node_ref: sort_btn
                        on_click: move |_: &UiEvent| {
                            let _: bool = sort_asc.update(|v: &mut bool| *v = !*v);
                        }
                    }
                }
                column {
                    width: Length::Percent(100.0)
                    border: (g.get_border_width(), p.get_border())
                    radius: g.get_radius_md()
                    overflow: Overflow::Hidden
                    bind_children: move || {
                        let q: String = filter.get().to_ascii_lowercase();
                        let asc: bool = sort_asc.get();
                        let active_sel: Option<usize> = sel_row.get();
                        let mut filtered: Vec<(usize, Vec<String>)> = rows
                        .iter()
                        .enumerate()
                        .filter(|(_, r): &(usize, &Vec<String>)| {
                                q.is_empty()
                                || r.iter()
                                .any(|cell: &String| cell.to_ascii_lowercase().contains(&q))
                        })
                        .map(|(i, r): (usize, &Vec<String>)| (i, r.clone()))
                        .collect();
                        filtered.sort_by(|a: &(usize, Vec<String>), b: &(usize, Vec<String>)| {
                                let ka: &str = a.1.first().map(String::as_str).unwrap_or("");
                                let kb: &str = b.1.first().map(String::as_str).unwrap_or("");
                                if asc { ka.cmp(kb) } else { kb.cmp(ka) }
                        });
                        let mut out: Vec<NodeId> = Vec::with_capacity(filtered.len() + 1);
                        out.push(view! {
                                row {
                                    width: Length::Percent(100.0)
                                    padding: Edges::symmetric(SPACE_SM, SPACE_MD)
                                    background: p.get_muted()
                                    gap: SPACE_SM
                                    overflow: Overflow::Hidden
                                    for h in headers.iter() {
                                        view {
                                            flex_grow: 1.0
                                            min_width: Length::Px(0.0)
                                            overflow: Overflow::Hidden
                                            text {
                                                width: Length::Percent(100.0)
                                                font_size: 12.0
                                                font_weight: FontWeight::SemiBold
                                                color: p.get_muted_foreground()
                                                text_overflow: TextOverflow::Ellipsis
                                                { h.clone() }
                                            }
                                        }
                                    }
                                }
                        });
                        for (orig_idx, r) in filtered.into_iter() {
                            let is_sel: bool = active_sel == Some(orig_idx);
                            out.push(view! {
                                    row {
                                        width: Length::Percent(100.0)
                                        padding: Edges::symmetric(SPACE_SM, SPACE_MD)
                                        background: if is_sel { p.get_accent() } else { p.get_card() }
                                        gap: SPACE_SM
                                        overflow: Overflow::Hidden
                                        cursor: CursorIcon::Pointer
                                        on_click: move |_: &UiEvent| {
                                            let _: bool = sel_row.set_if_changed(Some(orig_idx));
                                        }
                                        for cell_txt in r.into_iter() {
                                            view {
                                                flex_grow: 1.0
                                                min_width: Length::Px(0.0)
                                                overflow: Overflow::Hidden
                                                text {
                                                    width: Length::Percent(100.0)
                                                    font_size: 13.0
                                                    color: p.get_foreground()
                                                    text_overflow: TextOverflow::Ellipsis
                                                    { cell_txt }
                                                }
                                            }
                                        }
                                    }
                            });
                        }
                        out
                    }
                }
            }
        };
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                if let Some(n) = tree.get_node_mut(sort_btn) {
                    let mut st: Style = n.get_style();
                    st.get_mut_layout().set_width(Length::Percent(100.0));
                    n.set_style(st);
                }
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
        let selected_day: Signal<u32> = self.get_selected_day();
        let open: Signal<bool> = self.get_open();
        let Self { month_label, .. } = self.clone();
        let lbl_clone: String = month_label.clone();
        let mut cal_wrap: NodeId = NodeId::default();
        let wrap: NodeId = view! {
            column {
                width: Length::Percent(100.0)
                max_width: Length::Px(320.0)
                gap: SPACE_XS
                EuvButton(DATEPICKER_TRIGGER_LABEL.to_string(), tokens) {
                    outline()
                    sm()
                    on_click: move |_: &UiEvent| {
                        let _: bool = open.update(|v: &mut bool| *v = !*v);
                    }
                }
                text {
                    font_size: 12.0
                    color: tokens.get_palette().get_muted_foreground()
                    { move || format!("Selected date: {lbl_clone} {}", selected_day.get()) }
                }
                view {
                    node_ref: cal_wrap
                    bind_style: move || {
                        Style::default()
                        .width(Length::Percent(100.0))
                        .display(if open.get() {
                                Display::Flex
                            } else {
                                Display::None
                        })
                    }
                    EuvCalendar(month_label, 30, selected_day, tokens)
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
                }
                column {
                    flex_grow: 1.0
                    min_width: Length::Px(40.0)
                    EuvInput(value, placeholder, tokens) {
                        borderless: true
                    }
                }
                if !suffix.is_empty() {
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
        view! {
            row {
                height: Length::Px(g.get_control_md())
                padding: Edges::symmetric(0.0, SPACE_MD)
                align: AlignItems::Center
                background: p.get_card()
                border: (g.get_border_width(), p.get_border())
                radius: g.get_radius_md()
                cursor: CursorIcon::Pointer
                on_click: move |_: &UiEvent| {
                    let _: bool = selected.update(|v: &mut usize| *v = (*v + 1) % total);
                }
                text {
                    font_size: 13.0
                    color: p.get_foreground()
                    { move || {
                            let idx: usize = selected.get() % total;
                            let lbl: String = options
                            .get(idx)
                            .cloned()
                            .unwrap_or_else(|| NATIVE_SELECT_DEFAULT_LABEL.to_string());
                            format!("{lbl}  <>")
                    } }
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
                        EuvButton(label, tokens) {
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
                    }
                    text {
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
                                    EuvButton(SONNER_DISMISS_LABEL.to_string(), tokens) {
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
                    gap: 2.0
                    text {
                        font_size: 13.0
                        font_weight: FontWeight::Medium
                        color: p.get_foreground()
                        { filename }
                    }
                    text {
                        font_size: 11.5
                        color: p.get_muted_foreground()
                        { meta }
                    }
                }
                EuvButton(ATTACHMENT_REMOVE_LABEL.to_string(), tokens) {
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
                column {
                    max_width: Length::Percent(86.0)
                    padding: Edges::symmetric(SPACE_SM, SPACE_MD)
                    gap: 4.0
                    background: if outgoing { p.get_primary() } else { p.get_muted() }
                    border: (
                        if outgoing { 0.0 } else { g.get_border_width() },
                        p.get_border(),
                    )
                    radius: g.get_radius_lg()
                    text {
                        font_size: 11.0
                        font_weight: FontWeight::SemiBold
                        color: meta_fg
                        { header_line }
                    }
                    text {
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
                radius: 999.0
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
                text {
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
