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

#[test]
fn flex_grow_sidebar_and_table_cells_never_overflow_viewport() {
    let mut tree: SceneTree = SceneTree::default();
    let root: NodeId = tree.get_root();
    let shell: NodeId = tree.create_view(
        Style::empty()
            .row()
            .width(Length::Percent(100.0))
            .height(Length::Percent(100.0))
            .overflow(Overflow::Hidden),
    );
    let nav: NodeId = tree.create_view(
        Style::empty()
            .column()
            .width(Length::Px(240.0))
            .height(Length::Percent(100.0)),
    );
    let main_area: NodeId = tree.create_scroll_view(
        Style::empty()
            .column()
            .flex_grow(1.0)
            .height(Length::Percent(100.0))
            .padding(Edges::all(24.0)),
    );
    let table_row: NodeId = tree.create_view(Style::empty().row().width(Length::Percent(100.0)));
    let cell_a: NodeId = tree.create_view(Style::empty().flex_grow(1.0).overflow(Overflow::Hidden));
    let cell_b: NodeId = tree.create_view(Style::empty().flex_grow(1.0).overflow(Overflow::Hidden));
    let txt_a: NodeId = tree.create_text("Target".to_string(), Style::empty().font_size(13.0));
    let txt_b: NodeId =
        tree.create_text("Metal / Vulkan".to_string(), Style::empty().font_size(13.0));
    tree.append_child(cell_a, txt_a);
    tree.append_child(cell_b, txt_b);
    tree.append_child(table_row, cell_a);
    tree.append_child(table_row, cell_b);
    tree.append_child(main_area, table_row);
    tree.append_child(shell, nav);
    tree.append_child(shell, main_area);
    tree.append_child(root, shell);
    let mut engine: LayoutEngine = LayoutEngine::default();
    let _: LayoutStats = engine.compute(&mut tree, Size::new(1200.0, 800.0));
    let main_b: Rect = tree
        .get_node(main_area)
        .unwrap()
        .get_layout_box()
        .get_bounds();
    let row_b: Rect = tree
        .get_node(table_row)
        .unwrap()
        .get_layout_box()
        .get_bounds();
    assert!((main_b.get_x() - 240.0).abs() < 0.01);
    assert!((main_b.get_width() - 960.0).abs() < 0.01);
    assert!(main_b.right() <= 1200.01);
    assert!(row_b.get_height() > 10.0 && row_b.get_height() < 60.0);
    assert!(row_b.right() <= 1200.01);
}

#[test]
fn word_boundary_text_wrap_and_uniform_baseline_glyph_advances_prevent_overlap() {
    let measurer: DeterministicTextMeasurer = DeterministicTextMeasurer;
    let style: TextStyle = TextStyle::default();
    let result: TextLayoutResult = measurer.measure("Hello Native World", style, Some(90.0));
    assert!(result.get_lines().len() >= 2);
    assert_eq!(result.get_lines()[0].get_text().as_str(), "Hello Native");
    let mut atlas: GlyphAtlas = GlyphAtlas::default();
    let provider: NativeSystemFont = NativeSystemFont::system();
    let g_cap: RasterizedGlyph = atlas
        .get_or_rasterize_with(GlyphKey::from_style('W', 28.0, FontWeight::Bold), &provider)
        .clone();
    let g_desc: RasterizedGlyph = atlas
        .get_or_rasterize_with(GlyphKey::from_style('g', 28.0, FontWeight::Bold), &provider)
        .clone();
    let g_narrow: RasterizedGlyph = atlas
        .get_or_rasterize_with(
            GlyphKey::from_style('i', 28.0, FontWeight::Normal),
            &provider,
        )
        .clone();
    assert!(g_cap.get_advance() > g_narrow.get_advance());
    assert!(g_cap.get_advance() >= 10);
    assert_eq!(g_cap.get_height(), g_desc.get_height());
}

#[test]
fn top_layer_portal_hit_test_and_overlay_scrollbar_commands_work_when_scrolled() {
    let mut tree: SceneTree = SceneTree::default();
    let root: NodeId = tree.get_root();
    let scroller: NodeId = tree.create_scroll_view(
        Style::empty()
            .column()
            .width(Length::Px(400.0))
            .height(Length::Px(200.0))
            .padding(Edges::all(16.0)),
    );
    let tall_child: NodeId = tree.create_view(
        Style::empty()
            .width(Length::Percent(100.0))
            .height(Length::Px(800.0)),
    );
    let portal: NodeId = tree.create_portal(
        Style::empty()
            .position(Position::Fixed)
            .inset(Some(0.0), Some(0.0), Some(0.0), Some(0.0))
            .width(Length::Percent(100.0))
            .height(Length::Percent(100.0))
            .justify(JustifyContent::Center)
            .align(AlignItems::Center)
            .z_index(1000),
    );
    let dialog_btn: NodeId = tree.create_view(
        Style::empty()
            .width(Length::Px(120.0))
            .height(Length::Px(40.0))
            .background(Color::hex(0x18181B))
            .cursor(CursorIcon::Pointer),
    );
    tree.append_child(portal, dialog_btn);
    tree.append_child(scroller, tall_child);
    tree.append_child(scroller, portal);
    tree.append_child(root, scroller);
    let mut engine: LayoutEngine = LayoutEngine::default();
    let _: LayoutStats = engine.compute(&mut tree, Size::new(400.0, 200.0));
    tree.set_scroll_offset(scroller, Point::new(0.0, 300.0));
    assert_eq!(hit_test(&tree, Point::new(200.0, 100.0)), Some(dialog_btn));
    let mut dl_mobile: DisplayList = DisplayList::default();
    PaintPass::build(
        &tree,
        Size::new(400.0, 200.0),
        1.0,
        Color::hex(0xFFFFFF),
        &mut dl_mobile,
    );
    let mobile_scrollbars: usize = dl_mobile
        .get_commands()
        .iter()
        .filter(|cmd: &&DrawCommand| {
            matches!(
                cmd,
                DrawCommand::FillRoundedRect { rect, .. }
                    if rect.get_width() <= 6.5 && rect.get_x() > 380.0
            )
        })
        .count();
    assert_eq!(
        mobile_scrollbars, 0,
        "mobile compact viewports (< 640px) must hide scrollbars"
    );
    let mut dl: DisplayList = DisplayList::default();
    PaintPass::build(
        &tree,
        Size::new(800.0, 200.0),
        1.0,
        Color::hex(0xFFFFFF),
        &mut dl,
    );
    let scrollbar_rects: usize = dl
        .get_commands()
        .iter()
        .filter(|cmd: &&DrawCommand| {
            matches!(
                cmd,
                DrawCommand::FillRoundedRect { rect, .. }
                    if rect.get_width() <= 6.5 && rect.get_x() > 380.0
            )
        })
        .count();
    assert!(scrollbar_rects >= 1);
}
