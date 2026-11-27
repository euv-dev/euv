use super::*;

fn build_sample_terminal_scene() -> (SceneTree, NodeId, NodeId, NodeId, NodeId, Rc<Cell<u32>>) {
    let mut tree: SceneTree = SceneTree::default();
    let root: NodeId = tree.get_root();
    let card: NodeId =
        tree.create_view(Style::default().column().border(1.0, Color::hex(0x3F3F46)));
    let status: NodeId = tree.create_text("Status: Ready".to_string(), Style::default());
    let btn: NodeId = tree.create_view(Style::default().cursor(CursorIcon::Pointer));
    let btn_label: NodeId = tree.create_text("Run".to_string(), Style::default());
    tree.append_child(btn, btn_label);
    let clicks: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let clicks_cb: Rc<Cell<u32>> = Rc::clone(&clicks);
    if let Some(btn_node) = tree.get_node_mut(btn) {
        btn_node.get_mut_interaction().set_focusable(true);
        btn_node
            .get_mut_handlers()
            .set_on_click(Some(Rc::new(RefCell::new(move |_: &UiEvent| {
                clicks_cb.set(clicks_cb.get() + 1);
            }))));
    }
    let inp: NodeId = tree.create_input(
        InputNodeState::new(
            "secret123".to_string(),
            "Token".to_string(),
            9,
            true,
            false,
            false,
        ),
        Style::default(),
    );
    let img: NodeId = tree.create_image(ImageBuffer::new(320, 240, Vec::new()), Style::default());
    if let Some(img_node) = tree.get_node_mut(img) {
        img_node.set_a11y(Some(A11yProps::new(
            A11yRole::Image,
            "Chart Preview".to_string(),
        )));
    }
    let canvas: NodeId = tree.create_node(NodeKind::Canvas, Style::default());
    let scroll: NodeId = tree.create_scroll_view(Style::default().column());
    for idx in 0..8usize {
        let item: NodeId = tree.create_text(format!("Row {idx}"), Style::default());
        tree.append_child(scroll, item);
    }
    tree.append_child(card, status);
    tree.append_child(card, btn);
    tree.append_child(card, inp);
    tree.append_child(root, card);
    tree.append_child(root, img);
    tree.append_child(root, canvas);
    tree.append_child(root, scroll);
    (tree, status, btn, inp, scroll, clicks)
}

#[test]
fn terminal_char_width_handles_ascii_cjk_combining_and_control() {
    assert_eq!(terminal_char_width('A'), 1);
    assert_eq!(terminal_char_width(' '), 1);
    assert_eq!(terminal_char_width('中'), 2);
    assert_eq!(terminal_char_width('文'), 2);
    assert_eq!(terminal_char_width('あ'), 2);
    assert_eq!(terminal_char_width('한'), 2);
    assert_eq!(terminal_char_width('\u{0301}'), 0);
    assert_eq!(terminal_char_width('\u{200B}'), 0);
    assert_eq!(terminal_char_width('\x1b'), 0);
    assert_eq!(terminal_char_width('\n'), 0);
    assert_eq!(terminal_str_width("e\u{0301}中A"), 4);
}

#[test]
fn terminal_grid_wide_char_overwrite_clears_both_cells_and_edge_truncates() {
    let mut grid: TerminalCellGrid = TerminalCellGrid::allocate(5, 2);
    let written: u16 = grid.write_text(0, 0, 5, "中文A");
    assert_eq!(written, 5);
    assert_eq!(
        grid.cell(0, 0).map(|c: TerminalCell| c.get_ch()),
        Some('中')
    );
    assert!(
        grid.cell(1, 0)
            .is_some_and(|c: TerminalCell| c.is_continuation())
    );
    assert_eq!(
        grid.cell(2, 0).map(|c: TerminalCell| c.get_ch()),
        Some('文')
    );
    assert!(
        grid.cell(3, 0)
            .is_some_and(|c: TerminalCell| c.is_continuation())
    );
    assert_eq!(grid.cell(4, 0).map(|c: TerminalCell| c.get_ch()), Some('A'));
    let _: u16 = grid.put_cell(
        1,
        0,
        TerminalCell::styled('X', 1, None, None, false, false, false),
    );
    assert_eq!(grid.cell(0, 0).map(|c: TerminalCell| c.get_ch()), Some(' '));
    assert_eq!(grid.cell(1, 0).map(|c: TerminalCell| c.get_ch()), Some('X'));
    let _: u16 = grid.put_cell(
        2,
        0,
        TerminalCell::styled('Y', 1, None, None, false, false, false),
    );
    assert_eq!(grid.cell(2, 0).map(|c: TerminalCell| c.get_ch()), Some('Y'));
    assert_eq!(grid.cell(3, 0).map(|c: TerminalCell| c.get_ch()), Some(' '));
    assert!(
        !grid
            .cell(3, 0)
            .is_some_and(|c: TerminalCell| c.is_continuation())
    );
    let edge_written: u16 = grid.write_text(0, 1, 5, "ABC中");
    assert_eq!(edge_written, 5);
    let clipped: u16 = grid.write_text(4, 1, 1, "汉");
    assert_eq!(clipped, 0);
    assert_eq!(grid.to_plain_text().lines().next(), Some(" XY A"));
}

