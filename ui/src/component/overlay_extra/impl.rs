use super::*;

/// Declarative `#[component]` implementation for `EuvDrawer`.
#[component]
/// Inherent implementation of [`EuvDrawer`].
impl EuvDrawer {
    /// Renders `EuvDrawer` into the ambient framework-managed `Context`.
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
        let vp_sig: Signal<Size> = ctx.get_viewport_size();
        let open: Signal<bool> = self.get_open();
        let body: NodeId = self.get_body();
        let Self { title, .. } = self;
        let title_owned: String = title.clone();
        let init_open: bool = open.get_untracked();
        let expanded: Signal<bool> = Signal::new(false);
        let drag_active: Rc<Cell<bool>> = Rc::new(Cell::new(false));
        let drag_start_x: Rc<Cell<f32>> = Rc::new(Cell::new(0.0));
        let drag_base_w: Rc<Cell<f32>> = Rc::new(Cell::new(340.0));
        let drag_moved: Rc<Cell<bool>> = Rc::new(Cell::new(false));
        let mut panel: NodeId = NodeId::default();
        let mut grabber: NodeId = NodeId::default();
        let scrim: NodeId = view! {
            portal {
                style: Style::default().row()
                display: if init_open { Display::Flex } else { Display::None }
                position: Position::Fixed
                inset: (Some(0.0), Some(0.0), Some(0.0), Some(0.0))
                width: Length::Percent(100.0)
                height: Length::Percent(100.0)
                justify: JustifyContent::End
                background: p.get_overlay()
                stop_scroll_propagation: true
                z_index: Z_INDEX_DRAWER
                row {
                    node_ref: panel
                    width: Length::Px(340.0)
                    max_width: Length::Percent(94.0)
                    height: Length::Percent(100.0)
                    background: p.get_card()
                    border: (g.get_border_width(), p.get_border())
                    shadow: g.get_shadow_lg()
                    overflow: Overflow::Hidden
                    row {
                        node_ref: grabber
                        width: Length::Px(22.0)
                        height: Length::Percent(100.0)
                        flex_shrink: 0.0
                        justify: JustifyContent::Center
                        align: AlignItems::Center
                        cursor: CursorIcon::EwResize
                        stop_scroll_propagation: true
                        stop_event_propagation: true
                        view {
                            width: Length::Px(4.0)
                            height: Length::Px(46.0)
                            radius: 99.0
                            background: p.get_muted_foreground().with_alpha(128)
                        }
                    }
                    scroll {
                        style: Style::default().column()
                        flex_grow: 1.0
                        flex_shrink: 1.0
                        min_width: Length::Px(0.0)
                        height: Length::Percent(100.0)
                        padding: Edges::new(SPACE_XL, SPACE_XL, SPACE_XL, SPACE_SM)
                        gap: SPACE_MD
                        stop_scroll_propagation: true
                        text {
                            width: Length::Percent(100.0)
                            font_size: g.get_font_heading()
                            font_weight: FontWeight::Bold
                            color: p.get_card_foreground()
                            { title_owned }
                        }
                        { body }
                    }
                }
            }
        };
        let drag_down_active: Rc<Cell<bool>> = drag_active.clone();
        let drag_down_x: Rc<Cell<f32>> = drag_start_x.clone();
        let drag_down_w: Rc<Cell<f32>> = drag_base_w.clone();
        let drag_down_moved: Rc<Cell<bool>> = drag_moved.clone();
        let handle_down: SceneHandle = ctx.get_scene().clone();
        ctx.on_pointer_down(grabber, move |ev: &UiEvent| {
            if let UiEventKind::PointerDown(pt) = ev.get_kind() {
                let vp_w: f32 = vp_sig.get_untracked().get_width().max(320.0);
                let def_w: f32 = (vp_w * 0.80).clamp(240.0, 340.0);
                let cur_w: f32 = handle_down
                    .node_screen_bounds(panel)
                    .map(|r: Rect| r.get_width())
                    .filter(|w: &f32| *w > 40.0)
                    .unwrap_or(def_w);
                drag_down_active.set(true);
                drag_down_x.set(pt.get_x());
                drag_down_w.set(cur_w);
                drag_down_moved.set(false);
            }
        });
        for target_node in [grabber, panel, scrim].into_iter() {
            let h_move: SceneHandle = ctx.get_scene().clone();
            let da_move: Rc<Cell<bool>> = drag_active.clone();
            let sx_move: Rc<Cell<f32>> = drag_start_x.clone();
            let bw_move: Rc<Cell<f32>> = drag_base_w.clone();
            let dm_move: Rc<Cell<bool>> = drag_moved.clone();
            ctx.on_pointer_move(target_node, move |ev: &UiEvent| {
                if !da_move.get() {
                    return;
                }
                if let UiEventKind::PointerMove(pt) = ev.get_kind() {
                    let dx: f32 = pt.get_x() - sx_move.get();
                    if dx.abs() > 4.0 {
                        dm_move.set(true);
                    }
                    let vp_w: f32 = vp_sig.get_untracked().get_width().max(320.0);
                    let def_w: f32 = (vp_w * 0.80).clamp(240.0, 340.0);
                    let max_w: f32 = (vp_w * 0.92).clamp(def_w + 40.0, 560.0);
                    let raw_w: f32 = bw_move.get() - dx;
                    let (next_w, tx): (f32, f32) = if raw_w >= def_w {
                        (raw_w.min(max_w), 0.0)
                    } else {
                        (def_w, (def_w - raw_w).max(0.0))
                    };
                    h_move.with_write(
                        |tree: &mut SceneTree| {
                            if let Some(n) = tree.get_node_mut(panel) {
                                let st: &mut Style = n.get_mut_style();
                                st.get_mut_layout().set_width(Length::Px(next_w));
                                st.get_mut_paint()
                                    .set_transform(Transform2D::new(tx, 0.0, 1.0, 1.0, 0.0));
                                let layout_dirty: u16 = DIRTY_LAYOUT | DIRTY_SUBTREE_LAYOUT;
                                tree.mark_dirty(panel, layout_dirty | DIRTY_PAINT);
                            }
                        },
                        (),
                    );
                }
            });
            let h_up: SceneHandle = ctx.get_scene().clone();
            let da_up: Rc<Cell<bool>> = drag_active.clone();
            let sx_up: Rc<Cell<f32>> = drag_start_x.clone();
            ctx.on_pointer_up(target_node, move |ev: &UiEvent| {
                if !da_up.replace(false) {
                    return;
                }
                let dx: f32 = match ev.get_kind() {
                    UiEventKind::PointerUp(pt) => pt.get_x() - sx_up.get(),
                    _ => 0.0,
                };
                let vp_w: f32 = vp_sig.get_untracked().get_width().max(320.0);
                let def_w: f32 = (vp_w * 0.80).clamp(240.0, 340.0);
                let max_w: f32 = (vp_w * 0.92).clamp(def_w + 40.0, 560.0);
                let is_exp: bool = expanded.get_untracked();
                if dx > 52.0 {
                    if is_exp && dx < 150.0 {
                        let _: bool = expanded.set_if_changed(false);
                    } else {
                        let _: bool = expanded.set_if_changed(false);
                        let _: bool = open.set_if_changed(false);
                        return;
                    }
                } else if dx < -40.0 {
                    let _: bool = expanded.set_if_changed(true);
                }
                let target_w: f32 = if expanded.get_untracked() {
                    max_w
                } else {
                    def_w
                };
                h_up.with_write(
                    |tree: &mut SceneTree| {
                        if let Some(n) = tree.get_node_mut(panel) {
                            let st: &mut Style = n.get_mut_style();
                            st.get_mut_layout().set_width(Length::Px(target_w));
                            st.get_mut_paint().set_transform(Transform2D::default());
                            let layout_dirty: u16 = DIRTY_LAYOUT | DIRTY_SUBTREE_LAYOUT;
                            tree.mark_dirty(panel, layout_dirty | DIRTY_PAINT);
                        }
                    },
                    (),
                );
            });
        }
        let click_moved: Rc<Cell<bool>> = drag_moved.clone();
        ctx.on_click(scrim, move |ev: &UiEvent| {
            if click_moved.replace(false) {
                return;
            }
            if ev.get_target() == scrim {
                let _: bool = open.set_if_changed(false);
            }
        });
        let handle: SceneHandle = ctx.get_scene().clone();
        let handle_dim: SceneHandle = handle.clone();
        let prev_open: Rc<Cell<bool>> = Rc::new(Cell::new(init_open));
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            let _: Effect = EuvOverlayStack::bind_signal(open, DRAWER_OVERLAY_NAME, Z_INDEX_DRAWER);
            let _: Effect = Effect::new(move || {
                let vp_w: f32 = vp_sig.get().get_width().max(320.0);
                let is_exp: bool = expanded.get();
                let def_w: f32 = (vp_w * 0.80).clamp(240.0, 340.0);
                let max_w: f32 = (vp_w * 0.92).clamp(def_w + 40.0, 560.0);
                let target_w: f32 = if is_exp { max_w } else { def_w };
                handle_dim.with_write(
                    |tree: &mut SceneTree| {
                        let next_len: Length = Length::Px(target_w);
                        let changed: bool = if let Some(n) = tree.get_node_mut(panel) {
                            if n.style_ref().get_layout().get_width() != next_len {
                                n.get_mut_style().get_mut_layout().set_width(next_len);
                                true
                            } else {
                                false
                            }
                        } else {
                            false
                        };
                        if changed {
                            let layout_dirty: u16 = DIRTY_LAYOUT | DIRTY_SUBTREE_LAYOUT;
                            tree.mark_dirty(panel, layout_dirty | DIRTY_PAINT);
                        }
                    },
                    (),
                );
            });
            Effect::new(move || {
                let is_open: bool = open.get();
                let was_open: bool = prev_open.replace(is_open);
                if is_open == was_open {
                    return;
                }
                if is_open {
                    let _: bool = expanded.set_if_changed(false);
                    let vp_w: f32 = vp_sig.get_untracked().get_width().max(320.0);
                    let def_w: f32 = (vp_w * 0.80).clamp(240.0, 340.0);
                    handle.with_write(
                        |tree: &mut SceneTree| {
                            if let Some(n) = tree.get_node_mut(scrim) {
                                n.get_mut_style()
                                    .get_mut_paint()
                                    .set_pointer_events(PointerEvents::Auto);
                            }
                            if let Some(n) = tree.get_node_mut(panel) {
                                let st: &mut Style = n.get_mut_style();
                                st.get_mut_layout().set_width(Length::Px(def_w));
                                st.get_mut_paint().set_transform(Transform2D::default());
                            }
                            tree.start_transition(
                                NodeTransition::fade(scrim, 0.0, 1.0, 0.22, false)
                                    .with_easing(Easing::EaseOutCubic),
                            );
                            tree.start_transition(
                                NodeTransition::slide_fade(
                                    panel,
                                    Point::new(def_w, 0.0),
                                    Point::new(0.0, 0.0),
                                    1.0,
                                    1.0,
                                    0.26,
                                    false,
                                )
                                .with_easing(Easing::EaseOutQuint),
                            );
                        },
                        (),
                    );
                } else {
                    let _: bool = expanded.set_if_changed(false);
                    let vp_w: f32 = vp_sig.get_untracked().get_width().max(320.0);
                    let slide_w: f32 = (vp_w * 0.92).clamp(340.0, 560.0);
                    handle.with_write(
                        |tree: &mut SceneTree| {
                            if let Some(n) = tree.get_node_mut(scrim) {
                                n.get_mut_style()
                                    .get_mut_paint()
                                    .set_pointer_events(PointerEvents::None);
                            }
                            tree.start_transition(
                                NodeTransition::slide_fade(
                                    panel,
                                    Point::new(0.0, 0.0),
                                    Point::new(slide_w, 0.0),
                                    1.0,
                                    1.0,
                                    0.20,
                                    false,
                                )
                                .with_easing(Easing::EaseOutCubic),
                            );
                            tree.start_transition(
                                NodeTransition::fade(scrim, 1.0, 0.0, 0.20, true)
                                    .with_easing(Easing::EaseOutCubic),
                            );
                        },
                        (),
                    );
                }
            })
        });
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                tree.attach_scope(scrim, scope);
            },
            (),
        );
        scrim
    }
}

