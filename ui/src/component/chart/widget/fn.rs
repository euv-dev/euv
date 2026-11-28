use super::*;

/// Builds a compact square native vector icon-only toolbox button for `EuvEChart`.
///
/// # Arguments
///
/// - `ThemeTokens` - Active UI theme tokens.
/// - `EuvIconName` - Built-in vector icon glyph rendered via `draw_euv_icon`.
/// - `&'static str` - Accessible button label.
/// - `F` - Click callback closure.
///
/// # Returns
///
/// - `NodeId` - Mounted compact icon button node.
pub(crate) fn build_toolbox_icon_btn<F>(
    tokens: ThemeTokens,
    icon: EuvIconName,
    a11y_label: &'static str,
    on_click: F,
) -> NodeId
where
    F: FnMut(&UiEvent) + 'static,
{
    let p: ThemePalette = tokens.get_palette();
    let g: ThemeGeometry = tokens.get_geometry();
    let paint_icon = move |bounds: Rect, dl: &mut DisplayList| {
        let live_p: ThemePalette = live_tokens_untracked(tokens).get_palette();
        draw_euv_icon(icon, bounds, dl, live_p.get_muted_foreground());
    };
    view! {
        row {
            width: Length::Px(26.0)
            height: Length::Px(26.0)
            justify: JustifyContent::Center
            align: AlignItems::Center
            radius: g.get_radius_sm()
            background: p.get_muted().multiply_alpha(0.55)
            border: (1.0, p.get_border())
            cursor: CursorIcon::Pointer
            role: A11yRole::Button
            a11y_label: a11y_label.to_string()
            on_click: on_click
            canvas {
                width: Length::Px(13.0)
                height: Length::Px(13.0)
                painter: paint_icon
            }
        }
    }
}

