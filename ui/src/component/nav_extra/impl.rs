use super::*;

/// Fluent interactive selection builder for `EuvBreadcrumb`.
impl EuvBreadcrumb {
    /// Binds a reactive signal capturing the selected breadcrumb segment index when clicked.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source breadcrumb.
    /// - `Signal<usize>` - Active segment index signal.
    ///
    /// # Returns
    ///
    /// - `Self` - Interactive breadcrumb trail.
    #[must_use]
    pub fn active(mut self, active: Signal<usize>) -> Self {
        self.set_active(Some(active));
        self
    }
}

/// Declarative `#[component]` implementation for `EuvBreadcrumb`.
#[component]
/// Inherent implementation of [`EuvBreadcrumb`].
impl EuvBreadcrumb {
    /// Renders `EuvBreadcrumb` into the ambient framework-managed `Context`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component descriptor reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted root scene node handle.
    fn render(&self) -> NodeId {
        let mut ctx: Context = current_context();
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let Self {
            segments, active, ..
        } = self;
        let active: Option<Signal<usize>> = *active;
        let last_idx: usize = segments.len().saturating_sub(1);
        let fg_active: Color = p.get_foreground();
        let fg_muted: Color = p.get_muted_foreground();
        let mut seg_texts: Vec<NodeId> = Vec::with_capacity(segments.len());
        let mut row_items: Vec<NodeId> = Vec::with_capacity(segments.len() * 2);
        let init_active: usize = active
            .map(|s: Signal<usize>| s.get_untracked())
            .unwrap_or(0);
        for (idx, seg) in segments.iter().enumerate() {
            if let Some(active_sig) = active {
                let is_cur: bool = init_active == idx;
                let mut txt_id: NodeId = NodeId::default();
                let on_pick = move |_: &UiEvent| {
                    let _: bool = active_sig.set_if_changed(idx);
                };
                let item: NodeId = view! {
                    row {
                        padding: Edges::symmetric(2.0, 6.0)
                        radius: g.get_radius_sm()
                        cursor: CursorIcon::Pointer
                        on_click: on_pick
                        text {
                            node_ref: txt_id
                            font_size: 13.0
                            font_weight: if is_cur { FontWeight::SemiBold } else { FontWeight::Normal }
                            color: if is_cur { fg_active } else { fg_muted }
                            { seg.as_str() }
                        }
                    }
                };
                seg_texts.push(txt_id);
                row_items.push(item);
            } else {
                row_items.push(view! {
                    text {
                        font_size: 13.0
                        color: if idx == last_idx { fg_active } else { fg_muted }
                        { seg.as_str() }
                    }
                });
            }
            if idx < last_idx {
                row_items.push(view! {
                    text {
                        font_size: 12.0
                        color: fg_muted
                        { BREADCRUMB_SEP }
                    }
                });
            }
        }
        let root: NodeId = view! {
            row {
                flex_wrap: FlexWrap::Wrap
                width: Length::Percent(100.0)
                align: AlignItems::Center
                gap: SPACE_XS
            }
        };
        if !row_items.is_empty() {
            ctx.append_many(root, row_items.as_slice());
        }
        if let Some(active_sig) = active {
            let handle: SceneHandle = ctx.get_scene().clone();
            let prev_idx: Cell<usize> = Cell::new(init_active);
            let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
                Effect::new(move || {
                    let next_idx: usize = active_sig.get();
                    let old_idx: usize = prev_idx.replace(next_idx);
                    if old_idx == next_idx {
                        return;
                    }
                    handle.with_write(
                        |tree: &mut SceneTree| {
                            let dirty_l: u16 = DIRTY_LAYOUT | DIRTY_SUBTREE_LAYOUT;
                            let dirty_t: u16 = DIRTY_TEXT | DIRTY_PAINT;
                            if let Some(&old_txt) = seg_texts.get(old_idx)
                                && let Some(n) = tree.get_node_mut(old_txt)
                            {
                                n.get_mut_style()
                                    .get_mut_text()
                                    .set_weight(FontWeight::Normal)
                                    .set_color(fg_muted);
                                tree.mark_dirty(old_txt, dirty_l | dirty_t);
                            }
                            if let Some(&new_txt) = seg_texts.get(next_idx)
                                && let Some(n) = tree.get_node_mut(new_txt)
                            {
                                n.get_mut_style()
                                    .get_mut_text()
                                    .set_weight(FontWeight::SemiBold)
                                    .set_color(fg_active);
                                tree.mark_dirty(new_txt, dirty_l | dirty_t);
                            }
                        },
                        (),
                    );
                })
            });
            ctx.get_scene().with_write(
                |tree: &mut SceneTree| {
                    tree.attach_scope(root, scope);
                },
                (),
            );
        }
        root
    }
}

