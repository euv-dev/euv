use super::*;

/// `Mountable` implementation for `EuvChart`.
impl Mountable for EuvChart {
    /// Mounts a vector-drawn bar, line, or area chart using theme colors and horizontal gridlines.
    ///
    /// # Arguments
    ///
    /// - `Self` - Chart props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted chart card.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let kind: EuvChartKind = self.get_kind();
        let Self { title, data, .. } = self;
        let card: NodeId = ctx.view(
            Style::empty()
                .column()
                .width(Length::Percent(100.0))
                .padding(Edges::all(SPACE_LG))
                .gap(SPACE_SM)
                .background(p.get_card())
                .border(g.get_border_width(), p.get_border())
                .radius(g.get_radius_lg()),
        );
        let head: NodeId = ctx.text(
            title,
            Style::empty()
                .font_size(14.0)
                .font_weight(FontWeight::SemiBold)
                .color(p.get_card_foreground()),
        );
        let plot_data: Vec<EuvChartPoint> = data.clone();
        let bar_color: Color = p.get_primary();
        let grid_color: Color = p.get_border();
        let label_color: Color = p.get_muted_foreground();
        let radius_sm: f32 = g.get_radius_sm();
        let canvas: NodeId = ctx.canvas(
            Style::empty()
                .width(Length::Percent(100.0))
                .height(Length::Px(150.0)),
            move |bounds: Rect, dl: &mut DisplayList| {
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
                        rect: Rect::new(cx - slot_w * 0.45, base_y + 6.0, slot_w * 0.9, 14.0),
                        text: pt.get_label().clone(),
                        style: lbl_st,
                    });
                }
            },
        );
        ctx.append(card, head);
        ctx.append(card, canvas);
        card
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

