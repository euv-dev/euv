use super::*;

/// Fluent builder helpers for `EuvMermaid`.
impl EuvMermaid {
    /// Sets the maximum inline scroll viewport height in logical pixels.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source Mermaid component.
    /// - `f32` - Maximum viewport height in logical pixels.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated Mermaid component.
    #[must_use]
    pub fn max_height(mut self, height: f32) -> Self {
        self.set_max_height(height);
        self
    }

    /// Registers a callback invoked when the user clicks Copy on the Mermaid diagram header.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source Mermaid component.
    /// - `F` - Callback receiving the raw Mermaid source text.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated Mermaid component.
    #[must_use]
    pub fn on_copy<F>(mut self, handler: F) -> Self
    where
        F: Fn(&str) + 'static,
    {
        self.set_on_copy(Some(Rc::new(handler)));
        self
    }
}

/// Declarative `#[component]` implementation for `EuvMermaid`.
#[component]
/// Inherent implementation of [`EuvMermaid`].
impl EuvMermaid {
    /// Renders a native vector Mermaid diagram with scrollable viewport, Copy button, and interactive Fullscreen modal.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mermaid component descriptor reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted Mermaid diagram card handle.
    fn render(&self) -> NodeId {
        let ctx: Context = current_context();
        bind_fullscreen_context(&ctx);
        let tokens: ThemeTokens = self.get_tokens();
        let g: ThemeGeometry = tokens.get_geometry();
        let eva: CodeThemePalette = CodeThemePalette::resolve(tokens.get_mode());
        let Self {
            source,
            max_height,
            on_copy,
            ..
        } = self;
        let diagram: Rc<MermaidDiagram> = Rc::new(parse_mermaid_diagram(source));
        let nat_sz: Size = measure_mermaid_diagram(&diagram);
        let badge: &'static str = mermaid_badge_label(&diagram);
        let title_raw: String = mermaid_diagram_title(&diagram);
        let header_title_rc: Option<Rc<str>> = if title_raw.is_empty() {
            None
        } else {
            Some(Rc::from(title_raw))
        };
        let header_title_str: &str = header_title_rc.as_deref().unwrap_or(badge);
        let inline_max_h: f32 = if *max_height > 0.0 {
            *max_height
        } else {
            MERMAID_DEFAULT_MAX_HEIGHT
        };
        let copied_sig: Signal<bool> = Signal::new(false);
        let fs_open: Signal<bool> = Signal::new(false);
        let source_rc: Rc<str> = Rc::from(source.as_str());
        let copy_src: Rc<str> = Rc::clone(&source_rc);
        let copy_cb: Option<MarkdownStrCallback> = on_copy.clone();
        let ctx_copy: Context = ctx.clone();
        let on_copy_click = move |_: &UiEvent| {
            let _: bool = ctx_copy.copy_to_clipboard(&copy_src);
            let _: bool = copied_sig.set(true);
            if let Some(cb) = copy_cb.as_ref() {
                cb(&copy_src);
            }
        };
        let paint_inline_copy_icon = move |bounds: Rect, dl: &mut DisplayList| {
            let cur_tok: ThemeTokens = live_tokens_untracked(tokens);
            let cur_eva: CodeThemePalette = CodeThemePalette::resolve(cur_tok.get_mode());
            if copied_sig.get_untracked() {
                draw_euv_icon(
                    EuvIconName::Check,
                    bounds,
                    dl,
                    cur_tok.get_palette().get_success(),
                );
            } else {
                draw_euv_icon(EuvIconName::Copy, bounds, dl, cur_eva.get_function());
            }
        };
        let on_open_fs = move |_: &UiEvent| {
            let _: bool = fs_open.set_if_changed(true);
        };
        let diag_inline: Rc<MermaidDiagram> = Rc::clone(&diagram);
        let inline_cache: RefCell<Option<(Rect, ThemeTokens, Vec<DrawCommand>)>> =
            RefCell::new(None);
        let paint_inline = move |bounds: Rect, dl: &mut DisplayList| {
            let live_tok: ThemeTokens = live_tokens_untracked(tokens);
            if let Ok(guard) = inline_cache.try_borrow()
                && let Some((cached_bounds, cached_tok, cached_cmds)) = guard.as_ref()
                && *cached_bounds == bounds
                && *cached_tok == live_tok
            {
                dl.get_mut_commands()
                    .extend_from_slice(cached_cmds.as_slice());
                return;
            }
            let mut scratch: DisplayList = DisplayList::default();
            draw_mermaid_diagram(&diag_inline, bounds, &mut scratch, live_tok);
            let cmds: Vec<DrawCommand> = take(scratch.get_mut_commands());
            dl.get_mut_commands().extend_from_slice(cmds.as_slice());
            if let Ok(mut guard) = inline_cache.try_borrow_mut() {
                *guard = Some((bounds, live_tok, cmds));
            }
        };
        let paint_fs_icon = move |bounds: Rect, dl: &mut DisplayList| {
            let live_eva: CodeThemePalette =
                CodeThemePalette::resolve(live_tokens_untracked(tokens).get_mode());
            draw_code_fullscreen_icon(bounds, dl, live_eva.get_function());
        };
        let diag_fs: Rc<MermaidDiagram> = Rc::clone(&diagram);
        let fs_src_copy: Rc<str> = Rc::clone(&source_rc);
        let fs_cb_copy: Option<MarkdownStrCallback> = on_copy.clone();
        let fs_title_rc: Option<Rc<str>> = header_title_rc.clone();
        let ctx_fs_copy: Context = ctx.clone();
        let build_fs_children = move || -> Vec<NodeId> {
            if !fs_open.get() {
                return Vec::new();
            }
            let live_tok: ThemeTokens = live_tokens_untracked(tokens);
            let live_p: ThemePalette = live_tok.get_palette();
            let live_g: ThemeGeometry = live_tok.get_geometry();
            let live_eva: CodeThemePalette = CodeThemePalette::resolve(live_tok.get_mode());
            let diag_modal: Rc<MermaidDiagram> = Rc::clone(&diag_fs);
            let modal_cache: RefCell<Option<(Rect, ThemeTokens, Vec<DrawCommand>)>> =
                RefCell::new(None);
            let paint_modal = move |bounds: Rect, dl: &mut DisplayList| {
                let cur_tok: ThemeTokens = live_tokens_untracked(tokens);
                if let Ok(guard) = modal_cache.try_borrow()
                    && let Some((cached_bounds, cached_tok, cached_cmds)) = guard.as_ref()
                    && *cached_bounds == bounds
                    && *cached_tok == cur_tok
                {
                    dl.get_mut_commands()
                        .extend_from_slice(cached_cmds.as_slice());
                    return;
                }
                let mut scratch: DisplayList = DisplayList::default();
                draw_mermaid_diagram(&diag_modal, bounds, &mut scratch, cur_tok);
                let cmds: Vec<DrawCommand> = take(scratch.get_mut_commands());
                dl.get_mut_commands().extend_from_slice(cmds.as_slice());
                if let Ok(mut guard) = modal_cache.try_borrow_mut() {
                    *guard = Some((bounds, cur_tok, cmds));
                }
            };
            let modal_src: Rc<str> = Rc::clone(&fs_src_copy);
            let modal_cb: Option<MarkdownStrCallback> = fs_cb_copy.clone();
            let ctx_m: Context = ctx_fs_copy.clone();
            let modal_copy_icon_cell: Rc<Cell<NodeId>> = Rc::new(Cell::new(NodeId::default()));
            let modal_copy_icon_cb: Rc<Cell<NodeId>> = Rc::clone(&modal_copy_icon_cell);
            let modal_copy_btn_cell: Rc<Cell<NodeId>> = Rc::new(Cell::new(NodeId::default()));
            let modal_copy_btn_cb: Rc<Cell<NodeId>> = Rc::clone(&modal_copy_btn_cell);
            let on_modal_copy = move |_: &UiEvent| {
                let _: bool = ctx_m.copy_to_clipboard(&modal_src);
                let _: bool = copied_sig.set(true);
                let m_icon: NodeId = modal_copy_icon_cb.get();
                let m_btn: NodeId = modal_copy_btn_cb.get();
                ctx_m.get_scene().with_write(
                    |tree: &mut SceneTree| {
                        if let Some(n) = tree.get_node_mut(m_btn) {
                            *n.get_mut_a11y() = Some(A11yProps::new(
                                A11yRole::Button,
                                MERMAID_BTN_COPIED.to_string(),
                            ));
                        }
                        tree.mark_dirty(m_icon, DIRTY_PAINT);
                    },
                    (),
                );
                if let Some(cb) = modal_cb.as_ref() {
                    cb(&modal_src);
                }
            };
            let paint_modal_copy_icon = move |bounds: Rect, dl: &mut DisplayList| {
                let cur_tok: ThemeTokens = live_tokens_untracked(tokens);
                let cur_eva: CodeThemePalette = CodeThemePalette::resolve(cur_tok.get_mode());
                if copied_sig.get_untracked() {
                    draw_euv_icon(
                        EuvIconName::Check,
                        bounds,
                        dl,
                        cur_tok.get_palette().get_success(),
                    );
                } else {
                    draw_euv_icon(EuvIconName::Copy, bounds, dl, cur_eva.get_function());
                }
            };
            let paint_close_icon = move |bounds: Rect, dl: &mut DisplayList| {
                let cur_p: ThemePalette = live_tokens_untracked(tokens).get_palette();
                draw_euv_icon(
                    EuvIconName::Minimize,
                    bounds,
                    dl,
                    cur_p.get_primary_foreground(),
                );
            };
            let on_close_fs = move |_: &UiEvent| {
                let _: bool = fs_open.set_if_changed(false);
            };
            let init_copy_a11y: String = if copied_sig.get_untracked() {
                MERMAID_BTN_COPIED.to_string()
            } else {
                MERMAID_BTN_COPY.to_string()
            };
            let modal_title_str: &str = fs_title_rc.as_deref().unwrap_or(badge);
            let safe_sig: Signal<SafeAreaInsets> = ctx_fs_copy.get_safe_area();
            let init_safe: SafeAreaInsets = safe_sig.get_untracked();
            let mut modal_scroll_st: Style = Style::default();
            modal_scroll_st
                .get_mut_layout()
                .set_flex_direction(FlexDirection::Column);
            let mut top_bar_node: NodeId = NodeId::default();
            let mut body_scroll_node: NodeId = NodeId::default();
            let mut modal_copy_btn: NodeId = NodeId::default();
            let mut modal_copy_icon: NodeId = NodeId::default();
            let modal_panel: NodeId = view! {
                column {
                    width: Length::Percent(100.0)
                    height: Length::Percent(100.0)
                    flex_grow: 1.0
                    padding: Edges::all(0.0)
                    margin: Edges::all(0.0)
                    background: live_eva.get_background()
                    border: (0.0, COLOR_TRANSPARENT)
                    radius: 0.0
                    overflow: Overflow::Hidden
                    stop_event_propagation: true
                    row {
                        node_ref: top_bar_node
                        width: Length::Percent(100.0)
                        flex_shrink: 0.0
                        padding: Edges::new(
                            SPACE_SM + init_safe.get_top(),
                            SPACE_MD + init_safe.get_right(),
                            SPACE_SM,
                            SPACE_MD + init_safe.get_left()
                        )
                        justify: JustifyContent::SpaceBetween
                        align: AlignItems::Center
                        gap: SPACE_SM
                        background: live_eva.get_header_bg()
                        row {
                            flex_grow: 1.0
                            flex_shrink: 1.0
                            min_width: Length::Px(0.0)
                            align: AlignItems::Center
                            gap: SPACE_SM
                            overflow: Overflow::Hidden
                            row {
                                padding: Edges::symmetric(3.0, 8.0)
                                radius: live_g.get_radius_sm()
                                background: live_eva.get_background()
                                border: (1.0, live_eva.get_border())
                                text {
                                    font_size: 10.5
                                    font_weight: FontWeight::Bold
                                    color: live_eva.get_keyword()
                                    text_overflow: TextOverflow::Clip
                                    { badge }
                                }
                            }
                            text {
                                flex_shrink: 1.0
                                min_width: Length::Px(0.0)
                                overflow: Overflow::Hidden
                                text_overflow: TextOverflow::Ellipsis
                                font_size: 13.0
                                font_weight: FontWeight::SemiBold
                                color: live_p.get_foreground()
                                { modal_title_str }
                            }
                        }
                        row {
                            flex_shrink: 0.0
                            align: AlignItems::Center
                            gap: SPACE_XS
                            row {
                                node_ref: modal_copy_btn
                                width: Length::Px(28.0)
                                height: Length::Px(28.0)
                                justify: JustifyContent::Center
                                align: AlignItems::Center
                                radius: live_g.get_radius_sm()
                                cursor: CursorIcon::Pointer
                                background: live_eva.get_background()
                                border: (1.0, live_eva.get_border())
                                role: A11yRole::Button
                                a11y_label: init_copy_a11y
                                on_click: on_modal_copy
                                canvas {
                                    node_ref: modal_copy_icon
                                    width: Length::Px(14.0)
                                    height: Length::Px(14.0)
                                    painter: paint_modal_copy_icon
                                }
                            }
                            row {
                                width: Length::Px(28.0)
                                height: Length::Px(28.0)
                                justify: JustifyContent::Center
                                align: AlignItems::Center
                                radius: live_g.get_radius_sm()
                                cursor: CursorIcon::Pointer
                                background: live_p.get_primary()
                                role: A11yRole::Button
                                a11y_label: MERMAID_BTN_CLOSE.to_string()
                                on_click: on_close_fs
                                canvas {
                                    width: Length::Px(14.0)
                                    height: Length::Px(14.0)
                                    painter: paint_close_icon
                                }
                            }
                        }
                    }
                    view {
                        width: Length::Percent(100.0)
                        height: Length::Px(1.0)
                        background: live_eva.get_border()
                    }
                    scroll {
                        node_ref: body_scroll_node
                        style: modal_scroll_st
                        flex_grow: 1.0
                        flex_shrink: 1.0
                        min_height: Length::Px(0.0)
                        width: Length::Percent(100.0)
                        align: AlignItems::Start
                        padding: Edges::new(
                            SPACE_MD,
                            SPACE_MD + init_safe.get_right(),
                            SPACE_MD + init_safe.get_bottom(),
                            SPACE_MD + init_safe.get_left()
                        )
                        stop_scroll_propagation: true
                        canvas {
                            width: Length::Px(nat_sz.get_width().max(640.0))
                            height: Length::Px(nat_sz.get_height().max(320.0))
                            painter: paint_modal
                        }
                    }
                }
            };
            let handle_safe: SceneHandle = ctx_fs_copy.get_scene().clone();
            let (safe_scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
                Effect::new(move || {
                    let safe: SafeAreaInsets = safe_sig.get();
                    handle_safe.with_write(
                        |tree: &mut SceneTree| {
                            let dirty_l: u16 = DIRTY_LAYOUT | DIRTY_SUBTREE_LAYOUT;
                            if let Some(n) = tree.get_node_mut(top_bar_node) {
                                n.get_mut_style().get_mut_layout().set_padding(Edges::new(
                                    SPACE_SM + safe.get_top(),
                                    SPACE_MD + safe.get_right(),
                                    SPACE_SM,
                                    SPACE_MD + safe.get_left(),
                                ));
                                tree.mark_dirty(top_bar_node, dirty_l | DIRTY_PAINT);
                            }
                            if let Some(n) = tree.get_node_mut(body_scroll_node) {
                                n.get_mut_style().get_mut_layout().set_padding(Edges::new(
                                    SPACE_MD,
                                    SPACE_MD + safe.get_right(),
                                    SPACE_MD + safe.get_bottom(),
                                    SPACE_MD + safe.get_left(),
                                ));
                                tree.mark_dirty(body_scroll_node, dirty_l | DIRTY_PAINT);
                            }
                        },
                        (),
                    );
                })
            });
            ctx_fs_copy.get_scene().with_write(
                |tree: &mut SceneTree| {
                    tree.attach_scope(modal_panel, safe_scope);
                },
                (),
            );
            modal_copy_btn_cell.set(modal_copy_btn);
            modal_copy_icon_cell.set(modal_copy_icon);
            vec![modal_panel]
        };
        let mut fs_portal: NodeId = NodeId::default();
        let mut inline_copy_btn: NodeId = NodeId::default();
        let mut inline_copy_icon: NodeId = NodeId::default();
        let mut inline_scroll_st: Style = Style::default();
        inline_scroll_st
            .get_mut_layout()
            .set_flex_direction(FlexDirection::Column);
        let root: NodeId = view! {
            column {
                width: Length::Percent(100.0)
                background: eva.get_background()
                border: (1.0, eva.get_border())
                radius: g.get_radius_lg()
                overflow: Overflow::Hidden
                row {
                    width: Length::Percent(100.0)
                    padding: Edges::symmetric(SPACE_SM, SPACE_MD)
                    justify: JustifyContent::SpaceBetween
                    align: AlignItems::Center
                    gap: SPACE_SM
                    overflow: Overflow::Hidden
                    background: eva.get_header_bg()
                    row {
                        flex_grow: 1.0
                        flex_shrink: 1.0
                        min_width: Length::Px(0.0)
                        align: AlignItems::Center
                        gap: SPACE_SM
                        overflow: Overflow::Hidden
                        row {
                            flex_shrink: 0.0
                            padding: Edges::symmetric(3.0, 8.0)
                            radius: g.get_radius_sm()
                            background: eva.get_background()
                            border: (1.0, eva.get_border())
                            text {
                                font_size: 10.5
                                font_weight: FontWeight::Bold
                                color: eva.get_keyword()
                                text_overflow: TextOverflow::Clip
                                { badge }
                            }
                        }
                        text {
                            flex_shrink: 1.0
                            min_width: Length::Px(0.0)
                            overflow: Overflow::Hidden
                            text_overflow: TextOverflow::Ellipsis
                            font_size: 12.0
                            font_weight: FontWeight::SemiBold
                            color: eva.get_foreground()
                            { header_title_str }
                        }
                    }
                    row {
                        flex_shrink: 0.0
                        align: AlignItems::Center
                        gap: SPACE_XS
                        row {
                            width: Length::Px(26.0)
                            height: Length::Px(26.0)
                            radius: g.get_radius_sm()
                            justify: JustifyContent::Center
                            align: AlignItems::Center
                            cursor: CursorIcon::Pointer
                            background: eva.get_background()
                            border: (1.0, eva.get_border())
                            role: A11yRole::Button
                            a11y_label: MERMAID_BTN_FULLSCREEN.to_string()
                            on_click: on_open_fs
                            canvas {
                                width: Length::Px(13.0)
                                height: Length::Px(13.0)
                                painter: paint_fs_icon
                            }
                        }
                        row {
                            node_ref: inline_copy_btn
                            width: Length::Px(26.0)
                            height: Length::Px(26.0)
                            radius: g.get_radius_sm()
                            justify: JustifyContent::Center
                            align: AlignItems::Center
                            cursor: CursorIcon::Pointer
                            background: eva.get_background()
                            border: (1.0, eva.get_border())
                            role: A11yRole::Button
                            a11y_label: MERMAID_BTN_COPY.to_string()
                            on_click: on_copy_click
                            canvas {
                                node_ref: inline_copy_icon
                                width: Length::Px(13.0)
                                height: Length::Px(13.0)
                                painter: paint_inline_copy_icon
                            }
                        }
                    }
                }
                view {
                    width: Length::Percent(100.0)
                    height: Length::Px(1.0)
                    background: eva.get_border()
                }
                scroll {
                    style: inline_scroll_st
                    width: Length::Percent(100.0)
                    max_height: Length::Px(inline_max_h)
                    align: AlignItems::Start
                    stop_scroll_propagation: true
                    canvas {
                        width: Length::Px(nat_sz.get_width())
                        height: Length::Px(nat_sz.get_height())
                        painter: paint_inline
                    }
                }
                portal {
                    node_ref: fs_portal
                    style: Style::default().column()
                    display: Display::None
                    position: Position::Fixed
                    inset: (Some(0.0), Some(0.0), Some(0.0), Some(0.0))
                    width: Length::Percent(100.0)
                    height: Length::Percent(100.0)
                    padding: Edges::all(0.0)
                    margin: Edges::all(0.0)
                    justify: JustifyContent::Start
                    align: AlignItems::Stretch
                    background: eva.get_background()
                    border: (0.0, COLOR_TRANSPARENT)
                    radius: 0.0
                    overflow: Overflow::Hidden
                    pointer_events: PointerEvents::None
                    stop_scroll_propagation: true
                    stop_event_propagation: true
                    z_index: Z_INDEX_FULLSCREEN
                    bind_children: build_fs_children
                }
            }
        };
        let handle: SceneHandle = ctx.get_scene().clone();
        let handle_copy: SceneHandle = handle.clone();
        let prev_open: Rc<Cell<bool>> = Rc::new(Cell::new(false));
        let fs_acquired: Rc<Cell<bool>> = Rc::new(Cell::new(false));
        let fs_cleanup: Rc<Cell<bool>> = Rc::clone(&fs_acquired);
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            on_cleanup(move || {
                if fs_cleanup.replace(false) {
                    exit_fullscreen();
                }
            });
            let _: Effect = Effect::new(move || {
                let is_copied: bool = copied_sig.get();
                let next_label: &str = if is_copied {
                    MERMAID_BTN_COPIED
                } else {
                    MERMAID_BTN_COPY
                };
                handle_copy.with_write(
                    |tree: &mut SceneTree| {
                        if let Some(n) = tree.get_node_mut(inline_copy_btn) {
                            *n.get_mut_a11y() =
                                Some(A11yProps::new(A11yRole::Button, next_label.to_string()));
                        }
                        tree.mark_dirty(inline_copy_icon, DIRTY_PAINT);
                    },
                    (),
                );
            });
            let _: Effect = EuvOverlayStack::bind_signal(
                fs_open,
                MERMAID_FULLSCREEN_OVERLAY_NAME,
                Z_INDEX_FULLSCREEN,
            );
            Effect::new(move || {
                let is_open: bool = fs_open.get();
                if is_open == prev_open.replace(is_open) {
                    return;
                }
                if is_open {
                    if !fs_acquired.replace(true) {
                        enter_fullscreen();
                    }
                    let live_eva: CodeThemePalette =
                        CodeThemePalette::resolve(live_tokens_untracked(tokens).get_mode());
                    handle.with_write(
                        |tree: &mut SceneTree| {
                            tree.cancel_subtree_transitions(fs_portal);
                            if let Some(n) = tree.get_node_mut(fs_portal) {
                                let st: &mut Style = n.get_mut_style();
                                st.get_mut_layout().set_display(Display::Flex);
                                st.get_mut_paint()
                                    .set_background(Some(live_eva.get_background()))
                                    .set_pointer_events(PointerEvents::Auto)
                                    .set_opacity(1.0)
                                    .set_transform(Transform2D::default());
                            }
                            let dirty_l: u16 = DIRTY_LAYOUT | DIRTY_SUBTREE_LAYOUT;
                            tree.mark_dirty(fs_portal, dirty_l | DIRTY_PAINT);
                        },
                        (),
                    );
                } else {
                    if fs_acquired.replace(false) {
                        exit_fullscreen();
                    }
                    handle.with_write(
                        |tree: &mut SceneTree| {
                            tree.cancel_subtree_transitions(fs_portal);
                            if let Some(n) = tree.get_node_mut(fs_portal) {
                                let st: &mut Style = n.get_mut_style();
                                st.get_mut_layout().set_display(Display::None);
                                st.get_mut_paint()
                                    .set_pointer_events(PointerEvents::None)
                                    .set_opacity(1.0)
                                    .set_transform(Transform2D::default());
                            }
                            let dirty_l: u16 = DIRTY_LAYOUT | DIRTY_SUBTREE_LAYOUT;
                            tree.mark_dirty(fs_portal, dirty_l | DIRTY_PAINT);
                        },
                        (),
                    );
                }
            })
        });
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                tree.attach_scope(root, scope);
            },
            (),
        );
        root
    }
}