/// Declarative `#[component]` implementation for `EuvPagination`.
#[component]
/// Inherent implementation of [`EuvPagination`].
impl EuvPagination {
    /// Renders `EuvPagination` into the ambient framework-managed `Context`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component descriptor reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted root scene node handle.
    fn render(&self) -> NodeId {
        let tokens: ThemeTokens = self.get_tokens();
        let p: ThemePalette = tokens.get_palette();
        let current: Signal<usize> = self.get_current();
        let total: usize = self.get_total_pages();
        view! {
            row {
                flex_wrap: FlexWrap::Wrap
                align: AlignItems::Center
                gap: SPACE_SM
                EuvButton {
                    label: PAGE_PREV_LABEL.to_string()
                    tokens: tokens
                    outline()
                    sm()
                    on_click: move |_: &UiEvent| {
                        let _: bool = current.update(|v: &mut usize| *v = v.saturating_sub(1).max(1));
                    }
                }
                text {
                    font_size: 13.0
                    font_weight: FontWeight::Medium
                    color: p.get_foreground()
                    { move || format!("{} / {total}", current.get()) }
                }
                EuvButton {
                    label: PAGE_NEXT_LABEL.to_string()
                    tokens: tokens
                    outline()
                    sm()
                    on_click: move |_: &UiEvent| {
                        let _: bool = current.update(|v: &mut usize| *v = (*v + 1).min(total));
                    }
                }
            }
        }
    }
}

