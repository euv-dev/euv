use super::*;

/// Fluent section builder for `EuvNavItem`.
impl EuvNavItem {
    /// Assigns a sidebar category group label rendered above the first item in that group.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source navigation item.
    /// - `&str` - Category section title.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated navigation item.
    #[must_use]
    pub fn section(mut self, group: &str) -> Self {
        self.set_section(group.to_string());
        self
    }
}

/// Declarative `#[component]` implementation for `EuvSidebarNav`.
#[component]
/// Inherent implementation of [`EuvSidebarNav`].
impl EuvSidebarNav {
    /// Renders `EuvSidebarNav` into the ambient framework-managed `Context`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component descriptor reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted root scene node handle.
    fn render(&self) -> NodeId {
        let base_tokens: ThemeTokens = self.get_tokens();
        let active_route: Signal<String> = self.get_active_route();
        let Self { items, .. } = self.clone();
        let mut last_section: String = String::new();
        let mut entries: Vec<SidebarNavEntry> = Vec::with_capacity(items.len());
        for (idx, item) in items.into_iter().enumerate() {
            let sec: &str = item.get_section().as_str();
            let sec_header: Option<(String, f32)> =
                if !sec.is_empty() && sec != last_section.as_str() {
                    last_section = sec.to_string();
                    Some((sec.to_string(), if idx == 0 { 2.0 } else { 12.0 }))
                } else {
                    None
                };
            let target_path: String = item.get_path().clone();
            let click_path: String = target_path.clone();
            let indicator_path: String = target_path.clone();
            let label: String = item.get_label().clone();
            entries.push((sec_header, label, target_path, click_path, indicator_path));
        }
        let mut children: Vec<NodeId> = Vec::with_capacity(entries.len() * 2);
        for (sec_header, label, target_path, click_path, indicator_path) in entries.into_iter() {
            if let Some((sec_title, top_pad)) = sec_header {
                let sec_text_style = move || -> Style {
                    let live_p: ThemePalette = use_live_tokens(base_tokens).get_palette();
                    Style::default()
                        .font_size(11.0)
                        .font_weight(FontWeight::SemiBold)
                        .color(live_p.get_muted_foreground())
                };
                children.push(view! {
                    row {
                        width: Length::Percent(100.0)
                        padding: Edges::new(top_pad, SPACE_SM, 4.0, SPACE_SM)
                        text {
                            bind_style: sec_text_style
                            { sec_title }
                        }
                    }
                });
            }
            let row_style = move || -> Style {
                let live_tok: ThemeTokens = use_live_tokens(base_tokens);
                let live_p: ThemePalette = live_tok.get_palette();
                let live_g: ThemeGeometry = live_tok.get_geometry();
                let is_active: bool = active_route.get() == target_path;
                Style::default()
                    .row()
                    .width(Length::Percent(100.0))
                    .height(Length::Px(live_g.get_control_sm().max(34.0)))
                    .align(AlignItems::Center)
                    .gap(SPACE_SM)
                    .padding(Edges::symmetric(0.0, SPACE_SM))
                    .radius(live_g.get_radius_md())
                    .background(if is_active {
                        live_p.get_primary().with_alpha(24)
                    } else {
                        COLOR_TRANSPARENT
                    })
                    .cursor(CursorIcon::Pointer)
            };
            let on_item_click = move |_: &UiEvent| {
                let _: bool = active_route.set_if_changed(click_path.clone());
            };
            let ind_style = move || -> Style {
                let live_p: ThemePalette = use_live_tokens(base_tokens).get_palette();
                let is_active: bool = active_route.get() == indicator_path;
                Style::default()
                    .width(Length::Px(3.5))
                    .height(Length::Px(18.0))
                    .radius(99.0)
                    .background(if is_active {
                        live_p.get_primary()
                    } else {
                        COLOR_TRANSPARENT
                    })
            };
            let lbl_style = move || -> Style {
                let live_p: ThemePalette = use_live_tokens(base_tokens).get_palette();
                Style::default()
                    .font_size(13.5)
                    .font_weight(FontWeight::SemiBold)
                    .color(live_p.get_foreground())
                    .text_overflow(TextOverflow::Clip)
            };
            children.push(view! {
                row {
                    bind_style: row_style
                    on_click: on_item_click
                    view {
                        bind_style: ind_style
                    }
                    text {
                        bind_style: lbl_style
                        { label }
                    }
                }
            });
        }
        view! {
            column {
                width: Length::Percent(100.0)
                gap: 2.0
                ..children
            }
        }
    }
}

