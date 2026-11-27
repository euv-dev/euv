use super::*;

/// Core layout solver and hit-testing operations for `LayoutEngine`.
impl LayoutEngine {
    /// Computes layout for the entire `tree` within `viewport`, skipping clean cached subtrees.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable layout engine.
    /// - `&mut SceneTree` - Retained scene tree to lay out.
    /// - `Size` - Logical viewport dimensions.
    ///
    /// # Returns
    ///
    /// - `LayoutStats` - Computed vs cached subtree counts.
    pub fn compute(&mut self, tree: &mut SceneTree, viewport: Size) -> LayoutStats {
        let root_id: NodeId = tree.get_root();
        let space: AvailableSpace = AvailableSpace::root(viewport);
        let mut stats: LayoutStats = LayoutStats::default();
        self.get_mut_children_scratch().clear();
        self.get_mut_flow_scratch().clear();
        self.get_mut_abs_scratch().clear();
        self.get_mut_flex_items_scratch().clear();
        self.get_mut_flex_lines_scratch().clear();
        self.get_mut_translate_scratch().clear();
        self.layout_node(tree, root_id, space, &mut stats);
        debug_assert!(self.get_children_scratch().is_empty());
        debug_assert!(self.get_flow_scratch().is_empty());
        debug_assert!(self.get_abs_scratch().is_empty());
        debug_assert!(self.get_flex_items_scratch().is_empty());
        debug_assert!(self.get_flex_lines_scratch().is_empty());
        debug_assert!(self.get_mut_translate_scratch().is_empty());
        self.set_last_stats(stats);
        stats
    }

    /// Measures text bounding dimensions through the bounded `TextSizeCache`, delegating misses to `TextMeasurer::measure_size`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable layout engine reference.
    /// - `&str` - UTF-8 text content to measure.
    /// - `TextStyle` - Active typography style.
    /// - `Option<f32>` - Optional maximum wrap width in logical pixels.
    ///
    /// # Returns
    ///
    /// - `Size` - Exact measured bounding dimensions.
    fn measure_text_cached(
        &mut self,
        text: &str,
        style: TextStyle,
        max_width: Option<f32>,
    ) -> Size {
        let measurer: DeterministicTextMeasurer = self.get_measurer();
        self.get_mut_text_size_cache().get_or_measure(
            text,
            style,
            max_width,
            |raw: &str, text_style: TextStyle, limit: Option<f32>| {
                measurer.measure_size(raw, text_style, limit)
            },
        )
    }

    /// Performs recursive layout on `node_id` with O(1) clean-subtree caching.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable layout engine reference.
    /// - `&mut SceneTree` - Mutable scene tree.
    /// - `NodeId` - Node handle to compute.
    /// - `AvailableSpace` - Parent space constraint and origin.
    /// - `&mut LayoutStats` - Mutable telemetry counters.
    ///
    /// # Returns
    ///
    /// - `Size` - Computed border-box dimensions of `node_id`.
    fn layout_node(
        &mut self,
        tree: &mut SceneTree,
        node_id: NodeId,
        space: AvailableSpace,
        stats: &mut LayoutStats,
    ) -> Size {
        let Some(node) = tree.get_node(node_id) else {
            return Size::default();
        };
        if !node.get_dirty().intersects(DIRTY_LAYOUT_PASS_MASK)
            && let Some(prev_space) = node.cached_constraint()
            && prev_space.can_reuse(space)
        {
            let prev_bounds: Rect = node.get_layout_box().get_bounds();
            let cached_size: Size = Size::new(prev_bounds.get_width(), prev_bounds.get_height());
            if !prev_space.same_origin(space) {
                let dx: f32 = space.get_origin_x() - prev_space.get_origin_x();
                let dy: f32 = space.get_origin_y() - prev_space.get_origin_y();
                self.translate_subtree(tree, node_id, dx, dy);
            }
            stats.set_subtrees_skipped(stats.get_subtrees_skipped() + 1);
            return cached_size;
        }
        stats.set_nodes_computed(stats.get_nodes_computed() + 1);
        let style_ref: &Style = node.style_ref();
        let Style {
            layout: layout_ref,
            paint: paint_ref,
            text: text_ref,
        } = style_ref;
        if layout_ref.get_display() == Display::None {
            let stack_base: usize = self.get_children_scratch().len();
            self.get_mut_children_scratch().push(node_id);
            while self.get_children_scratch().len() > stack_base {
                if let Some(hid) = self.get_mut_children_scratch().pop()
                    && let Some(hidden_node) = tree.get_node_mut(hid)
                {
                    hidden_node.get_mut_layout_box().set_bounds(Rect::default());
                    hidden_node
                        .get_mut_layout_box()
                        .set_content_bounds(Rect::default());
                    hidden_node.set_last_constraint(Some(space));
                    hidden_node.get_mut_dirty().remove(DIRTY_LAYOUT_PASS_MASK);
                    self.get_mut_children_scratch()
                        .extend_from_slice(hidden_node.get_children().as_slice());
                }
            }
            return Size::default();
        }
        let padding: Edges = layout_ref.get_padding();
        let border_w: f32 = paint_ref.get_border().get_width();
        let chrome_h: f32 = padding.horizontal() + border_w * 2.0;
        let chrome_v: f32 = padding.vertical() + border_w * 2.0;
        let mut resolved_w: Option<f32> = layout_ref
            .get_width()
            .resolve(space.get_width(), space.get_viewport());
        let mut resolved_h: Option<f32> = layout_ref
            .get_height()
            .resolve(space.get_height(), space.get_viewport());
        if resolved_w.is_none() && space.get_stretch_width() {
            resolved_w = Some(space.get_width());
        }
        if resolved_h.is_none() && space.get_stretch_height() {
            resolved_h = Some(space.get_height());
        }
        let min_w: f32 = layout_ref
            .get_min_width()
            .resolve(space.get_width(), space.get_viewport())
            .unwrap_or(0.0);
        let min_h: f32 = layout_ref
            .get_min_height()
            .resolve(space.get_height(), space.get_viewport())
            .unwrap_or(0.0);
        let max_w: f32 = layout_ref
            .get_max_width()
            .resolve(space.get_width(), space.get_viewport())
            .unwrap_or(f32::INFINITY);
        let max_h: f32 = layout_ref
            .get_max_height()
            .resolve(space.get_height(), space.get_viewport())
            .unwrap_or(f32::INFINITY);
        let aspect_ratio: Option<f32> = layout_ref.aspect_ratio_value();
        if let Some(ratio) = aspect_ratio
            && ratio > 0.0
        {
            if let Some(w) = resolved_w
                && resolved_h.is_none()
            {
                resolved_h = Some(w.clamp(min_w, max_w) / ratio);
            } else if let Some(h) = resolved_h
                && resolved_w.is_none()
            {
                resolved_w = Some(h.clamp(min_h, max_h) * ratio);
            }
        }
        let inner_avail_w: f32 =
            (resolved_w.unwrap_or(space.get_width()).min(max_w) - chrome_h).max(0.0);
        let inner_avail_h: f32 =
            (resolved_h.unwrap_or(space.get_height()).min(max_h) - chrome_v).max(0.0);
        let origin_x: f32 = space.get_origin_x();
        let origin_y: f32 = space.get_origin_y();
        let content_origin_x: f32 = origin_x + padding.get_left() + border_w;
        let content_origin_y: f32 = origin_y + padding.get_top() + border_w;
        let mut child_range: Option<(usize, usize)> = None;
        let leaf_content_size: Option<Size> = match node.get_kind() {
            NodeKind::Text(text) => Some(self.measure_text_cached(
                text.as_str(),
                *text_ref,
                Some(inner_avail_w.max(1.0)),
            )),
            NodeKind::Image(img) => {
                Some(Size::new(img.get_width() as f32, img.get_height() as f32))
            }
            NodeKind::Input(input_state) => {
                let sample: &str = if input_state.get_value().is_empty() {
                    input_state.get_placeholder().as_str()
                } else {
                    input_state.get_value().as_str()
                };
                let measured_size: Size =
                    self.measure_text_cached(sample, *text_ref, Some(inner_avail_w.max(1.0)));
                Some(Size::new(
                    measured_size.get_width().max(120.0),
                    measured_size.get_height().max(20.0),
                ))
            }
            NodeKind::View
            | NodeKind::ScrollView
            | NodeKind::Portal
            | NodeKind::Fragment
            | NodeKind::Canvas => {
                let node_children: &[NodeId] = node.get_children().as_slice();
                if node_children.is_empty() {
                    Some(Size::default())
                } else {
                    let start: usize = self.get_children_scratch().len();
                    self.get_mut_children_scratch()
                        .extend_from_slice(node_children);
                    let end: usize = self.get_children_scratch().len();
                    child_range = Some((start, end));
                    None
                }
            }
        };
        let content_size: Size = if let Some(size) = leaf_content_size {
            size
        } else if let Some((child_start, child_end)) = child_range {
            let frame: ContainerFrame = ContainerFrame::new(
                *layout_ref,
                Size::new(inner_avail_w, inner_avail_h),
                space.get_viewport(),
                Point::new(content_origin_x, content_origin_y),
                resolved_w.is_some(),
                resolved_h.is_some(),
            );
            let size: Size = self.layout_children(tree, child_start, child_end, frame, stats);
            self.get_mut_children_scratch().truncate(child_start);
            size
        } else {
            Size::default()
        };
        let mut final_w: f32 = resolved_w.unwrap_or(content_size.get_width() + chrome_h);
        let mut final_h: f32 = resolved_h.unwrap_or(content_size.get_height() + chrome_v);
        if let Some(ratio) = aspect_ratio
            && ratio > 0.0
        {
            if resolved_w.is_some() && resolved_h.is_none() {
                final_h = final_w / ratio;
            } else if resolved_h.is_some() && resolved_w.is_none() {
                final_w = final_h * ratio;
            }
        }
        final_w = final_w.clamp(min_w, max_w).max(0.0);
        final_h = final_h.clamp(min_h, max_h).max(0.0);
        let bounds: Rect = Rect::new(origin_x, origin_y, final_w, final_h);
        let content_bounds: Rect = Rect::new(
            content_origin_x,
            content_origin_y,
            (final_w - chrome_h).max(0.0),
            (final_h - chrome_v).max(0.0),
        );
        if let Some(node) = tree.get_node_mut(node_id) {
            node.get_mut_layout_box().set_bounds(bounds);
            node.get_mut_layout_box().set_content_bounds(content_bounds);
            node.get_mut_layout_box().set_content_size(content_size);
            node.set_last_constraint(Some(space));
            node.get_mut_dirty().remove(DIRTY_LAYOUT_PASS_MASK);
        }
        Size::new(final_w, final_h)
    }

