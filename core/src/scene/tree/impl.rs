use super::*;

/// Default scene tree with a pre-created root `View` node.
impl Default for SceneTree {
    /// Allocates a new `SceneTree` containing a root container node.
    ///
    /// # Returns
    ///
    /// - `Self` - Initialized scene tree.
    fn default() -> Self {
        let root_style: Style = Style::default()
            .column()
            .width(Length::Percent(100.0))
            .height(Length::Percent(100.0));
        let root_node: SceneNode = SceneNode::new(1, NodeKind::View, root_style);
        Self {
            nodes: vec![root_node],
            free_list: Vec::new(),
            live_count: 1,
            root: NodeId::new(0, 1),
            focus: FocusManager::default(),
            revision: 1,
            transitions: Vec::new(),
            caret_visible: true,
            portal_nodes: Vec::new(),
            dirty_paint_nodes: Vec::new(),
            full_damage_pending: true,
            layout_damage_pending: true,
            full_damage_overflow: false,
            created_since_damage: 0,
            extra_logical_damage: Vec::new(),
            text_selection: None,
        }
    }
}

/// Core retained scene graph mutation and traversal operations.
impl SceneTree {
    /// Returns the incrementally maintained slice of live `NodeKind::Portal` and `Position::Fixed` nodes.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scene tree reference.
    ///
    /// # Returns
    ///
    /// - `&[NodeId]` - Live top-layer portal and fixed-position node handles.
    #[must_use]
    #[inline(always)]
    pub fn portal_nodes(&self) -> &[NodeId] {
        let Self { portal_nodes, .. } = self;
        portal_nodes.as_slice()
    }

    /// Returns `true` when at least one live `NodeKind::Portal` or `Position::Fixed` node is registered.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scene tree reference.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether any top-layer portal or fixed node exists in the scene.
    #[must_use]
    #[inline(always)]
    pub fn has_portal_nodes(&self) -> bool {
        let Self { portal_nodes, .. } = self;
        !portal_nodes.is_empty()
    }

    /// Synchronizes `id` membership in `portal_nodes` whenever a top-layer node is created, mutated, or disposed.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene tree.
    /// - `NodeId` - Target node handle.
    /// - `bool` - Whether the node currently qualifies as a `Portal` or `Position::Fixed` top-layer node.
    fn sync_portal_registration(&mut self, id: NodeId, is_portal: bool) {
        let portals: &mut Vec<NodeId> = self.get_mut_portal_nodes();
        if is_portal {
            if !portals.contains(&id) {
                portals.push(id);
            }
        } else if !portals.is_empty() {
            portals.retain(|existing: &NodeId| *existing != id);
        }
    }

    /// Allocates a new node of `kind` with initial `style`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene tree.
    /// - `NodeKind` - Semantic node kind.
    /// - `Style` - Initial style.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Generation-checked node handle.
    pub fn create_node(&mut self, kind: NodeKind, style: Style) -> NodeId {
        let is_portal: bool = matches!(kind, NodeKind::Portal)
            || style.get_layout().get_position() == Position::Fixed;
        let id: NodeId = if let Some(index) = self.get_mut_free_list().pop() {
            let slot: &mut SceneNode = &mut self.get_mut_nodes()[index as usize];
            let next_gen: u32 = slot.get_generation().wrapping_add(1).max(1);
            *slot = SceneNode::new(next_gen, kind, style);
            NodeId::new(index, next_gen)
        } else {
            let index: u32 = self.get_nodes().len() as u32;
            self.get_mut_nodes().push(SceneNode::new(1, kind, style));
            NodeId::new(index, 1)
        };
        if is_portal {
            self.get_mut_portal_nodes().push(id);
            self.set_full_damage_pending(true);
        }
        let next_live: usize = self.get_live_count().saturating_add(1);
        self.set_live_count(next_live);
        let next_created: usize = self.get_created_since_damage().saturating_add(1);
        self.set_created_since_damage(next_created);
        self.set_layout_damage_pending(true);
        if next_created > DIRTY_PAINT_FULL_THRESHOLD {
            self.set_full_damage_pending(true);
            self.set_full_damage_overflow(true);
        }
        let next_rev: u64 = self.get_revision().wrapping_add(1);
        self.set_revision(next_rev);
        id
    }

    /// Creates a container `View` node.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene tree.
    /// - `Style` - Initial box style.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Created node handle.
    pub fn create_view(&mut self, style: Style) -> NodeId {
        self.create_node(NodeKind::View, style)
    }

    /// Creates a `Text` node holding `content`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene tree.
    /// - `String` - Initial UTF-8 text.
    /// - `Style` - Initial text and box style.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Created text node handle.
    pub fn create_text(&mut self, content: String, style: Style) -> NodeId {
        self.create_node(NodeKind::Text(content), style)
    }

    /// Creates an editable `Input` node.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene tree.
    /// - `InputNodeState` - Initial input state.
    /// - `Style` - Initial input style.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Created input node handle.
    pub fn create_input(&mut self, state: InputNodeState, style: Style) -> NodeId {
        self.create_node(NodeKind::Input(state), style)
    }

    /// Creates an `Image` node displaying `buffer`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene tree.
    /// - `ImageBuffer` - Decoded RGBA8 image buffer.
    /// - `Style` - Initial image style.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Created image node handle.
    pub fn create_image(&mut self, buffer: ImageBuffer, style: Style) -> NodeId {
        self.create_node(NodeKind::Image(buffer), style)
    }

    /// Creates a `ScrollView` container with `Overflow::Scroll`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene tree.
    /// - `Style` - Initial scroll container style.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Created scroll view handle.
    pub fn create_scroll_view(&mut self, style: Style) -> NodeId {
        self.create_node(NodeKind::ScrollView, style.overflow(Overflow::Scroll))
    }

    /// Creates a custom `Canvas` node bound to `painter`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene tree.
    /// - `Style` - Initial canvas style.
    /// - `CanvasDrawCallback` - Draw callback invoked on each paint pass.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Created canvas node handle.
    pub fn create_canvas(&mut self, style: Style, painter: CanvasDrawCallback) -> NodeId {
        let id: NodeId = self.create_node(NodeKind::Canvas, style);
        if let Some(node) = self.get_node_mut(id) {
            node.set_canvas_painter(Some(painter));
        }
        id
    }

    /// Creates an overlay `Portal` node rendered in the top stacking context.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene tree.
    /// - `Style` - Initial portal style.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Created portal node handle.
    pub fn create_portal(&mut self, style: Style) -> NodeId {
        self.create_node(NodeKind::Portal, style)
    }

    /// Creates a logical `Fragment` grouping node.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene tree.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Created fragment node handle.
    pub fn create_fragment(&mut self) -> NodeId {
        self.create_node(NodeKind::Fragment, Style::default())
    }

    /// Returns an immutable reference to a live node matching `id`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scene tree reference.
    /// - `NodeId` - Node handle.
    ///
    /// # Returns
    ///
    /// - `Option<&SceneNode>` - Live node reference if valid.
    #[must_use]
    pub fn get_node(&self, id: NodeId) -> Option<&SceneNode> {
        let node: &SceneNode = self.get_nodes().get(id.as_usize())?;
        if node.get_alive() && node.get_generation() == id.get_generation() {
            Some(node)
        } else {
            None
        }
    }

    /// Returns a mutable reference to a live node matching `id`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene tree.
    /// - `NodeId` - Node handle.
    ///
    /// # Returns
    ///
    /// - `Option<&mut SceneNode>` - Mutable live node reference if valid.
    pub fn get_node_mut(&mut self, id: NodeId) -> Option<&mut SceneNode> {
        let node: &mut SceneNode = self.get_mut_nodes().get_mut(id.as_usize())?;
        if node.get_alive() && node.get_generation() == id.get_generation() {
            Some(node)
        } else {
            None
        }
    }

    /// Returns total count of live scene nodes in O(1).
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scene tree reference.
    ///
    /// # Returns
    ///
    /// - `usize` - Number of live nodes in the arena.
    #[must_use]
    #[inline(always)]
    pub fn live_node_count(&self) -> usize {
        self.get_live_count()
    }

    /// Appends `child` to `parent` and marks layout and paint dirty upward.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene tree.
    /// - `NodeId` - Parent container node.
    /// - `NodeId` - Child node to attach.
    pub fn append_child(&mut self, parent: NodeId, child: NodeId) {
        if parent == child {
            return;
        }
        let prev_parent: Option<NodeId> = self.get_node(child).and_then(SceneNode::parent_id);
        if let Some(old_parent) = prev_parent {
            if old_parent != parent {
                self.record_subtree_logical_damage(child);
            }
            if let Some(old_node) = self.get_node_mut(old_parent) {
                old_node
                    .get_mut_children()
                    .retain(|item: &NodeId| *item != child);
                if old_parent != parent {
                    self.mark_dirty(old_parent, DIRTY_CHILDREN_CHANGE_MASK);
                }
            }
        }
        if let Some(child_node) = self.get_node_mut(child) {
            child_node.set_parent(Some(parent));
        } else {
            return;
        }
        if let Some(parent_node) = self.get_node_mut(parent) {
            parent_node.get_mut_children().push(child);
        }
        self.mark_dirty(parent, DIRTY_CHILDREN_CHANGE_MASK);
    }

    /// Appends multiple `children` to `parent` in a single dirty-marking pass with zero-alloc fresh-orphan fast path.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene tree.
    /// - `NodeId` - Parent container node.
    /// - `&[NodeId]` - Ordered child nodes to append.
    pub fn append_children(&mut self, parent: NodeId, children: &[NodeId]) {
        if children.is_empty() || self.get_node(parent).is_none() {
            return;
        }
        let all_fresh_orphans: bool = children.iter().copied().all(|child: NodeId| {
            child != parent
                && self
                    .get_node(child)
                    .is_some_and(|node: &SceneNode| node.parent_id().is_none())
        });
        if all_fresh_orphans {
            for child in children.iter().copied() {
                if let Some(child_node) = self.get_node_mut(child) {
                    child_node.set_parent(Some(parent));
                }
            }
            if let Some(parent_node) = self.get_node_mut(parent) {
                parent_node.get_mut_children().extend_from_slice(children);
            }
            self.mark_dirty(parent, DIRTY_CHILDREN_CHANGE_MASK);
            return;
        }
        let mut attached: Vec<NodeId> = Vec::with_capacity(children.len());
        for child in children.iter().copied() {
            if child == parent {
                continue;
            }
            let prev_parent: Option<NodeId> = self
                .get_node(child)
                .and_then(|node: &SceneNode| node.parent_id());
            if let Some(old_parent) = prev_parent {
                if old_parent != parent {
                    self.record_subtree_logical_damage(child);
                }
                if let Some(old_node) = self.get_node_mut(old_parent) {
                    old_node
                        .get_mut_children()
                        .retain(|id: &NodeId| *id != child);
                }
                if old_parent != parent {
                    self.mark_dirty(old_parent, DIRTY_CHILDREN_CHANGE_MASK);
                }
            }
            if let Some(child_node) = self.get_node_mut(child) {
                child_node.set_parent(Some(parent));
                attached.push(child);
            }
        }
        if attached.is_empty() {
            return;
        }
        if let Some(parent_node) = self.get_node_mut(parent) {
            parent_node.get_mut_children().extend(attached);
        }
        self.mark_dirty(parent, DIRTY_CHILDREN_CHANGE_MASK);
    }

    /// Replaces all children of `parent` with `new_children`, disposing previous subtrees cleanly.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene tree.
    /// - `NodeId` - Parent container node.
    /// - `Vec<NodeId>` - Replacement ordered children.
    pub fn replace_children(&mut self, parent: NodeId, new_children: Vec<NodeId>) {
        let old_children: Vec<NodeId> = match self.get_node_mut(parent) {
            Some(node) => take(node.get_mut_children()),
            None => return,
        };
        if new_children.is_empty() {
            for old_child in old_children.into_iter() {
                self.dispose_subtree(old_child);
            }
        } else if old_children.len() > 16 && new_children.len() > 16 {
            let keep_set: HashSet<NodeId> = new_children.iter().copied().collect();
            for old_child in old_children.into_iter() {
                if !keep_set.contains(&old_child) {
                    self.dispose_subtree(old_child);
                }
            }
        } else {
            for old_child in old_children.into_iter() {
                if !new_children.contains(&old_child) {
                    self.dispose_subtree(old_child);
                }
            }
        }
        for child in new_children.iter().copied() {
            if let Some(child_node) = self.get_node_mut(child) {
                child_node.set_parent(Some(parent));
            }
        }
        if let Some(parent_node) = self.get_node_mut(parent) {
            parent_node.set_children(new_children);
        }
        self.mark_dirty(parent, DIRTY_CHILDREN_CHANGE_MASK);
    }

