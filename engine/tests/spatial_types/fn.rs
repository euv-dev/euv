use super::*;

fn point(x: f64, y: f64) -> Vector2D {
    Vector2D::new(x, y)
}

#[test]
fn two_identical_boxes_overlap() {
    assert!(QuadTree2D::boxes_overlap(
        point(0.0, 0.0),
        point(10.0, 10.0),
        point(0.0, 0.0),
        point(10.0, 10.0)
    ));
}

#[test]
fn boxes_sharing_only_an_edge_overlap() {
    assert!(
        QuadTree2D::boxes_overlap(
            point(0.0, 0.0),
            point(10.0, 10.0),
            point(10.0, 0.0),
            point(20.0, 10.0)
        ),
        "the comparison is <=, so a shared edge counts as contact"
    );
    assert!(
        QuadTree2D::boxes_overlap(
            point(0.0, 0.0),
            point(10.0, 10.0),
            point(10.0, 10.0),
            point(20.0, 20.0)
        ),
        "a single shared corner is contact too"
    );
}

#[test]
fn boxes_separated_by_a_gap_do_not_overlap() {
    assert!(!QuadTree2D::boxes_overlap(
        point(0.0, 0.0),
        point(10.0, 10.0),
        point(10.001, 0.0),
        point(20.0, 10.0)
    ));
}

#[test]
fn containment_counts_as_overlap() {
    assert!(QuadTree2D::boxes_overlap(
        point(0.0, 0.0),
        point(100.0, 100.0),
        point(40.0, 40.0),
        point(60.0, 60.0)
    ));
}

#[test]
fn separation_along_either_axis_alone_is_enough_to_reject() {
    let a_min: Vector2D = point(0.0, 0.0);
    let a_max: Vector2D = point(10.0, 10.0);
    assert!(
        !QuadTree2D::boxes_overlap(a_min, a_max, point(-5.0, 0.0), point(-1.0, 10.0)),
        "separated on x only"
    );
    assert!(
        !QuadTree2D::boxes_overlap(a_min, a_max, point(0.0, -5.0), point(10.0, -1.0)),
        "separated on y only"
    );
    assert!(
        !QuadTree2D::boxes_overlap(a_min, a_max, point(11.0, 11.0), point(20.0, 20.0)),
        "separated on both axes"
    );
}

#[test]
fn overlap_is_symmetric() {
    let cases: [(Vector2D, Vector2D, Vector2D, Vector2D); 4] = [
        (
            point(0.0, 0.0),
            point(4.0, 4.0),
            point(2.0, 2.0),
            point(9.0, 9.0),
        ),
        (
            point(0.0, 0.0),
            point(4.0, 4.0),
            point(4.0, 0.0),
            point(9.0, 4.0),
        ),
        (
            point(0.0, 0.0),
            point(4.0, 4.0),
            point(5.0, 5.0),
            point(9.0, 9.0),
        ),
        (
            point(-3.0, -3.0),
            point(3.0, 3.0),
            point(0.0, 0.0),
            point(0.0, 0.0),
        ),
    ];
    for (a_min, a_max, b_min, b_max) in cases {
        assert_eq!(
            QuadTree2D::boxes_overlap(a_min, a_max, b_min, b_max),
            QuadTree2D::boxes_overlap(b_min, b_max, a_min, a_max),
            "overlap must not depend on the argument order"
        );
    }
}

#[test]
fn a_cell_key_is_a_coordinate_pair_flexible_about_the_sign() {
    let key: CellKey2D = (-2, 7);
    assert_eq!(key.0, -2, "cells are indexed on both sides of the origin");
    assert_eq!(key.1, 7);
}

#[test]
fn a_cell_key3d_carries_a_third_axis() {
    let key: CellKey3D = (1, -1, 0);
    let flat: CellKey2D = (key.0, key.1);
    assert_eq!((key.0, key.1, key.2), (1, -1, 0));
    assert_ne!(
        (flat.0, flat.1, 1),
        (key.0, key.1, key.2),
        "the z index is a separate axis, not derived from x and y"
    );
}

#[test]
fn cell_entries_are_body_indices() {
    let entries: CellEntries = vec![3, 11];
    assert_eq!(entries, vec![3, 11]);
    assert_eq!(entries.first(), Some(&3));
}

#[test]
fn a_cell_map_holds_one_entry_list_per_key() {
    let mut map: SpatialCellMap2D = SpatialCellMap2D::new();
    map.insert((0, 0), vec![1, 2]);
    map.insert((1, 0), vec![3]);
    assert_eq!(map.len(), 2);
    assert_eq!(map.get(&(0, 0)).map(|v: &CellEntries| v.len()), Some(2));
    assert_eq!(
        map.get(&(9, 9)),
        None,
        "an unoccupied cell must read as absent rather than empty"
    );
}

