use super::*;

/// `Mountable` implementation for `EuvInput`.
impl Mountable for EuvInput {
    /// Mounts the input control and binds its `Change` events back to the reactive `Signal<String>`.
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
        let Self { placeholder, .. } = self;
        let state: InputNodeState =
            InputNodeState::new(initial, placeholder, caret, password, false, disabled);
        let style: Style = Style::empty()
            .width(Length::Percent(100.0))
            .height(Length::Px(g.get_control_md()))
            .padding(Edges::symmetric(SPACE_SM, SPACE_MD))
            .background(p.get_background())
            .border(g.get_border_width(), p.get_border())
            .radius(g.get_radius_md())
            .focus_ring(Border::new(g.get_ring_width(), p.get_ring()))
            .color(p.get_foreground())
            .font_size(g.get_font_body());
        let node: NodeId = ctx.input(state, style);
        ctx.on_change(node, move |ev: &UiEvent| {
            if let UiEventKind::Change(val) = ev.get_kind() {
                let _: bool = sig.set_if_changed(val.clone());
            }
        });
        node
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
        let checked: Signal<bool> = self.get_checked();
        let Self { label, .. } = self;
        let row: NodeId = ctx.view(Style::empty().row().align(AlignItems::Center).gap(SPACE_SM));
        let track: NodeId = ctx.view(
            Style::empty()
                .row()
                .width(Length::Px(42.0))
                .height(Length::Px(24.0))
                .padding(Edges::all(2.0))
                .radius(12.0)
                .background(if checked.get_untracked() {
                    p.get_primary()
                } else {
                    p.get_secondary()
                }),
        );
        let thumb: NodeId = ctx.view(
            Style::empty()
                .width(Length::Px(20.0))
                .height(Length::Px(20.0))
                .radius(10.0)
                .background(Color::hex(0xFFFFFF)),
        );
        ctx.append(track, thumb);
        let handle: SceneHandle = ctx.get_scene().clone();
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            handle.bind_style(track, move || {
                let is_on: bool = checked.get();
                Style::empty()
                    .row()
                    .width(Length::Px(42.0))
                    .height(Length::Px(24.0))
                    .padding(Edges::all(2.0))
                    .justify(if is_on {
                        JustifyContent::End
                    } else {
                        JustifyContent::Start
                    })
                    .radius(12.0)
                    .background(if is_on {
                        p.get_primary()
                    } else {
                        p.get_secondary()
                    })
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
            Style::empty().font_size(14.0).color(p.get_foreground()),
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
        let row: NodeId = ctx.view(Style::empty().row().align(AlignItems::Center).gap(SPACE_SM));
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
        let mark: NodeId = ctx.reactive_text(
            Style::empty()
                .font_size(12.0)
                .font_weight(FontWeight::Bold)
                .color(p.get_primary_foreground()),
            move || {
                if checked.get() {
                    CHECKBOX_MARK.to_string()
                } else {
                    String::new()
                }
            },
        );
        ctx.append(indicator, mark);
        let handle: SceneHandle = ctx.get_scene().clone();
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            handle.bind_style(indicator, move || {
                let is_on: bool = checked.get();
                Style::empty()
                    .row()
                    .justify(JustifyContent::Center)
                    .align(AlignItems::Center)
                    .width(Length::Px(18.0))
                    .height(Length::Px(18.0))
                    .background(if is_on {
                        p.get_primary()
                    } else {
                        p.get_background()
                    })
                    .border(g.get_border_width(), p.get_primary())
                    .radius(g.get_radius_sm())
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
            Style::empty().font_size(14.0).color(p.get_foreground()),
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
        for (idx, opt) in options.into_iter().enumerate() {
            let pill: NodeId = ctx.view(Style::empty());
            let txt: NodeId = ctx.text(
                opt,
                Style::empty()
                    .font_size(13.0)
                    .font_weight(FontWeight::Medium)
                    .color(p.get_foreground()),
            );
            ctx.append(pill, txt);
            let handle: SceneHandle = ctx.get_scene().clone();
            let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
                handle.bind_style(pill, move || {
                    let active: bool = selected.get() == idx;
                    Style::empty()
                        .row()
                        .padding(Edges::symmetric(6.0, SPACE_MD))
                        .radius(g.get_radius_sm())
                        .background(if active {
                            p.get_card()
                        } else {
                            COLOR_TRANSPARENT
                        })
                        .cursor(CursorIcon::Pointer)
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