/// `Mountable` implementation for `EuvCombobox`.
impl Mountable for EuvCombobox {
    /// Mounts a searchable dropdown combobox with live query filtering and unified overlay stack dismissal.
    ///
    /// # Arguments
    ///
    /// - `Self` - Combobox props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted combobox root.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
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
        } = self;
        let query: Signal<String> = query_opt.unwrap_or_else(|| Signal::new(String::new()));
        let resolved_ph: String = if placeholder.is_empty() {
            COMBOBOX_DEFAULT_PLACEHOLDER.to_string()
        } else {
            placeholder
        };
        let wrap: NodeId = ctx.view(
            Style::empty()
                .column()
                .width(Length::Percent(100.0))
                .gap(SPACE_XS),
        );
        let opts_trigger: Vec<String> = options.clone();
        let trigger: NodeId = ctx.view(
            Style::empty()
                .row()
                .width(Length::Percent(100.0))
                .height(Length::Px(g.get_control_md()))
                .padding(Edges::symmetric(0.0, SPACE_MD))
                .justify(JustifyContent::SpaceBetween)
                .align(AlignItems::Center)
                .background(p.get_card())
                .border(g.get_border_width(), p.get_border())
                .radius(g.get_radius_md())
                .cursor(CursorIcon::Pointer),
        );
        let trigger_txt: NodeId = ctx.reactive_text(
            Style::empty().font_size(13.0).color(p.get_foreground()),
            move || {
                let idx: usize = selected.get();
                let lbl: String = opts_trigger
                    .get(idx)
                    .cloned()
                    .unwrap_or_else(|| COMBOBOX_EMPTY_SELECTION.to_string());
                format!("{lbl}  v")
            },
        );
        ctx.append(trigger, trigger_txt);
        ctx.on_click(trigger, move |_: &UiEvent| {
            let _: bool = open.update(|v: &mut bool| *v = !*v);
        });
        let panel: NodeId = ctx.view(
            Style::empty()
                .column()
                .width(Length::Percent(100.0))
                .padding(Edges::all(SPACE_SM))
                .gap(SPACE_XS)
                .background(p.get_card())
                .border(g.get_border_width(), p.get_border())
                .radius(g.get_radius_md())
                .shadow(g.get_shadow_md())
                .stop_scroll_propagation(true),
        );
        let search: NodeId = EuvInput::new(query, resolved_ph, tokens).mount(ctx);
        let list_host: NodeId = ctx.view(
            Style::empty()
                .column()
                .width(Length::Percent(100.0))
                .gap(2.0),
        );
        let opts_filter: Vec<String> = options;
        ctx.bind_children(list_host, move || {
            let q: String = query.get().to_ascii_lowercase();
            let cur_sel: usize = selected.get();
            let mut nodes: Vec<NodeId> = Vec::new();
            let mut bctx: BuildContext = current_build_context();
            for (idx, opt) in opts_filter.iter().enumerate() {
                if !q.is_empty() && !opt.to_ascii_lowercase().contains(&q) {
                    continue;
                }
                let is_sel: bool = idx == cur_sel;
                let row: NodeId = bctx.view(
                    Style::empty()
                        .row()
                        .width(Length::Percent(100.0))
                        .height(Length::Px(g.get_control_sm()))
                        .padding(Edges::symmetric(0.0, SPACE_SM))
                        .align(AlignItems::Center)
                        .radius(g.get_radius_sm())
                        .background(if is_sel {
                            p.get_accent()
                        } else {
                            COLOR_TRANSPARENT
                        })
                        .cursor(CursorIcon::Pointer),
                );
                let txt: NodeId = bctx.text(
                    if is_sel {
                        format!("* {opt}")
                    } else {
                        opt.clone()
                    },
                    Style::empty().font_size(13.0).color(p.get_foreground()),
                );
                bctx.append(row, txt);
                bctx.on_click(row, move |_: &UiEvent| {
                    let _: bool = selected.set_if_changed(idx);
                    let _: bool = open.set_if_changed(false);
                });
                nodes.push(row);
            }
            nodes
        });
        ctx.append(panel, search);
        ctx.append(panel, list_host);
        let handle: SceneHandle = ctx.get_scene().clone();
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            let _: Effect = EuvOverlayStack::bind_signal(open, "EuvCombobox", Z_INDEX_DRAWER - 10);
            Effect::new(move || {
                let vis: Display = if open.get() {
                    Display::Flex
                } else {
                    Display::None
                };
                handle.with_write(
                    |tree: &mut SceneTree| {
                        if let Some(n) = tree.get_node_mut(panel) {
                            let mut st: Style = n.get_style();
                            st.get_mut_layout().set_display(vis);
                            n.set_style(st);
                        }
                        tree.mark_dirty(panel, DIRTY_LAYOUT | DIRTY_PAINT);
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
        ctx.append(wrap, trigger);
        ctx.append(wrap, panel);
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

/// `Mountable` implementation for `EuvDataTable`.
impl Mountable for EuvDataTable {
    /// Mounts a filterable, sortable, selectable production data table with overflow-safe cells.
    ///
    /// # Arguments
    ///
    /// - `Self` - Data table props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted data table container.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
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
        } = self;
        let filter: Signal<String> = filter_opt.unwrap_or_else(|| Signal::new(String::new()));
        let sort_asc: Signal<bool> = sort_opt.unwrap_or_else(|| Signal::new(true));
        let sel_row: Signal<Option<usize>> = sel_opt.unwrap_or_else(|| Signal::new(None));
        let wrap: NodeId = ctx.view(
            Style::empty()
                .column()
                .width(Length::Percent(100.0))
                .gap(SPACE_SM),
        );
        let toolbar: NodeId = ctx.view(
            Style::empty()
                .row()
                .wrap(FlexWrap::Wrap)
                .width(Length::Percent(100.0))
                .justify(JustifyContent::SpaceBetween)
                .align(AlignItems::Center)
                .gap(SPACE_SM),
        );
        let search_box: NodeId =
            EuvInput::new(filter, DATATABLE_FILTER_PLACEHOLDER.to_string(), tokens).mount(ctx);
        let sort_btn: NodeId = EuvButton::new(DATATABLE_SORT_LABEL.to_string(), tokens)
            .outline()
            .sm()
            .on_click(move |_: &UiEvent| {
                let _: bool = sort_asc.update(|v: &mut bool| *v = !*v);
            })
            .mount(ctx);
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
        ctx.append(toolbar, search_box);
        ctx.append(toolbar, sort_btn);
        let body_host: NodeId = ctx.view(
            Style::empty()
                .column()
                .width(Length::Percent(100.0))
                .border(g.get_border_width(), p.get_border())
                .radius(g.get_radius_md())
                .overflow(Overflow::Hidden),
        );
        ctx.bind_children(body_host, move || {
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
            let mut out: Vec<NodeId> = Vec::new();
            let mut bctx: BuildContext = current_build_context();
            let head_row: NodeId = bctx.view(
                Style::empty()
                    .row()
                    .width(Length::Percent(100.0))
                    .padding(Edges::symmetric(SPACE_SM, SPACE_MD))
                    .background(p.get_muted())
                    .gap(SPACE_SM)
                    .overflow(Overflow::Hidden),
            );
            for h in headers.iter() {
                let cell_wrap: NodeId = bctx.view(
                    Style::empty()
                        .flex_grow(1.0)
                        .min_width(Length::Px(0.0))
                        .overflow(Overflow::Hidden),
                );
                let cell: NodeId = bctx.text(
                    h.clone(),
                    Style::empty()
                        .width(Length::Percent(100.0))
                        .font_size(12.0)
                        .font_weight(FontWeight::SemiBold)
                        .color(p.get_muted_foreground())
                        .text_overflow(TextOverflow::Ellipsis),
                );
                bctx.append(cell_wrap, cell);
                bctx.append(head_row, cell_wrap);
            }
            out.push(head_row);
            for (orig_idx, r) in filtered.into_iter() {
                let is_sel: bool = active_sel == Some(orig_idx);
                let row_node: NodeId = bctx.view(
                    Style::empty()
                        .row()
                        .width(Length::Percent(100.0))
                        .padding(Edges::symmetric(SPACE_SM, SPACE_MD))
                        .background(if is_sel { p.get_accent() } else { p.get_card() })
                        .gap(SPACE_SM)
                        .overflow(Overflow::Hidden)
                        .cursor(CursorIcon::Pointer),
                );
                for cell_txt in r.into_iter() {
                    let cell_wrap: NodeId = bctx.view(
                        Style::empty()
                            .flex_grow(1.0)
                            .min_width(Length::Px(0.0))
                            .overflow(Overflow::Hidden),
                    );
                    let cell: NodeId = bctx.text(
                        cell_txt,
                        Style::empty()
                            .width(Length::Percent(100.0))
                            .font_size(13.0)
                            .color(p.get_foreground())
                            .text_overflow(TextOverflow::Ellipsis),
                    );
                    bctx.append(cell_wrap, cell);
                    bctx.append(row_node, cell_wrap);
                }
                bctx.on_click(row_node, move |_: &UiEvent| {
                    let _: bool = sel_row.set_if_changed(Some(orig_idx));
                });
                out.push(row_node);
            }
            out
        });
        ctx.append(wrap, toolbar);
        ctx.append(wrap, body_host);
        wrap
    }
}

/// `Mountable` implementation for `EuvDatePicker`.
impl Mountable for EuvDatePicker {
    /// Mounts a trigger button toggling an inline `EuvCalendar` picker registered in `EuvOverlayStack`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Date picker props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted date picker container.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let tokens: ThemeTokens = self.get_tokens();
        let selected_day: Signal<u32> = self.get_selected_day();
        let open: Signal<bool> = self.get_open();
        let Self { month_label, .. } = self;
        let wrap: NodeId = ctx.view(
            Style::empty()
                .column()
                .width(Length::Percent(100.0))
                .gap(SPACE_XS),
        );
        let lbl_clone: String = month_label.clone();
        let trigger: NodeId = EuvButton::new("Pick Date".to_string(), tokens)
            .outline()
            .sm()
            .on_click(move |_: &UiEvent| {
                let _: bool = open.update(|v: &mut bool| *v = !*v);
            })
            .mount(ctx);
        let summary: NodeId = ctx.reactive_text(
            Style::empty()
                .font_size(12.0)
                .color(tokens.get_palette().get_muted_foreground()),
            move || format!("Selected date: {lbl_clone} {}", selected_day.get()),
        );
        let cal_wrap: NodeId = ctx.view(Style::empty().width(Length::Percent(100.0)));
        let cal_node: NodeId = EuvCalendar::new(month_label, 30, selected_day, tokens).mount(ctx);
        ctx.append(cal_wrap, cal_node);
        let handle: SceneHandle = ctx.get_scene().clone();
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            let _: Effect =
                EuvOverlayStack::bind_signal(open, "EuvDatePicker", Z_INDEX_DRAWER - 10);
            Effect::new(move || {
                let vis: Display = if open.get() {
                    Display::Flex
                } else {
                    Display::None
                };
                handle.with_write(
                    |tree: &mut SceneTree| {
                        if let Some(n) = tree.get_node_mut(cal_wrap) {
                            let mut st: Style = n.get_style();
                            st.get_mut_layout().set_display(vis);
                            n.set_style(st);
                        }
                        tree.mark_dirty(cal_wrap, DIRTY_LAYOUT | DIRTY_PAINT);
                    },
                    (),
                );
            })
        });
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                tree.attach_scope(cal_wrap, scope);
            },
            (),
        );
        ctx.append(wrap, trigger);
        ctx.append(wrap, summary);
        ctx.append(wrap, cal_wrap);
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