    /// Lays out children according to Flex, Grid, or Block rules plus Absolute/Fixed overlays.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable layout engine reference.
    /// - `&mut SceneTree` - Mutable scene tree.
    /// - `usize` - Inclusive start index into `children_scratch`.
    /// - `usize` - Exclusive end index into `children_scratch`.
    /// - `ContainerFrame` - Parent layout constraints and origin.
    /// - `&mut LayoutStats` - Mutable telemetry counters.
    ///
    /// # Returns
    ///
    /// - `Size` - Intrinsic aggregate content size occupied by flow children.
    fn layout_children(
        &mut self,
        tree: &mut SceneTree,
        child_start: usize,
        child_end: usize,
        frame: ContainerFrame,
        stats: &mut LayoutStats,
    ) -> Size {
        let child_count: usize = child_end.saturating_sub(child_start);
        if child_count == 0 {
            return Size::default();
        }
        let parent_layout: LayoutStyle = frame.get_parent_layout();
        let inner_avail: Size = frame.get_inner_avail();
        let viewport: Size = frame.get_viewport();
        let content_origin: Point = frame.get_content_origin();
        let flex_dir_opt: Option<FlexDirection> = match parent_layout.get_display() {
            Display::Grid => None,
            Display::Block => Some(FlexDirection::Column),
            Display::Flex | Display::None => Some(parent_layout.get_flex_direction()),
        };
        let is_row_dir: bool = matches!(
            flex_dir_opt,
            Some(FlexDirection::Row | FlexDirection::RowReverse)
        );
        let is_reverse_dir: bool = matches!(
            flex_dir_opt,
            Some(FlexDirection::RowReverse | FlexDirection::ColumnReverse)
        );
        let parent_align: AlignItems = parent_layout.get_align_items();
        let parent_height_auto: bool = parent_layout.get_height() == Length::Auto;
        let mut single_pass_ok: bool = flex_dir_opt.is_some()
            && parent_layout.get_flex_wrap() != FlexWrap::Wrap
            && !is_reverse_dir
            && parent_layout.get_justify_content() == JustifyContent::Start;
        let flow_start: usize = self.get_flow_scratch().len();
        let abs_start: usize = self.get_abs_scratch().len();
        for idx in child_start..child_end {
            let child_id: NodeId = self.get_children_scratch()[idx];
            if let Some(child_node) = tree.get_node(child_id) {
                let Style {
                    layout: child_layout,
                    ..
                } = child_node.style_ref();
                if child_layout.get_display() == Display::None {
                    let _: Size = self.layout_node(
                        tree,
                        child_id,
                        AvailableSpace::new(0.0, 0.0, viewport, 0.0, 0.0),
                        stats,
                    );
                    continue;
                }
                match child_layout.get_position() {
                    Position::Relative => {
                        if single_pass_ok {
                            if child_layout.get_flex_grow() > 0.0 {
                                single_pass_ok = false;
                            } else {
                                let align: AlignItems = match child_layout.get_align_self() {
                                    AlignSelf::Auto => parent_align,
                                    AlignSelf::Stretch => AlignItems::Stretch,
                                    AlignSelf::Start => AlignItems::Start,
                                    AlignSelf::Center => AlignItems::Center,
                                    AlignSelf::End => AlignItems::End,
                                };
                                let non_start_align: bool =
                                    matches!(align, AlignItems::Center | AlignItems::End);
                                let auto_row_stretch: bool = align == AlignItems::Stretch
                                    && is_row_dir
                                    && child_layout.get_height() == Length::Auto
                                    && parent_height_auto;
                                if non_start_align || auto_row_stretch {
                                    single_pass_ok = false;
                                }
                            }
                        }
                        self.get_mut_flow_scratch().push(child_id);
                    }
                    Position::Absolute | Position::Fixed => {
                        self.get_mut_abs_scratch().push(child_id)
                    }
                }
            }
        }
        let flow_end: usize = self.get_flow_scratch().len();
        let abs_end: usize = self.get_abs_scratch().len();
        let flow_size: Size = if let Some(flex_dir) = flex_dir_opt {
            self.layout_flex(
                tree,
                (flow_start, flow_end),
                frame,
                flex_dir,
                single_pass_ok,
                stats,
            )
        } else {
            self.layout_grid(tree, flow_start, flow_end, frame, stats)
        };
        self.get_mut_flow_scratch().truncate(flow_start);
        for abs_idx in abs_start..abs_end {
            let abs_id: NodeId = self.get_abs_scratch()[abs_idx];
            let child_layout_opt: Option<LayoutStyle> = tree
                .get_node(abs_id)
                .map(|node: &SceneNode| node.style_ref().get_layout());
            let Some(child_layout) = child_layout_opt else {
                continue;
            };
            let is_fixed: bool = child_layout.get_position() == Position::Fixed;
            let ref_origin: Point = if is_fixed {
                Point::new(0.0, 0.0)
            } else {
                content_origin
            };
            let ref_size: Size = if is_fixed { viewport } else { inner_avail };
            let resolved_w: Option<f32> = child_layout
                .get_width()
                .resolve(ref_size.get_width(), viewport);
            let resolved_h: Option<f32> = child_layout
                .get_height()
                .resolve(ref_size.get_height(), viewport);
            let needs_intrinsic: bool = (resolved_w.is_none()
                && child_layout.left_offset().is_none()
                && child_layout.right_offset().is_some())
                || (resolved_h.is_none()
                    && child_layout.top_offset().is_none()
                    && child_layout.bottom_offset().is_some());
            let probed: Size = if needs_intrinsic {
                let probe_space: AvailableSpace = AvailableSpace::new(
                    ref_size.get_width(),
                    ref_size.get_height(),
                    viewport,
                    ref_origin.get_x(),
                    ref_origin.get_y(),
                );
                self.layout_node(tree, abs_id, probe_space, stats)
            } else {
                Size::new(ref_size.get_width(), ref_size.get_height())
            };
            let child_w: f32 = resolved_w.unwrap_or(probed.get_width());
            let child_h: f32 = resolved_h.unwrap_or(probed.get_height());
            let x: f32 = if let Some(left) = child_layout.left_offset() {
                ref_origin.get_x() + left
            } else if let Some(right) = child_layout.right_offset() {
                ref_origin.get_x() + (ref_size.get_width() - child_w - right)
            } else {
                ref_origin.get_x()
            };
            let y: f32 = if let Some(top) = child_layout.top_offset() {
                ref_origin.get_y() + top
            } else if let Some(bottom) = child_layout.bottom_offset() {
                ref_origin.get_y() + (ref_size.get_height() - child_h - bottom)
            } else {
                ref_origin.get_y()
            };
            let abs_space: AvailableSpace =
                AvailableSpace::new(ref_size.get_width(), ref_size.get_height(), viewport, x, y);
            let _: Size = self.layout_node(tree, abs_id, abs_space, stats);
        }
        self.get_mut_abs_scratch().truncate(abs_start);
        flow_size
    }

