use super::*;

#[test]
fn flexbox_layout_and_cache_skip_clean_subtrees() {
    let mut tree: SceneTree = SceneTree::default();
    let root: NodeId = tree.get_root();
    let row: NodeId = tree.create_view(
        Style::empty()
            .row()
            .width(Length::Px(400.0))
            .height(Length::Px(100.0))
            .gap(20.0)
            .padding(Edges::all(10.0)),
    );
    let left: NodeId = tree.create_view(
        Style::empty()
            .width(Length::Px(100.0))
            .height(Length::Px(40.0)),
    );
    let right: NodeId = tree.create_view(Style::empty().flex_grow(1.0).height(Length::Px(40.0)));
    tree.append_child(row, left);
    tree.append_child(row, right);
    tree.append_child(root, row);
    let mut engine: LayoutEngine = LayoutEngine::default();
    let first: LayoutStats = engine.compute(&mut tree, Size::new(800.0, 600.0));
    assert!(first.get_nodes_computed() >= 4);
    let left_rect: Rect = tree.get_node(left).unwrap().get_layout_box().get_bounds();
    let right_rect: Rect = tree.get_node(right).unwrap().get_layout_box().get_bounds();
    assert!((left_rect.get_x() - 10.0).abs() < 0.01);
    assert!((left_rect.get_width() - 100.0).abs() < 0.01);
    assert!((right_rect.get_x() - 130.0).abs() < 0.01);
    assert!((right_rect.get_width() - 260.0).abs() < 0.01);
    let second: LayoutStats = engine.compute(&mut tree, Size::new(800.0, 600.0));
    assert_eq!(second.get_nodes_computed(), 0);
    assert_eq!(second.get_subtrees_skipped(), 1);
}

#[test]
fn grid_layout_and_zindex_hit_test_work() {
    let mut tree: SceneTree = SceneTree::default();
    let root: NodeId = tree.get_root();
    let grid: NodeId = tree.create_view(Style::empty().grid(2).width(Length::Px(220.0)).gap(20.0));
    let cell_a: NodeId = tree.create_view(Style::empty().height(Length::Px(50.0)));
    let cell_b: NodeId = tree.create_view(Style::empty().height(Length::Px(50.0)));
    let overlay: NodeId = tree.create_view(
        Style::empty()
            .position(Position::Fixed)
            .inset(Some(0.0), None, None, Some(0.0))
            .width(Length::Px(220.0))
            .height(Length::Px(50.0))
            .z_index(100),
    );
    tree.append_child(grid, cell_a);
    tree.append_child(grid, cell_b);
    tree.append_child(root, grid);
    tree.append_child(root, overlay);
    let mut engine: LayoutEngine = LayoutEngine::default();
    let _: LayoutStats = engine.compute(&mut tree, Size::new(400.0, 300.0));
    let rect_b: Rect = tree.get_node(cell_b).unwrap().get_layout_box().get_bounds();
    assert!((rect_b.get_x() - 120.0).abs() < 0.01);
    assert!((rect_b.get_width() - 100.0).abs() < 0.01);
    assert_eq!(hit_test(&tree, Point::new(15.0, 15.0)), Some(overlay));
}
