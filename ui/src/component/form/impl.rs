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
        let Self { placeholder, .. } = self.clone();
        let a11y_label: String = placeholder.clone();
        let base_style: Style = Style::default()
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
        let node: NodeId = view! {
            input {
                style: style
                value: initial.clone()
                placeholder: placeholder
                caret: caret
                password: password
                multiline: false
                disabled: disabled
                role: A11yRole::TextInput
                a11y_label: a11y_label.clone()
                a11y_value: initial
                a11y_disabled: disabled
                on_change: move |ev: &UiEvent| {
                    if let UiEventKind::Change(val) = ev.get_kind() {
                        let _: bool = sig.set_if_changed(val.clone());
                    }
                }
            }
        };
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
        let Self { label, error, .. } = self.clone();
        view! {
            column {
                width: Length::Percent(100.0)
                gap: SPACE_XS
                text {
                    font_size: 13.0
                    font_weight: FontWeight::Medium
                    color: p.get_foreground()
                    { label }
                }
                { control }
                if !error.is_empty() {
                    text {
                        font_size: 12.0
                        color: p.get_destructive()
                        { error }
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
        let Self { label, .. } = self.clone();
        let a11y_label: String = label.clone();
        let off_track: Color = p.get_switch_track_off();
        let init_on: bool = checked.get_untracked();
        let mut track: NodeId = NodeId::default();
        let mut thumb: NodeId = NodeId::default();
        let row: NodeId = view! {
            row {
                align: AlignItems::Center
                padding: Edges::symmetric(SPACE_XS, SPACE_SM)
                radius: g.get_radius_sm()
                gap: SPACE_SM
                cursor: CursorIcon::Pointer
                on_click: move |_: &UiEvent| {
                    let _: bool = checked.update(|v: &mut bool| *v = !*v);
                }
                row {
                    node_ref: track
                    width: Length::Px(44.0)
                    height: Length::Px(24.0)
                    padding: Edges::all(2.0)
                    align: AlignItems::Center
                    justify: if init_on { JustifyContent::End } else { JustifyContent::Start }
                    radius: 12.0
                    background: if init_on { p.get_primary() } else { off_track }
                    view {
                        node_ref: thumb
                        width: Length::Px(20.0)
                        height: Length::Px(20.0)
                        radius: 10.0
                        background: if init_on { p.get_primary_foreground() } else { p.get_switch_thumb() }
                        shadow: g.get_shadow_xs()
                    }
                }
                text {
                    flex_grow: 1.0
                    font_size: 13.0
                    font_weight: FontWeight::Medium
                    color: p.get_foreground()
                    { label }
                }
            }
        };
        let handle: SceneHandle = ctx.get_scene().clone();
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            Effect::new(move || {
                let is_on: bool = checked.get();
                let lbl: String = a11y_label.clone();
                handle.with_write(
                    |tree: &mut SceneTree| {
                        tree.set_style(
                            track,
                            Style::default()
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
                            Style::default()
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
        let Self { label, .. } = self.clone();
        let a11y_label: String = label.clone();
        let check_fg: Color = p.get_primary_foreground();
        let mut indicator: NodeId = NodeId::default();
        let mut mark: NodeId = NodeId::default();
        let row: NodeId = view! {
            row {
                align: AlignItems::Center
                padding: Edges::symmetric(SPACE_XS, SPACE_SM)
                radius: g.get_radius_sm()
                gap: SPACE_SM
                cursor: CursorIcon::Pointer
                on_click: move |_: &UiEvent| {
                    let _: bool = checked.update(|v: &mut bool| *v = !*v);
                }
                row {
                    node_ref: indicator
                    justify: JustifyContent::Center
                    align: AlignItems::Center
                    width: Length::Px(18.0)
                    height: Length::Px(18.0)
                    border: (g.get_border_width(), p.get_primary())
                    radius: g.get_radius_sm()
                    canvas {
                        node_ref: mark
                        width: Length::Px(12.0)
                        height: Length::Px(12.0)
                        painter: move |bounds: Rect, dl: &mut DisplayList| {
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
                        }
                    }
                }
                text {
                    flex_grow: 1.0
                    font_size: 14.0
                    color: p.get_foreground()
                    { label }
                }
            }
        };
        let handle: SceneHandle = ctx.get_scene().clone();
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            Effect::new(move || {
                let is_on: bool = checked.get();
                let lbl: String = a11y_label.clone();
                handle.with_write(
                    |tree: &mut SceneTree| {
                        tree.set_style(
                            indicator,
                            Style::default()
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
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let selected: Signal<usize> = self.get_selected();
        let Self { options, .. } = self.clone();
        view! {
            row {
                flex_wrap: FlexWrap::Wrap
                width: Length::Percent(100.0)
                padding: Edges::all(SPACE_XS)
                gap: SPACE_XS
                background: p.get_muted()
                radius: g.get_radius_md()
                role: A11yRole::TabList
                for (idx, opt) in options.into_iter().enumerate() {
                    row {
                        role: A11yRole::Tab
                        a11y_label: opt.clone()
                        bind_style: move || {
                            let active: bool = selected.get() == idx;
                            let base: Style = Style::default()
                            .row()
                            .height(Length::Px(g.get_control_sm()))
                            .justify(JustifyContent::Center)
                            .align(AlignItems::Center)
                            .padding(Edges::symmetric(0.0, SPACE_MD))
                            .overflow(Overflow::Hidden)
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
                        }
                        on_click: move |_: &UiEvent| {
                            let _: bool = selected.set_if_changed(idx);
                        }
                        text {
                            font_size: 13.0
                            font_weight: FontWeight::Medium
                            color: p.get_foreground()
                            text_align: TextAlign::Center
                            text_overflow: TextOverflow::Ellipsis
                            { opt }
                        }
                    }
                }
            }
        }
    }
}