    /// Executes single-line or multi-line Flexbox layout with grow, gap, justification, and alignment.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable layout engine reference.
    /// - `&mut SceneTree` - Mutable scene tree.
    /// - `(usize, usize)` - Inclusive start and exclusive end indices into `flow_scratch`.
    /// - `ContainerFrame` - Container constraints and origin.
    /// - `FlexDirection` - Effective main-axis direction.
    /// - `bool` - Whether single-pass flex fast-path eligibility was confirmed while collecting flow children.
    /// - `&mut LayoutStats` - Mutable telemetry counters.
    ///
    /// # Returns
    ///
    /// - `Size` - Aggregate content size of all flex lines.
    fn layout_flex(
        &mut self,
        tree: &mut SceneTree,
        flow_range: (usize, usize),
        frame: ContainerFrame,
        direction: FlexDirection,
        single_pass_ok: bool,
        stats: &mut LayoutStats,
    ) -> Size {
        let (flow_start, flow_end): (usize, usize) = flow_range;
        let count_all: usize = flow_end.saturating_sub(flow_start);
        if count_all == 0 {
            return Size::default();
        }
        let parent_layout: LayoutStyle = frame.get_parent_layout();
        let inner_avail: Size = frame.get_inner_avail();
        let viewport: Size = frame.get_viewport();
        let content_origin: Point = frame.get_content_origin();
        let is_row: bool = matches!(direction, FlexDirection::Row | FlexDirection::RowReverse);
        let is_reverse: bool = matches!(
            direction,
            FlexDirection::RowReverse | FlexDirection::ColumnReverse
        );
        let main_gap: f32 = if is_row {
            parent_layout.get_column_gap()
        } else {
            parent_layout.get_row_gap()
        };
        let cross_gap: f32 = if is_row {
            parent_layout.get_row_gap()
        } else {
            parent_layout.get_column_gap()
        };
        let main_avail: f32 = if is_row {
            inner_avail.get_width()
        } else {
            inner_avail.get_height()
        };
        let cross_avail: f32 = if is_row {
            inner_avail.get_height()
        } else {
            inner_avail.get_width()
        };
        if is_reverse {
            self.get_mut_flow_scratch()[flow_start..flow_end].reverse();
        }
        let wrap_enabled: bool = parent_layout.get_flex_wrap() == FlexWrap::Wrap;
        if single_pass_ok {
            let parent_align: AlignItems = parent_layout.get_align_items();
            let mut main_cursor: f32 = 0.0;
            let mut max_cross: f32 = 0.0;
            let records_start: usize = self.get_flex_items_scratch().len();
            for offset in 0..count_all {
                let child_id: NodeId = self.get_flow_scratch()[flow_start + offset];
                let (margin, align, width_auto, height_auto, clean_cache): (
                    Edges,
                    AlignItems,
                    bool,
                    bool,
                    Option<(AvailableSpace, Size)>,
                ) = if let Some(node) = tree.get_node(child_id) {
                    let Style {
                        layout: child_layout,
                        ..
                    } = node.style_ref();
                    let a: AlignItems = match child_layout.get_align_self() {
                        AlignSelf::Auto => parent_align,
                        AlignSelf::Stretch => AlignItems::Stretch,
                        AlignSelf::Start => AlignItems::Start,
                        AlignSelf::Center => AlignItems::Center,
                        AlignSelf::End => AlignItems::End,
                    };
                    let cached: Option<(AvailableSpace, Size)> =
                        if !node.get_dirty().intersects(DIRTY_LAYOUT_PASS_MASK)
                            && let Some(prev_space) = node.cached_constraint()
                        {
                            let prev_bounds: Rect = node.get_layout_box().get_bounds();
                            Some((
                                prev_space,
                                Size::new(prev_bounds.get_width(), prev_bounds.get_height()),
                            ))
                        } else {
                            None
                        };
                    (
                        child_layout.get_margin(),
                        a,
                        child_layout.get_width() == Length::Auto,
                        child_layout.get_height() == Length::Auto,
                        cached,
                    )
                } else {
                    (Edges::default(), parent_align, true, true, None)
                };
                let (x, y): (f32, f32) = if is_row {
                    (
                        content_origin.get_x() + main_cursor + margin.get_left(),
                        content_origin.get_y() + margin.get_top(),
                    )
                } else {
                    (
                        content_origin.get_x() + margin.get_left(),
                        content_origin.get_y() + main_cursor + margin.get_top(),
                    )
                };
                let stretch_w: bool = !is_row && align == AlignItems::Stretch && width_auto;
                let stretch_h: bool = is_row && align == AlignItems::Stretch && height_auto;
                let space_w: f32 = if stretch_w {
                    (inner_avail.get_width() - margin.horizontal()).max(0.0)
                } else {
                    inner_avail.get_width()
                };
                let space_h: f32 = if stretch_h {
                    (inner_avail.get_height() - margin.vertical()).max(0.0)
                } else {
                    inner_avail.get_height()
                };
                let final_space: AvailableSpace =
                    AvailableSpace::new(space_w, space_h, viewport, x, y)
                        .with_stretch(stretch_w, stretch_h);
                let placed: Size = if let Some((prev_space, prev_size)) = clean_cache
                    && prev_space.can_reuse(final_space)
                {
                    if !prev_space.same_origin(final_space) {
                        let dx: f32 = x - prev_space.get_origin_x();
                        let dy: f32 = y - prev_space.get_origin_y();
                        self.translate_subtree(tree, child_id, dx, dy);
                    }
                    stats.set_subtrees_skipped(stats.get_subtrees_skipped() + 1);
                    prev_size
                } else {
                    self.layout_node(tree, child_id, final_space, stats)
                };
                let target_w: f32 = if stretch_w {
                    space_w.max(placed.get_width())
                } else {
                    placed.get_width()
                };
                let target_h: f32 = if stretch_h {
                    space_h.max(placed.get_height())
                } else {
                    placed.get_height()
                };
                if ((stretch_w && target_w > placed.get_width() + 0.01)
                    || (stretch_h && target_h > placed.get_height() + 0.01))
                    && let Some(node) = tree.get_node_mut(child_id)
                {
                    let mut b: Rect = node.get_layout_box().get_bounds();
                    b.set_width(target_w);
                    b.set_height(target_h);
                    node.get_mut_layout_box().set_bounds(b);
                }
                if stretch_w {
                    self.get_mut_flex_items_scratch().push(FlexItemRecord {
                        node_id: child_id,
                        size: Size::new(target_w, target_h),
                        probed_size: Size::default(),
                        probed_origin: Point::default(),
                        margin,
                        flex_grow: 0.0,
                        align_self: AlignSelf::Stretch,
                        overflow: Overflow::Visible,
                        flags: 0,
                    });
                }
                if is_row {
                    main_cursor += target_w + margin.horizontal();
                    max_cross = max_cross.max(target_h + margin.vertical());
                } else {
                    main_cursor += target_h + margin.vertical();
                    max_cross = max_cross.max(target_w + margin.horizontal());
                }
                if offset + 1 < count_all {
                    main_cursor += main_gap;
                }
            }
            let records_end: usize = self.get_flex_items_scratch().len();
            if records_end > records_start && max_cross > cross_avail + 0.5 {
                for rec_idx in records_start..records_end {
                    let rec: FlexItemRecord = self.get_flex_items_scratch()[rec_idx];
                    let widened: f32 =
                        (max_cross - rec.margin.horizontal()).max(rec.size.get_width());
                    if let Some(node) = tree.get_node_mut(rec.node_id) {
                        let mut b: Rect = node.get_layout_box().get_bounds();
                        b.set_width(widened);
                        node.get_mut_layout_box().set_bounds(b);
                    }
                }
            }
            self.get_mut_flex_items_scratch().truncate(records_start);
            return if is_row {
                Size::new(main_cursor, max_cross)
            } else {
                Size::new(max_cross, main_cursor)
            };
        }
        let items_start: usize = self.get_flex_items_scratch().len();
        let probe_template: AvailableSpace = AvailableSpace::new(
            inner_avail.get_width(),
            inner_avail.get_height(),
            viewport,
            content_origin.get_x(),
            content_origin.get_y(),
        );
        let mut total_grow: f32 = 0.0;
        let mut single_line_main: f32 = 0.0;
        let mut single_line_cross: f32 = 0.0;
        for flow_idx in flow_start..flow_end {
            let child_id: NodeId = self.get_flow_scratch()[flow_idx];
            let mut rec: FlexItemRecord = FlexItemRecord {
                node_id: child_id,
                size: Size::default(),
                probed_size: Size::default(),
                probed_origin: Point::default(),
                margin: Edges::default(),
                flex_grow: 0.0,
                align_self: AlignSelf::Auto,
                overflow: Overflow::Visible,
                flags: FLEX_ITEM_DEFAULT_LEAF_FLAGS,
            };
            let mut zero_basis_size: Option<Size> = None;
            let mut clean_probe: Option<(Size, Point)> = None;
            if let Some(node) = tree.get_node(child_id) {
                let style_ref: &Style = node.style_ref();
                let Style {
                    layout: child_layout,
                    paint: child_paint,
                    ..
                } = style_ref;
                let width_len: Length = child_layout.get_width();
                let height_len: Length = child_layout.get_height();
                let grow: f32 = child_layout.get_flex_grow();
                let width_auto: bool = width_len == Length::Auto;
                let height_auto: bool = height_len == Length::Auto;
                let is_leaf: bool = matches!(
                    node.get_kind(),
                    NodeKind::Text(_) | NodeKind::Image(_) | NodeKind::Input(_)
                ) || node.get_children().is_empty();
                let mut f: u16 = 0;
                if width_auto {
                    f |= FLEX_ITEM_WIDTH_AUTO;
                }
                if height_auto {
                    f |= FLEX_ITEM_HEIGHT_AUTO;
                }
                if matches!(width_len, Length::Percent(_)) {
                    f |= FLEX_ITEM_WIDTH_PERCENT;
                }
                if matches!(height_len, Length::Percent(_)) {
                    f |= FLEX_ITEM_HEIGHT_PERCENT;
                }
                if matches!(child_layout.get_min_width(), Length::Percent(_))
                    || matches!(child_layout.get_max_width(), Length::Percent(_))
                {
                    f |= FLEX_ITEM_MINMAX_W_PERCENT;
                }
                if matches!(child_layout.get_min_height(), Length::Percent(_))
                    || matches!(child_layout.get_max_height(), Length::Percent(_))
                {
                    f |= FLEX_ITEM_MINMAX_H_PERCENT;
                }
                if child_layout.aspect_ratio_value().is_some() {
                    f |= FLEX_ITEM_HAS_ASPECT_RATIO;
                }
                if child_layout.get_min_height() != Length::Auto
                    || child_layout.get_max_height() != Length::Auto
                {
                    f |= FLEX_ITEM_MINMAX_H_SET;
                }
                if is_leaf {
                    f |= FLEX_ITEM_IS_LEAF;
                }
                let zero_basis: bool =
                    !wrap_enabled && grow > 0.0 && if is_row { width_auto } else { height_auto };
                if zero_basis {
                    zero_basis_size = if is_row {
                        let h: f32 = height_len.resolve(cross_avail, viewport).unwrap_or(0.0);
                        Some(Size::new(0.0, h))
                    } else {
                        let w: f32 = width_len.resolve(cross_avail, viewport).unwrap_or(0.0);
                        Some(Size::new(w, 0.0))
                    };
                } else if !node.get_dirty().intersects(DIRTY_LAYOUT_PASS_MASK)
                    && let Some(prev) = node.cached_constraint()
                    && prev.can_reuse(probe_template)
                {
                    let b: Rect = node.get_layout_box().get_bounds();
                    let probed_h: f32 = if is_leaf
                        && height_auto
                        && (f & (FLEX_ITEM_MINMAX_H_SET | FLEX_ITEM_HAS_ASPECT_RATIO)) == 0
                    {
                        let pad: Edges = child_layout.get_padding();
                        let bw: f32 = child_paint.get_border().get_width();
                        let chrome_v: f32 = pad.vertical() + bw * 2.0;
                        (node.get_layout_box().get_content_size().get_height() + chrome_v).max(0.0)
                    } else {
                        b.get_height()
                    };
                    clean_probe = Some((
                        Size::new(b.get_width(), probed_h),
                        Point::new(prev.get_origin_x(), prev.get_origin_y()),
                    ));
                }
                rec.margin = child_layout.get_margin();
                rec.flex_grow = grow;
                rec.align_self = child_layout.get_align_self();
                rec.overflow = child_layout.get_overflow();
                rec.flags = f;
            }
            total_grow += rec.flex_grow;
            let (child_size, probed_origin): (Size, Point) = if let Some(zb) = zero_basis_size {
                (zb, content_origin)
            } else if let Some((cached_size, cached_origin)) = clean_probe {
                rec.flags |= FLEX_ITEM_PROBED;
                stats.set_subtrees_skipped(stats.get_subtrees_skipped() + 1);
                (cached_size, cached_origin)
            } else {
                let measured: Size = self.layout_node(tree, child_id, probe_template, stats);
                rec.flags |= FLEX_ITEM_PROBED;
                (measured, content_origin)
            };
            if !wrap_enabled {
                if is_row {
                    single_line_main += child_size.get_width() + rec.margin.horizontal();
                    single_line_cross =
                        single_line_cross.max(child_size.get_height() + rec.margin.vertical());
                } else {
                    single_line_main += child_size.get_height() + rec.margin.vertical();
                    single_line_cross =
                        single_line_cross.max(child_size.get_width() + rec.margin.horizontal());
                }
            }
            rec.size = child_size;
            rec.probed_size = child_size;
            rec.probed_origin = probed_origin;
            self.get_mut_flex_items_scratch().push(rec);
        }
        let items_end: usize = self.get_flex_items_scratch().len();
        if !wrap_enabled {
            let line_ctx: FlexLineContext = FlexLineContext {
                frame,
                is_row,
                wrap_enabled: false,
                main_gap,
                main_avail,
                cross_avail,
                total_grow,
                line_count: 1,
            };
            let line_size: Size = self.layout_flex_line(
                tree,
                (items_start, items_end),
                0.0,
                Some((single_line_main, single_line_cross)),
                line_ctx,
                stats,
            );
            self.get_mut_flex_items_scratch().truncate(items_start);
            return line_size;
        }
        let lines_start: usize = self.get_flex_lines_scratch().len();
        let mut cur_line_start: usize = items_start;
        let mut line_main: f32 = 0.0;
        for item_idx in items_start..items_end {
            let item: FlexItemRecord = self.get_flex_items_scratch()[item_idx];
            let item_main: f32 = if is_row {
                item.size.get_width() + item.margin.horizontal()
            } else {
                item.size.get_height() + item.margin.vertical()
            };
            let line_empty: bool = item_idx == cur_line_start;
            let needed: f32 = if line_empty {
                item_main
            } else {
                line_main + main_gap + item_main
            };
            if !line_empty && needed > main_avail && main_avail > 0.0 {
                self.get_mut_flex_lines_scratch()
                    .push((cur_line_start, item_idx));
                cur_line_start = item_idx;
                line_main = item_main;
            } else {
                line_main = needed;
            }
        }
        if cur_line_start < items_end {
            self.get_mut_flex_lines_scratch()
                .push((cur_line_start, items_end));
        }
        let lines_end: usize = self.get_flex_lines_scratch().len();
        let mut cross_cursor: f32 = 0.0;
        let mut max_main_extent: f32 = 0.0;
        let line_count: usize = lines_end.saturating_sub(lines_start);
        let line_ctx: FlexLineContext = FlexLineContext {
            frame,
            is_row,
            wrap_enabled: true,
            main_gap,
            main_avail,
            cross_avail,
            total_grow,
            line_count,
        };
        for line_offset in 0..line_count {
            let line_range: (usize, usize) =
                self.get_flex_lines_scratch()[lines_start + line_offset];
            let line_size: Size =
                self.layout_flex_line(tree, line_range, cross_cursor, None, line_ctx, stats);
            if is_row {
                max_main_extent = max_main_extent.max(line_size.get_width());
                cross_cursor += line_size.get_height();
            } else {
                max_main_extent = max_main_extent.max(line_size.get_height());
                cross_cursor += line_size.get_width();
            }
            if line_offset + 1 < line_count {
                cross_cursor += cross_gap;
            }
        }
        self.get_mut_flex_lines_scratch().truncate(lines_start);
        self.get_mut_flex_items_scratch().truncate(items_start);
        if is_row {
            Size::new(max_main_extent, cross_cursor)
        } else {
            Size::new(cross_cursor, max_main_extent)
        }
    }