#[test]
fn terminal_renderer_diff_idle_zero_bytes_and_resize_full_sync() {
    let (mut tree, status, _, _, _, _): (SceneTree, NodeId, NodeId, NodeId, NodeId, Rc<Cell<u32>>) =
        build_sample_terminal_scene();
    let mut renderer: TerminalRenderer = TerminalRenderer::new();
    let (first_out, first_stats): (String, TerminalDiffStats) =
        renderer.render_tree(&tree, 40, 14, TerminalRenderMode::InteractiveTui);
    assert!(first_stats.get_full_sync());
    assert!(!first_stats.get_is_idle());
    assert_eq!(first_stats.get_total_cells(), 40 * 14);
    assert_eq!(first_stats.get_changed_cells(), 40 * 14);
    assert!(first_out.contains(ANSI_CLEAR_AND_HOME));
    assert!(first_out.contains("Status: Ready"));
    let (idle_out, idle_stats): (String, TerminalDiffStats) =
        renderer.render_tree(&tree, 40, 14, TerminalRenderMode::InteractiveTui);
    assert!(idle_out.is_empty());
    assert!(idle_stats.get_is_idle());
    assert_eq!(idle_stats.get_changed_cells(), 0);
    assert_eq!(idle_stats.get_output_bytes(), 0);
    tree.set_text(status, "Status: Done!".to_string());
    let (diff_out, diff_stats): (String, TerminalDiffStats) =
        renderer.render_tree(&tree, 40, 14, TerminalRenderMode::InteractiveTui);
    assert!(!diff_stats.get_full_sync());
    assert!(!diff_stats.get_is_idle());
    assert!(diff_stats.get_changed_cells() > 0);
    assert!(diff_stats.get_changed_cells() < first_stats.get_total_cells());
    assert!(diff_stats.get_output_bytes() < first_stats.get_output_bytes());
    assert!(!diff_out.contains(ANSI_CLEAR_AND_HOME));
    let (resize_out, resize_stats): (String, TerminalDiffStats) =
        renderer.render_tree(&tree, 50, 16, TerminalRenderMode::InteractiveTui);
    assert!(resize_stats.get_full_sync());
    assert_eq!(resize_stats.get_total_cells(), 50 * 16);
    assert_eq!(resize_stats.get_changed_cells(), 50 * 16);
    assert!(resize_out.contains(ANSI_CLEAR_AND_HOME));
}

#[test]
fn terminal_renderer_static_plain_text_emits_zero_ansi_and_fallbacks() {
    let (tree, _, _, _, _, _): (SceneTree, NodeId, NodeId, NodeId, NodeId, Rc<Cell<u32>>) =
        build_sample_terminal_scene();
    let mut renderer: TerminalRenderer = TerminalRenderer::new();
    let (plain, stats): (String, TerminalDiffStats) =
        renderer.render_tree(&tree, 40, 14, TerminalRenderMode::StaticPlainText);
    assert!(!plain.contains('\x1b'));
    assert!(!plain.is_empty());
    assert_eq!(stats.get_output_bytes(), plain.len());
    assert!(plain.contains("Status: Ready"));
    assert!(plain.contains("[ Run ]"));
    assert!(plain.contains("*********"));
    assert!(!plain.contains("secret123"));
    assert!(plain.contains("[Image: Chart Preview]"));
    assert!(plain.contains("[Canvas/Viewport]"));
    assert!(plain.contains("Row 0"));
}

