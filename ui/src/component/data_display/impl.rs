use super::*;

/// `Mountable` implementation for `EuvTag`.
impl Mountable for EuvTag {
    /// Mounts the bordered tag chip into `ctx`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Tag props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted tag handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let (accent, _): (Color, Color) = self.get_tone().colors(p);
        let Self { label, .. } = self;
        let box_node: NodeId = ctx.view(
            Style::empty()
                .row()
                .align(AlignItems::Center)
                .padding(Edges::symmetric(2.0, SPACE_SM))
                .background(p.get_muted())
                .border(g.get_border_width(), accent)
                .radius(g.get_radius_sm()),
        );
        let txt: NodeId = ctx.text(
            label,
            Style::empty()
                .font_size(12.0)
                .font_weight(FontWeight::Medium)
                .color(p.get_foreground()),
        );
        ctx.append(box_node, txt);
        box_node
    }
}

/// `Mountable` implementation for `EuvAvatar`.
impl Mountable for EuvAvatar {
    /// Mounts the circular initials avatar into `ctx`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Avatar props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted avatar handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let sz: f32 = self.get_size();
        let Self { initials, .. } = self;
        let circle: NodeId = ctx.view(
            Style::empty()
                .row()
                .width(Length::Px(sz))
                .height(Length::Px(sz))
                .justify(JustifyContent::Center)
                .align(AlignItems::Center)
                .background(p.get_primary())
                .radius(sz * 0.5),
        );
        let txt: NodeId = ctx.text(
            initials,
            Style::empty()
                .font_size((sz * 0.42).max(10.0))
                .font_weight(FontWeight::Bold)
                .color(p.get_primary_foreground()),
        );
        ctx.append(circle, txt);
        circle
    }
}

/// `Mountable` implementation for `EuvIcon`.
impl Mountable for EuvIcon {
    /// Mounts the icon box into `ctx`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Icon props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted icon handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let sz: f32 = self.get_size();
        let color: Color = self.get_color();
        let Self { symbol, .. } = self;
        let box_node: NodeId = ctx.view(
            Style::empty()
                .row()
                .width(Length::Px(sz))
                .height(Length::Px(sz))
                .justify(JustifyContent::Center)
                .align(AlignItems::Center),
        );
        let txt: NodeId = ctx.text(
            symbol,
            Style::empty()
                .font_size(sz * 0.78)
                .font_weight(FontWeight::SemiBold)
                .color(color),
        );
        ctx.append(box_node, txt);
        box_node
    }
}

/// `Mountable` implementation for `EuvKbd`.
impl Mountable for EuvKbd {
    /// Mounts the monospace keycap surface into `ctx`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Keycap props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted keycap handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let Self { key, .. } = self;
        let cap: NodeId = ctx.view(
            Style::empty()
                .row()
                .padding(Edges::symmetric(2.0, 6.0))
                .background(p.get_muted())
                .border(g.get_border_width(), p.get_border())
                .radius(g.get_radius_sm()),
        );
        let txt: NodeId = ctx.text(
            key,
            Style::empty()
                .font_family(FontFamily::Monospace)
                .font_size(11.0)
                .color(p.get_foreground()),
        );
        ctx.append(cap, txt);
        cap
    }
}

/// `Mountable` implementation for `EuvTypography`.
impl Mountable for EuvTypography {
    /// Mounts the text node styled for its `EuvTypographyVariant`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Typography props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted text handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let variant: EuvTypographyVariant = self.get_variant();
        let Self { text, .. } = self;
        let style: Style = match variant {
            EuvTypographyVariant::H1 => Style::empty()
                .font_size(g.get_font_title())
                .font_weight(FontWeight::Bold)
                .color(p.get_foreground()),
            EuvTypographyVariant::H2 => Style::empty()
                .font_size(g.get_font_heading())
                .font_weight(FontWeight::Bold)
                .color(p.get_foreground()),
            EuvTypographyVariant::H3 => Style::empty()
                .font_size(15.0)
                .font_weight(FontWeight::SemiBold)
                .color(p.get_foreground()),
            EuvTypographyVariant::Body => Style::empty()
                .font_size(g.get_font_body())
                .color(p.get_foreground()),
            EuvTypographyVariant::Caption => Style::empty()
                .font_size(12.0)
                .color(p.get_muted_foreground()),
        };
        ctx.text(text, style)
    }
}

