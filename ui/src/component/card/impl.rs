use super::*;

/// Child composition helpers for `EuvCard`.
impl EuvCard {
    /// Appends a child `NodeId` to the card body.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source card.
    /// - `NodeId` - Mounted child node.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated card.
    #[must_use]
    pub fn child(mut self, child: NodeId) -> Self {
        self.get_mut_children().push(child);
        self
    }
}

/// `Mountable` implementation for `EuvCard`.
impl Mountable for EuvCard {
    /// Mounts the styled card surface, header text, and body children into `ctx`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Owned card props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted card node handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let tokens: ThemeTokens = self.get_tokens();
        let p: ThemePalette = tokens.get_palette();
        let g: ThemeGeometry = tokens.get_geometry();
        let Self {
            title,
            subtitle,
            children,
            ..
        } = self;
        let card: NodeId = ctx.view(tokens.card_style().width(Length::Percent(100.0)));
        if !title.is_empty() {
            let title_node: NodeId = ctx.text(
                title,
                Style::empty()
                    .font_size(g.get_font_heading())
                    .font_weight(FontWeight::SemiBold)
                    .color(p.get_card_foreground()),
            );
            ctx.append(card, title_node);
        }
        if !subtitle.is_empty() {
            let sub_node: NodeId = ctx.text(
                subtitle,
                Style::empty()
                    .font_size(13.0)
                    .color(p.get_muted_foreground()),
            );
            ctx.append(card, sub_node);
        }
        for child in children.into_iter() {
            ctx.append(card, child);
        }
        card
    }
}

/// `Mountable` implementation for `EuvHeader`.
impl Mountable for EuvHeader {
    /// Mounts the page title and subtitle block into `ctx`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Owned header props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted header node handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let Self {
            title, subtitle, ..
        } = self;
        let container: NodeId = ctx.view(
            Style::empty()
                .column()
                .width(Length::Percent(100.0))
                .gap(SPACE_XS)
                .padding(Edges::new(0.0, 0.0, SPACE_MD, 0.0)),
        );
        let h1: NodeId = ctx.text(
            title.clone(),
            Style::empty()
                .font_size(g.get_font_title())
                .font_weight(FontWeight::Bold)
                .color(p.get_foreground()),
        );
        let mut a11y: A11yProps = A11yProps::new(A11yRole::Heading, title);
        a11y.set_heading_level(1);
        ctx.set_a11y(h1, a11y);
        ctx.append(container, h1);
        if !subtitle.is_empty() {
            let sub: NodeId = ctx.text(
                subtitle,
                Style::empty()
                    .font_size(g.get_font_body())
                    .color(p.get_muted_foreground()),
            );
            ctx.append(container, sub);
        }
        container
    }
}

/// `Mountable` implementation for `EuvStatCard`.
impl Mountable for EuvStatCard {
    /// Mounts the stat card into `ctx`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Owned stat card props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted stat card handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let tokens: ThemeTokens = self.get_tokens();
        let p: ThemePalette = tokens.get_palette();
        let Self {
            label,
            value,
            caption,
            ..
        } = self;
        let box_node: NodeId = ctx.view(tokens.card_style().gap(SPACE_XS));
        let lbl: NodeId = ctx.text(
            label,
            Style::empty()
                .font_size(12.0)
                .color(p.get_muted_foreground()),
        );
        let val: NodeId = ctx.text(
            value,
            Style::empty()
                .font_size(22.0)
                .font_weight(FontWeight::Bold)
                .color(p.get_primary()),
        );
        ctx.append(box_node, lbl);
        ctx.append(box_node, val);
        if !caption.is_empty() {
            let cap: NodeId = ctx.text(
                caption,
                Style::empty()
                    .font_size(11.0)
                    .color(p.get_muted_foreground()),
            );
            ctx.append(box_node, cap);
        }
        box_node
    }
}