/// Declarative `#[component]` implementation for `EuvSteps`.
#[component]
/// Inherent implementation of [`EuvSteps`].
impl EuvSteps {
    /// Renders `EuvSteps` into the ambient framework-managed `Context`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component descriptor reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted root scene node handle.
    fn render(&self) -> NodeId {
        let mut ctx: Context = current_context();
        let p: ThemePalette = self.get_tokens().get_palette();
        let active: Signal<usize> = self.get_active();
        let Self { labels, .. } = self;
        let init_active: usize = active.get_untracked();
        let done_bg: Color = p.get_primary();
        let todo_bg: Color = p.get_secondary();
        let done_fg: Color = p.get_primary_foreground();
        let todo_fg: Color = p.get_muted_foreground();
        let border_col: Color = p.get_border();
        let mut step_nodes: Vec<(NodeId, NodeId)> = Vec::with_capacity(labels.len());
        let mut row_items: Vec<NodeId> = Vec::with_capacity(labels.len());
        for (idx, lbl) in labels.iter().enumerate() {
            let done: bool = init_active >= idx;
            let mut circle_id: NodeId = NodeId::default();
            let mut num_id: NodeId = NodeId::default();
            let on_pick = move |_: &UiEvent| {
                let _: bool = active.set_if_changed(idx);
            };
            let item: NodeId = view! {
                row {
                    align: AlignItems::Center
                    gap: SPACE_SM
                    cursor: CursorIcon::Pointer
                    on_click: on_pick
                    row {
                        node_ref: circle_id
                        width: Length::Px(24.0)
                        height: Length::Px(24.0)
                        justify: JustifyContent::Center
                        align: AlignItems::Center
                        radius: 12.0
                        border: (if done { 0.0 } else { 1.0 }, border_col)
                        background: if done { done_bg } else { todo_bg }
                        text {
                            node_ref: num_id
                            font_size: 12.0
                            font_weight: FontWeight::Bold
                            color: if done { done_fg } else { todo_fg }
                            { (idx + 1).to_string() }
                        }
                    }
                    text {
                        font_size: 13.0
                        font_weight: FontWeight::Medium
                        color: p.get_foreground()
                        { lbl.as_str() }
                    }
                }
            };
            step_nodes.push((circle_id, num_id));
            row_items.push(item);
        }
        let root: NodeId = view! {
            row {
                flex_wrap: FlexWrap::Wrap
                width: Length::Percent(100.0)
                gap: SPACE_MD
            }
        };
        if !row_items.is_empty() {
            ctx.append_many(root, row_items.as_slice());
        }
        let handle: SceneHandle = ctx.get_scene().clone();
        let prev_active: Cell<usize> = Cell::new(init_active);
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            Effect::new(move || {
                let next_active: usize = active.get();
                let old_active: usize = prev_active.replace(next_active);
                if old_active == next_active {
                    return;
                }
                let lo: usize = old_active.min(next_active);
                let hi: usize = old_active.max(next_active).min(step_nodes.len());
                handle.with_write(
                    |tree: &mut SceneTree| {
                        let dirty_l: u16 = DIRTY_LAYOUT | DIRTY_SUBTREE_LAYOUT;
                        for (idx, &(circle_id, num_id)) in step_nodes
                            .iter()
                            .enumerate()
                            .skip(lo)
                            .take(hi.saturating_sub(lo).saturating_add(1))
                        {
                            let done: bool = next_active >= idx;
                            if let Some(n) = tree.get_node_mut(circle_id) {
                                n.get_mut_style()
                                    .get_mut_paint()
                                    .set_border(Border::new(
                                        if done { 0.0 } else { 1.0 },
                                        border_col,
                                    ))
                                    .set_background(Some(if done { done_bg } else { todo_bg }));
                                tree.mark_dirty(circle_id, dirty_l | DIRTY_PAINT);
                            }
                            if let Some(n) = tree.get_node_mut(num_id) {
                                n.get_mut_style().get_mut_text().set_color(if done {
                                    done_fg
                                } else {
                                    todo_fg
                                });
                                tree.mark_dirty(num_id, DIRTY_PAINT);
                            }
                        }
                    },
                    (),
                );
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

/// Declarative `#[component]` implementation for `EuvAccordion`.
#[component]
/// Inherent implementation of [`EuvAccordion`].
impl EuvAccordion {
    /// Renders `EuvAccordion` into the ambient framework-managed `Context`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component descriptor reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted root scene node handle.
    fn render(&self) -> NodeId {
        let mut ctx: Context = current_context();
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let open_index: Signal<Option<usize>> = self.get_open_index();
        let Self { items, .. } = self;
        let init_open: Option<usize> = open_index.get_untracked();
        let mut bodies: Vec<NodeId> = Vec::with_capacity(items.len());
        let mut cards: Vec<NodeId> = Vec::with_capacity(items.len());
        for (idx, item) in items.iter().enumerate() {
            let is_open: bool = init_open == Some(idx);
            let mut body_id: NodeId = NodeId::default();
            let on_toggle = move |_: &UiEvent| {
                let _: bool = open_index.update(|cur: &mut Option<usize>| {
                    *cur = if *cur == Some(idx) { None } else { Some(idx) };
                });
            };
            let card: NodeId = view! {
                column {
                    width: Length::Percent(100.0)
                    background: p.get_card()
                    border: (g.get_border_width(), p.get_border())
                    radius: g.get_radius_md()
                    overflow: Overflow::Hidden
                    row {
                        width: Length::Percent(100.0)
                        padding: Edges::symmetric(SPACE_SM, SPACE_MD)
                        cursor: CursorIcon::Pointer
                        on_click: on_toggle
                        text {
                            font_size: 14.0
                            font_weight: FontWeight::SemiBold
                            color: p.get_foreground()
                            { item.get_title().as_str() }
                        }
                    }
                    column {
                        node_ref: body_id
                        display: if is_open { Display::Flex } else { Display::None }
                        width: Length::Percent(100.0)
                        padding: Edges::symmetric(SPACE_SM, SPACE_MD)
                        text {
                            font_size: 13.0
                            color: p.get_muted_foreground()
                            { item.get_content().as_str() }
                        }
                    }
                }
            };
            bodies.push(body_id);
            cards.push(card);
        }
        let root: NodeId = view! {
            column {
                width: Length::Percent(100.0)
                gap: SPACE_XS
            }
        };
        if !cards.is_empty() {
            ctx.append_many(root, cards.as_slice());
        }
        let handle: SceneHandle = ctx.get_scene().clone();
        let prev_open: Cell<Option<usize>> = Cell::new(init_open);
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            Effect::new(move || {
                let next_open: Option<usize> = open_index.get();
                let old_open: Option<usize> = prev_open.replace(next_open);
                if old_open == next_open {
                    return;
                }
                handle.with_write(
                    |tree: &mut SceneTree| {
                        let dirty_l: u16 = DIRTY_LAYOUT | DIRTY_SUBTREE_LAYOUT;
                        if let Some(old_idx) = old_open
                            && let Some(&old_body) = bodies.get(old_idx)
                            && let Some(n) = tree.get_node_mut(old_body)
                        {
                            n.get_mut_style()
                                .get_mut_layout()
                                .set_display(Display::None);
                            tree.mark_dirty(old_body, dirty_l | DIRTY_PAINT);
                        }
                        if let Some(new_idx) = next_open
                            && let Some(&new_body) = bodies.get(new_idx)
                            && let Some(n) = tree.get_node_mut(new_body)
                        {
                            n.get_mut_style()
                                .get_mut_layout()
                                .set_display(Display::Flex);
                            tree.mark_dirty(new_body, dirty_l | DIRTY_PAINT);
                        }
                    },
                    (),
                );
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

/// Declarative `#[component]` implementation for `EuvToc`.
#[component]
/// Inherent implementation of [`EuvToc`].
impl EuvToc {
    /// Renders `EuvToc` into the ambient framework-managed `Context`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component descriptor reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted root scene node handle.
    fn render(&self) -> NodeId {
        let mut ctx: Context = current_context();
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let active: Signal<usize> = self.get_active();
        let Self { headings, .. } = self;
        let init_active: usize = active.get_untracked();
        let accent_bg: Color = p.get_accent();
        let mut row_nodes: Vec<NodeId> = Vec::with_capacity(headings.len());
        for (idx, h) in headings.iter().enumerate() {
            let is_cur: bool = init_active == idx;
            let on_pick = move |_: &UiEvent| {
                let _: bool = active.set_if_changed(idx);
            };
            row_nodes.push(view! {
                row {
                    width: Length::Percent(100.0)
                    padding: Edges::symmetric(SPACE_XS, SPACE_SM)
                    radius: g.get_radius_sm()
                    background: if is_cur { accent_bg } else { COLOR_TRANSPARENT }
                    cursor: CursorIcon::Pointer
                    on_click: on_pick
                    text {
                        font_size: 13.0
                        color: p.get_foreground()
                        { h.as_str() }
                    }
                }
            });
        }
        let root: NodeId = view! {
            column {
                width: Length::Percent(100.0)
                padding: Edges::all(SPACE_MD)
                gap: SPACE_XS
                background: p.get_muted()
                border: (g.get_border_width(), p.get_border())
                radius: g.get_radius_md()
                overflow: Overflow::Hidden
                text {
                    font_size: 12.0
                    font_weight: FontWeight::Bold
                    color: p.get_muted_foreground()
                    { TOC_HEADING_DEFAULT }
                }
            }
        };
        if !row_nodes.is_empty() {
            ctx.append_many(root, row_nodes.as_slice());
        }
        let handle: SceneHandle = ctx.get_scene().clone();
        let prev_active: Cell<usize> = Cell::new(init_active);
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            Effect::new(move || {
                let next_active: usize = active.get();
                let old_active: usize = prev_active.replace(next_active);
                if old_active == next_active {
                    return;
                }
                handle.with_write(
                    |tree: &mut SceneTree| {
                        if let Some(&old_row) = row_nodes.get(old_active)
                            && let Some(n) = tree.get_node_mut(old_row)
                        {
                            n.get_mut_style()
                                .get_mut_paint()
                                .set_background(Some(COLOR_TRANSPARENT));
                            tree.mark_dirty(old_row, DIRTY_PAINT);
                        }
                        if let Some(&new_row) = row_nodes.get(next_active)
                            && let Some(n) = tree.get_node_mut(new_row)
                        {
                            n.get_mut_style()
                                .get_mut_paint()
                                .set_background(Some(accent_bg));
                            tree.mark_dirty(new_row, DIRTY_PAINT);
                        }
                    },
                    (),
                );
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