/// Declarative `#[component]` implementation for `EuvSheet`.
#[component]
/// Inherent implementation of [`EuvSheet`].
impl EuvSheet {
    /// Renders `EuvSheet` into the ambient framework-managed `Context`.
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
        let vp_sig: Signal<Size> = ctx.get_viewport_size();
        let open: Signal<bool> = self.get_open();
        let body: NodeId = self.get_body();
        let Self { title, .. } = self;
        let title_owned: String = title.clone();
        let init_open: bool = open.get_untracked();
        let expanded: Signal<bool> = Signal::new(false);
        let drag_active: Rc<Cell<bool>> = Rc::new(Cell::new(false));
        let drag_start_y: Rc<Cell<f32>> = Rc::new(Cell::new(0.0));
        let drag_base_h: Rc<Cell<f32>> = Rc::new(Cell::new(300.0));
        let drag_moved: Rc<Cell<bool>> = Rc::new(Cell::new(false));
        let mut panel: NodeId = NodeId::default();
        let mut grabber: NodeId = NodeId::default();
        let sheet_r: f32 = g.get_radius_xl();
        let mut sheet_panel_base: Style = Style::default().column();
        sheet_panel_base
            .get_mut_paint()
            .set_radius(CornerRadius::new(sheet_r, sheet_r, 0.0, 0.0));
        let scrim: NodeId = view! {
            portal {
                style: Style::default().column()
                display: if init_open { Display::Flex } else { Display::None }
                position: Position::Fixed
                inset: (Some(0.0), Some(0.0), Some(0.0), Some(0.0))
                width: Length::Percent(100.0)
                height: Length::Percent(100.0)
                justify: JustifyContent::End
                background: p.get_overlay()
                stop_scroll_propagation: true
                z_index: Z_INDEX_DRAWER
                column {
                    node_ref: panel
                    style: sheet_panel_base
                    width: Length::Percent(100.0)
                    height: Length::Px(300.0)
                    max_height: Length::Percent(90.0)
                    background: p.get_card()
                    border: (g.get_border_width(), p.get_border())
                    shadow: g.get_shadow_lg()
                    overflow: Overflow::Hidden
                    row {
                        node_ref: grabber
                        width: Length::Percent(100.0)
                        height: Length::Px(28.0)
                        flex_shrink: 0.0
                        justify: JustifyContent::Center
                        align: AlignItems::Center
                        cursor: CursorIcon::NsResize
                        stop_scroll_propagation: true
                        stop_event_propagation: true
                        view {
                            width: Length::Px(46.0)
                            height: Length::Px(5.0)
                            radius: 99.0
                            background: p.get_muted_foreground().with_alpha(128)
                        }
                    }
                    scroll {
                        style: Style::default().column()
                        flex_grow: 1.0
                        width: Length::Percent(100.0)
                        padding: Edges::new(SPACE_XS, SPACE_XL, SPACE_XL, SPACE_XL)
                        gap: SPACE_MD
                        stop_scroll_propagation: true
                        text {
                            width: Length::Percent(100.0)
                            font_size: g.get_font_heading()
                            font_weight: FontWeight::Bold
                            color: p.get_card_foreground()
                            { title_owned }
                        }
                        { body }
                    }
                }
            }
        };
        let drag_down_active: Rc<Cell<bool>> = drag_active.clone();
        let drag_down_y: Rc<Cell<f32>> = drag_start_y.clone();
        let drag_down_h: Rc<Cell<f32>> = drag_base_h.clone();
        let drag_down_moved: Rc<Cell<bool>> = drag_moved.clone();
        let handle_down: SceneHandle = ctx.get_scene().clone();
        ctx.on_pointer_down(grabber, move |ev: &UiEvent| {
            if let UiEventKind::PointerDown(pt) = ev.get_kind() {
                let vp_h: f32 = vp_sig.get_untracked().get_height().max(320.0);
                let def_h: f32 = (vp_h * 0.44).clamp(240.0, 360.0);
                let cur_h: f32 = handle_down
                    .node_screen_bounds(panel)
                    .map(|r: Rect| r.get_height())
                    .filter(|h: &f32| *h > 40.0)
                    .unwrap_or(def_h);
                drag_down_active.set(true);
                drag_down_y.set(pt.get_y());
                drag_down_h.set(cur_h);
                drag_down_moved.set(false);
            }
        });
        for target_node in [grabber, panel, scrim].into_iter() {
            let h_move: SceneHandle = ctx.get_scene().clone();
            let da_move: Rc<Cell<bool>> = drag_active.clone();
            let sy_move: Rc<Cell<f32>> = drag_start_y.clone();
            let bh_move: Rc<Cell<f32>> = drag_base_h.clone();
            let dm_move: Rc<Cell<bool>> = drag_moved.clone();
            ctx.on_pointer_move(target_node, move |ev: &UiEvent| {
                if !da_move.get() {
                    return;
                }
                if let UiEventKind::PointerMove(pt) = ev.get_kind() {
                    let dy: f32 = pt.get_y() - sy_move.get();
                    if dy.abs() > 4.0 {
                        dm_move.set(true);
                    }
                    let vp_h: f32 = vp_sig.get_untracked().get_height().max(320.0);
                    let def_h: f32 = (vp_h * 0.44).clamp(240.0, 360.0);
                    let max_h: f32 = (vp_h * 0.86).clamp(def_h + 60.0, 760.0);
                    let raw_h: f32 = bh_move.get() - dy;
                    let (next_h, ty): (f32, f32) = if raw_h >= def_h {
                        (raw_h.min(max_h), 0.0)
                    } else {
                        (def_h, (def_h - raw_h).max(0.0))
                    };
                    h_move.with_write(
                        |tree: &mut SceneTree| {
                            if let Some(n) = tree.get_node_mut(panel) {
                                let st: &mut Style = n.get_mut_style();
                                st.get_mut_layout().set_height(Length::Px(next_h));
                                st.get_mut_paint()
                                    .set_transform(Transform2D::new(0.0, ty, 1.0, 1.0, 0.0));
                                let layout_dirty: u16 = DIRTY_LAYOUT | DIRTY_SUBTREE_LAYOUT;
                                tree.mark_dirty(panel, layout_dirty | DIRTY_PAINT);
                            }
                        },
                        (),
                    );
                }
            });
            let h_up: SceneHandle = ctx.get_scene().clone();
            let da_up: Rc<Cell<bool>> = drag_active.clone();
            let sy_up: Rc<Cell<f32>> = drag_start_y.clone();
            ctx.on_pointer_up(target_node, move |ev: &UiEvent| {
                if !da_up.replace(false) {
                    return;
                }
                let dy: f32 = match ev.get_kind() {
                    UiEventKind::PointerUp(pt) => pt.get_y() - sy_up.get(),
                    _ => 0.0,
                };
                let vp_h: f32 = vp_sig.get_untracked().get_height().max(320.0);
                let def_h: f32 = (vp_h * 0.44).clamp(240.0, 360.0);
                let max_h: f32 = (vp_h * 0.86).clamp(def_h + 60.0, 760.0);
                let is_exp: bool = expanded.get_untracked();
                if dy > 52.0 {
                    if is_exp && dy < 150.0 {
                        let _: bool = expanded.set_if_changed(false);
                    } else {
                        let _: bool = expanded.set_if_changed(false);
                        let _: bool = open.set_if_changed(false);
                        return;
                    }
                } else if dy < -40.0 {
                    let _: bool = expanded.set_if_changed(true);
                }
                let target_h: f32 = if expanded.get_untracked() {
                    max_h
                } else {
                    def_h
                };
                h_up.with_write(
                    |tree: &mut SceneTree| {
                        if let Some(n) = tree.get_node_mut(panel) {
                            let st: &mut Style = n.get_mut_style();
                            st.get_mut_layout().set_height(Length::Px(target_h));
                            st.get_mut_paint().set_transform(Transform2D::default());
                            let layout_dirty: u16 = DIRTY_LAYOUT | DIRTY_SUBTREE_LAYOUT;
                            tree.mark_dirty(panel, layout_dirty | DIRTY_PAINT);
                        }
                    },
                    (),
                );
            });
        }
        let click_moved: Rc<Cell<bool>> = drag_moved.clone();
        ctx.on_click(scrim, move |ev: &UiEvent| {
            if click_moved.replace(false) {
                return;
            }
            if ev.get_target() == scrim {
                let _: bool = open.set_if_changed(false);
            }
        });
        let handle: SceneHandle = ctx.get_scene().clone();
        let handle_dim: SceneHandle = handle.clone();
        let prev_open: Rc<Cell<bool>> = Rc::new(Cell::new(init_open));
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            let _: Effect = EuvOverlayStack::bind_signal(open, SHEET_OVERLAY_NAME, Z_INDEX_DRAWER);
            let _: Effect = Effect::new(move || {
                let vp_h: f32 = vp_sig.get().get_height().max(320.0);
                let is_exp: bool = expanded.get();
                let def_h: f32 = (vp_h * 0.44).clamp(240.0, 360.0);
                let max_h: f32 = (vp_h * 0.86).clamp(def_h + 60.0, 760.0);
                let target_h: f32 = if is_exp { max_h } else { def_h };
                handle_dim.with_write(
                    |tree: &mut SceneTree| {
                        let next_len: Length = Length::Px(target_h);
                        let changed: bool = if let Some(n) = tree.get_node_mut(panel) {
                            if n.style_ref().get_layout().get_height() != next_len {
                                n.get_mut_style().get_mut_layout().set_height(next_len);
                                true
                            } else {
                                false
                            }
                        } else {
                            false
                        };
                        if changed {
                            let layout_dirty: u16 = DIRTY_LAYOUT | DIRTY_SUBTREE_LAYOUT;
                            tree.mark_dirty(panel, layout_dirty | DIRTY_PAINT);
                        }
                    },
                    (),
                );
            });
            Effect::new(move || {
                let is_open: bool = open.get();
                let was_open: bool = prev_open.replace(is_open);
                if is_open == was_open {
                    return;
                }
                if is_open {
                    let _: bool = expanded.set_if_changed(false);
                    let vp_h: f32 = vp_sig.get_untracked().get_height().max(320.0);
                    let def_h: f32 = (vp_h * 0.44).clamp(240.0, 360.0);
                    handle.with_write(
                        |tree: &mut SceneTree| {
                            if let Some(n) = tree.get_node_mut(scrim) {
                                n.get_mut_style()
                                    .get_mut_paint()
                                    .set_pointer_events(PointerEvents::Auto);
                            }
                            if let Some(n) = tree.get_node_mut(panel) {
                                let st: &mut Style = n.get_mut_style();
                                st.get_mut_layout().set_height(Length::Px(def_h));
                                st.get_mut_paint().set_transform(Transform2D::default());
                            }
                            tree.start_transition(
                                NodeTransition::fade(scrim, 0.0, 1.0, 0.22, false)
                                    .with_easing(Easing::EaseOutCubic),
                            );
                            tree.start_transition(
                                NodeTransition::slide_fade(
                                    panel,
                                    Point::new(0.0, def_h),
                                    Point::new(0.0, 0.0),
                                    1.0,
                                    1.0,
                                    0.26,
                                    false,
                                )
                                .with_easing(Easing::EaseOutQuint),
                            );
                        },
                        (),
                    );
                } else {
                    let _: bool = expanded.set_if_changed(false);
                    let vp_h: f32 = vp_sig.get_untracked().get_height().max(320.0);
                    let slide_h: f32 = (vp_h * 0.86).clamp(300.0, 760.0);
                    handle.with_write(
                        |tree: &mut SceneTree| {
                            if let Some(n) = tree.get_node_mut(scrim) {
                                n.get_mut_style()
                                    .get_mut_paint()
                                    .set_pointer_events(PointerEvents::None);
                            }
                            tree.start_transition(
                                NodeTransition::slide_fade(
                                    panel,
                                    Point::new(0.0, 0.0),
                                    Point::new(0.0, slide_h),
                                    1.0,
                                    1.0,
                                    0.20,
                                    false,
                                )
                                .with_easing(Easing::EaseOutCubic),
                            );
                            tree.start_transition(
                                NodeTransition::fade(scrim, 1.0, 0.0, 0.20, true)
                                    .with_easing(Easing::EaseOutCubic),
                            );
                        },
                        (),
                    );
                }
            })
        });
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                tree.attach_scope(scrim, scope);
            },
            (),
        );
        scrim
    }
}

