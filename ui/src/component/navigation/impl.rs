use super::*;

/// Fluent section builder for `EuvNavItem`.
impl EuvNavItem {
    /// Assigns a sidebar category group label rendered above the first item in that group.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source navigation item.
    /// - `&str` - Category section title.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated navigation item.
    #[must_use]
    pub fn section(mut self, group: &str) -> Self {
        self.set_section(group.to_string());
        self
    }
}

/// Declarative `#[component]` implementation for `EuvSidebarNav`.
#[component]
/// Inherent implementation of [`EuvSidebarNav`].
impl EuvSidebarNav {
    /// Renders `EuvSidebarNav` into the ambient framework-managed `Context`.
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
        let base_tokens: ThemeTokens = self.get_tokens();
        let init_tok: ThemeTokens = live_tokens_untracked(base_tokens);
        let init_g: ThemeGeometry = init_tok.get_geometry();
        let active_route: Signal<String> = self.get_active_route();
        let Self { items, .. } = self;
        let route_paths: RoutePathList = items
            .iter()
            .map(|item: &EuvNavItem| item.get_path().clone())
            .collect();
        let paths_for_sel: RoutePathList = Rc::clone(&route_paths);
        let mut children: Vec<NodeId> = Vec::with_capacity(items.len() * 2);
        let mut last_section: &str = "";
        let mut bound_items: Vec<(usize, NodeId, NodeId)> = Vec::with_capacity(items.len());
        for (idx, item) in items.iter().enumerate() {
            let sec: &str = item.get_section().as_str();
            if !sec.is_empty() && sec != last_section {
                last_section = sec;
                let top_pad: f32 = if idx == 0 { 2.0 } else { 12.0 };
                let sec_text_style = move || -> Style {
                    let live_p: ThemePalette = use_live_tokens(base_tokens).get_palette();
                    let mut st: Style = Style::default();
                    st.get_mut_text()
                        .set_font_size(11.0)
                        .set_weight(FontWeight::SemiBold)
                        .set_color(live_p.get_muted_foreground());
                    st
                };
                children.push(view! {
                    row {
                        width: Length::Percent(100.0)
                        padding: Edges::new(top_pad, SPACE_SM, 4.0, SPACE_SM)
                        text {
                            bind_style: sec_text_style
                            { sec }
                        }
                    }
                });
            }
            let click_path: String = item.get_path().clone();
            let on_item_click = move |_: &UiEvent| {
                let _: bool = active_route.set_if_changed(click_path.clone());
            };
            let lbl_style = move || -> Style {
                let live_p: ThemePalette = use_live_tokens(base_tokens).get_palette();
                let mut st: Style = Style::default();
                st.get_mut_text()
                    .set_font_size(13.5)
                    .set_weight(FontWeight::SemiBold)
                    .set_color(live_p.get_foreground())
                    .set_overflow(TextOverflow::Clip);
                st
            };
            let mut ind_node: NodeId = NodeId::default();
            let row_node: NodeId = view! {
                row {
                    width: Length::Percent(100.0)
                    height: Length::Px(init_g.get_control_sm().max(34.0))
                    align: AlignItems::Center
                    gap: SPACE_SM
                    padding: Edges::symmetric(0.0, SPACE_SM)
                    radius: init_g.get_radius_md()
                    background: COLOR_TRANSPARENT
                    cursor: CursorIcon::Pointer
                    on_click: on_item_click
                    view {
                        node_ref: ind_node
                        width: Length::Px(3.5)
                        height: Length::Px(18.0)
                        radius: 99.0
                        background: COLOR_TRANSPARENT
                    }
                    text {
                        bind_style: lbl_style
                        { item.get_label().as_str() }
                    }
                }
            };
            bound_items.push((idx, row_node, ind_node));
            children.push(row_node);
        }
        let root: NodeId = view! {
            column {
                width: Length::Percent(100.0)
                gap: 2.0
                ..children
            }
        };
        let scene: SceneHandle = ctx.get_scene().clone();
        let (nav_scope, _): (Scope, ()) = Scope::run(|_: Scope| {
            let active_sel: Rc<Selector<Option<usize>>> =
                Rc::new(Selector::new(move || -> Option<usize> {
                    let cur: String = active_route.get();
                    paths_for_sel
                        .iter()
                        .position(|p: &String| p.as_str() == cur.as_str())
                }));
            let sel_cleanup: Rc<Selector<Option<usize>>> = Rc::clone(&active_sel);
            on_cleanup(move || {
                sel_cleanup.dispose();
            });
            for (idx, row_node, ind_node) in bound_items.into_iter() {
                let sel_item: Rc<Selector<Option<usize>>> = Rc::clone(&active_sel);
                let scene_item: SceneHandle = scene.clone();
                let mut prev_tok: ThemeTokens = init_tok;
                let mut prev_active: bool = false;
                let mut has_synced: bool = false;
                let _: Effect = Effect::new(move || {
                    let is_active: bool = sel_item.is_selected(Some(idx));
                    let live_tok: ThemeTokens = use_live_tokens(base_tokens);
                    let theme_changed: bool = has_synced && live_tok != prev_tok;
                    if has_synced && !theme_changed && is_active == prev_active {
                        return;
                    }
                    prev_tok = live_tok;
                    prev_active = is_active;
                    has_synced = true;
                    let live_p: ThemePalette = live_tok.get_palette();
                    let live_g: ThemeGeometry = live_tok.get_geometry();
                    let row_bg: Color = if is_active {
                        live_p.get_primary().with_alpha(24)
                    } else {
                        COLOR_TRANSPARENT
                    };
                    let ind_bg: Color = if is_active {
                        live_p.get_primary()
                    } else {
                        COLOR_TRANSPARENT
                    };
                    scene_item.with_write(
                        |tree: &mut SceneTree| {
                            if let Some(node) = tree.get_node_mut(row_node) {
                                let mut st: Style = node.get_style();
                                if theme_changed {
                                    st.get_mut_layout()
                                        .set_height(Length::Px(live_g.get_control_sm().max(34.0)));
                                    st.get_mut_paint().set_radius(CornerRadius::all(
                                        live_g.get_radius_md().max(0.0),
                                    ));
                                }
                                st.get_mut_paint().set_background(Some(row_bg));
                                node.set_style(st);
                            }
                            if let Some(node) = tree.get_node_mut(ind_node) {
                                let mut st: Style = node.get_style();
                                st.get_mut_paint().set_background(Some(ind_bg));
                                node.set_style(st);
                            }
                            let dirty_layout: u16 = DIRTY_LAYOUT | DIRTY_SUBTREE_LAYOUT;
                            let row_dirty: u16 = if theme_changed {
                                dirty_layout | DIRTY_PAINT
                            } else {
                                DIRTY_PAINT
                            };
                            tree.mark_dirty(row_node, row_dirty);
                            tree.mark_dirty(ind_node, DIRTY_PAINT);
                        },
                        (),
                    );
                });
            }
        });
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                tree.attach_scope(root, nav_scope);
            },
            (),
        );
        root
    }
}