/// `Mountable` implementation for `EuvInputGroup`.
impl Mountable for EuvInputGroup {
    /// Mounts a unified prefix + borderless input + suffix control group with zero double borders.
    ///
    /// # Arguments
    ///
    /// - `Self` - Input group props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted input group node.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let tokens: ThemeTokens = self.get_tokens();
        let p: ThemePalette = tokens.get_palette();
        let g: ThemeGeometry = tokens.get_geometry();
        let value: Signal<String> = self.get_value();
        let Self {
            prefix,
            placeholder,
            suffix,
            ..
        } = self;
        let row: NodeId = ctx.view(
            Style::empty()
                .row()
                .width(Length::Percent(100.0))
                .align(AlignItems::Center)
                .background(p.get_card())
                .border(g.get_border_width(), p.get_border())
                .radius(g.get_radius_md())
                .overflow(Overflow::Hidden),
        );
        if !prefix.is_empty() {
            let pre: NodeId = ctx.view(
                Style::empty()
                    .row()
                    .height(Length::Px(g.get_control_md()))
                    .padding(Edges::symmetric(0.0, SPACE_SM))
                    .align(AlignItems::Center)
                    .background(p.get_muted()),
            );
            let pre_txt: NodeId = ctx.text(
                prefix,
                Style::empty()
                    .font_size(12.0)
                    .color(p.get_muted_foreground()),
            );
            ctx.append(pre, pre_txt);
            ctx.append(row, pre);
        }
        let inp_wrap: NodeId = ctx.view(
            Style::empty()
                .column()
                .flex_grow(1.0)
                .min_width(Length::Px(40.0)),
        );
        let inp: NodeId = EuvInput::new(value, placeholder, tokens)
            .borderless(true)
            .mount(ctx);
        ctx.append(inp_wrap, inp);
        ctx.append(row, inp_wrap);
        if !suffix.is_empty() {
            let suf: NodeId = ctx.view(
                Style::empty()
                    .row()
                    .height(Length::Px(g.get_control_md()))
                    .padding(Edges::symmetric(0.0, SPACE_SM))
                    .align(AlignItems::Center)
                    .background(p.get_muted()),
            );
            let suf_txt: NodeId = ctx.text(
                suffix,
                Style::empty()
                    .font_size(12.0)
                    .color(p.get_muted_foreground()),
            );
            ctx.append(suf, suf_txt);
            ctx.append(row, suf);
        }
        row
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

/// `Mountable` implementation for `EuvItem`.
impl Mountable for EuvItem {
    /// Mounts a structured shadcn Item row with optional leading monogram, title/description, and trailing badge.
    ///
    /// # Arguments
    ///
    /// - `Self` - Item props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted item row.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let Self {
            leading,
            title,
            description,
            trailing,
            on_click,
            ..
        } = self;
        let row: NodeId = ctx.view(
            Style::empty()
                .row()
                .width(Length::Percent(100.0))
                .padding(Edges::symmetric(SPACE_SM, SPACE_MD))
                .gap(SPACE_MD)
                .align(AlignItems::Center)
                .background(p.get_card())
                .border(g.get_border_width(), p.get_border())
                .radius(g.get_radius_md())
                .overflow(Overflow::Hidden)
                .cursor(CursorIcon::Pointer),
        );
        if !leading.is_empty() {
            let badge: NodeId = ctx.view(
                Style::empty()
                    .row()
                    .width(Length::Px(32.0))
                    .height(Length::Px(32.0))
                    .justify(JustifyContent::Center)
                    .align(AlignItems::Center)
                    .background(p.get_secondary())
                    .radius(g.get_radius_md()),
            );
            let badge_txt: NodeId = ctx.text(
                leading,
                Style::empty()
                    .font_size(12.0)
                    .font_weight(FontWeight::Bold)
                    .color(p.get_secondary_foreground()),
            );
            ctx.append(badge, badge_txt);
            ctx.append(row, badge);
        }
        let body: NodeId = ctx.view(
            Style::empty()
                .column()
                .flex_grow(1.0)
                .min_width(Length::Px(0.0))
                .gap(2.0)
                .overflow(Overflow::Hidden),
        );
        let t_node: NodeId = ctx.text(
            title,
            Style::empty()
                .width(Length::Percent(100.0))
                .font_size(13.0)
                .font_weight(FontWeight::SemiBold)
                .color(p.get_foreground()),
        );
        ctx.append(body, t_node);
        if !description.is_empty() {
            let d_node: NodeId = ctx.text(
                description,
                Style::empty()
                    .width(Length::Percent(100.0))
                    .font_size(12.0)
                    .color(p.get_muted_foreground()),
            );
            ctx.append(body, d_node);
        }
        ctx.append(row, body);
        if !trailing.is_empty() {
            let trail: NodeId = ctx.text(
                trailing,
                Style::empty()
                    .font_size(12.0)
                    .font_weight(FontWeight::Medium)
                    .color(p.get_muted_foreground()),
            );
            ctx.append(row, trail);
        }
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

/// `Mountable` implementation for `EuvNativeSelect`.
impl Mountable for EuvNativeSelect {
    /// Mounts a compact single-tap native selector cycling options deterministically.
    ///
    /// # Arguments
    ///
    /// - `Self` - Native select props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted native select control.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let selected: Signal<usize> = self.get_selected();
        let Self { options, .. } = self;
        let total: usize = options.len().max(1);
        let box_node: NodeId = ctx.view(
            Style::empty()
                .row()
                .height(Length::Px(g.get_control_md()))
                .padding(Edges::symmetric(0.0, SPACE_MD))
                .align(AlignItems::Center)
                .background(p.get_card())
                .border(g.get_border_width(), p.get_border())
                .radius(g.get_radius_md())
                .cursor(CursorIcon::Pointer),
        );
        let txt: NodeId = ctx.reactive_text(
            Style::empty().font_size(13.0).color(p.get_foreground()),
            move || {
                let idx: usize = selected.get() % total;
                let lbl: String = options
                    .get(idx)
                    .cloned()
                    .unwrap_or_else(|| "Default".to_string());
                format!("{lbl}  <>")
            },
        );
        ctx.append(box_node, txt);
        ctx.on_click(box_node, move |_: &UiEvent| {
            let _: bool = selected.update(|v: &mut usize| *v = (*v + 1) % total);
        });
        box_node
    }
}

