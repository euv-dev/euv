use super::*;

/// Builder and adaptive mounting methods for `EuvAppShell`.
impl EuvAppShell {
    /// Creates an adaptive cross-platform `EuvAppShell`.
    ///
    /// # Arguments
    ///
    /// - `String` - Application brand name.
    /// - `NodeId` - Sidebar navigation node.
    /// - `NodeId` - Main content node.
    /// - `ThemeTokens` - Active design tokens.
    ///
    /// # Returns
    ///
    /// - `Self` - Constructed app shell.
    #[must_use]
    pub fn create(
        brand: String,
        sidebar: NodeId,
        main_content: NodeId,
        tokens: ThemeTokens,
    ) -> Self {
        Self {
            brand,
            sidebar,
            main_content,
            tokens,
        }
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
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
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
            .background(p.get_background());
        if bp.is_compact() {
            let root: NodeId = ctx.view(root_style.column());
            let top_bar: NodeId = ctx.view(
                Style::empty()
                    .row()
                    .width(Length::Percent(100.0))
                    .height(Length::Px(MOBILE_HEADER_HEIGHT))
                    .padding(Edges::symmetric(0.0, SPACE_LG))
                    .align(AlignItems::Center)
                    .background(p.get_card())
                    .border(g.get_border_width(), p.get_border())
                    .z_index(Z_INDEX_CHROME),
            );
            let title: NodeId = ctx.text(
                brand,
                Style::empty()
                    .font_size(16.0)
                    .font_weight(FontWeight::Bold)
                    .color(p.get_foreground()),
            );
            ctx.append(top_bar, title);
            let body: NodeId = ctx.scroll_view(
                Style::empty()
                    .column()
                    .flex_grow(1.0)
                    .width(Length::Percent(100.0))
                    .padding(Edges::all(SPACE_LG))
                    .gap(SPACE_LG),
            );
            ctx.append(body, main_content);
            ctx.append(root, top_bar);
            ctx.append(root, body);
            root
        } else {
            let root: NodeId = ctx.view(root_style.row());
            let nav_w: f32 = if bp == Breakpoint::Medium {
                200.0
            } else {
                NAV_SIDEBAR_WIDTH
            };
            let nav_rail: NodeId = ctx.view(
                Style::empty()
                    .column()
                    .width(Length::Px(nav_w))
                    .height(Length::Percent(100.0))
                    .padding(Edges::all(SPACE_LG))
                    .gap(SPACE_MD)
                    .background(p.get_muted())
                    .border(g.get_border_width(), p.get_border())
                    .z_index(Z_INDEX_CHROME),
            );
            let brand_node: NodeId = ctx.text(
                brand,
                Style::empty()
                    .font_size(18.0)
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
                    .padding(Edges::all(SPACE_2XL))
                    .gap(SPACE_XL),
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
