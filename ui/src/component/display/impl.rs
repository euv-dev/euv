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

/// Declarative `#[component]` implementation for `EuvBadge`.
#[component]
/// Inherent implementation of [`EuvBadge`].
impl EuvBadge {
    /// Renders `EuvBadge` into the ambient framework-managed `Context`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component descriptor reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted root scene node handle.
    fn render(&self) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let (bg, fg): (Color, Color) = self.get_tone().colors(p);
        let Self { label, .. } = self.clone();
        view! {
            row {
                height: Length::Px(24.0)
                justify: JustifyContent::Center
                align: AlignItems::Center
                padding: Edges::symmetric(0.0, 10.0)
                overflow: Overflow::Hidden
                background: bg
                border: (1.0, p.get_border().with_alpha(72))
                radius: 999.0
                text {
                    font_size: 11.0
                    font_weight: FontWeight::SemiBold
                    color: fg
                    text_align: TextAlign::Center
                    text_overflow: TextOverflow::Ellipsis
                    { label }
                }
            }
        }
    }
}

/// Declarative `#[component]` implementation for `EuvAlert`.
#[component]
/// Inherent implementation of [`EuvAlert`].
impl EuvAlert {
    /// Renders `EuvAlert` into the ambient framework-managed `Context`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component descriptor reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted root scene node handle.
    fn render(&self) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let (accent, _): (Color, Color) = self.get_tone().colors(p);
        let border_col: Color = match self.get_tone() {
            EuvTone::Primary | EuvTone::Neutral => p.get_border(),
            _ => accent.with_alpha(152),
        };
        let Self {
            title, description, ..
        } = self.clone();
        view! {
            column {
                width: Length::Percent(100.0)
                padding: Edges::all(SPACE_MD)
                gap: SPACE_XS
                background: p.get_muted()
                border: (g.get_border_width(), border_col)
                radius: g.get_radius_md()
                overflow: Overflow::Hidden
                text {
                    width: Length::Percent(100.0)
                    font_size: 14.0
                    font_weight: FontWeight::SemiBold
                    color: p.get_foreground()
                    { title }
                }
                text {
                    width: Length::Percent(100.0)
                    font_size: 13.0
                    color: p.get_muted_foreground()
                    { description }
                }
            }
        }
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

/// Declarative `#[component]` implementation for `EuvProgress`.
#[component]
/// Inherent implementation of [`EuvProgress`].
impl EuvProgress {
    /// Renders `EuvProgress` into the ambient framework-managed `Context`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component descriptor reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted root scene node handle.
    fn render(&self) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let h: f32 = if self.get_height() > 0.0 {
            self.get_height().max(4.0)
        } else {
            8.0
        };
        let pct: f32 = self.get_value().clamp(0.0, 1.0) * 100.0;
        view! {
            row {
                width: Length::Percent(100.0)
                height: Length::Px(h)
                background: p.get_secondary()
                radius: h * 0.5
                overflow: Overflow::Hidden
                view {
                    width: Length::Percent(pct)
                    height: Length::Percent(100.0)
                    background: p.get_primary()
                    radius: h * 0.5
                }
            }
        }
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
        let mut ctx: Context = current_context();
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let scroll_sig: Signal<f32> = self.get_scroll_y();
        let total_items: usize = self.get_total_items();
        let overscan: usize = if self.get_overscan() == 0 {
            3
        } else {
            self.get_overscan()
        };
        let view_h: f32 = self.resolved_viewport_height().max(1.0);
        let builder_cell: Rc<RefCell<F>> = Rc::new(RefCell::new(row_builder));
        let border_w: f32 = g.get_border_width();
        let inner_view_h: f32 = (view_h - border_w * 2.0).max(1.0);
        let item_h: f32 = self.get_item_height().max(1.0);
        let vis_count: usize = (view_h / item_h).ceil() as usize;
        let total_h: f32 = (total_items as f32) * item_h;
        let max_scroll: f32 = (total_h - inner_view_h).max(0.0);
        let max_scroll_range: f32 = self.max_scroll_y();
        let container: NodeId = view! {
            scroll {
                style: Style::default().column()
                width: Length::Percent(100.0)
                height: Length::Px(view_h)
                background: p.get_card()
                border: (border_w, p.get_border())
                radius: g.get_radius_md()
                stop_scroll_propagation: true
                on_scroll: move |ev: &UiEvent| {
                    if let UiEventKind::Scroll(delta) = ev.get_kind() {
                        let next: f32 =
                            (scroll_sig.get_untracked() + delta.get_y()).clamp(0.0, max_scroll);
                        let _: bool = scroll_sig.set_if_changed(next);
                    }
                }
            }
        };
        let mut rows_host_style: Style = Style::default();
        rows_host_style
            .get_mut_layout()
            .set_flex_direction(FlexDirection::Column)
            .set_width(Length::Percent(100.0));
        let rows_host: NodeId = ctx.view(rows_host_style);
        ctx.append(container, rows_host);
        let init_scroll: f32 = scroll_sig.get_untracked().clamp(0.0, max_scroll);
        ctx.set_virtual_scroll(container, init_scroll, total_h);
        let scene_for_rows: SceneHandle = ctx.get_scene().clone();
        let ctx_for_rows: Context = ctx.clone();
        let window: Rc<RefCell<VecDeque<(usize, Scope, NodeId)>>> =
            Rc::new(RefCell::new(VecDeque::new()));
        let window_effect: Rc<RefCell<VecDeque<(usize, Scope, NodeId)>>> = Rc::clone(&window);
        let (vlist_scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            let window_cleanup: Rc<RefCell<VecDeque<(usize, Scope, NodeId)>>> = Rc::clone(&window);
            on_cleanup(move || {
                if let Ok(mut guard) = window_cleanup.try_borrow_mut() {
                    while let Some((_, row_scope, _)) = guard.pop_front() {
                        row_scope.dispose();
                    }
                }
            });
            let mut current_range: (usize, usize) = (0, 0);
            let mut initialized: bool = false;
            Effect::new(move || {
                let raw_scroll: f32 = scroll_sig.get();
                let cur_scroll: f32 = raw_scroll.clamp(0.0, max_scroll);
                let (new_start, new_end): (usize, usize) = if total_items == 0 {
                    (0, 0)
                } else {
                    let clamped_for_range: f32 = raw_scroll.clamp(0.0, max_scroll_range);
                    let first_vis: usize = (clamped_for_range / item_h).floor() as usize;
                    let start: usize = first_vis.saturating_sub(overscan);
                    let end: usize = (first_vis + vis_count + overscan).min(total_items);
                    (start, end)
                };
                let local_scroll_y: f32 = (cur_scroll - (new_start as f32) * item_h).max(0.0);
                if initialized && (new_start, new_end) == current_range {
                    scene_for_rows.with_write(
                        |tree: &mut SceneTree| {
                            tree.set_virtual_scroll(container, cur_scroll, total_h);
                            tree.set_scroll_offset(container, Point::new(0.0, local_scroll_y));
                        },
                        (),
                    );
                    return;
                }
                let (old_start, old_end): (usize, usize) = current_range;
                let has_overlap: bool = initialized
                    && old_start < old_end
                    && new_start < new_end
                    && new_start < old_end
                    && old_start < new_end;
                let mut retired_scopes: Vec<Scope> = Vec::new();
                let (prefix_start, prefix_end, suffix_start, suffix_end): (
                    usize,
                    usize,
                    usize,
                    usize,
                ) = if has_overlap {
                    if let Ok(mut guard) = window_effect.try_borrow_mut() {
                        while let Some(&(idx, _, _)) = guard.front() {
                            if idx < new_start {
                                if let Some((_, scope, _)) = guard.pop_front() {
                                    retired_scopes.push(scope);
                                }
                            } else {
                                break;
                            }
                        }
                        while let Some(&(idx, _, _)) = guard.back() {
                            if idx >= new_end {
                                if let Some((_, scope, _)) = guard.pop_back() {
                                    retired_scopes.push(scope);
                                }
                            } else {
                                break;
                            }
                        }
                    }
                    let keep_start: usize = old_start.max(new_start);
                    let keep_end: usize = old_end.min(new_end);
                    (new_start, keep_start, keep_end, new_end)
                } else {
                    if let Ok(mut guard) = window_effect.try_borrow_mut() {
                        retired_scopes.reserve(guard.len());
                        while let Some((_, scope, _)) = guard.pop_front() {
                            retired_scopes.push(scope);
                        }
                    }
                    (new_start, new_end, new_end, new_end)
                };
                for retired in retired_scopes.into_iter() {
                    retired.dispose();
                }
                let prefix_len: usize = prefix_end.saturating_sub(prefix_start);
                let suffix_len: usize = suffix_end.saturating_sub(suffix_start);
                let mut built_prefix: Vec<(usize, Scope, NodeId)> = Vec::with_capacity(prefix_len);
                let mut built_suffix: Vec<(usize, Scope, NodeId)> = Vec::with_capacity(suffix_len);
                if prefix_len > 0 || suffix_len > 0 {
                    with_context(ctx_for_rows.clone(), || {
                        if let Ok(mut build_row) = builder_cell.try_borrow_mut() {
                            for idx in prefix_start..prefix_end {
                                let (row_scope, row_node): (Scope, NodeId) =
                                    Scope::run(|_: Scope| build_row(idx));
                                built_prefix.push((idx, row_scope, row_node));
                            }
                            for idx in suffix_start..suffix_end {
                                let (row_scope, row_node): (Scope, NodeId) =
                                    Scope::run(|_: Scope| build_row(idx));
                                built_suffix.push((idx, row_scope, row_node));
                            }
                        }
                    });
                }
                let mut next_children: Vec<NodeId> =
                    Vec::with_capacity(new_end.saturating_sub(new_start));
                if let Ok(mut guard) = window_effect.try_borrow_mut() {
                    for entry in built_prefix.into_iter().rev() {
                        guard.push_front(entry);
                    }
                    for entry in built_suffix.into_iter() {
                        guard.push_back(entry);
                    }
                    for &(_, _, node_id) in guard.iter() {
                        next_children.push(node_id);
                    }
                }
                current_range = (new_start, new_end);
                initialized = true;
                scene_for_rows.with_write(
                    |tree: &mut SceneTree| {
                        tree.replace_children(rows_host, next_children);
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

/// Declarative `#[component]` implementation for `EuvTable`.
#[component]
/// Inherent implementation of [`EuvTable`].
impl EuvTable {
    /// Renders `EuvTable` into the ambient framework-managed `Context`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component descriptor reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted root scene node handle.
    fn render(&self) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let Self { headers, rows, .. } = self.clone();
        view! {
            column {
                width: Length::Percent(100.0)
                background: p.get_card()
                border: (g.get_border_width(), p.get_border())
                radius: g.get_radius_md()
                overflow: Overflow::Hidden
                row {
                    width: Length::Percent(100.0)
                    padding: Edges::symmetric(SPACE_SM, SPACE_MD)
                    background: p.get_muted()
                    gap: SPACE_SM
                    for h in headers.into_iter() {
                        view {
                            flex_grow: 1.0
                            overflow: Overflow::Hidden
                            text {
                                font_size: 12.0
                                font_weight: FontWeight::SemiBold
                                color: p.get_muted_foreground()
                                { h }
                            }
                        }
                    }
                }
                for row_cells in rows.into_iter() {
                    view {
                        width: Length::Percent(100.0)
                        height: Length::Px(g.get_border_width().max(1.0))
                        background: p.get_border()
                    }
                    row {
                        width: Length::Percent(100.0)
                        padding: Edges::symmetric(SPACE_SM, SPACE_MD)
                        gap: SPACE_SM
                        for cell_text in row_cells.into_iter() {
                            view {
                                flex_grow: 1.0
                                overflow: Overflow::Hidden
                                text {
                                    font_size: 13.0
                                    color: p.get_foreground()
                                    { cell_text }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