    /// Computes the minimal dirty bitmask and optional portal update comparing `prev` and `next` styles by reference.
    ///
    /// # Arguments
    ///
    /// - `&Style` - Previous compiled style reference.
    /// - `&Style` - Candidate replacement style reference.
    /// - `bool` - Whether the target node is already `NodeKind::Portal`.
    ///
    /// # Returns
    ///
    /// - `(u16, Option<bool>)` - Tuple `(dirty_mask, portal_update)`; `dirty_mask == DIRTY_NONE` on no-op writes.
    #[inline]
    fn diff_style_change(prev: &Style, next: &Style, is_portal_kind: bool) -> (u16, Option<bool>) {
        let layout_changed: bool = prev.layout_ref() != next.layout_ref();
        let text_metrics_changed: bool = !prev.text_ref().same_layout_metrics(next.text_ref());
        if layout_changed || text_metrics_changed {
            let prev_fixed: bool = prev.layout_ref().get_position() == Position::Fixed;
            let next_fixed: bool = next.layout_ref().get_position() == Position::Fixed;
            let portal_update: Option<bool> = if !is_portal_kind && prev_fixed != next_fixed {
                Some(next_fixed)
            } else {
                None
            };
            let layout_bits: u16 = DIRTY_LAYOUT | DIRTY_SUBTREE_LAYOUT;
            let visual_bits: u16 = DIRTY_TEXT | DIRTY_PAINT;
            let mask: u16 = layout_bits | visual_bits;
            return (mask, portal_update);
        }
        let paint_changed: bool = prev.paint_ref() != next.paint_ref();
        let text_visual_changed: bool = prev.text_ref() != next.text_ref();
        if paint_changed || text_visual_changed {
            (DIRTY_PAINT, None)
        } else {
            (DIRTY_NONE, None)
        }
    }

    /// Returns `true` when transitioning from `prev` to `next` paint style can shrink shadows, borders, transforms, or hide children.
    ///
    /// # Arguments
    ///
    /// - `&PaintStyle` - Previous paint style reference.
    /// - `&PaintStyle` - Replacement paint style reference.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the node's pre-mutation conservative bounds must be snapshotted before overwriting style.
    #[inline]
    fn needs_pre_paint_damage(prev: &PaintStyle, next: &PaintStyle) -> bool {
        let shadow_changed: bool = prev.box_shadow() != next.box_shadow();
        let transform_changed: bool = prev.get_transform() != next.get_transform();
        let border_shrank: bool = prev.get_border().get_width() > next.get_border().get_width();
        let focus_removed: bool =
            prev.focus_ring_border().is_some() && next.focus_ring_border().is_none();
        let opacity_toggled_zero: bool = (prev.get_opacity() > 0.0) != (next.get_opacity() > 0.0);
        let a: bool = shadow_changed || transform_changed;
        let b: bool = border_shrank || focus_removed;
        let ab: bool = a || b;
        ab || opacity_toggled_zero
    }

    /// Surgically updates a node's `Style`, marking only `DIRTY_PAINT` for pure paint changes and skipping no-op writes.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene tree.
    /// - `NodeId` - Target node handle.
    /// - `Style` - Replacement compiled style.
    pub fn set_style(&mut self, id: NodeId, style: Style) {
        let pre_snapshot: Option<(u16, Option<bool>, bool, bool)> =
            self.get_node(id).map(|node: &SceneNode| {
                let is_portal_kind: bool = matches!(node.get_kind(), NodeKind::Portal);
                let prev: &Style = node.style_ref();
                let (mask, update): (u16, Option<bool>) =
                    Self::diff_style_change(prev, &style, is_portal_kind);
                let pre_paint: bool = mask != DIRTY_NONE
                    && Self::needs_pre_paint_damage(prev.paint_ref(), style.paint_ref());
                let subtree_hidden: bool = pre_paint
                    && (prev.paint_ref().get_opacity() > 0.0)
                        != (style.paint_ref().get_opacity() > 0.0)
                    && !node.get_children().is_empty();
                (mask, update, pre_paint, subtree_hidden)
            });
        let Some((dirty_mask, portal_update, pre_paint, subtree_hidden)) = pre_snapshot else {
            return;
        };
        if dirty_mask == DIRTY_NONE {
            return;
        }
        if subtree_hidden {
            self.record_subtree_logical_damage(id);
        } else if pre_paint {
            self.record_node_logical_damage(id);
        }
        if let Some(node) = self.get_node_mut(id) {
            node.set_style(style);
        }
        if let Some(is_portal) = portal_update {
            self.sync_portal_registration(id, is_portal);
        }
        self.mark_dirty(id, dirty_mask);
    }

    /// Mutates a node's `Style` in place and marks the minimal dirty mask (`DIRTY_PAINT` vs layout) only if changed.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene tree.
    /// - `NodeId` - Target node handle.
    /// - `F` - In-place style mutator closure receiving `&mut Style`.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the node existed and its style changed.
    pub fn mutate_style<F>(&mut self, id: NodeId, mutator: F) -> bool
    where
        F: FnOnce(&mut Style),
    {
        let prev_opt: Option<(Style, bool, bool)> = self.get_node(id).map(|node: &SceneNode| {
            (
                *node.style_ref(),
                matches!(node.get_kind(), NodeKind::Portal),
                !node.get_children().is_empty(),
            )
        });
        let Some((prev, is_portal_kind, has_children)) = prev_opt else {
            return false;
        };
        let mut next: Style = prev;
        mutator(&mut next);
        let (dirty_mask, portal_update): (u16, Option<bool>) =
            Self::diff_style_change(&prev, &next, is_portal_kind);
        if dirty_mask == DIRTY_NONE {
            return false;
        }
        let pre_paint: bool = Self::needs_pre_paint_damage(prev.paint_ref(), next.paint_ref());
        if pre_paint {
            let opacity_toggled_zero: bool =
                (prev.paint_ref().get_opacity() > 0.0) != (next.paint_ref().get_opacity() > 0.0);
            if opacity_toggled_zero && has_children {
                self.record_subtree_logical_damage(id);
            } else {
                self.record_node_logical_damage(id);
            }
        }
        if let Some(node) = self.get_node_mut(id) {
            node.set_style(next);
        }
        if let Some(is_portal) = portal_update {
            self.sync_portal_registration(id, is_portal);
        }
        self.mark_dirty(id, dirty_mask);
        true
    }

    /// Surgically updates the UTF-8 string of a `NodeKind::Text` node.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene tree.
    /// - `NodeId` - Target text node handle.
    /// - `String` - Replacement text string.
    pub fn set_text(&mut self, id: NodeId, text: String) {
        let (changed, fixed_size): (bool, bool) = match self.get_node_mut(id) {
            Some(node) => {
                let layout: &LayoutStyle = node.style_ref().layout_ref();
                let has_fixed: bool = matches!(layout.get_width(), Length::Px(_))
                    && matches!(layout.get_height(), Length::Px(_));
                match node.get_mut_kind() {
                    NodeKind::Text(current) => {
                        if *current == text {
                            (false, has_fixed)
                        } else {
                            *current = text;
                            (true, has_fixed)
                        }
                    }
                    _ => (false, false),
                }
            }
            None => (false, false),
        };
        if changed {
            let mask: u16 = if fixed_size {
                DIRTY_PAINT
            } else {
                DIRTY_TEXT_CHANGE_MASK
            };
            self.mark_dirty(id, mask);
        }
    }

    /// Updates scroll offset on a scroll container without triggering full subtree re-layout.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene tree.
    /// - `NodeId` - Target scroll container handle.
    /// - `Point` - New scroll offset clamped to content bounds.
    pub fn set_scroll_offset(&mut self, id: NodeId, offset: Point) {
        let changed: bool = match self.get_node_mut(id) {
            Some(node) => {
                let layout_box: LayoutBox = node.get_layout_box();
                let unlaid_out: bool = layout_box.get_content_size().get_width() <= 0.0
                    && layout_box.get_content_size().get_height() <= 0.0
                    && layout_box.get_content_bounds().get_width() <= 0.0
                    && layout_box.get_content_bounds().get_height() <= 0.0;
                let clamped: Point = if unlaid_out || node.virtual_scroll_metrics().is_some() {
                    Point::new(offset.get_x().max(0.0), offset.get_y().max(0.0))
                } else {
                    let max_x: f32 = (layout_box.get_content_size().get_width()
                        - layout_box.get_content_bounds().get_width())
                    .max(0.0);
                    let max_y: f32 = (layout_box.get_content_size().get_height()
                        - layout_box.get_content_bounds().get_height())
                    .max(0.0);
                    Point::new(
                        offset.get_x().clamp(0.0, max_x),
                        offset.get_y().clamp(0.0, max_y),
                    )
                };
                if layout_box.get_scroll_offset() == clamped {
                    false
                } else {
                    node.get_mut_layout_box().set_scroll_offset(clamped);
                    true
                }
            }
            None => false,
        };
        if changed {
            self.mark_dirty(id, DIRTY_SCROLL | DIRTY_PAINT);
        }
    }

    /// Records virtualized scroll progress `(scroll_y, content_height)` on `id` for overlay scrollbar rendering.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene tree.
    /// - `NodeId` - Target virtualized scroll container.
    /// - `f32` - Active virtual vertical scroll offset in logical pixels.
    /// - `f32` - Full virtualized content height in logical pixels.
    pub fn set_virtual_scroll(&mut self, id: NodeId, scroll_y: f32, content_height: f32) {
        let changed: bool = match self.get_node_mut(id) {
            Some(node) => {
                let next: Option<(f32, f32)> = Some((scroll_y.max(0.0), content_height.max(1.0)));
                if node.virtual_scroll_metrics() == next {
                    false
                } else {
                    node.set_virtual_scroll(next);
                    true
                }
            }
            None => false,
        };
        if changed {
            self.mark_dirty(id, DIRTY_SCROLL | DIRTY_PAINT);
        }
    }

    /// Computes screen-space logical bounds for `id` after applying ancestor scroll offsets and transforms.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scene tree reference.
    /// - `NodeId` - Target node handle.
    ///
    /// # Returns
    ///
    /// - `Option<Rect>` - Screen-space rectangle if the node is alive.
    #[must_use]
    pub fn node_screen_bounds(&self, id: NodeId) -> Option<Rect> {
        let node: &SceneNode = self.get_node(id)?;
        let raw: Rect = node.get_layout_box().get_bounds();
        let (off, tx, ty, sx, sy): (Point, f32, f32, f32, f32) =
            self.node_scroll_and_transform(node);
        Some(Rect::new(
            raw.get_x() - off.get_x() + tx,
            raw.get_y() - off.get_y() + ty,
            raw.get_width() * sx,
            raw.get_height() * sy,
        ))
    }

    /// Computes accumulated ancestor scroll offset and combined 2D transform for `node` in a single ancestor walk.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scene tree reference.
    /// - `&SceneNode` - Borrowed target scene node.
    ///
    /// # Returns
    ///
    /// - `(Point, f32, f32, f32, f32)` - Tuple `(scroll_offset, tx, ty, scale_x, scale_y)`.
    #[inline]
    fn node_scroll_and_transform(&self, node: &SceneNode) -> (Point, f32, f32, f32, f32) {
        let style: &Style = node.style_ref();
        let tr: Transform2D = style.get_paint().get_transform();
        let mut tx: f32 = tr.get_translate_x();
        let mut ty: f32 = tr.get_translate_y();
        let sx_scale: f32 = tr.get_scale_x();
        let sy_scale: f32 = tr.get_scale_y();
        let start_top_layer: bool = matches!(node.get_kind(), NodeKind::Portal)
            || style.get_layout().get_position() == Position::Fixed;
        let mut scroll_x: f32 = 0.0;
        let mut scroll_y: f32 = 0.0;
        let mut cursor: Option<NodeId> = node.parent_id();
        while let Some(pid) = cursor {
            if let Some(pnode) = self.get_node(pid) {
                let pstyle: &Style = pnode.style_ref();
                let ptr: Transform2D = pstyle.get_paint().get_transform();
                tx += ptr.get_translate_x();
                ty += ptr.get_translate_y();
                if !start_top_layer {
                    let off: Point = pnode.get_layout_box().get_scroll_offset();
                    scroll_x += off.get_x();
                    scroll_y += off.get_y();
                }
                let parent_top_layer: bool = matches!(pnode.get_kind(), NodeKind::Portal)
                    || pstyle.get_layout().get_position() == Position::Fixed;
                if parent_top_layer {
                    break;
                }
                cursor = pnode.parent_id();
            } else {
                break;
            }
        }
        (Point::new(scroll_x, scroll_y), tx, ty, sx_scale, sy_scale)
    }