#[test]
fn terminal_renderer_keyboard_focus_scroll_activation_and_exit() {
    let (mut tree, _, btn, inp, scroll, clicks): (
        SceneTree,
        NodeId,
        NodeId,
        NodeId,
        NodeId,
        Rc<Cell<u32>>,
    ) = build_sample_terminal_scene();
    let mut renderer: TerminalRenderer = TerminalRenderer::new();
    let tab_ev: KeyEvent = KeyEvent::new(KeyCode::Tab, Modifiers::default(), false);
    assert_eq!(
        renderer.handle_key(&mut tree, tab_ev),
        TerminalNavAction::FocusMoved(btn)
    );
    assert_eq!(tree.get_focus().focused_node(), Some(btn));
    let (focused_plain, _): (String, TerminalDiffStats) =
        renderer.render_tree(&tree, 40, 14, TerminalRenderMode::StaticPlainText);
    assert!(focused_plain.contains("> [ Run ]"));
    let enter_ev: KeyEvent = KeyEvent::new(KeyCode::Enter, Modifiers::default(), false);
    assert_eq!(
        renderer.handle_key(&mut tree, enter_ev),
        TerminalNavAction::Activated(btn)
    );
    assert_eq!(clicks.get(), 1);
    assert_eq!(
        renderer.handle_key(&mut tree, tab_ev),
        TerminalNavAction::FocusMoved(inp)
    );
    let mut shift_mods: Modifiers = Modifiers::default();
    shift_mods.set_shift(true);
    let shift_tab: KeyEvent = KeyEvent::new(KeyCode::Tab, shift_mods, false);
    assert_eq!(
        renderer.handle_key(&mut tree, shift_tab),
        TerminalNavAction::FocusMoved(btn)
    );
    let down_ev: KeyEvent = KeyEvent::new(KeyCode::ArrowDown, Modifiers::default(), false);
    assert_eq!(
        renderer.handle_key(&mut tree, down_ev),
        TerminalNavAction::Scrolled(1)
    );
    let scroll_y: f32 = tree
        .get_node(scroll)
        .map(|n: &SceneNode| n.get_layout_box().get_scroll_offset().get_y())
        .unwrap_or(0.0);
    assert!(scroll_y > 0.0);
    let esc_ev: KeyEvent = KeyEvent::new(KeyCode::Escape, Modifiers::default(), false);
    assert_eq!(
        renderer.handle_key(&mut tree, esc_ev),
        TerminalNavAction::ExitRequested
    );
    assert!(renderer.get_exit_requested());
}

#[test]
fn terminal_session_guard_restores_on_normal_drop_and_panic_unwind() {
    let normal_sink: Rc<RefCell<String>> = Rc::new(RefCell::new(String::new()));
    {
        let guard: TerminalSessionGuard =
            TerminalSessionGuard::enter_captured(true, true, true, Some(Rc::clone(&normal_sink)));
        assert!(!guard.get_restored());
        let entered: String = normal_sink.borrow().clone();
        assert!(entered.contains(ANSI_ENTER_ALT_SCREEN));
        assert!(entered.contains(ANSI_HIDE_CURSOR));
        assert!(entered.contains(ANSI_ENTER_RAW_MODES));
    }
    let normal_log: String = normal_sink.borrow().clone();
    assert!(normal_log.contains(ANSI_RESET_STYLE));
    assert!(normal_log.contains(ANSI_LEAVE_RAW_MODES));
    assert!(normal_log.contains(ANSI_SHOW_CURSOR));
    assert!(normal_log.contains(ANSI_LEAVE_ALT_SCREEN));
    let unwind_sink: Rc<RefCell<String>> = Rc::new(RefCell::new(String::new()));
    let unwind_sink_inner: Rc<RefCell<String>> = Rc::clone(&unwind_sink);
    let res: thread::Result<()> = catch_unwind(AssertUnwindSafe(move || {
        let _guard: TerminalSessionGuard =
            TerminalSessionGuard::enter_captured(true, true, true, Some(unwind_sink_inner));
        panic!("simulated terminal unwind");
    }));
    assert!(res.is_err());
    let unwind_log: String = unwind_sink.borrow().clone();
    assert!(unwind_log.contains(ANSI_RESET_STYLE));
    assert!(unwind_log.contains(ANSI_LEAVE_RAW_MODES));
    assert!(unwind_log.contains(ANSI_SHOW_CURSOR));
    assert!(unwind_log.contains(ANSI_LEAVE_ALT_SCREEN));
}
