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

    /// Connects a reactive `UiThemeState` so toggling Light/Dark or switching schemes updates shell chrome in-place without tearing down mounted pages.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source app shell.
    /// - `UiThemeState` - Reactive theme controller.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated app shell.
    #[must_use]
    pub fn theme(mut self, theme: UiThemeState) -> Self {
        self.set_theme(Some(theme));
        self
    }

    /// Connects a shared reactive `Signal<Edges>` updated with the shell's responsive page padding (including safe area and bottom-nav clearance).
    ///
    /// # Arguments
    ///
    /// - `Self` - Source app shell.
    /// - `Signal<Edges>` - Shared page padding signal consumed by `EuvRouterOutlet::with_page_padding`.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated app shell.
    #[must_use]
    pub fn page_padding(mut self, padding: Signal<Edges>) -> Self {
        self.set_page_padding(Some(padding));
        self
    }

    /// Draws the minimalist `Euv Studio` geometric 3-bar `E` monogram inside a gradient squircle emblem.
    ///
    /// # Arguments
    ///
    /// - `Rect` - Emblem canvas bounds in logical pixels.
    /// - `&mut DisplayList` - Target display list.
    /// - `f32` - Corner radius in logical pixels.
    fn draw_euv_brand_emblem(bounds: Rect, dl: &mut DisplayList, radius: f32) {
        let x: f32 = bounds.get_x();
        let y: f32 = bounds.get_y();
        let w: f32 = bounds.get_width().max(1.0);
        let h: f32 = bounds.get_height().max(1.0);
        let s: f32 = w.min(h);
        let ox: f32 = x + (w - s) * 0.5;
        let oy: f32 = y + (h - s) * 0.5;
        let u: f32 = s / SHELL_BRAND_BADGE_SIZE;
        let sq: Rect = Rect::new(ox, oy, s, s);
        let rad: CornerRadius = CornerRadius::all(radius.min(s * 0.32));
        dl.push(DrawCommand::LinearGradientRect {
            rect: sq,
            gradient: LinearGradient::new(135.0, SHELL_LOGO_BG_TOP, SHELL_LOGO_BG_BOTTOM),
            radius: rad,
        });
        dl.push(DrawCommand::FillRoundedRect {
            rect: sq,
            color: COLOR_TRANSPARENT,
            radius: rad,
            border: Border::new(1.0, SHELL_LOGO_RIM),
        });
        let bar_r: CornerRadius = CornerRadius::all(1.15 * u);
        let spine_x: f32 = ox + 6.5 * u;
        let bar_h: f32 = 2.6 * u;
        let stub_w: f32 = 2.6 * u;
        let stub_h: f32 = 4.9 * u;
        dl.push(DrawCommand::FillRoundedRect {
            rect: Rect::new(spine_x, oy + 5.7 * u, 13.0 * u, bar_h),
            color: SHELL_LOGO_MONOGRAM,
            radius: bar_r,
            border: Border::default(),
        });
        dl.push(DrawCommand::FillRoundedRect {
            rect: Rect::new(spine_x, oy + 5.7 * u, stub_w, stub_h),
            color: SHELL_LOGO_MONOGRAM,
            radius: bar_r,
            border: Border::default(),
        });
        dl.push(DrawCommand::FillRoundedRect {
            rect: Rect::new(spine_x, oy + 11.7 * u, 8.2 * u, bar_h),
            color: SHELL_LOGO_MONOGRAM,
            radius: bar_r,
            border: Border::default(),
        });
        dl.push(DrawCommand::FillRoundedRect {
            rect: Rect::new(ox + 15.6 * u, oy + 11.7 * u, 3.9 * u, bar_h),
            color: SHELL_LOGO_SPARK,
            radius: bar_r,
            border: Border::default(),
        });
        dl.push(DrawCommand::FillRoundedRect {
            rect: Rect::new(ox + 15.6 * u, oy + 11.7 * u, 1.8 * u, bar_h),
            color: SHELL_LOGO_SPARK_HI,
            radius: bar_r,
            border: Border::default(),
        });
        dl.push(DrawCommand::FillRoundedRect {
            rect: Rect::new(spine_x, oy + 17.7 * u, 13.0 * u, bar_h),
            color: SHELL_LOGO_MONOGRAM,
            radius: bar_r,
            border: Border::default(),
        });
        dl.push(DrawCommand::FillRoundedRect {
            rect: Rect::new(spine_x, oy + 15.4 * u, stub_w, stub_h),
            color: SHELL_LOGO_MONOGRAM,
            radius: bar_r,
            border: Border::default(),
        });
    }

    /// Mounts the unified rounded-square brand logo mark shared identically between the top navbar, desktop sidebar, and mobile drawer.
    ///
    /// # Arguments
    ///
    /// - `&str` - Single-character brand initial text.
    /// - `ThemeTokens` - Active design tokens.
    /// - `Option<Signal<bool>>` - Optional drawer toggle signal when mounted interactively in the top navbar.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted brand logo badge node.
    fn mount_brand_logo(
        brand_initial: &str,
        tokens: ThemeTokens,
        toggle_menu: Option<Signal<bool>>,
    ) -> NodeId {
        let p: ThemePalette = tokens.get_palette();
        let g: ThemeGeometry = tokens.get_geometry();
        let badge_r: f32 = g.get_radius_sm().clamp(6.0, 8.0);
        let is_euv_mark: bool = brand_initial.eq_ignore_ascii_case(SHELL_DEFAULT_INITIAL);
        let initial_owned: String = brand_initial.to_string();
        if let Some(menu_sig) = toggle_menu {
            let on_toggle = move |_: &UiEvent| {
                let _: bool = menu_sig.update(|v: &mut bool| *v = !*v);
            };
            if is_euv_mark {
                let paint_emblem = move |bounds: Rect, dl: &mut DisplayList| {
                    Self::draw_euv_brand_emblem(bounds, dl, badge_r);
                };
                view! {
                    row {
                        width: Length::Px(SHELL_BRAND_BADGE_SIZE)
                        height: Length::Px(SHELL_BRAND_BADGE_SIZE)
                        flex_shrink: 0.0
                        radius: badge_r
                        justify: JustifyContent::Center
                        align: AlignItems::Center
                        overflow: Overflow::Hidden
                        cursor: CursorIcon::Pointer
                        role: A11yRole::Button
                        a11y_label: SHELL_MENU_LABEL.to_string()
                        on_click: on_toggle
                        canvas {
                            width: Length::Px(SHELL_BRAND_BADGE_SIZE)
                            height: Length::Px(SHELL_BRAND_BADGE_SIZE)
                            painter: paint_emblem
                        }
                    }
                }
            } else {
                view! {
                    row {
                        width: Length::Px(SHELL_BRAND_BADGE_SIZE)
                        height: Length::Px(SHELL_BRAND_BADGE_SIZE)
                        flex_shrink: 0.0
                        radius: badge_r
                        background: p.get_primary()
                        justify: JustifyContent::Center
                        align: AlignItems::Center
                        cursor: CursorIcon::Pointer
                        role: A11yRole::Button
                        a11y_label: SHELL_MENU_LABEL.to_string()
                        on_click: on_toggle
                        text {
                            font_size: 13.0
                            font_weight: FontWeight::Bold
                            color: p.get_primary_foreground()
                            text_align: TextAlign::Center
                            text_overflow: TextOverflow::Clip
                            { initial_owned }
                        }
                    }
                }
            }
        } else if is_euv_mark {
            let paint_emblem = move |bounds: Rect, dl: &mut DisplayList| {
                Self::draw_euv_brand_emblem(bounds, dl, badge_r);
            };
            view! {
                row {
                    width: Length::Px(SHELL_BRAND_BADGE_SIZE)
                    height: Length::Px(SHELL_BRAND_BADGE_SIZE)
                    flex_shrink: 0.0
                    radius: badge_r
                    justify: JustifyContent::Center
                    align: AlignItems::Center
                    overflow: Overflow::Hidden
                    role: A11yRole::Group
                    a11y_label: SHELL_BRAND_A11Y_LABEL.to_string()
                    canvas {
                        width: Length::Px(SHELL_BRAND_BADGE_SIZE)
                        height: Length::Px(SHELL_BRAND_BADGE_SIZE)
                        painter: paint_emblem
                    }
                }
            }
        } else {
            view! {
                row {
                    width: Length::Px(SHELL_BRAND_BADGE_SIZE)
                    height: Length::Px(SHELL_BRAND_BADGE_SIZE)
                    flex_shrink: 0.0
                    radius: badge_r
                    background: p.get_primary()
                    justify: JustifyContent::Center
                    align: AlignItems::Center
                    role: A11yRole::Group
                    a11y_label: SHELL_BRAND_A11Y_LABEL.to_string()
                    text {
                        font_size: 13.0
                        font_weight: FontWeight::Bold
                        color: p.get_primary_foreground()
                        text_align: TextAlign::Center
                        text_overflow: TextOverflow::Clip
                        { initial_owned }
                    }
                }
            }
        }
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
        bind_fullscreen_context(&ctx);
        let tokens: ThemeTokens = self.get_tokens();
        let p: ThemePalette = tokens.get_palette();
        let g: ThemeGeometry = tokens.get_geometry();
        let bp_sig: Signal<Breakpoint> = ctx.get_breakpoint();
        let vp_sig: Signal<Size> = ctx.get_viewport_size();
        let safe_sig: Signal<SafeAreaInsets> = ctx.get_safe_area();
        let gesture_sig: Signal<Option<GestureEvent>> = ctx.get_gesture();
        let swipe_drag_sig: Signal<f32> = ctx.get_swipe_drag_offset();
        let fps_sig: Signal<f32> = ctx.get_live_fps();
        let refresh_sig: Signal<f32> = ctx.get_display_refresh_rate();
        let app_state_sig: Signal<AppState> = ctx.get_app_state();
        let fs_active_sig: Signal<bool> = use_fullscreen_active();
        let ctx_fs_sig: Signal<bool> = ctx.immersive_fullscreen_signal();
        let overlay_count_sig: Signal<usize> = EuvOverlayStack::active_count_signal();
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
            theme,
            page_padding,
            ..
        } = self;
        let sidebar: NodeId = *sidebar;
        let main_content: NodeId = *main_content;
        let route_signal: Option<Signal<String>> = *route_signal;
        let top_bar_actions: Option<NodeId> = *top_bar_actions;
        let mobile_header_actions: Option<NodeId> = *mobile_header_actions;
        let sidebar_footer: Option<NodeId> = *sidebar_footer;
        let breadcrumb: Option<Signal<String>> = *breadcrumb;
        let bottom_nav: Option<NodeId> = *bottom_nav;
        let page_padding: Option<Signal<Edges>> = *page_padding;
        let swipe_routes_rc: Rc<[String]> = Rc::from(swipe_routes.as_slice());
        let theme_state_opt: Option<UiThemeState> = theme.clone();
        if let Some(theme_ref) = theme_state_opt.as_ref() {
            provide_theme(theme_ref.clone());
        }
        let has_managed_outlet: bool = ctx.get_scene().with_read(
            |tree: &SceneTree| {
                tree.get_node(main_content)
                    .map(|n: &SceneNode| {
                        let lay: LayoutStyle = n.style_ref().get_layout();
                        matches!(n.get_kind(), NodeKind::View)
                            && lay.get_overflow() == Overflow::Hidden
                            && lay.get_height() == Length::Percent(100.0)
                    })
                    .unwrap_or(false)
            },
            false,
        );
        let has_bottom_nav: bool = bottom_nav.is_some();
        let dock_radius: f32 = g.get_radius_lg().clamp(6.0, 14.0);
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
        let mut bottom_bar_host: NodeId = NodeId::default();
        let mut drawer_portal: NodeId = NodeId::default();
        let mut drawer_panel: NodeId = NodeId::default();
        let mut drawer_scroll: NodeId = NodeId::default();
        let mut drawer_grabber: NodeId = NodeId::default();
        let init_fps: f32 = fps_sig.get_untracked();
        let init_hz: f32 = refresh_sig.get_untracked().clamp(30.0, 480.0);
        let init_good_fps: f32 = (init_hz * 0.75).clamp(45.0, 90.0);
        let init_warn_fps: f32 = (init_hz * 0.45).clamp(24.0, 50.0);
        let init_dot_col: Color = if !init_fps.is_finite() || init_fps <= SHELL_FPS_ACTIVE_THRESHOLD
        {
            p.get_muted_foreground()
        } else if init_fps >= init_good_fps {
            p.get_success()
        } else if init_fps >= init_warn_fps {
            p.get_warning()
        } else {
            p.get_destructive()
        };
        let mut mobile_fps_dot: Option<NodeId> = None;
        let mobile_fps_pill: Option<NodeId> = if show_fps {
            let mut dot_node: NodeId = NodeId::default();
            let fps_text_mob = move || -> String {
                let raw: f32 = fps_sig.get();
                if !raw.is_finite() || raw <= SHELL_FPS_ACTIVE_THRESHOLD {
                    SHELL_FPS_IDLE_LABEL.to_string()
                } else {
                    let v: f32 = raw.clamp(1.0, 999.0);
                    format!("{v:.0}")
                }
            };
            let pill: NodeId = view! {
                row {
                    min_width: Length::Px(46.0)
                    height: Length::Px(24.0)
                    padding: Edges::symmetric(0.0, 6.0)
                    radius: g.get_radius_sm()
                    justify: JustifyContent::Center
                    align: AlignItems::Center
                    gap: 4.0
                    background: p.get_muted()
                    border: (1.0, p.get_border())
                    role: A11yRole::Group
                    a11y_label: SHELL_FPS_A11Y_LABEL.to_string()
                    view {
                        node_ref: dot_node
                        width: Length::Px(6.0)
                        height: Length::Px(6.0)
                        radius: 99.0
                        background: init_dot_col
                    }
                    text {
                        height: Length::Px(14.0)
                        font_size: 11.0
                        font_family: FontFamily::Monospace
                        font_weight: FontWeight::Bold
                        color: p.get_foreground()
                        text_align: TextAlign::Center
                        text_overflow: TextOverflow::Clip
                        { fps_text_mob }
                    }
                }
            };
            mobile_fps_dot = Some(dot_node);
            Some(pill)
        } else {
            None
        };
        let mut desktop_fps_dot: Option<NodeId> = None;
        let desktop_fps_pill: Option<NodeId> = if show_fps && !has_bottom_nav {
            let mut dot_node: NodeId = NodeId::default();
            let fps_text_desk = move || -> String {
                let raw: f32 = fps_sig.get();
                let hz: f32 = refresh_sig.get().clamp(30.0, 480.0);
                if !raw.is_finite() || raw <= SHELL_FPS_ACTIVE_THRESHOLD {
                    format!("{SHELL_FPS_IDLE_LABEL} · {hz:.0}Hz")
                } else {
                    let v: f32 = raw.clamp(1.0, 999.0);
                    format!("{v:.0} · {hz:.0}Hz")
                }
            };
            let pill: NodeId = view! {
                row {
                    min_width: Length::Px(82.0)
                    height: Length::Px(24.0)
                    padding: Edges::symmetric(0.0, 7.0)
                    radius: g.get_radius_sm()
                    justify: JustifyContent::Center
                    align: AlignItems::Center
                    gap: 4.0
                    background: p.get_muted()
                    border: (1.0, p.get_border())
                    role: A11yRole::Group
                    a11y_label: SHELL_FPS_A11Y_LABEL.to_string()
                    view {
                        node_ref: dot_node
                        width: Length::Px(6.0)
                        height: Length::Px(6.0)
                        radius: 99.0
                        background: init_dot_col
                    }
                    text {
                        height: Length::Px(14.0)
                        font_size: 11.0
                        font_family: FontFamily::Monospace
                        font_weight: FontWeight::Bold
                        color: p.get_foreground()
                        text_align: TextAlign::Center
                        text_overflow: TextOverflow::Clip
                        { fps_text_desk }
                    }
                }
            };
            desktop_fps_dot = Some(dot_node);
            Some(pill)
        } else {
            None
        };
        let paint_menu_icon = move |bounds: Rect, dl: &mut DisplayList| {
            let live_p: ThemePalette = live_tokens_untracked(tokens).get_palette();
            draw_euv_icon(EuvIconName::Menu, bounds, dl, live_p.get_foreground());
        };
        let on_toggle_menu_btn = move |_: &UiEvent| {
            let _: bool = menu_open.update(|v: &mut bool| *v = !*v);
        };
        let top_brand_badge: Option<NodeId> = if has_bottom_nav {
            Some(Self::mount_brand_logo(
                &brand_initial,
                tokens,
                Some(menu_open),
            ))
        } else {
            None
        };
        let nav_brand_badge: NodeId = Self::mount_brand_logo(&brand_initial, tokens, None);
        let drawer_brand_badge: NodeId = Self::mount_brand_logo(&brand_initial, tokens, None);
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
                        if let Some(badge_node) = top_brand_badge {
                            { badge_node }
                        }
                        text {
                            node_ref: top_title_txt
                            width: Length::Percent(100.0)
                            flex_grow: 1.0
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
                            row {
                                width: Length::Px(30.0)
                                height: Length::Px(30.0)
                                justify: JustifyContent::Center
                                align: AlignItems::Center
                                radius: g.get_radius_md()
                                background: p.get_background()
                                border: (g.get_border_width(), p.get_border())
                                cursor: CursorIcon::Pointer
                                role: A11yRole::Button
                                a11y_label: SHELL_MENU_LABEL.to_string()
                                on_click: on_toggle_menu_btn
                                canvas {
                                    width: Length::Px(15.0)
                                    height: Length::Px(15.0)
                                    painter: paint_menu_icon
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
                            { nav_brand_badge }
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
                            overflow: if has_managed_outlet { Overflow::Hidden } else { Overflow::Scroll }
                            gap: if has_managed_outlet { 0.0 } else { SPACE_LG }
                            { main_content }
                        }
                    }
                }
                row {
                    node_ref: bottom_bar_host
                    display: if has_bottom_nav { Display::Flex } else { Display::None }
                    position: Position::Fixed
                    inset: (None, None, Some(SHELL_BOTTOM_NAV_FLOAT_COMPACT), Some(SHELL_BOTTOM_NAV_SIDE_MARGIN))
                    width: Length::Px(340.0)
                    height: Length::Px(SHELL_BOTTOM_NAV_HEIGHT)
                    padding: Edges::symmetric(0.0, 6.0)
                    justify: JustifyContent::Center
                    align: AlignItems::Center
                    background: p.get_card().with_alpha(244)
                    border: (g.get_border_width(), p.get_border())
                    radius: dock_radius
                    shadow: g.get_shadow_lg()
                    overflow: Overflow::Hidden
                    stop_scroll_propagation: true
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
                                { drawer_brand_badge }
                                text {
                                    font_size: 14.5
                                    font_weight: FontWeight::Bold
                                    color: p.get_foreground()
                                    { brand.clone() }
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
        if !has_managed_outlet {
            let main_clamp_scene: SceneHandle = ctx.get_scene().clone();
            ctx.on_scroll(main_area, move |_: &UiEvent| {
                main_clamp_scene.with_write(
                    |tree: &mut SceneTree| {
                        if let Some(cur) = tree
                            .get_node(main_area)
                            .map(|n: &SceneNode| n.get_layout_box().get_scroll_offset())
                            && cur.get_x().abs() > 0.01
                        {
                            tree.set_scroll_offset(main_area, Point::new(0.0, cur.get_y()));
                        }
                    },
                    (),
                );
            });
        }
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
                                let st: &mut Style = n.get_mut_style();
                                st.get_mut_layout().set_width(Length::Px(next_w));
                                st.get_mut_paint()
                                    .set_transform(Transform2D::new(tx, 0.0, 1.0, 1.0, 0.0));
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
                            let st: &mut Style = n.get_mut_style();
                            st.get_mut_layout().set_width(Length::Px(target_w));
                            st.get_mut_paint().set_transform(Transform2D::default());
                            let layout_dirty: u16 = DIRTY_LAYOUT | DIRTY_SUBTREE_LAYOUT;
                            tree.mark_dirty(drawer_panel, layout_dirty | DIRTY_PAINT);
                        }
                    },
                    (),
                );
            });
            let h_cancel: SceneHandle = ctx.get_scene().clone();
            let da_cancel: Rc<Cell<bool>> = drawer_drag_active.clone();
            let dm_cancel: Rc<Cell<bool>> = drawer_drag_moved.clone();
            ctx.on_pointer_cancel(target_node, move |_: &UiEvent| {
                if !da_cancel.replace(false) {
                    return;
                }
                dm_cancel.set(false);
                h_cancel.with_write(
                    |tree: &mut SceneTree| {
                        if let Some(n) = tree.get_node_mut(drawer_panel) {
                            n.get_mut_style()
                                .get_mut_paint()
                                .set_transform(Transform2D::default());
                            tree.mark_dirty(drawer_panel, DIRTY_PAINT);
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
        let handle_state: SceneHandle = handle.clone();
        let drawer_da_close: Rc<Cell<bool>> = drawer_drag_active.clone();
        let drawer_dm_close: Rc<Cell<bool>> = drawer_drag_moved.clone();
        let drawer_da_state: Rc<Cell<bool>> = drawer_drag_active;
        let drawer_dm_state: Rc<Cell<bool>> = drawer_drag_moved;
        let prev_open: Rc<Cell<bool>> = Rc::new(Cell::new(false));
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            let _: Effect =
                EuvOverlayStack::bind_signal(menu_open, SHELL_DRAWER_OVERLAY_NAME, Z_INDEX_DRAWER);
            let _: Effect = Effect::new(move || {
                if app_state_sig.get() != AppState::Active && drawer_da_state.replace(false) {
                    drawer_dm_state.set(false);
                    handle_state.with_write(
                        |tree: &mut SceneTree| {
                            if let Some(n) = tree.get_node_mut(drawer_panel) {
                                n.get_mut_style()
                                    .get_mut_paint()
                                    .set_transform(Transform2D::default());
                                tree.mark_dirty(drawer_panel, DIRTY_PAINT);
                            }
                        },
                        (),
                    );
                }
            });
            if mobile_fps_dot.is_some() || desktop_fps_dot.is_some() {
                let handle_fps: SceneHandle = handle.clone();
                let prev_dot_col: Cell<Color> = Cell::new(init_dot_col);
                let _: Effect = Effect::new(move || {
                    let live_p: ThemePalette = use_live_tokens(tokens).get_palette();
                    let raw: f32 = fps_sig.get();
                    let target_hz: f32 = refresh_sig.get().clamp(30.0, 480.0);
                    let good_fps: f32 = (target_hz * 0.75).clamp(45.0, 90.0);
                    let warn_fps: f32 = (target_hz * 0.45).clamp(24.0, 50.0);
                    let dot_col: Color = if !raw.is_finite() || raw <= SHELL_FPS_ACTIVE_THRESHOLD {
                        live_p.get_muted_foreground()
                    } else if raw >= good_fps {
                        live_p.get_success()
                    } else if raw >= warn_fps {
                        live_p.get_warning()
                    } else {
                        live_p.get_destructive()
                    };
                    if prev_dot_col.replace(dot_col) == dot_col {
                        return;
                    }
                    handle_fps.with_write(
                        |tree: &mut SceneTree| {
                            if let Some(dot_id) = mobile_fps_dot
                                && let Some(n) = tree.get_node_mut(dot_id)
                            {
                                n.get_mut_style()
                                    .get_mut_paint()
                                    .set_background(Some(dot_col));
                                tree.mark_dirty(dot_id, DIRTY_PAINT);
                            }
                            if let Some(dot_id) = desktop_fps_dot
                                && let Some(n) = tree.get_node_mut(dot_id)
                            {
                                n.get_mut_style()
                                    .get_mut_paint()
                                    .set_background(Some(dot_col));
                                tree.mark_dirty(dot_id, DIRTY_PAINT);
                            }
                        },
                        (),
                    );
                });
            }
            if let Some(crumb_sig) = breadcrumb {
                let target_title_node: NodeId = if has_bottom_nav {
                    top_title_txt
                } else {
                    crumb_txt
                };
                let _: Effect = handle_crumb.bind_text(target_title_node, move || {
                    let full: String = crumb_sig.get();
                    if bp_sig.get().is_compact()
                        && let Some((_, leaf)) = full.rsplit_once(" / ")
                    {
                        leaf.to_string()
                    } else {
                        full
                    }
                });
            }
            let mut scratch_children: Vec<NodeId> = Vec::new();
            let _: Effect = Effect::new(move || {
                let bp: Breakpoint = bp_sig.get();
                let vp: Size = vp_sig.get();
                let safe: SafeAreaInsets = safe_sig.get();
                let is_fullscreen: bool = fs_active_sig.get() || ctx_fs_sig.get();
                let compact: bool = has_bottom_nav || bp.is_compact();
                let is_medium: bool = !has_bottom_nav && bp == Breakpoint::Medium;
                let top_inset: f32 = safe.get_top();
                let bottom_inset: f32 = safe.get_bottom();
                let left_inset: f32 = safe.get_left();
                let right_inset: f32 = safe.get_right();
                let vp_w: f32 = vp.get_width().max(240.0);
                let vp_h: f32 = vp.get_height().max(240.0);
                let header_h: f32 = MOBILE_HEADER_HEIGHT + top_inset;
                let float_gap: f32 = if bp.is_compact() {
                    SHELL_BOTTOM_NAV_FLOAT_COMPACT
                } else {
                    SHELL_BOTTOM_NAV_FLOAT_WIDE
                };
                let dock_bottom: f32 = float_gap + bottom_inset;
                let usable_w: f32 = (vp_w - left_inset - right_inset).max(200.0);
                let side_margin: f32 = if bp.is_compact() {
                    SHELL_BOTTOM_NAV_SIDE_MARGIN
                } else {
                    32.0
                };
                let min_dock_w: f32 = 240.0_f32.min((usable_w - 16.0).max(160.0));
                let dock_w: f32 =
                    (usable_w - side_margin * 2.0).clamp(min_dock_w, SHELL_BOTTOM_NAV_MAX_WIDTH);
                let dock_left: f32 = left_inset + ((usable_w - dock_w) * 0.5).max(8.0);
                let desktop_bar_h: f32 = 52.0 + top_inset;
                let body_h: f32 = if is_fullscreen {
                    vp_h
                } else if compact {
                    (vp_h - header_h - div_h).max(120.0)
                } else {
                    vp_h
                };
                let main_h: f32 = if is_fullscreen {
                    vp_h
                } else if compact {
                    body_h
                } else {
                    (vp_h - desktop_bar_h - div_h).max(120.0)
                };
                if !compact || is_fullscreen {
                    let _: bool = menu_open.set_if_changed(false);
                }
                let drawer_vp_w: f32 = vp.get_width().max(320.0);
                let drawer_def_w: f32 = (drawer_vp_w * 0.68).clamp(224.0, 256.0);
                let drawer_max_w: f32 = (drawer_vp_w * 0.88).clamp(drawer_def_w + 40.0, 380.0);
                let drawer_target_w: f32 = if drawer_expanded.get() {
                    drawer_max_w
                } else {
                    drawer_def_w
                };
                let compact_disp: Display = if is_fullscreen || !compact {
                    Display::None
                } else {
                    Display::Flex
                };
                let desktop_disp: Display = if is_fullscreen || compact {
                    Display::None
                } else {
                    Display::Flex
                };
                let bottom_disp: Display = if has_bottom_nav && !is_fullscreen {
                    Display::Flex
                } else {
                    Display::None
                };
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
                        scratch_children.clear();
                        if let Some(node) = tree.get_node(sidebar) {
                            scratch_children.extend_from_slice(node.get_children().as_slice());
                        }
                        for child_id in scratch_children.iter().copied() {
                            tree.mark_dirty(child_id, mask_all);
                        }
                        if let Some(n) = tree.get_node_mut(top_bar) {
                            n.get_mut_style()
                                .get_mut_layout()
                                .set_display(compact_disp)
                                .set_height(Length::Px(header_h))
                                .set_padding(Edges::new(
                                    top_inset,
                                    SPACE_MD + right_inset,
                                    0.0,
                                    SPACE_MD + left_inset,
                                ));
                            tree.mark_dirty(top_bar, mask_all);
                        }
                        if let Some(n) = tree.get_node_mut(top_bar_divider) {
                            n.get_mut_style().get_mut_layout().set_display(compact_disp);
                            tree.mark_dirty(top_bar_divider, mask_all);
                        }
                        if let Some(n) = tree.get_node_mut(desktop_top_bar) {
                            let bar_pad: f32 = if is_medium { SPACE_MD } else { SPACE_XL };
                            n.get_mut_style()
                                .get_mut_layout()
                                .set_display(desktop_disp)
                                .set_height(Length::Px(desktop_bar_h))
                                .set_padding(Edges::new(
                                    top_inset,
                                    bar_pad + right_inset,
                                    0.0,
                                    bar_pad,
                                ));
                            tree.mark_dirty(desktop_top_bar, mask_all);
                        }
                        if let Some(n) = tree.get_node_mut(desktop_top_divider) {
                            n.get_mut_style().get_mut_layout().set_display(desktop_disp);
                            tree.mark_dirty(desktop_top_divider, mask_all);
                        }
                        if let Some(n) = tree.get_node_mut(body_row) {
                            n.get_mut_style()
                                .get_mut_layout()
                                .set_height(Length::Px(body_h));
                            tree.mark_dirty(body_row, mask_all);
                        }
                        if let Some(n) = tree.get_node_mut(content_column) {
                            n.get_mut_style()
                                .get_mut_layout()
                                .set_height(Length::Px(body_h));
                            tree.mark_dirty(content_column, mask_all);
                        }
                        let nav_w: f32 = if is_medium { 204.0 } else { NAV_SIDEBAR_WIDTH };
                        let nav_pad: f32 = if is_medium { SPACE_MD } else { SPACE_LG };
                        if let Some(n) = tree.get_node_mut(nav_divider) {
                            n.get_mut_style().get_mut_layout().set_display(desktop_disp);
                            tree.mark_dirty(nav_divider, mask_all);
                        }
                        if let Some(n) = tree.get_node_mut(nav_rail) {
                            n.get_mut_style()
                                .get_mut_layout()
                                .set_display(desktop_disp)
                                .set_width(Length::Px(nav_w))
                                .set_padding(Edges::new(
                                    nav_pad + top_inset,
                                    nav_pad,
                                    nav_pad + bottom_inset,
                                    nav_pad + left_inset,
                                ));
                            tree.mark_dirty(nav_rail, mask_all);
                        }
                        let main_pad: f32 = if bp.is_compact() {
                            SPACE_MD
                        } else if bp == Breakpoint::Medium {
                            SPACE_LG
                        } else {
                            SPACE_XL
                        };
                        let main_bottom_pad: f32 = if has_bottom_nav && !is_fullscreen {
                            main_pad + SHELL_BOTTOM_NAV_HEIGHT + dock_bottom + 12.0
                        } else {
                            main_pad + bottom_inset
                        };
                        let page_edges: Edges = Edges::new(
                            main_pad,
                            main_pad + right_inset,
                            main_bottom_pad,
                            main_pad + left_inset,
                        );
                        if let Some(pad_sig) = page_padding {
                            let _: bool = pad_sig.set_if_changed(page_edges);
                        }
                        if let Some(n) = tree.get_node_mut(main_area) {
                            let lay: &mut LayoutStyle = n.get_mut_style().get_mut_layout();
                            lay.set_height(Length::Px(main_h));
                            if has_managed_outlet {
                                lay.set_padding(Edges::all(0.0))
                                    .set_overflow(Overflow::Hidden)
                                    .set_row_gap(0.0)
                                    .set_column_gap(0.0);
                            } else {
                                let main_gap: f32 =
                                    if bp.is_compact() { SPACE_MD } else { SPACE_LG };
                                lay.set_padding(page_edges)
                                    .set_row_gap(main_gap)
                                    .set_column_gap(main_gap);
                            }
                            tree.mark_dirty(main_area, mask_all);
                        }
                        if has_managed_outlet && page_padding.is_none() {
                            scratch_children.clear();
                            if let Some(n) = tree.get_node(main_content) {
                                scratch_children.extend_from_slice(n.get_children().as_slice());
                            }
                            for kid in scratch_children.iter().copied() {
                                if let Some(kn) = tree.get_node_mut(kid)
                                    && matches!(kn.get_kind(), NodeKind::ScrollView)
                                {
                                    kn.get_mut_style().get_mut_layout().set_padding(page_edges);
                                    tree.mark_dirty(kid, mask_all);
                                }
                            }
                        }
                        if let Some(n) = tree.get_node_mut(bottom_bar_host) {
                            let lay: &mut LayoutStyle = n.get_mut_style().get_mut_layout();
                            lay.set_display(bottom_disp);
                            if has_bottom_nav {
                                lay.set_width(Length::Px(dock_w))
                                    .set_height(Length::Px(SHELL_BOTTOM_NAV_HEIGHT))
                                    .set_left(Some(dock_left))
                                    .set_bottom(Some(dock_bottom))
                                    .set_right(None)
                                    .set_top(None)
                                    .set_padding(Edges::symmetric(0.0, 7.0));
                            }
                            tree.mark_dirty(bottom_bar_host, mask_all);
                        }
                        if let Some(n) = tree.get_node_mut(drawer_scroll) {
                            n.get_mut_style().get_mut_layout().set_padding(Edges::new(
                                SPACE_LG + top_inset,
                                SPACE_SM,
                                SPACE_LG + bottom_inset,
                                SPACE_LG + left_inset,
                            ));
                            tree.mark_dirty(drawer_scroll, mask_all);
                        }
                        if let Some(n) = tree.get_node_mut(drawer_panel) {
                            n.get_mut_style()
                                .get_mut_layout()
                                .set_width(Length::Px(drawer_target_w));
                            tree.mark_dirty(drawer_panel, mask_all);
                        }
                        tree.mark_dirty(root, mask_all);
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
            let drag_was_active: Rc<Cell<bool>> = Rc::new(Cell::new(false));
            let swipe_route_committed: Rc<Cell<bool>> = Rc::new(Cell::new(false));
            let swipe_route_committed_drag: Rc<Cell<bool>> = Rc::clone(&swipe_route_committed);
            let swipe_routes_drag: Rc<[String]> = Rc::clone(&swipe_routes_rc);
            let _: Effect = Effect::new(move || {
                let raw_dx: f32 = swipe_drag_sig.get();
                let app_state: AppState = app_state_sig.get();
                let is_fullscreen: bool = fs_active_sig.get() || ctx_fs_sig.get();
                let overlay_count: usize = overlay_count_sig.get();
                if has_managed_outlet
                    || menu_open.get_untracked()
                    || overlay_count > 0
                    || is_fullscreen
                    || app_state != AppState::Active
                    || swipe_routes_drag.is_empty()
                {
                    if drag_was_active.replace(false) {
                        handle_swipe.with_write(
                            |tree: &mut SceneTree| {
                                tree.cancel_subtree_transitions(main_area);
                                if let Some(n) = tree.get_node_mut(main_area) {
                                    n.get_mut_style()
                                        .get_mut_paint()
                                        .set_transform(Transform2D::default());
                                    tree.mark_dirty(main_area, DIRTY_PAINT);
                                }
                            },
                            (),
                        );
                    }
                    return;
                }
                if raw_dx.abs() > 1.0 {
                    drag_was_active.set(true);
                    let damped_tx: f32 = (raw_dx * 0.36).clamp(-120.0, 120.0);
                    handle_swipe.with_write(
                        |tree: &mut SceneTree| {
                            tree.cancel_subtree_transitions(main_area);
                            if let Some(n) = tree.get_node_mut(main_area) {
                                n.get_mut_style()
                                    .get_mut_paint()
                                    .set_transform(Transform2D::new(damped_tx, 0.0, 1.0, 1.0, 0.0));
                                tree.mark_dirty(main_area, DIRTY_PAINT);
                            }
                        },
                        (),
                    );
                } else if drag_was_active.replace(false) {
                    if swipe_route_committed_drag.replace(false) {
                        handle_swipe.with_write(
                            |tree: &mut SceneTree| {
                                tree.cancel_subtree_transitions(main_area);
                                if let Some(n) = tree.get_node_mut(main_area) {
                                    n.get_mut_style()
                                        .get_mut_paint()
                                        .set_transform(Transform2D::default());
                                    tree.mark_dirty(main_area, DIRTY_PAINT);
                                }
                            },
                            (),
                        );
                    } else {
                        handle_swipe.with_write(
                            |tree: &mut SceneTree| {
                                let cur_tx: f32 = tree
                                    .get_node(main_area)
                                    .map(|n: &SceneNode| {
                                        n.style_ref().get_paint().get_transform().get_translate_x()
                                    })
                                    .unwrap_or(0.0);
                                if cur_tx.abs() > 0.5 {
                                    tree.start_transition(
                                        NodeTransition::slide_fade(
                                            main_area,
                                            Point::new(cur_tx, 0.0),
                                            Point::new(0.0, 0.0),
                                            1.0,
                                            1.0,
                                            0.22,
                                            false,
                                        )
                                        .with_easing(Easing::EaseOutQuint),
                                    );
                                }
                            },
                            (),
                        );
                    }
                }
            });
            let handle_swipe_reset: SceneHandle = handle.clone();
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
                        if EuvOverlayStack::active_count() > 0 || is_fullscreen_active() {
                            return;
                        }
                        let drawer_intent: bool =
                            ev.get_origin().get_x() <= 72.0 || ev.get_delta().get_x() >= 96.0;
                        if !swipe_routes_rc.is_empty()
                            && let Some(route_sig) = route_signal
                        {
                            let cur_path: String = route_sig.get_untracked();
                            let cur_idx: usize = swipe_routes_rc
                                .iter()
                                .position(|r: &String| r.as_str() == cur_path.as_str())
                                .unwrap_or(0);
                            if ev.get_kind() == GestureEventKind::SwipeLeft
                                && cur_idx + 1 < swipe_routes_rc.len()
                            {
                                let next_path: String = swipe_routes_rc[cur_idx + 1].clone();
                                if route_sig.set_if_changed(next_path) {
                                    swipe_route_committed.set(true);
                                    handle_swipe_reset.with_write(
                                        |tree: &mut SceneTree| {
                                            tree.cancel_subtree_transitions(main_area);
                                            if let Some(n) = tree.get_node_mut(main_area) {
                                                n.get_mut_style()
                                                    .get_mut_paint()
                                                    .set_transform(Transform2D::default());
                                                tree.mark_dirty(main_area, DIRTY_PAINT);
                                            }
                                        },
                                        (),
                                    );
                                }
                            } else if ev.get_kind() == GestureEventKind::SwipeRight {
                                if cur_idx > 0 {
                                    let prev_path: String = swipe_routes_rc[cur_idx - 1].clone();
                                    if route_sig.set_if_changed(prev_path) {
                                        swipe_route_committed.set(true);
                                        handle_swipe_reset.with_write(
                                            |tree: &mut SceneTree| {
                                                tree.cancel_subtree_transitions(main_area);
                                                if let Some(n) = tree.get_node_mut(main_area) {
                                                    n.get_mut_style()
                                                        .get_mut_paint()
                                                        .set_transform(Transform2D::default());
                                                    tree.mark_dirty(main_area, DIRTY_PAINT);
                                                }
                                            },
                                            (),
                                        );
                                    }
                                } else if drawer_intent {
                                    let _: bool = menu_open.set_if_changed(true);
                                }
                            }
                        } else if bp_sig.get_untracked().is_compact()
                            && ev.get_kind() == GestureEventKind::SwipeRight
                            && drawer_intent
                        {
                            let _: bool = menu_open.set_if_changed(true);
                        }
                    }
                }
            });
            if let Some(theme_state) = theme_state_opt {
                let applied_shell_tokens: Rc<Cell<ThemeTokens>> = Rc::new(Cell::new(tokens));
                let handle_theme: SceneHandle = handle.clone();
                let mut excluded_nodes: Vec<NodeId> = Vec::with_capacity(2);
                if has_managed_outlet {
                    excluded_nodes.push(main_content);
                }
                if has_bottom_nav {
                    excluded_nodes.push(bottom_bar_host);
                }
                let _: Effect = Effect::new(move || {
                    let next_tok: ThemeTokens = theme_state.get_tokens().get();
                    let prev_tok: ThemeTokens = applied_shell_tokens.replace(next_tok);
                    if prev_tok == next_tok {
                        return;
                    }
                    let next_p: ThemePalette = next_tok.get_palette();
                    let next_g: ThemeGeometry = next_tok.get_geometry();
                    let next_dock_r: f32 = next_g.get_radius_lg().clamp(6.0, 14.0);
                    handle_theme.with_write(
                        |tree: &mut SceneTree| {
                            remap_subtree_theme_excluding_nodes(
                                tree,
                                root,
                                &excluded_nodes,
                                prev_tok,
                                next_tok,
                            );
                            if has_bottom_nav && let Some(bn) = tree.get_node_mut(bottom_bar_host) {
                                bn.get_mut_style()
                                    .get_mut_paint()
                                    .set_background(Some(next_p.get_card().with_alpha(244)))
                                    .set_border(Border::new(
                                        next_g.get_border_width(),
                                        next_p.get_border(),
                                    ))
                                    .set_radius(CornerRadius::all(next_dock_r))
                                    .set_shadow(Some(next_g.get_shadow_lg()));
                                tree.mark_dirty(bottom_bar_host, DIRTY_PAINT);
                            }
                        },
                        (),
                    );
                });
            }
            if let Some(route_sig) = route_signal {
                let prev_route: Rc<RefCell<String>> =
                    Rc::new(RefCell::new(route_sig.get_untracked()));
                let saved_scrolls: Rc<RefCell<HashMap<String, Point>>> =
                    Rc::new(RefCell::new(HashMap::new()));
                let handle_route_scroll: SceneHandle = handle.clone();
                let _: Effect = Effect::new(move || {
                    let next_r: String = route_sig.get();
                    let prev_changed: Option<String> = prev_route.try_borrow_mut().ok().and_then(
                        |mut guard: RefMut<'_, String>| {
                            if *guard != next_r {
                                let old: String = guard.clone();
                                *guard = next_r.clone();
                                Some(old)
                            } else {
                                None
                            }
                        },
                    );
                    if let Some(old_r) = prev_changed {
                        let _: bool = menu_open.set_if_changed(false);
                        if !has_managed_outlet {
                            handle_route_scroll.with_write(
                                |tree: &mut SceneTree| {
                                    let cur_off: Point = tree
                                        .get_node(main_area)
                                        .map(|n: &SceneNode| n.get_layout_box().get_scroll_offset())
                                        .unwrap_or_default();
                                    let restore_off: Point = saved_scrolls
                                        .try_borrow_mut()
                                        .ok()
                                        .map(|mut map: RefMut<'_, HashMap<String, Point>>| {
                                            let _: Option<Point> = map.insert(old_r, cur_off);
                                            map.get(&next_r).copied().unwrap_or_default()
                                        })
                                        .unwrap_or_default();
                                    tree.set_scroll_offset(main_area, restore_off);
                                },
                                (),
                            );
                        }
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
                            tree.cancel_subtree_transitions(drawer_portal);
                            tree.cancel_subtree_transitions(drawer_panel);
                            if let Some(n) = tree.get_node_mut(drawer_portal) {
                                let st: &mut Style = n.get_mut_style();
                                st.get_mut_layout().set_display(Display::Flex);
                                st.get_mut_paint()
                                    .set_pointer_events(PointerEvents::Auto)
                                    .set_transform(Transform2D::default());
                            }
                            let dirty_l: u16 = DIRTY_LAYOUT | DIRTY_SUBTREE_LAYOUT;
                            tree.mark_dirty(drawer_portal, dirty_l | DIRTY_PAINT);
                            if let Some(n) = tree.get_node_mut(drawer_panel) {
                                let st: &mut Style = n.get_mut_style();
                                st.get_mut_layout().set_width(Length::Px(def_w));
                                st.get_mut_paint().set_transform(Transform2D::default());
                            }
                            let cur_y: f32 = tree
                                .get_node(drawer_scroll)
                                .map(|n: &SceneNode| n.get_layout_box().get_scroll_offset().get_y())
                                .unwrap_or(0.0);
                            tree.set_scroll_offset(drawer_scroll, Point::new(0.0, cur_y));
                            drop(tree.update_focus(None));
                            tree.start_transition(
                                NodeTransition::fade(drawer_portal, 0.0, 1.0, 0.22, false)
                                    .with_easing(Easing::EaseOutCubic),
                            );
                            tree.start_transition(
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
                        },
                        (),
                    );
                } else {
                    drawer_da_close.set(false);
                    drawer_dm_close.set(false);
                    let _: bool = drawer_expanded.set_if_changed(false);
                    handle.with_write(
                        |tree: &mut SceneTree| {
                            tree.cancel_subtree_transitions(drawer_portal);
                            tree.cancel_subtree_transitions(drawer_panel);
                            if let Some(n) = tree.get_node_mut(drawer_portal) {
                                let st: &mut Style = n.get_mut_style();
                                st.get_mut_layout().set_display(Display::None);
                                st.get_mut_paint()
                                    .set_pointer_events(PointerEvents::None)
                                    .set_opacity(1.0)
                                    .set_transform(Transform2D::default());
                            }
                            if let Some(n) = tree.get_node_mut(drawer_panel) {
                                n.get_mut_style()
                                    .get_mut_paint()
                                    .set_opacity(1.0)
                                    .set_transform(Transform2D::default());
                            }
                            let mut hover_stack: Vec<NodeId> = vec![drawer_portal];
                            while let Some(cur_id) = hover_stack.pop() {
                                if let Some(cur_n) = tree.get_node_mut(cur_id) {
                                    cur_n.get_mut_interaction().set_hovered(false);
                                    for &cid in cur_n.get_children().iter() {
                                        hover_stack.push(cid);
                                    }
                                }
                            }
                            let dirty_l: u16 = DIRTY_LAYOUT | DIRTY_SUBTREE_LAYOUT;
                            tree.mark_dirty(drawer_portal, dirty_l | DIRTY_PAINT);
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
        let mut ctx: Context = current_context();
        let tokens: ThemeTokens = self.get_tokens();
        let Self {
            title,
            subtitle,
            sections,
            ..
        } = self;
        let root: NodeId = view! {
            column {
                width: Length::Percent(100.0)
                max_width: Length::Px(1040.0)
                gap: SPACE_LG
                EuvHeader {
                    title: title.clone()
                    subtitle: subtitle.clone()
                    tokens: tokens
                }
            }
        };
        if !sections.is_empty() {
            ctx.append_many(root, sections.as_slice());
        }
        root
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
        let Self { children } = self;
        let root: NodeId = view! {
            row {
                width: Length::Percent(100.0)
                flex_wrap: FlexWrap::Wrap
                align: AlignItems::Center
                gap: SPACE_SM
            }
        };
        if !children.is_empty() {
            ctx.get_scene().with_write(
                |tree: &mut SceneTree| {
                    for cid in children.iter().copied() {
                        let (label_len, is_switch_like): (usize, bool) = tree
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
                                let switch_like: bool = n.get_children().len() > 1
                                    || matches!(
                                        n.a11y_props().map(|a: A11yProps| a.get_role()),
                                        Some(A11yRole::Switch | A11yRole::Checkbox)
                                    );
                                (direct.max(child_max), switch_like)
                            })
                            .unwrap_or((0, false));
                        if let Some(n) = tree.get_node_mut(cid) {
                            let lay: &mut LayoutStyle = n.get_mut_style().get_mut_layout();
                            let fixed_w: bool = !matches!(lay.get_width(), Length::Auto);
                            if !fixed_w {
                                lay.set_flex_grow(1.0).set_flex_shrink(1.0);
                                if matches!(lay.get_min_width(), Length::Auto) {
                                    let min_w: f32 = if is_switch_like {
                                        ACTION_ROW_SWITCH_MIN_WIDTH
                                    } else {
                                        ((label_len as f32) * 7.2 + 24.0).clamp(
                                            ACTION_ROW_MIN_BTN_WIDTH,
                                            ACTION_ROW_MAX_BTN_WIDTH,
                                        )
                                    };
                                    lay.set_min_width(Length::Px(min_w));
                                }
                            }
                        }
                    }
                },
                (),
            );
            ctx.append_many(root, children.as_slice());
        }
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
        let mut ctx: Context = current_context();
        let bp_sig: Signal<Breakpoint> = ctx.get_breakpoint();
        let Self { cards } = self;
        let max_cols: u16 = (cards.len() as u16).clamp(1, 3);
        let init_cols: u16 = match bp_sig.get_untracked() {
            Breakpoint::Compact => 1,
            Breakpoint::Medium => max_cols.min(2),
            Breakpoint::Expanded => max_cols,
        };
        let root: NodeId = view! {
            grid {
                columns: init_cols
                width: Length::Percent(100.0)
                gap: SPACE_MD
            }
        };
        if !cards.is_empty() {
            ctx.append_many(root, cards.as_slice());
        }
        let handle: SceneHandle = ctx.get_scene().clone();
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            Effect::new(move || {
                let cols: u16 = match bp_sig.get() {
                    Breakpoint::Compact => 1,
                    Breakpoint::Medium => max_cols.min(2),
                    Breakpoint::Expanded => max_cols,
                };
                handle.with_write(
                    |tree: &mut SceneTree| {
                        let changed: bool = if let Some(n) = tree.get_node_mut(root) {
                            if n.style_ref().get_layout().get_grid_columns() != cols {
                                n.get_mut_style().get_mut_layout().set_grid_columns(cols);
                                true
                            } else {
                                false
                            }
                        } else {
                            false
                        };
                        if changed {
                            let dirty_l: u16 = DIRTY_LAYOUT | DIRTY_SUBTREE_LAYOUT;
                            tree.mark_dirty(root, dirty_l | DIRTY_PAINT);
                        }
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
