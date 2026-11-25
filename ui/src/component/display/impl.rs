use super::*;

/// Tone color resolution helper for `EuvTone`.
impl EuvTone {
    /// Resolves `(background, foreground)` colors for this tone under `palette`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Semantic tone.
    /// - `ThemePalette` - Active theme palette.
    ///
    /// # Returns
    ///
    /// - `(Color, Color)` - Background and foreground colors.
    #[must_use]
    pub fn colors(&self, palette: ThemePalette) -> (Color, Color) {
        match *self {
            Self::Neutral => (palette.get_secondary(), palette.get_secondary_foreground()),
            Self::Primary => (palette.get_primary(), palette.get_primary_foreground()),
            Self::Success => (palette.get_success(), Color::hex(0xFFFFFF)),
            Self::Warning => (palette.get_warning(), Color::hex(0x09090B)),
            Self::Destructive => (
                palette.get_destructive(),
                palette.get_destructive_foreground(),
            ),
        }
    }
}

/// Builder methods for `EuvBadge`.
impl EuvBadge {
    /// Creates a new `EuvBadge`.
    ///
    /// # Arguments
    ///
    /// - `String` - Badge text.
    /// - `EuvTone` - Semantic tone.
    /// - `ThemeTokens` - Active design tokens.
    ///
    /// # Returns
    ///
    /// - `Self` - Constructed badge.
    #[must_use]
    pub fn create(label: String, tone: EuvTone, tokens: ThemeTokens) -> Self {
        Self {
            label,
            tone,
            tokens,
        }
    }
}

/// `Mountable` implementation for `EuvBadge`.
impl Mountable for EuvBadge {
    /// Mounts the pill badge into `ctx`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Badge props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted badge node.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let (bg, fg): (Color, Color) = self.get_tone().colors(p);
        let Self { label, .. } = self;
        let pill: NodeId = ctx.view(
            Style::empty()
                .row()
                .align(AlignItems::Center)
                .padding(Edges::symmetric(SPACE_XS, SPACE_SM))
                .background(bg)
                .radius(g.get_radius_sm()),
        );
        let txt: NodeId = ctx.text(
            label,
            Style::empty()
                .font_size(12.0)
                .font_weight(FontWeight::SemiBold)
                .color(fg),
        );
        ctx.append(pill, txt);
        pill
    }
}

/// Builder methods for `EuvAlert`.
impl EuvAlert {
    /// Creates a new `EuvAlert`.
    ///
    /// # Arguments
    ///
    /// - `String` - Alert title.
    /// - `String` - Alert message.
    /// - `EuvTone` - Semantic tone.
    /// - `ThemeTokens` - Active design tokens.
    ///
    /// # Returns
    ///
    /// - `Self` - Constructed alert.
    #[must_use]
    pub fn create(title: String, description: String, tone: EuvTone, tokens: ThemeTokens) -> Self {
        Self {
            title,
            description,
            tone,
            tokens,
        }
    }
}

/// `Mountable` implementation for `EuvAlert`.
impl Mountable for EuvAlert {
    /// Mounts the alert box into `ctx`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Alert props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted alert node.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let (accent, _): (Color, Color) = self.get_tone().colors(p);
        let Self {
            title, description, ..
        } = self;
        let box_node: NodeId = ctx.view(
            Style::empty()
                .column()
                .width(Length::Percent(100.0))
                .padding(Edges::all(SPACE_MD))
                .gap(SPACE_XS)
                .background(p.get_muted())
                .border(g.get_border_width(), accent)
                .radius(g.get_radius_md()),
        );
        let t_node: NodeId = ctx.text(
            title,
            Style::empty()
                .font_size(14.0)
                .font_weight(FontWeight::SemiBold)
                .color(p.get_foreground()),
        );
        let d_node: NodeId = ctx.text(
            description,
            Style::empty()
                .font_size(13.0)
                .color(p.get_muted_foreground()),
        );
        ctx.append(box_node, t_node);
        ctx.append(box_node, d_node);
        box_node
    }
}

