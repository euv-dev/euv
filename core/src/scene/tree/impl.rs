use super::*;

/// Default scene tree with a pre-created root `View` node.
impl Default for SceneTree {
    /// Allocates a new `SceneTree` containing a root container node.
    ///
    /// # Returns
    ///
    /// - `Self` - Initialized scene tree.
    fn default() -> Self {
        let root_style: Style = Style::empty()
            .column()
            .width(Length::Percent(100.0))
            .height(Length::Percent(100.0));
        let root_node: SceneNode = SceneNode::new(1, NodeKind::View, root_style);
        Self {
            nodes: vec![root_node],
            free_list: Vec::new(),
            root: NodeId::new(0, 1),
            focus: FocusManager::default(),
            revision: 1,
            transitions: Vec::new(),
            caret_visible: true,
        }
    }
}

/// Core retained scene graph mutation and traversal operations.
impl SceneTree {
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
        self.create_node(NodeKind::Fragment, Style::empty())
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

    /// Returns total count of live scene nodes.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scene tree reference.
    ///
    /// # Returns
    ///
    /// - `usize` - Number of live nodes in the arena.
    #[must_use]
    pub fn live_node_count(&self) -> usize {
        self.get_nodes()
            .iter()
            .filter(|node: &&SceneNode| node.get_alive())
            .count()
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
        if let Some(old_parent) = prev_parent
            && let Some(old_node) = self.get_node_mut(old_parent)
        {
            old_node
                .get_mut_children()
                .retain(|item: &NodeId| *item != child);
            if old_parent != parent {
                self.mark_dirty(old_parent, DIRTY_CHILDREN_CHANGE_MASK);
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
        for old_child in old_children.into_iter() {
            if !new_children.contains(&old_child) {
                self.dispose_subtree(old_child);
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

    /// Surgically updates a node's `Style`, only marking layout dirty when `LayoutStyle` changed.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene tree.
    /// - `NodeId` - Target node handle.
    /// - `Style` - Replacement compiled style.
    pub fn set_style(&mut self, id: NodeId, style: Style) {
        let dirty_mask: u16 = match self.get_node_mut(id) {
            Some(node) => {
                let prev: Style = node.get_style();
                if prev == style {
                    return;
                }
                let mut mask: u16 = DIRTY_PAINT;
                if prev.get_layout() != style.get_layout() || prev.get_text() != style.get_text() {
                    mask |= DIRTY_LAYOUT | DIRTY_TEXT;
                }
                node.set_style(style);
                mask
            }
            None => return,
        };
        self.mark_dirty(id, dirty_mask);
    }

    /// Surgically updates the UTF-8 string of a `NodeKind::Text` node.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene tree.
    /// - `NodeId` - Target text node handle.
    /// - `String` - Replacement text string.
    pub fn set_text(&mut self, id: NodeId, text: String) {
        let changed: bool = match self.get_node_mut(id) {
            Some(node) => match node.get_mut_kind() {
                NodeKind::Text(current) => {
                    if *current == text {
                        false
                    } else {
                        *current = text;
                        true
                    }
                }
                _ => false,
            },
            None => false,
        };
        if changed {
            self.mark_dirty(id, DIRTY_TEXT_CHANGE_MASK);
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
                let clamped: Point = if unlaid_out {
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

    /// Computes screen-space logical bounds for `id` after subtracting ancestor scroll offsets.
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
        let off: Point = self.ancestor_scroll_offset(id);
        Some(Rect::new(
            raw.get_x() - off.get_x(),
            raw.get_y() - off.get_y(),
            raw.get_width(),
            raw.get_height(),
        ))
    }

    /// Starts or replaces a smooth paint/transform transition on `transition.get_node()`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene tree.
    /// - `NodeTransition` - Transition specification to run.
    pub fn start_transition(&mut self, transition: NodeTransition) {
        let target: NodeId = transition.get_node();
        self.get_mut_transitions()
            .retain(|tr: &NodeTransition| tr.get_node() != target);
        let (opacity, transform): (f32, Transform2D) = transition.sample();
        let mut unhidden: bool = false;
        if let Some(node) = self.get_node_mut(target) {
            let mut style: Style = node.get_style();
            if !transition.get_hide_on_complete()
                && style.get_layout().get_display() == Display::None
            {
                style.get_mut_layout().set_display(Display::Flex);
                unhidden = true;
            }
            style.get_mut_paint().set_opacity(opacity);
            style.get_mut_paint().set_transform(transform);
            node.set_style(style);
        }
        if unhidden {
            self.mark_dirty(target, DIRTY_LAYOUT | DIRTY_PAINT);
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
        if self.get_transitions().is_empty() {
            return false;
        }
        let step: f32 = dt_secs.clamp(0.001, 0.05);
        let mut active: Vec<NodeTransition> = take(self.get_mut_transitions());
        let mut remaining: Vec<NodeTransition> = Vec::with_capacity(active.len());
        let mut any_updated: bool = false;
        for mut tr in active.drain(..) {
            let target: NodeId = tr.get_node();
            if self.get_node(target).is_none() {
                continue;
            }
            tr.set_elapsed((tr.get_elapsed() + step).min(tr.get_duration()));
            let (opacity, transform): (f32, Transform2D) = tr.sample();
            let done: bool = tr.is_complete();
            let hide: bool = done && tr.get_hide_on_complete();
            if let Some(node) = self.get_node_mut(target) {
                let mut style: Style = node.get_style();
                style.get_mut_paint().set_opacity(opacity);
                style.get_mut_paint().set_transform(transform);
                if hide {
                    style.get_mut_layout().set_display(Display::None);
                }
                node.set_style(style);
            }
            if hide {
                self.mark_dirty(target, DIRTY_LAYOUT | DIRTY_PAINT);
            } else {
                self.mark_dirty(target, DIRTY_PAINT);
            }
            any_updated = true;
            if !done {
                remaining.push(tr);
            }
        }
        self.set_transitions(remaining);
        any_updated
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

    /// Attaches a reactive `Scope` to `id` so it automatically disposes when the node unmounts.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene tree.
    /// - `NodeId` - Target node handle.
    /// - `Scope` - Reactive ownership scope.
    pub fn attach_scope(&mut self, id: NodeId, scope: Scope) {
        if let Some(node) = self.get_node_mut(id) {
            node.set_scope(Some(scope));
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
        let mut parent_opt: Option<NodeId> = match self.get_node_mut(id) {
            Some(node) => {
                node.get_mut_dirty().insert(mask);
                node.parent_id()
            }
            None => return,
        };
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

    /// Computes accumulated ancestor scroll offset for `id` (zero for top-layer `Portal`/`Fixed` nodes).
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scene tree reference.
    /// - `NodeId` - Target node handle.
    ///
    /// # Returns
    ///
    /// - `Point` - Total scroll offset applied to `id`.
    fn ancestor_scroll_offset(&self, id: NodeId) -> Point {
        let Some(start) = self.get_node(id) else {
            return Point::default();
        };
        if matches!(start.get_kind(), NodeKind::Portal)
            || start.get_style().get_layout().get_position() == Position::Fixed
        {
            return Point::default();
        }
        let mut sx: f32 = 0.0;
        let mut sy: f32 = 0.0;
        let mut cursor: Option<NodeId> = start.parent_id();
        while let Some(pid) = cursor {
            if let Some(pnode) = self.get_node(pid) {
                let off: Point = pnode.get_layout_box().get_scroll_offset();
                sx += off.get_x();
                sy += off.get_y();
                if matches!(pnode.get_kind(), NodeKind::Portal)
                    || pnode.get_style().get_layout().get_position() == Position::Fixed
                {
                    break;
                }
                cursor = pnode.parent_id();
            } else {
                break;
            }
        }
        Point::new(sx, sy)
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
            if node.get_style().get_layout().get_display() == Display::None
                || node.get_style().get_paint().get_opacity() <= 0.0
            {
                return false;
            }
            cursor = node.parent_id();
        }
        true
    }

    /// Collects the tight physical damage region from dirty paint/scroll nodes and clears paint dirty flags.
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
        let mut full: bool = false;
        let mut dirty_ids: Vec<(NodeId, Rect, f32)> = Vec::new();
        for (idx, node) in self.get_nodes().iter().enumerate() {
            if !node.get_alive() {
                continue;
            }
            let bits: u16 = node.get_dirty().get_bits();
            if (bits & DIRTY_FULL_DAMAGE_MASK) != 0 {
                full = true;
                break;
            }
            if (bits & (DIRTY_PAINT | DIRTY_SCROLL)) != 0 {
                let b: Rect = node.get_layout_box().get_bounds();
                if b.get_width() > 0.0 && b.get_height() > 0.0 {
                    let mut margin: f32 = 10.0;
                    if let Some(shadow) = node.get_style().get_paint().box_shadow() {
                        margin = margin.max(
                            shadow.get_blur_radius()
                                + shadow.get_spread_radius()
                                + shadow.get_offset_x().abs().max(shadow.get_offset_y().abs())
                                + 4.0,
                        );
                    }
                    let nid: NodeId = NodeId::new(idx as u32, node.get_generation());
                    dirty_ids.push((nid, b, margin));
                } else if (bits & DIRTY_PAINT) != 0 {
                    full = true;
                    break;
                }
            }
        }
        let mut damage: DamageRegion = if full {
            DamageRegion::full()
        } else {
            let mut region: DamageRegion = DamageRegion::default();
            for (nid, raw_bounds, margin) in dirty_ids.into_iter() {
                let off: Point = self.ancestor_scroll_offset(nid);
                let phys: Rect = Rect::new(
                    ((raw_bounds.get_x() - off.get_x() - margin) * scale).floor(),
                    ((raw_bounds.get_y() - off.get_y() - margin) * scale).floor(),
                    ((raw_bounds.get_width() + margin * 2.0) * scale).ceil(),
                    ((raw_bounds.get_height() + margin * 2.0) * scale).ceil(),
                );
                region.add_rect(phys);
            }
            region
        };
        let clear_mask: u16 = DIRTY_PAINT_CLEAR_MASK;
        for node in self.get_mut_nodes().iter_mut() {
            if node.get_alive() && (node.get_dirty().get_bits() & clear_mask) != 0 {
                node.get_mut_dirty().remove(clear_mask);
            }
        }
        if full {
            damage.set_full_damage(true);
        }
        damage
    }

    /// Recursively removes and disposes `id`, recycling all arena slots and disposing scopes.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene tree.
    /// - `NodeId` - Subtree root handle to dispose.
    pub fn dispose_subtree(&mut self, id: NodeId) {
        if id == self.get_root() {
            return;
        }
        let mut stack: Vec<NodeId> = vec![id];
        let mut scopes: Vec<Scope> = Vec::new();
        while let Some(current) = stack.pop() {
            let children_opt: Option<Vec<NodeId>> = match self.get_node_mut(current) {
                Some(node) => {
                    node.set_alive(false);
                    if let Some(scope) = node.owned_scope() {
                        scopes.push(scope);
                    }
                    node.set_scope(None);
                    node.set_parent(None);
                    node.set_canvas_painter(None);
                    node.set_handlers(EventHandlers::default());
                    let next_gen: u32 = node.get_generation().wrapping_add(1).max(1);
                    node.set_generation(next_gen);
                    Some(take(node.get_mut_children()))
                }
                None => None,
            };
            let Some(children) = children_opt else {
                continue;
            };
            self.get_mut_free_list().push(current.get_index());
            for child in children.into_iter() {
                stack.push(child);
            }
        }
        for scope in scopes.into_iter() {
            scope.dispose();
        }
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

/// High-level declarative building helpers on `BuildContext`.
impl BuildContext {
    /// Creates a new `BuildContext` with reactive viewport, theme, and safe-area signals.
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
            system_back: Signal::new(0),
            dynamic_hub: DynamicUpdateHub::new(),
            fps_limit: Signal::new(None),
            live_fps: Signal::new(60.0),
            active_touches: Signal::new(Vec::new()),
        }
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
                    let mut style: Style = node.get_style();
                    style.get_mut_paint().set_cursor(CursorIcon::Pointer);
                    node.set_style(style);
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
                    let mut style: Style = node.get_style();
                    style.get_mut_paint().set_cursor(CursorIcon::Pointer);
                    node.set_style(style);
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
                    Scope::run(|_: Scope| with_build_context(ctx_clone.clone(), &mut builder));
                handle.with_write(
                    |tree: &mut SceneTree| {
                        let first_opt: Option<NodeId> = children.first().copied();
                        tree.replace_children(host, children);
                        if let Some(first) = first_opt {
                            tree.attach_scope(first, child_scope);
                        }
                    },
                    (),
                );
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

/// Restores the previously bound ambient `BuildContext` when leaving scope.
impl Drop for BuildContextGuard {
    /// Restores the saved thread-local `BuildContext`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scope guard being dropped.
    fn drop(&mut self) {
        let Self { previous } = self;
        let restored: Option<BuildContext> = previous.take();
        ACTIVE_BUILD_CONTEXT.with(|slot: &RefCell<Option<BuildContext>>| {
            *slot.borrow_mut() = restored;
        });
    }
}

/// Blanket `Mountable` implementation allowing raw `NodeId` values to be mounted directly.
impl Mountable for NodeId {
    /// Returns this `NodeId` unchanged.
    ///
    /// # Arguments
    ///
    /// - `Self` - Already mounted node handle.
    /// - `&mut BuildContext` - Unused build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Same node handle.
    fn mount(self, _ctx: &mut BuildContext) -> NodeId {
        self
    }
}
