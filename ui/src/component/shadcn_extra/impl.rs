use super::*;

/// Fluent configuration builders for `EuvAlertDialog`.
impl EuvAlertDialog {
    /// Sets the explanatory body description fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source alert dialog.
    /// - `S` - Body description string.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated alert dialog.
    #[must_use]
    pub fn description<S>(mut self, description: S) -> Self
    where
        S: Into<String>,
    {
        self.set_description(description.into());
        self
    }

    /// Sets custom cancel and confirm button labels fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source alert dialog.
    /// - `S1` - Cancel action label.
    /// - `S2` - Confirm action label.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated alert dialog.
    #[must_use]
    pub fn labels<S1, S2>(mut self, cancel_label: S1, confirm_label: S2) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        self.set_cancel_label(cancel_label.into());
        self.set_confirm_label(confirm_label.into());
        self
    }
}

/// Declarative `#[component]` implementation for `EuvAlertDialog`.
#[component]
/// Inherent implementation of [`EuvAlertDialog`].
impl EuvAlertDialog {
    /// Renders `EuvAlertDialog` into the ambient framework-managed `Context`.
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
        let tokens: ThemeTokens = self.get_tokens();
        let p: ThemePalette = tokens.get_palette();
        let g: ThemeGeometry = tokens.get_geometry();
        let open: Signal<bool> = self.get_open();
        let confirmed: Signal<bool> = self.get_confirmed();
        let Self {
            title,
            description,
            cancel_label,
            confirm_label,
            ..
        } = self;
        let title_owned: String = title.clone();
        let resolved_cancel: String = if cancel_label.is_empty() {
            ALERT_DEFAULT_CANCEL_LABEL.to_string()
        } else {
            cancel_label.clone()
        };
        let resolved_confirm: String = if confirm_label.is_empty() {
            ALERT_DEFAULT_CONFIRM_LABEL.to_string()
        } else {
            confirm_label.clone()
        };
        let init_open: bool = open.get_untracked();
        let mut card: NodeId = NodeId::default();
        let on_cancel = move |_: &UiEvent| {
            let _: bool = confirmed.set_if_changed(false);
            let _: bool = open.set_if_changed(false);
        };
        let on_confirm = move |_: &UiEvent| {
            let _: bool = confirmed.set_if_changed(true);
            let _: bool = open.set_if_changed(false);
        };
        let scrim: NodeId = view! {
            portal {
                display: if init_open { Display::Flex } else { Display::None }
                position: Position::Fixed
                inset: (Some(0.0), Some(0.0), Some(0.0), Some(0.0))
                width: Length::Percent(100.0)
                height: Length::Percent(100.0)
                justify: JustifyContent::Center
                align: AlignItems::Center
                background: p.get_overlay()
                stop_scroll_propagation: true
                z_index: Z_INDEX_MODAL
                column {
                    node_ref: card
                    width: Length::Percent(92.0)
                    max_width: Length::Px(440.0)
                    padding: Edges::all(SPACE_XL)
                    gap: SPACE_MD
                    background: p.get_card()
                    border: (g.get_border_width(), p.get_border())
                    radius: g.get_radius_lg()
                    overflow: Overflow::Hidden
                    shadow: g.get_shadow_lg()
                    role: A11yRole::Dialog
                    a11y_label: title_owned.clone()
                    text {
                        width: Length::Percent(100.0)
                        font_size: g.get_font_heading()
                        font_weight: FontWeight::SemiBold
                        color: p.get_card_foreground()
                        { title_owned }
                    }
                    text {
                        width: Length::Percent(100.0)
                        font_size: 13.0
                        color: p.get_muted_foreground()
                        { description.as_str() }
                    }
                    row {
                        flex_wrap: FlexWrap::Wrap
                        width: Length::Percent(100.0)
                        justify: JustifyContent::End
                        gap: SPACE_SM
                        EuvButton {
                            label: resolved_cancel
                            tokens: tokens
                            outline()
                            on_click: on_cancel
                        }
                        EuvButton {
                            label: resolved_confirm
                            tokens: tokens
                            destructive()
                            on_click: on_confirm
                        }
                    }
                }
            }
        };
        ctx.on_click(scrim, move |ev: &UiEvent| {
            if ev.get_target() == scrim {
                let _: bool = confirmed.set_if_changed(false);
                let _: bool = open.set_if_changed(false);
            }
        });
        let handle: SceneHandle = ctx.get_scene().clone();
        let prev_open: Rc<Cell<bool>> = Rc::new(Cell::new(init_open));
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            let _: Effect =
                EuvOverlayStack::bind_signal(open, ALERT_DIALOG_OVERLAY_NAME, Z_INDEX_MODAL);
            Effect::new(move || {
                let is_open: bool = open.get();
                let was_open: bool = prev_open.replace(is_open);
                if is_open == was_open {
                    return;
                }
                if is_open {
                    handle.with_write(
                        |tree: &mut SceneTree| {
                            tree.cancel_subtree_transitions(scrim);
                            tree.cancel_subtree_transitions(card);
                            if let Some(n) = tree.get_node_mut(scrim) {
                                let st: &mut Style = n.get_mut_style();
                                st.get_mut_layout().set_display(Display::Flex);
                                st.get_mut_paint()
                                    .set_pointer_events(PointerEvents::Auto)
                                    .set_transform(Transform2D::default());
                            }
                            let dirty_l: u16 = DIRTY_LAYOUT | DIRTY_SUBTREE_LAYOUT;
                            tree.mark_dirty(scrim, dirty_l | DIRTY_PAINT);
                            tree.start_transition(NodeTransition::fade(
                                scrim, 0.0, 1.0, 0.20, false,
                            ));
                            tree.start_transition(NodeTransition::modal_enter(card));
                        },
                        (),
                    );
                } else {
                    handle.with_write(
                        |tree: &mut SceneTree| {
                            tree.cancel_subtree_transitions(scrim);
                            tree.cancel_subtree_transitions(card);
                            if let Some(n) = tree.get_node_mut(scrim) {
                                let st: &mut Style = n.get_mut_style();
                                st.get_mut_layout().set_display(Display::None);
                                st.get_mut_paint()
                                    .set_pointer_events(PointerEvents::None)
                                    .set_opacity(1.0)
                                    .set_transform(Transform2D::default());
                            }
                            if let Some(n) = tree.get_node_mut(card) {
                                n.get_mut_style()
                                    .get_mut_paint()
                                    .set_opacity(1.0)
                                    .set_transform(Transform2D::default());
                            }
                            let dirty_l: u16 = DIRTY_LAYOUT | DIRTY_SUBTREE_LAYOUT;
                            tree.mark_dirty(scrim, dirty_l | DIRTY_PAINT);
                        },
                        (),
                    );
                }
            })
        });
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                tree.attach_scope(scrim, scope);
            },
            (),
        );
        scrim
    }
}

