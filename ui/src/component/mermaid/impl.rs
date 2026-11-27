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
        let copy_label_text = move || -> String {
            if copied_sig.get() {
                MERMAID_BTN_COPIED.to_string()
            } else {
                MERMAID_BTN_COPY.to_string()
            }
        };
        let on_open_fs = move |_: &UiEvent| {
            let _: bool = fs_open.set_if_changed(true);
        };
        let diag_inline: Rc<MermaidDiagram> = Rc::clone(&diagram);
        let paint_inline = move |bounds: Rect, dl: &mut DisplayList| {
            let live_tok: ThemeTokens = live_tokens_untracked(tokens);
            draw_mermaid_diagram(&diag_inline, bounds, dl, live_tok);
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
            let paint_modal = move |bounds: Rect, dl: &mut DisplayList| {
                let cur_tok: ThemeTokens = live_tokens_untracked(tokens);
                draw_mermaid_diagram(&diag_modal, bounds, dl, cur_tok);
            };
            let modal_src: Rc<str> = Rc::clone(&fs_src_copy);
            let modal_cb: Option<MarkdownStrCallback> = fs_cb_copy.clone();
            let ctx_m: Context = ctx_fs_copy.clone();
            let on_modal_copy = move |_: &UiEvent| {
                let _: bool = ctx_m.copy_to_clipboard(&modal_src);
                let _: bool = copied_sig.set(true);
                if let Some(cb) = modal_cb.as_ref() {
                    cb(&modal_src);
                }
            };
            let modal_copy_text = move || -> String {
                if copied_sig.get() {
                    MERMAID_BTN_COPIED.to_string()
                } else {
                    MERMAID_BTN_COPY.to_string()
                }
            };
            let on_close_fs = move |_: &UiEvent| {
                let _: bool = fs_open.set_if_changed(false);
            };
            let modal_title_str: &str = fs_title_rc.as_deref().unwrap_or(badge);
            let mut modal_scroll_st: Style = Style::default();
            modal_scroll_st
                .get_mut_layout()
                .set_flex_direction(FlexDirection::Column);
            let modal_panel: NodeId = view! {
                column {
                    width: Length::Percent(94.0)
                    max_width: Length::Px(1180.0)
                    height: Length::Percent(88.0)
                    background: live_eva.get_background()
                    border: (1.0, live_eva.get_border())
                    radius: live_g.get_radius_lg()
                    shadow: live_g.get_shadow_lg()
                    overflow: Overflow::Hidden
                    stop_event_propagation: true
                    row {
                        width: Length::Percent(100.0)
                        padding: Edges::symmetric(SPACE_SM, SPACE_MD)
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
                            gap: SPACE_SM
                            row {
                                padding: Edges::symmetric(4.0, 10.0)
                                radius: live_g.get_radius_sm()
                                cursor: CursorIcon::Pointer
                                background: live_eva.get_background()
                                border: (1.0, live_eva.get_border())
                                on_click: on_modal_copy
                                text {
                                    font_size: 11.5
                                    font_weight: FontWeight::SemiBold
                                    color: live_eva.get_function()
                                    text_overflow: TextOverflow::Clip
                                    { modal_copy_text }
                                }
                            }
                            row {
                                padding: Edges::symmetric(4.0, 10.0)
                                radius: live_g.get_radius_sm()
                                cursor: CursorIcon::Pointer
                                background: live_p.get_primary()
                                on_click: on_close_fs
                                text {
                                    font_size: 11.5
                                    font_weight: FontWeight::SemiBold
                                    color: live_p.get_primary_foreground()
                                    text_overflow: TextOverflow::Clip
                                    { MERMAID_BTN_CLOSE }
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
                        style: modal_scroll_st
                        flex_grow: 1.0
                        width: Length::Percent(100.0)
                        height: Length::Percent(100.0)
                        align: AlignItems::Start
                        padding: Edges::all(SPACE_MD)
                        stop_scroll_propagation: true
                        canvas {
                            width: Length::Px(nat_sz.get_width().max(640.0))
                            height: Length::Px(nat_sz.get_height().max(320.0))
                            painter: paint_modal
                        }
                    }
                }
            };
            vec![modal_panel]
        };
        let mut fs_portal: NodeId = NodeId::default();
        let on_scrim_click = move |ev: &UiEvent| {
            if ev.get_target() == fs_portal {
                let _: bool = fs_open.set_if_changed(false);
            }
        };
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
                            padding: Edges::symmetric(3.0, 9.0)
                            radius: g.get_radius_sm()
                            justify: JustifyContent::Center
                            align: AlignItems::Center
                            gap: 5.0
                            cursor: CursorIcon::Pointer
                            background: eva.get_background()
                            border: (1.0, eva.get_border())
                            on_click: on_open_fs
                            canvas {
                                width: Length::Px(12.0)
                                height: Length::Px(12.0)
                                painter: paint_fs_icon
                            }
                            text {
                                font_size: 11.0
                                font_weight: FontWeight::SemiBold
                                color: eva.get_function()
                                text_overflow: TextOverflow::Clip
                                { MERMAID_BTN_FULLSCREEN }
                            }
                        }
                        row {
                            padding: Edges::symmetric(3.0, 10.0)
                            radius: g.get_radius_sm()
                            justify: JustifyContent::Center
                            align: AlignItems::Center
                            cursor: CursorIcon::Pointer
                            background: eva.get_background()
                            border: (1.0, eva.get_border())
                            on_click: on_copy_click
                            text {
                                font_size: 11.0
                                font_weight: FontWeight::SemiBold
                                color: eva.get_function()
                                text_overflow: TextOverflow::Clip
                                { copy_label_text }
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
                    canvas {
                        width: Length::Px(nat_sz.get_width())
                        height: Length::Px(nat_sz.get_height())
                        painter: paint_inline
                    }
                }
                portal {
                    node_ref: fs_portal
                    display: Display::None
                    position: Position::Fixed
                    inset: (Some(0.0), Some(0.0), Some(0.0), Some(0.0))
                    width: Length::Percent(100.0)
                    height: Length::Percent(100.0)
                    justify: JustifyContent::Center
                    align: AlignItems::Center
                    background: tokens.get_palette().get_overlay()
                    stop_scroll_propagation: true
                    z_index: Z_INDEX_MODAL
                    on_click: on_scrim_click
                    bind_children: build_fs_children
                }
            }
        };
        let handle: SceneHandle = ctx.get_scene().clone();
        let prev_open: Rc<Cell<bool>> = Rc::new(Cell::new(false));
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            let _: Effect = EuvOverlayStack::bind_signal(
                fs_open,
                MERMAID_FULLSCREEN_OVERLAY_NAME,
                Z_INDEX_MODAL,
            );
            Effect::new(move || {
                let is_open: bool = fs_open.get();
                if is_open == prev_open.replace(is_open) {
                    return;
                }
                if is_open {
                    handle.start_transition(
                        NodeTransition::fade(fs_portal, 0.0, 1.0, 0.18, false)
                            .with_easing(Easing::EaseOutCubic),
                    );
                } else {
                    handle.start_transition(
                        NodeTransition::fade(fs_portal, 1.0, 0.0, 0.16, true)
                            .with_easing(Easing::EaseOutCubic),
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
