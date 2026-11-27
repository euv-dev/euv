use super::*;

/// Finds the topmost interactive scene node containing `point`, respecting z-index and clipping.
///
/// # Arguments
///
/// - `&SceneTree` - Retained scene tree after layout.
/// - `Point` - Logical pointer coordinates in root space.
///
/// # Returns
///
/// - `Option<NodeId>` - Topmost matching node handle, if any.
#[must_use]
pub fn hit_test(tree: &SceneTree, point: Point) -> Option<NodeId> {
    let registered: &[NodeId] = tree.portal_nodes();
    if !registered.is_empty() {
        let root: NodeId = tree.get_root();
        let mut first_active: Option<(i32, NodeId)> = None;
        let mut active_portals: Vec<(i32, NodeId)> = Vec::new();
        for portal_id in registered.iter().copied() {
            if is_active_top_layer_portal(tree, root, portal_id)
                && let Some(node) = tree.get_node(portal_id)
            {
                let z: i32 = node.style_ref().get_layout().get_z_index();
                if let Some(first) = first_active {
                    if active_portals.is_empty() {
                        active_portals.push(first);
                    }
                    active_portals.push((z, portal_id));
                } else {
                    first_active = Some((z, portal_id));
                }
            }
        }
        if !active_portals.is_empty() {
            active_portals.sort_by(|a: &(i32, NodeId), b: &(i32, NodeId)| {
                b.0.cmp(&a.0).then_with(|| {
                    let (rb, ra): (usize, usize) = tree_order_pair(tree, b.1, a.1);
                    rb.cmp(&ra)
                })
            });
            for (_, portal_id) in active_portals.into_iter() {
                if let Some(hit) =
                    hit_test_subtree(tree, portal_id, point, Point::new(0.0, 0.0), None, true)
                {
                    return Some(hit);
                }
            }
        } else if let Some((_, portal_id)) = first_active
            && let Some(hit) =
                hit_test_subtree(tree, portal_id, point, Point::new(0.0, 0.0), None, true)
        {
            return Some(hit);
        }
    }
    hit_test_subtree(
        tree,
        tree.get_root(),
        point,
        Point::new(0.0, 0.0),
        None,
        false,
    )
}

/// Checks in O(depth) whether `portal_id` is a visible, root-attached top-layer portal not nested inside another portal.
///
/// # Arguments
///
/// - `&SceneTree` - Retained scene tree.
/// - `NodeId` - Root scene node handle.
/// - `NodeId` - Candidate portal or `Position::Fixed` node handle.
///
/// # Returns
///
/// - `bool` - `true` when `portal_id` participates in the top-layer hit-test pass.
fn is_active_top_layer_portal(tree: &SceneTree, root: NodeId, portal_id: NodeId) -> bool {
    let Some(node) = tree.get_node(portal_id) else {
        return false;
    };
    let Style { layout, .. } = node.style_ref();
    if layout.get_display() == Display::None {
        return false;
    }
    let is_portal: bool =
        matches!(node.get_kind(), NodeKind::Portal) || layout.get_position() == Position::Fixed;
    if !is_portal {
        return false;
    }
    if portal_id == root {
        return true;
    }
    let mut cursor: Option<NodeId> = node.parent_id();
    while let Some(pid) = cursor {
        let Some(pnode) = tree.get_node(pid) else {
            return false;
        };
        let Style {
            layout: playout, ..
        } = pnode.style_ref();
        if playout.get_display() == Display::None {
            return false;
        }
        let parent_is_portal: bool = matches!(pnode.get_kind(), NodeKind::Portal)
            || playout.get_position() == Position::Fixed;
        if parent_is_portal {
            return false;
        }
        if pid == root {
            return true;
        }
        cursor = pnode.parent_id();
    }
    false
}