/// Declarative `#[component]` implementation for `EuvPopover`.
#[component]
/// Inherent implementation of [`EuvPopover`].
impl EuvPopover {
    /// Renders `EuvPopover` into the ambient framework-managed `Context`.
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
        let open: Signal<bool> = self.get_open();
        let trigger: NodeId = self.get_trigger();
        let content: NodeId = self.get_content();
        let init_open: bool = open.get_untracked();
        let mut card: NodeId = NodeId::default();
        let wrap: NodeId = view! {
            column {
                align: AlignItems::Start
                gap: SPACE_XS
                { trigger }
                column {
                    node_ref: card
                    display: if init_open { Display::Flex } else { Display::None }
                    padding: Edges::all(SPACE_MD)
                    background: p.get_card()
                    border: (g.get_border_width(), p.get_border())
                    radius: g.get_radius_md()
                    shadow: g.get_shadow_md()
                    z_index: Z_INDEX_FLOATING
                    { content }
                }
            }
        };
        let handle: SceneHandle = ctx.get_scene().clone();
        let prev_open: Rc<Cell<bool>> = Rc::new(Cell::new(init_open));
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            let _: Effect =
                EuvOverlayStack::bind_signal(open, POPOVER_OVERLAY_NAME, Z_INDEX_DRAWER - 10);
            Effect::new(move || {
                let is_open: bool = open.get();
                let was_open: bool = prev_open.replace(is_open);
                if is_open == was_open {
                    return;
                }
                if is_open {
                    handle.start_transition(
                        NodeTransition::slide_fade(
                            card,
                            Point::new(0.0, -8.0),
                            Point::new(0.0, 0.0),
                            0.0,
                            1.0,
                            0.24,
                            false,
                        )
                        .with_scale(0.95, 1.0)
                        .with_easing(Easing::SpringSnappy),
                    );
                } else {
                    handle.start_transition(
                        NodeTransition::slide_fade(
                            card,
                            Point::new(0.0, 0.0),
                            Point::new(0.0, -5.0),
                            1.0,
                            0.0,
                            0.14,
                            true,
                        )
                        .with_scale(1.0, 0.96)
                        .with_easing(Easing::EaseOutCubic),
                    );
                }
            })
        });
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                tree.attach_scope(card, scope);
            },
            (),
        );
        ctx.on_click(trigger, move |_: &UiEvent| {
            let _: bool = open.update(|v: &mut bool| *v = !*v);
        });
        wrap
    }
}

