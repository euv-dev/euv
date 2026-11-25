use super::*;

/// Builder and mounting methods for `EuvPageContainer`.
impl EuvPageContainer {
    /// Appends a child node to the container.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source container.
    /// - `NodeId` - Mounted child node.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated container.
    #[must_use]
    pub fn child(mut self, node: NodeId) -> Self {
        self.get_mut_children().push(node);
        self
    }
}

/// `Mountable` implementation for `EuvPageContainer`.
impl Mountable for EuvPageContainer {
    /// Mounts the centered page container into `ctx`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Container props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted container handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let max_w: f32 = if self.get_max_width() > 0.0 {
            self.get_max_width()
        } else {
            1120.0
        };
        let Self { children, .. } = self;
        let node: NodeId = ctx.view(
            Style::empty()
                .column()
                .width(Length::Percent(100.0))
                .max_width(Length::Px(max_w))
                .padding(Edges::all(SPACE_XL))
                .gap(SPACE_LG),
        );
        for c in children.into_iter() {
            ctx.append(node, c);
        }
        node
    }
}

/// Builder and mounting methods for `EuvPanel`.
impl EuvPanel {
    /// Appends a child node to the panel.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source panel.
    /// - `NodeId` - Child handle.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated panel.
    #[must_use]
    pub fn child(mut self, node: NodeId) -> Self {
        self.get_mut_children().push(node);
        self
    }
}

/// `Mountable` implementation for `EuvPanel`.
impl Mountable for EuvPanel {
    /// Mounts the panel surface into `ctx`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Panel props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted panel handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let Self {
            title, children, ..
        } = self;
        let panel: NodeId = ctx.view(
            Style::empty()
                .column()
                .width(Length::Percent(100.0))
                .padding(Edges::all(SPACE_MD))
                .gap(SPACE_SM)
                .background(p.get_muted())
                .border(g.get_border_width(), p.get_border())
                .radius(g.get_radius_md()),
        );
        if !title.is_empty() {
            let heading: NodeId = ctx.text(
                title,
                Style::empty()
                    .font_size(14.0)
                    .font_weight(FontWeight::SemiBold)
                    .color(p.get_foreground()),
            );
            ctx.append(panel, heading);
        }
        for c in children.into_iter() {
            ctx.append(panel, c);
        }
        panel
    }
}

/// Builder methods for `EuvSpace`.
impl EuvSpace {
    /// Creates a horizontal `EuvSpace` with `gap`.
    ///
    /// # Arguments
    ///
    /// - `f32` - Gap in logical pixels.
    ///
    /// # Returns
    ///
    /// - `Self` - Horizontal spacing container.
    #[must_use]
    pub fn row(gap: f32) -> Self {
        Self::new(false, gap)
    }
    /// Creates a vertical `EuvSpace` with `gap`.
    ///
    /// # Arguments
    ///
    /// - `f32` - Gap in logical pixels.
    ///
    /// # Returns
    ///
    /// - `Self` - Vertical spacing container.
    #[must_use]
    pub fn column(gap: f32) -> Self {
        Self::new(true, gap)
    }
    /// Appends a child node.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source container.
    /// - `NodeId` - Child handle.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated container.
    #[must_use]
    pub fn child(mut self, node: NodeId) -> Self {
        self.get_mut_children().push(node);
        self
    }
}

/// `Mountable` implementation for `EuvSpace`.
impl Mountable for EuvSpace {
    /// Mounts the flex spacing container into `ctx`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Spacing props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted flex container.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let base: Style = if self.get_vertical() {
            Style::empty().column()
        } else {
            Style::empty().row().align(AlignItems::Center)
        };
        let gap: f32 = self.get_gap();
        let Self { children, .. } = self;
        let container: NodeId = ctx.view(base.gap(gap));
        for c in children.into_iter() {
            ctx.append(container, c);
        }
        container
    }
}

/// Builder methods for `EuvGrid`.
impl EuvGrid {
    /// Appends a child cell to the grid.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source grid.
    /// - `NodeId` - Child node handle.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated grid.
    #[must_use]
    pub fn child(mut self, node: NodeId) -> Self {
        self.get_mut_children().push(node);
        self
    }
}

/// `Mountable` implementation for `EuvGrid`.
impl Mountable for EuvGrid {
    /// Mounts the grid container into `ctx`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Grid props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted grid handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let cols: u16 = self.get_columns();
        let gap: f32 = self.get_gap();
        let Self { children, .. } = self;
        let node: NodeId = ctx.view(
            Style::empty()
                .grid(cols)
                .width(Length::Percent(100.0))
                .gap(gap),
        );
        for c in children.into_iter() {
            ctx.append(node, c);
        }
        node
    }
}

/// Builder methods for `EuvScrollArea`.
impl EuvScrollArea {
    /// Appends a child node into the scroll area.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source scroll area.
    /// - `NodeId` - Child handle.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated scroll area.
    #[must_use]
    pub fn child(mut self, node: NodeId) -> Self {
        self.get_mut_children().push(node);
        self
    }
}

/// `Mountable` implementation for `EuvScrollArea`.
impl Mountable for EuvScrollArea {
    /// Mounts the scrollable viewport into `ctx`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Scroll area props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted scroll container handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let h: f32 = self.get_height();
        let Self { children, .. } = self;
        let area: NodeId = ctx.scroll_view(
            Style::empty()
                .column()
                .width(Length::Percent(100.0))
                .height(Length::Px(h))
                .padding(Edges::all(SPACE_SM))
                .gap(SPACE_SM)
                .background(p.get_card())
                .border(g.get_border_width(), p.get_border())
                .radius(g.get_radius_md()),
        );
        for c in children.into_iter() {
            ctx.append(area, c);
        }
        area
    }
}

