use super::*;

/// Builder helpers for `EuvAppShell`.
impl EuvAppShell {
    /// Binds an active route signal so compact mobile navigation drawers auto-close on route change.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source app shell.
    /// - `Signal<String>` - Active route path signal.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated app shell.
    #[must_use]
    pub fn route(mut self, route_signal: Signal<String>) -> Self {
        self.set_route_signal(Some(route_signal));
        self
    }

    /// Attaches global header actions (e.g. Command Palette button, Theme Toggle) to the persistent top chrome bar.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source app shell.
    /// - `NodeId` - Mounted header actions row node.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated app shell.
    #[must_use]
    pub fn top_bar_actions(mut self, actions: NodeId) -> Self {
        self.set_top_bar_actions(Some(actions));
        self
    }

    /// Attaches compact inline actions rendered beside the mobile header `Menu` button.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source app shell.
    /// - `NodeId` - Mounted compact actions node.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated app shell.
    #[must_use]
    pub fn mobile_header_actions(mut self, actions: NodeId) -> Self {
        self.set_mobile_header_actions(Some(actions));
        self
    }

    /// Attaches a footer widget (e.g. theme/DPI indicator) pinned below the sidebar navigation list.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source app shell.
    /// - `NodeId` - Mounted sidebar footer node.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated app shell.
    #[must_use]
    pub fn sidebar_footer(mut self, footer: NodeId) -> Self {
        self.set_sidebar_footer(Some(footer));
        self
    }

    /// Binds a reactive breadcrumb string displayed in the persistent desktop/tablet top header bar.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source app shell.
    /// - `Signal<String>` - Reactive breadcrumb text signal.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated app shell.
    #[must_use]
    pub fn breadcrumb(mut self, breadcrumb: Signal<String>) -> Self {
        self.set_breadcrumb(Some(breadcrumb));
        self
    }

    /// Attaches a persistent bottom navigation bar, switching the shell into Twitter/X-style bottom-nav layout.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source app shell.
    /// - `NodeId` - Mounted bottom navigation bar node (typically `EuvBottomNav`).
    ///
    /// # Returns
    ///
    /// - `Self` - Updated app shell.
    #[must_use]
    pub fn bottom_nav(mut self, bottom_bar: NodeId) -> Self {
        self.set_bottom_nav(Some(bottom_bar));
        self
    }

    /// Configures ordered route paths navigated directly via horizontal left/right viewport swipes.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source app shell.
    /// - `Vec<String>` - Ordered route path list.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated app shell.
    #[must_use]
    pub fn swipe_routes(mut self, routes: Vec<String>) -> Self {
        self.set_swipe_routes(routes);
        self
    }

    /// Enables or disables the live real-time FPS telemetry badge in the top navigation bar.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source app shell.
    /// - `bool` - Whether the live FPS badge is displayed.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated app shell.
    #[must_use]
    pub fn show_fps(mut self, show: bool) -> Self {
        self.set_show_fps(show);
        self
    }
}