/// Fluent configuration builders for `EuvTooltip`.
impl EuvTooltip {
    /// Controls whether the tooltip stays permanently visible or toggles reactively on pointer hover.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source tooltip.
    /// - `bool` - When `true`, renders permanently visible.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated tooltip.
    #[must_use]
    pub fn always_visible(mut self, always_visible: bool) -> Self {
        self.set_always_visible(always_visible);
        self
    }
}

/// Declarative `#[component]` implementation for `EuvTooltip`.
#[component]
/// Inherent implementation of [`EuvTooltip`].
impl EuvTooltip {
    /// Renders `EuvTooltip` into the ambient framework-managed `Context`.
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
        let trigger: NodeId = self.get_trigger();
        let pinned: bool = self.get_always_visible();
        let Self { text, .. } = self;
        let text_owned: String = text.clone();
        let hovered: Signal<bool> = Signal::new(pinned);
        let mut tip: NodeId = NodeId::default();
        let wrap: NodeId = view! {
            column {
                align: AlignItems::Start
                gap: SPACE_XS
                { trigger }
                row {
                    node_ref: tip
                    display: if pinned { Display::Flex } else { Display::None }
                    padding: Edges::symmetric(SPACE_XS, SPACE_SM)
                    background: p.get_foreground()
                    radius: g.get_radius_sm()
                    shadow: g.get_shadow_md()
                    pointer_events: PointerEvents::None
                    z_index: Z_INDEX_TOOLTIP
                    text {
                        font_size: 11.0
                        color: p.get_background()
                        { text_owned }
                    }
                }
            }
        };
        if !pinned {
            ctx.on_pointer_enter(wrap, move |_: &UiEvent| {
                let _: bool = hovered.set_if_changed(true);
            });
            ctx.on_pointer_leave(wrap, move |_: &UiEvent| {
                let _: bool = hovered.set_if_changed(false);
            });
            let handle_tip: SceneHandle = ctx.get_scene().clone();
            let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
                Effect::new(move || {
                    let vis: Display = if hovered.get() {
                        Display::Flex
                    } else {
                        Display::None
                    };
                    handle_tip.with_write(
                        |tree: &mut SceneTree| {
                            let changed: bool = if let Some(n) = tree.get_node_mut(tip) {
                                if n.style_ref().get_layout().get_display() != vis {
                                    n.get_mut_style().get_mut_layout().set_display(vis);
                                    true
                                } else {
                                    false
                                }
                            } else {
                                false
                            };
                            if changed {
                                let dirty_lay: u16 = DIRTY_LAYOUT | DIRTY_SUBTREE_LAYOUT;
                                tree.mark_dirty(tip, dirty_lay | DIRTY_PAINT);
                            }
                        },
                        (),
                    );
                })
            });
            ctx.get_scene().with_write(
                |tree: &mut SceneTree| {
                    tree.attach_scope(tip, scope);
                },
                (),
            );
        }
        wrap
    }
}