/// `Mountable` implementation for `EuvSplitter`.
impl Mountable for EuvSplitter {
    /// Mounts primary pane, divider handle, and secondary pane side-by-side.
    ///
    /// # Arguments
    ///
    /// - `Self` - Splitter props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted splitter container.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let ratio: f32 = self.get_ratio_percent();
        let primary: NodeId = self.get_primary();
        let secondary: NodeId = self.get_secondary();
        let row: NodeId = ctx.view(Style::empty().row().width(Length::Percent(100.0)));
        let left: NodeId = ctx.view(
            Style::empty()
                .column()
                .width(Length::Percent(ratio))
                .padding(Edges::all(SPACE_SM)),
        );
        let handle: NodeId = ctx.view(
            Style::empty()
                .width(Length::Px(4.0))
                .background(p.get_border()),
        );
        let right: NodeId = ctx.view(
            Style::empty()
                .column()
                .flex_grow(1.0)
                .padding(Edges::all(SPACE_SM)),
        );
        ctx.append(left, primary);
        ctx.append(right, secondary);
        ctx.append(row, left);
        ctx.append(row, handle);
        ctx.append(row, right);
        row
    }
}

/// Builder methods for `EuvNavbar`.
impl EuvNavbar {
    /// Appends a trailing action node to the navbar.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source navbar.
    /// - `NodeId` - Action node handle.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated navbar.
    #[must_use]
    pub fn action(mut self, node: NodeId) -> Self {
        self.get_mut_actions().push(node);
        self
    }
}

/// `Mountable` implementation for `EuvNavbar`.
impl Mountable for EuvNavbar {
    /// Mounts the top navigation bar into `ctx`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Navbar props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted navbar handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let Self { brand, actions, .. } = self;
        let bar: NodeId = ctx.view(
            Style::empty()
                .row()
                .width(Length::Percent(100.0))
                .height(Length::Px(MOBILE_HEADER_HEIGHT))
                .padding(Edges::symmetric(SPACE_SM, SPACE_LG))
                .justify(JustifyContent::SpaceBetween)
                .align(AlignItems::Center)
                .background(p.get_card())
                .border(g.get_border_width(), p.get_border()),
        );
        let title: NodeId = ctx.text(
            brand,
            Style::empty()
                .font_size(16.0)
                .font_weight(FontWeight::Bold)
                .color(p.get_foreground()),
        );
        let right: NodeId = ctx.view(Style::empty().row().align(AlignItems::Center).gap(SPACE_SM));
        for a in actions.into_iter() {
            ctx.append(right, a);
        }
        ctx.append(bar, title);
        ctx.append(bar, right);
        bar
    }
}

/// `Mountable` implementation for `EuvFooter`.
impl Mountable for EuvFooter {
    /// Mounts the bordered footer strip into `ctx`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Footer props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted footer handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let Self { text, .. } = self;
        let footer: NodeId = ctx.view(
            Style::empty()
                .row()
                .width(Length::Percent(100.0))
                .padding(Edges::symmetric(SPACE_MD, SPACE_LG))
                .justify(JustifyContent::Center)
                .background(p.get_muted())
                .border(g.get_border_width(), p.get_border()),
        );
        let txt: NodeId = ctx.text(
            text,
            Style::empty()
                .font_size(12.0)
                .color(p.get_muted_foreground()),
        );
        ctx.append(footer, txt);
        footer
    }
}

/// `Mountable` implementation for `EuvButtonGroup`.
impl Mountable for EuvButtonGroup {
    /// Mounts grouped buttons inside a shared rounded frame.
    ///
    /// # Arguments
    ///
    /// - `Self` - Button group props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted button group handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let Self { children, .. } = self;
        let grp: NodeId = ctx.view(
            Style::empty()
                .row()
                .gap(SPACE_XS)
                .padding(Edges::all(SPACE_XS))
                .border(g.get_border_width(), p.get_border())
                .radius(g.get_radius_md()),
        );
        for c in children.into_iter() {
            ctx.append(grp, c);
        }
        grp
    }
}

/// `Mountable` implementation for `EuvToggle`.
impl Mountable for EuvToggle {
    /// Mounts the pressable toggle with reactive pressed highlight.
    ///
    /// # Arguments
    ///
    /// - `Self` - Toggle props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted toggle handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let pressed: Signal<bool> = self.get_pressed();
        let Self { label, .. } = self;
        let btn: NodeId = ctx.view(Style::empty());
        let txt: NodeId = ctx.text(
            label,
            Style::empty()
                .font_size(13.0)
                .font_weight(FontWeight::Medium)
                .color(p.get_foreground()),
        );
        ctx.append(btn, txt);
        let handle: SceneHandle = ctx.get_scene().clone();
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            handle.bind_style(btn, move || {
                let on: bool = pressed.get();
                Style::empty()
                    .row()
                    .align(AlignItems::Center)
                    .padding(Edges::symmetric(SPACE_SM, SPACE_MD))
                    .radius(g.get_radius_md())
                    .border(g.get_border_width(), p.get_border())
                    .background(if on { p.get_accent() } else { p.get_card() })
                    .cursor(CursorIcon::Pointer)
            })
        });
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                tree.attach_scope(btn, scope);
            },
            (),
        );
        ctx.on_click(btn, move |_: &UiEvent| {
            let _: bool = pressed.update(|v: &mut bool| *v = !*v);
        });
        btn
    }
}