    /// Computes a conservative logical screen-space paint rectangle for `id` including ancestor scroll, transform, border, focus ring, and `3 * sigma` shadow halo.
    ///
    /// Returns `None` when a non-axis-aligned rotation or ancestor scale requires a safe full-viewport fallback.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scene tree reference.
    /// - `NodeId` - Target scene node handle.
    ///
    /// # Returns
    ///
    /// - `Option<Rect>` - Conservative logical screen bounds, `Some(Rect::default())` when unmeasured, or `None` for unsafe fallback.
    #[must_use]
    pub(crate) fn node_conservative_logical_rect(&self, id: NodeId) -> Option<Rect> {
        let node: &SceneNode = self.get_node(id)?;
        let raw_bounds: Rect = node.get_layout_box().get_bounds();
        if raw_bounds.get_width() <= 0.0 || raw_bounds.get_height() <= 0.0 {
            return Some(Rect::default());
        }
        let style: &Style = node.style_ref();
        let paint: &PaintStyle = style.paint_ref();
        let tr: Transform2D = paint.get_transform();
        if tr.get_rotation_rad().abs() > 0.0001
            || tr.get_scale_x() <= 0.0
            || tr.get_scale_y() <= 0.0
        {
            return None;
        }
        let mut tx: f32 = tr.get_translate_x();
        let mut ty: f32 = tr.get_translate_y();
        let sx: f32 = tr.get_scale_x();
        let sy: f32 = tr.get_scale_y();
        let start_top_layer: bool = matches!(node.get_kind(), NodeKind::Portal)
            || style.layout_ref().get_position() == Position::Fixed;
        let mut scroll_x: f32 = 0.0;
        let mut scroll_y: f32 = 0.0;
        let mut cursor: Option<NodeId> = node.parent_id();
        while let Some(pid) = cursor {
            let Some(pnode) = self.get_node(pid) else {
                break;
            };
            let pstyle: &Style = pnode.style_ref();
            let ptr: Transform2D = pstyle.paint_ref().get_transform();
            if ptr.get_rotation_rad().abs() > 0.0001
                || (ptr.get_scale_x() - 1.0).abs() > 0.001
                || (ptr.get_scale_y() - 1.0).abs() > 0.001
            {
                return None;
            }
            tx += ptr.get_translate_x();
            ty += ptr.get_translate_y();
            if !start_top_layer {
                let off: Point = pnode.get_layout_box().get_scroll_offset();
                scroll_x += off.get_x();
                scroll_y += off.get_y();
            }
            let parent_top_layer: bool = matches!(pnode.get_kind(), NodeKind::Portal)
                || pstyle.layout_ref().get_position() == Position::Fixed;
            if parent_top_layer {
                break;
            }
            cursor = pnode.parent_id();
        }
        let border_w: f32 = paint.get_border().get_width();
        let focus_pad: f32 = if paint.focus_ring_border().is_some() {
            12.0
        } else {
            0.0
        };
        let mut margin: f32 = 10.0_f32.max(border_w + 6.0).max(focus_pad);
        if let Some(shadow) = paint.box_shadow() {
            let blur_pad: f32 = (shadow.get_blur_radius().max(0.5) * 1.5).min(96.0) + 4.0;
            let spread: f32 = shadow.get_spread_radius().max(0.0);
            let off_max: f32 = shadow.get_offset_x().abs().max(shadow.get_offset_y().abs());
            margin = margin.max(blur_pad + spread + off_max + 4.0);
        }
        let base_x: f32 = raw_bounds.get_x() - scroll_x;
        let base_y: f32 = raw_bounds.get_y() - scroll_y;
        let base_rect: Rect = Rect::new(
            base_x - margin,
            base_y - margin,
            raw_bounds.get_width() + margin * 2.0,
            raw_bounds.get_height() + margin * 2.0,
        );
        let shifted_x: bool = tx != 0.0 || sx != 1.0;
        let shifted_y: bool = ty != 0.0 || sy != 1.0;
        if shifted_x || shifted_y {
            let scale_pad_x: f32 = ((sx - 1.0).abs() * raw_bounds.get_width()).max(0.0);
            let scale_pad_y: f32 = ((sy - 1.0).abs() * raw_bounds.get_height()).max(0.0);
            let shifted_rect: Rect = Rect::new(
                base_x + tx - margin - scale_pad_x,
                base_y + ty - margin - scale_pad_y,
                raw_bounds.get_width() * sx + (margin + scale_pad_x) * 2.0,
                raw_bounds.get_height() * sy + (margin + scale_pad_y) * 2.0,
            );
            Some(base_rect.union(shifted_rect))
        } else {
            Some(base_rect)
        }
    }

    /// Converts a conservative logical screen rectangle into an exact outward-rounded physical pixel rectangle.
    ///
    /// # Arguments
    ///
    /// - `Rect` - Conservative logical screen rectangle.
    /// - `f32` - Active display scale factor.
    ///
    /// # Returns
    ///
    /// - `Rect` - Physical pixel rectangle guaranteed to enclose all covered pixels.
    #[must_use]
    pub(crate) fn scale_logical_to_phys_rect(logical: Rect, scale: f32) -> Rect {
        let x0: f32 = (logical.get_x() * scale).floor();
        let y0: f32 = (logical.get_y() * scale).floor();
        let x1: f32 = (logical.right() * scale).ceil();
        let y1: f32 = (logical.bottom() * scale).ceil();
        Rect::new(x0, y0, (x1 - x0).max(0.0), (y1 - y0).max(0.0))
    }

    /// Records the current conservative logical bounds of `id` into `extra_logical_damage` prior to destructive mutation.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene tree.
    /// - `NodeId` - Target scene node handle.
    fn record_node_logical_damage(&mut self, id: NodeId) {
        if self.get_full_damage_pending() {
            return;
        }
        let rect_opt: Option<Rect> = self.node_conservative_logical_rect(id);
        match rect_opt {
            Some(rect) => {
                if rect.get_width() <= 0.0 || rect.get_height() <= 0.0 {
                    return;
                }
                if self.get_extra_logical_damage().len() >= DIRTY_PAINT_FULL_THRESHOLD {
                    self.set_full_damage_pending(true);
                    self.set_full_damage_overflow(true);
                    self.get_mut_extra_logical_damage().clear();
                } else {
                    self.get_mut_extra_logical_damage().push(rect);
                }
            }
            None => {
                self.set_full_damage_pending(true);
            }
        }
    }

    /// Records pre-mutation conservative logical bounds for every live laid-out node in the subtree rooted at `root`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene tree.
    /// - `NodeId` - Subtree root handle.
    fn record_subtree_logical_damage(&mut self, root: NodeId) {
        if self.get_full_damage_pending() {
            return;
        }
        let mut stack: Vec<NodeId> = vec![root];
        let mut visited: usize = 0;
        while let Some(current) = stack.pop() {
            let Some(node) = self.get_node(current) else {
                continue;
            };
            visited += 1;
            if visited > DIRTY_PAINT_FULL_THRESHOLD {
                self.set_full_damage_pending(true);
                self.set_full_damage_overflow(true);
                self.get_mut_extra_logical_damage().clear();
                return;
            }
            let children: Vec<NodeId> = node.get_children().clone();
            let raw_bounds: Rect = node.get_layout_box().get_bounds();
            if raw_bounds.get_width() > 0.0 && raw_bounds.get_height() > 0.0 {
                self.record_node_logical_damage(current);
                if self.get_full_damage_pending() {
                    return;
                }
            }
            for child in children.into_iter() {
                stack.push(child);
            }
        }
    }

    /// Starts or smoothly retargets a paint/transform transition on `transition.get_node()`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene tree.
    /// - `NodeTransition` - Transition specification to run.
    pub fn start_transition(&mut self, mut transition: NodeTransition) {
        let target: NodeId = transition.get_node();
        let had_active: bool = self
            .get_transitions()
            .iter()
            .any(|tr: &NodeTransition| tr.get_node() == target);
        self.get_mut_transitions()
            .retain(|tr: &NodeTransition| tr.get_node() != target);
        if had_active && let Some(node) = self.get_node(target) {
            let paint: PaintStyle = node.get_style().get_paint();
            let tr: Transform2D = paint.get_transform();
            let cur_op: f32 = paint.get_opacity();
            let cur_tx: f32 = tr.get_translate_x();
            let cur_ty: f32 = tr.get_translate_y();
            let cur_sc: f32 = tr.get_scale_x();
            let full_dist: f32 = (transition.get_to_tx() - transition.get_from_tx())
                .hypot(transition.get_to_ty() - transition.get_from_ty())
                .max((transition.get_to_opacity() - transition.get_from_opacity()).abs() * 100.0);
            let rem_dist: f32 = (transition.get_to_tx() - cur_tx)
                .hypot(transition.get_to_ty() - cur_ty)
                .max((transition.get_to_opacity() - cur_op).abs() * 100.0);
            if full_dist > 0.01 {
                let ratio: f32 = (rem_dist / full_dist).clamp(0.20, 1.0);
                transition.set_duration((transition.get_duration() * ratio).max(0.05));
            }
            transition.set_from_opacity(cur_op);
            transition.set_from_tx(cur_tx);
            transition.set_from_ty(cur_ty);
            transition.set_from_scale(cur_sc);
        }
        let (opacity, transform): (f32, Transform2D) = transition.sample();
        let mut unhidden: bool = false;
        if let Some(node) = self.get_node_mut(target) {
            let style: &mut Style = node.get_mut_style();
            if !transition.get_hide_on_complete()
                && style.layout_ref().get_display() == Display::None
            {
                style.get_mut_layout().set_display(Display::Flex);
                unhidden = true;
            }
            style.get_mut_paint().set_opacity(opacity);
            style.get_mut_paint().set_transform(transform);
        }
        if unhidden {
            let layout_dirty: u16 = DIRTY_LAYOUT | DIRTY_SUBTREE_LAYOUT;
            self.mark_dirty(target, layout_dirty | DIRTY_PAINT);
        } else {
            self.mark_dirty(target, DIRTY_PAINT);
        }
        self.get_mut_transitions().push(transition);
    }

    /// Advances all active transitions by `dt_secs` and returns `true` if any node updated.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene tree.
    /// - `f32` - Elapsed frame time in seconds.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether at least one transition ticked on this frame.
    pub fn tick_transitions(&mut self, dt_secs: f32) -> bool {
        let len: usize = self.get_transitions().len();
        if len == 0 {
            return false;
        }
        let step: f32 = dt_secs.clamp(0.001, 0.05);
        let mut any_updated: bool = false;
        let mut write_idx: usize = 0;
        for read_idx in 0..len {
            let target: NodeId = self.get_transitions()[read_idx].get_node();
            if self.get_node(target).is_none() {
                continue;
            }
            let (done, sample_opt, hide): (bool, Option<(f32, Transform2D)>, bool) = {
                let tr: &mut NodeTransition = &mut self.get_mut_transitions()[read_idx];
                let total_span: f32 = tr.get_delay() + tr.get_duration();
                let prev_elapsed: f32 = tr.get_elapsed();
                let next_elapsed: f32 = (prev_elapsed + step).min(total_span);
                tr.set_elapsed(next_elapsed);
                let is_done: bool = tr.is_complete();
                let in_hold: bool =
                    prev_elapsed > 0.0 && next_elapsed <= tr.get_delay() && !is_done;
                if in_hold {
                    (is_done, None, false)
                } else {
                    let sampled: (f32, Transform2D) = tr.sample();
                    let should_hide: bool = is_done && tr.get_hide_on_complete();
                    (is_done, Some(sampled), should_hide)
                }
            };
            if let Some((opacity, transform)) = sample_opt {
                if let Some(node) = self.get_node_mut(target) {
                    let style: &mut Style = node.get_mut_style();
                    style.get_mut_paint().set_opacity(opacity);
                    style.get_mut_paint().set_transform(transform);
                    if hide {
                        style.get_mut_layout().set_display(Display::None);
                    }
                }
                if hide {
                    self.clear_subtree_hover(target);
                    let layout_dirty: u16 = DIRTY_LAYOUT | DIRTY_SUBTREE_LAYOUT;
                    self.mark_dirty(target, layout_dirty | DIRTY_PAINT);
                } else {
                    self.mark_dirty(target, DIRTY_PAINT);
                }
                any_updated = true;
            }
            if !done {
                if write_idx != read_idx {
                    self.get_mut_transitions().swap(write_idx, read_idx);
                }
                write_idx += 1;
            }
        }
        self.get_mut_transitions().truncate(write_idx);
        any_updated
    }

    /// Clears transient hover/active flags throughout the subtree rooted at `root` when an overlay hides.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene tree.
    /// - `NodeId` - Root node being hidden.
    fn clear_subtree_hover(&mut self, root: NodeId) {
        let mut stack: Vec<NodeId> = vec![root];
        while let Some(id) = stack.pop() {
            let children_opt: Option<Vec<NodeId>> = match self.get_node_mut(id) {
                Some(node) => {
                    node.get_mut_interaction().set_hovered(false);
                    node.get_mut_interaction().set_pressed(false);
                    if node.get_children().is_empty() {
                        None
                    } else {
                        Some(node.get_children().clone())
                    }
                }
                None => continue,
            };
            if let Some(children) = children_opt {
                stack.extend(children);
            }
        }
    }