    /// Places a single contiguous flex line slice `[line_start..line_end)` and returns its occupied `(width, height)`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable layout engine reference.
    /// - `&mut SceneTree` - Mutable scene tree.
    /// - `(usize, usize)` - Inclusive start and exclusive end indices into `flex_items_scratch`.
    /// - `f32` - Cross-axis cursor offset where this line begins.
    /// - `Option<(f32, f32)>` - Optional pre-accumulated `(items_main_sum, line_cross)` for unwrapped single-line passes.
    /// - `FlexLineContext` - Enclosing container axis and gap parameters.
    /// - `&mut LayoutStats` - Mutable telemetry counters.
    ///
    /// # Returns
    ///
    /// - `Size` - Occupied `(width, height)` extent of this flex line.
    #[inline(always)]
    fn layout_flex_line(
        &mut self,
        tree: &mut SceneTree,
        line_range: (usize, usize),
        cross_cursor: f32,
        precomputed: Option<(f32, f32)>,
        ctx: FlexLineContext,
        stats: &mut LayoutStats,
    ) -> Size {
        let (line_start, line_end): (usize, usize) = line_range;
        let frame: ContainerFrame = ctx.frame;
        let is_row: bool = ctx.is_row;
        let wrap_enabled: bool = ctx.wrap_enabled;
        let main_gap: f32 = ctx.main_gap;
        let main_avail: f32 = ctx.main_avail;
        let cross_avail: f32 = ctx.cross_avail;
        let total_grow: f32 = ctx.total_grow;
        let line_count: usize = ctx.line_count;
        let parent_layout: LayoutStyle = frame.get_parent_layout();
        let inner_avail: Size = frame.get_inner_avail();
        let viewport: Size = frame.get_viewport();
        let content_origin: Point = frame.get_content_origin();
        let count: usize = line_end.saturating_sub(line_start);
        let gaps_total: f32 = if count > 1 {
            main_gap * ((count - 1) as f32)
        } else {
            0.0
        };
        let has_definite_or_grow: bool = frame.get_definite_width() || total_grow > 0.0;
        let effective_main_avail: f32 = if is_row {
            if has_definite_or_grow || wrap_enabled {
                main_avail
            } else {
                parent_layout
                    .get_min_width()
                    .resolve(viewport.get_width(), viewport)
                    .map(|mw: f32| (mw - parent_layout.get_padding().horizontal()).max(0.0))
                    .unwrap_or(0.0)
            }
        } else if frame.get_definite_height() {
            main_avail
        } else {
            parent_layout
                .get_min_height()
                .resolve(viewport.get_height(), viewport)
                .map(|mh: f32| (mh - parent_layout.get_padding().vertical()).max(0.0))
                .unwrap_or(0.0)
        };
        let (final_used_main, line_cross): (f32, f32) = if let Some((items_main, pre_cross)) =
            precomputed
            && total_grow == 0.0
        {
            (items_main + gaps_total, pre_cross)
        } else {
            let used_main: f32 = if let Some((items_main, _)) = precomputed {
                items_main + gaps_total
            } else {
                let mut acc: f32 = gaps_total;
                for item_idx in line_start..line_end {
                    let item: FlexItemRecord = self.get_flex_items_scratch()[item_idx];
                    let sz: f32 = if is_row {
                        item.size.get_width() + item.margin.horizontal()
                    } else {
                        item.size.get_height() + item.margin.vertical()
                    };
                    acc += sz;
                }
                acc
            };
            let free_main: f32 = (effective_main_avail - used_main).max(0.0);
            if total_grow > 0.0 && free_main > 0.0 && !wrap_enabled {
                for item_idx in line_start..line_end {
                    let item: &mut FlexItemRecord =
                        &mut self.get_mut_flex_items_scratch()[item_idx];
                    let grow: f32 = item.flex_grow;
                    if grow > 0.0 {
                        let extra: f32 = free_main * (grow / total_grow);
                        if is_row {
                            item.size.set_width(item.size.get_width() + extra);
                        } else {
                            item.size.set_height(item.size.get_height() + extra);
                        }
                    }
                }
            }
            if !wrap_enabled && total_grow > 0.0 {
                for item_idx in line_start..line_end {
                    let item: FlexItemRecord = self.get_flex_items_scratch()[item_idx];
                    let auto_cross: bool = item.flex_grow > 0.0
                        && if is_row {
                            (item.flags & FLEX_ITEM_HEIGHT_AUTO) != 0
                        } else {
                            (item.flags & FLEX_ITEM_WIDTH_AUTO) != 0
                        };
                    if auto_cross {
                        let probe_w: f32 = if is_row {
                            item.size.get_width()
                        } else {
                            cross_avail
                        };
                        let probe_h: f32 = if is_row {
                            cross_avail
                        } else {
                            item.size.get_height()
                        };
                        let probe_sp: AvailableSpace = AvailableSpace::new(
                            probe_w,
                            probe_h,
                            viewport,
                            content_origin.get_x(),
                            content_origin.get_y(),
                        );
                        let measured: Size = self.layout_node(tree, item.node_id, probe_sp, stats);
                        let slot: &mut FlexItemRecord =
                            &mut self.get_mut_flex_items_scratch()[item_idx];
                        if is_row {
                            slot.size.set_height(measured.get_height());
                        } else {
                            slot.size.set_width(measured.get_width());
                        }
                        slot.flags &= !FLEX_ITEM_PROBED;
                    }
                }
            }
            let mut sum_main: f32 = gaps_total;
            let mut max_cross: f32 = 0.0;
            for item_idx in line_start..line_end {
                let item: FlexItemRecord = self.get_flex_items_scratch()[item_idx];
                if is_row {
                    sum_main += item.size.get_width() + item.margin.horizontal();
                    max_cross = max_cross.max(item.size.get_height() + item.margin.vertical());
                } else {
                    sum_main += item.size.get_height() + item.margin.vertical();
                    max_cross = max_cross.max(item.size.get_width() + item.margin.horizontal());
                }
            }
            (sum_main, max_cross)
        };
        let rem: f32 = (effective_main_avail - final_used_main).max(0.0);
        let (mut main_cursor, extra_step): (f32, f32) = match parent_layout.get_justify_content() {
            JustifyContent::Start => (0.0, 0.0),
            JustifyContent::Center => (rem * 0.5, 0.0),
            JustifyContent::End => (rem, 0.0),
            JustifyContent::SpaceBetween => {
                if count > 1 {
                    (0.0, rem / ((count - 1) as f32))
                } else {
                    (0.0, 0.0)
                }
            }
            JustifyContent::SpaceAround => {
                let slot: f32 = rem / (count.max(1) as f32);
                (slot * 0.5, slot)
            }
            JustifyContent::SpaceEvenly => {
                let slot: f32 = rem / ((count + 1) as f32);
                (slot, slot)
            }
        };
        let definite_cross: bool = !is_row || frame.get_definite_height();
        let effective_line_cross: f32 = if line_count == 1 && !wrap_enabled && definite_cross {
            line_cross.max(cross_avail)
        } else {
            line_cross
        };
        let mut actual_line_cross: f32 = line_cross;
        let parent_align: AlignItems = parent_layout.get_align_items();
        for idx in 0..count {
            let rec: FlexItemRecord = self.get_flex_items_scratch()[line_start + idx];
            let child_id: NodeId = rec.node_id;
            let child_size: Size = rec.size;
            let margin: Edges = rec.margin;
            let width_auto: bool = (rec.flags & FLEX_ITEM_WIDTH_AUTO) != 0;
            let height_auto: bool = (rec.flags & FLEX_ITEM_HEIGHT_AUTO) != 0;
            let is_leaf: bool = (rec.flags & FLEX_ITEM_IS_LEAF) != 0;
            let item_cross: f32 = if is_row {
                child_size.get_height()
            } else {
                child_size.get_width()
            };
            let align: AlignItems = match rec.align_self {
                AlignSelf::Auto => parent_align,
                AlignSelf::Stretch => AlignItems::Stretch,
                AlignSelf::Start => AlignItems::Start,
                AlignSelf::Center => AlignItems::Center,
                AlignSelf::End => AlignItems::End,
            };
            let avail_item_cross: f32 = (effective_line_cross
                - if is_row {
                    margin.vertical()
                } else {
                    margin.horizontal()
                })
            .max(0.0);
            let cross_offset: f32 = match align {
                AlignItems::Start | AlignItems::Stretch => 0.0,
                AlignItems::Center => ((avail_item_cross - item_cross) * 0.5).max(0.0),
                AlignItems::End => (avail_item_cross - item_cross).max(0.0),
            };
            let auto_cross_grow: bool = !wrap_enabled
                && rec.flex_grow > 0.0
                && if is_row { height_auto } else { width_auto };
            let target_w: f32 = if !is_row && align == AlignItems::Stretch && width_auto {
                avail_item_cross.max(child_size.get_width())
            } else {
                child_size.get_width()
            };
            let target_h: f32 = if is_row && align == AlignItems::Stretch && height_auto {
                avail_item_cross.max(child_size.get_height())
            } else {
                child_size.get_height()
            };
            let (x, y): (f32, f32) = if is_row {
                (
                    content_origin.get_x() + main_cursor + margin.get_left(),
                    content_origin.get_y() + cross_cursor + cross_offset + margin.get_top(),
                )
            } else {
                (
                    content_origin.get_x() + cross_cursor + cross_offset + margin.get_left(),
                    content_origin.get_y() + main_cursor + margin.get_top(),
                )
            };
            let space_w: f32 = if (rec.flags & FLEX_ITEM_WIDTH_PERCENT) != 0 {
                inner_avail.get_width()
            } else if auto_cross_grow && !is_row {
                cross_avail
            } else {
                target_w
            };
            let space_h: f32 = if (rec.flags & FLEX_ITEM_HEIGHT_PERCENT) != 0 {
                inner_avail.get_height()
            } else if auto_cross_grow && is_row {
                cross_avail
            } else {
                target_h
            };
            let stretch_child_w: bool = !(auto_cross_grow && !is_row)
                && ((!is_row && align == AlignItems::Stretch && width_auto)
                    || (is_row && rec.flex_grow > 0.0 && width_auto));
            let stretch_child_h: bool = !(auto_cross_grow && is_row)
                && ((is_row && align == AlignItems::Stretch && height_auto)
                    || (!is_row && rec.flex_grow > 0.0 && height_auto));
            let same_w: bool = (target_w - rec.probed_size.get_width()).abs() <= 0.001;
            let same_h: bool = (target_h - rec.probed_size.get_height()).abs() <= 0.001;
            let width_fixed_or_leaf: bool = is_leaf || !width_auto;
            let width_no_pct_bounds: bool = (rec.flags & FLEX_ITEM_MINMAX_W_PERCENT) == 0;
            let width_invariant: bool = (space_w - inner_avail.get_width()).abs() <= 0.01
                || (width_fixed_or_leaf && width_no_pct_bounds);
            let height_fixed_or_leaf: bool = is_leaf || !height_auto;
            let height_no_pct_bounds: bool = (rec.flags & FLEX_ITEM_MINMAX_H_PERCENT) == 0;
            let height_invariant: bool = (space_h - inner_avail.get_height()).abs() <= 0.01
                || (height_fixed_or_leaf && height_no_pct_bounds);
            let probed_ok: bool = (rec.flags & FLEX_ITEM_PROBED) != 0
                && !auto_cross_grow
                && (rec.flags & FLEX_ITEM_HAS_ASPECT_RATIO) == 0
                && width_invariant
                && height_invariant;
            let can_translate_exact: bool = probed_ok
                && same_w
                && same_h
                && (is_leaf || (!stretch_child_w && !stretch_child_h));
            let can_stretch_leaf_h: bool = probed_ok
                && is_leaf
                && is_row
                && same_w
                && !stretch_child_w
                && stretch_child_h
                && height_auto
                && (rec.flags & FLEX_ITEM_MINMAX_H_SET) == 0;
            if can_translate_exact {
                let dx: f32 = x - rec.probed_origin.get_x();
                let dy: f32 = y - rec.probed_origin.get_y();
                if dx.abs() > 0.001 || dy.abs() > 0.001 {
                    self.translate_subtree(tree, child_id, dx, dy);
                }
            } else if can_stretch_leaf_h {
                if let Some(node) = tree.get_node_mut(child_id) {
                    let cur_bounds: Rect = node.get_layout_box().get_bounds();
                    let same_box: bool = (cur_bounds.get_x() - x).abs() <= 0.001
                        && (cur_bounds.get_y() - y).abs() <= 0.001
                        && (cur_bounds.get_width() - target_w).abs() <= 0.001
                        && (cur_bounds.get_height() - target_h).abs() <= 0.001;
                    if !same_box {
                        let Style {
                            layout: child_layout,
                            paint: child_paint,
                            ..
                        } = node.style_ref();
                        let pad: Edges = child_layout.get_padding();
                        let bw: f32 = child_paint.get_border().get_width();
                        let chrome_h: f32 = pad.horizontal() + bw * 2.0;
                        let chrome_v: f32 = pad.vertical() + bw * 2.0;
                        let new_bounds: Rect = Rect::new(x, y, target_w, target_h);
                        let new_content: Rect = Rect::new(
                            x + pad.get_left() + bw,
                            y + pad.get_top() + bw,
                            (target_w - chrome_h).max(0.0),
                            (target_h - chrome_v).max(0.0),
                        );
                        node.get_mut_layout_box().set_bounds(new_bounds);
                        node.get_mut_layout_box().set_content_bounds(new_content);
                        if let Some(mut c) = node.cached_constraint() {
                            c.set_origin_x(x);
                            c.set_origin_y(y);
                            node.set_last_constraint(Some(c));
                        }
                    }
                }
            } else {
                let final_space: AvailableSpace =
                    AvailableSpace::new(space_w, space_h, viewport, x, y)
                        .with_stretch(stretch_child_w, stretch_child_h);
                let placed: Size = self.layout_node(tree, child_id, final_space, stats);
                if let Some(node) = tree.get_node_mut(child_id) {
                    let mut b: Rect = node.get_layout_box().get_bounds();
                    let is_clipped: bool =
                        matches!(rec.overflow, Overflow::Hidden | Overflow::Scroll);
                    if auto_cross_grow {
                        if is_row {
                            b.set_width(target_w);
                            b.set_height(placed.get_height());
                            actual_line_cross =
                                actual_line_cross.max(placed.get_height() + margin.vertical());
                        } else {
                            b.set_width(placed.get_width());
                            b.set_height(target_h);
                            actual_line_cross =
                                actual_line_cross.max(placed.get_width() + margin.horizontal());
                        }
                    } else if is_clipped || rec.flex_grow > 0.0 {
                        b.set_width(target_w);
                        b.set_height(target_h);
                    } else {
                        b.set_width(target_w.max(placed.get_width()));
                        b.set_height(target_h.max(placed.get_height()));
                    }
                    node.get_mut_layout_box().set_bounds(b);
                }
            }
            let advance: f32 = if is_row {
                target_w + margin.horizontal()
            } else {
                target_h + margin.vertical()
            };
            main_cursor += advance;
            if idx + 1 < count {
                main_cursor += main_gap + extra_step;
            }
        }
        if is_row {
            Size::new(final_used_main, actual_line_cross)
        } else {
            Size::new(actual_line_cross, final_used_main)
        }
    }

