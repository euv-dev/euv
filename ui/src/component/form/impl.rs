use super::*;

/// Fluent configuration builders for `EuvInput`.
impl EuvInput {
    /// Enables or disables password masking fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source input.
    /// - `bool` - Whether characters are masked.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated input.
    #[must_use]
    pub fn password(mut self, password: bool) -> Self {
        self.set_password(password);
        self
    }

    /// Enables or disables interaction fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source input.
    /// - `bool` - Whether editing is disabled.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated input.
    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.set_disabled(disabled);
        self
    }

    /// Removes outer chrome so the input embeds cleanly inside `EuvInputGroup`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source input.
    /// - `bool` - Whether borderless embedding is active.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated input.
    #[must_use]
    pub fn borderless(mut self, borderless: bool) -> Self {
        self.set_borderless(borderless);
        self
    }
}

/// Declarative `#[component]` implementation for `EuvInput`.
#[component]
/// Inherent implementation of [`EuvInput`].
impl EuvInput {
    /// Renders `EuvInput` into the ambient framework-managed `Context`.
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
        let sig: Signal<String> = self.get_value();
        let initial: String = sig.get_untracked();
        let caret: usize = initial.len();
        let password: bool = self.get_password();
        let disabled: bool = self.get_disabled();
        let borderless: bool = self.get_borderless();
        let Self { placeholder, .. } = self;
        let placeholder_owned: String = placeholder.clone();
        let a11y_label: String = placeholder_owned.clone();
        let mut style: Style = Style::default();
        style
            .get_mut_layout()
            .set_width(Length::Percent(100.0))
            .set_height(Length::Px(g.get_control_md()))
            .set_padding(Edges::symmetric(0.0, SPACE_MD));
        style
            .get_mut_text()
            .set_color(p.get_foreground())
            .set_font_size(13.0);
        if borderless {
            style
                .get_mut_paint()
                .set_background(Some(COLOR_TRANSPARENT));
        } else {
            style
                .get_mut_paint()
                .set_background(Some(p.get_card()))
                .set_border(Border::new(g.get_border_width(), p.get_border()))
                .set_radius(CornerRadius::all(g.get_radius_md().max(0.0)))
                .set_focus_ring(Some(Border::new(g.get_ring_width(), p.get_ring())));
        }
        let on_input_change = move |ev: &UiEvent| {
            if let UiEventKind::Change(val) = ev.get_kind() {
                let _: bool = sig.set_if_changed(val.clone());
            }
        };
        let node: NodeId = view! {
            input {
                style: style
                value: initial.clone()
                placeholder: placeholder_owned
                caret: caret
                password: password
                multiline: false
                disabled: disabled
                role: A11yRole::TextInput
                a11y_label: a11y_label.clone()
                a11y_value: initial
                a11y_disabled: disabled
                on_change: on_input_change
            }
        };
        let handle: SceneHandle = ctx.get_scene().clone();
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            Effect::new(move || {
                let next_val: String = sig.get();
                handle.with_write(
                    |tree: &mut SceneTree| {
                        let mut changed: bool = false;
                        if let Some(n) = tree.get_node_mut(node) {
                            if let NodeKind::Input(inp) = n.get_mut_kind()
                                && inp.get_value() != &next_val
                            {
                                let len: usize = next_val.len();
                                inp.set_value(next_val.clone());
                                inp.set_caret(len);
                                changed = true;
                            }
                            if let Some(a11y) = n.get_mut_a11y().as_mut() {
                                a11y.set_value(next_val);
                                a11y.set_disabled(disabled);
                            } else {
                                let mut next_a11y: A11yProps =
                                    A11yProps::new(A11yRole::TextInput, a11y_label.clone());
                                next_a11y.set_value(next_val);
                                next_a11y.set_disabled(disabled);
                                n.set_a11y(Some(next_a11y));
                            }
                        }
                        if changed {
                            tree.mark_dirty(node, DIRTY_PAINT);
                        }
                    },
                    (),
                );
            })
        });
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                tree.attach_scope(node, scope);
            },
            (),
        );
        node
    }
}

/// Fluent configuration builders for `EuvField`.
impl EuvField {
    /// Sets a validation error caption rendered beneath the field control.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source field.
    /// - `S` - Error message text.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated field.
    #[must_use]
    pub fn error<S>(mut self, error: S) -> Self
    where
        S: Into<String>,
    {
        self.set_error(error.into());
        self
    }
}

