use super::*;

/// Default props for `EuvEChart` (100% width, 320px height, interactive enabled).
impl Default for EuvEChart {
    /// Creates an empty interactive `EuvEChart` ready for `view!` named-prop setters.
    ///
    /// # Returns
    ///
    /// - `Self` - Default chart component.
    fn default() -> Self {
        Self {
            option: EuvChartOption::default(),
            option_signal: None,
            tokens: ThemeTokens::default(),
            width: Length::Percent(100.0),
            height: Length::Px(ECHART_DEFAULT_HEIGHT),
            interactive: true,
            on_point_click: None,
            on_legend_change: None,
            on_zoom_change: None,
        }
    }
}

/// Constructors and fluent prop builders for `EuvEChart`.
impl EuvEChart {
    /// Constructs an interactive `EuvEChart` from a static `EuvChartOption` and `ThemeTokens`.
    ///
    /// # Arguments
    ///
    /// - `EuvChartOption` - Declarative chart option document.
    /// - `ThemeTokens` - Active UI theme tokens.
    ///
    /// # Returns
    ///
    /// - `Self` - Constructed chart component.
    #[must_use]
    pub fn new(option: EuvChartOption, tokens: ThemeTokens) -> Self {
        let mut chart: Self = Self::default();
        chart.set_option(option);
        chart.set_tokens(tokens);
        chart
    }

    /// Constructs a reactive `EuvEChart` driven by a live `Signal<EuvChartOption>`.
    ///
    /// # Arguments
    ///
    /// - `Signal<EuvChartOption>` - Reactive chart option signal.
    /// - `ThemeTokens` - Active UI theme tokens.
    ///
    /// # Returns
    ///
    /// - `Self` - Constructed reactive chart component.
    #[must_use]
    pub fn from_signal(option_signal: Signal<EuvChartOption>, tokens: ThemeTokens) -> Self {
        let mut chart: Self = Self::default();
        chart.set_option_signal(Some(option_signal));
        chart.set_tokens(tokens);
        chart
    }

    /// Sets the declarative chart option fluently (used by `view! { EuvEChart { option: ... } }`).
    ///
    /// # Arguments
    ///
    /// - `Self` - Source chart component.
    /// - `EuvChartOption` - Declarative chart option.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated chart component.
    #[must_use]
    pub fn option(mut self, option: EuvChartOption) -> Self {
        self.set_option(option);
        self
    }

    /// Binds a reactive `Signal<EuvChartOption>` fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source chart component.
    /// - `Signal<EuvChartOption>` - Reactive option signal.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated chart component.
    #[must_use]
    pub fn option_signal(mut self, option_signal: Signal<EuvChartOption>) -> Self {
        self.set_option_signal(Some(option_signal));
        self
    }

    /// Sets the active UI theme tokens fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source chart component.
    /// - `ThemeTokens` - Theme token bundle.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated chart component.
    #[must_use]
    pub fn tokens(mut self, tokens: ThemeTokens) -> Self {
        self.set_tokens(tokens);
        self
    }

    /// Sets the outer component width fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source chart component.
    /// - `Length` - Target width.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated chart component.
    #[must_use]
    pub fn width(mut self, width: Length) -> Self {
        self.set_width(width);
        self
    }

    /// Sets the chart canvas height fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source chart component.
    /// - `Length` - Target height.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated chart component.
    #[must_use]
    pub fn height(mut self, height: Length) -> Self {
        self.set_height(height);
        self
    }

    /// Enables or disables pointer interaction fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source chart component.
    /// - `bool` - Interaction enabled flag.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated chart component.
    #[must_use]
    pub fn interactive(mut self, interactive: bool) -> Self {
        self.set_interactive(interactive);
        self
    }

    /// Registers a datum click callback fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source chart component.
    /// - `EuvChartHitCallback` - Click callback invoked with clicked datum hit.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated chart component.
    #[must_use]
    pub fn on_point_click(mut self, handler: EuvChartHitCallback) -> Self {
        self.set_on_point_click(Some(handler));
        self
    }