    /// Returns `true` while one or more node transitions are still running.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scene tree reference.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether animations are active.
    #[must_use]
    pub fn has_active_transitions(&self) -> bool {
        !self.get_transitions().is_empty()
    }

    /// Attaches a `Lifecycle` hook observer onto `id` for mount, activate, freeze, resume, unmount, and destroy dispatch.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene tree.
    /// - `NodeId` - Target scene node handle.
    /// - `Rc<dyn Lifecycle>` - Shared lifecycle hook observer.
    pub fn attach_lifecycle(&mut self, id: NodeId, hook: Rc<dyn Lifecycle>) {
        if let Some(node) = self.get_node_mut(id) {
            node.add_lifecycle(hook);
        }
    }

    /// Collects all `(NodeId, Rc<dyn Lifecycle>)` observers in the subtree rooted at `root`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scene tree reference.
    /// - `NodeId` - Subtree root handle.
    ///
    /// # Returns
    ///
    /// - `Vec<(NodeId, Rc<dyn Lifecycle>)>` - Ordered observers in pre-order traversal.
    #[must_use]
    pub fn collect_subtree_lifecycles(&self, root: NodeId) -> Vec<(NodeId, Rc<dyn Lifecycle>)> {
        let mut out: Vec<(NodeId, Rc<dyn Lifecycle>)> = Vec::new();
        let mut stack: Vec<NodeId> = vec![root];
        while let Some(current) = stack.pop() {
            if let Some(node) = self.get_node(current) {
                for hook in node.lifecycle_observers().into_iter() {
                    out.push((current, hook));
                }
                for child in node.get_children().iter().rev().copied() {
                    stack.push(child);
                }
            }
        }
        out
    }

    /// Collects all `Lifecycle` observers attached to visible (`Display != None`) nodes across the entire tree.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scene tree reference.
    ///
    /// # Returns
    ///
    /// - `Vec<(NodeId, Rc<dyn Lifecycle>)>` - Visible `(node, lifecycle)` observer pairs in pre-order.
    #[must_use]
    pub fn collect_visible_lifecycles(&self) -> Vec<(NodeId, Rc<dyn Lifecycle>)> {
        let mut out: Vec<(NodeId, Rc<dyn Lifecycle>)> = Vec::new();
        let mut stack: Vec<NodeId> = vec![self.get_root()];
        while let Some(current) = stack.pop() {
            if let Some(node) = self.get_node(current) {
                if node.style_ref().layout_ref().get_display() == Display::None {
                    continue;
                }
                for hook in node.lifecycle_observers().into_iter() {
                    out.push((current, hook));
                }
                for child in node.get_children().iter().rev().copied() {
                    stack.push(child);
                }
            }
        }
        out
    }

    /// Collects all reactive `ScopeId`s attached to nodes within the subtree rooted at `root`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scene tree reference.
    /// - `NodeId` - Subtree root handle.
    ///
    /// # Returns
    ///
    /// - `Vec<ScopeId>` - Owned scope identifiers discovered within `root`.
    #[must_use]
    pub fn collect_subtree_scope_ids(&self, root: NodeId) -> Vec<ScopeId> {
        let mut out: Vec<ScopeId> = Vec::new();
        let mut stack: Vec<NodeId> = vec![root];
        while let Some(current) = stack.pop() {
            if let Some(node) = self.get_node(current) {
                if let Some(scope) = node.owned_scope() {
                    out.push(scope.id());
                }
                for child in node.get_children().iter().rev().copied() {
                    stack.push(child);
                }
            }
        }
        out
    }

    /// Cancels any active paint/transform transitions targeting nodes within the subtree rooted at `root`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene tree.
    /// - `NodeId` - Subtree root handle.
    pub fn cancel_subtree_transitions(&mut self, root: NodeId) {
        if self.get_transitions().is_empty() {
            return;
        }
        let mut ids: HashSet<NodeId> = HashSet::new();
        let mut stack: Vec<NodeId> = vec![root];
        while let Some(current) = stack.pop() {
            if ids.insert(current)
                && let Some(node) = self.get_node(current)
            {
                for child in node.get_children().iter().copied() {
                    stack.push(child);
                }
            }
        }
        self.get_mut_transitions()
            .retain(|tr: &NodeTransition| !ids.contains(&tr.get_node()));
    }

    /// Attaches a reactive `Scope` to `id` so it automatically disposes when the node unmounts.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene tree.
    /// - `NodeId` - Target node handle.
    /// - `Scope` - Reactive ownership scope.
    pub fn attach_scope(&mut self, id: NodeId, scope: Scope) {
        if let Some(node) = self.get_node_mut(id) {
            if let Some(existing) = node.owned_scope() {
                existing.enter(|| {
                    on_cleanup(move || {
                        scope.dispose();
                    });
                });
            } else {
                node.set_scope(Some(scope));
            }
        }
    }

    /// Marks `id` dirty with `mask` and propagates subtree dirty bits to ancestors in O(depth).
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene tree.
    /// - `NodeId` - Mutated node handle.
    /// - `u16` - Dirty flag bits to set.
    pub fn mark_dirty(&mut self, id: NodeId, mask: u16) {
        let check_portal: bool = (mask & DIRTY_LAYOUT) != 0;
        let (prev_bits, mut parent_opt, portal_state): (u16, Option<NodeId>, Option<bool>) =
            match self.get_node_mut(id) {
                Some(node) => {
                    let prev: u16 = node.get_dirty().get_bits();
                    node.get_mut_dirty().insert(mask);
                    let portal_opt: Option<bool> = if check_portal {
                        let is_portal: bool = matches!(node.get_kind(), NodeKind::Portal)
                            || node.style_ref().get_layout().get_position() == Position::Fixed;
                        Some(is_portal)
                    } else {
                        None
                    };
                    (prev, node.parent_id(), portal_opt)
                }
                None => return,
            };
        if let Some(is_portal) = portal_state
            && (is_portal || self.has_portal_nodes())
        {
            self.sync_portal_registration(id, is_portal);
        }
        if (mask & DIRTY_FULL_DAMAGE_MASK) != 0 {
            self.set_layout_damage_pending(true);
        }
        let paint_or_scroll: u16 = DIRTY_PAINT | DIRTY_SCROLL;
        if !self.get_full_damage_pending()
            && (mask & paint_or_scroll) != 0
            && (prev_bits & paint_or_scroll) == 0
        {
            if self.get_dirty_paint_nodes().len() >= DIRTY_PAINT_FULL_THRESHOLD {
                self.set_full_damage_pending(true);
                self.set_full_damage_overflow(true);
                self.get_mut_dirty_paint_nodes().clear();
            } else {
                self.get_mut_dirty_paint_nodes().push(id);
            }
        }
        let mut ancestor_mask: u16 = DIRTY_NONE;
        if (mask & DIRTY_ANCESTOR_LAYOUT_TRIGGER) != 0 {
            ancestor_mask |= DIRTY_SUBTREE_LAYOUT;
        }
        if (mask & DIRTY_ANCESTOR_PAINT_TRIGGER) != 0 {
            ancestor_mask |= DIRTY_SUBTREE_PAINT;
        }
        while let Some(ancestor_id) = parent_opt {
            if let Some(ancestor) = self.get_node_mut(ancestor_id) {
                let already: bool =
                    (ancestor.get_dirty().get_bits() & ancestor_mask) == ancestor_mask;
                ancestor.get_mut_dirty().insert(ancestor_mask);
                if already {
                    break;
                }
                parent_opt = ancestor.parent_id();
            } else {
                break;
            }
        }
        let next_rev: u64 = self.get_revision().wrapping_add(1);
        self.set_revision(next_rev);
    }

    /// Returns `true` when `id` and every ancestor up to the root are alive, non-`Display::None`, and have positive opacity.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scene tree reference.
    /// - `NodeId` - Target node handle.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the node is visibly rendered in the scene.
    #[must_use]
    pub fn is_node_visible(&self, id: NodeId) -> bool {
        let mut cursor: Option<NodeId> = Some(id);
        while let Some(cur) = cursor {
            let Some(node) = self.get_node(cur) else {
                return false;
            };
            let style: &Style = node.style_ref();
            if style.get_layout().get_display() == Display::None
                || style.get_paint().get_opacity() <= 0.0
            {
                return false;
            }
            cursor = node.parent_id();
        }
        true
    }

    /// Collects the tight physical damage region from dirty paint/scroll nodes in O(dirty) and clears paint dirty flags.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene tree.
    /// - `f32` - Active display scale factor.
    ///
    /// # Returns
    ///
    /// - `DamageRegion` - Minimal damaged region, full damage on structural/layout change, or clean when idle.
    pub fn collect_paint_damage(&mut self, scale: f32) -> DamageRegion {
        let (damage, _): (DamageRegion, bool) = self.collect_surgical_paint_damage(scale, false);
        damage
    }

    /// Collects tight physical paint/scroll/removed-node damage while optionally deferring layout bounds diffing to `LayoutEngine`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene tree.
    /// - `f32` - Active display scale factor.
    /// - `bool` - When `true`, allows surgical pre/post-layout damage diffing instead of promoting all layout mutations to full redraw.
    ///
    /// # Returns
    ///
    /// - `(DamageRegion, bool)` - Collected damage region and whether a layout pass is still pending.
    pub(crate) fn collect_surgical_paint_damage(
        &mut self,
        scale: f32,
        allow_partial_layout: bool,
    ) -> (DamageRegion, bool) {
        let clear_mask: u16 = DIRTY_PAINT_CLEAR_MASK;
        let layout_pending: bool = self.get_layout_damage_pending();
        let no_paint_nodes: bool = self.get_dirty_paint_nodes().is_empty();
        let no_extra: bool = self.get_extra_logical_damage().is_empty();
        if !self.get_full_damage_pending() && !layout_pending && no_paint_nodes && no_extra {
            self.set_created_since_damage(0);
            return (DamageRegion::default(), false);
        }
        let mut full: bool =
            self.get_full_damage_pending() || (!allow_partial_layout && layout_pending);
        let mut region: DamageRegion = DamageRegion::default();
        if !full {
            let extra_len: usize = self.get_extra_logical_damage().len();
            for i in 0..extra_len {
                let logical_rect: Rect = self.get_extra_logical_damage()[i];
                if logical_rect.get_width() > 0.0 && logical_rect.get_height() > 0.0 {
                    region.add_rect(Self::scale_logical_to_phys_rect(logical_rect, scale));
                }
            }
            let dirty_len: usize = self.get_dirty_paint_nodes().len();
            for i in 0..dirty_len {
                let nid: NodeId = self.get_dirty_paint_nodes()[i];
                let Some(node) = self.get_node(nid) else {
                    continue;
                };
                let bits: u16 = node.get_dirty().get_bits();
                let has_layout_bits: bool = (bits & DIRTY_LAYOUT_PASS_MASK) != 0;
                if !allow_partial_layout && (bits & DIRTY_FULL_DAMAGE_MASK) != 0 {
                    full = true;
                    break;
                }
                if (bits & DIRTY_SCROLL) != 0 && has_layout_bits {
                    full = true;
                    break;
                }
                if (bits & (DIRTY_PAINT | DIRTY_SCROLL)) == 0 {
                    continue;
                }
                let raw_bounds: Rect = node.get_layout_box().get_bounds();
                if raw_bounds.get_width() > 0.0 && raw_bounds.get_height() > 0.0 {
                    match self.node_conservative_logical_rect(nid) {
                        Some(logical_rect) => {
                            region.add_rect(Self::scale_logical_to_phys_rect(logical_rect, scale));
                        }
                        None => {
                            full = true;
                            break;
                        }
                    }
                } else if (bits & DIRTY_PAINT) != 0
                    && !has_layout_bits
                    && node.style_ref().layout_ref().get_display() != Display::None
                {
                    full = true;
                    break;
                }
            }
        }
        self.set_layout_damage_pending(false);
        self.set_created_since_damage(0);
        self.get_mut_extra_logical_damage().clear();
        if !full {
            let dirty_len: usize = self.get_dirty_paint_nodes().len();
            for i in 0..dirty_len {
                let nid: NodeId = self.get_dirty_paint_nodes()[i];
                let mut parent_opt: Option<NodeId> = match self.get_node_mut(nid) {
                    Some(node) => {
                        node.get_mut_dirty().remove(clear_mask);
                        node.parent_id()
                    }
                    None => None,
                };
                while let Some(ancestor_id) = parent_opt {
                    if let Some(ancestor) = self.get_node_mut(ancestor_id) {
                        if (ancestor.get_dirty().get_bits() & DIRTY_SUBTREE_PAINT) == 0 {
                            break;
                        }
                        ancestor.get_mut_dirty().remove(DIRTY_SUBTREE_PAINT);
                        parent_opt = ancestor.parent_id();
                    } else {
                        break;
                    }
                }
            }
            self.get_mut_dirty_paint_nodes().clear();
            return (region, layout_pending);
        }
        self.set_full_damage_pending(false);
        self.get_mut_dirty_paint_nodes().clear();
        for node in self.get_mut_nodes().iter_mut() {
            if node.get_alive() && (node.get_dirty().get_bits() & clear_mask) != 0 {
                node.get_mut_dirty().remove(clear_mask);
            }
        }
        (DamageRegion::full(), layout_pending)
    }