    /// Executes uniform-column CSS Grid layout with row and column gaps.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable layout engine reference.
    /// - `&mut SceneTree` - Mutable scene tree.
    /// - `usize` - Inclusive start index into `flow_scratch`.
    /// - `usize` - Exclusive end index into `flow_scratch`.
    /// - `ContainerFrame` - Grid container constraints and origin.
    /// - `&mut LayoutStats` - Mutable telemetry counters.
    ///
    /// # Returns
    ///
    /// - `Size` - Aggregate grid content dimensions.
    fn layout_grid(
        &mut self,
        tree: &mut SceneTree,
        flow_start: usize,
        flow_end: usize,
        frame: ContainerFrame,
        stats: &mut LayoutStats,
    ) -> Size {
        if flow_start >= flow_end {
            return Size::default();
        }
        let parent_layout: LayoutStyle = frame.get_parent_layout();
        let inner_avail: Size = frame.get_inner_avail();
        let viewport: Size = frame.get_viewport();
        let content_origin: Point = frame.get_content_origin();
        let cols: usize = (parent_layout.get_grid_columns() as usize).max(1);
        let col_gap: f32 = parent_layout.get_column_gap();
        let row_gap: f32 = parent_layout.get_row_gap();
        let total_col_gaps: f32 = col_gap * ((cols.saturating_sub(1)) as f32);
        let col_w: f32 = ((inner_avail.get_width() - total_col_gaps) / (cols as f32)).max(0.0);
        let col_step: f32 = col_w + col_gap;
        let origin_x: f32 = content_origin.get_x();
        let origin_y: f32 = content_origin.get_y();
        let inner_h: f32 = inner_avail.get_height();
        let mut y_cursor: f32 = 0.0;
        let mut idx: usize = flow_start;
        while idx < flow_end {
            let row_end: usize = (idx + cols).min(flow_end);
            let mut row_h: f32 = 0.0;
            let mut min_row_h: f32 = f32::INFINITY;
            let y: f32 = origin_y + y_cursor;
            let mut x: f32 = origin_x;
            for pos in idx..row_end {
                let child_id: NodeId = self.get_flow_scratch()[pos];
                let (auto_w, clean_cache): (bool, Option<(AvailableSpace, Size)>) =
                    if let Some(n) = tree.get_node(child_id) {
                        let Style { layout, .. } = n.style_ref();
                        let is_auto_w: bool = layout.get_width() == Length::Auto;
                        let cached: Option<(AvailableSpace, Size)> =
                            if !n.get_dirty().intersects(DIRTY_LAYOUT_PASS_MASK)
                                && let Some(prev_space) = n.cached_constraint()
                            {
                                let prev_bounds: Rect = n.get_layout_box().get_bounds();
                                Some((
                                    prev_space,
                                    Size::new(prev_bounds.get_width(), prev_bounds.get_height()),
                                ))
                            } else {
                                None
                            };
                        (is_auto_w, cached)
                    } else {
                        (false, None)
                    };
                let cell_space: AvailableSpace =
                    AvailableSpace::new(col_w, inner_h, viewport, x, y).with_stretch(auto_w, false);
                let child_size: Size = if let Some((prev_space, prev_size)) = clean_cache
                    && prev_space.can_reuse(cell_space)
                {
                    if !prev_space.same_origin(cell_space) {
                        let dx: f32 = x - prev_space.get_origin_x();
                        let dy: f32 = y - prev_space.get_origin_y();
                        self.translate_subtree(tree, child_id, dx, dy);
                    }
                    stats.set_subtrees_skipped(stats.get_subtrees_skipped() + 1);
                    prev_size
                } else {
                    self.layout_node(tree, child_id, cell_space, stats)
                };
                if (child_size.get_width() - col_w).abs() > 0.001
                    && let Some(node) = tree.get_node_mut(child_id)
                {
                    let mut b: Rect = node.get_layout_box().get_bounds();
                    b.set_width(col_w);
                    node.get_mut_layout_box().set_bounds(b);
                }
                let ch: f32 = child_size.get_height();
                row_h = row_h.max(ch);
                min_row_h = min_row_h.min(ch);
                x += col_step;
            }
            if row_end - idx > 1 && min_row_h < row_h {
                for pos in idx..row_end {
                    let child_id: NodeId = self.get_flow_scratch()[pos];
                    if let Some(node) = tree.get_node_mut(child_id) {
                        let mut b: Rect = node.get_layout_box().get_bounds();
                        if b.get_height() < row_h {
                            b.set_height(row_h);
                            node.get_mut_layout_box().set_bounds(b);
                        }
                    }
                }
            }
            y_cursor += row_h;
            if row_end < flow_end {
                y_cursor += row_gap;
            }
            idx = row_end;
        }
        Size::new(inner_avail.get_width(), y_cursor)
    }