/// Declarative `#[component]` implementation for `EuvTabs`.
#[component]
/// Inherent implementation of [`EuvTabs`].
impl EuvTabs {
    /// Renders `EuvTabs` into the ambient framework-managed `Context`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component descriptor reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted root scene node handle.
    fn render(&self) -> NodeId {
        let base_tokens: ThemeTokens = self.get_tokens();
        let active: Signal<usize> = self.get_active();
        let Self { labels, .. } = self.clone();
        let bar_style = move || -> Style {
            let live_tok: ThemeTokens = use_live_tokens(base_tokens);
            let live_p: ThemePalette = live_tok.get_palette();
            let live_g: ThemeGeometry = live_tok.get_geometry();
            Style::default()
                .row()
                .wrap(FlexWrap::Wrap)
                .width(Length::Percent(100.0))
                .gap(SPACE_XS)
                .background(live_p.get_muted())
                .padding(Edges::all(SPACE_XS))
                .radius(live_g.get_radius_md())
        };
        let tab_nodes: Vec<NodeId> = labels
            .into_iter()
            .enumerate()
            .map(|(idx, label): (usize, String)| {
                let tab_style = move || -> Style {
                    let live_tok: ThemeTokens = use_live_tokens(base_tokens);
                    let live_p: ThemePalette = live_tok.get_palette();
                    let live_g: ThemeGeometry = live_tok.get_geometry();
                    let selected: bool = active.get() == idx;
                    let mut st: Style = Style::default()
                        .row()
                        .height(Length::Px(live_g.get_control_sm()))
                        .justify(JustifyContent::Center)
                        .align(AlignItems::Center)
                        .padding(Edges::symmetric(0.0, SPACE_MD))
                        .overflow(Overflow::Hidden)
                        .radius(live_g.get_radius_sm())
                        .background(if selected {
                            live_p.get_card()
                        } else {
                            COLOR_TRANSPARENT
                        })
                        .border(
                            if selected {
                                live_g.get_border_width()
                            } else {
                                0.0
                            },
                            live_p.get_border(),
                        )
                        .cursor(CursorIcon::Pointer);
                    if selected {
                        st = st.shadow(live_g.get_shadow_xs());
                    }
                    st
                };
                let on_tab_click = move |_: &UiEvent| {
                    let _: bool = active.set_if_changed(idx);
                };
                let lbl_style = move || -> Style {
                    let live_p: ThemePalette = use_live_tokens(base_tokens).get_palette();
                    Style::default()
                        .font_size(13.0)
                        .font_weight(FontWeight::Medium)
                        .color(live_p.get_foreground())
                        .text_align(TextAlign::Center)
                        .text_overflow(TextOverflow::Ellipsis)
                };
                view! {
                    row {
                        bind_style: tab_style
                        on_click: on_tab_click
                        text {
                            bind_style: lbl_style
                            { label }
                        }
                    }
                }
            })
            .collect();
        view! {
            row {
                bind_style: bar_style
                ..tab_nodes
            }
        }
    }
}