/// Computes an order-preserving `(rank_a, rank_b)` pair comparing tree preorder of `a` and `b` in O(depth).
///
/// # Arguments
///
/// - `&SceneTree` - Retained scene tree.
/// - `NodeId` - First root-attached node handle.
/// - `NodeId` - Second root-attached node handle.
///
/// # Returns
///
/// - `(usize, usize)` - Comparable rank pair where `rank_a < rank_b` iff `a` precedes `b` in preorder.
fn tree_order_pair(tree: &SceneTree, a: NodeId, b: NodeId) -> (usize, usize) {
    if a == b {
        return (0, 0);
    }
    let mut path_a: Vec<NodeId> = Vec::new();
    let mut cur_a: Option<NodeId> = Some(a);
    while let Some(id) = cur_a {
        path_a.push(id);
        cur_a = tree.get_node(id).and_then(SceneNode::parent_id);
    }
    path_a.reverse();
    let mut path_b: Vec<NodeId> = Vec::new();
    let mut cur_b: Option<NodeId> = Some(b);
    while let Some(id) = cur_b {
        path_b.push(id);
        cur_b = tree.get_node(id).and_then(SceneNode::parent_id);
    }
    path_b.reverse();
    let min_len: usize = path_a.len().min(path_b.len());
    let mut diff_idx: usize = 0;
    while diff_idx < min_len && path_a[diff_idx] == path_b[diff_idx] {
        diff_idx += 1;
    }
    if diff_idx == min_len {
        return (path_a.len(), path_b.len());
    }
    if diff_idx == 0 {
        return (a.as_usize(), b.as_usize());
    }
    let lca: NodeId = path_a[diff_idx - 1];
    let branch_a: NodeId = path_a[diff_idx];
    let branch_b: NodeId = path_b[diff_idx];
    if let Some(lca_node) = tree.get_node(lca) {
        for child_id in lca_node.get_children().iter().copied() {
            if child_id == branch_a {
                return (0, 1);
            }
            if child_id == branch_b {
                return (1, 0);
            }
        }
    }
    (a.as_usize(), b.as_usize())
}

/// Computes screen-space node bounds after applying accumulated scroll offset and `Transform2D`.
///
/// # Arguments
///
/// - `Rect` - Node layout border-box bounds.
/// - `Point` - Accumulated ancestor scroll offset.
/// - `Transform2D` - Node paint transform.
///
/// # Returns
///
/// - `Rect` - Transformed and scroll-shifted bounds in root logical coordinates.
#[inline(always)]
fn shifted_node_bounds(raw_bounds: Rect, scroll_offset: Point, transform: Transform2D) -> Rect {
    let tx: f32 = transform.get_translate_x();
    let ty: f32 = transform.get_translate_y();
    let sx: f32 = transform.get_scale_x();
    let sy: f32 = transform.get_scale_y();
    if sx == 1.0 && sy == 1.0 {
        Rect::new(
            raw_bounds.get_x() - scroll_offset.get_x() + tx,
            raw_bounds.get_y() - scroll_offset.get_y() + ty,
            raw_bounds.get_width(),
            raw_bounds.get_height(),
        )
    } else {
        let scaled_w: f32 = raw_bounds.get_width() * sx;
        let scaled_h: f32 = raw_bounds.get_height() * sy;
        let scale_dx: f32 = (raw_bounds.get_width() - scaled_w) * 0.5;
        let scale_dy: f32 = (raw_bounds.get_height() - scaled_h) * 0.5;
        Rect::new(
            raw_bounds.get_x() - scroll_offset.get_x() + tx + scale_dx,
            raw_bounds.get_y() - scroll_offset.get_y() + ty + scale_dy,
            scaled_w,
            scaled_h,
        )
    }
}

/// Recursively hit-tests `node_id` in reverse paint order (topmost z-index first).
///
/// # Arguments
///
/// - `&SceneTree` - Retained scene tree.
/// - `NodeId` - Subtree root handle.
/// - `Point` - Target logical pointer position.
/// - `Point` - Accumulated scroll offset from ancestor scroll views.
/// - `Option<Rect>` - Active ancestor clip rectangle.
/// - `bool` - Whether this call is already inside the top-layer portal pass.
///
/// # Returns
///
/// - `Option<NodeId>` - Deepest topmost hit node handle.
fn hit_test_subtree(
    tree: &SceneTree,
    node_id: NodeId,
    point: Point,
    scroll_offset: Point,
    clip_rect: Option<Rect>,
    in_portal_pass: bool,
) -> Option<NodeId> {
    let node: &SceneNode = tree.get_node(node_id)?;
    hit_test_resolved(
        tree,
        node_id,
        node,
        point,
        scroll_offset,
        clip_rect,
        in_portal_pass,
    )
}