/// Declarative `#[component]` implementation for `EuvTabs`.
#[component]
/// Inherent implementation of [`EuvTabs`].
impl EuvTabs {
    /// Renders `EuvTabs` into the ambient framework-managed `Context`.
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
        let base_tokens: ThemeTokens = self.get_tokens();
        let init_tok: ThemeTokens = live_tokens_untracked(base_tokens);
        let init_g: ThemeGeometry = init_tok.get_geometry();
        let active: Signal<usize> = self.get_active();
        let Self { labels, .. } = self;
        let bar_style = move || -> Style {
            let live_tok: ThemeTokens = use_live_tokens(base_tokens);
            let live_p: ThemePalette = live_tok.get_palette();
            let live_g: ThemeGeometry = live_tok.get_geometry();
            let mut st: Style = Style::default();
            st.get_mut_layout()
                .set_flex_direction(FlexDirection::Row)
                .set_flex_wrap(FlexWrap::Wrap)
                .set_width(Length::Percent(100.0))
                .set_row_gap(SPACE_XS)
                .set_column_gap(SPACE_XS)
                .set_padding(Edges::all(SPACE_XS));
            st.get_mut_paint()
                .set_background(Some(live_p.get_muted()))
                .set_radius(CornerRadius::all(live_g.get_radius_md().max(0.0)));
            st
        };
        let mut tab_nodes: Vec<NodeId> = Vec::with_capacity(labels.len());
        for (idx, label) in labels.iter().enumerate() {
            let on_tab_click = move |_: &UiEvent| {
                let _: bool = active.set_if_changed(idx);
            };
            let lbl_style = move || -> Style {
                let live_p: ThemePalette = use_live_tokens(base_tokens).get_palette();
                let mut st: Style = Style::default();
                st.get_mut_text()
                    .set_font_size(13.0)
                    .set_weight(FontWeight::Medium)
                    .set_color(live_p.get_foreground())
                    .set_align(TextAlign::Center)
                    .set_overflow(TextOverflow::Ellipsis);
                st
            };
            let tab_node: NodeId = view! {
                row {
                    height: Length::Px(init_g.get_control_sm())
                    justify: JustifyContent::Center
                    align: AlignItems::Center
                    padding: Edges::symmetric(0.0, SPACE_MD)
                    overflow: Overflow::Hidden
                    radius: init_g.get_radius_sm()
                    background: COLOR_TRANSPARENT
                    cursor: CursorIcon::Pointer
                    on_click: on_tab_click
                    text {
                        bind_style: lbl_style
                        { label.as_str() }
                    }
                }
            };
            tab_nodes.push(tab_node);
        }
        let bar_node: NodeId = view! {
            row {
                bind_style: bar_style
            }
        };
        if !tab_nodes.is_empty() {
            let mut ctx_append: Context = ctx.clone();
            ctx_append.append_many(bar_node, &tab_nodes);
        }
        let scene: SceneHandle = ctx.get_scene().clone();
        let (tabs_scope, _): (Scope, ()) = Scope::run(|_: Scope| {
            let active_sel: Rc<Selector<usize>> = Rc::new(Selector::new(move || active.get()));
            let sel_cleanup: Rc<Selector<usize>> = Rc::clone(&active_sel);
            on_cleanup(move || {
                sel_cleanup.dispose();
            });
            for (idx, tab_node) in tab_nodes.into_iter().enumerate() {
                let sel_tab: Rc<Selector<usize>> = Rc::clone(&active_sel);
                let scene_tab: SceneHandle = scene.clone();
                let mut prev_tok: ThemeTokens = init_tok;
                let mut prev_sel: bool = false;
                let mut has_synced: bool = false;
                let _: Effect = Effect::new(move || {
                    let selected: bool = sel_tab.is_selected(idx);
                    let live_tok: ThemeTokens = use_live_tokens(base_tokens);
                    let theme_changed: bool = has_synced && live_tok != prev_tok;
                    if has_synced && !theme_changed && selected == prev_sel {
                        return;
                    }
                    prev_tok = live_tok;
                    prev_sel = selected;
                    has_synced = true;
                    let live_p: ThemePalette = live_tok.get_palette();
                    let live_g: ThemeGeometry = live_tok.get_geometry();
                    scene_tab.with_write(
                        |tree: &mut SceneTree| {
                            if let Some(node) = tree.get_node_mut(tab_node) {
                                let mut st: Style = node.get_style();
                                if theme_changed {
                                    st.get_mut_layout()
                                        .set_height(Length::Px(live_g.get_control_sm()));
                                    st.get_mut_paint().set_radius(CornerRadius::all(
                                        live_g.get_radius_sm().max(0.0),
                                    ));
                                }
                                let paint: &mut PaintStyle = st.get_mut_paint();
                                paint
                                    .set_background(Some(if selected {
                                        live_p.get_card()
                                    } else {
                                        COLOR_TRANSPARENT
                                    }))
                                    .set_border(Border::new(
                                        if selected {
                                            live_g.get_border_width().max(0.0)
                                        } else {
                                            0.0
                                        },
                                        live_p.get_border(),
                                    ))
                                    .set_shadow(if selected {
                                        Some(live_g.get_shadow_xs())
                                    } else {
                                        None
                                    });
                                node.set_style(st);
                            }
                            let dirty_layout: u16 = DIRTY_LAYOUT | DIRTY_SUBTREE_LAYOUT;
                            let dirty: u16 = if theme_changed {
                                dirty_layout | DIRTY_PAINT
                            } else {
                                DIRTY_PAINT
                            };
                            tree.mark_dirty(tab_node, dirty);
                        },
                        (),
                    );
                });
            }
        });
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                tree.attach_scope(bar_node, tabs_scope);
            },
            (),
        );
        bar_node
    }
}

