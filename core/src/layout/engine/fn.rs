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
    let mut portals: Vec<(i32, usize, NodeId)> = Vec::new();
    collect_top_layer_portals(tree, tree.get_root(), &mut portals);
    portals.sort_by(|a: &(i32, usize, NodeId), b: &(i32, usize, NodeId)| {
        b.0.cmp(&a.0).then_with(|| b.1.cmp(&a.1))
    });
    for (_, _, portal_id) in portals.into_iter() {
        if let Some(hit) =
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

/// Collects all visible top-layer `Portal` and `Position::Fixed` nodes in tree order.
///
/// # Arguments
///
/// - `&SceneTree` - Retained scene tree.
/// - `NodeId` - Current subtree root.
/// - `&mut Vec<(i32, usize, NodeId)>` - Output list of `(z_index, order, node_id)`.
fn collect_top_layer_portals(
    tree: &SceneTree,
    node_id: NodeId,
    out: &mut Vec<(i32, usize, NodeId)>,
) {
    let Some(node) = tree.get_node(node_id) else {
        return;
    };
    let layout: LayoutStyle = node.get_style().get_layout();
    if layout.get_display() == Display::None {
        return;
    }
    if matches!(node.get_kind(), NodeKind::Portal) || layout.get_position() == Position::Fixed {
        out.push((layout.get_z_index(), out.len(), node_id));
        return;
    }
    for child_id in node.get_children().iter().copied() {
        collect_top_layer_portals(tree, child_id, out);
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
    let style: Style = node.get_style();
    let layout: LayoutStyle = style.get_layout();
    if layout.get_display() == Display::None
        || style.get_paint().get_pointer_events() == PointerEvents::None
    {
        return None;
    }
    if !in_portal_pass
        && (matches!(node.get_kind(), NodeKind::Portal) || layout.get_position() == Position::Fixed)
    {
        return None;
    }
    let raw_bounds: Rect = node.get_layout_box().get_bounds();
    let shifted_bounds: Rect = Rect::new(
        raw_bounds.get_x() - scroll_offset.get_x(),
        raw_bounds.get_y() - scroll_offset.get_y(),
        raw_bounds.get_width(),
        raw_bounds.get_height(),
    );
    let effective_clip: Option<Rect> = match layout.get_overflow() {
        Overflow::Hidden | Overflow::Scroll => match clip_rect {
            Some(parent_clip) => parent_clip.intersect(shifted_bounds),
            None => Some(shifted_bounds),
        },
        Overflow::Visible => clip_rect,
    };
    if let Some(clip) = effective_clip
        && !clip.contains(point)
    {
        return None;
    }
    let child_scroll: Point = Point::new(
        scroll_offset.get_x() + node.get_layout_box().get_scroll_offset().get_x(),
        scroll_offset.get_y() + node.get_layout_box().get_scroll_offset().get_y(),
    );
    let mut sorted_children: Vec<(i32, usize, NodeId)> = node
        .get_children()
        .iter()
        .copied()
        .enumerate()
        .filter_map(|(order, cid): (usize, NodeId)| {
            tree.get_node(cid)
                .map(|c: &SceneNode| (c.get_style().get_layout().get_z_index(), order, cid))
        })
        .collect();
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
    if shifted_bounds.contains(point) {
        Some(node_id)
    } else {
        None
    }
}