/// Builder methods for `EuvProgress`.
impl EuvProgress {
    /// Creates a progress bar at `value` in `[0.0, 1.0]`.
    ///
    /// # Arguments
    ///
    /// - `f32` - Completion ratio.
    /// - `ThemeTokens` - Active design tokens.
    ///
    /// # Returns
    ///
    /// - `Self` - Constructed progress bar.
    #[must_use]
    pub fn create(value: f32, tokens: ThemeTokens) -> Self {
        Self {
            value: value.clamp(0.0, 1.0),
            height: 8.0,
            tokens,
        }
    }
}

/// `Mountable` implementation for `EuvProgress`.
impl Mountable for EuvProgress {
    /// Mounts the progress track and filled indicator bar.
    ///
    /// # Arguments
    ///
    /// - `Self` - Progress props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted track node.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let h: f32 = self.get_height().max(4.0);
        let pct: f32 = self.get_value().clamp(0.0, 1.0) * 100.0;
        let track: NodeId = ctx.view(
            Style::empty()
                .row()
                .width(Length::Percent(100.0))
                .height(Length::Px(h))
                .background(p.get_secondary())
                .radius(h * 0.5)
                .overflow(Overflow::Hidden),
        );
        let fill: NodeId = ctx.view(
            Style::empty()
                .width(Length::Percent(pct))
                .height(Length::Percent(100.0))
                .background(p.get_primary())
                .radius(h * 0.5),
        );
        ctx.append(track, fill);
        track
    }
}

/// Builder and mounting for `EuvCodeBlock`.
impl EuvCodeBlock {
    /// Creates a monospace code block.
    ///
    /// # Arguments
    ///
    /// - `String` - Code text.
    /// - `ThemeTokens` - Active design tokens.
    ///
    /// # Returns
    ///
    /// - `Self` - Constructed code block.
    #[must_use]
    pub fn create(code: String, tokens: ThemeTokens) -> Self {
        Self { code, tokens }
    }
}

/// `Mountable` implementation for `EuvCodeBlock`.
impl Mountable for EuvCodeBlock {
    /// Mounts the monospace code surface into `ctx`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Code block props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted code block node.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let Self { code, .. } = self;
        let container: NodeId = ctx.view(
            Style::empty()
                .column()
                .width(Length::Percent(100.0))
                .padding(Edges::all(SPACE_MD))
                .background(p.get_muted())
                .border(g.get_border_width(), p.get_border())
                .radius(g.get_radius_md()),
        );
        let text: NodeId = ctx.text(
            code,
            Style::empty()
                .font_family(FontFamily::Monospace)
                .font_size(13.0)
                .color(p.get_foreground()),
        );
        ctx.append(container, text);
        container
    }
}

/// Window calculation and mounting for `EuvVirtualList`.
impl EuvVirtualList {
    /// Creates a virtualized list descriptor for `total_items` fixed-height rows.
    ///
    /// # Arguments
    ///
    /// - `usize` - Total logical rows.
    /// - `f32` - Row height in pixels.
    /// - `f32` - Visible viewport height.
    /// - `f32` - Current scroll Y offset.
    /// - `ThemeTokens` - Active design tokens.
    ///
    /// # Returns
    ///
    /// - `Self` - Virtual list configuration.
    #[must_use]
    pub fn create(
        total_items: usize,
        item_height: f32,
        viewport_height: f32,
        scroll_y: f32,
        tokens: ThemeTokens,
    ) -> Self {
        Self {
            total_items,
            item_height: item_height.max(1.0),
            viewport_height: viewport_height.max(1.0),
            scroll_y: scroll_y.max(0.0),
            overscan: 3,
            tokens,
        }
    }

    /// Computes the `(start_index, end_index_exclusive)` window of rows visible in the viewport.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Virtual list configuration.
    ///
    /// # Returns
    ///
    /// - `(usize, usize)` - Half-open range of indices to materialize.
    #[must_use]
    pub fn visible_range(&self) -> (usize, usize) {
        let total: usize = self.get_total_items();
        if total == 0 {
            return (0, 0);
        }
        let first_vis: usize = (self.get_scroll_y() / self.get_item_height()).floor() as usize;
        let vis_count: usize =
            (self.get_viewport_height() / self.get_item_height()).ceil() as usize;
        let start: usize = first_vis.saturating_sub(self.get_overscan());
        let end: usize = (first_vis + vis_count + self.get_overscan()).min(total);
        (start, end)
    }