/// Declarative `#[component]` implementation for `EuvField`.
#[component]
/// Inherent implementation of [`EuvField`].
impl EuvField {
    /// Renders `EuvField` into the ambient framework-managed `Context`.
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
        let control: NodeId = self.get_control();
        let Self { label, error, .. } = self;
        view! {
            column {
                width: Length::Percent(100.0)
                gap: SPACE_XS
                text {
                    font_size: 13.0
                    font_weight: FontWeight::Medium
                    color: p.get_foreground()
                    { label.clone() }
                }
                { control }
                if !error.is_empty() {
                    text {
                        font_size: 12.0
                        color: p.get_destructive()
                        { error.clone() }
                    }
                }
            }
        }
    }
}

/// Declarative `#[component]` implementation for `EuvSwitch`.
#[component]
/// Inherent implementation of [`EuvSwitch`].
impl EuvSwitch {
    /// Renders `EuvSwitch` into the ambient framework-managed `Context`.
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
        let checked: Signal<bool> = self.get_checked();
        let Self { label, .. } = self;
        let label_owned: String = label.clone();
        let a11y_label: String = label_owned.clone();
        let on_track: Color = p.get_primary();
        let off_track: Color = p.get_switch_track_off();
        let on_thumb: Color = p.get_primary_foreground();
        let off_thumb: Color = p.get_switch_thumb();
        let init_on: bool = checked.get_untracked();
        let mut track: NodeId = NodeId::default();
        let mut thumb: NodeId = NodeId::default();
        let on_toggle = move |_: &UiEvent| {
            let _: bool = checked.update(|v: &mut bool| *v = !*v);
        };
        let row: NodeId = view! {
            row {
                align: AlignItems::Center
                padding: Edges::symmetric(SPACE_XS, SPACE_SM)
                radius: g.get_radius_sm()
                gap: SPACE_SM
                cursor: CursorIcon::Pointer
                on_click: on_toggle
                row {
                    node_ref: track
                    width: Length::Px(44.0)
                    height: Length::Px(24.0)
                    padding: Edges::all(2.0)
                    align: AlignItems::Center
                    justify: if init_on { JustifyContent::End } else { JustifyContent::Start }
                    radius: 12.0
                    background: if init_on { on_track } else { off_track }
                    view {
                        node_ref: thumb
                        width: Length::Px(20.0)
                        height: Length::Px(20.0)
                        radius: 10.0
                        background: if init_on { on_thumb } else { off_thumb }
                        shadow: g.get_shadow_xs()
                    }
                }
                text {
                    flex_grow: 1.0
                    font_size: 13.0
                    font_weight: FontWeight::Medium
                    color: p.get_foreground()
                    { label_owned }
                }
            }
        };
        let handle: SceneHandle = ctx.get_scene().clone();
        let prev_on: Cell<bool> = Cell::new(init_on);
        let a11y_fallback: String = a11y_label.clone();
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            Effect::new(move || {
                let is_on: bool = checked.get();
                if prev_on.replace(is_on) == is_on {
                    return;
                }
                handle.with_write(
                    |tree: &mut SceneTree| {
                        if let Some(n) = tree.get_node_mut(track) {
                            let st: &mut Style = n.get_mut_style();
                            st.get_mut_layout().set_justify_content(if is_on {
                                JustifyContent::End
                            } else {
                                JustifyContent::Start
                            });
                            st.get_mut_paint().set_background(Some(if is_on {
                                on_track
                            } else {
                                off_track
                            }));
                            let dirty_l: u16 = DIRTY_LAYOUT | DIRTY_SUBTREE_LAYOUT;
                            tree.mark_dirty(track, dirty_l | DIRTY_PAINT);
                        }
                        if let Some(n) = tree.get_node_mut(thumb) {
                            n.get_mut_style()
                                .get_mut_paint()
                                .set_background(Some(if is_on { on_thumb } else { off_thumb }));
                            tree.mark_dirty(thumb, DIRTY_PAINT);
                        }
                        if let Some(n) = tree.get_node_mut(row) {
                            if let Some(a11y) = n.get_mut_a11y().as_mut() {
                                a11y.set_checked(Some(is_on));
                            } else {
                                let mut a11y: A11yProps =
                                    A11yProps::new(A11yRole::Switch, a11y_fallback.clone());
                                a11y.set_checked(Some(is_on));
                                n.set_a11y(Some(a11y));
                            }
                        }
                    },
                    (),
                );
            })
        });
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                if let Some(n) = tree.get_node_mut(row) {
                    let mut a11y: A11yProps = A11yProps::new(A11yRole::Switch, a11y_label);
                    a11y.set_checked(Some(init_on));
                    n.set_a11y(Some(a11y));
                }
                tree.attach_scope(track, scope);
            },
            (),
        );
        row
    }
}