/// `Mountable` implementation for `EuvNavigationMenu`.
impl Mountable for EuvNavigationMenu {
    /// Mounts a top-level horizontal navigation menu with expandable panel detail.
    ///
    /// # Arguments
    ///
    /// - `Self` - Navigation menu props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted navigation menu container.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let tokens: ThemeTokens = self.get_tokens();
        let p: ThemePalette = tokens.get_palette();
        let g: ThemeGeometry = tokens.get_geometry();
        let active: Signal<Option<usize>> = self.get_active();
        let Self { items, .. } = self;
        let wrap: NodeId = ctx.view(
            Style::empty()
                .column()
                .width(Length::Percent(100.0))
                .gap(SPACE_XS),
        );
        let bar: NodeId = ctx.view(
            Style::empty()
                .row()
                .wrap(FlexWrap::Wrap)
                .width(Length::Percent(100.0))
                .gap(SPACE_XS),
        );
        let summaries: Vec<String> = items
            .iter()
            .map(|(_, s): &(String, String)| s.clone())
            .collect();
        for (idx, (label, _)) in items.into_iter().enumerate() {
            let btn: NodeId = EuvButton::new(label, tokens)
                .ghost()
                .sm()
                .on_click(move |_: &UiEvent| {
                    let cur: Option<usize> = active.get_untracked();
                    let next: Option<usize> = if cur == Some(idx) { None } else { Some(idx) };
                    let _: bool = active.set_if_changed(next);
                })
                .mount(ctx);
            ctx.append(bar, btn);
        }
        let panel: NodeId = ctx.view(
            Style::empty()
                .column()
                .width(Length::Percent(100.0))
                .padding(Edges::all(SPACE_MD))
                .background(p.get_card())
                .border(g.get_border_width(), p.get_border())
                .radius(g.get_radius_md()),
        );
        let panel_txt: NodeId = ctx.reactive_text(
            Style::empty()
                .font_size(13.0)
                .color(p.get_muted_foreground()),
            move || {
                active
                    .get()
                    .and_then(|i: usize| summaries.get(i).cloned())
                    .unwrap_or_default()
            },
        );
        ctx.append(panel, panel_txt);
        let handle: SceneHandle = ctx.get_scene().clone();
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            Effect::new(move || {
                let vis: Display = if active.get().is_some() {
                    Display::Flex
                } else {
                    Display::None
                };
                handle.with_write(
                    |tree: &mut SceneTree| {
                        if let Some(n) = tree.get_node_mut(panel) {
                            let mut st: Style = n.get_style();
                            st.get_mut_layout().set_display(vis);
                            n.set_style(st);
                        }
                        tree.mark_dirty(panel, DIRTY_LAYOUT | DIRTY_PAINT);
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
        ctx.append(wrap, bar);
        ctx.append(wrap, panel);
        wrap
    }
}

/// `Mountable` implementation for `EuvResizable`.
impl Mountable for EuvResizable {
    /// Mounts a draggable split-pane container with pointer-captured handle resizing.
    ///
    /// # Arguments
    ///
    /// - `Self` - Resizable panel group props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted resizable split container.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let ratio: Signal<f32> = self.get_split_ratio();
        let container: NodeId = ctx.view(
            Style::empty()
                .row()
                .width(Length::Percent(100.0))
                .height(Length::Px(132.0))
                .background(p.get_card())
                .border(g.get_border_width(), p.get_border())
                .radius(g.get_radius_md())
                .overflow(Overflow::Hidden),
        );
        let left_pane: NodeId = ctx.view(
            Style::empty()
                .column()
                .height(Length::Percent(100.0))
                .padding(Edges::all(SPACE_MD))
                .overflow(Overflow::Hidden),
        );
        let grip: NodeId = ctx.view(
            Style::empty()
                .width(Length::Px(8.0))
                .height(Length::Percent(100.0))
                .background(p.get_border())
                .cursor(CursorIcon::EwResize),
        );
        let right_pane: NodeId = ctx.view(
            Style::empty()
                .column()
                .flex_grow(1.0)
                .height(Length::Percent(100.0))
                .padding(Edges::all(SPACE_MD))
                .overflow(Overflow::Hidden),
        );
        ctx.append(left_pane, self.get_left());
        ctx.append(right_pane, self.get_right());
        ctx.append(container, left_pane);
        ctx.append(container, grip);
        ctx.append(container, right_pane);
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
        let handle: SceneHandle = ctx.get_scene().clone();
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            Effect::new(move || {
                let pct: f32 = (ratio.get().clamp(0.25, 0.75)) * 100.0;
                handle.with_write(
                    |tree: &mut SceneTree| {
                        if let Some(n) = tree.get_node_mut(left_pane) {
                            let mut st: Style = n.get_style();
                            st.get_mut_layout().set_width(Length::Percent(pct));
                            n.set_style(st);
                        }
                        tree.mark_dirty(container, DIRTY_LAYOUT | DIRTY_PAINT);
                    },
                    (),
                );
            })
        });
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                tree.attach_scope(container, scope);
            },
            (),
        );
        container
    }
}

