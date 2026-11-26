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

/// `Mountable` implementation for `EuvInput`.
impl Mountable for EuvInput {
    /// Mounts the input control and binds its `Change` events and external `Signal<String>` updates bidirectionally.
    ///
    /// # Arguments
    ///
    /// - `Self` - Input props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted input handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let sig: Signal<String> = self.get_value();
        let initial: String = sig.get_untracked();
        let caret: usize = initial.len();
        let password: bool = self.get_password();
        let disabled: bool = self.get_disabled();
        let borderless: bool = self.get_borderless();
        let Self { placeholder, .. } = self;
        let a11y_label: String = placeholder.clone();
        let state: InputNodeState = InputNodeState::new(
            initial.clone(),
            placeholder,
            caret,
            password,
            false,
            disabled,
        );
        let base_style: Style = Style::empty()
            .width(Length::Percent(100.0))
            .height(Length::Px(g.get_control_md()))
            .padding(Edges::symmetric(0.0, SPACE_MD))
            .color(p.get_foreground())
            .font_size(13.0);
        let style: Style = if borderless {
            base_style.background(COLOR_TRANSPARENT)
        } else {
            base_style
                .background(p.get_card())
                .border(g.get_border_width(), p.get_border())
                .radius(g.get_radius_md())
                .focus_ring(Border::new(g.get_ring_width(), p.get_ring()))
        };
        let node: NodeId = ctx.input(state, style);
        let mut a11y: A11yProps = A11yProps::new(A11yRole::TextInput, a11y_label.clone());
        a11y.set_value(initial);
        a11y.set_disabled(disabled);
        ctx.set_a11y(node, a11y);
        ctx.on_change(node, move |ev: &UiEvent| {
            if let UiEventKind::Change(val) = ev.get_kind() {
                let _: bool = sig.set_if_changed(val.clone());
            }
        });
        let handle: SceneHandle = ctx.get_scene().clone();
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            Effect::new(move || {
                let next_val: String = sig.get();
                let lbl: String = a11y_label.clone();
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
                            let mut next_a11y: A11yProps = A11yProps::new(A11yRole::TextInput, lbl);
                            next_a11y.set_value(next_val);
                            next_a11y.set_disabled(disabled);
                            n.set_a11y(Some(next_a11y));
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

/// `Mountable` implementation for `EuvField`.
impl Mountable for EuvField {
    /// Mounts label, control, and optional error text vertically.
    ///
    /// # Arguments
    ///
    /// - `Self` - Field props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted field container handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let control: NodeId = self.get_control();
        let Self { label, error, .. } = self;
        let col: NodeId = ctx.view(
            Style::empty()
                .column()
                .width(Length::Percent(100.0))
                .gap(SPACE_XS),
        );
        let lbl: NodeId = ctx.text(
            label,
            Style::empty()
                .font_size(13.0)
                .font_weight(FontWeight::Medium)
                .color(p.get_foreground()),
        );
        ctx.append(col, lbl);
        ctx.append(col, control);
        if !error.is_empty() {
            let err_node: NodeId = ctx.text(
                error,
                Style::empty().font_size(12.0).color(p.get_destructive()),
            );
            ctx.append(col, err_node);
        }
        col
    }
}

/// `Mountable` implementation for `EuvSwitch`.
impl Mountable for EuvSwitch {
    /// Mounts an interactive toggle track and thumb bound reactively to `checked`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Switch props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted switch row handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let checked: Signal<bool> = self.get_checked();
        let Self { label, .. } = self;
        let a11y_label: String = label.clone();
        let row: NodeId = ctx.view(
            Style::empty()
                .row()
                .align(AlignItems::Center)
                .padding(Edges::symmetric(SPACE_XS, SPACE_SM))
                .radius(g.get_radius_sm())
                .gap(SPACE_SM)
                .cursor(CursorIcon::Pointer),
        );
        let off_track: Color = p.get_switch_track_off();
        let init_on: bool = checked.get_untracked();
        let track: NodeId = ctx.view(
            Style::empty()
                .row()
                .width(Length::Px(44.0))
                .height(Length::Px(24.0))
                .padding(Edges::all(2.0))
                .align(AlignItems::Center)
                .justify(if init_on {
                    JustifyContent::End
                } else {
                    JustifyContent::Start
                })
                .radius(12.0)
                .background(if init_on { p.get_primary() } else { off_track }),
        );
        let thumb: NodeId = ctx.view(
            Style::empty()
                .width(Length::Px(20.0))
                .height(Length::Px(20.0))
                .radius(10.0)
                .background(if init_on {
                    p.get_primary_foreground()
                } else {
                    p.get_switch_thumb()
                })
                .shadow(g.get_shadow_xs()),
        );
        ctx.append(track, thumb);
        let handle: SceneHandle = ctx.get_scene().clone();
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            Effect::new(move || {
                let is_on: bool = checked.get();
                let lbl: String = a11y_label.clone();
                handle.with_write(
                    |tree: &mut SceneTree| {
                        tree.set_style(
                            track,
                            Style::empty()
                                .row()
                                .width(Length::Px(44.0))
                                .height(Length::Px(24.0))
                                .padding(Edges::all(2.0))
                                .align(AlignItems::Center)
                                .justify(if is_on {
                                    JustifyContent::End
                                } else {
                                    JustifyContent::Start
                                })
                                .radius(12.0)
                                .background(if is_on { p.get_primary() } else { off_track }),
                        );
                        tree.set_style(
                            thumb,
                            Style::empty()
                                .width(Length::Px(20.0))
                                .height(Length::Px(20.0))
                                .radius(10.0)
                                .background(if is_on {
                                    p.get_primary_foreground()
                                } else {
                                    p.get_switch_thumb()
                                })
                                .shadow(g.get_shadow_xs()),
                        );
                        if let Some(n) = tree.get_node_mut(row) {
                            let mut a11y: A11yProps = A11yProps::new(A11yRole::Switch, lbl);
                            a11y.set_checked(Some(is_on));
                            n.set_a11y(Some(a11y));
                        }
                    },
                    (),
                );
            })
        });
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                tree.attach_scope(track, scope);
            },
            (),
        );
        ctx.on_click(row, move |_: &UiEvent| {
            let _: bool = checked.update(|v: &mut bool| *v = !*v);
        });
        let txt: NodeId = ctx.text(
            label,
            Style::empty()
                .flex_grow(1.0)
                .font_size(13.0)
                .font_weight(FontWeight::Medium)
                .color(p.get_foreground()),
        );
        ctx.append(row, track);
        ctx.append(row, txt);
        row
    }
}

