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
        let init_p: ThemePalette = init_tok.get_palette();
        let init_g: ThemeGeometry = init_tok.get_geometry();
        let active_route: Signal<String> = self.get_active_route();
        let Self { items, .. } = self;
        let route_paths: RoutePathList = items
            .iter()
            .map(|item: &EuvNavItem| item.get_path().clone())
            .collect();
        let init_idx: Option<usize> = {
            let cur: String = active_route.get_untracked();
            route_paths
                .iter()
                .position(|p: &String| p.as_str() == cur.as_str())
        };
        let row_h: f32 = init_g.get_control_sm().max(34.0);
        let row_rad: CornerRadius = CornerRadius::all(init_g.get_radius_md().max(0.0));
        let mut root_st: Style = Style::default();
        root_st
            .get_mut_layout()
            .set_flex_direction(FlexDirection::Column)
            .set_width(Length::Percent(100.0))
            .set_row_gap(2.0)
            .set_column_gap(2.0);
        let mut base_sec_txt_st: Style = Style::default();
        base_sec_txt_st
            .get_mut_text()
            .set_font_size(11.0)
            .set_weight(FontWeight::SemiBold)
            .set_color(init_p.get_muted_foreground());
        let mut base_row_st: Style = Style::default();
        base_row_st
            .get_mut_layout()
            .set_flex_direction(FlexDirection::Row)
            .set_width(Length::Percent(100.0))
            .set_height(Length::Px(row_h))
            .set_align_items(AlignItems::Center)
            .set_row_gap(SPACE_SM)
            .set_column_gap(SPACE_SM)
            .set_padding(Edges::symmetric(0.0, SPACE_SM));
        base_row_st
            .get_mut_paint()
            .set_radius(row_rad)
            .set_cursor(CursorIcon::Pointer);
        let mut base_ind_st: Style = Style::default();
        base_ind_st
            .get_mut_layout()
            .set_width(Length::Px(3.5))
            .set_height(Length::Px(18.0));
        base_ind_st
            .get_mut_paint()
            .set_radius(CornerRadius::all(99.0));
        let mut base_lbl_st: Style = Style::default();
        base_lbl_st
            .get_mut_text()
            .set_font_size(13.5)
            .set_weight(FontWeight::SemiBold)
            .set_color(init_p.get_foreground())
            .set_overflow(TextOverflow::Clip);
        let scene: SceneHandle = ctx.get_scene().clone();
        let (root, section_lbl_nodes, bound_items): (
            NodeId,
            Vec<NodeId>,
            Vec<(NodeId, NodeId, NodeId)>,
        ) = scene.with_write(
            |tree: &mut SceneTree| {
                let root_id: NodeId = tree.create_view(root_st);
                let mut children: Vec<NodeId> = Vec::with_capacity(items.len() * 2);
                let mut sec_lbls: Vec<NodeId> = Vec::new();
                let mut bound: Vec<(NodeId, NodeId, NodeId)> = Vec::with_capacity(items.len());
                let mut last_section: &str = "";
                let active_row_bg: Color = init_p.get_primary().with_alpha(24);
                let active_ind_bg: Color = init_p.get_primary();
                for (idx, item) in items.iter().enumerate() {
                    let sec: &str = item.get_section().as_str();
                    if !sec.is_empty() && sec != last_section {
                        last_section = sec;
                        let top_pad: f32 = if idx == 0 { 2.0 } else { 12.0 };
                        let mut sec_row_st: Style = Style::default();
                        sec_row_st
                            .get_mut_layout()
                            .set_flex_direction(FlexDirection::Row)
                            .set_width(Length::Percent(100.0))
                            .set_padding(Edges::new(top_pad, SPACE_SM, 4.0, SPACE_SM));
                        let sec_row: NodeId = tree.create_view(sec_row_st);
                        let sec_txt: NodeId = tree.create_text(sec.to_string(), base_sec_txt_st);
                        tree.append_child(sec_row, sec_txt);
                        sec_lbls.push(sec_txt);
                        children.push(sec_row);
                    }
                    let is_active: bool = init_idx == Some(idx);
                    let mut row_st: Style = base_row_st;
                    row_st.get_mut_paint().set_background(Some(if is_active {
                        active_row_bg
                    } else {
                        COLOR_TRANSPARENT
                    }));
                    let row_node: NodeId = tree.create_view(row_st);
                    let click_path: String = item.get_path().clone();
                    let click_handler: UiEventHandler =
                        Rc::new(RefCell::new(move |_: &UiEvent| {
                            let _: bool = active_route.set_if_changed(click_path.clone());
                        }));
                    if let Some(node) = tree.get_node_mut(row_node) {
                        node.get_mut_handlers().set_on_click(Some(click_handler));
                        node.get_mut_interaction().set_focusable(true);
                    }
                    let mut ind_st: Style = base_ind_st;
                    ind_st.get_mut_paint().set_background(Some(if is_active {
                        active_ind_bg
                    } else {
                        COLOR_TRANSPARENT
                    }));
                    let ind_node: NodeId = tree.create_view(ind_st);
                    let lbl_node: NodeId = tree.create_text(item.get_label().clone(), base_lbl_st);
                    tree.append_children(row_node, &[ind_node, lbl_node]);
                    bound.push((row_node, ind_node, lbl_node));
                    children.push(row_node);
                }
                if !children.is_empty() {
                    tree.append_children(root_id, &children);
                }
                (root_id, sec_lbls, bound)
            },
            (NodeId::default(), Vec::new(), Vec::new()),
        );
        let scene_eff: SceneHandle = scene.clone();
        let (nav_scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            let mut prev_tok: ThemeTokens = init_tok;
            let mut prev_idx: Option<usize> = init_idx;
            Effect::new(move || {
                let next_idx: Option<usize> = active_route
                    .with(|cur: &String| {
                        route_paths
                            .iter()
                            .position(|p: &String| p.as_str() == cur.as_str())
                    })
                    .flatten();
                let live_tok: ThemeTokens = use_live_tokens(base_tokens);
                let theme_changed: bool = live_tok != prev_tok;
                if !theme_changed && next_idx == prev_idx {
                    return;
                }
                let old_idx: Option<usize> = prev_idx;
                prev_tok = live_tok;
                prev_idx = next_idx;
                let live_p: ThemePalette = live_tok.get_palette();
                let live_g: ThemeGeometry = live_tok.get_geometry();
                let active_row_bg: Color = live_p.get_primary().with_alpha(24);
                let active_ind_bg: Color = live_p.get_primary();
                scene_eff.with_write(
                    |tree: &mut SceneTree| {
                        if theme_changed {
                            let dirty_layout: u16 = DIRTY_LAYOUT | DIRTY_SUBTREE_LAYOUT;
                            let row_dirty: u16 = dirty_layout | DIRTY_PAINT;
                            let next_h: Length = Length::Px(live_g.get_control_sm().max(34.0));
                            let next_rad: CornerRadius =
                                CornerRadius::all(live_g.get_radius_md().max(0.0));
                            let fg: Color = live_p.get_foreground();
                            let muted_fg: Color = live_p.get_muted_foreground();
                            for &sec_id in section_lbl_nodes.iter() {
                                if let Some(node) = tree.get_node_mut(sec_id) {
                                    node.get_mut_style().get_mut_text().set_color(muted_fg);
                                    tree.mark_dirty(sec_id, DIRTY_PAINT);
                                }
                            }
                            for (idx, &(row_node, ind_node, lbl_node)) in
                                bound_items.iter().enumerate()
                            {
                                let is_active: bool = next_idx == Some(idx);
                                if let Some(node) = tree.get_node_mut(row_node) {
                                    let st: &mut Style = node.get_mut_style();
                                    st.get_mut_layout().set_height(next_h);
                                    st.get_mut_paint().set_radius(next_rad).set_background(Some(
                                        if is_active {
                                            active_row_bg
                                        } else {
                                            COLOR_TRANSPARENT
                                        },
                                    ));
                                }
                                if let Some(node) = tree.get_node_mut(ind_node) {
                                    node.get_mut_style().get_mut_paint().set_background(Some(
                                        if is_active {
                                            active_ind_bg
                                        } else {
                                            COLOR_TRANSPARENT
                                        },
                                    ));
                                }
                                if let Some(node) = tree.get_node_mut(lbl_node) {
                                    node.get_mut_style().get_mut_text().set_color(fg);
                                }
                                tree.mark_dirty(row_node, row_dirty);
                                tree.mark_dirty(ind_node, DIRTY_PAINT);
                                tree.mark_dirty(lbl_node, DIRTY_PAINT);
                            }
                        } else {
                            if let Some(old_i) = old_idx
                                && let Some(&(row_node, ind_node, _)) = bound_items.get(old_i)
                            {
                                if let Some(node) = tree.get_node_mut(row_node) {
                                    node.get_mut_style()
                                        .get_mut_paint()
                                        .set_background(Some(COLOR_TRANSPARENT));
                                }
                                if let Some(node) = tree.get_node_mut(ind_node) {
                                    node.get_mut_style()
                                        .get_mut_paint()
                                        .set_background(Some(COLOR_TRANSPARENT));
                                }
                                tree.mark_dirty(row_node, DIRTY_PAINT);
                                tree.mark_dirty(ind_node, DIRTY_PAINT);
                            }
                            if let Some(new_i) = next_idx
                                && let Some(&(row_node, ind_node, _)) = bound_items.get(new_i)
                            {
                                if let Some(node) = tree.get_node_mut(row_node) {
                                    node.get_mut_style()
                                        .get_mut_paint()
                                        .set_background(Some(active_row_bg));
                                }
                                if let Some(node) = tree.get_node_mut(ind_node) {
                                    node.get_mut_style()
                                        .get_mut_paint()
                                        .set_background(Some(active_ind_bg));
                                }
                                tree.mark_dirty(row_node, DIRTY_PAINT);
                                tree.mark_dirty(ind_node, DIRTY_PAINT);
                            }
                        }
                    },
                    (),
                );
            })
        });
        scene.with_write(
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
        let init_p: ThemePalette = init_tok.get_palette();
        let init_g: ThemeGeometry = init_tok.get_geometry();
        let active: Signal<usize> = self.get_active();
        let init_active: usize = active.get_untracked();
        let Self { labels, .. } = self;
        let mut bar_st: Style = Style::default();
        bar_st
            .get_mut_layout()
            .set_flex_direction(FlexDirection::Row)
            .set_flex_wrap(FlexWrap::Wrap)
            .set_width(Length::Percent(100.0))
            .set_row_gap(SPACE_XS)
            .set_column_gap(SPACE_XS)
            .set_padding(Edges::all(SPACE_XS))
            .set_overflow(Overflow::Hidden);
        bar_st
            .get_mut_paint()
            .set_background(Some(init_p.get_muted()))
            .set_radius(CornerRadius::all(init_g.get_radius_md().max(0.0)));
        let mut base_tab_st: Style = Style::default();
        base_tab_st
            .get_mut_layout()
            .set_flex_direction(FlexDirection::Row)
            .set_height(Length::Px(init_g.get_control_sm()))
            .set_justify_content(JustifyContent::Center)
            .set_align_items(AlignItems::Center)
            .set_padding(Edges::symmetric(0.0, SPACE_MD))
            .set_overflow(Overflow::Hidden);
        base_tab_st
            .get_mut_paint()
            .set_radius(CornerRadius::all(init_g.get_radius_sm().max(0.0)))
            .set_cursor(CursorIcon::Pointer);
        let mut base_lbl_st: Style = Style::default();
        base_lbl_st
            .get_mut_text()
            .set_font_size(13.0)
            .set_weight(FontWeight::Medium)
            .set_color(init_p.get_foreground())
            .set_align(TextAlign::Center)
            .set_overflow(TextOverflow::Ellipsis);
        let scene: SceneHandle = ctx.get_scene().clone();
        let (bar_node, bound_tabs): (NodeId, Vec<(NodeId, NodeId)>) = scene.with_write(
            |tree: &mut SceneTree| {
                let bar_id: NodeId = tree.create_view(bar_st);
                let mut tab_ids: Vec<NodeId> = Vec::with_capacity(labels.len());
                let mut bound: Vec<(NodeId, NodeId)> = Vec::with_capacity(labels.len());
                let bw: f32 = init_g.get_border_width().max(0.0);
                let border_col: Color = init_p.get_border();
                let shadow_xs: Shadow = init_g.get_shadow_xs();
                let card_bg: Color = init_p.get_card();
                for (idx, label) in labels.iter().enumerate() {
                    let selected: bool = idx == init_active;
                    let mut tab_st: Style = base_tab_st;
                    tab_st
                        .get_mut_paint()
                        .set_background(Some(if selected { card_bg } else { COLOR_TRANSPARENT }))
                        .set_border(Border::new(if selected { bw } else { 0.0 }, border_col))
                        .set_shadow(if selected { Some(shadow_xs) } else { None });
                    let tab_id: NodeId = tree.create_view(tab_st);
                    let click_handler: UiEventHandler =
                        Rc::new(RefCell::new(move |_: &UiEvent| {
                            let _: bool = active.set_if_changed(idx);
                        }));
                    if let Some(node) = tree.get_node_mut(tab_id) {
                        node.get_mut_handlers().set_on_click(Some(click_handler));
                        node.get_mut_interaction().set_focusable(true);
                    }
                    let lbl_id: NodeId = tree.create_text(label.clone(), base_lbl_st);
                    tree.append_child(tab_id, lbl_id);
                    bound.push((tab_id, lbl_id));
                    tab_ids.push(tab_id);
                }
                if !tab_ids.is_empty() {
                    tree.append_children(bar_id, &tab_ids);
                }
                (bar_id, bound)
            },
            (NodeId::default(), Vec::new()),
        );
        let scene_eff: SceneHandle = scene.clone();
        let (tabs_scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            let mut prev_tok: ThemeTokens = init_tok;
            let mut prev_idx: usize = init_active;
            Effect::new(move || {
                let next_idx: usize = active.get();
                let live_tok: ThemeTokens = use_live_tokens(base_tokens);
                let theme_changed: bool = live_tok != prev_tok;
                if !theme_changed && next_idx == prev_idx {
                    return;
                }
                let old_idx: usize = prev_idx;
                prev_tok = live_tok;
                prev_idx = next_idx;
                let live_p: ThemePalette = live_tok.get_palette();
                let live_g: ThemeGeometry = live_tok.get_geometry();
                let card_bg: Color = live_p.get_card();
                let border_col: Color = live_p.get_border();
                let bw: f32 = live_g.get_border_width().max(0.0);
                let shadow_xs: Shadow = live_g.get_shadow_xs();
                scene_eff.with_write(
                    |tree: &mut SceneTree| {
                        if theme_changed {
                            let dirty_layout: u16 = DIRTY_LAYOUT | DIRTY_SUBTREE_LAYOUT;
                            let dirty_all: u16 = dirty_layout | DIRTY_PAINT;
                            if let Some(node) = tree.get_node_mut(bar_node) {
                                node.get_mut_style()
                                    .get_mut_paint()
                                    .set_background(Some(live_p.get_muted()))
                                    .set_radius(CornerRadius::all(live_g.get_radius_md().max(0.0)));
                                tree.mark_dirty(bar_node, DIRTY_PAINT);
                            }
                            let tab_h: Length = Length::Px(live_g.get_control_sm());
                            let tab_rad: CornerRadius =
                                CornerRadius::all(live_g.get_radius_sm().max(0.0));
                            let fg: Color = live_p.get_foreground();
                            for (idx, &(tab_id, lbl_id)) in bound_tabs.iter().enumerate() {
                                let selected: bool = idx == next_idx;
                                if let Some(node) = tree.get_node_mut(tab_id) {
                                    let st: &mut Style = node.get_mut_style();
                                    st.get_mut_layout().set_height(tab_h);
                                    st.get_mut_paint()
                                        .set_radius(tab_rad)
                                        .set_background(Some(if selected {
                                            card_bg
                                        } else {
                                            COLOR_TRANSPARENT
                                        }))
                                        .set_border(Border::new(
                                            if selected { bw } else { 0.0 },
                                            border_col,
                                        ))
                                        .set_shadow(if selected { Some(shadow_xs) } else { None });
                                }
                                if let Some(node) = tree.get_node_mut(lbl_id) {
                                    node.get_mut_style().get_mut_text().set_color(fg);
                                }
                                tree.mark_dirty(tab_id, dirty_all);
                                tree.mark_dirty(lbl_id, DIRTY_PAINT);
                            }
                        } else {
                            if let Some(&(old_tab, _)) = bound_tabs.get(old_idx) {
                                if let Some(node) = tree.get_node_mut(old_tab) {
                                    node.get_mut_style()
                                        .get_mut_paint()
                                        .set_background(Some(COLOR_TRANSPARENT))
                                        .set_border(Border::new(0.0, border_col))
                                        .set_shadow(None);
                                }
                                tree.mark_dirty(old_tab, DIRTY_PAINT);
                            }
                            if let Some(&(new_tab, _)) = bound_tabs.get(next_idx) {
                                if let Some(node) = tree.get_node_mut(new_tab) {
                                    node.get_mut_style()
                                        .get_mut_paint()
                                        .set_background(Some(card_bg))
                                        .set_border(Border::new(bw, border_col))
                                        .set_shadow(Some(shadow_xs));
                                }
                                tree.mark_dirty(new_tab, DIRTY_PAINT);
                            }
                        }
                    },
                    (),
                );
            })
        });
        scene.with_write(
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
        let (tabs_scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            let mut prev_under: Option<usize> = initial_idx;
            let mut prev_p: ThemePalette = init_p;
            Effect::new(move || {
                let cur_idx: Option<usize> = active_route
                    .with(|cur: &String| {
                        paths_for_pill
                            .iter()
                            .position(|path: &String| path.as_str() == cur.as_str())
                    })
                    .flatten();
                let dx: f32 = drag_sig.get();
                let next_under: Option<usize> = if dx.abs() <= 0.5 {
                    cur_idx
                } else {
                    let vp_w: f32 = vp_sig.get().get_width();
                    (0..item_count).find(|i: &usize| {
                        bottom_nav_tab_under_pill(*i, cur_idx, item_count, dx, vp_w)
                    })
                };
                let live_p: ThemePalette = use_live_tokens(base_tokens).get_palette();
                let palette_changed: bool = live_p != prev_p;
                if !palette_changed && next_under == prev_under {
                    return;
                }
                let old_under: Option<usize> = prev_under;
                prev_under = next_under;
                prev_p = live_p;
                let active_fg: Color = live_p.get_primary_foreground();
                let muted_fg: Color = live_p.get_muted_foreground();
                scene_tabs.with_write(
                    |tree: &mut SceneTree| {
                        if palette_changed {
                            for &(tab_idx, icon_canvas, label_node, ref under_cell) in
                                bound_tabs.iter()
                            {
                                let under: bool = next_under == Some(tab_idx);
                                under_cell.set(under);
                                if let Some(node) = tree.get_node_mut(label_node) {
                                    node.get_mut_style()
                                        .get_mut_text()
                                        .set_weight(if under {
                                            FontWeight::Bold
                                        } else {
                                            FontWeight::Medium
                                        })
                                        .set_color(if under { active_fg } else { muted_fg });
                                }
                                tree.mark_dirty(label_node, DIRTY_LAYOUT | DIRTY_PAINT);
                                tree.mark_dirty(icon_canvas, DIRTY_PAINT);
                            }
                        } else {
                            if let Some(old_i) = old_under
                                && let Some(&(_, icon_canvas, label_node, ref under_cell)) =
                                    bound_tabs.get(old_i)
                            {
                                under_cell.set(false);
                                if let Some(node) = tree.get_node_mut(label_node) {
                                    node.get_mut_style()
                                        .get_mut_text()
                                        .set_weight(FontWeight::Medium)
                                        .set_color(muted_fg);
                                }
                                tree.mark_dirty(label_node, DIRTY_LAYOUT | DIRTY_PAINT);
                                tree.mark_dirty(icon_canvas, DIRTY_PAINT);
                            }
                            if let Some(new_i) = next_under
                                && let Some(&(_, icon_canvas, label_node, ref under_cell)) =
                                    bound_tabs.get(new_i)
                            {
                                under_cell.set(true);
                                if let Some(node) = tree.get_node_mut(label_node) {
                                    node.get_mut_style()
                                        .get_mut_text()
                                        .set_weight(FontWeight::Bold)
                                        .set_color(active_fg);
                                }
                                tree.mark_dirty(label_node, DIRTY_LAYOUT | DIRTY_PAINT);
                                tree.mark_dirty(icon_canvas, DIRTY_PAINT);
                            }
                        }
                    },
                    (),
                );
            })
        });
        let last_idx: Rc<Cell<Option<usize>>> = Rc::new(Cell::new(initial_idx));
        let last_drag_tx: Rc<Cell<f32>> = Rc::new(Cell::new(0.0));
        let route_paths_anim: RoutePathList = Rc::clone(&route_paths);
        let ctx_anim: Context = ctx.clone();
        let last_idx_route: Rc<Cell<Option<usize>>> = Rc::clone(&last_idx);
        let last_drag_route: Rc<Cell<f32>> = Rc::clone(&last_drag_tx);
        let (route_scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            Effect::new(move || {
                let next_idx: Option<usize> = active_route
                    .with(|current_path: &String| {
                        route_paths_anim
                            .iter()
                            .position(|path: &String| path.as_str() == current_path.as_str())
                    })
                    .flatten();
                let vp_w: f32 = ctx_anim.get_viewport_size().get().get_width();
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
                                    n.style_ref().get_paint().get_transform().get_translate_x()
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
                                let st: &mut Style = node.get_mut_style();
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
                                let st: &mut Style = node.get_mut_style();
                                st.get_mut_layout().set_display(Display::None);
                                st.get_mut_paint().set_transform(Transform2D::default());
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
                                node.get_mut_style()
                                    .get_mut_paint()
                                    .set_transform(Transform2D::new(tx, 0.0, 1.0, 1.0, 0.0));
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