/// `Mountable` implementation for `EuvSeparator`.
impl Mountable for EuvSeparator {
    /// Mounts a horizontal hairline separator with optional centered text caption.
    ///
    /// # Arguments
    ///
    /// - `Self` - Separator props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted separator node.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let Self { label, .. } = self;
        if label.is_empty() {
            return ctx.view(
                Style::empty()
                    .width(Length::Percent(100.0))
                    .height(Length::Px(1.0))
                    .background(p.get_border()),
            );
        }
        let row: NodeId = ctx.view(
            Style::empty()
                .row()
                .width(Length::Percent(100.0))
                .align(AlignItems::Center)
                .gap(SPACE_SM),
        );
        let left_line: NodeId = ctx.view(
            Style::empty()
                .flex_grow(1.0)
                .height(Length::Px(1.0))
                .background(p.get_border()),
        );
        let txt: NodeId = ctx.text(
            label,
            Style::empty()
                .font_size(11.0)
                .color(p.get_muted_foreground()),
        );
        let right_line: NodeId = ctx.view(
            Style::empty()
                .flex_grow(1.0)
                .height(Length::Px(1.0))
                .background(p.get_border()),
        );
        ctx.append(row, left_line);
        ctx.append(row, txt);
        ctx.append(row, right_line);
        row
    }
}