/// Fluent controlled-state builders for `EuvDropdown`.
impl EuvDropdown {
    /// Binds a controlled reactive open flag signal fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source dropdown.
    /// - `Signal<bool>` - Controlled open state signal.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated dropdown.
    #[must_use]
    pub fn open(mut self, open: Signal<bool>) -> Self {
        self.set_open(Some(open));
        self
    }

    /// Binds a reactive signal capturing the index of the most recently clicked menu item.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source dropdown.
    /// - `Signal<Option<usize>>` - Controlled selected item index signal.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated dropdown.
    #[must_use]
    pub fn selected(mut self, last_selected: Signal<Option<usize>>) -> Self {
        self.set_last_selected(Some(last_selected));
        self
    }
}

/// Declarative `#[component]` implementation for `EuvDropdown`.
#[component]
/// Inherent implementation of [`EuvDropdown`].
impl EuvDropdown {
    /// Renders `EuvDropdown` into the ambient framework-managed `Context`.
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
            label,
            items,
            open: open_opt,
            last_selected: sel_opt,
            ..
        } = self;
        let open: Signal<bool> = open_opt.unwrap_or_else(|| Signal::new(false));
        let last_selected: Signal<Option<usize>> = sel_opt.unwrap_or_else(|| Signal::new(None));
        let label_owned: String = label.clone();
        let init_open: bool = open.get_untracked();
        let mut menu: NodeId = NodeId::default();
        let chevron_color: Color = p.get_muted_foreground();
        let on_toggle = move |_: &UiEvent| {
            let _: bool = open.update(|v: &mut bool| *v = !*v);
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
        let menu_items: Vec<NodeId> = items
            .iter()
            .enumerate()
            .map(|(idx, item_label): (usize, &String)| {
                let on_pick = move |_: &UiEvent| {
                    let _: bool = last_selected.set_if_changed(Some(idx));
                    let _: bool = open.set_if_changed(false);
                };
                view! {
                    row {
                        width: Length::Percent(100.0)
                        height: Length::Px(g.get_control_sm())
                        padding: Edges::symmetric(0.0, SPACE_MD)
                        align: AlignItems::Center
                        radius: g.get_radius_sm()
                        overflow: Overflow::Hidden
                        cursor: CursorIcon::Pointer
                        role: A11yRole::MenuItem
                        a11y_label: item_label.clone()
                        on_click: on_pick
                        text {
                            width: Length::Percent(100.0)
                            font_size: 13.0
                            color: p.get_foreground()
                            text_overflow: TextOverflow::Ellipsis
                            { item_label.clone() }
                        }
                    }
                }
            })
            .collect();
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
                    a11y_label: label_owned.clone()
                    on_click: on_toggle
                    text {
                        flex_grow: 1.0
                        flex_shrink: 1.0
                        min_width: Length::Px(0.0)
                        font_size: 14.0
                        font_weight: FontWeight::Medium
                        color: p.get_foreground()
                        text_overflow: TextOverflow::Ellipsis
                        { label_owned }
                    }
                    canvas {
                        width: Length::Px(12.0)
                        height: Length::Px(12.0)
                        flex_shrink: 0.0
                        painter: paint_chevron
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
                    overflow: Overflow::Hidden
                    z_index: Z_INDEX_FLOATING
                    role: A11yRole::Menu
                    ..menu_items
                }
            }
        };
        let handle: SceneHandle = ctx.get_scene().clone();
        let prev_open: Rc<Cell<bool>> = Rc::new(Cell::new(init_open));
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            let _: Effect =
                EuvOverlayStack::bind_signal(open, DROPDOWN_OVERLAY_NAME, Z_INDEX_DRAWER - 10);
            Effect::new(move || {
                let is_open: bool = open.get();
                let was_open: bool = prev_open.replace(is_open);
                if is_open == was_open {
                    return;
                }
                if is_open {
                    handle.start_transition(
                        NodeTransition::slide_fade(
                            menu,
                            Point::new(0.0, -8.0),
                            Point::new(0.0, 0.0),
                            0.0,
                            1.0,
                            0.24,
                            false,
                        )
                        .with_scale(0.95, 1.0)
                        .with_easing(Easing::SpringSnappy),
                    );
                } else {
                    handle.start_transition(
                        NodeTransition::slide_fade(
                            menu,
                            Point::new(0.0, 0.0),
                            Point::new(0.0, -5.0),
                            1.0,
                            0.0,
                            0.14,
                            true,
                        )
                        .with_scale(1.0, 0.96)
                        .with_easing(Easing::EaseOutCubic),
                    );
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

/// Fluent tone builder for `EuvNotification`.
impl EuvNotification {
    /// Sets the semantic tone accent border fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source notification.
    /// - `EuvTone` - Target semantic tone.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated notification.
    #[must_use]
    pub fn tone(mut self, tone: EuvTone) -> Self {
        self.set_tone(tone);
        self
    }
}

/// Declarative `#[component]` implementation for `EuvNotification`.
#[component]
/// Inherent implementation of [`EuvNotification`].
impl EuvNotification {
    /// Renders `EuvNotification` into the ambient framework-managed `Context`.
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
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let (accent, _): (Color, Color) = self.get_tone().colors(p);
        let visible: Signal<bool> = self.get_visible();
        let Self { title, message, .. } = self;
        let title_owned: String = title.clone();
        let message_owned: String = message.clone();
        let init_vis: bool = visible.get_untracked();
        let card: NodeId = view! {
            portal {
                style: Style::default().column()
                display: if init_vis { Display::Flex } else { Display::None }
                position: Position::Fixed
                inset: (Some(SPACE_LG), Some(SPACE_LG), None, None)
                width: Length::Percent(90.0)
                max_width: Length::Px(340.0)
                padding: Edges::all(SPACE_MD)
                gap: SPACE_XS
                background: p.get_card()
                border: (g.get_border_width(), accent)
                radius: g.get_radius_lg()
                shadow: g.get_shadow_lg()
                overflow: Overflow::Hidden
                z_index: Z_INDEX_TOAST
                role: A11yRole::Alert
                a11y_label: title_owned.clone()
                text {
                    font_size: 14.0
                    font_weight: FontWeight::Bold
                    color: p.get_card_foreground()
                    { title_owned }
                }
                text {
                    font_size: 12.0
                    color: p.get_muted_foreground()
                    { message_owned }
                }
            }
        };
        let handle: SceneHandle = ctx.get_scene().clone();
        let prev_vis: Rc<Cell<bool>> = Rc::new(Cell::new(init_vis));
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            Effect::new(move || {
                let is_vis: bool = visible.get();
                let was_vis: bool = prev_vis.replace(is_vis);
                if is_vis == was_vis {
                    return;
                }
                if is_vis {
                    handle.start_transition(
                        NodeTransition::slide_fade(
                            card,
                            Point::new(36.0, 0.0),
                            Point::new(0.0, 0.0),
                            0.0,
                            1.0,
                            0.28,
                            false,
                        )
                        .with_scale(0.95, 1.0)
                        .with_easing(Easing::SpringSnappy),
                    );
                } else {
                    handle.start_transition(
                        NodeTransition::slide_fade(
                            card,
                            Point::new(0.0, 0.0),
                            Point::new(28.0, 0.0),
                            1.0,
                            0.0,
                            0.18,
                            true,
                        )
                        .with_scale(1.0, 0.96)
                        .with_easing(Easing::EaseOutCubic),
                    );
                }
            })
        });
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                tree.attach_scope(card, scope);
            },
            (),
        );
        card
    }
}

