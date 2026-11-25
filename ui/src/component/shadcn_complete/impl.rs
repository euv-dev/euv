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
        let query: Signal<String> = self.get_query();
        let selected: Signal<usize> = self.get_selected();
        let open: Signal<bool> = self.get_open();
        let Self {
            placeholder,
            options,
            ..
        } = self;
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
                    .unwrap_or_else(|| "Select option...".to_string());
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
                .stop_scroll_propagation(true),
        );
        let search: NodeId = EuvInput::new(query, placeholder, tokens).mount(ctx);
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

/// `Mountable` implementation for `EuvDataTable`.
impl Mountable for EuvDataTable {
    /// Mounts a filterable, sortable, selectable production data table.
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
        let filter: Signal<String> = self.get_filter();
        let sort_asc: Signal<bool> = self.get_sort_ascending();
        let sel_row: Signal<Option<usize>> = self.get_selected_row();
        let Self { headers, rows, .. } = self;
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
            EuvInput::new(filter, "Filter table rows...".to_string(), tokens).mount(ctx);
        let sort_btn: NodeId = EuvButton::new("Toggle Sort (A-Z / Z-A)".to_string(), tokens)
            .outline()
            .sm()
            .on_click(move |_: &UiEvent| {
                let _: bool = sort_asc.update(|v: &mut bool| *v = !*v);
            })
            .mount(ctx);
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
                    .gap(SPACE_SM),
            );
            for h in headers.iter() {
                let cell: NodeId = bctx.text(
                    h.clone(),
                    Style::empty()
                        .flex_grow(1.0)
                        .font_size(12.0)
                        .font_weight(FontWeight::SemiBold)
                        .color(p.get_muted_foreground()),
                );
                bctx.append(head_row, cell);
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
                        .cursor(CursorIcon::Pointer),
                );
                for cell_txt in r.into_iter() {
                    let cell: NodeId = bctx.text(
                        cell_txt,
                        Style::empty()
                            .flex_grow(1.0)
                            .font_size(13.0)
                            .color(p.get_foreground()),
                    );
                    bctx.append(row_node, cell);
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

/// `Mountable` implementation for `EuvInputGroup`.
impl Mountable for EuvInputGroup {
    /// Mounts a unified prefix + input + suffix control group.
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
        let inp_wrap: NodeId = ctx.view(
            Style::empty()
                .column()
                .flex_grow(1.0)
                .min_width(Length::Px(40.0)),
        );
        let inp: NodeId = EuvInput::new(value, placeholder, tokens).mount(ctx);
        ctx.append(inp_wrap, inp);
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
        ctx.append(row, pre);
        ctx.append(row, inp_wrap);
        ctx.append(row, suf);
        row
    }
}

/// Builder helpers for `EuvItem`.
impl EuvItem {
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
    /// Mounts a structured shadcn Item row with leading monogram, title/description, and trailing badge.
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
                .cursor(CursorIcon::Pointer),
        );
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
        let body: NodeId = ctx.view(Style::empty().column().flex_grow(1.0).gap(2.0));
        let t_node: NodeId = ctx.text(
            title,
            Style::empty()
                .font_size(13.0)
                .font_weight(FontWeight::SemiBold)
                .color(p.get_foreground()),
        );
        let d_node: NodeId = ctx.text(
            description,
            Style::empty()
                .font_size(12.0)
                .color(p.get_muted_foreground()),
        );
        ctx.append(body, t_node);
        ctx.append(body, d_node);
        let trail: NodeId = ctx.text(
            trailing,
            Style::empty()
                .font_size(12.0)
                .font_weight(FontWeight::Medium)
                .color(p.get_muted_foreground()),
        );
        ctx.append(row, badge);
        ctx.append(row, body);
        ctx.append(row, trail);
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
                        .shadow(Shadow::new(0.0, 4.0, 12.0, 0.0, Color::rgba(0, 0, 0, 24))),
                );
                let col: NodeId = bctx.view(Style::empty().column().flex_grow(1.0).gap(2.0));
                let t: NodeId = bctx.text(
                    title,
                    Style::empty()
                        .font_size(13.0)
                        .font_weight(FontWeight::SemiBold)
                        .color(p.get_card_foreground()),
                );
                let d: NodeId = bctx.text(
                    desc,
                    Style::empty()
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