/// `Mountable` implementation for `EuvSonner`.
impl Mountable for EuvSonner {
    /// Mounts a stacking toast deck rendering live notifications with individual dismiss buttons.
    ///
    /// # Arguments
    ///
    /// - `Self` - Sonner props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted toast stack container.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let tokens: ThemeTokens = self.get_tokens();
        let p: ThemePalette = tokens.get_palette();
        let g: ThemeGeometry = tokens.get_geometry();
        let messages: Signal<Vec<(String, String)>> = self.get_messages();
        let deck: NodeId = ctx.view(
            Style::empty()
                .column()
                .width(Length::Percent(100.0))
                .gap(SPACE_XS),
        );
        ctx.bind_children(deck, move || {
            let list: Vec<(String, String)> = messages.get();
            let mut out: Vec<NodeId> = Vec::new();
            let mut bctx: BuildContext = current_build_context();
            for (idx, (title, desc)) in list.into_iter().enumerate() {
                let toast: NodeId = bctx.view(
                    Style::empty()
                        .row()
                        .width(Length::Percent(100.0))
                        .padding(Edges::symmetric(SPACE_SM, SPACE_MD))
                        .justify(JustifyContent::SpaceBetween)
                        .align(AlignItems::Center)
                        .gap(SPACE_SM)
                        .background(p.get_card())
                        .border(g.get_border_width(), p.get_border())
                        .radius(g.get_radius_md())
                        .shadow(g.get_shadow_md())
                        .overflow(Overflow::Hidden),
                );
                let col: NodeId = bctx.view(
                    Style::empty()
                        .column()
                        .flex_grow(1.0)
                        .gap(2.0)
                        .overflow(Overflow::Hidden),
                );
                let t: NodeId = bctx.text(
                    title,
                    Style::empty()
                        .width(Length::Percent(100.0))
                        .font_size(13.0)
                        .font_weight(FontWeight::SemiBold)
                        .color(p.get_card_foreground()),
                );
                let d: NodeId = bctx.text(
                    desc,
                    Style::empty()
                        .width(Length::Percent(100.0))
                        .font_size(12.0)
                        .color(p.get_muted_foreground()),
                );
                bctx.append(col, t);
                bctx.append(col, d);
                let dismiss: NodeId = EuvButton::new("Dismiss".to_string(), tokens)
                    .ghost()
                    .sm()
                    .on_click(move |_: &UiEvent| {
                        let _: bool = messages.update(|v: &mut Vec<(String, String)>| {
                            if idx < v.len() {
                                let _: (String, String) = v.remove(idx);
                            }
                        });
                    })
                    .mount(&mut bctx);
                bctx.append(toast, col);
                bctx.append(toast, dismiss);
                out.push(toast);
            }
            out
        });
        deck
    }
}

/// `Mountable` implementation for `EuvDirection`.
impl Mountable for EuvDirection {
    /// Mounts a direction-aware wrapper aligning content start/end cleanly for LTR or RTL locales.
    ///
    /// # Arguments
    ///
    /// - `Self` - Direction props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted direction container.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let align: AlignItems = match self.get_direction() {
            EuvTextDirection::Ltr => AlignItems::Start,
            EuvTextDirection::Rtl => AlignItems::End,
        };
        let wrap: NodeId = ctx.view(
            Style::empty()
                .column()
                .width(Length::Percent(100.0))
                .align(align),
        );
        ctx.append(wrap, self.get_child());
        wrap
    }
}

/// `Mountable` implementation for `EuvTrace`.
impl Mountable for EuvTrace {
    /// Mounts a shadcn execution waterfall trace card with proportional timing bars.
    ///
    /// # Arguments
    ///
    /// - `Self` - Trace props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted trace container handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let Self { title, spans, .. } = self;
        let card: NodeId = ctx.view(
            Style::empty()
                .column()
                .width(Length::Percent(100.0))
                .padding(Edges::all(SPACE_MD))
                .gap(SPACE_SM)
                .background(p.get_card())
                .border(g.get_border_width(), p.get_border())
                .radius(g.get_radius_md()),
        );
        let head: NodeId = ctx.text(
            title,
            Style::empty()
                .font_size(13.5)
                .font_weight(FontWeight::SemiBold)
                .color(p.get_foreground()),
        );
        ctx.append(card, head);
        for span in spans.into_iter() {
            let (bar_bg, _): (Color, Color) = span.get_tone().colors(p);
            let row: NodeId = ctx.view(
                Style::empty()
                    .column()
                    .width(Length::Percent(100.0))
                    .gap(4.0),
            );
            let meta: NodeId = ctx.view(
                Style::empty()
                    .row()
                    .width(Length::Percent(100.0))
                    .justify(JustifyContent::SpaceBetween)
                    .align(AlignItems::Center),
            );
            let name_txt: NodeId = ctx.text(
                span.get_name().clone(),
                Style::empty()
                    .font_size(12.0)
                    .font_weight(FontWeight::Medium)
                    .color(p.get_foreground()),
            );
            let dur_txt: NodeId = ctx.text(
                format!("{:.2}ms", span.get_duration_ms()),
                Style::empty()
                    .font_size(11.5)
                    .color(p.get_muted_foreground()),
            );
            ctx.append(meta, name_txt);
            ctx.append(meta, dur_txt);
            let track: NodeId = ctx.view(
                Style::empty()
                    .row()
                    .width(Length::Percent(100.0))
                    .height(Length::Px(8.0))
                    .background(p.get_muted())
                    .radius(4.0)
                    .overflow(Overflow::Hidden),
            );
            let left_pct: f32 = (span.get_offset_frac().clamp(0.0, 0.9)) * 100.0;
            let bar_pct: f32 = (span
                .get_width_frac()
                .clamp(0.05, 1.0 - span.get_offset_frac()))
                * 100.0;
            if left_pct > 0.5 {
                let spacer: NodeId = ctx.view(
                    Style::empty()
                        .width(Length::Percent(left_pct))
                        .height(Length::Percent(100.0)),
                );
                ctx.append(track, spacer);
            }
            let bar: NodeId = ctx.view(
                Style::empty()
                    .width(Length::Percent(bar_pct))
                    .height(Length::Percent(100.0))
                    .background(bar_bg)
                    .radius(4.0),
            );
            ctx.append(track, bar);
            ctx.append(row, meta);
            ctx.append(row, track);
            ctx.append(card, row);
        }
        card
    }
}

