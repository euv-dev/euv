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
        let Self {
            brand,
            sidebar,
            main_content,
            route_signal,
            top_bar_actions,
            mobile_header_actions,
            sidebar_footer,
            breadcrumb,
            ..
        } = self.clone();
        let brand_initial: String = brand
            .chars()
            .next()
            .map(|c: char| c.to_ascii_uppercase().to_string())
            .unwrap_or_else(|| SHELL_DEFAULT_INITIAL.to_string());
        let menu_open: Signal<bool> = Signal::new(false);
        let div_h: f32 = g.get_border_width().max(1.0);
        let mut top_bar: NodeId = NodeId::default();
        let mut top_bar_divider: NodeId = NodeId::default();
        let mut body_row: NodeId = NodeId::default();
        let mut nav_rail: NodeId = NodeId::default();
        let mut nav_divider: NodeId = NodeId::default();
        let mut content_column: NodeId = NodeId::default();
        let mut desktop_top_bar: NodeId = NodeId::default();
        let mut desktop_top_divider: NodeId = NodeId::default();
        let mut crumb_txt: NodeId = NodeId::default();
        let mut main_area: NodeId = NodeId::default();
        let mut drawer_portal: NodeId = NodeId::default();
        let mut drawer_panel: NodeId = NodeId::default();
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
                    text {
                        flex_shrink: 1.0
                        font_size: 15.0
                        font_weight: FontWeight::Bold
                        color: p.get_foreground()
                        text_overflow: TextOverflow::Ellipsis
                        { brand.clone() }
                    }
                    row {
                        align: AlignItems::Center
                        gap: SPACE_XS
                        if let Some(mob_act) = mobile_header_actions {
                            { mob_act }
                        }
                        EuvButton(SHELL_MENU_LABEL.to_string(), tokens) {
                            outline()
                            sm()
                            on_click: move |_: &UiEvent| {
                                let _: bool = menu_open.update(|v: &mut bool| *v = !*v);
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
                            if let Some(actions_node) = top_bar_actions {
                                { actions_node }
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
                    scroll {
                        node_ref: drawer_panel
                        style: Style::default().column()
                        width: Length::Percent(64.0)
                        max_width: Length::Px(232.0)
                        height: Length::Percent(100.0)
                        gap: SPACE_SM
                        background: p.get_card()
                        border: (g.get_border_width(), p.get_border())
                        shadow: g.get_shadow_lg()
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
                }
            }
        };
        ctx.on_click(drawer_portal, move |ev: &UiEvent| {
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
                let _: Effect = handle_crumb.bind_text(crumb_txt, move || crumb_sig.get());
            }
            let _: Effect = Effect::new(move || {
                let bp: Breakpoint = bp_sig.get();
                let vp: Size = vp_sig.get();
                let safe: SafeAreaInsets = safe_sig.get();
                let compact: bool = bp.is_compact();
                let is_medium: bool = bp == Breakpoint::Medium;
                let top_inset: f32 = safe.get_top();
                let bottom_inset: f32 = safe.get_bottom();
                let left_inset: f32 = safe.get_left();
                let right_inset: f32 = safe.get_right();
                let vp_h: f32 = vp.get_height().max(240.0);
                let header_h: f32 = MOBILE_HEADER_HEIGHT + top_inset;
                let desktop_bar_h: f32 = 52.0 + top_inset;
                let body_h: f32 = if compact {
                    (vp_h - header_h - 1.0).max(120.0)
                } else {
                    vp_h
                };
                let main_h: f32 = if compact {
                    body_h
                } else {
                    (vp_h - desktop_bar_h - 1.0).max(120.0)
                };
                if !compact {
                    let _: bool = menu_open.set_if_changed(false);
                }
                handle_layout.with_write(
                    |tree: &mut SceneTree| {
                        let target_sidebar_parent: NodeId =
                            if compact { drawer_panel } else { nav_rail };
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
                        let main_pad: f32 = if compact {
                            SPACE_MD
                        } else if is_medium {
                            SPACE_LG
                        } else {
                            SPACE_XL
                        };
                        if let Some(n) = tree.get_node_mut(main_area) {
                            let mut st: Style = n.get_style();
                            st.get_mut_layout().set_height(Length::Px(main_h));
                            st.get_mut_layout().set_padding(Edges::new(
                                main_pad,
                                main_pad + right_inset,
                                main_pad + bottom_inset,
                                if compact {
                                    SPACE_MD + left_inset
                                } else {
                                    main_pad
                                },
                            ));
                            st = st.gap(if compact { SPACE_MD } else { SPACE_LG });
                            n.set_style(st);
                        }
                        if let Some(n) = tree.get_node_mut(drawer_panel) {
                            let mut st: Style = n.get_style();
                            st.get_mut_layout().set_padding(Edges::new(
                                SPACE_LG + top_inset,
                                SPACE_LG,
                                SPACE_LG + bottom_inset,
                                SPACE_LG + left_inset,
                            ));
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
            let _: Effect = Effect::new(move || {
                if let Some(ev) = gesture_sig.get() {
                    let seq: u64 = ev.get_sequence();
                    if seq != last_g_seq.replace(seq) && bp_sig.get_untracked().is_compact() {
                        if ev.get_kind() == GestureEventKind::SwipeRight
                            && !menu_open.get_untracked()
                        {
                            let _: bool = menu_open.set_if_changed(true);
                        } else if ev.get_kind() == GestureEventKind::SwipeLeft
                            && menu_open.get_untracked()
                        {
                            let _: bool = menu_open.set_if_changed(false);
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
                    handle.with_write(
                        |tree: &mut SceneTree| {
                            if let Some(n) = tree.get_node_mut(drawer_portal) {
                                let mut st: Style = n.get_style();
                                st.get_mut_paint().set_pointer_events(PointerEvents::Auto);
                                n.set_style(st);
                            }
                            let cur_y: f32 = tree
                                .get_node(drawer_panel)
                                .map(|n: &SceneNode| n.get_layout_box().get_scroll_offset().get_y())
                                .unwrap_or(0.0);
                            tree.set_scroll_offset(drawer_panel, Point::new(0.0, cur_y));
                            drop(tree.update_focus(None));
                        },
                        (),
                    );
                    handle.start_transition(
                        NodeTransition::fade(drawer_portal, 0.0, 1.0, 0.24, false)
                            .with_easing(Easing::EaseOutQuint),
                    );
                    handle.start_transition(
                        NodeTransition::slide_fade(
                            drawer_panel,
                            Point::new(-248.0, 0.0),
                            Point::new(0.0, 0.0),
                            0.92,
                            1.0,
                            0.34,
                            false,
                        )
                        .with_scale(0.97, 1.0)
                        .with_easing(Easing::SpringGentle),
                    );
                } else {
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
                            Point::new(-248.0, 0.0),
                            1.0,
                            0.94,
                            0.20,
                            false,
                        )
                        .with_scale(1.0, 0.98)
                        .with_easing(Easing::EaseOutQuint),
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