/// Converts a screen-space pointer coordinate into canvas-local logical coordinates matching `render_echart_to_display_list`.
///
/// # Arguments
///
/// - `&SceneHandle` - Active scene handle.
/// - `NodeId` - Canvas node handle.
/// - `Point` - Screen-space pointer coordinate.
/// - `&RefCell<Rect>` - Last painted logical canvas rect.
///
/// # Returns
///
/// - `Option<Point>` - Canvas coordinate matching emitted hit boxes.
#[must_use]
pub(crate) fn to_canvas_point(
    scene: &SceneHandle,
    node: NodeId,
    screen_pt: Point,
    last_bounds: &RefCell<Rect>,
) -> Option<Point> {
    let sb: Rect = scene.node_screen_bounds(node)?;
    let paint_b: Rect = last_bounds
        .try_borrow()
        .map(|g: Ref<'_, Rect>| *g)
        .unwrap_or(sb);
    Some(Point::new(
        paint_b.get_x() + (screen_pt.get_x() - sb.get_x()),
        paint_b.get_y() + (screen_pt.get_y() - sb.get_y()),
    ))
}

/// Tests whether `pt` lies inside `rect`.
///
/// # Arguments
///
/// - `Rect` - Target hit rectangle.
/// - `Point` - Probe point.
///
/// # Returns
///
/// - `bool` - True when `pt` is inside `rect`.
#[must_use]
pub(crate) fn rect_contains(rect: Rect, pt: Point) -> bool {
    pt.get_x() >= rect.get_x()
        && pt.get_x() <= rect.get_x() + rect.get_width()
        && pt.get_y() >= rect.get_y()
        && pt.get_y() <= rect.get_y() + rect.get_height()
}

/// Attaches pointer hover, click, drag, and wheel DataZoom listeners onto the `EuvEChart` canvas node.
///
/// # Arguments
///
/// - `&mut Context` - Active framework context.
/// - `ChartPointerWiring` - Bundled canvas node, state signal, hit caches, and user callbacks.
pub(crate) fn wire_chart_pointer_events(ctx: &mut Context, w: ChartPointerWiring) {
    let canvas_node: NodeId = w.canvas_node;
    let state_sig: Signal<EuvChartState> = w.state_sig;
    let opt_sig: Option<Signal<EuvChartOption>> = w.opt_sig;
    let scene_down: SceneHandle = ctx.get_scene().clone();
    let res_down: Rc<RefCell<EuvChartRenderResult>> = Rc::clone(&w.last_render);
    let b_down: Rc<RefCell<Rect>> = Rc::clone(&w.last_bounds);
    let drag_down: Rc<Cell<(ChartDragTarget, f32, f32, f32)>> = Rc::clone(&w.drag_state);
    let opt_down: Rc<EuvChartOption> = Rc::clone(&w.static_opt);
    ctx.on_pointer_down(canvas_node, move |ev: &UiEvent| {
        let UiEventKind::PointerDown(raw_pt) = ev.get_kind() else {
            return;
        };
        let Some(pt) = to_canvas_point(&scene_down, canvas_node, *raw_pt, &b_down) else {
            return;
        };
        let cur_st: EuvChartState = state_sig.get_untracked();
        if let Ok(r_guard) = res_down.try_borrow() {
            if let Some(sl) = r_guard.slider_rects_opt() {
                if rect_contains(sl.get_left_handle(), pt) {
                    drag_down.set((
                        ChartDragTarget::SliderLeft,
                        pt.get_x(),
                        cur_st.get_zoom_start_pct(),
                        cur_st.get_zoom_end_pct(),
                    ));
                    return;
                }
                if rect_contains(sl.get_right_handle(), pt) {
                    drag_down.set((
                        ChartDragTarget::SliderRight,
                        pt.get_x(),
                        cur_st.get_zoom_start_pct(),
                        cur_st.get_zoom_end_pct(),
                    ));
                    return;
                }
                if rect_contains(sl.get_window_rect(), pt) {
                    drag_down.set((
                        ChartDragTarget::SliderWindow,
                        pt.get_x(),
                        cur_st.get_zoom_start_pct(),
                        cur_st.get_zoom_end_pct(),
                    ));
                    return;
                }
            }
            let cur_opt: EuvChartOption = match opt_sig {
                Some(sig) => sig.get_untracked(),
                None => (*opt_down).clone(),
            };
            let has_inside: bool = cur_opt
                .get_data_zoom()
                .iter()
                .any(|dz: &EuvChartDataZoom| dz.get_kind() == EuvChartDataZoomKind::Inside);
            if has_inside && rect_contains(r_guard.get_plot_rect(), pt) {
                drag_down.set((
                    ChartDragTarget::InsidePan,
                    pt.get_x(),
                    cur_st.get_zoom_start_pct(),
                    cur_st.get_zoom_end_pct(),
                ));
            }
        }
    });
    let scene_move: SceneHandle = ctx.get_scene().clone();
    let res_move: Rc<RefCell<EuvChartRenderResult>> = Rc::clone(&w.last_render);
    let b_move: Rc<RefCell<Rect>> = Rc::clone(&w.last_bounds);
    let drag_move: Rc<Cell<(ChartDragTarget, f32, f32, f32)>> = Rc::clone(&w.drag_state);
    let z_cb_move: Option<EuvChartZoomCallback> = w.on_zoom_change.clone();
    ctx.on_pointer_move(canvas_node, move |ev: &UiEvent| {
        let UiEventKind::PointerMove(raw_pt) = ev.get_kind() else {
            return;
        };
        let Some(pt) = to_canvas_point(&scene_move, canvas_node, *raw_pt, &b_move) else {
            return;
        };
        let (target, start_x, orig_s, orig_e): (ChartDragTarget, f32, f32, f32) = drag_move.get();
        if target != ChartDragTarget::None
            && let Ok(r_guard) = res_move.try_borrow()
        {
            let mut next_range: Option<(f32, f32)> = None;
            match target {
                ChartDragTarget::SliderLeft => {
                    if let Some(sl) = r_guard.slider_rects_opt() {
                        let tw: f32 = sl.get_track_rect().get_width().max(1.0);
                        let frac: f32 =
                            ((pt.get_x() - sl.get_track_rect().get_x()) / tw).clamp(0.0, 1.0);
                        let lo: f32 = (frac * 100.0).min(orig_e - ECHART_ZOOM_DEFAULT_MIN_SPAN);
                        next_range = Some((lo.max(0.0), orig_e));
                    }
                }
                ChartDragTarget::SliderRight => {
                    if let Some(sl) = r_guard.slider_rects_opt() {
                        let tw: f32 = sl.get_track_rect().get_width().max(1.0);
                        let frac: f32 =
                            ((pt.get_x() - sl.get_track_rect().get_x()) / tw).clamp(0.0, 1.0);
                        let hi: f32 = (frac * 100.0).max(orig_s + ECHART_ZOOM_DEFAULT_MIN_SPAN);
                        next_range = Some((orig_s, hi.min(100.0)));
                    }
                }
                ChartDragTarget::SliderWindow => {
                    if let Some(sl) = r_guard.slider_rects_opt() {
                        let tw: f32 = sl.get_track_rect().get_width().max(1.0);
                        let delta_pct: f32 = ((pt.get_x() - start_x) / tw) * 100.0;
                        let span: f32 = orig_e - orig_s;
                        let lo: f32 = (orig_s + delta_pct).clamp(0.0, 100.0 - span);
                        next_range = Some((lo, lo + span));
                    }
                }
                ChartDragTarget::InsidePan => {
                    let pw: f32 = r_guard.get_plot_rect().get_width().max(1.0);
                    let span: f32 = orig_e - orig_s;
                    let delta_pct: f32 = -((pt.get_x() - start_x) / pw) * span;
                    let lo: f32 = (orig_s + delta_pct).clamp(0.0, 100.0 - span);
                    next_range = Some((lo, lo + span));
                }
                ChartDragTarget::None => {}
            }
            if let Some((lo, hi)) = next_range {
                let _: bool = state_sig.update(|st: &mut EuvChartState| {
                    st.set_zoom(lo, hi, ECHART_ZOOM_DEFAULT_MIN_SPAN);
                });
                if let Some(cb) = z_cb_move.as_ref() {
                    cb(lo, hi);
                }
                return;
            }
        }
        let _: bool = state_sig.update(|st: &mut EuvChartState| {
            st.set_hover_pos(Some(pt));
        });
    });
    let drag_up: Rc<Cell<(ChartDragTarget, f32, f32, f32)>> = Rc::clone(&w.drag_state);
    ctx.on_pointer_up(canvas_node, move |_: &UiEvent| {
        drag_up.set((ChartDragTarget::None, 0.0, 0.0, 100.0));
    });
    let drag_cancel: Rc<Cell<(ChartDragTarget, f32, f32, f32)>> = Rc::clone(&w.drag_state);
    ctx.on_pointer_cancel(canvas_node, move |_: &UiEvent| {
        drag_cancel.set((ChartDragTarget::None, 0.0, 0.0, 100.0));
        let _: bool = state_sig.update(|st: &mut EuvChartState| {
            st.set_hover_pos(None);
        });
    });
    let drag_leave: Rc<Cell<(ChartDragTarget, f32, f32, f32)>> = Rc::clone(&w.drag_state);
    ctx.on_pointer_leave(canvas_node, move |_: &UiEvent| {
        drag_leave.set((ChartDragTarget::None, 0.0, 0.0, 100.0));
        let _: bool = state_sig.update(|st: &mut EuvChartState| {
            st.set_hover_pos(None);
        });
    });
    let scene_click: SceneHandle = ctx.get_scene().clone();
    let res_click: Rc<RefCell<EuvChartRenderResult>> = Rc::clone(&w.last_render);
    let b_click: Rc<RefCell<Rect>> = Rc::clone(&w.last_bounds);
    let opt_click: Rc<EuvChartOption> = Rc::clone(&w.static_opt);
    let click_cb: Option<EuvChartHitCallback> = w.on_point_click;
    let legend_cb: Option<EuvChartLegendCallback> = w.on_legend_change;
    ctx.on_click(canvas_node, move |ev: &UiEvent| {
        let UiEventKind::Click(raw_pt) = ev.get_kind() else {
            return;
        };
        let Some(pt) = to_canvas_point(&scene_click, canvas_node, *raw_pt, &b_click) else {
            return;
        };
        let Ok(r_guard) = res_click.try_borrow() else {
            return;
        };
        for l_hit in r_guard.get_legend_hits().iter() {
            if rect_contains(l_hit.get_rect(), pt) {
                let name: String = l_hit.get_name().clone();
                let mut now_vis: bool = true;
                let _: bool = state_sig.update(|st: &mut EuvChartState| {
                    now_vis = st.toggle_series(name.as_str());
                });
                if let Some(cb) = legend_cb.as_ref() {
                    cb(name.as_str(), now_vis);
                }
                return;
            }
        }
        let matched: Vec<EuvChartDatumHit> =
            hit_test_chart(r_guard.get_hits(), pt, EuvChartTooltipTrigger::Item);
        if let Some(hit) = matched.first() {
            let cur_opt: EuvChartOption = match opt_sig {
                Some(sig) => sig.get_untracked(),
                None => (*opt_click).clone(),
            };
            if let Some(s) = cur_opt.get_series().get(hit.get_series_index())
                && matches!(s.get_data(), EuvChartSeriesData::Pie(_))
            {
                let d_idx: usize = hit.get_data_index();
                let _: bool = state_sig.update(|st: &mut EuvChartState| {
                    st.toggle_pie_slice(d_idx);
                });
            }
            if let Some(cb) = click_cb.as_ref() {
                cb(hit);
            }
        }
    });
    let opt_scroll: Rc<EuvChartOption> = Rc::clone(&w.static_opt);
    let res_scroll: Rc<RefCell<EuvChartRenderResult>> = Rc::clone(&w.last_render);
    let z_cb_scroll: Option<EuvChartZoomCallback> = w.on_zoom_change;
    ctx.on_scroll(canvas_node, move |ev: &UiEvent| {
        let UiEventKind::Scroll(delta) = ev.get_kind() else {
            return;
        };
        let cur_opt: EuvChartOption = match opt_sig {
            Some(sig) => sig.get_untracked(),
            None => (*opt_scroll).clone(),
        };
        if cur_opt.get_data_zoom().is_empty() {
            return;
        }
        let min_span: f32 = cur_opt
            .get_data_zoom()
            .first()
            .map(EuvChartDataZoom::get_min_span_pct)
            .unwrap_or(ECHART_ZOOM_DEFAULT_MIN_SPAN);
        let anchor_frac: f32 = state_sig
            .get_untracked()
            .hover_pos_opt()
            .and_then(|hp: Point| {
                res_scroll
                    .try_borrow()
                    .ok()
                    .map(|r: Ref<'_, EuvChartRenderResult>| {
                        let pr: Rect = r.get_plot_rect();
                        ((hp.get_x() - pr.get_x()) / pr.get_width().max(1.0)).clamp(0.0, 1.0)
                    })
            })
            .unwrap_or(0.5);
        let mut out_range: Option<(f32, f32)> = None;
        let _: bool = state_sig.update(|st: &mut EuvChartState| {
            if st.zoom_by_wheel(delta.get_y(), anchor_frac, min_span) {
                out_range = Some((st.get_zoom_start_pct(), st.get_zoom_end_pct()));
            }
        });
        if let Some((lo, hi)) = out_range
            && let Some(cb) = z_cb_scroll.as_ref()
        {
            cb(lo, hi);
        }
    });
}