/// `Mountable` implementation for `EuvAttachment`.
impl Mountable for EuvAttachment {
    /// Mounts a dismissible shadcn attachment chip with file metadata.
    ///
    /// # Arguments
    ///
    /// - `Self` - Attachment props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted attachment chip handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let tokens: ThemeTokens = self.get_tokens();
        let p: ThemePalette = tokens.get_palette();
        let g: ThemeGeometry = tokens.get_geometry();
        let visible: Signal<bool> = self.get_visible();
        let Self { filename, meta, .. } = self;
        let host: NodeId = ctx.view(
            Style::empty()
                .row()
                .width(Length::Percent(100.0))
                .justify(JustifyContent::SpaceBetween)
                .align(AlignItems::Center)
                .padding(Edges::symmetric(SPACE_SM, SPACE_MD))
                .gap(SPACE_SM)
                .background(p.get_muted())
                .border(g.get_border_width(), p.get_border())
                .radius(g.get_radius_md()),
        );
        let info: NodeId = ctx.view(Style::empty().column().flex_grow(1.0).gap(2.0));
        let name_n: NodeId = ctx.text(
            filename,
            Style::empty()
                .font_size(13.0)
                .font_weight(FontWeight::Medium)
                .color(p.get_foreground()),
        );
        let meta_n: NodeId = ctx.text(
            meta,
            Style::empty()
                .font_size(11.5)
                .color(p.get_muted_foreground()),
        );
        ctx.append(info, name_n);
        ctx.append(info, meta_n);
        let rm_btn: NodeId = EuvButton::new("Remove".to_string(), tokens)
            .outline()
            .sm()
            .on_click(move |_: &UiEvent| {
                let _: bool = visible.set_if_changed(false);
            })
            .mount(ctx);
        ctx.append(host, info);
        ctx.append(host, rm_btn);
        let handle: SceneHandle = ctx.get_scene().clone();
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            Effect::new(move || {
                let show: bool = visible.get();
                handle.with_write(
                    |tree: &mut SceneTree| {
                        if let Some(node) = tree.get_node_mut(host) {
                            let mut st: Style = node.get_style();
                            st.get_mut_layout().set_display(if show {
                                Display::Flex
                            } else {
                                Display::None
                            });
                            node.set_style(st);
                        }
                        tree.mark_dirty(host, DIRTY_LAYOUT | DIRTY_PAINT);
                    },
                    (),
                );
            })
        });
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                tree.attach_scope(host, scope);
            },
            (),
        );
        host
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

/// `Mountable` implementation for `EuvBubble`.
impl Mountable for EuvBubble {
    /// Mounts a shadcn chat/message bubble with role-aligned surface styling.
    ///
    /// # Arguments
    ///
    /// - `Self` - Bubble props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted message bubble row handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let outgoing: bool = self.get_outgoing();
        let Self {
            author,
            content,
            timestamp,
            ..
        } = self;
        let row: NodeId = ctx.view(Style::empty().row().width(Length::Percent(100.0)).justify(
            if outgoing {
                JustifyContent::End
            } else {
                JustifyContent::Start
            },
        ));
        let card: NodeId = ctx.view(
            Style::empty()
                .column()
                .max_width(Length::Percent(86.0))
                .padding(Edges::symmetric(SPACE_SM, SPACE_MD))
                .gap(4.0)
                .background(if outgoing {
                    p.get_primary()
                } else {
                    p.get_muted()
                })
                .border(
                    if outgoing { 0.0 } else { g.get_border_width() },
                    p.get_border(),
                )
                .radius(g.get_radius_lg()),
        );
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
        let head: NodeId = ctx.text(
            header_line,
            Style::empty()
                .font_size(11.0)
                .font_weight(FontWeight::SemiBold)
                .color(meta_fg),
        );
        let body: NodeId = ctx.text(content, Style::empty().font_size(13.0).color(fg));
        ctx.append(card, head);
        ctx.append(card, body);
        ctx.append(row, card);
        row
    }
}

/// `Mountable` implementation for `EuvMarker`.
impl Mountable for EuvMarker {
    /// Mounts a compact numbered status marker pill.
    ///
    /// # Arguments
    ///
    /// - `Self` - Marker props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted marker row handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let (dot_bg, dot_fg): (Color, Color) = self.get_tone().colors(p);
        let Self { code, label, .. } = self;
        let wrap: NodeId = ctx.view(
            Style::empty()
                .row()
                .align(AlignItems::Center)
                .padding(Edges::symmetric(SPACE_XS, SPACE_SM))
                .gap(SPACE_XS)
                .background(p.get_card())
                .border(g.get_border_width(), p.get_border())
                .radius(999.0),
        );
        let circle: NodeId = ctx.view(
            Style::empty()
                .row()
                .width(Length::Px(20.0))
                .height(Length::Px(20.0))
                .justify(JustifyContent::Center)
                .align(AlignItems::Center)
                .background(dot_bg)
                .radius(10.0),
        );
        let code_txt: NodeId = ctx.text(
            code,
            Style::empty()
                .font_size(10.5)
                .font_weight(FontWeight::Bold)
                .color(dot_fg),
        );
        let lbl_txt: NodeId = ctx.text(
            label,
            Style::empty()
                .font_size(12.0)
                .font_weight(FontWeight::Medium)
                .color(p.get_foreground()),
        );
        ctx.append(circle, code_txt);
        ctx.append(wrap, circle);
        ctx.append(wrap, lbl_txt);
        wrap
    }
}