/// `Mountable` implementation for `EuvEmpty`.
impl Mountable for EuvEmpty {
    /// Mounts the centered empty-state box into `ctx`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Empty props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted empty container.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let Self {
            title, description, ..
        } = self;
        let box_node: NodeId = ctx.view(
            Style::empty()
                .column()
                .width(Length::Percent(100.0))
                .align(AlignItems::Center)
                .padding(Edges::all(SPACE_2XL))
                .gap(SPACE_SM)
                .border(g.get_border_width(), p.get_border())
                .radius(g.get_radius_lg()),
        );
        let glyph: NodeId = ctx.text(
            EMPTY_ICON_GLYPH.to_string(),
            Style::empty()
                .font_size(20.0)
                .color(p.get_muted_foreground()),
        );
        let t_node: NodeId = ctx.text(
            title,
            Style::empty()
                .font_size(15.0)
                .font_weight(FontWeight::SemiBold)
                .color(p.get_foreground()),
        );
        let d_node: NodeId = ctx.text(
            description,
            Style::empty()
                .font_size(13.0)
                .color(p.get_muted_foreground()),
        );
        ctx.append(box_node, glyph);
        ctx.append(box_node, t_node);
        ctx.append(box_node, d_node);
        box_node
    }
}

/// `Mountable` implementation for `EuvSkeleton`.
impl Mountable for EuvSkeleton {
    /// Mounts the rounded skeleton surface into `ctx`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Skeleton props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted skeleton handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        ctx.view(
            Style::empty()
                .width(Length::Px(self.get_width()))
                .height(Length::Px(self.get_height()))
                .background(p.get_secondary())
                .radius(g.get_radius_sm()),
        )
    }
}

/// `Mountable` implementation for `EuvSpinner`.
impl Mountable for EuvSpinner {
    /// Mounts a vector `Canvas` spinner ring into `ctx`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Spinner props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted spinner canvas handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let sz: f32 = self.get_size();
        ctx.canvas(
            Style::empty().width(Length::Px(sz)).height(Length::Px(sz)),
            move |bounds: Rect, dl: &mut DisplayList| {
                let cx: f32 = bounds.get_x() + bounds.get_width() * 0.5;
                let cy: f32 = bounds.get_y() + bounds.get_height() * 0.5;
                let r: f32 = (bounds.get_width().min(bounds.get_height()) * 0.45).max(4.0);
                dl.push(DrawCommand::Circle {
                    center: Point::new(cx, cy),
                    radius: r,
                    color: p.get_primary(),
                });
                dl.push(DrawCommand::Circle {
                    center: Point::new(cx, cy),
                    radius: (r * 0.62).max(2.0),
                    color: p.get_background(),
                });
            },
        )
    }
}

/// `Mountable` implementation for `EuvTimeline`.
impl Mountable for EuvTimeline {
    /// Mounts timeline dots, titles, timestamps, and details into `ctx`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Timeline props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted timeline handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let Self { items, .. } = self;
        let col: NodeId = ctx.view(
            Style::empty()
                .column()
                .width(Length::Percent(100.0))
                .gap(SPACE_MD),
        );
        for item in items.into_iter() {
            let row: NodeId = ctx.view(
                Style::empty()
                    .row()
                    .width(Length::Percent(100.0))
                    .gap(SPACE_MD),
            );
            let dot: NodeId = ctx.view(
                Style::empty()
                    .width(Length::Px(10.0))
                    .height(Length::Px(10.0))
                    .background(p.get_primary())
                    .radius(5.0),
            );
            let body: NodeId = ctx.view(Style::empty().column().flex_grow(1.0).gap(2.0));
            let title: NodeId = ctx.text(
                item.get_title().clone(),
                Style::empty()
                    .font_size(14.0)
                    .font_weight(FontWeight::SemiBold)
                    .color(p.get_foreground()),
            );
            let ts: NodeId = ctx.text(
                item.get_timestamp().clone(),
                Style::empty()
                    .font_size(11.0)
                    .color(p.get_muted_foreground()),
            );
            let detail: NodeId = ctx.text(
                item.get_detail().clone(),
                Style::empty()
                    .font_size(13.0)
                    .color(p.get_muted_foreground()),
            );
            ctx.append(body, title);
            ctx.append(body, ts);
            ctx.append(body, detail);
            ctx.append(row, dot);
            ctx.append(row, body);
            ctx.append(col, row);
        }
        col
    }
}