#[test]
fn a_cell_map3d_keys_on_three_indices() {
    let mut map: SpatialCellMap3D = SpatialCellMap3D::new();
    map.insert((0, 0, 0), vec![5]);
    assert_eq!(map.get(&(0, 0, 0)).map(|v: &CellEntries| v.len()), Some(1));
    assert_eq!(map.get(&(0, 0, 1)), None);
}

#[test]
fn quad_tree_children_are_a_fixed_four_slot_handle_array() {
    let children: QuadTreeChildren2D = [1, 2, 3, 4];
    assert_eq!(children.len(), 4, "a quadtree subdivides into exactly four");
    let _: QuadTreeChildren2D = [usize::MAX; 4];
}

#[test]
fn a_quad_tree_entry_carries_its_body_index_verbatim() {
    let entry: QuadTreeEntry2D = QuadTreeEntry2D::new(42, point(0.0, 0.0), point(1.0, 1.0));
    let rendered: String = format!("{entry:?}");
    assert!(
        rendered.contains("index: 42"),
        "queries return the caller's body index unchanged, got: {rendered}"
    );
}

#[test]
fn a_quad_tree_node_starts_as_a_leaf_at_depth_zero() {
    let node: QuadTreeNode2D = QuadTreeNode2D::new(
        point(0.0, 0.0),
        point(8.0, 8.0),
        0,
        [usize::MAX; 4],
        true,
        false,
        Vec::new(),
    );
    let rendered: String = format!("{node:?}");
    assert!(
        rendered.contains("depth: 0"),
        "the root sits at level zero, got: {rendered}"
    );
    assert!(
        rendered.contains("leaf: true"),
        "a fresh node has subdivided nothing yet, got: {rendered}"
    );
}

#[test]
fn two_quad_tree_nodes_at_different_depths_stay_distinct() {
    let root: QuadTreeNode2D = QuadTreeNode2D::new(
        point(0.0, 0.0),
        point(8.0, 8.0),
        0,
        [usize::MAX; 4],
        true,
        false,
        Vec::new(),
    );
    let child: QuadTreeNode2D = QuadTreeNode2D::new(
        point(0.0, 0.0),
        point(4.0, 4.0),
        1,
        [usize::MAX; 4],
        false,
        false,
        Vec::new(),
    );
    assert_ne!(root, child);
}

#[test]
fn a_quad_tree_node_list_is_an_arena_addressed_by_handle() {
    let nodes: QuadTreeNodeList2D = vec![
        QuadTreeNode2D::new(
            point(0.0, 0.0),
            point(8.0, 8.0),
            0,
            [usize::MAX; 4],
            true,
            false,
            Vec::new(),
        ),
        QuadTreeNode2D::new(
            point(0.0, 0.0),
            point(4.0, 4.0),
            1,
            [usize::MAX; 4],
            false,
            false,
            Vec::new(),
        ),
    ];
    assert_eq!(nodes.len(), 2);
    assert!(
        format!("{:?}", nodes[1]).contains("depth: 1"),
        "a handle indexes the flat arena, so slot 1 must still be the child"
    );
}

#[test]
fn a_quad_tree_entry_list_holds_the_entries_of_one_node() {
    let list: QuadTreeEntryList2D = vec![
        QuadTreeEntry2D::new(0, point(0.0, 0.0), point(1.0, 1.0)),
        QuadTreeEntry2D::new(1, point(1.0, 1.0), point(2.0, 2.0)),
    ];
    assert_eq!(list.len(), 2);
    assert!(format!("{:?}", list[1]).contains("index: 1"));
}

#[test]
fn a_quad_tree_stack_is_an_explicit_depth_first_stack() {
    let mut stack: QuadTreeNodeStack2D = Vec::new();
    stack.push(0);
    stack.push(1);
    assert_eq!(stack.len(), 2);
    assert_eq!(
        stack.pop(),
        Some(1),
        "traversal is depth-first, last in first out"
    );
    assert_eq!(stack.pop(), Some(0));
    assert_eq!(stack.pop(), None);
}

#[test]
fn a_fresh_quad_tree_holds_no_bodies() {
    assert_eq!(QuadTree2D::with_half_extent(64.0).len(), 0);
    assert_eq!(QuadTree2D::with_default_size().len(), 0);
}