    /// Registers a legend toggle callback fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source chart component.
    /// - `EuvChartLegendCallback` - Legend toggle callback `(series_name, visible)`.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated chart component.
    #[must_use]
    pub fn on_legend_change(mut self, handler: EuvChartLegendCallback) -> Self {
        self.set_on_legend_change(Some(handler));
        self
    }

    /// Registers a data-zoom change callback fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source chart component.
    /// - `EuvChartZoomCallback` - Zoom callback `(start_pct, end_pct)`.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated chart component.
    #[must_use]
    pub fn on_zoom_change(mut self, handler: EuvChartZoomCallback) -> Self {
        self.set_on_zoom_change(Some(handler));
        self
    }

    /// Resolves the initial `EuvChartOption` snapshot from `option_signal` or static `option`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component descriptor.
    ///
    /// # Returns
    ///
    /// - `EuvChartOption` - Snapshot option.
    #[must_use]
    fn current_option_untracked(&self) -> EuvChartOption {
        let Self {
            option,
            option_signal,
            ..
        } = self;
        match option_signal {
            Some(sig) => sig.get_untracked(),
            None => option.clone(),
        }
    }
}

/// Declarative `#[component]` implementation for `EuvEChart`.
#[component]
/// Inherent implementation of [`EuvEChart`].
impl EuvEChart {
    /// Mounts `EuvEChart` with cached vector canvas rendering, pointer tooltip/legend/DataZoom handling, compact icon-only Toolbox, DataView table, and Fullscreen portal modal.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Chart component descriptor.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted root scene node handle.
    fn render(&self) -> NodeId {
        let mut ctx: Context = current_context();
        bind_fullscreen_context(&ctx);
        let tokens: ThemeTokens = self.get_tokens();
        let p: ThemePalette = tokens.get_palette();
        let g: ThemeGeometry = tokens.get_geometry();
        let width: Length = self.get_width();
        let height: Length = self.get_height();
        let interactive: bool = self.get_interactive();
        let Self {
            option: static_opt,
            option_signal,
            on_point_click,
            on_legend_change,
            on_zoom_change,
            ..
        } = self;
        let opt_sig_opt: Option<Signal<EuvChartOption>> = *option_signal;
        let static_opt_rc: Rc<EuvChartOption> = Rc::new(static_opt.clone());
        let initial_opt: EuvChartOption = self.current_option_untracked();
        let state_sig: Signal<EuvChartState> =
            Signal::new(EuvChartState::from_option(&initial_opt));
        let fs_open: Signal<bool> = Signal::new(false);
        let opt_rev: Rc<Cell<u64>> = Rc::new(Cell::new(0));
        let last_render: Rc<RefCell<EuvChartRenderResult>> =
            Rc::new(RefCell::new(EuvChartRenderResult::default()));
        let last_bounds: Rc<RefCell<Rect>> = Rc::new(RefCell::new(Rect::default()));
        let drag_state: Rc<Cell<(ChartDragTarget, f32, f32, f32)>> =
            Rc::new(Cell::new((ChartDragTarget::None, 0.0, 0.0, 100.0)));
        let has_data_zoom: bool = !initial_opt.get_data_zoom().is_empty();
        let toolbox_cfg: Option<EuvChartToolbox> = initial_opt.toolbox_opt().cloned();
        let show_toolbox: bool = toolbox_cfg.as_ref().is_some_and(EuvChartToolbox::get_show);
        let paint_opt_rc: Rc<EuvChartOption> = Rc::clone(&static_opt_rc);
        let paint_res: Rc<RefCell<EuvChartRenderResult>> = Rc::clone(&last_render);
        let paint_bounds: Rc<RefCell<Rect>> = Rc::clone(&last_bounds);
        let paint_opt_rev: Rc<Cell<u64>> = Rc::clone(&opt_rev);
        let inline_cache: RefCell<Option<ChartPaintCache>> = RefCell::new(None);
        let paint_inline = move |bounds: Rect, dl: &mut DisplayList| {
            let live_tok: ThemeTokens = live_tokens_untracked(tokens);
            let cur_state: EuvChartState = state_sig.get_untracked();
            let cur_rev: u64 = paint_opt_rev.get();
            if let Ok(mut b_guard) = paint_bounds.try_borrow_mut() {
                *b_guard = bounds;
            }
            if let Ok(guard) = inline_cache.try_borrow()
                && let Some(cached) = guard.as_ref()
                && cached.bounds == bounds
                && cached.tokens == live_tok
                && cached.state == cur_state
                && cached.rev == cur_rev
            {
                dl.get_mut_commands()
                    .extend_from_slice(cached.commands.as_slice());
                return;
            }
            let opt_snapshot: EuvChartOption = match opt_sig_opt {
                Some(sig) => sig.get_untracked(),
                None => (*paint_opt_rc).clone(),
            };
            let mut scratch: DisplayList = DisplayList::default();
            let res: EuvChartRenderResult = render_echart_to_display_list(
                &opt_snapshot,
                &cur_state,
                bounds,
                live_tok,
                &mut scratch,
            );
            if let Ok(mut r_guard) = paint_res.try_borrow_mut() {
                *r_guard = res;
            }
            let cmds: Vec<DrawCommand> = take(scratch.get_mut_commands());
            dl.get_mut_commands().extend_from_slice(cmds.as_slice());
            if let Ok(mut guard) = inline_cache.try_borrow_mut() {
                *guard = Some(ChartPaintCache {
                    bounds,
                    tokens: live_tok,
                    state: cur_state,
                    rev: cur_rev,
                    commands: cmds,
                });
            }
        };
        let dv_opt_rc: Rc<EuvChartOption> = Rc::clone(&static_opt_rc);
        let dv_ctx: Context = ctx.clone();
        let build_data_view = move || -> Vec<NodeId> {
            let mut local_ctx: Context = dv_ctx.clone();
            let st: EuvChartState = state_sig.get();
            if !st.get_data_view_open() {
                return Vec::new();
            }
            let live_tok: ThemeTokens = live_tokens_untracked(tokens);
            let live_p: ThemePalette = live_tok.get_palette();
            let live_g: ThemeGeometry = live_tok.get_geometry();
            let opt_snapshot: EuvChartOption = match opt_sig_opt {
                Some(sig) => sig.get(),
                None => (*dv_opt_rc).clone(),
            };
            let table: EuvChartDataViewTable = extract_data_view_table(&opt_snapshot, &st);
            let mut header_row: NodeId = NodeId::default();
            let mut body_col: NodeId = NodeId::default();
            let panel: NodeId = view! {
                column {
                    width: Length::Percent(100.0)
                    padding: Edges::all(SPACE_SM)
                    gap: SPACE_XS
                    background: live_p.get_muted().multiply_alpha(0.35)
                    border: (1.0, live_p.get_border())
                    radius: live_g.get_radius_md()
                    overflow: Overflow::Hidden
                    text {
                        width: Length::Percent(100.0)
                        font_size: 11.5
                        font_weight: FontWeight::SemiBold
                        color: live_p.get_foreground()
                        { ECHART_DATA_VIEW_TITLE }
                    }
                    row {
                        node_ref: header_row
                        width: Length::Percent(100.0)
                        padding: Edges::symmetric(4.0, SPACE_SM)
                        gap: SPACE_SM
                        background: live_p.get_card()
                        radius: live_g.get_radius_sm()
                    }
                    column {
                        node_ref: body_col
                        width: Length::Percent(100.0)
                        gap: 2.0
                    }
                }
            };
            for h in table.get_headers().iter() {
                let h_txt: String = h.clone();
                let cell: NodeId = view! {
                    text {
                        flex_grow: 1.0
                        flex_shrink: 1.0
                        min_width: Length::Px(0.0)
                        font_size: 11.0
                        font_weight: FontWeight::SemiBold
                        color: live_p.get_foreground()
                        text_overflow: TextOverflow::Ellipsis
                        { h_txt }
                    }
                };
                local_ctx.append(header_row, cell);
            }
            for row in table.get_rows().iter().take(40) {
                let row_node: NodeId = view! {
                    row {
                        width: Length::Percent(100.0)
                        padding: Edges::symmetric(4.0, SPACE_SM)
                        gap: SPACE_SM
                        border: (1.0, live_p.get_border().multiply_alpha(0.35))
                    }
                };
                for c in row.iter() {
                    let c_txt: String = c.clone();
                    let cell: NodeId = view! {
                        text {
                            flex_grow: 1.0
                            flex_shrink: 1.0
                            min_width: Length::Px(0.0)
                            font_size: 11.0
                            color: live_p.get_muted_foreground()
                            text_overflow: TextOverflow::Ellipsis
                            { c_txt }
                        }
                    };
                    local_ctx.append(row_node, cell);
                }
                local_ctx.append(body_col, row_node);
            }
            vec![panel]
        };
        let mut toolbox_nodes: Vec<NodeId> = Vec::new();
        if show_toolbox && let Some(tb) = toolbox_cfg.as_ref() {
            if tb.get_restore() {
                let r_opt: Rc<EuvChartOption> = Rc::clone(&static_opt_rc);
                let z_cb: Option<EuvChartZoomCallback> = on_zoom_change.clone();
                let on_restore = move |_: &UiEvent| {
                    let cur_opt: EuvChartOption = match opt_sig_opt {
                        Some(sig) => sig.get_untracked(),
                        None => (*r_opt).clone(),
                    };
                    let restored: EuvChartState = EuvChartState::from_option(&cur_opt);
                    let s_pct: f32 = restored.get_zoom_start_pct();
                    let e_pct: f32 = restored.get_zoom_end_pct();
                    let _: bool = state_sig.set(restored);
                    if let Some(cb) = z_cb.as_ref() {
                        cb(s_pct, e_pct);
                    }
                };
                toolbox_nodes.push(build_toolbox_icon_btn(
                    tokens,
                    EuvIconName::Refresh,
                    ECHART_BTN_RESTORE,
                    on_restore,
                ));
            }
            if tb.get_magic_type() {
                let on_magic = move |_: &UiEvent| {
                    let _: bool = state_sig.update(|st: &mut EuvChartState| {
                        let _: EuvChartMagicType = st.cycle_magic_type();
                    });
                };
                toolbox_nodes.push(build_toolbox_icon_btn(
                    tokens,
                    EuvIconName::Palette,
                    ECHART_BTN_MAGIC_TYPE,
                    on_magic,
                ));
            }
            if tb.get_data_view() {
                let on_dv = move |_: &UiEvent| {
                    let _: bool = state_sig.update(|st: &mut EuvChartState| {
                        let _: bool = st.toggle_data_view();
                    });
                };
                toolbox_nodes.push(build_toolbox_icon_btn(
                    tokens,
                    EuvIconName::Menu,
                    ECHART_BTN_DATA_VIEW,
                    on_dv,
                ));
            }
            if tb.get_fullscreen() {
                let on_fs = move |_: &UiEvent| {
                    let _: bool = fs_open.set_if_changed(true);
                };
                toolbox_nodes.push(build_toolbox_icon_btn(
                    tokens,
                    EuvIconName::Maximize,
                    ECHART_BTN_FULLSCREEN,
                    on_fs,
                ));
            }
        }
        let fs_opt_rc: Rc<EuvChartOption> = Rc::clone(&static_opt_rc);
        let ctx_fs: Context = ctx.clone();
        let fs_opt_rev: Rc<Cell<u64>> = Rc::clone(&opt_rev);
        let fs_click_cb: Option<EuvChartHitCallback> = on_point_click.clone();
        let fs_legend_cb: Option<EuvChartLegendCallback> = on_legend_change.clone();
        let fs_zoom_cb: Option<EuvChartZoomCallback> = on_zoom_change.clone();
        let build_fs_children = move || -> Vec<NodeId> {
            if !fs_open.get() {
                return Vec::new();
            }
            let mut local_fs_ctx: Context = ctx_fs.clone();
            let live_tok: ThemeTokens = live_tokens_untracked(tokens);
            let live_p: ThemePalette = live_tok.get_palette();
            let live_g: ThemeGeometry = live_tok.get_geometry();
            let cur_opt: EuvChartOption = match opt_sig_opt {
                Some(sig) => sig.get_untracked(),
                None => (*fs_opt_rc).clone(),
            };
            let modal_title: String = cur_opt
                .title_opt()
                .map(|t: &EuvChartTitle| t.get_text().clone())
                .filter(|t: &String| !t.is_empty())
                .unwrap_or_else(|| ECHART_DEFAULT_MODAL_TITLE.to_string());
            let modal_opt_rc: Rc<EuvChartOption> = Rc::new(cur_opt);
            let modal_opt_wire: Rc<EuvChartOption> = Rc::clone(&modal_opt_rc);
            let modal_render: Rc<RefCell<EuvChartRenderResult>> =
                Rc::new(RefCell::new(EuvChartRenderResult::default()));
            let modal_bounds: Rc<RefCell<Rect>> = Rc::new(RefCell::new(Rect::default()));
            let modal_drag: Rc<Cell<(ChartDragTarget, f32, f32, f32)>> =
                Rc::new(Cell::new((ChartDragTarget::None, 0.0, 0.0, 100.0)));
            let modal_render_paint: Rc<RefCell<EuvChartRenderResult>> = Rc::clone(&modal_render);
            let modal_bounds_paint: Rc<RefCell<Rect>> = Rc::clone(&modal_bounds);
            let modal_rev_paint: Rc<Cell<u64>> = Rc::clone(&fs_opt_rev);
            let modal_cache: RefCell<Option<ChartPaintCache>> = RefCell::new(None);
            let paint_modal = move |bounds: Rect, dl: &mut DisplayList| {
                let cur_tok: ThemeTokens = live_tokens_untracked(tokens);
                let cur_st: EuvChartState = state_sig.get_untracked();
                let cur_rev: u64 = modal_rev_paint.get();
                if let Ok(mut b_guard) = modal_bounds_paint.try_borrow_mut() {
                    *b_guard = bounds;
                }
                if let Ok(guard) = modal_cache.try_borrow()
                    && let Some(cached) = guard.as_ref()
                    && cached.bounds == bounds
                    && cached.tokens == cur_tok
                    && cached.state == cur_st
                    && cached.rev == cur_rev
                {
                    dl.get_mut_commands()
                        .extend_from_slice(cached.commands.as_slice());
                    return;
                }
                let live_opt: EuvChartOption = match opt_sig_opt {
                    Some(sig) => sig.get_untracked(),
                    None => (*modal_opt_rc).clone(),
                };
                let mut scratch: DisplayList = DisplayList::default();
                let res: EuvChartRenderResult = render_echart_to_display_list(
                    &live_opt,
                    &cur_st,
                    bounds,
                    cur_tok,
                    &mut scratch,
                );
                if let Ok(mut r_guard) = modal_render_paint.try_borrow_mut() {
                    *r_guard = res;
                }
                let cmds: Vec<DrawCommand> = take(scratch.get_mut_commands());
                dl.get_mut_commands().extend_from_slice(cmds.as_slice());
                if let Ok(mut guard) = modal_cache.try_borrow_mut() {
                    *guard = Some(ChartPaintCache {
                        bounds,
                        tokens: cur_tok,
                        state: cur_st,
                        rev: cur_rev,
                        commands: cmds,
                    });
                }
            };
            let paint_close_icon = move |bounds: Rect, dl: &mut DisplayList| {
                let cur_p: ThemePalette = live_tokens_untracked(tokens).get_palette();
                draw_euv_icon(
                    EuvIconName::Minimize,
                    bounds,
                    dl,
                    cur_p.get_primary_foreground(),
                );
            };
            let on_close_fs = move |_: &UiEvent| {
                let _: bool = fs_open.set_if_changed(false);
            };
            let safe_sig: Signal<SafeAreaInsets> = local_fs_ctx.get_safe_area();
            let init_safe: SafeAreaInsets = safe_sig.get_untracked();
            let mut top_bar_node: NodeId = NodeId::default();
            let mut body_host_node: NodeId = NodeId::default();
            let mut modal_canvas: NodeId = NodeId::default();
            let panel: NodeId = view! {
                column {
                    width: Length::Percent(100.0)
                    height: Length::Percent(100.0)
                    flex_grow: 1.0
                    padding: Edges::all(0.0)
                    margin: Edges::all(0.0)
                    background: live_p.get_card()
                    border: (0.0, COLOR_TRANSPARENT)
                    radius: 0.0
                    overflow: Overflow::Hidden
                    stop_event_propagation: true
                    row {
                        node_ref: top_bar_node
                        width: Length::Percent(100.0)
                        flex_shrink: 0.0
                        padding: Edges::new(
                            SPACE_SM + init_safe.get_top(),
                            SPACE_MD + init_safe.get_right(),
                            SPACE_SM,
                            SPACE_MD + init_safe.get_left()
                        )
                        justify: JustifyContent::SpaceBetween
                        align: AlignItems::Center
                        gap: SPACE_SM
                        background: live_p.get_muted().multiply_alpha(0.4)
                        text {
                            flex_shrink: 1.0
                            min_width: Length::Px(0.0)
                            overflow: Overflow::Hidden
                            text_overflow: TextOverflow::Ellipsis
                            font_size: 13.5
                            font_weight: FontWeight::SemiBold
                            color: live_p.get_foreground()
                            { modal_title }
                        }
                        row {
                            flex_shrink: 0.0
                            align: AlignItems::Center
                            gap: SPACE_XS
                            row {
                                width: Length::Px(28.0)
                                height: Length::Px(28.0)
                                justify: JustifyContent::Center
                                align: AlignItems::Center
                                radius: live_g.get_radius_sm()
                                cursor: CursorIcon::Pointer
                                background: live_p.get_primary()
                                role: A11yRole::Button
                                a11y_label: ECHART_BTN_CLOSE.to_string()
                                on_click: on_close_fs
                                canvas {
                                    width: Length::Px(14.0)
                                    height: Length::Px(14.0)
                                    painter: paint_close_icon
                                }
                            }
                        }
                    }
                    view {
                        width: Length::Percent(100.0)
                        height: Length::Px(1.0)
                        background: live_p.get_border()
                    }
                    column {
                        node_ref: body_host_node
                        flex_grow: 1.0
                        flex_shrink: 1.0
                        min_height: Length::Px(0.0)
                        width: Length::Percent(100.0)
                        padding: Edges::new(
                            SPACE_XS,
                            init_safe.get_right(),
                            init_safe.get_bottom(),
                            init_safe.get_left()
                        )
                        overflow: Overflow::Hidden
                        canvas {
                            node_ref: modal_canvas
                            width: Length::Percent(100.0)
                            height: Length::Percent(100.0)
                            flex_grow: 1.0
                            cursor: if interactive { CursorIcon::Pointer } else { CursorIcon::Default }
                            stop_scroll_propagation: has_data_zoom
                            stop_event_propagation: has_data_zoom
                            painter: paint_modal
                        }
                    }
                }
            };
            if interactive {
                wire_chart_pointer_events(
                    &mut local_fs_ctx,
                    ChartPointerWiring {
                        canvas_node: modal_canvas,
                        state_sig,
                        last_render: modal_render,
                        last_bounds: modal_bounds,
                        drag_state: Rc::clone(&modal_drag),
                        static_opt: modal_opt_wire,
                        opt_sig: opt_sig_opt,
                        on_point_click: fs_click_cb.clone(),
                        on_legend_change: fs_legend_cb.clone(),
                        on_zoom_change: fs_zoom_cb.clone(),
                    },
                );
            }
            let handle_safe: SceneHandle = local_fs_ctx.get_scene().clone();
            let handle_modal_st: SceneHandle = handle_safe.clone();
            let app_state_fs: Signal<AppState> = local_fs_ctx.get_app_state();
            let (safe_scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
                let _: Effect = Effect::new(move || {
                    if app_state_fs.get() != AppState::Active {
                        modal_drag.set((ChartDragTarget::None, 0.0, 0.0, 100.0));
                    }
                });
                let _: Effect = Effect::new(move || {
                    let _: EuvChartState = state_sig.get();
                    handle_modal_st.mark_dirty(modal_canvas, DIRTY_PAINT);
                });
                if let Some(sig) = opt_sig_opt {
                    let handle_modal_opt: SceneHandle = handle_safe.clone();
                    let _: Effect = Effect::new(move || {
                        let _: EuvChartOption = sig.get();
                        handle_modal_opt.mark_dirty(modal_canvas, DIRTY_PAINT);
                    });
                }
                Effect::new(move || {
                    let safe: SafeAreaInsets = safe_sig.get();
                    handle_safe.with_write(
                        |tree: &mut SceneTree| {
                            let dirty_l: u16 = DIRTY_LAYOUT | DIRTY_SUBTREE_LAYOUT;
                            if let Some(n) = tree.get_node_mut(top_bar_node) {
                                n.get_mut_style().get_mut_layout().set_padding(Edges::new(
                                    SPACE_SM + safe.get_top(),
                                    SPACE_MD + safe.get_right(),
                                    SPACE_SM,
                                    SPACE_MD + safe.get_left(),
                                ));
                                tree.mark_dirty(top_bar_node, dirty_l | DIRTY_PAINT);
                            }
                            if let Some(n) = tree.get_node_mut(body_host_node) {
                                n.get_mut_style().get_mut_layout().set_padding(Edges::new(
                                    SPACE_XS,
                                    safe.get_right(),
                                    safe.get_bottom(),
                                    safe.get_left(),
                                ));
                                tree.mark_dirty(body_host_node, dirty_l | DIRTY_PAINT);
                            }
                        },
                        (),
                    );
                })
            });
            local_fs_ctx.get_scene().with_write(
                |tree: &mut SceneTree| {
                    tree.attach_scope(panel, safe_scope);
                },
                (),
            );
            vec![panel]
        };
        let mut toolbox_row: NodeId = NodeId::default();
        let mut chart_canvas: NodeId = NodeId::default();
        let mut fs_portal: NodeId = NodeId::default();
        let root: NodeId = view! {
            column {
                width: width
                background: p.get_card()
                border: (g.get_border_width(), p.get_border())
                radius: g.get_radius_lg()
                overflow: Overflow::Hidden
                row {
                    node_ref: toolbox_row
                    width: Length::Percent(100.0)
                    display: if show_toolbox { Display::Flex } else { Display::None }
                    justify: JustifyContent::End
                    align: AlignItems::Center
                    padding: Edges::new(SPACE_XS, SPACE_SM, 0.0, SPACE_SM)
                    gap: SPACE_XS
                }
                canvas {
                    node_ref: chart_canvas
                    width: Length::Percent(100.0)
                    height: height
                    cursor: if interactive { CursorIcon::Pointer } else { CursorIcon::Default }
                    stop_scroll_propagation: has_data_zoom
                    stop_event_propagation: has_data_zoom
                    painter: paint_inline
                }
                column {
                    width: Length::Percent(100.0)
                    bind_children: build_data_view
                }
                portal {
                    node_ref: fs_portal
                    style: Style::default().column()
                    display: Display::None
                    position: Position::Fixed
                    inset: (Some(0.0), Some(0.0), Some(0.0), Some(0.0))
                    width: Length::Percent(100.0)
                    height: Length::Percent(100.0)
                    padding: Edges::all(0.0)
                    margin: Edges::all(0.0)
                    justify: JustifyContent::Start
                    align: AlignItems::Stretch
                    background: p.get_card()
                    border: (0.0, COLOR_TRANSPARENT)
                    radius: 0.0
                    overflow: Overflow::Hidden
                    pointer_events: PointerEvents::None
                    stop_scroll_propagation: true
                    stop_event_propagation: true
                    z_index: Z_INDEX_FULLSCREEN
                    bind_children: build_fs_children
                }
            }
        };
        for tb_node in toolbox_nodes.into_iter() {
            ctx.append(toolbox_row, tb_node);
        }
        let drag_bg_reset: Rc<Cell<(ChartDragTarget, f32, f32, f32)>> = Rc::clone(&drag_state);
        if interactive {
            wire_chart_pointer_events(
                &mut ctx,
                ChartPointerWiring {
                    canvas_node: chart_canvas,
                    state_sig,
                    last_render,
                    last_bounds,
                    drag_state,
                    static_opt: static_opt_rc,
                    opt_sig: opt_sig_opt,
                    on_point_click: on_point_click.clone(),
                    on_legend_change: on_legend_change.clone(),
                    on_zoom_change: on_zoom_change.clone(),
                },
            );
        }
        let scene_dirty: SceneHandle = ctx.get_scene().clone();
        let scene_fs: SceneHandle = ctx.get_scene().clone();
        let app_state_sig: Signal<AppState> = ctx.get_app_state();
        let prev_open: Rc<Cell<bool>> = Rc::new(Cell::new(false));
        let fs_acquired: Rc<Cell<bool>> = Rc::new(Cell::new(false));
        let fs_cleanup: Rc<Cell<bool>> = Rc::clone(&fs_acquired);
        let rev_watch: Rc<Cell<u64>> = Rc::clone(&opt_rev);
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            on_cleanup(move || {
                if fs_cleanup.replace(false) {
                    exit_fullscreen();
                }
            });
            let _: Effect = Effect::new(move || {
                if app_state_sig.get() != AppState::Active {
                    drag_bg_reset.set((ChartDragTarget::None, 0.0, 0.0, 100.0));
                }
            });
            let _: Effect = EuvOverlayStack::bind_signal(
                fs_open,
                ECHART_FULLSCREEN_OVERLAY_NAME,
                Z_INDEX_FULLSCREEN,
            );
            let scene_st: SceneHandle = scene_dirty.clone();
            let _: Effect = Effect::new(move || {
                let _: EuvChartState = state_sig.get();
                scene_st.mark_dirty(chart_canvas, DIRTY_PAINT);
            });
            if let Some(sig) = opt_sig_opt {
                let scene_opt: SceneHandle = scene_dirty.clone();
                let _: Effect = Effect::new(move || {
                    let _: EuvChartOption = sig.get();
                    rev_watch.set(rev_watch.get().wrapping_add(1));
                    scene_opt.mark_dirty(chart_canvas, DIRTY_PAINT);
                });
            }
            Effect::new(move || {
                let is_open: bool = fs_open.get();
                if is_open == prev_open.replace(is_open) {
                    return;
                }
                if is_open {
                    if !fs_acquired.replace(true) {
                        enter_fullscreen();
                    }
                    let live_bg: Color = live_tokens_untracked(tokens).get_palette().get_card();
                    scene_fs.with_write(
                        |tree: &mut SceneTree| {
                            tree.cancel_subtree_transitions(fs_portal);
                            if let Some(n) = tree.get_node_mut(fs_portal) {
                                let st: &mut Style = n.get_mut_style();
                                st.get_mut_layout().set_display(Display::Flex);
                                st.get_mut_paint()
                                    .set_background(Some(live_bg))
                                    .set_pointer_events(PointerEvents::Auto)
                                    .set_opacity(1.0)
                                    .set_transform(Transform2D::default());
                            }
                            let dirty_l: u16 = DIRTY_LAYOUT | DIRTY_SUBTREE_LAYOUT;
                            tree.mark_dirty(fs_portal, dirty_l | DIRTY_PAINT);
                        },
                        (),
                    );
                } else {
                    if fs_acquired.replace(false) {
                        exit_fullscreen();
                    }
                    scene_fs.with_write(
                        |tree: &mut SceneTree| {
                            tree.cancel_subtree_transitions(fs_portal);
                            if let Some(n) = tree.get_node_mut(fs_portal) {
                                let st: &mut Style = n.get_mut_style();
                                st.get_mut_layout().set_display(Display::None);
                                st.get_mut_paint()
                                    .set_pointer_events(PointerEvents::None)
                                    .set_opacity(1.0)
                                    .set_transform(Transform2D::default());
                            }
                            let dirty_l: u16 = DIRTY_LAYOUT | DIRTY_SUBTREE_LAYOUT;
                            tree.mark_dirty(fs_portal, dirty_l | DIRTY_PAINT);
                        },
                        (),
                    );
                }
            })
        });
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                tree.attach_scope(root, scope);
            },
            (),
        );
        root
    }
}
