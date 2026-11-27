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
        self.layout_node(tree, root_id, space, &mut stats);
        self.set_last_stats(stats);
        stats
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
        {
            let same_size: bool = (prev_space.get_width() - space.get_width()).abs() <= 0.01
                && (prev_space.get_height() - space.get_height()).abs() <= 0.01
                && (prev_space.get_viewport().get_width() - space.get_viewport().get_width()).abs()
                    <= 0.01
                && (prev_space.get_viewport().get_height() - space.get_viewport().get_height())
                    .abs()
                    <= 0.01
                && prev_space.get_stretch_width() == space.get_stretch_width()
                && prev_space.get_stretch_height() == space.get_stretch_height();
            if same_size {
                let dx: f32 = space.get_origin_x() - prev_space.get_origin_x();
                let dy: f32 = space.get_origin_y() - prev_space.get_origin_y();
                let prev_bounds: Rect = node.get_layout_box().get_bounds();
                if dx.abs() > 0.001 || dy.abs() > 0.001 {
                    self.translate_subtree(tree, node_id, dx, dy);
                }
                stats.set_subtrees_skipped(stats.get_subtrees_skipped() + 1);
                return Size::new(prev_bounds.get_width(), prev_bounds.get_height());
            }
        }
        stats.set_nodes_computed(stats.get_nodes_computed() + 1);
        let style_ref: &Style = node.style_ref();
        let Style {
            paint: paint_ref,
            text: text_ref,
            ..
        } = style_ref;
        let layout: LayoutStyle = style_ref.get_layout();
        if layout.get_display() == Display::None {
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
        let padding: Edges = layout.get_padding();
        let border_w: f32 = paint_ref.get_border().get_width();
        let chrome_h: f32 = padding.horizontal() + border_w * 2.0;
        let chrome_v: f32 = padding.vertical() + border_w * 2.0;
        let mut resolved_w: Option<f32> = layout
            .get_width()
            .resolve(space.get_width(), space.get_viewport());
        let mut resolved_h: Option<f32> = layout
            .get_height()
            .resolve(space.get_height(), space.get_viewport());
        if resolved_w.is_none() && space.get_stretch_width() {
            resolved_w = Some(space.get_width());
        }
        if resolved_h.is_none() && space.get_stretch_height() {
            resolved_h = Some(space.get_height());
        }
        let min_w: f32 = layout
            .get_min_width()
            .resolve(space.get_width(), space.get_viewport())
            .unwrap_or(0.0);
        let min_h: f32 = layout
            .get_min_height()
            .resolve(space.get_height(), space.get_viewport())
            .unwrap_or(0.0);
        let max_w: f32 = layout
            .get_max_width()
            .resolve(space.get_width(), space.get_viewport())
            .unwrap_or(f32::INFINITY);
        let max_h: f32 = layout
            .get_max_height()
            .resolve(space.get_height(), space.get_viewport())
            .unwrap_or(f32::INFINITY);
        if let Some(ratio) = layout.aspect_ratio_value()
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
            NodeKind::Text(text) => {
                let measured: TextLayoutResult = self.get_measurer().measure(
                    text.as_str(),
                    *text_ref,
                    Some(inner_avail_w.max(1.0)),
                );
                Some(measured.get_size())
            }
            NodeKind::Image(img) => {
                Some(Size::new(img.get_width() as f32, img.get_height() as f32))
            }
            NodeKind::Input(input_state) => {
                let sample: &str = if input_state.get_value().is_empty() {
                    input_state.get_placeholder().as_str()
                } else {
                    input_state.get_value().as_str()
                };
                let measured: TextLayoutResult =
                    self.get_measurer()
                        .measure(sample, *text_ref, Some(inner_avail_w.max(1.0)));
                Some(Size::new(
                    measured.get_size().get_width().max(120.0),
                    measured.get_size().get_height().max(20.0),
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
                layout,
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
        if let Some(ratio) = layout.aspect_ratio_value()
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
        let mut flow_children: Vec<NodeId> = Vec::with_capacity(child_count);
        let mut abs_children: Vec<NodeId> = Vec::new();
        for idx in child_start..child_end {
            let child_id: NodeId = self.get_children_scratch()[idx];
            if let Some(child_node) = tree.get_node(child_id) {
                let child_layout: LayoutStyle = child_node.style_ref().get_layout();
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
                    Position::Relative => flow_children.push(child_id),
                    Position::Absolute | Position::Fixed => abs_children.push(child_id),
                }
            }
        }
        let flow_size: Size = match parent_layout.get_display() {
            Display::Grid => self.layout_grid(tree, &flow_children, frame, stats),
            Display::Block => {
                self.layout_flex(tree, &flow_children, frame, FlexDirection::Column, stats)
            }
            Display::Flex | Display::None => self.layout_flex(
                tree,
                &flow_children,
                frame,
                parent_layout.get_flex_direction(),
                stats,
            ),
        };
        for abs_id in abs_children.into_iter() {
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
        flow_size
    }

    /// Executes single-line or multi-line Flexbox layout with grow, gap, justification, and alignment.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Layout engine reference.
    /// - `&mut SceneTree` - Mutable scene tree.
    /// - `&[NodeId]` - Normal-flow children.
    /// - `ContainerFrame` - Container constraints and origin.
    /// - `FlexDirection` - Effective main-axis direction.
    /// - `&mut LayoutStats` - Mutable telemetry counters.
    ///
    /// # Returns
    ///
    /// - `Size` - Aggregate content size of all flex lines.
    fn layout_flex(
        &mut self,
        tree: &mut SceneTree,
        children: &[NodeId],
        frame: ContainerFrame,
        direction: FlexDirection,
        stats: &mut LayoutStats,
    ) -> Size {
        if children.is_empty() {
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
        let mut measured_sizes: Vec<(NodeId, Size, LayoutStyle)> =
            Vec::with_capacity(children.len());
        let mut ordered: Vec<NodeId> = children.to_vec();
        if is_reverse {
            ordered.reverse();
        }
        let wrap_enabled: bool = parent_layout.get_flex_wrap() == FlexWrap::Wrap;
        if !wrap_enabled
            && !is_reverse
            && parent_layout.get_justify_content() == JustifyContent::Start
        {
            let mut single_pass_ok: bool = true;
            let mut has_stretched_auto: bool = false;
            for child_id in ordered.iter().copied() {
                let child_layout: LayoutStyle = tree
                    .get_node(child_id)
                    .map(|node: &SceneNode| node.style_ref().get_layout())
                    .unwrap_or_default();
                if child_layout.get_flex_grow() > 0.0 {
                    single_pass_ok = false;
                    break;
                }
                let align: AlignItems = match child_layout.get_align_self() {
                    AlignSelf::Auto => parent_layout.get_align_items(),
                    AlignSelf::Stretch => AlignItems::Stretch,
                    AlignSelf::Start => AlignItems::Start,
                    AlignSelf::Center => AlignItems::Center,
                    AlignSelf::End => AlignItems::End,
                };
                if matches!(align, AlignItems::Center | AlignItems::End) {
                    single_pass_ok = false;
                    break;
                }
                if align == AlignItems::Stretch {
                    if is_row {
                        if child_layout.get_height() == Length::Auto
                            && parent_layout.get_height() == Length::Auto
                        {
                            single_pass_ok = false;
                            break;
                        }
                    } else if child_layout.get_width() == Length::Auto {
                        has_stretched_auto = true;
                    }
                }
            }
            if single_pass_ok {
                let mut main_cursor: f32 = 0.0;
                let mut max_cross: f32 = 0.0;
                let count: usize = ordered.len();
                let mut child_records: Vec<(NodeId, LayoutStyle, Size)> = if has_stretched_auto {
                    Vec::with_capacity(count)
                } else {
                    Vec::new()
                };
                for (idx, child_id) in ordered.iter().copied().enumerate() {
                    let child_layout: LayoutStyle = tree
                        .get_node(child_id)
                        .map(|node: &SceneNode| node.style_ref().get_layout())
                        .unwrap_or_default();
                    let margin: Edges = child_layout.get_margin();
                    let align: AlignItems = match child_layout.get_align_self() {
                        AlignSelf::Auto => parent_layout.get_align_items(),
                        AlignSelf::Stretch => AlignItems::Stretch,
                        AlignSelf::Start => AlignItems::Start,
                        AlignSelf::Center => AlignItems::Center,
                        AlignSelf::End => AlignItems::End,
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
                    let stretch_w: bool = !is_row
                        && align == AlignItems::Stretch
                        && child_layout.get_width() == Length::Auto;
                    let stretch_h: bool = is_row
                        && align == AlignItems::Stretch
                        && child_layout.get_height() == Length::Auto;
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
                    let placed: Size = self.layout_node(tree, child_id, final_space, stats);
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
                    if has_stretched_auto {
                        child_records.push((child_id, child_layout, Size::new(target_w, target_h)));
                    }
                    if is_row {
                        main_cursor += target_w + margin.horizontal();
                        max_cross = max_cross.max(target_h + margin.vertical());
                    } else {
                        main_cursor += target_h + margin.vertical();
                        max_cross = max_cross.max(target_w + margin.horizontal());
                    }
                    if idx + 1 < count {
                        main_cursor += main_gap;
                    }
                }
                if has_stretched_auto && max_cross > cross_avail + 0.5 {
                    for (child_id, child_layout, child_size) in child_records.into_iter() {
                        let align: AlignItems = match child_layout.get_align_self() {
                            AlignSelf::Auto => parent_layout.get_align_items(),
                            AlignSelf::Stretch => AlignItems::Stretch,
                            AlignSelf::Start => AlignItems::Start,
                            AlignSelf::Center => AlignItems::Center,
                            AlignSelf::End => AlignItems::End,
                        };
                        if align == AlignItems::Stretch && child_layout.get_width() == Length::Auto
                        {
                            let widened: f32 = (max_cross - child_layout.get_margin().horizontal())
                                .max(child_size.get_width());
                            if let Some(node) = tree.get_node_mut(child_id) {
                                let mut b: Rect = node.get_layout_box().get_bounds();
                                b.set_width(widened);
                                node.get_mut_layout_box().set_bounds(b);
                            }
                        }
                    }
                }
                return if is_row {
                    Size::new(main_cursor, max_cross)
                } else {
                    Size::new(max_cross, main_cursor)
                };
            }
        }
        let mut total_grow: f32 = 0.0;
        for child_id in ordered.iter().copied() {
            let child_layout: LayoutStyle = tree
                .get_node(child_id)
                .map(|node: &SceneNode| node.style_ref().get_layout())
                .unwrap_or_default();
            let grow: f32 = child_layout.get_flex_grow();
            total_grow += grow;
            let zero_basis_grow: bool = !wrap_enabled
                && grow > 0.0
                && if is_row {
                    child_layout.get_width() == Length::Auto
                } else {
                    child_layout.get_height() == Length::Auto
                };
            let child_size: Size = if zero_basis_grow {
                if is_row {
                    let h: f32 = child_layout
                        .get_height()
                        .resolve(cross_avail, viewport)
                        .unwrap_or(0.0);
                    Size::new(0.0, h)
                } else {
                    let w: f32 = child_layout
                        .get_width()
                        .resolve(cross_avail, viewport)
                        .unwrap_or(0.0);
                    Size::new(w, 0.0)
                }
            } else {
                let probe_space: AvailableSpace = AvailableSpace::new(
                    inner_avail.get_width(),
                    inner_avail.get_height(),
                    viewport,
                    content_origin.get_x(),
                    content_origin.get_y(),
                );
                self.layout_node(tree, child_id, probe_space, stats)
            };
            measured_sizes.push((child_id, child_size, child_layout));
        }
        let mut lines: Vec<Vec<(NodeId, Size, LayoutStyle)>> = Vec::new();
        let mut current_line: Vec<(NodeId, Size, LayoutStyle)> = Vec::new();
        let mut line_main: f32 = 0.0;
        for item in measured_sizes.into_iter() {
            let item_main: f32 = if is_row {
                item.1.get_width() + item.2.get_margin().horizontal()
            } else {
                item.1.get_height() + item.2.get_margin().vertical()
            };
            let needed: f32 = if current_line.is_empty() {
                item_main
            } else {
                line_main + main_gap + item_main
            };
            if wrap_enabled && !current_line.is_empty() && needed > main_avail && main_avail > 0.0 {
                lines.push(take(&mut current_line));
                line_main = item_main;
                current_line.push(item);
            } else {
                line_main = needed;
                current_line.push(item);
            }
        }
        if !current_line.is_empty() {
            lines.push(current_line);
        }
        let mut cross_cursor: f32 = 0.0;
        let mut max_main_extent: f32 = 0.0;
        let line_count: usize = lines.len();
        for (line_idx, mut line) in lines.into_iter().enumerate() {
            let count: usize = line.len();
            let gaps_total: f32 = if count > 1 {
                main_gap * ((count - 1) as f32)
            } else {
                0.0
            };
            let used_main: f32 =
                line.iter()
                    .fold(0.0_f32, |acc: f32, item: &(NodeId, Size, LayoutStyle)| {
                        let sz: f32 = if is_row {
                            item.1.get_width() + item.2.get_margin().horizontal()
                        } else {
                            item.1.get_height() + item.2.get_margin().vertical()
                        };
                        acc + sz
                    })
                    + gaps_total;
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
            let free_main: f32 = (effective_main_avail - used_main).max(0.0);
            if total_grow > 0.0 && free_main > 0.0 && !wrap_enabled {
                for item in line.iter_mut() {
                    let grow: f32 = item.2.get_flex_grow();
                    if grow > 0.0 {
                        let extra: f32 = free_main * (grow / total_grow);
                        if is_row {
                            item.1.set_width(item.1.get_width() + extra);
                        } else {
                            item.1.set_height(item.1.get_height() + extra);
                        }
                    }
                }
            }
            if !wrap_enabled {
                for item in line.iter_mut() {
                    let auto_cross: bool = item.2.get_flex_grow() > 0.0
                        && if is_row {
                            item.2.get_height() == Length::Auto
                        } else {
                            item.2.get_width() == Length::Auto
                        };
                    if auto_cross {
                        let probe_w: f32 = if is_row {
                            item.1.get_width()
                        } else {
                            cross_avail
                        };
                        let probe_h: f32 = if is_row {
                            cross_avail
                        } else {
                            item.1.get_height()
                        };
                        let probe_sp: AvailableSpace = AvailableSpace::new(
                            probe_w,
                            probe_h,
                            viewport,
                            content_origin.get_x(),
                            content_origin.get_y(),
                        );
                        let measured: Size = self.layout_node(tree, item.0, probe_sp, stats);
                        if is_row {
                            item.1.set_height(measured.get_height());
                        } else {
                            item.1.set_width(measured.get_width());
                        }
                    }
                }
            }
            let final_used_main: f32 =
                line.iter()
                    .fold(0.0_f32, |acc: f32, item: &(NodeId, Size, LayoutStyle)| {
                        let sz: f32 = if is_row {
                            item.1.get_width() + item.2.get_margin().horizontal()
                        } else {
                            item.1.get_height() + item.2.get_margin().vertical()
                        };
                        acc + sz
                    })
                    + gaps_total;
            let rem: f32 = (effective_main_avail - final_used_main).max(0.0);
            let (mut main_cursor, extra_step): (f32, f32) =
                match parent_layout.get_justify_content() {
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
            let line_cross: f32 =
                line.iter()
                    .fold(0.0_f32, |acc: f32, item: &(NodeId, Size, LayoutStyle)| {
                        let c: f32 = if is_row {
                            item.1.get_height() + item.2.get_margin().vertical()
                        } else {
                            item.1.get_width() + item.2.get_margin().horizontal()
                        };
                        acc.max(c)
                    });
            let definite_cross: bool = !is_row || frame.get_definite_height();
            let effective_line_cross: f32 = if line_count == 1 && !wrap_enabled && definite_cross {
                line_cross.max(cross_avail)
            } else {
                line_cross
            };
            let mut actual_line_cross: f32 = line_cross;
            for (idx, (child_id, child_size, child_layout)) in line.into_iter().enumerate() {
                let margin: Edges = child_layout.get_margin();
                let item_cross: f32 = if is_row {
                    child_size.get_height()
                } else {
                    child_size.get_width()
                };
                let align: AlignItems = match child_layout.get_align_self() {
                    AlignSelf::Auto => parent_layout.get_align_items(),
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
                    && child_layout.get_flex_grow() > 0.0
                    && if is_row {
                        child_layout.get_height() == Length::Auto
                    } else {
                        child_layout.get_width() == Length::Auto
                    };
                let target_w: f32 = if !is_row
                    && align == AlignItems::Stretch
                    && child_layout.get_width() == Length::Auto
                {
                    avail_item_cross.max(child_size.get_width())
                } else {
                    child_size.get_width()
                };
                let target_h: f32 = if is_row
                    && align == AlignItems::Stretch
                    && child_layout.get_height() == Length::Auto
                {
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
                let space_w: f32 = if matches!(child_layout.get_width(), Length::Percent(_)) {
                    inner_avail.get_width()
                } else if auto_cross_grow && !is_row {
                    cross_avail
                } else {
                    target_w
                };
                let space_h: f32 = if matches!(child_layout.get_height(), Length::Percent(_)) {
                    inner_avail.get_height()
                } else if auto_cross_grow && is_row {
                    cross_avail
                } else {
                    target_h
                };
                let stretch_child_w: bool = !(auto_cross_grow && !is_row)
                    && ((!is_row
                        && align == AlignItems::Stretch
                        && child_layout.get_width() == Length::Auto)
                        || (is_row
                            && child_layout.get_flex_grow() > 0.0
                            && child_layout.get_width() == Length::Auto));
                let stretch_child_h: bool = !(auto_cross_grow && is_row)
                    && ((is_row
                        && align == AlignItems::Stretch
                        && child_layout.get_height() == Length::Auto)
                        || (!is_row
                            && child_layout.get_flex_grow() > 0.0
                            && child_layout.get_height() == Length::Auto));
                let final_space: AvailableSpace =
                    AvailableSpace::new(space_w, space_h, viewport, x, y)
                        .with_stretch(stretch_child_w, stretch_child_h);
                let placed: Size = self.layout_node(tree, child_id, final_space, stats);
                if let Some(node) = tree.get_node_mut(child_id) {
                    let mut b: Rect = node.get_layout_box().get_bounds();
                    let is_clipped: bool = matches!(
                        child_layout.get_overflow(),
                        Overflow::Hidden | Overflow::Scroll
                    );
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
                    } else if is_clipped || child_layout.get_flex_grow() > 0.0 {
                        b.set_width(target_w);
                        b.set_height(target_h);
                    } else {
                        b.set_width(target_w.max(placed.get_width()));
                        b.set_height(target_h.max(placed.get_height()));
                    }
                    node.get_mut_layout_box().set_bounds(b);
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
            max_main_extent = max_main_extent.max(final_used_main);
            cross_cursor += actual_line_cross;
            if line_idx + 1 < line_count {
                cross_cursor += cross_gap;
            }
        }
        if is_row {
            Size::new(max_main_extent, cross_cursor)
        } else {
            Size::new(cross_cursor, max_main_extent)
        }
    }

    /// Executes uniform-column CSS Grid layout with row and column gaps.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Layout engine reference.
    /// - `&mut SceneTree` - Mutable scene tree.
    /// - `&[NodeId]` - Grid child handles.
    /// - `ContainerFrame` - Grid container constraints and origin.
    /// - `&mut LayoutStats` - Mutable telemetry counters.
    ///
    /// # Returns
    ///
    /// - `Size` - Aggregate grid content dimensions.
    fn layout_grid(
        &mut self,
        tree: &mut SceneTree,
        children: &[NodeId],
        frame: ContainerFrame,
        stats: &mut LayoutStats,
    ) -> Size {
        if children.is_empty() {
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
        let mut y_cursor: f32 = 0.0;
        let mut idx: usize = 0;
        while idx < children.len() {
            let row_end: usize = (idx + cols).min(children.len());
            let mut row_h: f32 = 0.0;
            for (col_idx, child_id) in children[idx..row_end].iter().copied().enumerate() {
                let x: f32 = content_origin.get_x() + (col_idx as f32) * (col_w + col_gap);
                let y: f32 = content_origin.get_y() + y_cursor;
                let auto_w: bool = tree
                    .get_node(child_id)
                    .map(|n: &SceneNode| n.style_ref().get_layout().get_width() == Length::Auto)
                    .unwrap_or(false);
                let cell_space: AvailableSpace =
                    AvailableSpace::new(col_w, inner_avail.get_height(), viewport, x, y)
                        .with_stretch(auto_w, false);
                let child_size: Size = self.layout_node(tree, child_id, cell_space, stats);
                if let Some(node) = tree.get_node_mut(child_id) {
                    let mut b: Rect = node.get_layout_box().get_bounds();
                    b.set_width(col_w);
                    node.get_mut_layout_box().set_bounds(b);
                }
                row_h = row_h.max(child_size.get_height());
            }
            for child_id in children[idx..row_end].iter().copied() {
                if let Some(node) = tree.get_node_mut(child_id) {
                    let mut b: Rect = node.get_layout_box().get_bounds();
                    if b.get_height() < row_h {
                        b.set_height(row_h);
                        node.get_mut_layout_box().set_bounds(b);
                    }
                }
            }
            y_cursor += row_h;
            if row_end < children.len() {
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
    /// - `&Self` - Layout engine reference.
    /// - `&mut SceneTree` - Mutable retained scene tree.
    /// - `NodeId` - Root node of the clean subtree being shifted.
    /// - `f32` - Horizontal translation delta in logical pixels.
    /// - `f32` - Vertical translation delta in logical pixels.
    fn translate_subtree(&self, tree: &mut SceneTree, root_id: NodeId, dx: f32, dy: f32) {
        let mut stack: Vec<(NodeId, bool)> = vec![(root_id, true)];
        while let Some((nid, is_root)) = stack.pop() {
            let children_to_visit: Vec<NodeId> = {
                let Some(node) = tree.get_node_mut(nid) else {
                    continue;
                };
                let layout: LayoutStyle = node.style_ref().get_layout();
                if !is_root && layout.get_position() == Position::Fixed {
                    continue;
                }
                if let Some(mut c) = node.cached_constraint() {
                    c.set_origin_x(c.get_origin_x() + dx);
                    c.set_origin_y(c.get_origin_y() + dy);
                    node.set_last_constraint(Some(c));
                }
                if layout.get_display() == Display::None {
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
                node.get_children().clone()
            };
            for child_id in children_to_visit.into_iter() {
                stack.push((child_id, false));
            }
        }
    }
}