/// Declarative `#[component]` implementation for `EuvCollapsible`.
#[component]
/// Inherent implementation of [`EuvCollapsible`].
impl EuvCollapsible {
    /// Renders `EuvCollapsible` into the ambient framework-managed `Context`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component descriptor reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted root scene node handle.
    fn render(&self) -> NodeId {
        let ctx: Context = current_context();
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let open: Signal<bool> = self.get_open();
        let content: NodeId = self.get_content();
        let Self { title, .. } = self;
        let init_open: bool = open.get_untracked();
        let init_disp: Display = if init_open {
            Display::Flex
        } else {
            Display::None
        };
        let toggle_click = move |_: &UiEvent| {
            let _: bool = open.update(|v: &mut bool| *v = !*v);
        };
        let toggle_label = move || -> String {
            if open.get() {
                COLLAPSIBLE_HIDE_LABEL.to_string()
            } else {
                COLLAPSIBLE_SHOW_LABEL.to_string()
            }
        };
        let div_h: f32 = g.get_border_width().max(1.0);
        let mut divider: NodeId = NodeId::default();
        let mut body: NodeId = NodeId::default();
        let root: NodeId = view! {
            column {
                width: Length::Percent(100.0)
                background: p.get_card()
                border: (g.get_border_width(), p.get_border())
                radius: g.get_radius_md()
                overflow: Overflow::Hidden
                row {
                    width: Length::Percent(100.0)
                    padding: Edges::symmetric(SPACE_SM, SPACE_MD)
                    justify: JustifyContent::SpaceBetween
                    align: AlignItems::Center
                    cursor: CursorIcon::Pointer
                    on_click: toggle_click
                    text {
                        font_size: 13.0
                        font_weight: FontWeight::SemiBold
                        color: p.get_foreground()
                        { title.as_str() }
                    }
                    text {
                        font_size: 12.0
                        font_weight: FontWeight::Medium
                        color: p.get_muted_foreground()
                        { toggle_label }
                    }
                }
                view {
                    node_ref: divider
                    display: init_disp
                    width: Length::Percent(100.0)
                    height: Length::Px(div_h)
                    background: p.get_border()
                }
                column {
                    node_ref: body
                    display: init_disp
                    width: Length::Percent(100.0)
                    padding: Edges::all(SPACE_MD)
                    { content }
                }
            }
        };
        let handle: SceneHandle = ctx.get_scene().clone();
        let prev_open: Cell<bool> = Cell::new(init_open);
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            Effect::new(move || {
                let is_open: bool = open.get();
                if prev_open.replace(is_open) == is_open {
                    return;
                }
                let next_disp: Display = if is_open {
                    Display::Flex
                } else {
                    Display::None
                };
                handle.with_write(
                    |tree: &mut SceneTree| {
                        let dirty_l: u16 = DIRTY_LAYOUT | DIRTY_SUBTREE_LAYOUT;
                        if let Some(n) = tree.get_node_mut(divider) {
                            n.get_mut_style().get_mut_layout().set_display(next_disp);
                            tree.mark_dirty(divider, dirty_l | DIRTY_PAINT);
                        }
                        if let Some(n) = tree.get_node_mut(body) {
                            n.get_mut_style().get_mut_layout().set_display(next_disp);
                            tree.mark_dirty(body, dirty_l | DIRTY_PAINT);
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

/// Declarative `#[component]` implementation for `EuvToggleGroup`.
#[component]
/// Inherent implementation of [`EuvToggleGroup`].
impl EuvToggleGroup {
    /// Renders `EuvToggleGroup` into the ambient framework-managed `Context`.
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
        let selected: Signal<usize> = self.get_selected();
        let Self { options, .. } = self;
        let init_sel: usize = selected.get_untracked();
        let border_w: f32 = g.get_border_width();
        let on_border: Border = Border::new(border_w, p.get_primary());
        let off_border: Border = Border::new(border_w, p.get_border());
        let on_bg: Color = p.get_accent();
        let off_bg: Color = p.get_background();
        let item_nodes: Vec<NodeId> = options
            .iter()
            .enumerate()
            .map(|(idx, opt): (usize, &String)| {
                let on: bool = init_sel == idx;
                let on_pick = move |_: &UiEvent| {
                    let _: bool = selected.set_if_changed(idx);
                };
                view! {
                    row {
                        height: Length::Px(g.get_control_sm())
                        justify: JustifyContent::Center
                        align: AlignItems::Center
                        padding: Edges::symmetric(0.0, SPACE_MD)
                        overflow: Overflow::Hidden
                        radius: g.get_radius_md()
                        border: (border_w, if on { p.get_primary() } else { p.get_border() })
                        background: if on { on_bg } else { off_bg }
                        cursor: CursorIcon::Pointer
                        on_click: on_pick
                        text {
                            font_size: 13.0
                            font_weight: FontWeight::Medium
                            color: p.get_foreground()
                            text_align: TextAlign::Center
                            text_overflow: TextOverflow::Ellipsis
                            { opt.as_str() }
                        }
                    }
                }
            })
            .collect();
        let root: NodeId = view! {
            row {
                flex_wrap: FlexWrap::Wrap
                width: Length::Percent(100.0)
                gap: SPACE_XS
            }
        };
        if !item_nodes.is_empty() {
            ctx.append_many(root, item_nodes.as_slice());
        }
        let handle: SceneHandle = ctx.get_scene().clone();
        let prev_sel: Cell<usize> = Cell::new(init_sel);
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            Effect::new(move || {
                let next_sel: usize = selected.get();
                let old_sel: usize = prev_sel.replace(next_sel);
                if old_sel == next_sel {
                    return;
                }
                handle.with_write(
                    |tree: &mut SceneTree| {
                        if let Some(&old_id) = item_nodes.get(old_sel)
                            && let Some(n) = tree.get_node_mut(old_id)
                        {
                            n.get_mut_style()
                                .get_mut_paint()
                                .set_border(off_border)
                                .set_background(Some(off_bg));
                            tree.mark_dirty(old_id, DIRTY_PAINT);
                        }
                        if let Some(&new_id) = item_nodes.get(next_sel)
                            && let Some(n) = tree.get_node_mut(new_id)
                        {
                            n.get_mut_style()
                                .get_mut_paint()
                                .set_border(on_border)
                                .set_background(Some(on_bg));
                            tree.mark_dirty(new_id, DIRTY_PAINT);
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

/// Declarative `#[component]` implementation for `EuvInputOtp`.
#[component]
/// Inherent implementation of [`EuvInputOtp`].
impl EuvInputOtp {
    /// Renders `EuvInputOtp` into the ambient framework-managed `Context`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component descriptor reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted root scene node handle.
    fn render(&self) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let value: Signal<String> = self.get_value();
        let slots: usize = self.get_length().clamp(4, 8);
        view! {
            row {
                flex_wrap: FlexWrap::Wrap
                align: AlignItems::Center
                gap: SPACE_XS
                for slot_idx in 0..slots {
                    row {
                        width: Length::Px(36.0)
                        height: Length::Px(40.0)
                        justify: JustifyContent::Center
                        align: AlignItems::Center
                        background: p.get_background()
                        border: (g.get_border_width(), p.get_border())
                        radius: g.get_radius_md()
                        cursor: CursorIcon::Pointer
                        on_click: move |_: &UiEvent| {
                            let _: bool = value.update(|s: &mut String| {
                                let mut chars: Vec<char> = s.chars().collect();
                                while chars.len() <= slot_idx {
                                    chars.push('0');
                                }
                                let next_digit: u32 = chars[slot_idx]
                                .to_digit(10)
                                .map(|d: u32| (d + 1) % 10)
                                .unwrap_or(1);
                                chars[slot_idx] = char::from_digit(next_digit, 10).unwrap_or('0');
                                chars.truncate(slots);
                                *s = chars.into_iter().collect();
                            });
                        }
                        text {
                            font_family: FontFamily::Monospace
                            font_size: 15.0
                            font_weight: FontWeight::SemiBold
                            color: p.get_foreground()
                            text_align: TextAlign::Center
                            { move || {
                                let cur: String = value.get();
                                cur.chars()
                                .nth(slot_idx)
                                .map(|c: char| c.to_string())
                                .unwrap_or_else(|| OTP_EMPTY_SLOT.to_string())
                            } }
                        }
                    }
                }
            }
        }
    }
}

/// Fluent search query and placeholder builders for `EuvCommand`.
impl EuvCommand {
    /// Binds an external reactive query signal fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source command palette.
    /// - `Signal<String>` - Controlled query signal.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated command palette.
    #[must_use]
    pub fn query(mut self, query: Signal<String>) -> Self {
        self.set_query(Some(query));
        self
    }

    /// Sets the search input placeholder text fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source command palette.
    /// - `S` - Placeholder prompt string.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated command palette.
    #[must_use]
    pub fn placeholder<S>(mut self, placeholder: S) -> Self
    where
        S: Into<String>,
    {
        self.set_placeholder(placeholder.into());
        self
    }
}

/// Declarative `#[component]` implementation for `EuvCommand`.
#[component]
/// Inherent implementation of [`EuvCommand`].
impl EuvCommand {
    /// Renders `EuvCommand` into the ambient framework-managed `Context`.
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
        let g: ThemeGeometry = tokens.get_geometry();
        let selected: Signal<Option<usize>> = self.get_selected();
        let Self {
            query: query_opt,
            placeholder,
            items,
            ..
        } = self;
        let query: Signal<String> = query_opt.unwrap_or_else(|| Signal::new(String::new()));
        let resolved_ph: String = if placeholder.is_empty() {
            COMMAND_DEFAULT_PLACEHOLDER.to_string()
        } else {
            placeholder.clone()
        };
        let items_shared: Rc<[String]> = Rc::from(items.as_slice());
        let build_cmd_items = move || -> Vec<NodeId> {
            let live_tok: ThemeTokens = use_live_tokens(tokens);
            let live_p: ThemePalette = live_tok.get_palette();
            let live_g: ThemeGeometry = live_tok.get_geometry();
            let raw_q: String = query.get();
            let q_ascii: bool = raw_q.is_ascii();
            let needle_unicode: String = if q_ascii || raw_q.is_empty() {
                String::new()
            } else {
                raw_q.to_lowercase()
            };
            let cur_sel: Option<usize> = selected.get();
            let mut nodes: Vec<NodeId> = Vec::with_capacity(items_shared.len().max(1));
            for (idx, item) in items_shared.iter().enumerate() {
                if !raw_q.is_empty() {
                    let matched: bool = if q_ascii {
                        contains_ascii_ignore_case(item.as_str(), raw_q.as_str())
                    } else {
                        item.to_lowercase().contains(&needle_unicode)
                    };
                    if !matched {
                        continue;
                    }
                }
                let is_sel: bool = cur_sel == Some(idx);
                let on_pick = move |_: &UiEvent| {
                    let _: bool = selected.set_if_changed(Some(idx));
                };
                let row: NodeId = view! {
                    row {
                        width: Length::Percent(100.0)
                        padding: Edges::symmetric(6.0, SPACE_MD)
                        radius: live_g.get_radius_sm()
                        background: if is_sel {
                            live_p.get_accent()
                        } else {
                            COLOR_TRANSPARENT
                        }
                        cursor: CursorIcon::Pointer
                        on_click: on_pick
                        text {
                            font_size: 13.0
                            font_weight: if is_sel {
                                FontWeight::SemiBold
                            } else {
                                FontWeight::Normal
                            }
                            color: live_p.get_foreground()
                            { item.as_str() }
                        }
                    }
                };
                nodes.push(row);
            }
            if nodes.is_empty() {
                nodes.push(view! {
                    text {
                        font_size: 12.0
                        color: live_p.get_muted_foreground()
                        { COMMAND_EMPTY_LABEL }
                    }
                });
            }
            nodes
        };
        view! {
            column {
                width: Length::Percent(100.0)
                padding: Edges::all(SPACE_SM)
                gap: SPACE_SM
                background: p.get_card()
                border: (g.get_border_width(), p.get_border())
                radius: g.get_radius_lg()
                overflow: Overflow::Hidden
                shadow: g.get_shadow_md()
                EuvInput {
                    value: query
                    placeholder: resolved_ph
                    tokens: tokens
                }
                column {
                    width: Length::Percent(100.0)
                    gap: SPACE_XS
                    bind_children: build_cmd_items
                }
            }
        }
    }
}

/// Fluent controlled-state and body builders for `EuvHoverCard`.
impl EuvHoverCard {
    /// Binds a controlled reactive open visibility signal fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source hover card.
    /// - `Signal<bool>` - Controlled open flag signal.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated hover card.
    #[must_use]
    pub fn open(mut self, open: Signal<bool>) -> Self {
        self.set_open(Some(open));
        self
    }

    /// Sets the preview card description text fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source hover card.
    /// - `S` - Preview description string.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated hover card.
    #[must_use]
    pub fn description<S>(mut self, description: S) -> Self
    where
        S: Into<String>,
    {
        self.set_description(description.into());
        self
    }
}

/// Declarative `#[component]` implementation for `EuvHoverCard`.
#[component]
/// Inherent implementation of [`EuvHoverCard`].
impl EuvHoverCard {
    /// Renders `EuvHoverCard` into the ambient framework-managed `Context`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component descriptor reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted root scene node handle.
    fn render(&self) -> NodeId {
        let ctx: Context = current_context();
        let tokens: ThemeTokens = self.get_tokens();
        let p: ThemePalette = tokens.get_palette();
        let g: ThemeGeometry = tokens.get_geometry();
        let Self {
            open: open_opt,
            trigger_label,
            title,
            description,
            ..
        } = self;
        let open: Signal<bool> = open_opt.unwrap_or_else(|| Signal::new(false));
        let init_open: bool = open.get_untracked();
        let opened_by_hover: Rc<Cell<bool>> = Rc::new(Cell::new(false));
        let hover_click: Rc<Cell<bool>> = opened_by_hover.clone();
        let hover_enter: Rc<Cell<bool>> = opened_by_hover.clone();
        let hover_leave: Rc<Cell<bool>> = opened_by_hover;
        let on_enter = move |_: &UiEvent| {
            if !open.get_untracked() {
                hover_enter.set(true);
            }
            let _: bool = open.set_if_changed(true);
        };
        let on_leave = move |_: &UiEvent| {
            hover_leave.set(false);
            let _: bool = open.set_if_changed(false);
        };
        let on_btn_click = move |_: &UiEvent| {
            if hover_click.replace(false) {
                let _: bool = open.set_if_changed(true);
            } else {
                let _: bool = open.update(|v: &mut bool| *v = !*v);
            }
        };
        let mut popup: NodeId = NodeId::default();
        let root: NodeId = view! {
            column {
                align: AlignItems::Start
                gap: SPACE_XS
                on_pointer_enter: on_enter
                on_pointer_leave: on_leave
                EuvButton {
                    label: trigger_label.clone()
                    tokens: tokens
                    link()
                    sm()
                    on_click: on_btn_click
                }
                column {
                    node_ref: popup
                    display: if init_open { Display::Flex } else { Display::None }
                    width: Length::Percent(100.0)
                    max_width: Length::Px(320.0)
                    padding: Edges::all(SPACE_MD)
                    gap: SPACE_XS
                    background: p.get_card()
                    border: (g.get_border_width(), p.get_border())
                    radius: g.get_radius_lg()
                    overflow: Overflow::Hidden
                    shadow: g.get_shadow_lg()
                    z_index: Z_INDEX_FLOATING
                    text {
                        width: Length::Percent(100.0)
                        font_size: 13.0
                        font_weight: FontWeight::SemiBold
                        color: p.get_card_foreground()
                        { title.as_str() }
                    }
                    if !description.is_empty() {
                        text {
                            width: Length::Percent(100.0)
                            font_size: 12.0
                            color: p.get_muted_foreground()
                            { description.as_str() }
                        }
                    }
                }
            }
        };
        let handle: SceneHandle = ctx.get_scene().clone();
        let prev_open: Cell<bool> = Cell::new(init_open);
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            Effect::new(move || {
                let is_open: bool = open.get();
                if prev_open.replace(is_open) == is_open {
                    return;
                }
                let next_disp: Display = if is_open {
                    Display::Flex
                } else {
                    Display::None
                };
                handle.with_write(
                    |tree: &mut SceneTree| {
                        if let Some(n) = tree.get_node_mut(popup) {
                            n.get_mut_style().get_mut_layout().set_display(next_disp);
                            let dirty_l: u16 = DIRTY_LAYOUT | DIRTY_SUBTREE_LAYOUT;
                            tree.mark_dirty(popup, dirty_l | DIRTY_PAINT);
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

/// Declarative `#[component]` implementation for `EuvCalendar`.
#[component]
/// Inherent implementation of [`EuvCalendar`].
impl EuvCalendar {
    /// Renders `EuvCalendar` into the ambient framework-managed `Context`.
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
        let selected_day: Signal<u32> = self.get_selected_day();
        let total_days: u32 = self.get_days_in_month().clamp(28, 31);
        let Self { month_label, .. } = self;
        let init_day: u32 = selected_day.get_untracked();
        let border_w: f32 = g.get_border_width();
        let primary_col: Color = p.get_primary();
        let accent_bg: Color = p.get_accent();
        let on_border: Border = Border::new(border_w, primary_col);
        let off_border: Border = Border::new(0.0, primary_col);
        let mut days_grid: NodeId = NodeId::default();
        let day_nodes: Vec<NodeId> = (1..=total_days)
            .map(|day: u32| {
                let is_sel: bool = init_day == day;
                let on_pick = move |_: &UiEvent| {
                    let _: bool = selected_day.set_if_changed(day);
                };
                view! {
                    row {
                        height: Length::Px(28.0)
                        justify: JustifyContent::Center
                        align: AlignItems::Center
                        radius: g.get_radius_sm()
                        background: if is_sel { accent_bg } else { COLOR_TRANSPARENT }
                        border: (if is_sel { border_w } else { 0.0 }, primary_col)
                        cursor: CursorIcon::Pointer
                        on_click: on_pick
                        text {
                            font_size: 12.0
                            font_weight: FontWeight::Medium
                            color: p.get_foreground()
                            text_align: TextAlign::Center
                            { day.to_string() }
                        }
                    }
                }
            })
            .collect();
        let root: NodeId = view! {
            column {
                width: Length::Percent(100.0)
                padding: Edges::all(SPACE_MD)
                gap: SPACE_SM
                background: p.get_card()
                border: (border_w, p.get_border())
                radius: g.get_radius_lg()
                overflow: Overflow::Hidden
                text {
                    font_size: 14.0
                    font_weight: FontWeight::SemiBold
                    color: p.get_foreground()
                    text_align: TextAlign::Center
                    { month_label.as_str() }
                }
                grid {
                    columns: 7
                    width: Length::Percent(100.0)
                    gap: 2.0
                    for wd in CALENDAR_WEEKDAYS.iter().copied() {
                        row {
                            justify: JustifyContent::Center
                            align: AlignItems::Center
                            padding: Edges::symmetric(2.0, 0.0)
                            text {
                                font_size: 11.0
                                font_weight: FontWeight::Medium
                                color: p.get_muted_foreground()
                                text_align: TextAlign::Center
                                { wd }
                            }
                        }
                    }
                }
                grid {
                    node_ref: days_grid
                    columns: 7
                    width: Length::Percent(100.0)
                    gap: 2.0
                }
            }
        };
        if !day_nodes.is_empty() {
            ctx.append_many(days_grid, day_nodes.as_slice());
        }
        let handle: SceneHandle = ctx.get_scene().clone();
        let prev_day: Cell<u32> = Cell::new(init_day);
        let day_dirty: u16 = if border_w > 0.0 {
            let dirty_l: u16 = DIRTY_LAYOUT | DIRTY_SUBTREE_LAYOUT;
            dirty_l | DIRTY_PAINT
        } else {
            DIRTY_PAINT
        };
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            Effect::new(move || {
                let next_day: u32 = selected_day.get();
                let old_day: u32 = prev_day.replace(next_day);
                if old_day == next_day {
                    return;
                }
                handle.with_write(
                    |tree: &mut SceneTree| {
                        if let Some(old_slot) = old_day.checked_sub(1)
                            && let Some(&old_id) = day_nodes.get(old_slot as usize)
                            && let Some(n) = tree.get_node_mut(old_id)
                        {
                            n.get_mut_style()
                                .get_mut_paint()
                                .set_background(Some(COLOR_TRANSPARENT))
                                .set_border(off_border);
                            tree.mark_dirty(old_id, day_dirty);
                        }
                        if let Some(new_slot) = next_day.checked_sub(1)
                            && let Some(&new_id) = day_nodes.get(new_slot as usize)
                            && let Some(n) = tree.get_node_mut(new_id)
                        {
                            n.get_mut_style()
                                .get_mut_paint()
                                .set_background(Some(accent_bg))
                                .set_border(on_border);
                            tree.mark_dirty(new_id, day_dirty);
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

/// Declarative `#[component]` implementation for `EuvAspectRatio`.
#[component]
/// Inherent implementation of [`EuvAspectRatio`].
impl EuvAspectRatio {
    /// Renders `EuvAspectRatio` into the ambient framework-managed `Context`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component descriptor reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted root scene node handle.
    fn render(&self) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let ratio: f32 = if self.get_ratio() > 0.0 && self.get_ratio().is_finite() {
            self.get_ratio()
        } else {
            16.0 / 9.0
        };
        let child: NodeId = self.get_child();
        view! {
            column {
                width: Length::Percent(100.0)
                aspect_ratio: ratio
                justify: JustifyContent::Center
                align: AlignItems::Center
                background: p.get_muted()
                border: (g.get_border_width(), p.get_border())
                radius: g.get_radius_md()
                overflow: Overflow::Hidden
                { child }
            }
        }
    }
}

/// Declarative `#[component]` implementation for `EuvMenubar`.
#[component]
/// Inherent implementation of [`EuvMenubar`].
impl EuvMenubar {
    /// Renders `EuvMenubar` into the ambient framework-managed `Context`.
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
        let active_menu: Signal<Option<usize>> = self.get_active_menu();
        let Self { menus, .. } = self;
        let init_active: Option<usize> = active_menu.get_untracked();
        let accent_bg: Color = p.get_accent();
        let menu_nodes: Vec<NodeId> = menus
            .iter()
            .enumerate()
            .map(|(idx, label): (usize, &String)| {
                let is_open: bool = init_active == Some(idx);
                let on_pick = move |_: &UiEvent| {
                    let _: bool = active_menu.update(|cur: &mut Option<usize>| {
                        *cur = if *cur == Some(idx) { None } else { Some(idx) };
                    });
                };
                view! {
                    row {
                        height: Length::Px(g.get_control_sm())
                        justify: JustifyContent::Center
                        align: AlignItems::Center
                        padding: Edges::symmetric(0.0, SPACE_MD)
                        overflow: Overflow::Hidden
                        radius: g.get_radius_sm()
                        background: if is_open { accent_bg } else { COLOR_TRANSPARENT }
                        cursor: CursorIcon::Pointer
                        on_click: on_pick
                        text {
                            font_size: 13.0
                            font_weight: FontWeight::Medium
                            color: p.get_foreground()
                            text_align: TextAlign::Center
                            text_overflow: TextOverflow::Ellipsis
                            { label.as_str() }
                        }
                    }
                }
            })
            .collect();
        let root: NodeId = view! {
            row {
                flex_wrap: FlexWrap::Wrap
                width: Length::Percent(100.0)
                padding: Edges::all(SPACE_XS)
                gap: SPACE_XS
                background: p.get_card()
                border: (g.get_border_width(), p.get_border())
                radius: g.get_radius_md()
            }
        };
        if !menu_nodes.is_empty() {
            ctx.append_many(root, menu_nodes.as_slice());
        }
        let handle: SceneHandle = ctx.get_scene().clone();
        let prev_active: Cell<Option<usize>> = Cell::new(init_active);
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            Effect::new(move || {
                let next_active: Option<usize> = active_menu.get();
                let old_active: Option<usize> = prev_active.replace(next_active);
                if old_active == next_active {
                    return;
                }
                handle.with_write(
                    |tree: &mut SceneTree| {
                        if let Some(old_idx) = old_active
                            && let Some(&old_id) = menu_nodes.get(old_idx)
                            && let Some(n) = tree.get_node_mut(old_id)
                        {
                            n.get_mut_style()
                                .get_mut_paint()
                                .set_background(Some(COLOR_TRANSPARENT));
                            tree.mark_dirty(old_id, DIRTY_PAINT);
                        }
                        if let Some(new_idx) = next_active
                            && let Some(&new_id) = menu_nodes.get(new_idx)
                            && let Some(n) = tree.get_node_mut(new_id)
                        {
                            n.get_mut_style()
                                .get_mut_paint()
                                .set_background(Some(accent_bg));
                            tree.mark_dirty(new_id, DIRTY_PAINT);
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

/// Declarative `#[component]` implementation for `EuvCarousel`.
#[component]
/// Inherent implementation of [`EuvCarousel`].
impl EuvCarousel {
    /// Renders `EuvCarousel` into the ambient framework-managed `Context`.
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
        let g: ThemeGeometry = tokens.get_geometry();
        let active: Signal<usize> = self.get_active_slide();
        let Self { slides, .. } = self;
        let total: usize = slides.len();
        let slides_for_text: Rc<[String]> = Rc::from(slides.as_slice());
        let slide_label = move || -> String {
            if total == 0 {
                CAROUSEL_EMPTY_LABEL.to_string()
            } else {
                let idx: usize = active.get() % total;
                slides_for_text[idx].clone()
            }
        };
        let slide_counter = move || -> String {
            if total == 0 {
                CAROUSEL_ZERO_COUNTER.to_string()
            } else {
                format!("{} / {total}", (active.get() % total) + 1)
            }
        };
        let on_prev = move |_: &UiEvent| {
            if total > 0 {
                let _: bool = active.update(|v: &mut usize| {
                    *v = if *v == 0 { total - 1 } else { *v - 1 };
                });
            }
        };
        let on_next = move |_: &UiEvent| {
            if total > 0 {
                let _: bool = active.update(|v: &mut usize| *v = (*v + 1) % total);
            }
        };
        view! {
            column {
                width: Length::Percent(100.0)
                gap: SPACE_SM
                column {
                    width: Length::Percent(100.0)
                    padding: Edges::all(SPACE_XL)
                    justify: JustifyContent::Center
                    align: AlignItems::Center
                    background: p.get_muted()
                    border: (g.get_border_width(), p.get_border())
                    radius: g.get_radius_lg()
                    overflow: Overflow::Hidden
                    text {
                        font_size: 14.0
                        font_weight: FontWeight::SemiBold
                        color: p.get_foreground()
                        text_align: TextAlign::Center
                        { slide_label }
                    }
                }
                row {
                    flex_wrap: FlexWrap::Wrap
                    width: Length::Percent(100.0)
                    justify: JustifyContent::SpaceBetween
                    align: AlignItems::Center
                    gap: SPACE_SM
                    EuvButton {
                        label: PAGE_PREV_LABEL.to_string()
                        tokens: tokens
                        outline()
                        sm()
                        on_click: on_prev
                    }
                    text {
                        font_size: 12.0
                        color: p.get_muted_foreground()
                        { slide_counter }
                    }
                    EuvButton {
                        label: PAGE_NEXT_LABEL.to_string()
                        tokens: tokens
                        outline()
                        sm()
                        on_click: on_next
                    }
                }
            }
        }
    }
}