    /// Consumes and returns whether the most recent dirty-paint accumulation overflowed `DIRTY_PAINT_FULL_THRESHOLD`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene tree.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if dirty paint nodes overflowed into full damage.
    pub(crate) fn take_full_damage_overflow(&mut self) -> bool {
        let was_overflow: bool = self.get_full_damage_overflow();
        self.set_full_damage_overflow(false);
        was_overflow
    }

    /// Recursively detaches and recycles `id`, returning all owned `Scope`s and `Lifecycle` observers for borrow-safe finalization.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene tree.
    /// - `NodeId` - Subtree root handle to dispose.
    ///
    /// # Returns
    ///
    /// - `DisposedSubtreePayload` - Collected scopes and lifecycle observers to finalize outside any borrow.
    pub fn dispose_subtree_collect(&mut self, id: NodeId) -> DisposedSubtreePayload {
        if id == self.get_root() {
            return (Vec::new(), Vec::new());
        }
        self.record_subtree_logical_damage(id);
        self.cancel_subtree_transitions(id);
        let mut stack: Vec<NodeId> = vec![id];
        let mut scopes: Vec<Scope> = Vec::new();
        let mut observers: Vec<(NodeId, Rc<dyn Lifecycle>)> = Vec::new();
        while let Some(current) = stack.pop() {
            let children_opt: Option<Vec<NodeId>> = match self.get_node_mut(current) {
                Some(node) => {
                    node.set_alive(false);
                    if node.has_extras() {
                        if let Some(scope) = node.owned_scope() {
                            scopes.push(scope);
                        }
                        for hook in node.take_lifecycles().into_iter() {
                            observers.push((current, hook));
                        }
                        node.clear_extras();
                    }
                    node.set_parent(None);
                    let next_gen: u32 = node.get_generation().wrapping_add(1).max(1);
                    node.set_generation(next_gen);
                    Some(take(node.get_mut_children()))
                }
                None => None,
            };
            let Some(children) = children_opt else {
                continue;
            };
            let next_live: usize = self.get_live_count().saturating_sub(1);
            self.set_live_count(next_live);
            if !self.portal_nodes().is_empty() {
                self.sync_portal_registration(current, false);
            }
            if self
                .text_selection()
                .map(|sel: SelectionState| sel.get_node_id())
                == Some(current)
            {
                self.set_text_selection(None);
            }
            self.get_mut_free_list().push(current.get_index());
            for child in children.into_iter() {
                stack.push(child);
            }
        }
        (scopes, observers)
    }

    /// Recursively removes and disposes `id`, recycling all arena slots, running lifecycle hooks, and disposing scopes.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene tree.
    /// - `NodeId` - Subtree root handle to dispose.
    pub fn dispose_subtree(&mut self, id: NodeId) {
        let (scopes, observers): DisposedSubtreePayload = self.dispose_subtree_collect(id);
        finalize_disposed_subtree(scopes, observers);
    }

    /// Updates keyboard focus to `target`, emitting `Blur` and `Focus` events.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene tree.
    /// - `Option<NodeId>` - Newly focused node, or `None` to clear focus.
    ///
    /// # Returns
    ///
    /// - `Vec<(UiEventHandler, UiEvent)>` - Callbacks to invoke outside the tree borrow.
    pub fn update_focus(&mut self, target: Option<NodeId>) -> Vec<(UiEventHandler, UiEvent)> {
        let prev: Option<NodeId> = self.get_focus().focused_node();
        if prev == target {
            return Vec::new();
        }
        let mut callbacks: Vec<(UiEventHandler, UiEvent)> = Vec::new();
        if let Some(old_id) = prev {
            if let Some(old_node) = self.get_node_mut(old_id) {
                old_node.get_mut_interaction().set_focused(false);
                if let Some(handler) = old_node.get_handlers().handler_for(&UiEventKind::Blur) {
                    callbacks.push((handler, UiEvent::new(old_id, UiEventKind::Blur)));
                }
            }
            self.mark_dirty(old_id, DIRTY_PAINT);
        }
        self.get_mut_focus().set_focused(target);
        if let Some(new_id) = target {
            if let Some(new_node) = self.get_node_mut(new_id) {
                new_node.get_mut_interaction().set_focused(true);
                if let Some(handler) = new_node.get_handlers().handler_for(&UiEventKind::Focus) {
                    callbacks.push((handler, UiEvent::new(new_id, UiEventKind::Focus)));
                }
            }
            self.mark_dirty(new_id, DIRTY_PAINT);
        }
        callbacks
    }

    /// Resolves whether `id` allows interactive text selection, walking ancestors unless an explicit opt-out is encountered.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scene tree reference.
    /// - `NodeId` - Target scene node handle.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when `id` or an ancestor enables `selectable` without an intervening explicit `selectable(false)`.
    #[must_use]
    pub fn is_node_selectable(&self, id: NodeId) -> bool {
        let mut cursor: Option<NodeId> = Some(id);
        while let Some(cur) = cursor {
            let Some(node) = self.get_node(cur) else {
                return false;
            };
            let text_style: &TextStyle = node.style_ref().text_ref();
            if text_style.get_selectable() {
                return true;
            }
            if text_style.get_selectable_explicit() {
                return false;
            }
            cursor = node.parent_id();
        }
        false
    }

    /// Resolves the effective text selection highlight color for `id`, inheriting from ancestors or falling back to default tint.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scene tree reference.
    /// - `NodeId` - Target text node handle.
    ///
    /// # Returns
    ///
    /// - `Color` - Selection highlight fill color.
    #[must_use]
    pub fn effective_selection_color(&self, id: NodeId) -> Color {
        let mut cursor: Option<NodeId> = Some(id);
        while let Some(cur) = cursor {
            let Some(node) = self.get_node(cur) else {
                break;
            };
            if let Some(color) = node.style_ref().text_ref().selection_color() {
                return color;
            }
            cursor = node.parent_id();
        }
        DEFAULT_TEXT_SELECTION_COLOR
    }

    /// Returns the active text selection state, if any.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scene tree reference.
    ///
    /// # Returns
    ///
    /// - `Option<SelectionState>` - Current selection state.
    #[must_use]
    pub fn text_selection(&self) -> Option<SelectionState> {
        let Self { text_selection, .. } = self;
        *text_selection
    }

    /// Updates the active text selection state and marks affected text nodes dirty for repaint.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene tree.
    /// - `Option<SelectionState>` - Replacement selection state (`None` clears selection).
    pub fn set_text_selection(&mut self, selection: Option<SelectionState>) {
        let prev: Option<SelectionState> = self.text_selection();
        if prev == selection {
            return;
        }
        let Self { text_selection, .. } = self;
        *text_selection = selection;
        if let Some(old_sel) = prev {
            self.mark_dirty(old_sel.get_node_id(), DIRTY_PAINT);
        }
        if let Some(new_sel) = selection {
            self.mark_dirty(new_sel.get_node_id(), DIRTY_PAINT);
        }
    }

    /// Clears any active text selection and repaints the previously selected node.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene tree.
    pub fn clear_text_selection(&mut self) {
        self.set_text_selection(None);
    }

    /// Returns the currently selected UTF-8 substring when a non-empty selection exists on a live text node.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scene tree reference.
    ///
    /// # Returns
    ///
    /// - `Option<String>` - Selected text string, or `None` when no text is selected.
    #[must_use]
    pub fn get_selected_text(&self) -> Option<String> {
        let sel: SelectionState = self.text_selection()?;
        if sel.is_empty() {
            return None;
        }
        let node: &SceneNode = self.get_node(sel.get_node_id())?;
        let NodeKind::Text(content) = node.get_kind() else {
            return None;
        };
        let extracted: &str = sel.extract_from(content.as_str());
        if extracted.is_empty() {
            None
        } else {
            Some(extracted.to_string())
        }
    }

    /// Selects the entire character range of selectable `NodeKind::Text` node `id`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene tree.
    /// - `NodeId` - Target text node handle.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when `id` is a live selectable non-empty text node and selection was applied.
    pub fn select_all_text(&mut self, id: NodeId) -> bool {
        if !self.is_node_selectable(id) {
            return false;
        }
        let char_len: Option<usize> = self.get_node(id).and_then(|node: &SceneNode| {
            if let NodeKind::Text(content) = node.get_kind() {
                Some(content.chars().count())
            } else {
                None
            }
        });
        let Some(len) = char_len else {
            return false;
        };
        if len == 0 {
            return false;
        }
        self.set_text_selection(Some(SelectionState::new(id, 0, len)));
        true
    }
}

/// Shared scene handle operations enabling surgical reactive node updates.
impl SceneHandle {
    /// Creates a new `SceneHandle` wrapping a fresh `SceneTree`.
    ///
    /// # Returns
    ///
    /// - `Self` - Shared scene handle.
    #[must_use]
    pub fn new_tree() -> Self {
        Self {
            inner: Rc::new(RefCell::new(SceneTree::default())),
        }
    }

    /// Updates virtualized scroll metrics `(scroll_y, content_height)` on `id` for accurate scrollbar thumbs.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scene handle reference.
    /// - `NodeId` - Target virtualized scroll container.
    /// - `f32` - Active virtual vertical scroll offset in logical pixels.
    /// - `f32` - Total virtualized content height in logical pixels.
    pub fn set_virtual_scroll(&self, id: NodeId, scroll_y: f32, content_height: f32) {
        self.with_write(
            |tree: &mut SceneTree| {
                tree.set_virtual_scroll(id, scroll_y, content_height);
            },
            (),
        );
    }

    /// Computes screen-space logical bounds for `id` subtracting ancestor scroll offsets.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scene handle reference.
    /// - `NodeId` - Target node handle.
    ///
    /// # Returns
    ///
    /// - `Option<Rect>` - Screen-space bounds if the node is alive.
    #[must_use]
    pub fn node_screen_bounds(&self, id: NodeId) -> Option<Rect> {
        self.with_read(|tree: &SceneTree| tree.node_screen_bounds(id), None)
    }

    /// Starts or replaces a smooth paint/transform transition on the target node.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scene handle reference.
    /// - `NodeTransition` - Transition specification to run.
    pub fn start_transition(&self, transition: NodeTransition) {
        self.with_write(
            |tree: &mut SceneTree| {
                tree.start_transition(transition);
            },
            (),
        );
    }

    /// Marks `id` dirty with `mask` for incremental repainting.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scene handle reference.
    /// - `NodeId` - Target node handle.
    /// - `u16` - Dirty flag bits.
    pub fn mark_dirty(&self, id: NodeId, mask: u16) {
        self.with_write(
            |tree: &mut SceneTree| {
                tree.mark_dirty(id, mask);
            },
            (),
        );
    }

    /// Replaces all children of `parent` and finalizes disposed scopes and `Lifecycle` hooks outside the tree borrow.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scene handle reference.
    /// - `NodeId` - Parent container node.
    /// - `Vec<NodeId>` - Replacement ordered children.
    pub fn replace_children_safe(&self, parent: NodeId, new_children: Vec<NodeId>) {
        let (scopes, observers): DisposedSubtreePayload = self.with_write(
            |tree: &mut SceneTree| {
                let old_children: Vec<NodeId> = match tree.get_node_mut(parent) {
                    Some(node) => take(node.get_mut_children()),
                    None => return (Vec::new(), Vec::new()),
                };
                let mut all_scopes: Vec<Scope> = Vec::new();
                let mut all_obs: Vec<(NodeId, Rc<dyn Lifecycle>)> = Vec::new();
                if new_children.is_empty() {
                    for old_child in old_children.into_iter() {
                        let (sc, ob): DisposedSubtreePayload =
                            tree.dispose_subtree_collect(old_child);
                        all_scopes.extend(sc);
                        all_obs.extend(ob);
                    }
                } else if old_children.len() > 16 && new_children.len() > 16 {
                    let keep_set: HashSet<NodeId> = new_children.iter().copied().collect();
                    for old_child in old_children.into_iter() {
                        if !keep_set.contains(&old_child) {
                            let (sc, ob): DisposedSubtreePayload =
                                tree.dispose_subtree_collect(old_child);
                            all_scopes.extend(sc);
                            all_obs.extend(ob);
                        }
                    }
                } else {
                    for old_child in old_children.into_iter() {
                        if !new_children.contains(&old_child) {
                            let (sc, ob): DisposedSubtreePayload =
                                tree.dispose_subtree_collect(old_child);
                            all_scopes.extend(sc);
                            all_obs.extend(ob);
                        }
                    }
                }
                for child in new_children.iter().copied() {
                    if let Some(child_node) = tree.get_node_mut(child) {
                        child_node.set_parent(Some(parent));
                    }
                }
                if let Some(parent_node) = tree.get_node_mut(parent) {
                    parent_node.set_children(new_children);
                }
                tree.mark_dirty(parent, DIRTY_CHILDREN_CHANGE_MASK);
                (all_scopes, all_obs)
            },
            (Vec::new(), Vec::new()),
        );
        finalize_disposed_subtree(scopes, observers);
    }