/// Hit-tests an already-fetched `SceneNode` with inline leaf, clipped-bounds, and single-leaf-child fast paths.
///
/// # Arguments
///
/// - `&SceneTree` - Retained scene tree.
/// - `NodeId` - Handle corresponding to `node`.
/// - `&SceneNode` - Live scene node reference.
/// - `Point` - Target logical pointer position.
/// - `Point` - Accumulated ancestor scroll offset.
/// - `Option<Rect>` - Active ancestor clip rectangle already verified to contain `point`.
/// - `bool` - Whether this call is already inside the top-layer portal pass.
///
/// # Returns
///
/// - `Option<NodeId>` - Deepest topmost hit node handle.
#[inline(always)]
fn hit_test_resolved(
    tree: &SceneTree,
    node_id: NodeId,
    node: &SceneNode,
    point: Point,
    scroll_offset: Point,
    clip_rect: Option<Rect>,
    in_portal_pass: bool,
) -> Option<NodeId> {
    let Style { layout, paint, .. } = node.style_ref();
    if layout.get_display() == Display::None || paint.get_pointer_events() == PointerEvents::None {
        return None;
    }
    let is_portal: bool =
        matches!(node.get_kind(), NodeKind::Portal) || layout.get_position() == Position::Fixed;
    if !in_portal_pass && is_portal {
        return None;
    }
    let transform: Transform2D = paint.get_transform();
    let shifted_bounds: Rect =
        shifted_node_bounds(node.get_layout_box().get_bounds(), scroll_offset, transform);
    let is_clipped: bool = matches!(layout.get_overflow(), Overflow::Hidden | Overflow::Scroll);
    let children: &[NodeId] = node.get_children().as_slice();
    if children.is_empty() {
        return if shifted_bounds.contains(point) {
            Some(node_id)
        } else {
            None
        };
    }
    if is_clipped && !shifted_bounds.contains(point) {
        return None;
    }
    let effective_clip: Option<Rect> = if is_clipped {
        match clip_rect {
            Some(parent_clip) => parent_clip.intersect(shifted_bounds),
            None => Some(shifted_bounds),
        }
    } else {
        clip_rect
    };
    if let Some(clip) = effective_clip
        && !clip.contains(point)
    {
        return None;
    }
    let node_scroll: Point = node.get_layout_box().get_scroll_offset();
    let child_scroll: Point = Point::new(
        scroll_offset.get_x() + node_scroll.get_x() - transform.get_translate_x(),
        scroll_offset.get_y() + node_scroll.get_y() - transform.get_translate_y(),
    );
    if children.len() == 1 {
        let single_id: NodeId = children[0];
        if let Some(single_node) = tree.get_node(single_id) {
            if single_node.get_children().is_empty() {
                let Style {
                    layout: c_layout,
                    paint: c_paint,
                    ..
                } = single_node.style_ref();
                let c_hidden: bool = c_layout.get_display() == Display::None
                    || c_paint.get_pointer_events() == PointerEvents::None;
                let c_portal: bool = matches!(single_node.get_kind(), NodeKind::Portal)
                    || c_layout.get_position() == Position::Fixed;
                if !c_hidden && (in_portal_pass || !c_portal) {
                    let c_bounds: Rect = shifted_node_bounds(
                        single_node.get_layout_box().get_bounds(),
                        child_scroll,
                        c_paint.get_transform(),
                    );
                    if c_bounds.contains(point) {
                        return Some(single_id);
                    }
                }
            } else if let Some(hit) = hit_test_resolved(
                tree,
                single_id,
                single_node,
                point,
                child_scroll,
                effective_clip,
                in_portal_pass,
            ) {
                return Some(hit);
            }
        }
        return if shifted_bounds.contains(point) {
            Some(node_id)
        } else {
            None
        };
    }
    hit_test_multi_children(
        tree,
        node_id,
        children,
        shifted_bounds,
        (point, child_scroll),
        effective_clip,
        in_portal_pass,
    )
}