/// Declarative `#[component]` implementation for `EuvLoading`.
#[component]
/// Inherent implementation of [`EuvLoading`].
impl EuvLoading {
    /// Renders `EuvLoading` into the ambient framework-managed `Context`.
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
        let tokens: ThemeTokens = self.get_tokens();
        let p: ThemePalette = tokens.get_palette();
        let active: Signal<bool> = self.get_active();
        let Self { label, .. } = self;
        let label_owned: String = label.clone();
        let init_active: bool = active.get_untracked();
        let portal: NodeId = view! {
            portal {
                style: Style::default().column()
                display: if init_active { Display::Flex } else { Display::None }
                position: Position::Fixed
                inset: (Some(0.0), Some(0.0), Some(0.0), Some(0.0))
                width: Length::Percent(100.0)
                height: Length::Percent(100.0)
                justify: JustifyContent::Center
                align: AlignItems::Center
                gap: SPACE_MD
                background: p.get_overlay()
                stop_scroll_propagation: true
                z_index: Z_INDEX_MODAL
                EuvSpinner {
                    size: 32.0
                    tokens: tokens
                }
                text {
                    font_size: 14.0
                    font_weight: FontWeight::Medium
                    color: p.get_foreground()
                    { label_owned }
                }
            }
        };
        ctx.on_click(portal, move |ev: &UiEvent| {
            if ev.get_target() == portal {
                let _: bool = active.set_if_changed(false);
            }
        });
        let handle: SceneHandle = ctx.get_scene().clone();
        let prev_active: Rc<Cell<bool>> = Rc::new(Cell::new(init_active));
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            let _: Effect =
                EuvOverlayStack::bind_signal(active, LOADING_OVERLAY_NAME, Z_INDEX_MODAL);
            Effect::new(move || {
                let is_on: bool = active.get();
                let was_on: bool = prev_active.replace(is_on);
                if is_on == was_on {
                    return;
                }
                if is_on {
                    handle.with_write(
                        |tree: &mut SceneTree| {
                            if let Some(n) = tree.get_node_mut(portal) {
                                n.get_mut_style()
                                    .get_mut_paint()
                                    .set_pointer_events(PointerEvents::Auto);
                            }
                            tree.start_transition(NodeTransition::fade(
                                portal, 0.0, 1.0, 0.18, false,
                            ));
                        },
                        (),
                    );
                } else {
                    handle.with_write(
                        |tree: &mut SceneTree| {
                            if let Some(n) = tree.get_node_mut(portal) {
                                n.get_mut_style()
                                    .get_mut_paint()
                                    .set_pointer_events(PointerEvents::None);
                            }
                            tree.start_transition(NodeTransition::fade(
                                portal, 1.0, 0.0, 0.18, true,
                            ));
                        },
                        (),
                    );
                }
            })
        });
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                tree.attach_scope(portal, scope);
            },
            (),
        );
        portal
    }
}