    /// Removes `child` from `parent`, disposes its subtree, and invokes `on_unmount`/`on_destroy` outside any scene borrow.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scene handle reference.
    /// - `NodeId` - Parent container handle.
    /// - `NodeId` - Child subtree root handle to remove and destroy.
    pub fn remove_and_dispose_child(&self, parent: NodeId, child: NodeId) {
        let (scopes, observers): DisposedSubtreePayload = self.with_write(
            |tree: &mut SceneTree| {
                if let Some(pnode) = tree.get_node_mut(parent) {
                    pnode.get_mut_children().retain(|id: &NodeId| *id != child);
                }
                let out: DisposedSubtreePayload = tree.dispose_subtree_collect(child);
                tree.mark_dirty(parent, DIRTY_CHILDREN_CHANGE_MASK);
                out
            },
            (Vec::new(), Vec::new()),
        );
        finalize_disposed_subtree(scopes, observers);
    }

    /// Mutates a node's `Style` in place under one write lock, marking only minimal dirty flags when changed.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scene handle reference.
    /// - `NodeId` - Target node handle.
    /// - `F` - In-place style mutator closure receiving `&mut Style`.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the node existed and its style changed.
    pub fn mutate_style<F>(&self, id: NodeId, mutator: F) -> bool
    where
        F: FnOnce(&mut Style),
    {
        self.with_write(|tree: &mut SceneTree| tree.mutate_style(id, mutator), false)
    }

    /// Freezes a subtree visually (`Display::None`), cancels its transitions, and dispatches `Lifecycle::on_freeze` outside any scene borrow.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scene handle reference.
    /// - `NodeId` - Subtree root handle to freeze.
    pub fn freeze_subtree(&self, root: NodeId) {
        let observers: Vec<(NodeId, Rc<dyn Lifecycle>)> = self.with_write(
            |tree: &mut SceneTree| {
                let obs: Vec<(NodeId, Rc<dyn Lifecycle>)> = tree.collect_subtree_lifecycles(root);
                tree.cancel_subtree_transitions(root);
                let changed: bool = if let Some(node) = tree.get_node_mut(root) {
                    let layout: &mut LayoutStyle = node.get_mut_style().get_mut_layout();
                    if layout.get_display() == Display::None {
                        false
                    } else {
                        layout.set_display(Display::None);
                        true
                    }
                } else {
                    false
                };
                if changed {
                    tree.mark_dirty(root, DIRTY_LAYOUT | DIRTY_PAINT);
                }
                obs
            },
            Vec::new(),
        );
        dispatch_lifecycle_freeze(&observers);
    }

    /// Freezes a subtree immediately for lifecycle/effects while animating its visual exit to `Display::None`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scene handle reference.
    /// - `NodeId` - Subtree root handle to freeze and animate out.
    /// - `NodeTransition` - Exit transition specification (`hide_on_complete` is enforced).
    pub fn freeze_subtree_animated(&self, root: NodeId, exit_transition: NodeTransition) {
        let observers: Vec<(NodeId, Rc<dyn Lifecycle>)> = self.with_write(
            |tree: &mut SceneTree| {
                let obs: Vec<(NodeId, Rc<dyn Lifecycle>)> = tree.collect_subtree_lifecycles(root);
                tree.cancel_subtree_transitions(root);
                let mut demoted_fixed: bool = false;
                if let Some(node) = tree.get_node_mut(root) {
                    let is_portal: bool = matches!(node.get_kind(), NodeKind::Portal);
                    let st: &mut Style = node.get_mut_style();
                    demoted_fixed = !is_portal && st.layout_ref().get_position() == Position::Fixed;
                    st.get_mut_layout()
                        .set_position(Position::Absolute)
                        .set_top(Some(0.0))
                        .set_right(Some(0.0))
                        .set_bottom(None)
                        .set_left(Some(0.0))
                        .set_z_index(0);
                    st.get_mut_paint().set_pointer_events(PointerEvents::None);
                }
                if demoted_fixed {
                    tree.sync_portal_registration(root, false);
                }
                tree.start_transition(exit_transition.with_hide_on_complete(true));
                tree.mark_dirty(root, DIRTY_LAYOUT | DIRTY_PAINT);
                obs
            },
            Vec::new(),
        );
        dispatch_lifecycle_freeze(&observers);
    }

    /// Resumes a previously frozen subtree (`Display::Flex`) and dispatches `Lifecycle::on_resume` outside any scene borrow.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scene handle reference.
    /// - `NodeId` - Subtree root handle to resume.
    pub fn resume_subtree(&self, root: NodeId) {
        let observers: Vec<(NodeId, Rc<dyn Lifecycle>)> = self.with_write(
            |tree: &mut SceneTree| {
                let mut unhidden: bool = false;
                if let Some(node) = tree.get_node_mut(root) {
                    let layout: &mut LayoutStyle = node.get_mut_style().get_mut_layout();
                    if layout.get_display() == Display::None {
                        layout.set_display(Display::Flex);
                        unhidden = true;
                    }
                }
                if unhidden {
                    tree.mark_dirty(root, DIRTY_LAYOUT | DIRTY_PAINT);
                } else {
                    tree.mark_dirty(root, DIRTY_PAINT);
                }
                tree.collect_subtree_lifecycles(root)
            },
            Vec::new(),
        );
        dispatch_lifecycle_resume(&observers);
    }

    /// Dispatches `Lifecycle::on_activate` across all observers in the subtree rooted at `root`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scene handle reference.
    /// - `NodeId` - Subtree root handle becoming active.
    pub fn activate_subtree(&self, root: NodeId) {
        let observers: Vec<(NodeId, Rc<dyn Lifecycle>)> = self.with_read(
            |tree: &SceneTree| tree.collect_subtree_lifecycles(root),
            Vec::new(),
        );
        dispatch_lifecycle_activate(&observers);
    }

    /// Dispatches `Lifecycle::on_update` across all observers in the subtree rooted at `root`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scene handle reference.
    /// - `NodeId` - Subtree root handle receiving updates.
    pub fn update_subtree(&self, root: NodeId) {
        let observers: Vec<(NodeId, Rc<dyn Lifecycle>)> = self.with_read(
            |tree: &SceneTree| tree.collect_subtree_lifecycles(root),
            Vec::new(),
        );
        dispatch_lifecycle_update(&observers);
    }

    /// Dispatches `SystemEvent::Appear` to all `Lifecycle` observers and scoped hooks owned by `root`'s subtree.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scene handle reference.
    /// - `NodeId` - Page or component subtree root handle becoming visible.
    pub fn emit_subtree_appear(&self, root: NodeId) {
        let (observers, scopes): SubtreeLifecycleTargets = self.with_read(
            |tree: &SceneTree| {
                (
                    tree.collect_subtree_lifecycles(root),
                    tree.collect_subtree_scope_ids(root),
                )
            },
            (Vec::new(), Vec::new()),
        );
        let ev: SystemEvent = SystemEvent::Appear;
        dispatch_lifecycle_system_event(&observers, &ev);
        dispatch_system_event_to_scopes(&scopes, &ev);
    }

    /// Dispatches `SystemEvent::Disappear` to all `Lifecycle` observers and scoped hooks owned by `root`'s subtree.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scene handle reference.
    /// - `NodeId` - Page or component subtree root handle ceasing to be visible.
    pub fn emit_subtree_disappear(&self, root: NodeId) {
        let (observers, scopes): SubtreeLifecycleTargets = self.with_read(
            |tree: &SceneTree| {
                (
                    tree.collect_subtree_lifecycles(root),
                    tree.collect_subtree_scope_ids(root),
                )
            },
            (Vec::new(), Vec::new()),
        );
        let ev: SystemEvent = SystemEvent::Disappear;
        dispatch_lifecycle_system_event(&observers, &ev);
        dispatch_system_event_to_scopes(&scopes, &ev);
    }

    /// Returns `true` if any node transition is currently active.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scene handle reference.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether animations are running.
    #[must_use]
    pub fn has_active_transitions(&self) -> bool {
        self.with_read(SceneTree::has_active_transitions, false)
    }

    /// Returns the currently selected text substring across the retained scene, if any.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scene handle reference.
    ///
    /// # Returns
    ///
    /// - `Option<String>` - Active selected UTF-8 text.
    #[must_use]
    pub fn get_selected_text(&self) -> Option<String> {
        self.with_read(SceneTree::get_selected_text, None)
    }

    /// Clears any active text selection across the retained scene.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scene handle reference.
    pub fn clear_text_selection(&self) {
        self.with_write(SceneTree::clear_text_selection, ());
    }

    /// Executes `operation` with immutable access to the underlying `SceneTree`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scene handle.
    /// - `F` - Reader closure.
    /// - `R` - Fallback if re-entrantly borrowed.
    ///
    /// # Returns
    ///
    /// - `R` - Closure result or fallback.
    pub fn with_read<F, R>(&self, operation: F, fallback: R) -> R
    where
        F: FnOnce(&SceneTree) -> R,
    {
        let Self { inner } = self;
        match inner.try_borrow() {
            Ok(guard) => operation(&guard),
            Err(_) => fallback,
        }
    }

    /// Executes `operation` with mutable access to the underlying `SceneTree`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scene handle.
    /// - `F` - Mutation closure.
    /// - `R` - Fallback if re-entrantly borrowed.
    ///
    /// # Returns
    ///
    /// - `R` - Closure result or fallback.
    pub fn with_write<F, R>(&self, operation: F, fallback: R) -> R
    where
        F: FnOnce(&mut SceneTree) -> R,
    {
        let Self { inner } = self;
        match inner.try_borrow_mut() {
            Ok(mut guard) => operation(&mut guard),
            Err(_) => fallback,
        }
    }

    /// Binds a reactive text expression to `node_id` via a surgical `Effect`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scene handle.
    /// - `NodeId` - Target text node.
    /// - `F` - Reactive text producer closure.
    ///
    /// # Returns
    ///
    /// - `Effect` - Registered reactive effect.
    pub fn bind_text<F>(&self, node_id: NodeId, mut producer: F) -> Effect
    where
        F: FnMut() -> String + 'static,
    {
        let handle: Self = self.clone();
        Effect::new(move || {
            let value: String = producer();
            handle.with_write(
                |tree: &mut SceneTree| {
                    tree.set_text(node_id, value);
                },
                (),
            );
        })
    }

    /// Binds a reactive `Style` expression to `node_id` via a surgical `Effect`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scene handle.
    /// - `NodeId` - Target node.
    /// - `F` - Reactive style producer closure.
    ///
    /// # Returns
    ///
    /// - `Effect` - Registered reactive effect.
    pub fn bind_style<F>(&self, node_id: NodeId, mut producer: F) -> Effect
    where
        F: FnMut() -> Style + 'static,
    {
        let handle: Self = self.clone();
        Effect::new(move || {
            let style: Style = producer();
            handle.with_write(
                |tree: &mut SceneTree| {
                    tree.set_style(node_id, style);
                },
                (),
            );
        })
    }
}

/// High-level declarative building helpers on `Context`.
impl Context {
    /// Creates a new `Context` with reactive viewport, theme, and safe-area signals.
    ///
    /// # Arguments
    ///
    /// - `SceneHandle` - Shared scene handle.
    /// - `PlatformKind` - Active platform kind.
    ///
    /// # Returns
    ///
    /// - `Self` - Constructed build context.
    #[must_use]
    pub fn new(scene: SceneHandle, platform: PlatformKind) -> Self {
        Self {
            scene,
            breakpoint: Signal::new(Breakpoint::Expanded),
            viewport_size: Signal::new(Size::new(1200.0, 800.0)),
            theme_mode: Signal::new(ThemeMode::Light),
            platform,
            safe_area: Signal::new(SafeAreaInsets::default()),
            gesture: Signal::new(None),
            swipe_drag_offset: Signal::new(0.0),
            system_back: Signal::new(0),
            dynamic_hub: DynamicUpdateHub::new(),
            fps_limit: Signal::new(None),
            live_fps: Signal::new(60.0),
            present_fps: Signal::new(0.0),
            is_idle: Signal::new(true),
            display_refresh_rate: Signal::new(60.0),
            active_touches: Signal::new(Vec::new()),
            exit_requested: Signal::new(false),
            app_state: Signal::new(AppState::Active),
            network_status: Signal::new(NetworkStatus::Online),
            keyboard_visible: Signal::new(false),
            keyboard_height: Signal::new(0.0),
            orientation: Signal::new(ScreenOrientation::Landscape),
            low_power_mode: Signal::new(false),
            locale: Signal::new(DEFAULT_SYSTEM_LOCALE.to_string()),
            window_focused: Signal::new(true),
            clock_ms: Rc::new(Cell::new(0)),
        }
    }

