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
}

/// `Mountable` implementation adapting shell layout to `Breakpoint` and `SafeAreaInsets`.
impl Mountable for EuvAppShell {
    /// Mounts the adaptive shell respecting safe-area insets on mobile/tablet and sidebar on desktop.
    ///
    /// # Arguments
    ///
    /// - `Self` - Owned shell props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Root shell node handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let tokens: ThemeTokens = self.get_tokens();
        let p: ThemePalette = tokens.get_palette();
        let g: ThemeGeometry = tokens.get_geometry();
        let bp: Breakpoint = ctx.get_breakpoint().get_untracked();
        let safe: SafeAreaInsets = ctx.get_safe_area().get_untracked();
        let Self {
            brand,
            sidebar,
            main_content,
            route_signal,
            ..
        } = self;
        let root_style: Style = Style::empty()
            .width(Length::Percent(100.0))
            .height(Length::Percent(100.0))
            .background(p.get_background())
            .overflow(Overflow::Hidden);
        if bp.is_compact() {
            let root: NodeId = ctx.view(root_style.column());
            let menu_open: Signal<bool> = Signal::new(false);
            let top_inset: f32 = safe.get_top();
            let bottom_inset: f32 = safe.get_bottom();
            let left_inset: f32 = safe.get_left();
            let right_inset: f32 = safe.get_right();
            let top_bar: NodeId = ctx.view(
                Style::empty()
                    .row()
                    .width(Length::Percent(100.0))
                    .height(Length::Px(MOBILE_HEADER_HEIGHT + top_inset))
                    .padding(Edges::new(
                        top_inset,
                        SPACE_MD + right_inset,
                        0.0,
                        SPACE_MD + left_inset,
                    ))
                    .justify(JustifyContent::SpaceBetween)
                    .align(AlignItems::Center)
                    .background(p.get_card())
                    .border(g.get_border_width(), p.get_border())
                    .z_index(Z_INDEX_CHROME),
            );
            let title: NodeId = ctx.text(
                brand.clone(),
                Style::empty()
                    .font_size(15.0)
                    .font_weight(FontWeight::Bold)
                    .color(p.get_foreground()),
            );
            let menu_btn_node: NodeId = EuvButton::new("Menu".to_string(), tokens)
                .outline()
                .sm()
                .on_click(move |_: &UiEvent| {
                    let _: bool = menu_open.update(|v: &mut bool| *v = !*v);
                })
                .mount(ctx);
            ctx.append(top_bar, title);
            ctx.append(top_bar, menu_btn_node);
            let vp_h: f32 = ctx
                .get_viewport_size()
                .get_untracked()
                .get_height()
                .max(240.0);
            let header_h: f32 = MOBILE_HEADER_HEIGHT + top_inset;
            let body_h: f32 = (vp_h - header_h).max(120.0);
            let body: NodeId = ctx.scroll_view(
                Style::empty()
                    .column()
                    .flex_grow(1.0)
                    .width(Length::Percent(100.0))
                    .height(Length::Px(body_h))
                    .padding(Edges::new(
                        SPACE_MD,
                        SPACE_MD + right_inset,
                        SPACE_MD + bottom_inset,
                        SPACE_MD + left_inset,
                    ))
                    .gap(SPACE_MD),
            );
            ctx.append(body, main_content);
            let drawer_portal: NodeId = ctx.portal(
                Style::empty()
                    .row()
                    .display(Display::None)
                    .position(Position::Fixed)
                    .inset(Some(0.0), Some(0.0), Some(0.0), Some(0.0))
                    .width(Length::Percent(100.0))
                    .height(Length::Percent(100.0))
                    .justify(JustifyContent::Start)
                    .background(p.get_overlay())
                    .z_index(Z_INDEX_DRAWER),
            );
            let drawer_panel: NodeId = ctx.scroll_view(
                Style::empty()
                    .column()
                    .width(Length::Percent(82.0))
                    .max_width(Length::Px(280.0))
                    .height(Length::Percent(100.0))
                    .padding(Edges::new(
                        SPACE_LG + top_inset,
                        SPACE_LG,
                        SPACE_LG + bottom_inset,
                        SPACE_LG + left_inset,
                    ))
                    .gap(SPACE_MD)
                    .background(p.get_card())
                    .border(g.get_border_width(), p.get_border())
                    .shadow(Shadow::new(4.0, 0.0, 20.0, 0.0, Color::rgba(0, 0, 0, 64))),
            );
            let drawer_head: NodeId = ctx.view(
                Style::empty()
                    .row()
                    .width(Length::Percent(100.0))
                    .justify(JustifyContent::SpaceBetween)
                    .align(AlignItems::Center),
            );
            let drawer_title: NodeId = ctx.text(
                brand,
                Style::empty()
                    .font_size(15.0)
                    .font_weight(FontWeight::Bold)
                    .color(p.get_foreground()),
            );
            let close_node: NodeId = EuvButton::new("Close".to_string(), tokens)
                .ghost()
                .sm()
                .on_click(move |_: &UiEvent| {
                    let _: bool = menu_open.set_if_changed(false);
                })
                .mount(ctx);
            ctx.append(drawer_head, drawer_title);
            ctx.append(drawer_head, close_node);
            ctx.append(drawer_panel, drawer_head);
            ctx.append(drawer_panel, sidebar);
            ctx.append(drawer_portal, drawer_panel);
            ctx.on_click(drawer_portal, move |ev: &UiEvent| {
                if ev.get_target() == drawer_portal {
                    let _: bool = menu_open.set_if_changed(false);
                }
            });
            let handle: SceneHandle = ctx.get_scene().clone();
            let gesture_sig: Signal<Option<GestureEvent>> = ctx.get_gesture();
            let prev_open: Rc<Cell<bool>> = Rc::new(Cell::new(false));
            let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
                let last_g_seq: Rc<Cell<u64>> = Rc::new(Cell::new(
                    gesture_sig
                        .get_untracked()
                        .map(|ev: GestureEvent| ev.get_sequence())
                        .unwrap_or(0),
                ));
                let _: Effect = Effect::new(move || {
                    if let Some(ev) = gesture_sig.get() {
                        let seq: u64 = ev.get_sequence();
                        if seq != last_g_seq.replace(seq) {
                            if ev.get_kind() == GestureEventKind::SwipeRight
                                && ev.get_origin().get_x() <= 48.0
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
                    let _: Effect = Effect::new(move || {
                        let next_r: String = route_sig.get();
                        if let Ok(mut guard) = prev_route.try_borrow_mut()
                            && *guard != next_r
                        {
                            *guard = next_r;
                            let _: bool = menu_open.set_if_changed(false);
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
                            },
                            (),
                        );
                        handle.start_transition(NodeTransition::fade(
                            drawer_portal,
                            0.0,
                            1.0,
                            0.22,
                            false,
                        ));
                        handle.start_transition(NodeTransition::slide_fade(
                            drawer_panel,
                            Point::new(-280.0, 0.0),
                            Point::new(0.0, 0.0),
                            0.92,
                            1.0,
                            0.24,
                            false,
                        ));
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
                        handle.start_transition(NodeTransition::slide_fade(
                            drawer_panel,
                            Point::new(0.0, 0.0),
                            Point::new(-280.0, 0.0),
                            1.0,
                            0.9,
                            0.22,
                            false,
                        ));
                        handle.start_transition(NodeTransition::fade(
                            drawer_portal,
                            1.0,
                            0.0,
                            0.22,
                            true,
                        ));
                    }
                })
            });
            ctx.get_scene().with_write(
                |tree: &mut SceneTree| {
                    tree.attach_scope(drawer_portal, scope);
                },
                (),
            );
            ctx.append(root, top_bar);
            ctx.append(root, body);
            ctx.append(root, drawer_portal);
            root
        } else {
            let root: NodeId = ctx.view(root_style.row());
            let is_medium: bool = bp == Breakpoint::Medium;
            let nav_w: f32 = if is_medium { 192.0 } else { NAV_SIDEBAR_WIDTH };
            let nav_pad: f32 = if is_medium { SPACE_MD } else { SPACE_LG };
            let main_pad: f32 = if is_medium { SPACE_LG } else { SPACE_2XL };
            let top_inset: f32 = safe.get_top();
            let bottom_inset: f32 = safe.get_bottom();
            let nav_rail: NodeId = ctx.scroll_view(
                Style::empty()
                    .column()
                    .width(Length::Px(nav_w))
                    .height(Length::Percent(100.0))
                    .padding(Edges::new(
                        nav_pad + top_inset,
                        nav_pad,
                        nav_pad + bottom_inset,
                        nav_pad + safe.get_left(),
                    ))
                    .gap(SPACE_MD)
                    .background(p.get_card())
                    .border(g.get_border_width(), p.get_border())
                    .z_index(Z_INDEX_CHROME),
            );
            let brand_node: NodeId = ctx.text(
                brand,
                Style::empty()
                    .font_size(16.0)
                    .font_weight(FontWeight::Bold)
                    .color(p.get_foreground()),
            );
            ctx.append(nav_rail, brand_node);
            ctx.append(nav_rail, sidebar);
            let main_area: NodeId = ctx.scroll_view(
                Style::empty()
                    .column()
                    .flex_grow(1.0)
                    .height(Length::Percent(100.0))
                    .padding(Edges::new(
                        main_pad + top_inset,
                        main_pad + safe.get_right(),
                        main_pad + bottom_inset,
                        main_pad,
                    ))
                    .gap(SPACE_LG),
            );
            ctx.append(main_area, main_content);
            ctx.append(root, nav_rail);
            ctx.append(root, main_area);
            root
        }
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

/// `Mountable` implementation for `EuvDivider`.
impl Mountable for EuvDivider {
    /// Mounts a 1px hairline separator node.
    ///
    /// # Arguments
    ///
    /// - `Self` - Divider props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted separator node.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let border_color: Color = self.get_tokens().get_palette().get_border();
        let style: Style = if self.get_vertical() {
            Style::empty()
                .width(Length::Px(1.0))
                .height(Length::Percent(100.0))
                .background(border_color)
        } else {
            Style::empty()
                .width(Length::Percent(100.0))
                .height(Length::Px(1.0))
                .background(border_color)
        };
        ctx.view(style)
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

/// `Mountable` implementation for `EuvPage`.
impl Mountable for EuvPage {
    /// Mounts the standardized page column with `EuvHeader` and uniform `SPACE_LG` section rhythm.
    ///
    /// # Arguments
    ///
    /// - `Self` - Page descriptor.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted page container handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let tokens: ThemeTokens = self.get_tokens();
        let Self {
            title,
            subtitle,
            sections,
            ..
        } = self;
        let col: NodeId = ctx.view(
            Style::empty()
                .column()
                .width(Length::Percent(100.0))
                .gap(SPACE_LG),
        );
        let header: NodeId = EuvHeader::new(title, subtitle, tokens).mount(ctx);
        ctx.append(col, header);
        for sec in sections.into_iter() {
            ctx.append(col, sec);
        }
        col
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

/// `Mountable` implementation for `EuvActionRow`.
impl Mountable for EuvActionRow {
    /// Mounts a wrapping center-aligned horizontal control row with uniform `SPACE_SM` spacing.
    ///
    /// # Arguments
    ///
    /// - `Self` - Action row props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted action row handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let Self { children } = self;
        let row: NodeId = ctx.view(
            Style::empty()
                .row()
                .wrap(FlexWrap::Wrap)
                .width(Length::Percent(100.0))
                .align(AlignItems::Center)
                .gap(SPACE_SM),
        );
        for child in children.into_iter() {
            ctx.append(row, child);
        }
        row
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

/// `Mountable` implementation for `EuvStatGrid`.
impl Mountable for EuvStatGrid {
    /// Mounts a breakpoint-aware 1/2/3-column stat card grid.
    ///
    /// # Arguments
    ///
    /// - `Self` - Stat grid props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted grid node handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let cols: u16 = match ctx.get_breakpoint().get_untracked() {
            Breakpoint::Compact => 1,
            Breakpoint::Medium => 2,
            Breakpoint::Expanded => 3,
        };
        let Self { cards } = self;
        let grid: NodeId = ctx.view(
            Style::empty()
                .grid(cols)
                .width(Length::Percent(100.0))
                .gap(SPACE_MD),
        );
        for card in cards.into_iter() {
            ctx.append(grid, card);
        }
        grid
    }
}
