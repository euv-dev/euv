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
    #[inline(always)]
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
        let ctx: Context = current_context();
        let tokens: ThemeTokens = self.get_tokens();
        let p: ThemePalette = tokens.get_palette();
        let g: ThemeGeometry = tokens.get_geometry();
        let (bg, fg): (Color, Color) = self.get_tone().colors(p);
        let Self { label, .. } = self;
        let mut row_st: Style = Style::default();
        row_st
            .get_mut_layout()
            .set_flex_direction(FlexDirection::Row)
            .set_height(Length::Px(24.0))
            .set_justify_content(JustifyContent::Center)
            .set_align_items(AlignItems::Center)
            .set_padding(Edges::symmetric(0.0, 10.0))
            .set_overflow(Overflow::Hidden);
        row_st
            .get_mut_paint()
            .set_background(Some(bg))
            .set_border(Border::new(1.0, p.get_border().with_alpha(72)))
            .set_radius(CornerRadius::all(g.get_radius_sm().max(0.0)));
        let mut txt_st: Style = Style::default();
        txt_st
            .get_mut_text()
            .set_font_size(11.0)
            .set_weight(FontWeight::SemiBold)
            .set_color(fg)
            .set_align(TextAlign::Center)
            .set_overflow(TextOverflow::Ellipsis);
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                let root: NodeId = tree.create_view(row_st);
                let txt_id: NodeId = tree.create_text(label.clone(), txt_st);
                tree.append_child(root, txt_id);
                root
            },
            NodeId::default(),
        )
    }
}