    /// Returns the reactive `Signal<AppState>` tracking canonical cross-platform application lifecycle state.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Context reference.
    ///
    /// # Returns
    ///
    /// - `Signal<AppState>` - Reactive lifecycle state signal.
    #[must_use]
    pub fn app_state_signal(&self) -> Signal<AppState> {
        self.get_app_state()
    }

    /// Dispatches a page-level `SystemEvent::Appear` to all currently active non-frozen hooks and visible observers.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Context reference.
    pub fn emit_page_appear(&self) {
        let ev: SystemEvent = SystemEvent::Appear;
        let observers: Vec<(NodeId, Rc<dyn Lifecycle>)> = self
            .get_scene()
            .with_read(SceneTree::collect_visible_lifecycles, Vec::new());
        dispatch_lifecycle_system_event(&observers, &ev);
        dispatch_system_event_listeners(&ev);
    }

    /// Dispatches a page-level `SystemEvent::Disappear` to all currently active non-frozen hooks and visible observers.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Context reference.
    pub fn emit_page_disappear(&self) {
        let ev: SystemEvent = SystemEvent::Disappear;
        let observers: Vec<(NodeId, Rc<dyn Lifecycle>)> = self
            .get_scene()
            .with_read(SceneTree::collect_visible_lifecycles, Vec::new());
        dispatch_lifecycle_system_event(&observers, &ev);
        dispatch_system_event_listeners(&ev);
    }

    /// Dispatches `SystemEvent::Appear` strictly to `Lifecycle` components and scoped hooks within `root`'s subtree.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Context reference.
    /// - `NodeId` - Routed page or component subtree root handle becoming active.
    pub fn emit_subtree_appear(&self, root: NodeId) {
        self.get_scene().emit_subtree_appear(root);
    }

    /// Dispatches `SystemEvent::Disappear` strictly to `Lifecycle` components and scoped hooks within `root`'s subtree.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Context reference.
    /// - `NodeId` - Routed page or component subtree root handle becoming hidden.
    pub fn emit_subtree_disappear(&self, root: NodeId) {
        self.get_scene().emit_subtree_disappear(root);
    }

    /// Returns the monotonic runtime elapsed clock in milliseconds.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Context reference.
    ///
    /// # Returns
    ///
    /// - `u64` - Elapsed runtime time in milliseconds.
    #[must_use]
    pub fn elapsed_ms(&self) -> u64 {
        let Self { clock_ms, .. } = self;
        clock_ms.get()
    }

    /// Advances the context's monotonic runtime clock by `delta_ms` milliseconds.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Context reference.
    /// - `u64` - Milliseconds elapsed since the previous frame.
    pub(crate) fn advance_clock_ms(&self, delta_ms: u64) {
        let Self { clock_ms, .. } = self;
        clock_ms.set(clock_ms.get().saturating_add(delta_ms));
    }

    /// Requests graceful OS-level application termination across all platforms.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Context reference.
    pub fn request_exit(&self) {
        let _: bool = self.get_exit_requested().set_if_changed(true);
        let _: bool = dispatch_native_action(NATIVE_ACTION_EXIT, "", "");
    }

    /// Copies `text` to the host OS or browser clipboard via native clipboard hooks.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Context reference.
    /// - `&str` - UTF-8 text string to copy.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when a native or host clipboard bridge accepted `text`.
    pub fn copy_to_clipboard(&self, text: &str) -> bool {
        write_system_clipboard(text)
    }

    /// Creates a `View` node with the given style.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Build context.
    /// - `Style` - Initial style.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Created node handle.
    pub fn view(&mut self, style: Style) -> NodeId {
        self.get_scene().with_write(
            |tree: &mut SceneTree| tree.create_view(style),
            NodeId::default(),
        )
    }

    /// Creates a `Text` node with static initial text.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Build context.
    /// - `String` - Text content.
    /// - `Style` - Text style.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Created node handle.
    pub fn text(&mut self, content: String, style: Style) -> NodeId {
        self.get_scene().with_write(
            |tree: &mut SceneTree| tree.create_text(content, style),
            NodeId::default(),
        )
    }

    /// Creates a reactive `Text` node whose content updates surgically whenever tracked signals change.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Build context.
    /// - `Style` - Text style.
    /// - `F` - Reactive text producer.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Created reactive text node handle.
    pub fn reactive_text<F>(&mut self, style: Style, producer: F) -> NodeId
    where
        F: FnMut() -> String + 'static,
    {
        let node_id: NodeId = self.text(String::new(), style);
        let handle: SceneHandle = self.get_scene().clone();
        let (scope, _): (Scope, Effect) =
            Scope::run(|_: Scope| handle.bind_text(node_id, producer));
        self.get_scene().with_write(
            |tree: &mut SceneTree| {
                tree.attach_scope(node_id, scope);
            },
            (),
        );
        node_id
    }

    /// Creates a `ScrollView` node with the given style.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Build context.
    /// - `Style` - Scroll container style.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Created scroll view handle.
    pub fn scroll_view(&mut self, style: Style) -> NodeId {
        self.get_scene().with_write(
            |tree: &mut SceneTree| tree.create_scroll_view(style),
            NodeId::default(),
        )
    }

    /// Creates an `Input` node with initial state and style.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Build context.
    /// - `InputNodeState` - Initial input state.
    /// - `Style` - Input style.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Created input handle.
    pub fn input(&mut self, state: InputNodeState, style: Style) -> NodeId {
        self.get_scene().with_write(
            |tree: &mut SceneTree| tree.create_input(state, style),
            NodeId::default(),
        )
    }

    /// Creates an `Image` node with decoded RGBA8 pixel data.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Build context.
    /// - `ImageBuffer` - Image buffer.
    /// - `Style` - Image style.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Created image handle.
    pub fn image(&mut self, buffer: ImageBuffer, style: Style) -> NodeId {
        self.get_scene().with_write(
            |tree: &mut SceneTree| tree.create_image(buffer, style),
            NodeId::default(),
        )
    }

    /// Creates a `Canvas` node bound to a custom draw callback.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Build context.
    /// - `Style` - Canvas style.
    /// - `F` - Draw callback receiving node bounds and mutable `DisplayList`.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Created canvas handle.
    pub fn canvas<F>(&mut self, style: Style, painter: F) -> NodeId
    where
        F: FnMut(Rect, &mut DisplayList) + 'static,
    {
        let cb: CanvasDrawCallback = Rc::new(RefCell::new(painter));
        self.get_scene().with_write(
            |tree: &mut SceneTree| tree.create_canvas(style, cb),
            NodeId::default(),
        )
    }

    /// Creates an overlay `Portal` node.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Build context.
    /// - `Style` - Portal style.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Created portal handle.
    pub fn portal(&mut self, style: Style) -> NodeId {
        self.get_scene().with_write(
            |tree: &mut SceneTree| tree.create_portal(style),
            NodeId::default(),
        )
    }

    /// Creates a transparent logical `Fragment` grouping container for sibling nodes.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Created fragment handle.
    pub fn fragment(&mut self) -> NodeId {
        self.get_scene().with_write(
            |tree: &mut SceneTree| tree.create_fragment(),
            NodeId::default(),
        )
    }

    /// Appends `child` to `parent`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Build context.
    /// - `NodeId` - Parent container.
    /// - `NodeId` - Child node.
    pub fn append(&mut self, parent: NodeId, child: NodeId) {
        self.get_scene().with_write(
            |tree: &mut SceneTree| {
                tree.append_child(parent, child);
            },
            (),
        );
    }

    /// Appends a slice of `children` to `parent` under a single scene write lock and dirty pass.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Build context.
    /// - `NodeId` - Parent container.
    /// - `&[NodeId]` - Ordered child node slice.
    pub fn append_many(&mut self, parent: NodeId, children: &[NodeId]) {
        if children.is_empty() {
            return;
        }
        self.get_scene().with_write(
            |tree: &mut SceneTree| {
                tree.append_children(parent, children);
            },
            (),
        );
    }

    /// Binds a reactive `Style` closure to `node_id` and attaches its disposal `Scope` automatically.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Build context.
    /// - `NodeId` - Target scene node.
    /// - `F` - Reactive `Style` producer closure.
    pub fn bind_style<F>(&mut self, node_id: NodeId, producer: F)
    where
        F: FnMut() -> Style + 'static,
    {
        let handle: SceneHandle = self.get_scene().clone();
        let (scope, _): (Scope, Effect) =
            Scope::run(|_: Scope| handle.bind_style(node_id, producer));
        self.get_scene().with_write(
            |tree: &mut SceneTree| {
                tree.attach_scope(node_id, scope);
            },
            (),
        );
    }

    /// Registers a pointer-enter handler on `node_id`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Build context.
    /// - `NodeId` - Target node handle.
    /// - `F` - Pointer-enter callback closure.
    pub fn on_pointer_enter<F>(&mut self, node_id: NodeId, callback: F)
    where
        F: FnMut(&UiEvent) + 'static,
    {
        let handler: UiEventHandler = Rc::new(RefCell::new(callback));
        self.get_scene().with_write(
            |tree: &mut SceneTree| {
                if let Some(node) = tree.get_node_mut(node_id) {
                    node.get_mut_handlers().set_on_pointer_enter(Some(handler));
                }
            },
            (),
        );
    }

    /// Registers a pointer-leave handler on `node_id`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Build context.
    /// - `NodeId` - Target node handle.
    /// - `F` - Pointer-leave callback closure.
    pub fn on_pointer_leave<F>(&mut self, node_id: NodeId, callback: F)
    where
        F: FnMut(&UiEvent) + 'static,
    {
        let handler: UiEventHandler = Rc::new(RefCell::new(callback));
        self.get_scene().with_write(
            |tree: &mut SceneTree| {
                if let Some(node) = tree.get_node_mut(node_id) {
                    node.get_mut_handlers().set_on_pointer_leave(Some(handler));
                }
            },
            (),
        );
    }

    /// Registers a focus-gained handler on `node_id` and marks it focusable.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Build context.
    /// - `NodeId` - Target node handle.
    /// - `F` - Focus callback closure receiving `UiEventKind::Focus`.
    pub fn on_focus<F>(&mut self, node_id: NodeId, callback: F)
    where
        F: FnMut(&UiEvent) + 'static,
    {
        let handler: UiEventHandler = Rc::new(RefCell::new(callback));
        self.get_scene().with_write(
            |tree: &mut SceneTree| {
                if let Some(node) = tree.get_node_mut(node_id) {
                    node.get_mut_handlers().set_on_focus(Some(handler));
                    node.get_mut_interaction().set_focusable(true);
                }
            },
            (),
        );
    }

    /// Registers a focus-lost handler on `node_id`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Build context.
    /// - `NodeId` - Target node handle.
    /// - `F` - Blur callback closure receiving `UiEventKind::Blur`.
    pub fn on_blur<F>(&mut self, node_id: NodeId, callback: F)
    where
        F: FnMut(&UiEvent) + 'static,
    {
        let handler: UiEventHandler = Rc::new(RefCell::new(callback));
        self.get_scene().with_write(
            |tree: &mut SceneTree| {
                if let Some(node) = tree.get_node_mut(node_id) {
                    node.get_mut_handlers().set_on_blur(Some(handler));
                }
            },
            (),
        );
    }

    /// Configures whether `node_id` participates in keyboard focus navigation.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Build context.
    /// - `NodeId` - Target node handle.
    /// - `bool` - Whether keyboard focus is enabled.
    pub fn set_focusable(&mut self, node_id: NodeId, focusable: bool) {
        self.get_scene().with_write(
            |tree: &mut SceneTree| {
                if let Some(node) = tree.get_node_mut(node_id) {
                    node.get_mut_interaction().set_focusable(focusable);
                }
            },
            (),
        );
    }

    /// Registers a click handler on `node_id` and sets pointer cursor automatically.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Build context.
    /// - `NodeId` - Target node.
    /// - `F` - Click callback closure.
    pub fn on_click<F>(&mut self, node_id: NodeId, callback: F)
    where
        F: FnMut(&UiEvent) + 'static,
    {
        let handler: UiEventHandler = Rc::new(RefCell::new(callback));
        self.get_scene().with_write(
            |tree: &mut SceneTree| {
                if let Some(node) = tree.get_node_mut(node_id) {
                    node.get_mut_handlers().set_on_click(Some(handler));
                    node.get_mut_interaction().set_focusable(true);
                    let paint: &mut PaintStyle = node.get_mut_style().get_mut_paint();
                    if paint.get_cursor() == CursorIcon::Default {
                        paint.set_cursor(CursorIcon::Pointer);
                    }
                }
            },
            (),
        );
    }