/// Declarative `#[component]` implementation for `EuvCheckbox`.
#[component]
/// Inherent implementation of [`EuvCheckbox`].
impl EuvCheckbox {
    /// Renders `EuvCheckbox` into the ambient framework-managed `Context`.
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
        let checked: Signal<bool> = self.get_checked();
        let Self { label, .. } = self;
        let label_owned: String = label.clone();
        let a11y_label: String = label_owned.clone();
        let on_bg: Color = p.get_primary();
        let off_bg: Color = p.get_card();
        let check_fg: Color = p.get_primary_foreground();
        let init_on: bool = checked.get_untracked();
        let mut indicator: NodeId = NodeId::default();
        let mut mark: NodeId = NodeId::default();
        let on_toggle = move |_: &UiEvent| {
            let _: bool = checked.update(|v: &mut bool| *v = !*v);
        };
        let paint_check = move |bounds: Rect, dl: &mut DisplayList| {
            if checked.get_untracked() {
                let x0: f32 = bounds.get_x() + 1.5;
                let y0: f32 = bounds.get_y() + 6.0;
                let xm: f32 = bounds.get_x() + 4.8;
                let ym: f32 = bounds.get_y() + 9.5;
                let x1: f32 = bounds.get_x() + 10.5;
                let y1: f32 = bounds.get_y() + 2.5;
                dl.push(DrawCommand::Line {
                    from: Point::new(x0, y0),
                    to: Point::new(xm, ym),
                    color: check_fg,
                    width: 2.0,
                });
                dl.push(DrawCommand::Line {
                    from: Point::new(xm, ym),
                    to: Point::new(x1, y1),
                    color: check_fg,
                    width: 2.0,
                });
            }
        };
        let row: NodeId = view! {
            row {
                align: AlignItems::Center
                padding: Edges::symmetric(SPACE_XS, SPACE_SM)
                radius: g.get_radius_sm()
                gap: SPACE_SM
                cursor: CursorIcon::Pointer
                on_click: on_toggle
                row {
                    node_ref: indicator
                    justify: JustifyContent::Center
                    align: AlignItems::Center
                    width: Length::Px(18.0)
                    height: Length::Px(18.0)
                    background: if init_on { on_bg } else { off_bg }
                    border: (g.get_border_width(), on_bg)
                    radius: g.get_radius_sm()
                    canvas {
                        node_ref: mark
                        width: Length::Px(12.0)
                        height: Length::Px(12.0)
                        painter: paint_check
                    }
                }
                text {
                    flex_grow: 1.0
                    font_size: 14.0
                    color: p.get_foreground()
                    { label_owned }
                }
            }
        };
        let handle: SceneHandle = ctx.get_scene().clone();
        let prev_on: Cell<bool> = Cell::new(init_on);
        let a11y_fallback: String = a11y_label.clone();
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            Effect::new(move || {
                let is_on: bool = checked.get();
                if prev_on.replace(is_on) == is_on {
                    return;
                }
                handle.with_write(
                    |tree: &mut SceneTree| {
                        if let Some(n) = tree.get_node_mut(indicator) {
                            n.get_mut_style()
                                .get_mut_paint()
                                .set_background(Some(if is_on { on_bg } else { off_bg }));
                            tree.mark_dirty(indicator, DIRTY_PAINT);
                        }
                        tree.mark_dirty(mark, DIRTY_PAINT);
                        if let Some(n) = tree.get_node_mut(row) {
                            if let Some(a11y) = n.get_mut_a11y().as_mut() {
                                a11y.set_checked(Some(is_on));
                            } else {
                                let mut a11y: A11yProps =
                                    A11yProps::new(A11yRole::Checkbox, a11y_fallback.clone());
                                a11y.set_checked(Some(is_on));
                                n.set_a11y(Some(a11y));
                            }
                        }
                    },
                    (),
                );
            })
        });
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                if let Some(n) = tree.get_node_mut(row) {
                    let mut a11y: A11yProps = A11yProps::new(A11yRole::Checkbox, a11y_label);
                    a11y.set_checked(Some(init_on));
                    n.set_a11y(Some(a11y));
                }
                tree.attach_scope(indicator, scope);
            },
            (),
        );
        row
    }
}

