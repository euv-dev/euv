use super::*;

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
            ..
        } = self;
        let root_style: Style = Style::empty()
            .width(Length::Percent(100.0))
            .height(Length::Percent(100.0))
            .padding(safe.to_edges())
            .background(p.get_background())
            .overflow(Overflow::Hidden);
        if bp.is_compact() {
            let root: NodeId = ctx.view(root_style.column());
            let menu_open: Signal<bool> = Signal::new(false);
            let top_bar: NodeId = ctx.view(
                Style::empty()
                    .row()
                    .width(Length::Percent(100.0))
                    .height(Length::Px(MOBILE_HEADER_HEIGHT))
                    .padding(Edges::symmetric(0.0, SPACE_MD))
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
            let mut menu_btn: EuvButton =
                EuvButton::new("Menu".to_string(), tokens).on_click(move |_: &UiEvent| {
                    let _: bool = menu_open.update(|v: &mut bool| *v = !*v);
                });
            menu_btn
                .set_variant(EuvButtonVariant::Outline)
                .set_size(EuvButtonSize::Small);
            let menu_btn_node: NodeId = menu_btn.mount(ctx);
            ctx.append(top_bar, title);
            ctx.append(top_bar, menu_btn_node);
            let body: NodeId = ctx.scroll_view(
                Style::empty()
                    .column()
                    .flex_grow(1.0)
                    .width(Length::Percent(100.0))
                    .padding(Edges::all(SPACE_MD))
                    .gap(SPACE_MD),
            );
            ctx.append(body, main_content);
            let drawer_portal: NodeId = ctx.portal(Style::empty());
            let drawer_panel: NodeId = ctx.scroll_view(
                Style::empty()
                    .column()
                    .width(Length::Percent(82.0))
                    .max_width(Length::Px(280.0))
                    .height(Length::Percent(100.0))
                    .padding(Edges::all(SPACE_LG))
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
            let mut close_btn: EuvButton =
                EuvButton::new("Close".to_string(), tokens).on_click(move |_: &UiEvent| {
                    let _: bool = menu_open.set_if_changed(false);
                });
            close_btn
                .set_variant(EuvButtonVariant::Ghost)
                .set_size(EuvButtonSize::Small);
            let close_node: NodeId = close_btn.mount(ctx);
            ctx.append(drawer_head, drawer_title);
            ctx.append(drawer_head, close_node);
            ctx.append(drawer_panel, drawer_head);
            ctx.append(drawer_panel, sidebar);
            ctx.append(drawer_portal, drawer_panel);
            let handle: SceneHandle = ctx.get_scene().clone();
            let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
                handle.bind_style(drawer_portal, move || {
                    let disp: Display = if menu_open.get() {
                        Display::Flex
                    } else {
                        Display::None
                    };
                    Style::empty()
                        .row()
                        .display(disp)
                        .position(Position::Fixed)
                        .inset(Some(0.0), Some(0.0), Some(0.0), Some(0.0))
                        .width(Length::Percent(100.0))
                        .height(Length::Percent(100.0))
                        .justify(JustifyContent::Start)
                        .background(p.get_overlay())
                        .z_index(Z_INDEX_DRAWER)
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
            let nav_rail: NodeId = ctx.scroll_view(
                Style::empty()
                    .column()
                    .width(Length::Px(nav_w))
                    .height(Length::Percent(100.0))
                    .padding(Edges::all(nav_pad))
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
                    .padding(Edges::all(main_pad))
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
