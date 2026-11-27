use super::*;

#[test]
fn flexbox_layout_and_cache_skip_clean_subtrees() {
    let mut tree: SceneTree = SceneTree::default();
    let root: NodeId = tree.get_root();
    let row: NodeId = tree.create_view(
        Style::default()
            .row()
            .width(Length::Px(400.0))
            .height(Length::Px(100.0))
            .gap(20.0)
            .padding(Edges::all(10.0)),
    );
    let left: NodeId = tree.create_view(
        Style::default()
            .width(Length::Px(100.0))
            .height(Length::Px(40.0)),
    );
    let right: NodeId = tree.create_view(Style::default().flex_grow(1.0).height(Length::Px(40.0)));
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
    let grid: NodeId =
        tree.create_view(Style::default().grid(2).width(Length::Px(220.0)).gap(20.0));
    let cell_a: NodeId = tree.create_view(Style::default().height(Length::Px(50.0)));
    let cell_b: NodeId = tree.create_view(Style::default().height(Length::Px(50.0)));
    let overlay: NodeId = tree.create_view(
        Style::default()
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
        Style::default()
            .row()
            .width(Length::Percent(100.0))
            .height(Length::Percent(100.0))
            .overflow(Overflow::Hidden),
    );
    let nav: NodeId = tree.create_view(
        Style::default()
            .column()
            .width(Length::Px(240.0))
            .height(Length::Percent(100.0)),
    );
    let main_area: NodeId = tree.create_scroll_view(
        Style::default()
            .column()
            .flex_grow(1.0)
            .height(Length::Percent(100.0))
            .padding(Edges::all(24.0)),
    );
    let table_row: NodeId = tree.create_view(Style::default().row().width(Length::Percent(100.0)));
    let cell_a: NodeId =
        tree.create_view(Style::default().flex_grow(1.0).overflow(Overflow::Hidden));
    let cell_b: NodeId =
        tree.create_view(Style::default().flex_grow(1.0).overflow(Overflow::Hidden));
    let txt_a: NodeId = tree.create_text("Target".to_string(), Style::default().font_size(13.0));
    let txt_b: NodeId = tree.create_text(
        "Metal / Vulkan".to_string(),
        Style::default().font_size(13.0),
    );
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
        Style::default()
            .column()
            .width(Length::Px(400.0))
            .height(Length::Px(200.0))
            .padding(Edges::all(16.0)),
    );
    let tall_child: NodeId = tree.create_view(
        Style::default()
            .width(Length::Percent(100.0))
            .height(Length::Px(800.0)),
    );
    let portal: NodeId = tree.create_portal(
        Style::default()
            .position(Position::Fixed)
            .inset(Some(0.0), Some(0.0), Some(0.0), Some(0.0))
            .width(Length::Percent(100.0))
            .height(Length::Percent(100.0))
            .justify(JustifyContent::Center)
            .align(AlignItems::Center)
            .z_index(1000),
    );
    let dialog_btn: NodeId = tree.create_view(
        Style::default()
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
    let mut dl_land_phone: DisplayList = DisplayList::default();
    PaintPass::build(
        &tree,
        Size::new(900.0, 411.0),
        1.0,
        Color::hex(0xFFFFFF),
        &mut dl_land_phone,
    );
    let land_phone_scrollbars: usize = dl_land_phone
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
        land_phone_scrollbars, 0,
        "landscape phone viewports must hide scrollbars"
    );
    let mut dl: DisplayList = DisplayList::default();
    PaintPass::build(
        &tree,
        Size::new(800.0, 640.0),
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

#[test]
fn percent_width_flex_child_does_not_resolve_percentage_twice() {
    let mut tree: SceneTree = SceneTree::default();
    let root: NodeId = tree.create_node(
        NodeKind::View,
        Style::default()
            .row()
            .width(Length::Percent(100.0))
            .height(Length::Percent(100.0)),
    );
    tree.set_root(root);
    let drawer: NodeId = tree.create_node(
        NodeKind::ScrollView,
        Style::default()
            .column()
            .width(Length::Percent(64.0))
            .max_width(Length::Px(232.0))
            .height(Length::Percent(100.0))
            .padding(Edges::all(16.0)),
    );
    let item: NodeId = tree.create_node(
        NodeKind::View,
        Style::default()
            .width(Length::Percent(100.0))
            .height(Length::Px(36.0)),
    );
    tree.append_child(drawer, item);
    tree.append_child(root, drawer);
    let mut engine: LayoutEngine = LayoutEngine::default();
    let _: LayoutStats = engine.compute(&mut tree, Size::new(411.0, 914.0));
    let drawer_w: f32 = tree
        .get_node(drawer)
        .expect("drawer node")
        .get_layout_box()
        .get_bounds()
        .get_width();
    let item_w: f32 = tree
        .get_node(item)
        .expect("item node")
        .get_layout_box()
        .get_bounds()
        .get_width();
    assert!((drawer_w - 232.0).abs() < 1.0, "drawer_w={drawer_w}");
    assert!((item_w - 200.0).abs() < 1.0, "item_w={item_w}");
}

#[test]
fn scroll_offset_preserved_across_freeze_resume_and_inplace_restyle() {
    reset_reactive_runtime();
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(480, 320).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let (page_a, scroll_a, page_b, scroll_b): (NodeId, NodeId, NodeId, NodeId) =
        app.get_scene().with_write(
            |tree: &mut SceneTree| {
                let root: NodeId = tree.get_root();
                let pa: NodeId = tree.create_view(
                    Style::default()
                        .column()
                        .width(Length::Percent(100.0))
                        .height(Length::Percent(100.0)),
                );
                let sa: NodeId = tree.create_scroll_view(
                    Style::default()
                        .column()
                        .width(Length::Px(400.0))
                        .height(Length::Px(220.0))
                        .background(COLOR_WHITE),
                );
                let content_a: NodeId = tree.create_view(
                    Style::default()
                        .width(Length::Px(960.0))
                        .height(Length::Px(1200.0))
                        .background(Color::hex(0xF1F5F9)),
                );
                tree.append_child(sa, content_a);
                tree.append_child(pa, sa);
                let pb: NodeId = tree.create_view(
                    Style::default()
                        .column()
                        .width(Length::Percent(100.0))
                        .height(Length::Percent(100.0))
                        .display(Display::None),
                );
                let sb: NodeId = tree.create_scroll_view(
                    Style::default()
                        .column()
                        .width(Length::Px(400.0))
                        .height(Length::Px(220.0))
                        .background(COLOR_WHITE),
                );
                let content_b: NodeId = tree.create_view(
                    Style::default()
                        .width(Length::Percent(100.0))
                        .height(Length::Px(1000.0)),
                );
                tree.append_child(sb, content_b);
                tree.append_child(pb, sb);
                tree.replace_children(root, vec![pa, pb]);
                (pa, sa, pb, sb)
            },
            (
                NodeId::default(),
                NodeId::default(),
                NodeId::default(),
                NodeId::default(),
            ),
        );
    let _: FrameStats = app.step_frame().unwrap();
    app.get_scene().with_write(
        |tree: &mut SceneTree| {
            tree.set_scroll_offset(scroll_a, Point::new(145.0, 320.0));
        },
        (),
    );
    app.get_scene().freeze_subtree(page_a);
    app.get_scene().resume_subtree(page_b);
    let _: FrameStats = app.step_frame().unwrap();
    let wid: WindowId = app.get_window_state().get_id();
    app.dispatch_window_event(WindowEvent::Scroll(wid, Point::new(0.0, 90.0)));
    let _: FrameStats = app.step_frame().unwrap();
    let (sa_frozen, sb_scrolled): (Point, Point) = app.get_scene().with_read(
        |tree: &SceneTree| {
            (
                tree.get_node(scroll_a)
                    .unwrap()
                    .get_layout_box()
                    .get_scroll_offset(),
                tree.get_node(scroll_b)
                    .unwrap()
                    .get_layout_box()
                    .get_scroll_offset(),
            )
        },
        (Point::default(), Point::default()),
    );
    assert!((sa_frozen.get_x() - 145.0).abs() < 0.01);
    assert!((sa_frozen.get_y() - 320.0).abs() < 0.01);
    assert!((sb_scrolled.get_y() - 90.0).abs() < 0.01);
    app.get_scene().with_write(
        |tree: &mut SceneTree| {
            let mut st: Style = tree.get_node(scroll_a).unwrap().get_style();
            st = st.background(Color::hex(0x0F172A)).color(COLOR_WHITE);
            tree.set_style(scroll_a, st);
        },
        (),
    );
    let _: FrameStats = app.step_frame().unwrap();
    app.get_scene().freeze_subtree(page_b);
    app.get_scene().resume_subtree(page_a);
    let _: FrameStats = app.step_frame().unwrap();
    let sa_resumed: Point = app.get_scene().with_read(
        |tree: &SceneTree| {
            tree.get_node(scroll_a)
                .unwrap()
                .get_layout_box()
                .get_scroll_offset()
        },
        Point::default(),
    );
    assert!((sa_resumed.get_x() - 145.0).abs() < 0.01);
    assert!((sa_resumed.get_y() - 320.0).abs() < 0.01);
    app.get_scene().with_write(
        |tree: &mut SceneTree| {
            let mut st: Style = tree.get_node(scroll_a).unwrap().get_style();
            st = st
                .background(Color::hex(0x1E293B))
                .color(Color::hex(0xE2E8F0));
            tree.set_style(scroll_a, st);
        },
        (),
    );
    let restyle_stats: FrameStats = app.step_frame().unwrap();
    assert_eq!(restyle_stats.get_layout_nodes(), 0);
    let sa_after_restyle: Point = app.get_scene().with_read(
        |tree: &SceneTree| {
            tree.get_node(scroll_a)
                .unwrap()
                .get_layout_box()
                .get_scroll_offset()
        },
        Point::default(),
    );
    assert!((sa_after_restyle.get_x() - 145.0).abs() < 0.01);
    assert!((sa_after_restyle.get_y() - 320.0).abs() < 0.01);
    app.get_scene().with_write(
        |tree: &mut SceneTree| {
            tree.set_scroll_offset(scroll_a, Point::default());
        },
        (),
    );
    let sa_reset: Point = app.get_scene().with_read(
        |tree: &SceneTree| {
            tree.get_node(scroll_a)
                .unwrap()
                .get_layout_box()
                .get_scroll_offset()
        },
        Point::new(99.0, 99.0),
    );
    assert_eq!(sa_reset, Point::default());
}

#[test]
fn nested_bidirectional_scroll_container_scrolls_both_axes_and_chains_leftover_to_parent() {
    reset_reactive_runtime();
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(800, 640).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let (outer_sc, inner_sc): (NodeId, NodeId) = app.get_scene().with_write(
        |tree: &mut SceneTree| {
            let root: NodeId = tree.get_root();
            let outer: NodeId = tree.create_scroll_view(
                Style::default()
                    .column()
                    .width(Length::Percent(100.0))
                    .height(Length::Px(400.0))
                    .padding(Edges::all(16.0))
                    .gap(12.0),
            );
            let top_pad: NodeId = tree.create_view(
                Style::default()
                    .width(Length::Percent(100.0))
                    .height(Length::Px(40.0)),
            );
            let inner: NodeId = tree.create_scroll_view(
                Style::default()
                    .column()
                    .width(Length::Px(320.0))
                    .max_height(Length::Px(160.0))
                    .padding(Edges::all(8.0)),
            );
            let code_sheet: NodeId = tree.create_view(
                Style::default()
                    .column()
                    .min_width(Length::Px(900.0))
                    .gap(4.0),
            );
            for line_idx in 0..16u32 {
                let line: NodeId = tree.create_text(
                    format!("let line_{line_idx}: &str = \"long_horizontal_code_segment_for_overflow_testing\";"),
                    Style::default()
                        .font_size(13.0)
                        .text_overflow(TextOverflow::Clip),
                );
                tree.append_child(code_sheet, line);
            }
            let bottom_pad: NodeId = tree.create_view(
                Style::default()
                    .width(Length::Percent(100.0))
                    .height(Length::Px(900.0)),
            );
            tree.append_child(inner, code_sheet);
            tree.append_child(outer, top_pad);
            tree.append_child(outer, inner);
            tree.append_child(outer, bottom_pad);
            tree.replace_children(root, vec![outer]);
            (outer, inner)
        },
        (NodeId::default(), NodeId::default()),
    );
    let _: FrameStats = app.step_frame().unwrap();
    let (inner_max_x, inner_max_y, outer_max_y): (f32, f32, f32) = app.get_scene().with_read(
        |tree: &SceneTree| {
            let ib: LayoutBox = tree.get_node(inner_sc).unwrap().get_layout_box();
            let ob: LayoutBox = tree.get_node(outer_sc).unwrap().get_layout_box();
            (
                (ib.get_content_size().get_width() - ib.get_content_bounds().get_width()).max(0.0),
                (ib.get_content_size().get_height() - ib.get_content_bounds().get_height())
                    .max(0.0),
                (ob.get_content_size().get_height() - ob.get_content_bounds().get_height())
                    .max(0.0),
            )
        },
        (0.0, 0.0, 0.0),
    );
    assert!(inner_max_x > 500.0, "inner_max_x={inner_max_x}");
    assert!(inner_max_y > 150.0, "inner_max_y={inner_max_y}");
    assert!(outer_max_y > 500.0, "outer_max_y={outer_max_y}");
    let wid: WindowId = app.get_window_state().get_id();
    let inner_bounds: Rect = app.get_scene().node_screen_bounds(inner_sc).unwrap();
    let inside_inner: Point = Point::new(
        inner_bounds.get_x() + inner_bounds.get_width() * 0.5,
        inner_bounds.get_y() + inner_bounds.get_height() * 0.5,
    );
    app.dispatch_window_event(WindowEvent::PointerMove(
        wid,
        PointerEvent::new(
            1,
            PointerDevice::Mouse,
            inside_inner,
            PointerButton::Primary,
            0.0,
            Modifiers::default(),
        ),
    ));
    app.dispatch_window_event(WindowEvent::Scroll(wid, Point::new(120.0, 0.0)));
    let after_h: Point = app.get_scene().with_read(
        |tree: &SceneTree| {
            tree.get_node(inner_sc)
                .unwrap()
                .get_layout_box()
                .get_scroll_offset()
        },
        Point::default(),
    );
    assert!((after_h.get_x() - 120.0).abs() < 0.01);
    assert!(after_h.get_y().abs() < 0.01);
    app.dispatch_window_event(WindowEvent::PointerMove(
        wid,
        PointerEvent::new(
            1,
            PointerDevice::Mouse,
            inside_inner,
            PointerButton::Primary,
            0.0,
            Modifiers::new(true, false, false, false),
        ),
    ));
    app.dispatch_window_event(WindowEvent::Scroll(wid, Point::new(0.0, 80.0)));
    let after_shift: Point = app.get_scene().with_read(
        |tree: &SceneTree| {
            tree.get_node(inner_sc)
                .unwrap()
                .get_layout_box()
                .get_scroll_offset()
        },
        Point::default(),
    );
    assert!((after_shift.get_x() - 200.0).abs() < 0.01);
    app.dispatch_window_event(WindowEvent::PointerMove(
        wid,
        PointerEvent::new(
            1,
            PointerDevice::Mouse,
            inside_inner,
            PointerButton::Primary,
            0.0,
            Modifiers::default(),
        ),
    ));
    app.dispatch_window_event(WindowEvent::Scroll(wid, Point::new(0.0, 100.0)));
    let (inner_v1, outer_v1): (Point, Point) = app.get_scene().with_read(
        |tree: &SceneTree| {
            (
                tree.get_node(inner_sc)
                    .unwrap()
                    .get_layout_box()
                    .get_scroll_offset(),
                tree.get_node(outer_sc)
                    .unwrap()
                    .get_layout_box()
                    .get_scroll_offset(),
            )
        },
        (Point::default(), Point::default()),
    );
    assert!((inner_v1.get_y() - 100.0).abs() < 0.01);
    assert!(outer_v1.get_y().abs() < 0.01);
    app.dispatch_window_event(WindowEvent::Scroll(
        wid,
        Point::new(0.0, inner_max_y + 75.0),
    ));
    let (inner_v2, outer_v2): (Point, Point) = app.get_scene().with_read(
        |tree: &SceneTree| {
            (
                tree.get_node(inner_sc)
                    .unwrap()
                    .get_layout_box()
                    .get_scroll_offset(),
                tree.get_node(outer_sc)
                    .unwrap()
                    .get_layout_box()
                    .get_scroll_offset(),
            )
        },
        (Point::default(), Point::default()),
    );
    assert!((inner_v2.get_y() - inner_max_y).abs() < 0.5);
    assert!(
        outer_v2.get_y() > 50.0,
        "leftover vertical scroll must propagate to outer container: outer_y={}",
        outer_v2.get_y()
    );
    let mut dl: DisplayList = DisplayList::default();
    app.get_scene().with_read(
        |tree: &SceneTree| {
            PaintPass::build(tree, Size::new(800.0, 640.0), 1.0, COLOR_WHITE, &mut dl);
        },
        (),
    );
    let inner_screen: Rect = app.get_scene().node_screen_bounds(inner_sc).unwrap();
    let has_v_thumb: bool = dl.get_commands().iter().any(|cmd: &DrawCommand| {
        matches!(
            cmd,
            DrawCommand::FillRoundedRect { rect, .. }
                if rect.get_width() <= 6.5
                    && rect.get_height() >= 18.0
                    && rect.get_x() >= inner_screen.right() - 12.0
                    && rect.get_x() <= inner_screen.right()
                    && rect.get_y() >= inner_screen.get_y()
                    && rect.bottom() <= inner_screen.bottom()
        )
    });
    let has_h_thumb: bool = dl.get_commands().iter().any(|cmd: &DrawCommand| {
        matches!(
            cmd,
            DrawCommand::FillRoundedRect { rect, .. }
                if rect.get_height() <= 6.5
                    && rect.get_width() >= 18.0
                    && rect.get_y() >= inner_screen.bottom() - 12.0
                    && rect.get_y() <= inner_screen.bottom()
                    && rect.get_x() >= inner_screen.get_x()
                    && rect.right() <= inner_screen.right()
        )
    });
    assert!(has_v_thumb, "expected vertical scrollbar thumb on inner_sc");
    assert!(
        has_h_thumb,
        "expected horizontal scrollbar thumb on inner_sc"
    );
    let h_bar_y: f32 = inner_screen.bottom() - 4.0;
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::new(
            1,
            PointerDevice::Mouse,
            Point::new(inner_screen.get_x() + 110.0, h_bar_y),
            PointerButton::Primary,
            1.0,
            Modifiers::default(),
        ),
    ));
    app.dispatch_window_event(WindowEvent::PointerMove(
        wid,
        PointerEvent::new(
            1,
            PointerDevice::Mouse,
            Point::new(inner_screen.get_x() + 160.0, h_bar_y),
            PointerButton::Primary,
            1.0,
            Modifiers::default(),
        ),
    ));
    app.dispatch_window_event(WindowEvent::PointerUp(
        wid,
        PointerEvent::new(
            1,
            PointerDevice::Mouse,
            Point::new(inner_screen.get_x() + 160.0, h_bar_y),
            PointerButton::Primary,
            0.0,
            Modifiers::default(),
        ),
    ));
    let after_hbar_drag_x: f32 = app.get_scene().with_read(
        |tree: &SceneTree| {
            tree.get_node(inner_sc)
                .unwrap()
                .get_layout_box()
                .get_scroll_offset()
                .get_x()
        },
        0.0,
    );
    assert!(
        after_hbar_drag_x > 220.0 && after_hbar_drag_x < inner_max_x - 80.0,
        "dragging horizontal scrollbar must move scroll_x into middle range: got {after_hbar_drag_x}"
    );
    let touch_start: Point = Point::new(inner_screen.get_x() + 220.0, inner_screen.get_y() + 60.0);
    let touch_end: Point = Point::new(inner_screen.get_x() + 140.0, inner_screen.get_y() + 60.0);
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::new(
            9,
            PointerDevice::Touch,
            touch_start,
            PointerButton::Primary,
            1.0,
            Modifiers::default(),
        ),
    ));
    app.dispatch_window_event(WindowEvent::PointerMove(
        wid,
        PointerEvent::new(
            9,
            PointerDevice::Touch,
            touch_end,
            PointerButton::Primary,
            1.0,
            Modifiers::default(),
        ),
    ));
    app.dispatch_window_event(WindowEvent::PointerUp(
        wid,
        PointerEvent::new(
            9,
            PointerDevice::Touch,
            touch_end,
            PointerButton::Primary,
            0.0,
            Modifiers::default(),
        ),
    ));
    let after_touch_x: f32 = app.get_scene().with_read(
        |tree: &SceneTree| {
            tree.get_node(inner_sc)
                .unwrap()
                .get_layout_box()
                .get_scroll_offset()
                .get_x()
        },
        0.0,
    );
    assert!(
        after_touch_x > after_hbar_drag_x + 20.0,
        "horizontal touch drag must scroll inner_sc: before={after_hbar_drag_x} after={after_touch_x}"
    );
}