    /// Registers a change handler on an `Input` node.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Build context.
    /// - `NodeId` - Target input node.
    /// - `F` - Change callback closure.
    pub fn on_change<F>(&mut self, node_id: NodeId, callback: F)
    where
        F: FnMut(&UiEvent) + 'static,
    {
        let handler: UiEventHandler = Rc::new(RefCell::new(callback));
        self.get_scene().with_write(
            |tree: &mut SceneTree| {
                if let Some(node) = tree.get_node_mut(node_id) {
                    node.get_mut_handlers().set_on_change(Some(handler));
                }
            },
            (),
        );
    }

    /// Attaches semantic accessibility properties to `node_id`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Build context.
    /// - `NodeId` - Target scene node.
    /// - `A11yProps` - Semantic role, label, hint, and state metadata.
    pub fn set_a11y(&mut self, node_id: NodeId, props: A11yProps) {
        self.get_scene().with_write(
            |tree: &mut SceneTree| {
                if let Some(node) = tree.get_node_mut(node_id) {
                    node.set_a11y(Some(props));
                }
            },
            (),
        );
    }

    /// Registers a scroll handler on a scrollable container node.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Build context.
    /// - `NodeId` - Target scroll container node.
    /// - `F` - Scroll callback closure receiving `UiEventKind::Scroll(delta)`.
    pub fn on_scroll<F>(&mut self, node_id: NodeId, callback: F)
    where
        F: FnMut(&UiEvent) + 'static,
    {
        let handler: UiEventHandler = Rc::new(RefCell::new(callback));
        self.get_scene().with_write(
            |tree: &mut SceneTree| {
                if let Some(node) = tree.get_node_mut(node_id) {
                    node.get_mut_handlers().set_on_scroll(Some(handler));
                }
            },
            (),
        );
    }

    /// Registers a pointer-down handler on `node_id` for draggable interactive controls.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Build context.
    /// - `NodeId` - Target node handle.
    /// - `F` - Pointer-down callback closure.
    pub fn on_pointer_down<F>(&mut self, node_id: NodeId, callback: F)
    where
        F: FnMut(&UiEvent) + 'static,
    {
        let handler: UiEventHandler = Rc::new(RefCell::new(callback));
        self.get_scene().with_write(
            |tree: &mut SceneTree| {
                if let Some(node) = tree.get_node_mut(node_id) {
                    node.get_mut_handlers().set_on_pointer_down(Some(handler));
                    let paint: &mut PaintStyle = node.get_mut_style().get_mut_paint();
                    if paint.get_cursor() == CursorIcon::Default {
                        paint.set_cursor(CursorIcon::Pointer);
                    }
                }
            },
            (),
        );
    }

    /// Registers a pointer-move handler on `node_id` receiving captured drag coordinates while pressed.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Build context.
    /// - `NodeId` - Target node handle.
    /// - `F` - Pointer-move callback closure.
    pub fn on_pointer_move<F>(&mut self, node_id: NodeId, callback: F)
    where
        F: FnMut(&UiEvent) + 'static,
    {
        let handler: UiEventHandler = Rc::new(RefCell::new(callback));
        self.get_scene().with_write(
            |tree: &mut SceneTree| {
                if let Some(node) = tree.get_node_mut(node_id) {
                    node.get_mut_handlers().set_on_pointer_move(Some(handler));
                }
            },
            (),
        );
    }

    /// Registers a pointer-up handler on `node_id` invoked when drag capture releases.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Build context.
    /// - `NodeId` - Target node handle.
    /// - `F` - Pointer-up callback closure.
    pub fn on_pointer_up<F>(&mut self, node_id: NodeId, callback: F)
    where
        F: FnMut(&UiEvent) + 'static,
    {
        let handler: UiEventHandler = Rc::new(RefCell::new(callback));
        self.get_scene().with_write(
            |tree: &mut SceneTree| {
                if let Some(node) = tree.get_node_mut(node_id) {
                    node.get_mut_handlers().set_on_pointer_up(Some(handler));
                }
            },
            (),
        );
    }

    /// Registers a hover handler on `node_id` invoked on both `PointerEnter` and `PointerLeave`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Build context.
    /// - `NodeId` - Target node handle.
    /// - `F` - Hover callback closure receiving `UiEventKind::PointerEnter` or `PointerLeave`.
    pub fn on_hover<F>(&mut self, node_id: NodeId, callback: F)
    where
        F: FnMut(&UiEvent) + 'static,
    {
        let handler: UiEventHandler = Rc::new(RefCell::new(callback));
        let leave_handler: UiEventHandler = handler.clone();
        self.get_scene().with_write(
            |tree: &mut SceneTree| {
                if let Some(node) = tree.get_node_mut(node_id) {
                    node.get_mut_handlers().set_on_pointer_enter(Some(handler));
                    node.get_mut_handlers()
                        .set_on_pointer_leave(Some(leave_handler));
                }
            },
            (),
        );
    }

    /// Registers a keyboard key-down handler on `node_id` and marks it focusable.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Build context.
    /// - `NodeId` - Target node handle.
    /// - `F` - Key-down callback closure receiving `UiEventKind::KeyDown`.
    pub fn on_key<F>(&mut self, node_id: NodeId, callback: F)
    where
        F: FnMut(&UiEvent) + 'static,
    {
        let handler: UiEventHandler = Rc::new(RefCell::new(callback));
        self.get_scene().with_write(
            |tree: &mut SceneTree| {
                if let Some(node) = tree.get_node_mut(node_id) {
                    node.get_mut_handlers().set_on_key_down(Some(handler));
                    node.get_mut_interaction().set_focusable(true);
                }
            },
            (),
        );
    }

    /// Records virtualized scroll progress on `node_id` so the overlay scrollbar tracks accurately.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Build context.
    /// - `NodeId` - Target virtualized scroll container.
    /// - `f32` - Active virtual vertical scroll offset in logical pixels.
    /// - `f32` - Full virtualized content height in logical pixels.
    pub fn set_virtual_scroll(&mut self, node_id: NodeId, scroll_y: f32, content_height: f32) {
        self.get_scene()
            .set_virtual_scroll(node_id, scroll_y, content_height);
    }

    /// Sets the concrete inner pixel scroll offset on `node_id` (used by virtualized lists for sub-pixel alignment).
    ///
    /// # Arguments
    ///
    /// - `&Self` - Build context.
    /// - `NodeId` - Target scroll container.
    /// - `Point` - Scroll offset `(x, y)` in logical pixels.
    pub fn set_scroll_offset(&mut self, node_id: NodeId, offset: Point) {
        self.get_scene().with_write(
            |tree: &mut SceneTree| {
                tree.set_scroll_offset(node_id, offset);
            },
            (),
        );
    }

    /// Starts a smooth paint/transform transition on `transition.get_node()`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Build context.
    /// - `NodeTransition` - Transition specification.
    pub fn start_transition(&mut self, transition: NodeTransition) {
        self.get_scene().start_transition(transition);
    }

    /// Mounts a reactive dynamic subtree whose children rebuild cleanly inside an isolated `Scope`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Build context.
    /// - `NodeId` - Host container node.
    /// - `F` - Reactive zero-`ctx` builder returning child `NodeId` values.
    pub fn bind_children<F>(&mut self, host: NodeId, mut builder: F)
    where
        F: FnMut() -> Vec<NodeId> + 'static,
    {
        let ctx_clone: Self = self.clone();
        let handle: SceneHandle = self.get_scene().clone();
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            Effect::new(move || {
                let (child_scope, children): (Scope, Vec<NodeId>) =
                    Scope::run(|_: Scope| with_context(ctx_clone.clone(), &mut builder));
                let first_opt: Option<NodeId> = children.first().copied();
                handle.replace_children_safe(host, children);
                if let Some(first) = first_opt {
                    handle.with_write(
                        |tree: &mut SceneTree| {
                            tree.attach_scope(first, child_scope);
                        },
                        (),
                    );
                }
            })
        });
        self.get_scene().with_write(
            |tree: &mut SceneTree| {
                tree.attach_scope(host, scope);
            },
            (),
        );
    }
}

/// Restores the previously bound ambient `Context` when leaving scope.
impl Drop for ContextGuard {
    /// Restores the saved thread-local `Context`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scope guard being dropped.
    fn drop(&mut self) {
        let Self { previous } = self;
        let restored: Option<Context> = previous.take();
        ACTIVE_CONTEXT.with(|slot: &RefCell<Option<Context>>| {
            *slot.borrow_mut() = restored;
        });
    }
}

/// `NodeRefTarget` implementation storing a `NodeId` directly into a mutable local binding.
impl NodeRefTarget for NodeId {
    /// Assigns `id` to `self`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Target `NodeId` slot.
    /// - `NodeId` - Mounted node handle.
    fn bind_node_ref(&mut self, id: NodeId) {
        *self = id;
    }
}

/// `NodeRefTarget` implementation storing a `NodeId` into an `Option<NodeId>`.
impl NodeRefTarget for Option<NodeId> {
    /// Assigns `Some(id)` to `self`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Target optional `NodeId` slot.
    /// - `NodeId` - Mounted node handle.
    fn bind_node_ref(&mut self, id: NodeId) {
        *self = Some(id);
    }
}

/// `NodeRefTarget` implementation storing a `NodeId` into a `Cell<NodeId>`.
impl NodeRefTarget for Cell<NodeId> {
    /// Sets `self` to `id`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Target `Cell<NodeId>`.
    /// - `NodeId` - Mounted node handle.
    fn bind_node_ref(&mut self, id: NodeId) {
        self.set(id);
    }
}

/// `NodeRefTarget` implementation storing a `NodeId` into a shared `Rc<Cell<NodeId>>`.
impl NodeRefTarget for Rc<Cell<NodeId>> {
    /// Sets the shared cell value to `id`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Target shared `Cell<NodeId>`.
    /// - `NodeId` - Mounted node handle.
    fn bind_node_ref(&mut self, id: NodeId) {
        self.set(id);
    }
}

/// `NodeRefTarget` implementation storing a `NodeId` into a `Cell<Option<NodeId>>`.
impl NodeRefTarget for Cell<Option<NodeId>> {
    /// Sets `self` to `Some(id)`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Target `Cell<Option<NodeId>>`.
    /// - `NodeId` - Mounted node handle.
    fn bind_node_ref(&mut self, id: NodeId) {
        self.set(Some(id));
    }
}

/// `NodeRefTarget` implementation storing a `NodeId` into a shared `Rc<Cell<Option<NodeId>>>`.
impl NodeRefTarget for Rc<Cell<Option<NodeId>>> {
    /// Sets the shared cell value to `Some(id)`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Target shared `Cell<Option<NodeId>>`.
    /// - `NodeId` - Mounted node handle.
    fn bind_node_ref(&mut self, id: NodeId) {
        self.set(Some(id));
    }
}

/// `TextNodeContent` implementation mounting an owned `String` as a static text node.
impl TextNodeContent for String {
    /// Mounts a static text scene node from this `String`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Owned text string.
    /// - `&mut Context` - Active scene build context.
    /// - `Style` - Resolved text style.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted text node handle.
    fn mount_text_content(self, ctx: &mut Context, style: Style) -> NodeId {
        ctx.text(self, style)
    }
}

/// `TextNodeContent` implementation mounting a string slice as a static text node.
impl TextNodeContent for &str {
    /// Mounts a static text scene node from this `&str`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Borrowed text slice.
    /// - `&mut Context` - Active scene build context.
    /// - `Style` - Resolved text style.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted text node handle.
    fn mount_text_content(self, ctx: &mut Context, style: Style) -> NodeId {
        ctx.text(self.to_string(), style)
    }
}

/// `TextNodeContent` implementation mounting a borrowed `&String` as a static text node.
impl TextNodeContent for &String {
    /// Mounts a static text scene node from this `&String`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Borrowed string reference.
    /// - `&mut Context` - Active scene build context.
    /// - `Style` - Resolved text style.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted text node handle.
    fn mount_text_content(self, ctx: &mut Context, style: Style) -> NodeId {
        ctx.text(self.clone(), style)
    }
}

/// `TextNodeContent` implementation mounting a `Cow<str>` as a static text node.
impl TextNodeContent for Cow<'_, str> {
    /// Mounts a static text scene node from this `Cow<str>`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Copy-on-write string value.
    /// - `&mut Context` - Active scene build context.
    /// - `Style` - Resolved text style.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted text node handle.
    fn mount_text_content(self, ctx: &mut Context, style: Style) -> NodeId {
        ctx.text(self.into_owned(), style)
    }
}

/// `TextNodeContent` implementation mounting any `FnMut() -> S` closure or function item as a reactive text node.
impl<F, S> TextNodeContent for F
where
    F: FnMut() -> S + 'static,
    S: Into<String>,
{
    /// Mounts a reactive text scene node bound to this closure or function item.
    ///
    /// # Arguments
    ///
    /// - `Self` - Reactive text producer closure.
    /// - `&mut Context` - Active scene build context.
    /// - `Style` - Resolved text style.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted reactive text node handle.
    fn mount_text_content(mut self, ctx: &mut Context, style: Style) -> NodeId {
        ctx.reactive_text(style, move || self().into())
    }
}