/// `Mountable` implementation for `EuvList`.
impl Mountable for EuvList {
    /// Mounts the bordered list rows into `ctx`.
    ///
    /// # Arguments
    ///
    /// - `Self` - List props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted list handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let Self { items, .. } = self;
        let list: NodeId = ctx.view(
            Style::empty()
                .column()
                .width(Length::Percent(100.0))
                .background(p.get_card())
                .border(g.get_border_width(), p.get_border())
                .radius(g.get_radius_md()),
        );
        for item in items.into_iter() {
            let row: NodeId = ctx.view(
                Style::empty()
                    .row()
                    .width(Length::Percent(100.0))
                    .padding(Edges::symmetric(SPACE_SM, SPACE_MD))
                    .border(g.get_border_width(), p.get_border()),
            );
            let txt: NodeId = ctx.text(
                item,
                Style::empty().font_size(13.0).color(p.get_foreground()),
            );
            ctx.append(row, txt);
            ctx.append(list, row);
        }
        list
    }
}

/// Builder and recursive rendering methods for `EuvTree`.
impl EuvTree {
    /// Recursively appends indented tree rows into `parent`.
    ///
    /// # Arguments
    ///
    /// - `&mut BuildContext` - Active build context.
    /// - `NodeId` - Parent container.
    /// - `&[EuvTreeNode]` - Sibling tree nodes.
    /// - `usize` - Current indentation depth.
    /// - `ThemePalette` - Active theme palette.
    fn mount_branch(
        ctx: &mut BuildContext,
        parent: NodeId,
        nodes: &[EuvTreeNode],
        depth: usize,
        palette: ThemePalette,
    ) {
        for node in nodes.iter() {
            let prefix: &str = if node.get_children().is_empty() {
                TREE_LEAF_PREFIX
            } else {
                TREE_EXPAND_PREFIX
            };
            let indent: f32 = (depth as f32) * 16.0;
            let row: NodeId = ctx.view(
                Style::empty()
                    .row()
                    .width(Length::Percent(100.0))
                    .padding(Edges::new(SPACE_XS, SPACE_SM, SPACE_XS, SPACE_SM + indent)),
            );
            let txt: NodeId = ctx.text(
                format!("{prefix}{}", node.get_label()),
                Style::empty()
                    .font_size(13.0)
                    .color(palette.get_foreground()),
            );
            ctx.append(row, txt);
            ctx.append(parent, row);
            if !node.get_children().is_empty() {
                Self::mount_branch(ctx, parent, node.get_children(), depth + 1, palette);
            }
        }
    }
}

/// `Mountable` implementation for `EuvTree`.
impl Mountable for EuvTree {
    /// Mounts the indented tree container into `ctx`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Tree props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted tree handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let Self { roots, .. } = self;
        let container: NodeId = ctx.view(
            Style::empty()
                .column()
                .width(Length::Percent(100.0))
                .padding(Edges::all(SPACE_SM))
                .background(p.get_card())
                .border(g.get_border_width(), p.get_border())
                .radius(g.get_radius_md()),
        );
        Self::mount_branch(ctx, container, &roots, 0, p);
        container
    }
}