/// Declarative `#[component]` implementation for `EuvAppShell`.
#[component]
/// Inherent implementation of [`EuvAppShell`].
impl EuvAppShell {
    /// Renders `EuvAppShell` into the ambient framework-managed `Context`.
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
        let g: ThemeGeometry = tokens.get_geometry();
        let bp_sig: Signal<Breakpoint> = ctx.get_breakpoint();
        let vp_sig: Signal<Size> = ctx.get_viewport_size();
        let safe_sig: Signal<SafeAreaInsets> = ctx.get_safe_area();
        let gesture_sig: Signal<Option<GestureEvent>> = ctx.get_gesture();
        let fps_sig: Signal<f32> = ctx.get_live_fps();
        let show_fps: bool = self.get_show_fps();
        let Self {
            brand,
            sidebar,
            main_content,
            route_signal,
            top_bar_actions,
            mobile_header_actions,
            sidebar_footer,
            breadcrumb,
            bottom_nav,
            swipe_routes,
            ..
        } = self.clone();
        let has_bottom_nav: bool = bottom_nav.is_some();
        let brand_initial: String = brand
            .chars()
            .next()
            .map(|c: char| c.to_ascii_uppercase().to_string())
            .unwrap_or_else(|| SHELL_DEFAULT_INITIAL.to_string());
        let menu_open: Signal<bool> = Signal::new(false);
        let drawer_expanded: Signal<bool> = Signal::new(false);
        let drawer_drag_active: Rc<Cell<bool>> = Rc::new(Cell::new(false));
        let drawer_drag_start_x: Rc<Cell<f32>> = Rc::new(Cell::new(0.0));
        let drawer_drag_base_w: Rc<Cell<f32>> = Rc::new(Cell::new(248.0));
        let drawer_drag_moved: Rc<Cell<bool>> = Rc::new(Cell::new(false));
        let div_h: f32 = g.get_border_width().max(1.0);
        let mut top_bar: NodeId = NodeId::default();
        let mut top_bar_divider: NodeId = NodeId::default();
        let mut top_title_txt: NodeId = NodeId::default();
        let mut body_row: NodeId = NodeId::default();
        let mut nav_rail: NodeId = NodeId::default();
        let mut nav_divider: NodeId = NodeId::default();
        let mut content_column: NodeId = NodeId::default();
        let mut desktop_top_bar: NodeId = NodeId::default();
        let mut desktop_top_divider: NodeId = NodeId::default();
        let mut crumb_txt: NodeId = NodeId::default();
        let mut main_area: NodeId = NodeId::default();
        let mut bottom_divider: NodeId = NodeId::default();
        let mut bottom_bar_host: NodeId = NodeId::default();
        let mut drawer_portal: NodeId = NodeId::default();
        let mut drawer_panel: NodeId = NodeId::default();
        let mut drawer_scroll: NodeId = NodeId::default();
        let mut drawer_grabber: NodeId = NodeId::default();
        let mobile_fps_pill: Option<NodeId> = if show_fps {
            Some(view! {
                row {
                    min_width: Length::Px(64.0)
                    height: Length::Px(26.0)
                    padding: Edges::symmetric(0.0, 8.0)
                    radius: 99.0
                    justify: JustifyContent::Center
                    align: AlignItems::Center
                    gap: 5.0
                    background: p.get_muted()
                    border: (1.0, p.get_border())
                    view {
                        bind_style: move || {
                            let fps: f32 = fps_sig.get();
                            let dot_col: Color = if fps >= 48.0 {
                                p.get_success()
                            } else if fps >= 28.0 {
                                p.get_warning()
                            } else {
                                p.get_destructive()
                            };
                            Style::default()
                            .width(Length::Px(6.0))
                            .height(Length::Px(6.0))
                            .radius(99.0)
                            .background(dot_col)
                        }
                    }
                    text {
                        font_size: 11.0
                        font_family: FontFamily::Monospace
                        font_weight: FontWeight::Bold
                        color: p.get_foreground()
                        text_align: TextAlign::Center
                        text_overflow: TextOverflow::Clip
                        { move || format!("{:.0} FPS", fps_sig.get().clamp(1.0, 999.0)) }
                    }
                }
            })
        } else {
            None
        };
        let desktop_fps_pill: Option<NodeId> = if show_fps && !has_bottom_nav {
            Some(view! {
                row {
                    min_width: Length::Px(68.0)
                    height: Length::Px(26.0)
                    padding: Edges::symmetric(0.0, 8.0)
                    radius: 99.0
                    justify: JustifyContent::Center
                    align: AlignItems::Center
                    gap: 5.0
                    background: p.get_muted()
                    border: (1.0, p.get_border())
                    view {
                        bind_style: move || {
                            let fps: f32 = fps_sig.get();
                            let dot_col: Color = if fps >= 48.0 {
                                p.get_success()
                            } else if fps >= 28.0 {
                                p.get_warning()
                            } else {
                                p.get_destructive()
                            };
                            Style::default()
                            .width(Length::Px(6.0))
                            .height(Length::Px(6.0))
                            .radius(99.0)
                            .background(dot_col)
                        }
                    }
                    text {
                        font_size: 11.0
                        font_family: FontFamily::Monospace
                        font_weight: FontWeight::Bold
                        color: p.get_foreground()
                        text_align: TextAlign::Center
                        text_overflow: TextOverflow::Clip
                        { move || format!("{:.0} FPS", fps_sig.get().clamp(1.0, 999.0)) }
                    }
                }
            })
        } else {
            None
        };
        let root: NodeId = view! {
            column {
                width: Length::Percent(100.0)
                height: Length::Percent(100.0)
                background: p.get_background()
                overflow: Overflow::Hidden
                row {
                    node_ref: top_bar
                    width: Length::Percent(100.0)
                    height: Length::Px(MOBILE_HEADER_HEIGHT)
                    justify: JustifyContent::SpaceBetween
                    align: AlignItems::Center
                    gap: SPACE_XS
                    background: p.get_card()
                    overflow: Overflow::Hidden
                    z_index: Z_INDEX_CHROME
                    row {
                        flex_grow: 1.0
                        flex_shrink: 1.0
                        min_width: Length::Px(0.0)
                        align: AlignItems::Center
                        gap: SPACE_SM
                        overflow: Overflow::Hidden
                        if has_bottom_nav {
                            row {
                                width: Length::Px(30.0)
                                height: Length::Px(30.0)
                                flex_shrink: 0.0
                                radius: 99.0
                                background: p.get_primary()
                                justify: JustifyContent::Center
                                align: AlignItems::Center
                                cursor: CursorIcon::Pointer
                                on_click: move |_: &UiEvent| {
                                    let _: bool = menu_open.update(|v: &mut bool| *v = !*v);
                                }
                                text {
                                    font_size: 13.0
                                    font_weight: FontWeight::Bold
                                    color: p.get_primary_foreground()
                                    text_align: TextAlign::Center
                                    text_overflow: TextOverflow::Clip
                                    { brand_initial.clone() }
                                }
                            }
                        }
                        text {
                            node_ref: top_title_txt
                            flex_shrink: 1.0
                            min_width: Length::Px(0.0)
                            font_size: 15.0
                            font_weight: FontWeight::Bold
                            color: p.get_foreground()
                            overflow: Overflow::Hidden
                            text_overflow: TextOverflow::Ellipsis
                            { brand.clone() }
                        }
                    }
                    row {
                        flex_shrink: 0.0
                        align: AlignItems::Center
                        gap: SPACE_XS
                        if let Some(fps_node) = mobile_fps_pill {
                            { fps_node }
                        }
                        if let Some(mob_act) = mobile_header_actions {
                            { mob_act }
                        }
                        if !has_bottom_nav {
                            EuvButton(SHELL_MENU_LABEL.to_string(), tokens) {
                                outline()
                                sm()
                                on_click: move |_: &UiEvent| {
                                    let _: bool = menu_open.update(|v: &mut bool| *v = !*v);
                                }
                            }
                        }
                    }
                }
                view {
                    node_ref: top_bar_divider
                    width: Length::Percent(100.0)
                    height: Length::Px(div_h)
                    background: p.get_border()
                }
                row {
                    node_ref: body_row
                    flex_grow: 1.0
                    width: Length::Percent(100.0)
                    height: Length::Percent(100.0)
                    overflow: Overflow::Hidden
                    scroll {
                        node_ref: nav_rail
                        style: Style::default().column()
                        width: Length::Px(NAV_SIDEBAR_WIDTH)
                        height: Length::Percent(100.0)
                        gap: SPACE_MD
                        background: p.get_card()
                        stop_scroll_propagation: true
                        z_index: Z_INDEX_CHROME
                        row {
                            width: Length::Percent(100.0)
                            align: AlignItems::Center
                            gap: SPACE_SM
                            padding: Edges::new(0.0, 0.0, SPACE_XS, 0.0)
                            view {
                                width: Length::Px(24.0)
                                height: Length::Px(24.0)
                                radius: 6.0
                                background: p.get_primary()
                                justify: JustifyContent::Center
                                align: AlignItems::Center
                                text {
                                    font_size: 13.0
                                    font_weight: FontWeight::Bold
                                    color: p.get_primary_foreground()
                                    { brand_initial.clone() }
                                }
                            }
                            text {
                                font_size: 14.5
                                font_weight: FontWeight::Bold
                                color: p.get_foreground()
                                { brand.clone() }
                            }
                        }
                    }
                    view {
                        node_ref: nav_divider
                        width: Length::Px(div_h)
                        height: Length::Percent(100.0)
                        background: p.get_border()
                    }
                    column {
                        node_ref: content_column
                        flex_grow: 1.0
                        height: Length::Percent(100.0)
                        overflow: Overflow::Hidden
                        row {
                            node_ref: desktop_top_bar
                            width: Length::Percent(100.0)
                            height: Length::Px(52.0)
                            justify: JustifyContent::SpaceBetween
                            align: AlignItems::Center
                            padding: Edges::symmetric(0.0, SPACE_XL)
                            background: p.get_card()
                            z_index: Z_INDEX_CHROME
                            text {
                                node_ref: crumb_txt
                                font_size: 13.0
                                font_weight: FontWeight::Medium
                                color: p.get_muted_foreground()
                                { brand.clone() }
                            }
                            row {
                                align: AlignItems::Center
                                gap: SPACE_SM
                                if let Some(fps_d) = desktop_fps_pill {
                                    { fps_d }
                                }
                                if let Some(actions_node) = top_bar_actions {
                                    { actions_node }
                                }
                            }
                        }
                        view {
                            node_ref: desktop_top_divider
                            width: Length::Percent(100.0)
                            height: Length::Px(div_h)
                            background: p.get_border()
                        }
                        scroll {
                            node_ref: main_area
                            style: Style::default().column()
                            flex_grow: 1.0
                            width: Length::Percent(100.0)
                            height: Length::Percent(100.0)
                            align: AlignItems::Center
                            gap: SPACE_LG
                            { main_content }
                        }
                    }
                }
                view {
                    node_ref: bottom_divider
                    display: if has_bottom_nav { Display::Flex } else { Display::None }
                    width: Length::Percent(100.0)
                    height: Length::Px(div_h)
                    background: p.get_border()
                }
                row {
                    node_ref: bottom_bar_host
                    display: if has_bottom_nav { Display::Flex } else { Display::None }
                    width: Length::Percent(100.0)
                    height: Length::Px(SHELL_BOTTOM_NAV_HEIGHT)
                    justify: JustifyContent::Center
                    align: AlignItems::Center
                    background: p.get_card()
                    z_index: Z_INDEX_CHROME
                    if let Some(bn) = bottom_nav {
                        { bn }
                    }
                }
                portal {
                    node_ref: drawer_portal
                    style: Style::default().row()
                    display: Display::None
                    position: Position::Fixed
                    inset: (Some(0.0), Some(0.0), Some(0.0), Some(0.0))
                    width: Length::Percent(100.0)
                    height: Length::Percent(100.0)
                    justify: JustifyContent::Start
                    background: p.get_overlay()
                    stop_scroll_propagation: true
                    z_index: Z_INDEX_DRAWER
                    row {
                        node_ref: drawer_panel
                        width: Length::Px(248.0)
                        max_width: Length::Percent(92.0)
                        height: Length::Percent(100.0)
                        background: p.get_card()
                        border: (g.get_border_width(), p.get_border())
                        shadow: g.get_shadow_lg()
                        overflow: Overflow::Hidden
                        scroll {
                            node_ref: drawer_scroll
                            style: Style::default().column()
                            flex_grow: 1.0
                            flex_shrink: 1.0
                            min_width: Length::Px(0.0)
                            height: Length::Percent(100.0)
                            gap: SPACE_SM
                            stop_scroll_propagation: true
                            row {
                                width: Length::Percent(100.0)
                                align: AlignItems::Center
                                gap: SPACE_SM
                                padding: Edges::new(0.0, 0.0, SPACE_XS, 0.0)
                                view {
                                    width: Length::Px(22.0)
                                    height: Length::Px(22.0)
                                    radius: 6.0
                                    background: p.get_primary()
                                    justify: JustifyContent::Center
                                    align: AlignItems::Center
                                    text {
                                        font_size: 12.0
                                        font_weight: FontWeight::Bold
                                        color: p.get_primary_foreground()
                                        { brand_initial }
                                    }
                                }
                                text {
                                    font_size: 14.5
                                    font_weight: FontWeight::Bold
                                    color: p.get_foreground()
                                    { brand }
                                }
                            }
                        }
                        row {
                            node_ref: drawer_grabber
                            width: Length::Px(20.0)
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
                    }
                }
            }
        };
        let d_down_active: Rc<Cell<bool>> = drawer_drag_active.clone();
        let d_down_x: Rc<Cell<f32>> = drawer_drag_start_x.clone();
        let d_down_w: Rc<Cell<f32>> = drawer_drag_base_w.clone();
        let d_down_moved: Rc<Cell<bool>> = drawer_drag_moved.clone();
        let h_down: SceneHandle = ctx.get_scene().clone();
        ctx.on_pointer_down(drawer_grabber, move |ev: &UiEvent| {
            if let UiEventKind::PointerDown(pt) = ev.get_kind() {
                let vp_w: f32 = vp_sig.get_untracked().get_width().max(320.0);
                let def_w: f32 = (vp_w * 0.68).clamp(224.0, 256.0);
                let cur_w: f32 = h_down
                    .node_screen_bounds(drawer_panel)
                    .map(|r: Rect| r.get_width())
                    .filter(|w: &f32| *w > 40.0)
                    .unwrap_or(def_w);
                d_down_active.set(true);
                d_down_x.set(pt.get_x());
                d_down_w.set(cur_w);
                d_down_moved.set(false);
            }
        });
        for target_node in [drawer_grabber, drawer_panel, drawer_portal].into_iter() {
            let h_move: SceneHandle = ctx.get_scene().clone();
            let da_move: Rc<Cell<bool>> = drawer_drag_active.clone();
            let sx_move: Rc<Cell<f32>> = drawer_drag_start_x.clone();
            let bw_move: Rc<Cell<f32>> = drawer_drag_base_w.clone();
            let dm_move: Rc<Cell<bool>> = drawer_drag_moved.clone();
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
                    let def_w: f32 = (vp_w * 0.68).clamp(224.0, 256.0);
                    let max_w: f32 = (vp_w * 0.88).clamp(def_w + 40.0, 380.0);
                    let raw_w: f32 = bw_move.get() + dx;
                    let (next_w, tx): (f32, f32) = if raw_w >= def_w {
                        (raw_w.min(max_w), 0.0)
                    } else {
                        (def_w, (raw_w - def_w).min(0.0))
                    };
                    h_move.with_write(
                        |tree: &mut SceneTree| {
                            if let Some(n) = tree.get_node_mut(drawer_panel) {
                                let mut st: Style = n.get_style();
                                st.get_mut_layout().set_width(Length::Px(next_w));
                                st.get_mut_paint()
                                    .set_transform(Transform2D::new(tx, 0.0, 1.0, 1.0, 0.0));
                                n.set_style(st);
                                let layout_dirty: u16 = DIRTY_LAYOUT | DIRTY_SUBTREE_LAYOUT;
                                tree.mark_dirty(drawer_panel, layout_dirty | DIRTY_PAINT);
                            }
                        },
                        (),
                    );
                }
            });
            let h_up: SceneHandle = ctx.get_scene().clone();
            let da_up: Rc<Cell<bool>> = drawer_drag_active.clone();
            let sx_up: Rc<Cell<f32>> = drawer_drag_start_x.clone();
            ctx.on_pointer_up(target_node, move |ev: &UiEvent| {
                if !da_up.replace(false) {
                    return;
                }
                let dx: f32 = match ev.get_kind() {
                    UiEventKind::PointerUp(pt) => pt.get_x() - sx_up.get(),
                    _ => 0.0,
                };
                let vp_w: f32 = vp_sig.get_untracked().get_width().max(320.0);
                let def_w: f32 = (vp_w * 0.68).clamp(224.0, 256.0);
                let max_w: f32 = (vp_w * 0.88).clamp(def_w + 40.0, 380.0);
                let is_exp: bool = drawer_expanded.get_untracked();
                if dx < -52.0 {
                    if is_exp && dx > -140.0 {
                        let _: bool = drawer_expanded.set_if_changed(false);
                    } else {
                        let _: bool = drawer_expanded.set_if_changed(false);
                        let _: bool = menu_open.set_if_changed(false);
                        return;
                    }
                } else if dx > 40.0 {
                    let _: bool = drawer_expanded.set_if_changed(true);
                }
                let target_w: f32 = if drawer_expanded.get_untracked() {
                    max_w
                } else {
                    def_w
                };
                h_up.with_write(
                    |tree: &mut SceneTree| {
                        if let Some(n) = tree.get_node_mut(drawer_panel) {
                            let mut st: Style = n.get_style();
                            st.get_mut_layout().set_width(Length::Px(target_w));
                            st.get_mut_paint().set_transform(Transform2D::default());
                            n.set_style(st);
                            let layout_dirty: u16 = DIRTY_LAYOUT | DIRTY_SUBTREE_LAYOUT;
                            tree.mark_dirty(drawer_panel, layout_dirty | DIRTY_PAINT);
                        }
                    },
                    (),
                );
            });
        }
        let drawer_click_moved: Rc<Cell<bool>> = drawer_drag_moved.clone();
        ctx.on_click(drawer_portal, move |ev: &UiEvent| {
            if drawer_click_moved.replace(false) {
                return;
            }
            if ev.get_target() == drawer_portal {
                let _: bool = menu_open.set_if_changed(false);
            }
        });
        let handle: SceneHandle = ctx.get_scene().clone();
        let handle_layout: SceneHandle = handle.clone();
        let handle_crumb: SceneHandle = handle.clone();
        let prev_open: Rc<Cell<bool>> = Rc::new(Cell::new(false));
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            let _: Effect =
                EuvOverlayStack::bind_signal(menu_open, SHELL_DRAWER_OVERLAY_NAME, Z_INDEX_DRAWER);
            if let Some(crumb_sig) = breadcrumb {
                let target_title_node: NodeId = if has_bottom_nav {
                    top_title_txt
                } else {
                    crumb_txt
                };
                let _: Effect = handle_crumb.bind_text(target_title_node, move || crumb_sig.get());
            }
            let _: Effect = Effect::new(move || {
                let bp: Breakpoint = bp_sig.get();
                let vp: Size = vp_sig.get();
                let safe: SafeAreaInsets = safe_sig.get();
                let compact: bool = has_bottom_nav || bp.is_compact();
                let is_medium: bool = !has_bottom_nav && bp == Breakpoint::Medium;
                let top_inset: f32 = safe.get_top();
                let bottom_inset: f32 = safe.get_bottom();
                let left_inset: f32 = safe.get_left();
                let right_inset: f32 = safe.get_right();
                let vp_h: f32 = vp.get_height().max(240.0);
                let header_h: f32 = MOBILE_HEADER_HEIGHT + top_inset;
                let bottom_h: f32 = if has_bottom_nav {
                    SHELL_BOTTOM_NAV_HEIGHT + bottom_inset + div_h
                } else {
                    0.0
                };
                let desktop_bar_h: f32 = 52.0 + top_inset;
                let body_h: f32 = if compact {
                    (vp_h - header_h - div_h - bottom_h).max(120.0)
                } else {
                    vp_h
                };
                let main_h: f32 = if compact {
                    body_h
                } else {
                    (vp_h - desktop_bar_h - div_h).max(120.0)
                };
                if !compact {
                    let _: bool = menu_open.set_if_changed(false);
                }
                handle_layout.with_write(
                    |tree: &mut SceneTree| {
                        let target_sidebar_parent: NodeId =
                            if compact { drawer_scroll } else { nav_rail };
                        let cur_parent: Option<NodeId> =
                            tree.get_node(sidebar).and_then(SceneNode::parent_id);
                        if cur_parent != Some(target_sidebar_parent) {
                            tree.append_child(target_sidebar_parent, sidebar);
                        }
                        if let Some(footer_id) = sidebar_footer {
                            let cur_f: Option<NodeId> =
                                tree.get_node(footer_id).and_then(SceneNode::parent_id);
                            if cur_f != Some(target_sidebar_parent) {
                                tree.append_child(target_sidebar_parent, footer_id);
                            }
                        }
                        let dirty_l: u16 = DIRTY_LAYOUT | DIRTY_SUBTREE_LAYOUT;
                        let mask_all: u16 = dirty_l | DIRTY_PAINT;
                        tree.mark_dirty(sidebar, mask_all);
                        let nav_children: Vec<NodeId> = tree
                            .get_node(sidebar)
                            .map(|node: &SceneNode| node.get_children().clone())
                            .unwrap_or_default();
                        for child_id in nav_children.into_iter() {
                            tree.mark_dirty(child_id, mask_all);
                        }
                        if let Some(n) = tree.get_node_mut(top_bar) {
                            let mut st: Style = n.get_style();
                            st.get_mut_layout().set_display(if compact {
                                Display::Flex
                            } else {
                                Display::None
                            });
                            st.get_mut_layout().set_height(Length::Px(header_h));
                            st.get_mut_layout().set_padding(Edges::new(
                                top_inset,
                                SPACE_MD + right_inset,
                                0.0,
                                SPACE_MD + left_inset,
                            ));
                            n.set_style(st);
                        }
                        if let Some(n) = tree.get_node_mut(top_bar_divider) {
                            let mut st: Style = n.get_style();
                            st.get_mut_layout().set_display(if compact {
                                Display::Flex
                            } else {
                                Display::None
                            });
                            n.set_style(st);
                        }
                        if let Some(n) = tree.get_node_mut(desktop_top_bar) {
                            let mut st: Style = n.get_style();
                            st.get_mut_layout().set_display(if compact {
                                Display::None
                            } else {
                                Display::Flex
                            });
                            st.get_mut_layout().set_height(Length::Px(desktop_bar_h));
                            let bar_pad: f32 = if is_medium { SPACE_MD } else { SPACE_XL };
                            st.get_mut_layout().set_padding(Edges::new(
                                top_inset,
                                bar_pad + right_inset,
                                0.0,
                                bar_pad,
                            ));
                            n.set_style(st);
                        }
                        if let Some(n) = tree.get_node_mut(desktop_top_divider) {
                            let mut st: Style = n.get_style();
                            st.get_mut_layout().set_display(if compact {
                                Display::None
                            } else {
                                Display::Flex
                            });
                            n.set_style(st);
                        }
                        if let Some(n) = tree.get_node_mut(body_row) {
                            let mut st: Style = n.get_style();
                            st.get_mut_layout().set_height(Length::Px(body_h));
                            n.set_style(st);
                        }
                        if let Some(n) = tree.get_node_mut(content_column) {
                            let mut st: Style = n.get_style();
                            st.get_mut_layout().set_height(Length::Px(body_h));
                            n.set_style(st);
                        }
                        let nav_w: f32 = if is_medium { 204.0 } else { NAV_SIDEBAR_WIDTH };
                        let nav_pad: f32 = if is_medium { SPACE_MD } else { SPACE_LG };
                        if let Some(n) = tree.get_node_mut(nav_divider) {
                            let mut st: Style = n.get_style();
                            st.get_mut_layout().set_display(if compact {
                                Display::None
                            } else {
                                Display::Flex
                            });
                            n.set_style(st);
                        }
                        if let Some(n) = tree.get_node_mut(nav_rail) {
                            let mut st: Style = n.get_style();
                            st.get_mut_layout().set_display(if compact {
                                Display::None
                            } else {
                                Display::Flex
                            });
                            st.get_mut_layout().set_width(Length::Px(nav_w));
                            st.get_mut_layout().set_padding(Edges::new(
                                nav_pad + top_inset,
                                nav_pad,
                                nav_pad + bottom_inset,
                                nav_pad + left_inset,
                            ));
                            n.set_style(st);
                        }
                        let main_pad: f32 = if bp.is_compact() {
                            SPACE_MD
                        } else if bp == Breakpoint::Medium {
                            SPACE_LG
                        } else {
                            SPACE_XL
                        };
                        let main_bottom_pad: f32 = if has_bottom_nav {
                            main_pad
                        } else {
                            main_pad + bottom_inset
                        };
                        if let Some(n) = tree.get_node_mut(main_area) {
                            let mut st: Style = n.get_style();
                            st.get_mut_layout().set_height(Length::Px(main_h));
                            st.get_mut_layout().set_padding(Edges::new(
                                main_pad,
                                main_pad + right_inset,
                                main_bottom_pad,
                                main_pad + left_inset,
                            ));
                            st = st.gap(if bp.is_compact() { SPACE_MD } else { SPACE_LG });
                            n.set_style(st);
                        }
                        if has_bottom_nav && let Some(n) = tree.get_node_mut(bottom_bar_host) {
                            let mut st: Style = n.get_style();
                            st.get_mut_layout()
                                .set_height(Length::Px(SHELL_BOTTOM_NAV_HEIGHT + bottom_inset));
                            st.get_mut_layout().set_padding(Edges::new(
                                0.0,
                                right_inset,
                                bottom_inset,
                                left_inset,
                            ));
                            n.set_style(st);
                        }
                        if let Some(n) = tree.get_node_mut(drawer_scroll) {
                            let mut st: Style = n.get_style();
                            st.get_mut_layout().set_padding(Edges::new(
                                SPACE_LG + top_inset,
                                SPACE_SM,
                                SPACE_LG + bottom_inset,
                                SPACE_LG + left_inset,
                            ));
                            n.set_style(st);
                        }
                        if let Some(n) = tree.get_node_mut(drawer_panel) {
                            let vp_w: f32 = vp.get_width().max(320.0);
                            let def_w: f32 = (vp_w * 0.68).clamp(224.0, 256.0);
                            let max_w: f32 = (vp_w * 0.88).clamp(def_w + 40.0, 380.0);
                            let target_w: f32 = if drawer_expanded.get() { max_w } else { def_w };
                            let mut st: Style = n.get_style();
                            st.get_mut_layout().set_width(Length::Px(target_w));
                            n.set_style(st);
                        }
                        let mask: u16 = DIRTY_LAYOUT | DIRTY_SUBTREE_LAYOUT;
                        tree.mark_dirty(root, mask | DIRTY_PAINT);
                    },
                    (),
                );
            });
            let last_g_seq: Rc<Cell<u64>> = Rc::new(Cell::new(
                gesture_sig
                    .get_untracked()
                    .map(|ev: GestureEvent| ev.get_sequence())
                    .unwrap_or(0),
            ));
            let handle_swipe: SceneHandle = handle.clone();
            let _: Effect = Effect::new(move || {
                if let Some(ev) = gesture_sig.get() {
                    let seq: u64 = ev.get_sequence();
                    if seq != last_g_seq.replace(seq) {
                        if menu_open.get_untracked() {
                            if ev.get_kind() == GestureEventKind::SwipeLeft {
                                let _: bool = menu_open.set_if_changed(false);
                            }
                            return;
                        }
                        if !swipe_routes.is_empty()
                            && let Some(route_sig) = route_signal
                        {
                            let cur_path: String = route_sig.get_untracked();
                            let cur_idx: usize = swipe_routes
                                .iter()
                                .position(|r: &String| r.as_str() == cur_path.as_str())
                                .unwrap_or(0);
                            if ev.get_kind() == GestureEventKind::SwipeLeft
                                && cur_idx + 1 < swipe_routes.len()
                            {
                                let next_path: String = swipe_routes[cur_idx + 1].clone();
                                if route_sig.set_if_changed(next_path) {
                                    handle_swipe.start_transition(
                                        NodeTransition::slide_fade(
                                            main_area,
                                            Point::new(28.0, 0.0),
                                            Point::new(0.0, 0.0),
                                            0.88,
                                            1.0,
                                            0.18,
                                            false,
                                        )
                                        .with_easing(Easing::EaseOutQuint),
                                    );
                                }
                            } else if ev.get_kind() == GestureEventKind::SwipeRight {
                                if cur_idx > 0 {
                                    let prev_path: String = swipe_routes[cur_idx - 1].clone();
                                    if route_sig.set_if_changed(prev_path) {
                                        handle_swipe.start_transition(
                                            NodeTransition::slide_fade(
                                                main_area,
                                                Point::new(-28.0, 0.0),
                                                Point::new(0.0, 0.0),
                                                0.88,
                                                1.0,
                                                0.18,
                                                false,
                                            )
                                            .with_easing(Easing::EaseOutQuint),
                                        );
                                    }
                                } else {
                                    let _: bool = menu_open.set_if_changed(true);
                                }
                            }
                        } else if bp_sig.get_untracked().is_compact()
                            && ev.get_kind() == GestureEventKind::SwipeRight
                        {
                            let _: bool = menu_open.set_if_changed(true);
                        }
                    }
                }
            });
            if let Some(route_sig) = route_signal {
                let prev_route: Rc<RefCell<String>> =
                    Rc::new(RefCell::new(route_sig.get_untracked()));
                let handle_route_scroll: SceneHandle = handle.clone();
                let _: Effect = Effect::new(move || {
                    let next_r: String = route_sig.get();
                    if let Ok(mut guard) = prev_route.try_borrow_mut()
                        && *guard != next_r
                    {
                        *guard = next_r;
                        let _: bool = menu_open.set_if_changed(false);
                        handle_route_scroll.with_write(
                            |tree: &mut SceneTree| {
                                tree.set_scroll_offset(main_area, Point::new(0.0, 0.0));
                            },
                            (),
                        );
                    }
                });
            }
            Effect::new(move || {
                let is_open: bool = menu_open.get();
                let was_open: bool = prev_open.replace(is_open);
                if is_open == was_open {
                    return;
                }
                if is_open {
                    let _: bool = drawer_expanded.set_if_changed(false);
                    let vp_w: f32 = vp_sig.get_untracked().get_width().max(320.0);
                    let def_w: f32 = (vp_w * 0.68).clamp(224.0, 256.0);
                    handle.with_write(
                        |tree: &mut SceneTree| {
                            if let Some(n) = tree.get_node_mut(drawer_portal) {
                                let mut st: Style = n.get_style();
                                st.get_mut_paint().set_pointer_events(PointerEvents::Auto);
                                n.set_style(st);
                            }
                            if let Some(n) = tree.get_node_mut(drawer_panel) {
                                let mut st: Style = n.get_style();
                                st.get_mut_layout().set_width(Length::Px(def_w));
                                st.get_mut_paint().set_transform(Transform2D::default());
                                n.set_style(st);
                            }
                            let cur_y: f32 = tree
                                .get_node(drawer_scroll)
                                .map(|n: &SceneNode| n.get_layout_box().get_scroll_offset().get_y())
                                .unwrap_or(0.0);
                            tree.set_scroll_offset(drawer_scroll, Point::new(0.0, cur_y));
                            drop(tree.update_focus(None));
                        },
                        (),
                    );
                    handle.start_transition(
                        NodeTransition::fade(drawer_portal, 0.0, 1.0, 0.22, false)
                            .with_easing(Easing::EaseOutCubic),
                    );
                    handle.start_transition(
                        NodeTransition::slide_fade(
                            drawer_panel,
                            Point::new(-def_w, 0.0),
                            Point::new(0.0, 0.0),
                            1.0,
                            1.0,
                            0.26,
                            false,
                        )
                        .with_easing(Easing::EaseOutQuint),
                    );
                } else {
                    let _: bool = drawer_expanded.set_if_changed(false);
                    let vp_w: f32 = vp_sig.get_untracked().get_width().max(320.0);
                    let slide_w: f32 = (vp_w * 0.88).clamp(256.0, 380.0);
                    handle.with_write(
                        |tree: &mut SceneTree| {
                            if let Some(n) = tree.get_node_mut(drawer_portal) {
                                let mut st: Style = n.get_style();
                                st.get_mut_paint().set_pointer_events(PointerEvents::None);
                                n.set_style(st);
                            }
                        },
                        (),
                    );
                    handle.start_transition(
                        NodeTransition::slide_fade(
                            drawer_panel,
                            Point::new(0.0, 0.0),
                            Point::new(-slide_w, 0.0),
                            1.0,
                            1.0,
                            0.20,
                            false,
                        )
                        .with_easing(Easing::EaseOutCubic),
                    );
                    handle.start_transition(
                        NodeTransition::fade(drawer_portal, 1.0, 0.0, 0.20, true)
                            .with_easing(Easing::EaseOutCubic),
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

/// Construction and mounting for `EuvDivider`.
impl EuvDivider {
    /// Creates a horizontal divider.
    ///
    /// # Arguments
    ///
    /// - `ThemeTokens` - Active design tokens.
    ///
    /// # Returns
    ///
    /// - `Self` - Horizontal divider.
    #[must_use]
    pub fn horizontal(tokens: ThemeTokens) -> Self {
        Self {
            vertical: false,
            tokens,
        }
    }

    /// Creates a vertical divider.
    ///
    /// # Arguments
    ///
    /// - `ThemeTokens` - Active design tokens.
    ///
    /// # Returns
    ///
    /// - `Self` - Vertical divider.
    #[must_use]
    pub fn vertical(tokens: ThemeTokens) -> Self {
        Self {
            vertical: true,
            tokens,
        }
    }
}

/// Declarative `#[component]` implementation for `EuvDivider`.
#[component]
/// Inherent implementation of [`EuvDivider`].
impl EuvDivider {
    /// Renders `EuvDivider` into the ambient framework-managed `Context`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component descriptor reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted root scene node handle.
    fn render(&self) -> NodeId {
        let border_color: Color = self.get_tokens().get_palette().get_border();
        let is_vert: bool = self.get_vertical();
        view! {
            view {
                width: if is_vert { Length::Px(1.0) } else { Length::Percent(100.0) }
                height: if is_vert { Length::Percent(100.0) } else { Length::Px(1.0) }
                background: border_color
            }
        }
    }
}

/// Fluent section composition helpers for `EuvPage`.
impl EuvPage {
    /// Appends a section node below the page header.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source page wrapper.
    /// - `NodeId` - Mounted section node.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated page wrapper.
    #[must_use]
    pub fn section(mut self, node: NodeId) -> Self {
        self.get_mut_sections().push(node);
        self
    }
}

/// Declarative `#[component]` implementation for `EuvPage`.
#[component]
/// Inherent implementation of [`EuvPage`].
impl EuvPage {
    /// Renders `EuvPage` into the ambient framework-managed `Context`.
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
        let Self {
            title,
            subtitle,
            sections,
            ..
        } = self.clone();
        view! {
            column {
                width: Length::Percent(100.0)
                max_width: Length::Px(1040.0)
                gap: SPACE_LG
                EuvHeader(title, subtitle, tokens)
                ..sections
            }
        }
    }
}

/// Fluent child composition helpers for `EuvActionRow`.
impl EuvActionRow {
    /// Appends a control node to the wrapping horizontal action bar.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source action row.
    /// - `NodeId` - Mounted control node.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated action row.
    #[must_use]
    pub fn child(mut self, node: NodeId) -> Self {
        self.get_mut_children().push(node);
        self
    }
}

/// Declarative `#[component]` implementation for `EuvActionRow`.
#[component]
/// Inherent implementation of [`EuvActionRow`].
impl EuvActionRow {
    /// Renders `EuvActionRow` into the ambient framework-managed `Context`.
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
        let bp_sig: Signal<Breakpoint> = ctx.get_breakpoint();
        let Self { children } = self.clone();
        let root: NodeId = view! {
            column {
                width: Length::Percent(100.0)
                gap: SPACE_SM
            }
        };
        let (orig_widths, wide_flags): (Vec<Length>, Vec<bool>) = ctx.get_scene().with_read(
            |tree: &SceneTree| {
                let mut widths: Vec<Length> = Vec::with_capacity(children.len());
                let mut wide: Vec<bool> = Vec::with_capacity(children.len());
                for cid in children.iter().copied() {
                    let w: Length = tree
                        .get_node(cid)
                        .map(|n: &SceneNode| n.get_style().get_layout().get_width())
                        .unwrap_or(Length::Auto);
                    let label_len: usize = tree
                        .get_node(cid)
                        .map(|n: &SceneNode| {
                            let direct: usize = match n.get_kind() {
                                NodeKind::Text(t) => t.chars().count(),
                                _ => 0,
                            };
                            let child_max: usize = n
                                .get_children()
                                .iter()
                                .filter_map(|sub: &NodeId| tree.get_node(*sub))
                                .map(|sub_n: &SceneNode| match sub_n.get_kind() {
                                    NodeKind::Text(t) => t.chars().count(),
                                    _ => 0,
                                })
                                .max()
                                .unwrap_or(0);
                            direct.max(child_max)
                        })
                        .unwrap_or(0);
                    let is_switch_like: bool = tree
                        .get_node(cid)
                        .map(|n: &SceneNode| {
                            n.get_children().len() > 1
                                || matches!(
                                    n.a11y_props().map(|a: A11yProps| a.get_role()),
                                    Some(A11yRole::Switch | A11yRole::Checkbox)
                                )
                        })
                        .unwrap_or(false);
                    widths.push(w);
                    wide.push(is_switch_like || label_len > 11);
                }
                (widths, wide)
            },
            (
                vec![Length::Auto; children.len()],
                vec![false; children.len()],
            ),
        );
        let pair_count: usize = children.len() / 2;
        let mut pair_rows: Vec<NodeId> = Vec::with_capacity(pair_count);
        for _ in 0..pair_count {
            let pr: NodeId = view! {
                grid {
                    columns: 2
                    width: Length::Percent(100.0)
                    gap: SPACE_SM
                }
            };
            ctx.append(root, pr);
            pair_rows.push(pr);
        }
        let handle: SceneHandle = ctx.get_scene().clone();
        let children_c: Vec<NodeId> = children.clone();
        let pair_rows_c: Vec<NodeId> = pair_rows.clone();
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            Effect::new(move || {
                let compact: bool = bp_sig.get().is_compact();
                handle.with_write(
                    |tree: &mut SceneTree| {
                        if compact {
                            if let Some(n) = tree.get_node_mut(root) {
                                let mut st: Style = n.get_style();
                                st.get_mut_layout()
                                    .set_flex_direction(FlexDirection::Column)
                                    .set_flex_wrap(FlexWrap::NoWrap)
                                    .set_align_items(AlignItems::Stretch);
                                n.set_style(st);
                            }
                            for pair_row in pair_rows_c.iter().copied() {
                                if let Some(n) = tree.get_node_mut(pair_row) {
                                    let mut st: Style = n.get_style();
                                    st.get_mut_layout().set_display(Display::None);
                                    n.set_style(st);
                                }
                            }
                            let mut idx: usize = 0;
                            let mut pair_slot: usize = 0;
                            while idx < children_c.len() {
                                let a: NodeId = children_c[idx];
                                let a_wide: bool = wide_flags.get(idx).copied().unwrap_or(false);
                                let can_pair: bool = !a_wide
                                    && idx + 1 < children_c.len()
                                    && !wide_flags.get(idx + 1).copied().unwrap_or(false)
                                    && pair_slot < pair_rows_c.len();
                                if can_pair {
                                    let pair_row: NodeId = pair_rows_c[pair_slot];
                                    pair_slot += 1;
                                    if let Some(n) = tree.get_node_mut(pair_row) {
                                        let mut st: Style = n.get_style();
                                        st.get_mut_layout()
                                            .set_display(Display::Grid)
                                            .set_grid_columns(2);
                                        n.set_style(st);
                                    }
                                    tree.append_child(root, pair_row);
                                    let b: NodeId = children_c[idx + 1];
                                    for item in [a, b].into_iter() {
                                        if let Some(n) = tree.get_node_mut(item) {
                                            let mut st: Style = n.get_style();
                                            st.get_mut_layout().set_width(Length::Percent(100.0));
                                            n.set_style(st);
                                        }
                                        tree.append_child(pair_row, item);
                                    }
                                    idx += 2;
                                } else {
                                    if let Some(n) = tree.get_node_mut(a) {
                                        let mut st: Style = n.get_style();
                                        st.get_mut_layout().set_width(Length::Percent(100.0));
                                        n.set_style(st);
                                    }
                                    tree.append_child(root, a);
                                    idx += 1;
                                }
                            }
                        } else {
                            if let Some(n) = tree.get_node_mut(root) {
                                let mut st: Style = n.get_style();
                                st.get_mut_layout()
                                    .set_flex_direction(FlexDirection::Row)
                                    .set_flex_wrap(FlexWrap::Wrap)
                                    .set_align_items(AlignItems::Center);
                                n.set_style(st);
                            }
                            for pair_row in pair_rows_c.iter().copied() {
                                if let Some(n) = tree.get_node_mut(pair_row) {
                                    let mut st: Style = n.get_style();
                                    st.get_mut_layout().set_display(Display::None);
                                    n.set_style(st);
                                }
                            }
                            for (idx, cid) in children_c.iter().copied().enumerate() {
                                if let Some(n) = tree.get_node_mut(cid) {
                                    let mut st: Style = n.get_style();
                                    let ow: Length =
                                        orig_widths.get(idx).copied().unwrap_or(Length::Auto);
                                    st.get_mut_layout().set_width(ow);
                                    n.set_style(st);
                                }
                                tree.append_child(root, cid);
                            }
                        }
                        let dirty_l: u16 = DIRTY_LAYOUT | DIRTY_SUBTREE_LAYOUT;
                        tree.mark_dirty(root, dirty_l | DIRTY_PAINT);
                    },
                    (),
                );
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

/// Fluent card composition helpers for `EuvStatGrid`.
impl EuvStatGrid {
    /// Appends a KPI stat card node to the responsive grid.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source stat grid.
    /// - `NodeId` - Mounted stat card node.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated stat grid.
    #[must_use]
    pub fn card(mut self, node: NodeId) -> Self {
        self.get_mut_cards().push(node);
        self
    }
}

/// Declarative `#[component]` implementation for `EuvStatGrid`.
#[component]
/// Inherent implementation of [`EuvStatGrid`].
impl EuvStatGrid {
    /// Renders `EuvStatGrid` into the ambient framework-managed `Context`.
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
        let bp_sig: Signal<Breakpoint> = ctx.get_breakpoint();
        let Self { cards } = self.clone();
        let max_cols: u16 = (cards.len() as u16).clamp(1, 3);
        view! {
            grid {
                bind_style: move || {
                    let cols: u16 = match bp_sig.get() {
                        Breakpoint::Compact => 1,
                        Breakpoint::Medium => max_cols.min(2),
                        Breakpoint::Expanded => max_cols,
                    };
                    Style::default()
                    .grid(cols)
                    .width(Length::Percent(100.0))
                    .gap(SPACE_MD)
                }
                ..cards
            }
        }
    }
}