/// Declarative `#[component]` implementation for `EuvBottomNav`.
#[component]
/// Inherent implementation of [`EuvBottomNav`].
impl EuvBottomNav {
    /// Renders `EuvBottomNav` as a floating dock with disciplined design-token corner radii and a smooth monotonic sliding active-tab indicator tracking clicks, route switches, and live swipe drag progress.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component descriptor reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted bottom navigation bar container handle.
    fn render(&self) -> NodeId {
        let ctx: Context = current_context();
        let base_tokens: ThemeTokens = self.get_tokens();
        let active_route: Signal<String> = self.get_active_route();
        let Self { items, .. } = self.clone();
        let item_count: usize = items.len().max(1);
        let slot_pct: f32 = 100.0 / (item_count as f32);
        let route_paths: Rc<Vec<String>> = Rc::new(
            items
                .iter()
                .map(|item: &EuvNavItem| item.get_path().clone())
                .collect(),
        );
        let initial_idx: Option<usize> = {
            let cur: String = active_route.get_untracked();
            route_paths
                .iter()
                .position(|path: &String| path.as_str() == cur.as_str())
        };
        let drag_sig: Signal<f32> = ctx.get_swipe_drag_offset();
        let vp_sig: Signal<Size> = ctx.get_viewport_size();
        let pill_radius_init: f32 = base_tokens
            .get_geometry()
            .get_radius_md()
            .clamp(NAV_INDICATOR_MIN_RADIUS, NAV_INDICATOR_MAX_RADIUS);
        let pill_style = move || -> Style {
            let live_tok: ThemeTokens = use_live_tokens(base_tokens);
            let live_p: ThemePalette = live_tok.get_palette();
            let live_g: ThemeGeometry = live_tok.get_geometry();
            let pill_radius: f32 = live_g
                .get_radius_md()
                .clamp(NAV_INDICATOR_MIN_RADIUS, NAV_INDICATOR_MAX_RADIUS);
            Style::default()
                .width(Length::Percent(100.0))
                .height(Length::Percent(100.0))
                .radius(pill_radius)
                .background(live_p.get_primary())
        };
        let initial_display: Display = if initial_idx.is_some() {
            Display::Flex
        } else {
            Display::None
        };
        let indicator_node: NodeId = view! {
            row {
                display: initial_display
                position: Position::Absolute
                inset: (Some(6.0), None, None, Some(0.0))
                width: Length::Percent(slot_pct)
                height: Length::Px(46.0)
                padding: Edges::symmetric(0.0, 2.0)
                z_index: 0
                pointer_events: PointerEvents::None
                view {
                    bind_style: pill_style
                }
            }
        };
        let tab_nodes: Vec<NodeId> = items
            .into_iter()
            .enumerate()
            .map(|(tab_idx, item): (usize, EuvNavItem)| {
                let target_path: String = item.get_path().clone();
                let click_path: String = target_path.clone();
                let icon_path: String = target_path;
                let label: String = item.get_label().clone();
                let icon_label: String = label.clone();
                let icon_paths: Rc<Vec<String>> = Rc::clone(&route_paths);
                let label_paths: Rc<Vec<String>> = Rc::clone(&route_paths);
                let on_tab_click = move |_: &UiEvent| {
                    let _: bool = active_route.set_if_changed(click_path.clone());
                };
                let paint_icon = move |bounds: Rect, dl: &mut DisplayList| {
                    let live_p: ThemePalette = live_tokens_untracked(base_tokens).get_palette();
                    let cur: String = active_route.get_untracked();
                    let cur_idx: Option<usize> = icon_paths
                        .iter()
                        .position(|path: &String| path.as_str() == cur.as_str());
                    let dx: f32 = drag_sig.get_untracked();
                    let vp_w: f32 = vp_sig.get_untracked().get_width();
                    let under_pill: bool =
                        bottom_nav_tab_under_pill(tab_idx, cur_idx, item_count, dx, vp_w);
                    let col: Color = if under_pill {
                        live_p.get_primary_foreground()
                    } else {
                        live_p.get_muted_foreground()
                    };
                    draw_bottom_nav_icon(bounds, dl, &icon_path, &icon_label, col);
                };
                let label_style = move || -> Style {
                    let live_p: ThemePalette = use_live_tokens(base_tokens).get_palette();
                    let cur: String = active_route.get();
                    let cur_idx: Option<usize> = label_paths
                        .iter()
                        .position(|path: &String| path.as_str() == cur.as_str());
                    let dx: f32 = drag_sig.get();
                    let vp_w: f32 = vp_sig.get().get_width();
                    let under_pill: bool =
                        bottom_nav_tab_under_pill(tab_idx, cur_idx, item_count, dx, vp_w);
                    Style::default()
                        .font_size(10.5)
                        .font_weight(if under_pill {
                            FontWeight::Bold
                        } else {
                            FontWeight::Medium
                        })
                        .color(if under_pill {
                            live_p.get_primary_foreground()
                        } else {
                            live_p.get_muted_foreground()
                        })
                        .text_align(TextAlign::Center)
                        .text_overflow(TextOverflow::Clip)
                };
                let icon_canvas: NodeId = view! {
                    canvas {
                        width: Length::Px(20.0)
                        height: Length::Px(18.0)
                        painter: paint_icon
                    }
                };
                let ctx_icon: Context = ctx.clone();
                let (icon_scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
                    Effect::new(move || {
                        let _: String = active_route.get();
                        let _: f32 = drag_sig.get();
                        let _: ThemeTokens = use_live_tokens(base_tokens);
                        ctx_icon.get_scene().with_write(
                            |tree: &mut SceneTree| {
                                tree.mark_dirty(icon_canvas, DIRTY_PAINT);
                            },
                            (),
                        );
                    })
                });
                ctx.get_scene().with_write(
                    |tree: &mut SceneTree| {
                        tree.attach_scope(icon_canvas, icon_scope);
                    },
                    (),
                );
                view! {
                    column {
                        flex_grow: 1.0
                        flex_shrink: 1.0
                        min_width: Length::Px(0.0)
                        height: Length::Px(46.0)
                        justify: JustifyContent::Center
                        align: AlignItems::Center
                        gap: 2.0
                        padding: Edges::symmetric(3.0, 6.0)
                        radius: pill_radius_init
                        overflow: Overflow::Hidden
                        background: COLOR_TRANSPARENT
                        z_index: 1
                        cursor: CursorIcon::Pointer
                        on_click: on_tab_click
                        { icon_canvas }
                        text {
                            bind_style: label_style
                            { label }
                        }
                    }
                }
            })
            .collect();
        let bar_node: NodeId = view! {
            row {
                width: Length::Percent(100.0)
                height: Length::Percent(100.0)
                justify: JustifyContent::SpaceBetween
                align: AlignItems::Center
                gap: 0.0
                overflow: Overflow::Hidden
                { indicator_node }
                ..tab_nodes
            }
        };
        let last_idx: Rc<Cell<Option<usize>>> = Rc::new(Cell::new(initial_idx));
        let last_drag_tx: Rc<Cell<f32>> = Rc::new(Cell::new(0.0));
        let route_paths_anim: Rc<Vec<String>> = Rc::clone(&route_paths);
        let ctx_anim: Context = ctx.clone();
        let last_idx_route: Rc<Cell<Option<usize>>> = Rc::clone(&last_idx);
        let last_drag_route: Rc<Cell<f32>> = Rc::clone(&last_drag_tx);
        let (route_scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            Effect::new(move || {
                let current_path: String = active_route.get();
                let vp_w: f32 = ctx_anim.get_viewport_size().get().get_width();
                let next_idx: Option<usize> = route_paths_anim
                    .iter()
                    .position(|path: &String| path.as_str() == current_path.as_str());
                let prev_idx: Option<usize> = last_idx_route.replace(next_idx);
                let prev_drag: f32 = last_drag_route.replace(0.0);
                let dirty_layout: u16 = DIRTY_LAYOUT | DIRTY_SUBTREE_LAYOUT;
                ctx_anim.get_scene().with_write(
                    |tree: &mut SceneTree| {
                        let bar_w: f32 = tree
                            .get_node(bar_node)
                            .map(|n: &SceneNode| n.get_layout_box().get_bounds().get_width())
                            .filter(|w: &f32| *w > 1.0)
                            .unwrap_or_else(|| (vp_w - 48.0).clamp(240.0, 460.0));
                        let bar_h: f32 = tree
                            .get_node(bar_node)
                            .map(|n: &SceneNode| {
                                n.get_layout_box().get_content_bounds().get_height()
                            })
                            .filter(|h: &f32| *h > 1.0)
                            .unwrap_or(58.0);
                        let top_px: f32 = ((bar_h - 46.0) * 0.5).max(0.0);
                        let slot_w: f32 = bar_w / (item_count as f32);
                        if let Some(target_i) = next_idx {
                            let target_left_px: f32 = (target_i as f32) * slot_w;
                            let cur_visual_tx: f32 = tree
                                .get_node(indicator_node)
                                .map(|n: &SceneNode| {
                                    n.get_style().get_paint().get_transform().get_translate_x()
                                })
                                .unwrap_or(prev_drag);
                            let from_x: f32 = if let Some(old_i) = prev_idx
                                && old_i != target_i
                            {
                                let delta_slots: f32 = (old_i as f32) - (target_i as f32);
                                delta_slots * slot_w + cur_visual_tx
                            } else {
                                cur_visual_tx
                            };
                            if let Some(node) = tree.get_node_mut(indicator_node) {
                                let mut st: Style = node.get_style();
                                st.get_mut_layout()
                                    .set_display(Display::Flex)
                                    .set_top(Some(top_px))
                                    .set_bottom(None)
                                    .set_left(Some(target_left_px))
                                    .set_right(None)
                                    .set_width(Length::Percent(slot_pct));
                                if from_x.abs() > 0.5 {
                                    st.get_mut_paint().set_transform(Transform2D::new(
                                        from_x, 0.0, 1.0, 1.0, 0.0,
                                    ));
                                }
                                node.set_style(st);
                            }
                            tree.mark_dirty(indicator_node, dirty_layout | DIRTY_PAINT);
                            if let Some(old_i) = prev_idx
                                && old_i != target_i
                            {
                                tree.start_transition(
                                    NodeTransition::slide_fade(
                                        indicator_node,
                                        Point::new(from_x, 0.0),
                                        Point::new(0.0, 0.0),
                                        1.0,
                                        1.0,
                                        NAV_INDICATOR_SLIDE_SECS,
                                        false,
                                    )
                                    .with_easing(Easing::EaseOutCubic),
                                );
                            } else if from_x.abs() > 0.5 {
                                tree.start_transition(
                                    NodeTransition::slide_fade(
                                        indicator_node,
                                        Point::new(from_x, 0.0),
                                        Point::new(0.0, 0.0),
                                        1.0,
                                        1.0,
                                        NAV_INDICATOR_CANCEL_SECS,
                                        false,
                                    )
                                    .with_easing(Easing::EaseOutCubic),
                                );
                            }
                        } else {
                            if let Some(node) = tree.get_node_mut(indicator_node) {
                                let mut st: Style = node.get_style();
                                st.get_mut_layout().set_display(Display::None);
                                st.get_mut_paint().set_transform(Transform2D::default());
                                node.set_style(st);
                            }
                            tree.mark_dirty(indicator_node, dirty_layout | DIRTY_PAINT);
                        }
                    },
                    (),
                );
            })
        });
        let ctx_drag: Context = ctx.clone();
        let last_idx_drag: Rc<Cell<Option<usize>>> = Rc::clone(&last_idx);
        let last_drag_live: Rc<Cell<f32>> = Rc::clone(&last_drag_tx);
        let (drag_scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            Effect::new(move || {
                let dx: f32 = drag_sig.get();
                let Some(cur_i) = last_idx_drag.get() else {
                    return;
                };
                let vp_w: f32 = ctx_drag
                    .get_viewport_size()
                    .get_untracked()
                    .get_width()
                    .max(1.0);
                if dx.abs() > 0.5 {
                    let mut frac: f32 = (-dx / vp_w).clamp(-1.0, 1.0);
                    if cur_i == 0 && frac < 0.0 {
                        frac = 0.0;
                    }
                    if cur_i + 1 >= item_count && frac > 0.0 {
                        frac = 0.0;
                    }
                    ctx_drag.get_scene().with_write(
                        |tree: &mut SceneTree| {
                            let bar_w: f32 = tree
                                .get_node(bar_node)
                                .map(|n: &SceneNode| n.get_layout_box().get_bounds().get_width())
                                .filter(|w: &f32| *w > 1.0)
                                .unwrap_or_else(|| (vp_w - 48.0).clamp(240.0, 460.0));
                            let slot_w: f32 = bar_w / (item_count as f32);
                            let tx: f32 = frac * slot_w;
                            last_drag_live.set(tx);
                            if let Some(node) = tree.get_node_mut(indicator_node) {
                                let mut st: Style = node.get_style();
                                st.get_mut_paint()
                                    .set_transform(Transform2D::new(tx, 0.0, 1.0, 1.0, 0.0));
                                node.set_style(st);
                            }
                            tree.mark_dirty(indicator_node, DIRTY_PAINT);
                        },
                        (),
                    );
                } else {
                    let prev_tx: f32 = last_drag_live.replace(0.0);
                    if prev_tx.abs() > 0.5 {
                        ctx_drag.get_scene().with_write(
                            |tree: &mut SceneTree| {
                                tree.start_transition(
                                    NodeTransition::slide_fade(
                                        indicator_node,
                                        Point::new(prev_tx, 0.0),
                                        Point::new(0.0, 0.0),
                                        1.0,
                                        1.0,
                                        NAV_INDICATOR_CANCEL_SECS,
                                        false,
                                    )
                                    .with_easing(Easing::EaseOutCubic),
                                );
                            },
                            (),
                        );
                    }
                }
            })
        });
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                tree.attach_scope(bar_node, route_scope);
                tree.attach_scope(bar_node, drag_scope);
            },
            (),
        );
        bar_node
    }
}