/// Declarative `#[component]` implementation for `EuvSegmented`.
#[component]
/// Inherent implementation of [`EuvSegmented`].
impl EuvSegmented {
    /// Renders `EuvSegmented` into the ambient framework-managed `Context`.
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
        let active_bg: Color = p.get_card();
        let border_w: f32 = g.get_border_width();
        let border_col: Color = p.get_border();
        let active_shadow: Shadow = g.get_shadow_xs();
        let tab_nodes: Vec<NodeId> = options
            .iter()
            .enumerate()
            .map(|(idx, opt): (usize, &String)| {
                let active: bool = init_sel == idx;
                let on_pick = move |_: &UiEvent| {
                    let _: bool = selected.set_if_changed(idx);
                };
                let mut tab_style: Style = Style::default();
                tab_style
                    .get_mut_layout()
                    .set_flex_direction(FlexDirection::Row)
                    .set_height(Length::Px(g.get_control_sm()))
                    .set_justify_content(JustifyContent::Center)
                    .set_align_items(AlignItems::Center)
                    .set_padding(Edges::symmetric(0.0, SPACE_MD))
                    .set_overflow(Overflow::Hidden);
                tab_style
                    .get_mut_paint()
                    .set_radius(CornerRadius::all(g.get_radius_sm().max(0.0)))
                    .set_background(Some(if active { active_bg } else { COLOR_TRANSPARENT }))
                    .set_border(Border::new(if active { border_w } else { 0.0 }, border_col))
                    .set_shadow(if active { Some(active_shadow) } else { None })
                    .set_cursor(CursorIcon::Pointer);
                view! {
                    row {
                        style: tab_style
                        role: A11yRole::Tab
                        a11y_label: opt.clone()
                        on_click: on_pick
                        text {
                            font_size: 13.0
                            font_weight: FontWeight::Medium
                            color: p.get_foreground()
                            text_align: TextAlign::Center
                            text_overflow: TextOverflow::Ellipsis
                            { opt.clone() }
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
                background: p.get_muted()
                radius: g.get_radius_md()
                role: A11yRole::TabList
            }
        };
        if !tab_nodes.is_empty() {
            ctx.append_many(root, tab_nodes.as_slice());
        }
        let handle: SceneHandle = ctx.get_scene().clone();
        let prev_sel: Cell<usize> = Cell::new(init_sel);
        let dirty_mask: u16 = if border_w > 0.0 {
            let dirty_l: u16 = DIRTY_LAYOUT | DIRTY_SUBTREE_LAYOUT;
            dirty_l | DIRTY_PAINT
        } else {
            DIRTY_PAINT
        };
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            Effect::new(move || {
                let next_sel: usize = selected.get();
                let old_sel: usize = prev_sel.replace(next_sel);
                if old_sel == next_sel {
                    return;
                }
                handle.with_write(
                    |tree: &mut SceneTree| {
                        if let Some(&old_node) = tab_nodes.get(old_sel)
                            && let Some(n) = tree.get_node_mut(old_node)
                        {
                            n.get_mut_style()
                                .get_mut_paint()
                                .set_background(Some(COLOR_TRANSPARENT))
                                .set_border(Border::new(0.0, border_col))
                                .set_shadow(None);
                            tree.mark_dirty(old_node, dirty_mask);
                        }
                        if let Some(&new_node) = tab_nodes.get(next_sel)
                            && let Some(n) = tree.get_node_mut(new_node)
                        {
                            n.get_mut_style()
                                .get_mut_paint()
                                .set_background(Some(active_bg))
                                .set_border(Border::new(border_w, border_col))
                                .set_shadow(Some(active_shadow));
                            tree.mark_dirty(new_node, dirty_mask);
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