    /// Translates the cached layout geometry of `root_id` and all non-fixed descendants by `(dx, dy)` in O(subtree) without re-running layout or text measurement.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable layout engine reference.
    /// - `&mut SceneTree` - Mutable retained scene tree.
    /// - `NodeId` - Root node of the clean subtree being shifted.
    /// - `f32` - Horizontal translation delta in logical pixels.
    /// - `f32` - Vertical translation delta in logical pixels.
    fn translate_subtree(&mut self, tree: &mut SceneTree, root_id: NodeId, dx: f32, dy: f32) {
        if dx == 0.0 && dy == 0.0 {
            return;
        }
        let Some(root_node) = tree.get_node_mut(root_id) else {
            return;
        };
        let Style { layout, .. } = root_node.style_ref();
        let root_none_disp: bool = layout.get_display() == Display::None;
        if let Some(mut c) = root_node.cached_constraint() {
            c.set_origin_x(c.get_origin_x() + dx);
            c.set_origin_y(c.get_origin_y() + dy);
            root_node.set_last_constraint(Some(c));
        }
        if root_none_disp {
            return;
        }
        let mut root_bounds: Rect = root_node.get_layout_box().get_bounds();
        root_bounds.set_x(root_bounds.get_x() + dx);
        root_bounds.set_y(root_bounds.get_y() + dy);
        root_node.get_mut_layout_box().set_bounds(root_bounds);
        let mut root_content: Rect = root_node.get_layout_box().get_content_bounds();
        root_content.set_x(root_content.get_x() + dx);
        root_content.set_y(root_content.get_y() + dy);
        root_node
            .get_mut_layout_box()
            .set_content_bounds(root_content);
        let root_children: &[NodeId] = root_node.get_children().as_slice();
        if root_children.is_empty() {
            return;
        }
        let stack: &mut Vec<NodeId> = self.get_mut_translate_scratch();
        stack.clear();
        stack.extend_from_slice(root_children);
        while let Some(nid) = stack.pop() {
            let Some(node) = tree.get_node_mut(nid) else {
                continue;
            };
            let Style { layout, .. } = node.style_ref();
            let is_fixed_pos: bool = layout.get_position() == Position::Fixed;
            let is_none_disp: bool = layout.get_display() == Display::None;
            if is_fixed_pos {
                continue;
            }
            if let Some(mut c) = node.cached_constraint() {
                c.set_origin_x(c.get_origin_x() + dx);
                c.set_origin_y(c.get_origin_y() + dy);
                node.set_last_constraint(Some(c));
            }
            if is_none_disp {
                continue;
            }
            let mut bounds: Rect = node.get_layout_box().get_bounds();
            bounds.set_x(bounds.get_x() + dx);
            bounds.set_y(bounds.get_y() + dy);
            node.get_mut_layout_box().set_bounds(bounds);
            let mut content: Rect = node.get_layout_box().get_content_bounds();
            content.set_x(content.get_x() + dx);
            content.set_y(content.get_y() + dy);
            node.get_mut_layout_box().set_content_bounds(content);
            let children_slice: &[NodeId] = node.get_children().as_slice();
            if !children_slice.is_empty() {
                stack.extend_from_slice(children_slice);
            }
        }
    }
}