/// Declarative `#[component]` implementation for `EuvBottomNav`.
#[component]
/// Inherent implementation of [`EuvBottomNav`].
impl EuvBottomNav {
    /// Renders `EuvBottomNav` as a floating dock with disciplined design-token corner radii and a smooth monotonic sliding active-tab indicator tracking clicks, route switches, and live swipe drag progress.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component descriptor reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted bottom navigation bar container handle.
    fn render(&self) -> NodeId {
        let ctx: Context = current_context();
        let base_tokens: ThemeTokens = self.get_tokens();
        let init_p: ThemePalette = live_tokens_untracked(base_tokens).get_palette();
        let active_route: Signal<String> = self.get_active_route();
        let Self { items, .. } = self;
        let item_count: usize = items.len().max(1);
        let slot_pct: f32 = 100.0 / (item_count as f32);
        let route_paths: RoutePathList = items
            .iter()
            .map(|item: &EuvNavItem| item.get_path().clone())
            .collect();
        let initial_idx: Option<usize> = {
            let cur: String = active_route.get_untracked();
            route_paths
                .iter()
                .position(|path: &String| path.as_str() == cur.as_str())
        };
        let drag_sig: Signal<f32> = ctx.get_swipe_drag_offset();
        let vp_sig: Signal<Size> = ctx.get_viewport_size();
        let pill_radius_init: f32 = base_tokens
            .get_geometry()
            .get_radius_md()
            .clamp(NAV_INDICATOR_MIN_RADIUS, NAV_INDICATOR_MAX_RADIUS);
        let pill_style = move || -> Style {
            let live_tok: ThemeTokens = use_live_tokens(base_tokens);
            let live_p: ThemePalette = live_tok.get_palette();
            let live_g: ThemeGeometry = live_tok.get_geometry();
            let pill_radius: f32 = live_g
                .get_radius_md()
                .clamp(NAV_INDICATOR_MIN_RADIUS, NAV_INDICATOR_MAX_RADIUS);
            let mut st: Style = Style::default();
            st.get_mut_layout()
                .set_width(Length::Percent(100.0))
                .set_height(Length::Percent(100.0));
            st.get_mut_paint()
                .set_radius(CornerRadius::all(pill_radius.max(0.0)))
                .set_background(Some(live_p.get_primary()));
            st
        };
        let initial_display: Display = if initial_idx.is_some() {
            Display::Flex
        } else {
            Display::None
        };
        let indicator_node: NodeId = view! {
            row {
                display: initial_display
                position: Position::Absolute
                inset: (Some(6.0), None, None, Some(0.0))
                width: Length::Percent(slot_pct)
                height: Length::Px(46.0)
                padding: Edges::symmetric(0.0, 2.0)
                z_index: 0
                pointer_events: PointerEvents::None
                view {
                    bind_style: pill_style
                }
            }
        };
        let mut tab_nodes: Vec<NodeId> = Vec::with_capacity(items.len());
        let mut bound_tabs: Vec<(usize, NodeId, NodeId, Rc<Cell<bool>>)> =
            Vec::with_capacity(items.len());
        for (tab_idx, item) in items.iter().enumerate() {
            let click_path: String = item.get_path().clone();
            let icon_path: String = click_path.clone();
            let icon_label: String = item.get_label().clone();
            let init_under: bool = initial_idx == Some(tab_idx);
            let under_cell: Rc<Cell<bool>> = Rc::new(Cell::new(init_under));
            let under_paint: Rc<Cell<bool>> = Rc::clone(&under_cell);
            let on_tab_click = move |_: &UiEvent| {
                let _: bool = active_route.set_if_changed(click_path.clone());
            };
            let paint_icon = move |bounds: Rect, dl: &mut DisplayList| {
                let live_p: ThemePalette = live_tokens_untracked(base_tokens).get_palette();
                let col: Color = if under_paint.get() {
                    live_p.get_primary_foreground()
                } else {
                    live_p.get_muted_foreground()
                };
                draw_bottom_nav_icon(bounds, dl, &icon_path, &icon_label, col);
            };
            let icon_canvas: NodeId = view! {
                canvas {
                    width: Length::Px(20.0)
                    height: Length::Px(18.0)
                    painter: paint_icon
                }
            };
            let mut label_node: NodeId = NodeId::default();
            let tab_col: NodeId = view! {
                column {
                    flex_grow: 1.0
                    flex_shrink: 1.0
                    min_width: Length::Px(0.0)
                    height: Length::Px(46.0)
                    justify: JustifyContent::Center
                    align: AlignItems::Center
                    gap: 2.0
                    padding: Edges::symmetric(3.0, 6.0)
                    radius: pill_radius_init
                    overflow: Overflow::Hidden
                    background: COLOR_TRANSPARENT
                    z_index: 1
                    cursor: CursorIcon::Pointer
                    on_click: on_tab_click
                    { icon_canvas }
                    text {
                        node_ref: label_node
                        font_size: 10.5
                        font_weight: if init_under { FontWeight::Bold } else { FontWeight::Medium }
                        color: if init_under {
                            init_p.get_primary_foreground()
                        } else {
                            init_p.get_muted_foreground()
                        }
                        text_align: TextAlign::Center
                        text_overflow: TextOverflow::Clip
                        { item.get_label().as_str() }
                    }
                }
            };
            bound_tabs.push((tab_idx, icon_canvas, label_node, under_cell));
            tab_nodes.push(tab_col);
        }
        let bar_node: NodeId = view! {
            row {
                width: Length::Percent(100.0)
                height: Length::Percent(100.0)
                justify: JustifyContent::SpaceBetween
                align: AlignItems::Center
                gap: 0.0
                overflow: Overflow::Hidden
                { indicator_node }
                ..tab_nodes
            }
        };
        let scene_tabs: SceneHandle = ctx.get_scene().clone();
        let paths_for_pill: RoutePathList = Rc::clone(&route_paths);
        let (tabs_scope, _): (Scope, ()) = Scope::run(|_: Scope| {
            let pill_sel: Rc<Selector<Option<usize>>> =
                Rc::new(Selector::new(move || -> Option<usize> {
                    let cur: String = active_route.get();
                    let cur_idx: Option<usize> = paths_for_pill
                        .iter()
                        .position(|path: &String| path.as_str() == cur.as_str());
                    let dx: f32 = drag_sig.get();
                    if dx.abs() <= 0.5 {
                        return cur_idx;
                    }
                    let vp_w: f32 = vp_sig.get().get_width();
                    (0..item_count).find(|i: &usize| {
                        bottom_nav_tab_under_pill(*i, cur_idx, item_count, dx, vp_w)
                    })
                }));
            let sel_cleanup: Rc<Selector<Option<usize>>> = Rc::clone(&pill_sel);
            on_cleanup(move || {
                sel_cleanup.dispose();
            });
            for (tab_idx, icon_canvas, label_node, under_cell) in bound_tabs.into_iter() {
                let sel_tab: Rc<Selector<Option<usize>>> = Rc::clone(&pill_sel);
                let scene_tab: SceneHandle = scene_tabs.clone();
                let _: Effect = Effect::new(move || {
                    let under: bool = sel_tab.is_selected(Some(tab_idx));
                    let live_p: ThemePalette = use_live_tokens(base_tokens).get_palette();
                    under_cell.set(under);
                    let fg: Color = if under {
                        live_p.get_primary_foreground()
                    } else {
                        live_p.get_muted_foreground()
                    };
                    let weight: FontWeight = if under {
                        FontWeight::Bold
                    } else {
                        FontWeight::Medium
                    };
                    scene_tab.with_write(
                        |tree: &mut SceneTree| {
                            if let Some(node) = tree.get_node_mut(label_node) {
                                let mut st: Style = node.get_style();
                                st.get_mut_text().set_weight(weight).set_color(fg);
                                node.set_style(st);
                            }
                            tree.mark_dirty(label_node, DIRTY_LAYOUT | DIRTY_PAINT);
                            tree.mark_dirty(icon_canvas, DIRTY_PAINT);
                        },
                        (),
                    );
                });
            }
        });
        let last_idx: Rc<Cell<Option<usize>>> = Rc::new(Cell::new(initial_idx));
        let last_drag_tx: Rc<Cell<f32>> = Rc::new(Cell::new(0.0));
        let route_paths_anim: RoutePathList = Rc::clone(&route_paths);
        let ctx_anim: Context = ctx.clone();
        let last_idx_route: Rc<Cell<Option<usize>>> = Rc::clone(&last_idx);
        let last_drag_route: Rc<Cell<f32>> = Rc::clone(&last_drag_tx);
        let (route_scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            Effect::new(move || {
                let current_path: String = active_route.get();
                let vp_w: f32 = ctx_anim.get_viewport_size().get().get_width();
                let next_idx: Option<usize> = route_paths_anim
                    .iter()
                    .position(|path: &String| path.as_str() == current_path.as_str());
                let prev_idx: Option<usize> = last_idx_route.replace(next_idx);
                let prev_drag: f32 = last_drag_route.replace(0.0);
                let dirty_layout: u16 = DIRTY_LAYOUT | DIRTY_SUBTREE_LAYOUT;
                ctx_anim.get_scene().with_write(
                    |tree: &mut SceneTree| {
                        let bar_w: f32 = tree
                            .get_node(bar_node)
                            .map(|n: &SceneNode| n.get_layout_box().get_bounds().get_width())
                            .filter(|w: &f32| *w > 1.0)
                            .unwrap_or_else(|| (vp_w - 48.0).clamp(240.0, 460.0));
                        let bar_h: f32 = tree
                            .get_node(bar_node)
                            .map(|n: &SceneNode| {
                                n.get_layout_box().get_content_bounds().get_height()
                            })
                            .filter(|h: &f32| *h > 1.0)
                            .unwrap_or(58.0);
                        let top_px: f32 = ((bar_h - 46.0) * 0.5).max(0.0);
                        let slot_w: f32 = bar_w / (item_count as f32);
                        if let Some(target_i) = next_idx {
                            let target_left_px: f32 = (target_i as f32) * slot_w;
                            let cur_visual_tx: f32 = tree
                                .get_node(indicator_node)
                                .map(|n: &SceneNode| {
                                    n.get_style().get_paint().get_transform().get_translate_x()
                                })
                                .unwrap_or(prev_drag);
                            let from_x: f32 = if let Some(old_i) = prev_idx
                                && old_i != target_i
                            {
                                let delta_slots: f32 = (old_i as f32) - (target_i as f32);
                                delta_slots * slot_w + cur_visual_tx
                            } else {
                                cur_visual_tx
                            };
                            if let Some(node) = tree.get_node_mut(indicator_node) {
                                let mut st: Style = node.get_style();
                                st.get_mut_layout()
                                    .set_display(Display::Flex)
                                    .set_top(Some(top_px))
                                    .set_bottom(None)
                                    .set_left(Some(target_left_px))
                                    .set_right(None)
                                    .set_width(Length::Percent(slot_pct));
                                if from_x.abs() > 0.5 {
                                    st.get_mut_paint().set_transform(Transform2D::new(
                                        from_x, 0.0, 1.0, 1.0, 0.0,
                                    ));
                                }
                                node.set_style(st);
                            }
                            tree.mark_dirty(indicator_node, dirty_layout | DIRTY_PAINT);
                            if let Some(old_i) = prev_idx
                                && old_i != target_i
                            {
                                tree.start_transition(
                                    NodeTransition::slide_fade(
                                        indicator_node,
                                        Point::new(from_x, 0.0),
                                        Point::new(0.0, 0.0),
                                        1.0,
                                        1.0,
                                        NAV_INDICATOR_SLIDE_SECS,
                                        false,
                                    )
                                    .with_easing(Easing::EaseOutCubic),
                                );
                            } else if from_x.abs() > 0.5 {
                                tree.start_transition(
                                    NodeTransition::slide_fade(
                                        indicator_node,
                                        Point::new(from_x, 0.0),
                                        Point::new(0.0, 0.0),
                                        1.0,
                                        1.0,
                                        NAV_INDICATOR_CANCEL_SECS,
                                        false,
                                    )
                                    .with_easing(Easing::EaseOutCubic),
                                );
                            }
                        } else {
                            if let Some(node) = tree.get_node_mut(indicator_node) {
                                let mut st: Style = node.get_style();
                                st.get_mut_layout().set_display(Display::None);
                                st.get_mut_paint().set_transform(Transform2D::default());
                                node.set_style(st);
                            }
                            tree.mark_dirty(indicator_node, dirty_layout | DIRTY_PAINT);
                        }
                    },
                    (),
                );
            })
        });
        let ctx_drag: Context = ctx.clone();
        let last_idx_drag: Rc<Cell<Option<usize>>> = Rc::clone(&last_idx);
        let last_drag_live: Rc<Cell<f32>> = Rc::clone(&last_drag_tx);
        let (drag_scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            Effect::new(move || {
                let dx: f32 = drag_sig.get();
                let Some(cur_i) = last_idx_drag.get() else {
                    return;
                };
                let vp_w: f32 = ctx_drag
                    .get_viewport_size()
                    .get_untracked()
                    .get_width()
                    .max(1.0);
                if dx.abs() > 0.5 {
                    let mut frac: f32 = (-dx / vp_w).clamp(-1.0, 1.0);
                    if cur_i == 0 && frac < 0.0 {
                        frac = 0.0;
                    }
                    if cur_i + 1 >= item_count && frac > 0.0 {
                        frac = 0.0;
                    }
                    ctx_drag.get_scene().with_write(
                        |tree: &mut SceneTree| {
                            let bar_w: f32 = tree
                                .get_node(bar_node)
                                .map(|n: &SceneNode| n.get_layout_box().get_bounds().get_width())
                                .filter(|w: &f32| *w > 1.0)
                                .unwrap_or_else(|| (vp_w - 48.0).clamp(240.0, 460.0));
                            let slot_w: f32 = bar_w / (item_count as f32);
                            let tx: f32 = frac * slot_w;
                            last_drag_live.set(tx);
                            if let Some(node) = tree.get_node_mut(indicator_node) {
                                let mut st: Style = node.get_style();
                                st.get_mut_paint()
                                    .set_transform(Transform2D::new(tx, 0.0, 1.0, 1.0, 0.0));
                                node.set_style(st);
                            }
                            tree.mark_dirty(indicator_node, DIRTY_PAINT);
                        },
                        (),
                    );
                } else {
                    let prev_tx: f32 = last_drag_live.replace(0.0);
                    if prev_tx.abs() > 0.5 {
                        ctx_drag.get_scene().with_write(
                            |tree: &mut SceneTree| {
                                tree.start_transition(
                                    NodeTransition::slide_fade(
                                        indicator_node,
                                        Point::new(prev_tx, 0.0),
                                        Point::new(0.0, 0.0),
                                        1.0,
                                        1.0,
                                        NAV_INDICATOR_CANCEL_SECS,
                                        false,
                                    )
                                    .with_easing(Easing::EaseOutCubic),
                                );
                            },
                            (),
                        );
                    }
                }
            })
        });
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                tree.attach_scope(bar_node, tabs_scope);
                tree.attach_scope(bar_node, route_scope);
                tree.attach_scope(bar_node, drag_scope);
            },
            (),
        );
        bar_node
    }
}