    /// Mounts only the visible window of rows using `row_builder` into a clipped scroll container.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Virtual list configuration.
    /// - `&mut BuildContext` - Active build context.
    /// - `F` - Row builder closure taking `(&mut BuildContext, usize)` and returning `NodeId`.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted scroll viewport handle.
    pub fn mount_with<F>(&self, ctx: &mut BuildContext, mut row_builder: F) -> NodeId
    where
        F: FnMut(&mut BuildContext, usize) -> NodeId,
    {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let (start, end): (usize, usize) = self.visible_range();
        let container: NodeId = ctx.scroll_view(
            Style::empty()
                .column()
                .width(Length::Percent(100.0))
                .height(Length::Px(self.get_viewport_height()))
                .background(p.get_card())
                .border(g.get_border_width(), p.get_border())
                .radius(g.get_radius_md()),
        );
        let top_spacer_h: f32 = (start as f32) * self.get_item_height();
        if top_spacer_h > 0.0 {
            let spacer: NodeId = ctx.view(Style::empty().height(Length::Px(top_spacer_h)));
            ctx.append(container, spacer);
        }
        for idx in start..end {
            let row_node: NodeId = row_builder(ctx, idx);
            ctx.append(container, row_node);
        }
        let remaining: usize = self.get_total_items().saturating_sub(end);
        if remaining > 0 {
            let bottom_spacer: NodeId = ctx.view(
                Style::empty().height(Length::Px((remaining as f32) * self.get_item_height())),
            );
            ctx.append(container, bottom_spacer);
        }
        container
    }
}

/// Builder and mounting for `EuvTable`.
impl EuvTable {
    /// Creates a cross-platform structured table.
    ///
    /// # Arguments
    ///
    /// - `Vec<String>` - Column headers.
    /// - `Vec<Vec<String>>` - Table rows.
    /// - `ThemeTokens` - Active design tokens.
    ///
    /// # Returns
    ///
    /// - `Self` - Constructed table.
    #[must_use]
    pub fn create(headers: Vec<String>, rows: Vec<Vec<String>>, tokens: ThemeTokens) -> Self {
        Self {
            headers,
            rows,
            tokens,
        }
    }
}

/// `Mountable` implementation for `EuvTable`.
impl Mountable for EuvTable {
    /// Mounts header row and data rows consistently across all platforms.
    ///
    /// # Arguments
    ///
    /// - `Self` - Table props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted table node.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let Self { headers, rows, .. } = self;
        let table: NodeId = ctx.view(
            Style::empty()
                .column()
                .width(Length::Percent(100.0))
                .background(p.get_card())
                .border(g.get_border_width(), p.get_border())
                .radius(g.get_radius_md())
                .overflow(Overflow::Hidden),
        );
        let head_row: NodeId = ctx.view(
            Style::empty()
                .row()
                .width(Length::Percent(100.0))
                .padding(Edges::symmetric(SPACE_SM, SPACE_MD))
                .background(p.get_muted())
                .gap(SPACE_SM),
        );
        for h in headers.into_iter() {
            let cell: NodeId = ctx.view(Style::empty().flex_grow(1.0));
            let txt: NodeId = ctx.text(
                h,
                Style::empty()
                    .font_size(12.0)
                    .font_weight(FontWeight::SemiBold)
                    .color(p.get_muted_foreground()),
            );
            ctx.append(cell, txt);
            ctx.append(head_row, cell);
        }
        ctx.append(table, head_row);
        for row in rows.into_iter() {
            let row_node: NodeId = ctx.view(
                Style::empty()
                    .row()
                    .width(Length::Percent(100.0))
                    .padding(Edges::symmetric(SPACE_SM, SPACE_MD))
                    .border(g.get_border_width(), p.get_border())
                    .gap(SPACE_SM),
            );
            for cell_text in row.into_iter() {
                let cell: NodeId = ctx.view(Style::empty().flex_grow(1.0));
                let txt: NodeId = ctx.text(
                    cell_text,
                    Style::empty().font_size(13.0).color(p.get_foreground()),
                );
                ctx.append(cell, txt);
                ctx.append(row_node, cell);
            }
            ctx.append(table, row_node);
        }
        table
    }
}