/// `Mountable` implementation for `EuvCheckbox`.
impl Mountable for EuvCheckbox {
    /// Mounts a clickable checkbox indicator and label.
    ///
    /// # Arguments
    ///
    /// - `Self` - Checkbox props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted checkbox row handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let checked: Signal<bool> = self.get_checked();
        let Self { label, .. } = self;
        let a11y_label: String = label.clone();
        let row: NodeId = ctx.view(
            Style::empty()
                .row()
                .align(AlignItems::Center)
                .padding(Edges::symmetric(SPACE_XS, SPACE_SM))
                .radius(g.get_radius_sm())
                .gap(SPACE_SM)
                .cursor(CursorIcon::Pointer),
        );
        let indicator: NodeId = ctx.view(
            Style::empty()
                .row()
                .justify(JustifyContent::Center)
                .align(AlignItems::Center)
                .width(Length::Px(18.0))
                .height(Length::Px(18.0))
                .border(g.get_border_width(), p.get_primary())
                .radius(g.get_radius_sm()),
        );
        let check_fg: Color = p.get_primary_foreground();
        let mark: NodeId = ctx.canvas(
            Style::empty()
                .width(Length::Px(12.0))
                .height(Length::Px(12.0)),
            move |bounds: Rect, dl: &mut DisplayList| {
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
            },
        );
        ctx.append(indicator, mark);
        let handle: SceneHandle = ctx.get_scene().clone();
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            Effect::new(move || {
                let is_on: bool = checked.get();
                let lbl: String = a11y_label.clone();
                handle.with_write(
                    |tree: &mut SceneTree| {
                        tree.set_style(
                            indicator,
                            Style::empty()
                                .row()
                                .justify(JustifyContent::Center)
                                .align(AlignItems::Center)
                                .width(Length::Px(18.0))
                                .height(Length::Px(18.0))
                                .background(if is_on { p.get_primary() } else { p.get_card() })
                                .border(g.get_border_width(), p.get_primary())
                                .radius(g.get_radius_sm()),
                        );
                        tree.mark_dirty(mark, DIRTY_PAINT);
                        if let Some(n) = tree.get_node_mut(row) {
                            let mut a11y: A11yProps = A11yProps::new(A11yRole::Checkbox, lbl);
                            a11y.set_checked(Some(is_on));
                            n.set_a11y(Some(a11y));
                        }
                    },
                    (),
                );
            })
        });
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                tree.attach_scope(indicator, scope);
            },
            (),
        );
        ctx.on_click(row, move |_: &UiEvent| {
            let _: bool = checked.update(|v: &mut bool| *v = !*v);
        });
        let txt: NodeId = ctx.text(
            label,
            Style::empty()
                .flex_grow(1.0)
                .font_size(14.0)
                .color(p.get_foreground()),
        );
        ctx.append(row, indicator);
        ctx.append(row, txt);
        row
    }
}