/// Fluent selection builder for `EuvAlert`.
impl EuvAlert {
    /// Controls whether the alert heading and description text are selectable.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source alert.
    /// - `bool` - `true` to enable text selection.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated alert.
    #[must_use]
    pub fn selectable(mut self, enabled: bool) -> Self {
        self.set_selectable(enabled);
        self
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
        let ctx: Context = current_context();
        let tokens: ThemeTokens = self.get_tokens();
        let p: ThemePalette = tokens.get_palette();
        let g: ThemeGeometry = tokens.get_geometry();
        let is_selectable: bool = self.get_selectable();
        let (accent, _): (Color, Color) = self.get_tone().colors(p);
        let border_col: Color = match self.get_tone() {
            EuvTone::Primary | EuvTone::Neutral => p.get_border(),
            _ => accent.with_alpha(152),
        };
        let Self {
            title, description, ..
        } = self;
        let mut col_st: Style = Style::default().selectable(is_selectable);
        col_st
            .get_mut_layout()
            .set_flex_direction(FlexDirection::Column)
            .set_width(Length::Percent(100.0))
            .set_padding(Edges::all(SPACE_MD))
            .set_row_gap(SPACE_XS)
            .set_column_gap(SPACE_XS)
            .set_overflow(Overflow::Hidden);
        col_st
            .get_mut_paint()
            .set_background(Some(p.get_muted()))
            .set_border(Border::new(g.get_border_width().max(0.0), border_col))
            .set_radius(CornerRadius::all(g.get_radius_md().max(0.0)));
        let mut title_st: Style = Style::default();
        title_st.get_mut_layout().set_width(Length::Percent(100.0));
        title_st
            .get_mut_text()
            .set_font_size(14.0)
            .set_weight(FontWeight::SemiBold)
            .set_color(p.get_foreground());
        let mut desc_st: Style = Style::default();
        desc_st.get_mut_layout().set_width(Length::Percent(100.0));
        desc_st
            .get_mut_text()
            .set_font_size(13.0)
            .set_color(p.get_muted_foreground());
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                let root: NodeId = tree.create_view(col_st);
                let title_id: NodeId = tree.create_text(title.clone(), title_st);
                let desc_id: NodeId = tree.create_text(description.clone(), desc_st);
                tree.append_children(root, &[title_id, desc_id]);
                root
            },
            NodeId::default(),
        )
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
        let ctx: Context = current_context();
        let p: ThemePalette = self.get_tokens().get_palette();
        let h: f32 = if self.get_height() > 0.0 {
            self.get_height().max(4.0)
        } else {
            8.0
        };
        let pct: f32 = self.get_value().clamp(0.0, 1.0) * 100.0;
        let radius: CornerRadius = CornerRadius::all((h * 0.5).max(0.0));
        let mut track_st: Style = Style::default();
        track_st
            .get_mut_layout()
            .set_flex_direction(FlexDirection::Row)
            .set_width(Length::Percent(100.0))
            .set_height(Length::Px(h))
            .set_overflow(Overflow::Hidden);
        track_st
            .get_mut_paint()
            .set_background(Some(p.get_secondary()))
            .set_radius(radius);
        let mut fill_st: Style = Style::default();
        fill_st
            .get_mut_layout()
            .set_width(Length::Percent(pct))
            .set_height(Length::Percent(100.0));
        fill_st
            .get_mut_paint()
            .set_background(Some(p.get_primary()))
            .set_radius(radius);
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                let track: NodeId = tree.create_view(track_st);
                let fill: NodeId = tree.create_view(fill_st);
                tree.append_child(track, fill);
                track
            },
            NodeId::default(),
        )
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

    /// Mounts a reactive clipped scrollable list returning a controller handle for O(1) stable-ID single-row patches and refreshes.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Virtual list configuration.
    /// - `F` - Zero-context row builder closure taking `usize` row index and returning `NodeId`.
    ///
    /// # Returns
    ///
    /// - `EuvVirtualListHandle` - Mounted viewport root and targeted single-row update controller.
    #[must_use]
    pub fn mount_with_handle<F>(&self, row_builder: F) -> EuvVirtualListHandle
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
        let builder_cell: Rc<RefCell<VirtualRowBuilder>> =
            Rc::new(RefCell::new(Box::new(row_builder)));
        let builder_for_effect: Rc<RefCell<VirtualRowBuilder>> = Rc::clone(&builder_cell);
        let border_w: f32 = g.get_border_width();
        let inner_view_h: f32 = (view_h - border_w * 2.0).max(1.0);
        let item_h: f32 = self.get_item_height().max(1.0);
        let vis_count: usize = (view_h / item_h).ceil() as usize;
        let total_h: f32 = (total_items as f32) * item_h;
        let max_scroll: f32 = (total_h - inner_view_h).max(0.0);
        let max_scroll_range: f32 = self.max_scroll_y();
        let on_vlist_scroll = move |ev: &UiEvent| {
            if let UiEventKind::Scroll(delta) = ev.get_kind() {
                let next: f32 = (scroll_sig.get_untracked() + delta.get_y()).clamp(0.0, max_scroll);
                let _: bool = scroll_sig.set_if_changed(next);
            }
        };
        let container: NodeId = view! {
            scroll {
                style: Style::default().column()
                width: Length::Percent(100.0)
                height: Length::Px(view_h)
                background: p.get_card()
                border: (border_w, p.get_border())
                radius: g.get_radius_md()
                stop_scroll_propagation: true
                on_scroll: on_vlist_scroll
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
        let scene_for_handle: SceneHandle = scene_for_rows.clone();
        let ctx_for_rows: Context = ctx.clone();
        let ctx_for_handle: Context = ctx.clone();
        let window: VirtualRowWindow = Rc::new(RefCell::new(VecDeque::new()));
        let window_effect: VirtualRowWindow = Rc::clone(&window);
        let window_handle: VirtualRowWindow = Rc::clone(&window);
        let (vlist_scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            let window_cleanup: VirtualRowWindow = Rc::clone(&window);
            on_cleanup(move || {
                if let Ok(mut guard) = window_cleanup.try_borrow_mut() {
                    while let Some((_, row_scope, _)) = guard.pop_front() {
                        row_scope.dispose();
                    }
                }
            });
            let mut current_range: (usize, usize) = (0, 0);
            let mut initialized: bool = false;
            let mut retired_scopes: Vec<Scope> = Vec::new();
            let mut retired_nodes: Vec<NodeId> = Vec::new();
            let mut built_prefix: Vec<(usize, Scope, NodeId)> = Vec::new();
            let mut built_suffix: Vec<(usize, Scope, NodeId)> = Vec::new();
            let mut next_children: Vec<NodeId> = Vec::new();
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
                retired_scopes.clear();
                retired_nodes.clear();
                let (prefix_start, prefix_end, suffix_start, suffix_end): (
                    usize,
                    usize,
                    usize,
                    usize,
                ) = if has_overlap {
                    if let Ok(mut guard) = window_effect.try_borrow_mut() {
                        while let Some(&(idx, _, _)) = guard.front() {
                            if idx < new_start {
                                if let Some((_, scope, node_id)) = guard.pop_front() {
                                    retired_scopes.push(scope);
                                    retired_nodes.push(node_id);
                                }
                            } else {
                                break;
                            }
                        }
                        while let Some(&(idx, _, _)) = guard.back() {
                            if idx >= new_end {
                                if let Some((_, scope, node_id)) = guard.pop_back() {
                                    retired_scopes.push(scope);
                                    retired_nodes.push(node_id);
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
                        retired_nodes.reserve(guard.len());
                        while let Some((_, scope, node_id)) = guard.pop_front() {
                            retired_scopes.push(scope);
                            retired_nodes.push(node_id);
                        }
                    }
                    (new_start, new_end, new_end, new_end)
                };
                for retired in retired_scopes.drain(..) {
                    retired.dispose();
                }
                let prefix_len: usize = prefix_end.saturating_sub(prefix_start);
                let suffix_len: usize = suffix_end.saturating_sub(suffix_start);
                built_prefix.clear();
                built_suffix.clear();
                if prefix_len > 0 || suffix_len > 0 {
                    built_prefix.reserve(prefix_len);
                    built_suffix.reserve(suffix_len);
                    untracked(|| {
                        with_context(ctx_for_rows.clone(), || {
                            if let Ok(mut build_row) = builder_for_effect.try_borrow_mut() {
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
                    });
                }
                next_children.clear();
                next_children.reserve(new_end.saturating_sub(new_start));
                let mut newly_attached: Vec<NodeId> = Vec::with_capacity(prefix_len + suffix_len);
                if let Ok(mut guard) = window_effect.try_borrow_mut() {
                    for entry in built_prefix.drain(..).rev() {
                        newly_attached.push(entry.2);
                        guard.push_front(entry);
                    }
                    for entry in built_suffix.drain(..) {
                        newly_attached.push(entry.2);
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
                        for old_node in retired_nodes.drain(..) {
                            tree.dispose_subtree(old_node);
                        }
                        for new_node in newly_attached.iter().copied() {
                            if let Some(child_node) = tree.get_node_mut(new_node) {
                                child_node.set_parent(Some(rows_host));
                            }
                        }
                        if let Some(host_node) = tree.get_node_mut(rows_host) {
                            let kids: &mut Vec<NodeId> = host_node.get_mut_children();
                            kids.clear();
                            kids.extend_from_slice(next_children.as_slice());
                        }
                        let dirty_l: u16 = DIRTY_LAYOUT | DIRTY_SUBTREE_LAYOUT;
                        let dirty_c: u16 = DIRTY_CHILDREN | DIRTY_PAINT;
                        tree.mark_dirty(rows_host, dirty_l | dirty_c);
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
        EuvVirtualListHandle {
            container,
            rows_host,
            scene: scene_for_handle,
            ctx: ctx_for_handle,
            window: window_handle,
            builder: builder_cell,
        }
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
        self.mount_with_handle(row_builder).root()
    }
}

/// Stable-ID O(1) single-row lookup, surgical patching, and isolated refresh operations for `EuvVirtualListHandle`.
impl EuvVirtualListHandle {
    /// Returns the mounted scrollable container `NodeId`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Virtual list controller reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted container root handle.
    #[must_use]
    pub fn root(&self) -> NodeId {
        let Self { container, .. } = self;
        *container
    }

    /// Resolves the mounted `NodeId` for logical row `index` in O(1) when currently materialized in the viewport window.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Virtual list controller reference.
    /// - `usize` - Stable logical row index.
    ///
    /// # Returns
    ///
    /// - `Option<NodeId>` - Active row node handle if currently visible, or `None` when off-screen.
    #[must_use]
    pub fn row_node(&self, index: usize) -> Option<NodeId> {
        let Self { window, .. } = self;
        let guard: Ref<'_, VecDeque<(usize, Scope, NodeId)>> = window.try_borrow().ok()?;
        let &(start_idx, _, _) = guard.front()?;
        let slot: usize = index.checked_sub(start_idx)?;
        let &(found_idx, _, node_id) = guard.get(slot)?;
        if found_idx == index {
            Some(node_id)
        } else {
            None
        }
    }

    /// Mutates a single materialized row in place inside one `SceneTree` write lock without rebuilding sibling rows or invalidating `rows_host` children.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Virtual list controller reference.
    /// - `usize` - Stable logical row index to patch.
    /// - `F` - Mutation closure receiving `(&mut SceneTree, NodeId)` for the target row.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when `index` was materialized and patched, or `false` when off-screen.
    pub fn patch_row<F>(&self, index: usize, patch: F) -> bool
    where
        F: FnOnce(&mut SceneTree, NodeId),
    {
        let Some(target_node) = self.row_node(index) else {
            return false;
        };
        let Self { scene, .. } = self;
        scene.with_write(
            |tree: &mut SceneTree| {
                if tree.get_node(target_node).is_some() {
                    patch(tree, target_node);
                    true
                } else {
                    false
                }
            },
            false,
        )
    }

    /// Rebuilds and swaps only the single materialized row at logical `index` in O(1), leaving all sibling rows and their layout caches untouched.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Virtual list controller reference.
    /// - `usize` - Stable logical row index to refresh.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when `index` was visible and refreshed, or `false` when off-screen.
    pub fn update_row(&self, index: usize) -> bool {
        let Self {
            rows_host,
            scene,
            ctx,
            window,
            builder,
            ..
        } = self;
        let slot_opt: Option<usize> = window.try_borrow().ok().and_then(
            |guard: Ref<'_, VecDeque<(usize, Scope, NodeId)>>| {
                let &(start_idx, _, _) = guard.front()?;
                let slot: usize = index.checked_sub(start_idx)?;
                let &(found_idx, _, _) = guard.get(slot)?;
                if found_idx == index { Some(slot) } else { None }
            },
        );
        let Some(slot) = slot_opt else {
            return false;
        };
        let built_opt: Option<(Scope, NodeId)> = untracked(|| {
            with_context(ctx.clone(), || {
                builder
                    .try_borrow_mut()
                    .ok()
                    .map(|mut build_row: RefMut<'_, VirtualRowBuilder>| {
                        Scope::run(|_: Scope| build_row(index))
                    })
            })
        });
        let Some((new_scope, new_node)) = built_opt else {
            return false;
        };
        let replaced: Option<(Scope, NodeId)> = window.try_borrow_mut().ok().and_then(
            |mut guard: RefMut<'_, VecDeque<(usize, Scope, NodeId)>>| {
                let entry: &mut (usize, Scope, NodeId) = guard.get_mut(slot)?;
                if entry.0 != index {
                    return None;
                }
                let old_entry: (usize, Scope, NodeId) =
                    replace(entry, (index, new_scope, new_node));
                Some((old_entry.1, old_entry.2))
            },
        );
        let Some((old_scope, old_node)) = replaced else {
            new_scope.dispose();
            scene.with_write(
                |tree: &mut SceneTree| {
                    tree.dispose_subtree(new_node);
                },
                (),
            );
            return false;
        };
        old_scope.dispose();
        let host_id: NodeId = *rows_host;
        scene.with_write(
            |tree: &mut SceneTree| {
                if let Some(child_node) = tree.get_node_mut(new_node) {
                    child_node.set_parent(Some(host_id));
                }
                if let Some(host_node) = tree.get_node_mut(host_id) {
                    let kids: &mut Vec<NodeId> = host_node.get_mut_children();
                    if let Some(slot_ref) = kids.get_mut(slot) {
                        *slot_ref = new_node;
                    }
                }
                tree.dispose_subtree(old_node);
                let dirty_l: u16 = DIRTY_LAYOUT | DIRTY_SUBTREE_LAYOUT;
                let dirty_c: u16 = DIRTY_CHILDREN | DIRTY_PAINT;
                tree.mark_dirty(host_id, dirty_l | dirty_c);
            },
            (),
        );
        true
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
        let tokens: ThemeTokens = self.get_tokens();
        let p: ThemePalette = tokens.get_palette();
        let g: ThemeGeometry = tokens.get_geometry();
        let Self { headers, rows, .. } = self;
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
                    for h in headers.iter() {
                        view {
                            flex_grow: 1.0
                            overflow: Overflow::Hidden
                            text {
                                font_size: 12.0
                                font_weight: FontWeight::SemiBold
                                color: p.get_muted_foreground()
                                { h.as_str() }
                            }
                        }
                    }
                }
                for row_cells in rows.iter() {
                    view {
                        width: Length::Percent(100.0)
                        height: Length::Px(g.get_border_width().max(1.0))
                        background: p.get_border()
                    }
                    row {
                        width: Length::Percent(100.0)
                        padding: Edges::symmetric(SPACE_SM, SPACE_MD)
                        gap: SPACE_SM
                        for cell_text in row_cells.iter() {
                            view {
                                flex_grow: 1.0
                                overflow: Overflow::Hidden
                                text {
                                    font_size: 13.0
                                    color: p.get_foreground()
                                    { cell_text.as_str() }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
