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
            Self::Success => (palette.get_success(), palette.get_success_foreground()),
            Self::Warning => (palette.get_warning(), palette.get_warning_foreground()),
            Self::Destructive => (
                palette.get_destructive(),
                palette.get_destructive_foreground(),
            ),
        }
    }
}

/// Fluent tone builder for `EuvBadge`.
impl EuvBadge {
    /// Overrides the semantic tone fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source badge.
    /// - `EuvTone` - Target semantic tone.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated badge.
    #[must_use]
    pub fn tone(mut self, tone: EuvTone) -> Self {
        self.set_tone(tone);
        self
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
        let (bg, fg): (Color, Color) = self.get_tone().colors(p);
        let Self { label, .. } = self;
        let pill: NodeId = ctx.view(
            Style::empty()
                .row()
                .height(Length::Px(22.0))
                .justify(JustifyContent::Center)
                .align(AlignItems::Center)
                .padding(Edges::symmetric(0.0, 10.0))
                .background(bg)
                .radius(999.0),
        );
        let txt: NodeId = ctx.text(
            label,
            Style::empty()
                .font_size(11.0)
                .font_weight(FontWeight::SemiBold)
                .color(fg)
                .text_overflow(TextOverflow::Clip),
        );
        ctx.append(pill, txt);
        pill
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

/// Fluent height configuration for `EuvProgress`.
impl EuvProgress {
    /// Sets the progress bar track height in logical pixels.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source progress bar.
    /// - `f32` - Desired track height in logical pixels.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated progress bar.
    #[must_use]
    pub fn height(mut self, height: f32) -> Self {
        self.set_height(height);
        self
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
        let h: f32 = if self.get_height() > 0.0 {
            self.get_height().max(4.0)
        } else {
            8.0
        };
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

/// Window calculation, clamped scrolling, and reactive mounting for `EuvVirtualList`.
impl EuvVirtualList {
    /// Sets the visible scroll viewport height in logical pixels fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source virtual list.
    /// - `f32` - Viewport height in logical pixels.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated virtual list.
    #[must_use]
    pub fn viewport_height(mut self, height: f32) -> Self {
        self.set_viewport_height(height);
        self
    }

    /// Sets the overscan row count mounted above and below the viewport fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source virtual list.
    /// - `usize` - Number of extra rows to mount per edge.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated virtual list.
    #[must_use]
    pub fn overscan(mut self, overscan: usize) -> Self {
        self.set_overscan(overscan);
        self
    }

    /// Resolves effective viewport height (defaulting to `320.0` when unset).
    ///
    /// # Arguments
    ///
    /// - `&Self` - Virtual list configuration.
    ///
    /// # Returns
    ///
    /// - `f32` - Resolved viewport height in logical pixels.
    #[must_use]
    fn resolved_viewport_height(&self) -> f32 {
        let h: f32 = self.get_viewport_height();
        if h > 0.0 { h } else { 320.0 }
    }

    /// Returns the maximum valid scroll Y offset in logical pixels.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Virtual list configuration.
    ///
    /// # Returns
    ///
    /// - `f32` - Upper scroll bound `>= 0.0`.
    #[must_use]
    pub fn max_scroll_y(&self) -> f32 {
        let total_h: f32 = (self.get_total_items() as f32) * self.get_item_height().max(1.0);
        (total_h - self.resolved_viewport_height().max(1.0)).max(0.0)
    }

    /// Advances the reactive scroll offset by `delta_y`, clamped to `[0.0, max_scroll_y()]`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Virtual list configuration.
    /// - `f32` - Incremental vertical scroll delta in logical pixels.
    pub fn scroll_by(&self, delta_y: f32) {
        let max_y: f32 = self.max_scroll_y();
        let next: f32 = (self.get_scroll_y().get_untracked() + delta_y).clamp(0.0, max_y);
        let _: bool = self.get_scroll_y().set_if_changed(next);
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
        let item_h: f32 = self.get_item_height().max(1.0);
        let view_h: f32 = self.resolved_viewport_height().max(1.0);
        let scroll_y: f32 = self
            .get_scroll_y()
            .get_untracked()
            .clamp(0.0, self.max_scroll_y());
        let overscan: usize = if self.get_overscan() == 0 {
            3
        } else {
            self.get_overscan()
        };
        let first_vis: usize = (scroll_y / item_h).floor() as usize;
        let vis_count: usize = (view_h / item_h).ceil() as usize;
        let start: usize = first_vis.saturating_sub(overscan);
        let end: usize = (first_vis + vis_count + overscan).min(total);
        (start, end)
    }

    /// Mounts a reactive clipped scrollable list materializing only the visible row window as `scroll_y` changes.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Virtual list configuration.
    /// - `F` - Zero-context row builder closure taking `usize` row index and returning `NodeId`.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted reactive scroll viewport handle.
    pub fn mount_with<F>(&self, row_builder: F) -> NodeId
    where
        F: FnMut(usize) -> NodeId + 'static,
    {
        let mut ctx: BuildContext = current_build_context();
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let list_cfg: Self = self.clone();
        let scroll_cfg: Self = self.clone();
        let scroll_sig: Signal<f32> = self.get_scroll_y();
        let view_h: f32 = self.resolved_viewport_height().max(1.0);
        let builder_cell: Rc<RefCell<F>> = Rc::new(RefCell::new(row_builder));
        let border_w: f32 = g.get_border_width();
        let inner_view_h: f32 = (view_h - border_w * 2.0).max(1.0);
        let item_h: f32 = self.get_item_height().max(1.0);
        let total_h: f32 = (self.get_total_items() as f32) * item_h;
        let max_scroll: f32 = (total_h - inner_view_h).max(0.0);
        let container: NodeId = ctx.scroll_view(
            Style::empty()
                .column()
                .width(Length::Percent(100.0))
                .height(Length::Px(view_h))
                .background(p.get_card())
                .border(border_w, p.get_border())
                .radius(g.get_radius_md())
                .stop_scroll_propagation(true),
        );
        let init_scroll: f32 = scroll_sig.get_untracked().clamp(0.0, max_scroll);
        ctx.set_virtual_scroll(container, init_scroll, total_h);
        ctx.on_scroll(container, move |ev: &UiEvent| {
            if let UiEventKind::Scroll(delta) = ev.get_kind() {
                let next: f32 = (scroll_cfg.get_scroll_y().get_untracked() + delta.get_y())
                    .clamp(0.0, max_scroll);
                let _: bool = scroll_cfg.get_scroll_y().set_if_changed(next);
            }
        });
        let range_memo: Memo<(usize, usize)> = Memo::new(move || list_cfg.visible_range());
        let range_for_scroll: Memo<(usize, usize)> = range_memo;
        let scene_for_scroll: SceneHandle = ctx.get_scene().clone();
        let scene_for_rows: SceneHandle = ctx.get_scene().clone();
        let ctx_for_rows: BuildContext = ctx.clone();
        let (vlist_scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            let _: Effect = Effect::new(move || {
                let (start, end): (usize, usize) = range_memo.get();
                let cur_scroll: f32 = scroll_sig.get_untracked().clamp(0.0, max_scroll);
                let local_scroll_y: f32 = (cur_scroll - (start as f32) * item_h).max(0.0);
                let (child_scope, rows_host): (Scope, NodeId) = Scope::run(|_: Scope| {
                    with_build_context(ctx_for_rows.clone(), || {
                        let mut row_ctx: BuildContext = current_build_context();
                        let host: NodeId =
                            row_ctx.view(Style::empty().column().width(Length::Percent(100.0)));
                        if let Ok(mut build_row) = builder_cell.try_borrow_mut() {
                            for idx in start..end {
                                let row_node: NodeId = build_row(idx);
                                row_ctx.append(host, row_node);
                            }
                        }
                        host
                    })
                });
                scene_for_rows.with_write(
                    |tree: &mut SceneTree| {
                        tree.replace_children(container, vec![rows_host]);
                        tree.attach_scope(rows_host, child_scope);
                        tree.set_virtual_scroll(container, cur_scroll, total_h);
                        tree.set_scroll_offset(container, Point::new(0.0, local_scroll_y));
                    },
                    (),
                );
            });
            Effect::new(move || {
                let cur_scroll: f32 = scroll_sig.get().clamp(0.0, max_scroll);
                let (start, _): (usize, usize) = range_for_scroll.get();
                let local_scroll_y: f32 = (cur_scroll - (start as f32) * item_h).max(0.0);
                scene_for_scroll.with_write(
                    |tree: &mut SceneTree| {
                        tree.set_virtual_scroll(container, cur_scroll, total_h);
                        tree.set_scroll_offset(container, Point::new(0.0, local_scroll_y));
                    },
                    (),
                );
            })
        });
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                tree.attach_scope(container, vlist_scope);
            },
            (),
        );
        container
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
            let cell: NodeId = ctx.view(Style::empty().flex_grow(1.0).overflow(Overflow::Hidden));
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
                let cell: NodeId =
                    ctx.view(Style::empty().flex_grow(1.0).overflow(Overflow::Hidden));
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