/// `Mountable` implementation for `EuvQuestionnaire`.
impl Mountable for EuvQuestionnaire {
    /// Mounts an interactive single-choice shadcn questionnaire card.
    ///
    /// # Arguments
    ///
    /// - `Self` - Questionnaire props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted questionnaire card handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let selected: Signal<Option<usize>> = self.get_selected();
        let Self {
            prompt, options, ..
        } = self;
        let card: NodeId = ctx.view(
            Style::empty()
                .column()
                .width(Length::Percent(100.0))
                .padding(Edges::all(SPACE_MD))
                .gap(SPACE_SM)
                .background(p.get_card())
                .border(g.get_border_width(), p.get_border())
                .radius(g.get_radius_md()),
        );
        let q_txt: NodeId = ctx.text(
            prompt,
            Style::empty()
                .font_size(13.5)
                .font_weight(FontWeight::SemiBold)
                .color(p.get_foreground()),
        );
        ctx.append(card, q_txt);
        for (idx, opt) in options.into_iter().enumerate() {
            let item: NodeId = ctx.view(Style::empty());
            let txt: NodeId = ctx.text(
                opt,
                Style::empty()
                    .font_size(13.0)
                    .font_weight(FontWeight::Medium)
                    .color(p.get_foreground()),
            );
            ctx.append(item, txt);
            let handle: SceneHandle = ctx.get_scene().clone();
            let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
                handle.bind_style(item, move || {
                    let is_sel: bool = selected.get() == Some(idx);
                    Style::empty()
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
                })
            });
            ctx.get_scene().with_write(
                |tree: &mut SceneTree| {
                    tree.attach_scope(item, scope);
                },
                (),
            );
            ctx.on_click(item, move |_: &UiEvent| {
                let _: bool = selected.set_if_changed(Some(idx));
            });
            ctx.append(card, item);
        }
        card
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

/// `Mountable` implementation for `EuvContextMenu`.
impl Mountable for EuvContextMenu {
    /// Mounts the dashed context trigger zone and floating action menu registered in `EuvOverlayStack`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Context menu props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted context menu wrapper handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
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
        } = self;
        let open: Signal<bool> = open_opt.unwrap_or_else(|| Signal::new(false));
        let selected: Signal<Option<usize>> = sel_opt.unwrap_or_else(|| Signal::new(None));
        let wrap: NodeId = ctx.view(
            Style::empty()
                .column()
                .width(Length::Percent(100.0))
                .gap(SPACE_XS),
        );
        let trigger: NodeId = ctx.view(
            Style::empty()
                .column()
                .width(Length::Percent(100.0))
                .padding(Edges::symmetric(SPACE_MD, SPACE_LG))
                .justify(JustifyContent::Center)
                .align(AlignItems::Center)
                .gap(SPACE_XS)
                .background(p.get_muted())
                .border(g.get_border_width(), p.get_border())
                .radius(g.get_radius_md())
                .cursor(CursorIcon::Pointer),
        );
        ctx.set_a11y(
            trigger,
            A11yProps::new(A11yRole::Button, trigger_label.clone()),
        );
        let trig_title: NodeId = ctx.text(
            trigger_label,
            Style::empty()
                .font_size(13.0)
                .font_weight(FontWeight::SemiBold)
                .color(p.get_foreground())
                .text_align(TextAlign::Center),
        );
        let trig_hint: NodeId = ctx.text(
            CONTEXT_MENU_HINT_LABEL.to_string(),
            Style::empty()
                .font_size(11.5)
                .color(p.get_muted_foreground())
                .text_align(TextAlign::Center),
        );
        ctx.append(trigger, trig_title);
        ctx.append(trigger, trig_hint);
        ctx.on_click(trigger, move |_: &UiEvent| {
            let _: bool = open.update(|v: &mut bool| *v = !*v);
        });
        let init_open: bool = open.get_untracked();
        let menu: NodeId = ctx.view(
            Style::empty()
                .column()
                .display(if init_open {
                    Display::Flex
                } else {
                    Display::None
                })
                .width(Length::Percent(100.0))
                .max_width(Length::Px(260.0))
                .padding(Edges::all(SPACE_XS))
                .gap(2.0)
                .background(p.get_card())
                .border(g.get_border_width(), p.get_border())
                .radius(g.get_radius_md())
                .shadow(g.get_shadow_md())
                .z_index(Z_INDEX_FLOATING),
        );
        ctx.set_a11y(menu, A11yProps::new(A11yRole::Menu, String::new()));
        for (idx, item_label) in items.into_iter().enumerate() {
            let row: NodeId = ctx.view(
                Style::empty()
                    .row()
                    .width(Length::Percent(100.0))
                    .padding(Edges::symmetric(SPACE_SM, SPACE_MD))
                    .radius(g.get_radius_sm())
                    .cursor(CursorIcon::Pointer),
            );
            ctx.set_a11y(row, A11yProps::new(A11yRole::MenuItem, item_label.clone()));
            let txt: NodeId = ctx.text(
                item_label,
                Style::empty().font_size(13.0).color(p.get_foreground()),
            );
            ctx.append(row, txt);
            ctx.on_click(row, move |_: &UiEvent| {
                let _: bool = selected.set_if_changed(Some(idx));
                let _: bool = open.set_if_changed(false);
            });
            ctx.append(menu, row);
        }
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
                EuvOverlayStack::bind_signal(open, "EuvContextMenu", Z_INDEX_DRAWER - 10);
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
        ctx.append(wrap, trigger);
        ctx.append(wrap, menu);
        wrap
    }
}