/// Hit-tests a container with two or more children using a single-pass reverse fast path when `z_index == 0`.
///
/// # Arguments
///
/// - `&SceneTree` - Retained scene tree.
/// - `NodeId` - Parent container handle.
/// - `&[NodeId]` - Child node handles in paint order.
/// - `Rect` - Parent shifted bounds for self-hit fallback.
/// - `(Point, Point)` - Pair of `(pointer_point, child_scroll_offset)`.
/// - `Option<Rect>` - Effective clip rectangle already containing `point`.
/// - `bool` - Whether this call is already inside the top-layer portal pass.
///
/// # Returns
///
/// - `Option<NodeId>` - Deepest topmost hit node handle.
fn hit_test_multi_children(
    tree: &SceneTree,
    node_id: NodeId,
    children: &[NodeId],
    shifted_bounds: Rect,
    coords: (Point, Point),
    effective_clip: Option<Rect>,
    in_portal_pass: bool,
) -> Option<NodeId> {
    let (point, child_scroll): (Point, Point) = coords;
    let mut needs_full_z_sort: bool = false;
    for rev_idx in (0..children.len()).rev() {
        let child_id: NodeId = children[rev_idx];
        let Some(child_node) = tree.get_node(child_id) else {
            continue;
        };
        if child_node.style_ref().get_layout().get_z_index() != 0 {
            needs_full_z_sort = true;
            break;
        }
        if let Some(hit) = hit_test_resolved(
            tree,
            child_id,
            child_node,
            point,
            child_scroll,
            effective_clip,
            in_portal_pass,
        ) {
            let mut pos_z_children: Vec<(i32, usize, NodeId)> = Vec::new();
            for (prev_idx, prev_id) in children[..rev_idx].iter().copied().enumerate() {
                if let Some(prev_node) = tree.get_node(prev_id) {
                    let prev_z: i32 = prev_node.style_ref().get_layout().get_z_index();
                    if prev_z > 0 {
                        pos_z_children.push((prev_z, prev_idx, prev_id));
                    }
                }
            }
            if !pos_z_children.is_empty() {
                pos_z_children.sort_by(|a: &(i32, usize, NodeId), b: &(i32, usize, NodeId)| {
                    b.0.cmp(&a.0).then_with(|| b.1.cmp(&a.1))
                });
                for (_, _, z_child_id) in pos_z_children.into_iter() {
                    if let Some(z_hit) = hit_test_subtree(
                        tree,
                        z_child_id,
                        point,
                        child_scroll,
                        effective_clip,
                        in_portal_pass,
                    ) {
                        return Some(z_hit);
                    }
                }
            }
            return Some(hit);
        }
    }
    if needs_full_z_sort {
        let mut sorted_children: Vec<(i32, usize, NodeId)> = Vec::with_capacity(children.len());
        for (order, cid) in children.iter().copied().enumerate() {
            if let Some(child_node) = tree.get_node(cid) {
                sorted_children.push((
                    child_node.style_ref().get_layout().get_z_index(),
                    order,
                    cid,
                ));
            }
        }
        sorted_children.sort_by(|a: &(i32, usize, NodeId), b: &(i32, usize, NodeId)| {
            b.0.cmp(&a.0).then_with(|| b.1.cmp(&a.1))
        });
        for (_, _, child_id) in sorted_children.into_iter() {
            if let Some(hit) = hit_test_subtree(
                tree,
                child_id,
                point,
                child_scroll,
                effective_clip,
                in_portal_pass,
            ) {
                return Some(hit);
            }
        }
    }
    if shifted_bounds.contains(point) {
        Some(node_id)
    } else {
        None
    }
}