/// `Mountable` implementation for `EuvSegmented`.
impl Mountable for EuvSegmented {
    /// Mounts mutually exclusive option pills with reactive active styling.
    ///
    /// # Arguments
    ///
    /// - `Self` - Segmented props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted segmented control handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let selected: Signal<usize> = self.get_selected();
        let Self { options, .. } = self;
        let container: NodeId = ctx.view(
            Style::empty()
                .row()
                .wrap(FlexWrap::Wrap)
                .width(Length::Percent(100.0))
                .padding(Edges::all(SPACE_XS))
                .gap(SPACE_XS)
                .background(p.get_muted())
                .radius(g.get_radius_md()),
        );
        ctx.set_a11y(container, A11yProps::new(A11yRole::TabList, String::new()));
        for (idx, opt) in options.into_iter().enumerate() {
            let pill: NodeId = ctx.view(Style::empty());
            ctx.set_a11y(pill, A11yProps::new(A11yRole::Tab, opt.clone()));
            let txt: NodeId = ctx.text(
                opt,
                Style::empty()
                    .font_size(13.0)
                    .font_weight(FontWeight::Medium)
                    .color(p.get_foreground())
                    .text_overflow(TextOverflow::Clip),
            );
            ctx.append(pill, txt);
            let handle: SceneHandle = ctx.get_scene().clone();
            let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
                handle.bind_style(pill, move || {
                    let active: bool = selected.get() == idx;
                    let base: Style = Style::empty()
                        .row()
                        .height(Length::Px(g.get_control_sm()))
                        .justify(JustifyContent::Center)
                        .align(AlignItems::Center)
                        .padding(Edges::symmetric(0.0, SPACE_MD))
                        .radius(g.get_radius_sm())
                        .background(if active {
                            p.get_card()
                        } else {
                            COLOR_TRANSPARENT
                        })
                        .border(
                            if active { g.get_border_width() } else { 0.0 },
                            p.get_border(),
                        )
                        .cursor(CursorIcon::Pointer);
                    if active {
                        base.shadow(g.get_shadow_xs())
                    } else {
                        base
                    }
                })
            });
            ctx.get_scene().with_write(
                |tree: &mut SceneTree| {
                    tree.attach_scope(pill, scope);
                },
                (),
            );
            ctx.on_click(pill, move |_: &UiEvent| {
                let _: bool = selected.set_if_changed(idx);
            });
            ctx.append(container, pill);
        }
        container
    }
}
