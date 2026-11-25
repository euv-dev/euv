use super::*;

/// `Mountable` implementation for `EuvLabel`.
impl Mountable for EuvLabel {
    /// Mounts the label text and optional required indicator into `ctx`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Label props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted label row handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let required: bool = self.get_required();
        let Self { text, .. } = self;
        let row: NodeId = ctx.view(Style::empty().row().gap(4.0));
        let lbl: NodeId = ctx.text(
            text,
            Style::empty()
                .font_size(13.0)
                .font_weight(FontWeight::Medium)
                .color(p.get_foreground()),
        );
        ctx.append(row, lbl);
        if required {
            let star: NodeId = ctx.text(
                STAR_FILLED.to_string(),
                Style::empty()
                    .font_size(13.0)
                    .font_weight(FontWeight::Bold)
                    .color(p.get_destructive()),
            );
            ctx.append(row, star);
        }
        row
    }
}

/// Builder methods for `EuvForm`.
impl EuvForm {
    /// Appends a mounted field node to the form.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source form.
    /// - `NodeId` - Field node handle.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated form.
    #[must_use]
    pub fn field(mut self, node: NodeId) -> Self {
        self.get_mut_fields().push(node);
        self
    }
}

/// `Mountable` implementation for `EuvForm`.
impl Mountable for EuvForm {
    /// Mounts the vertical form column into `ctx`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Form props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted form container handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let Self { fields, .. } = self;
        let col: NodeId = ctx.view(
            Style::empty()
                .column()
                .width(Length::Percent(100.0))
                .gap(SPACE_MD),
        );
        for f in fields.into_iter() {
            ctx.append(col, f);
        }
        col
    }
}

/// `Mountable` implementation for `EuvTextarea`.
impl Mountable for EuvTextarea {
    /// Mounts a multiline `Input` node bound to `value`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Textarea props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted textarea handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let sig: Signal<String> = self.get_value();
        let h: f32 = self.get_height();
        let initial: String = sig.get_untracked();
        let caret: usize = initial.len();
        let Self { placeholder, .. } = self;
        let state: InputNodeState =
            InputNodeState::new(initial, placeholder, caret, false, true, false);
        let node: NodeId = ctx.input(
            state,
            Style::empty()
                .width(Length::Percent(100.0))
                .height(Length::Px(h))
                .padding(Edges::all(SPACE_SM))
                .background(p.get_background())
                .border(g.get_border_width(), p.get_border())
                .radius(g.get_radius_md())
                .color(p.get_foreground())
                .font_size(g.get_font_body()),
        );
        ctx.on_change(node, move |ev: &UiEvent| {
            if let UiEventKind::Change(val) = ev.get_kind() {
                let _: bool = sig.set_if_changed(val.clone());
            }
        });
        node
    }
}

/// `Mountable` implementation for `EuvNumberInput`.
impl Mountable for EuvNumberInput {
    /// Mounts `-`, reactive numeric readout, and `+` stepper buttons.
    ///
    /// # Arguments
    ///
    /// - `Self` - Number input props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted stepper row handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let tokens: ThemeTokens = self.get_tokens();
        let p: ThemePalette = tokens.get_palette();
        let g: ThemeGeometry = tokens.get_geometry();
        let sig: Signal<i64> = self.get_value();
        let min_v: i64 = self.get_min();
        let max_v: i64 = self.get_max();
        let step_v: i64 = self.get_step();
        let row: NodeId = ctx.view(Style::empty().row().align(AlignItems::Center).gap(SPACE_SM));
        let mut dec_btn: EuvButton =
            EuvButton::new(STEP_MINUS.to_string(), tokens).on_click(move |_: &UiEvent| {
                let _: bool = sig.update(|v: &mut i64| *v = (*v - step_v).clamp(min_v, max_v));
            });
        dec_btn
            .set_variant(EuvButtonVariant::Outline)
            .set_size(EuvButtonSize::Small);
        let dec: NodeId = dec_btn.mount(ctx);
        let display: NodeId = ctx.view(
            Style::empty()
                .row()
                .width(Length::Px(72.0))
                .height(Length::Px(g.get_control_sm()))
                .justify(JustifyContent::Center)
                .align(AlignItems::Center)
                .background(p.get_background())
                .border(g.get_border_width(), p.get_border())
                .radius(g.get_radius_sm()),
        );
        let num_txt: NodeId = ctx.reactive_text(
            Style::empty()
                .font_size(14.0)
                .font_weight(FontWeight::Medium)
                .color(p.get_foreground()),
            move || sig.get().to_string(),
        );
        ctx.append(display, num_txt);
        let mut inc_btn: EuvButton =
            EuvButton::new(STEP_PLUS.to_string(), tokens).on_click(move |_: &UiEvent| {
                let _: bool = sig.update(|v: &mut i64| *v = (*v + step_v).clamp(min_v, max_v));
            });
        inc_btn
            .set_variant(EuvButtonVariant::Outline)
            .set_size(EuvButtonSize::Small);
        let inc: NodeId = inc_btn.mount(ctx);
        ctx.append(row, dec);
        ctx.append(row, display);
        ctx.append(row, inc);
        row
    }
}

/// `Mountable` implementation for `EuvRadioGroup`.
impl Mountable for EuvRadioGroup {
    /// Mounts clickable radio rings and labels bound reactively to `selected`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Radio group props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted radio group handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let selected: Signal<usize> = self.get_selected();
        let Self { options, .. } = self;
        let col: NodeId = ctx.view(Style::empty().column().gap(SPACE_SM));
        for (idx, opt) in options.into_iter().enumerate() {
            let row: NodeId = ctx.view(
                Style::empty()
                    .row()
                    .align(AlignItems::Center)
                    .gap(SPACE_SM)
                    .cursor(CursorIcon::Pointer),
            );
            let ring: NodeId = ctx.view(Style::empty());
            let handle: SceneHandle = ctx.get_scene().clone();
            let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
                handle.bind_style(ring, move || {
                    let active: bool = selected.get() == idx;
                    Style::empty()
                        .width(Length::Px(16.0))
                        .height(Length::Px(16.0))
                        .radius(8.0)
                        .border(g.get_border_width(), p.get_primary())
                        .background(if active {
                            p.get_primary()
                        } else {
                            p.get_background()
                        })
                })
            });
            ctx.get_scene().with_write(
                |tree: &mut SceneTree| {
                    tree.attach_scope(ring, scope);
                },
                (),
            );
            ctx.on_click(row, move |_: &UiEvent| {
                let _: bool = selected.set_if_changed(idx);
            });
            let txt: NodeId = ctx.text(
                opt,
                Style::empty().font_size(14.0).color(p.get_foreground()),
            );
            ctx.append(row, ring);
            ctx.append(row, txt);
            ctx.append(col, row);
        }
        col
    }
}

/// `Mountable` implementation for `EuvSlider`.
impl Mountable for EuvSlider {
    /// Mounts the slider track, reactive filled range, and quick step buttons.
    ///
    /// # Arguments
    ///
    /// - `Self` - Slider props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted slider handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let value: Signal<f32> = self.get_value();
        let min_v: f32 = self.get_min();
        let max_v: f32 = self.get_max();
        let span: f32 = (max_v - min_v).max(0.001);
        let row: NodeId = ctx.view(
            Style::empty()
                .row()
                .width(Length::Percent(100.0))
                .align(AlignItems::Center)
                .gap(SPACE_SM),
        );
        let track: NodeId = ctx.view(
            Style::empty()
                .row()
                .flex_grow(1.0)
                .height(Length::Px(8.0))
                .background(p.get_secondary())
                .radius(4.0)
                .overflow(Overflow::Hidden),
        );
        let fill: NodeId = ctx.view(Style::empty());
        ctx.append(track, fill);
        let handle: SceneHandle = ctx.get_scene().clone();
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            handle.bind_style(fill, move || {
                let pct: f32 = ((value.get() - min_v) / span).clamp(0.0, 1.0) * 100.0;
                Style::empty()
                    .width(Length::Percent(pct))
                    .height(Length::Percent(100.0))
                    .background(p.get_primary())
                    .radius(4.0)
            })
        });
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                tree.attach_scope(fill, scope);
            },
            (),
        );
        let step_amt: f32 = span * 0.1;
        ctx.on_click(track, move |_: &UiEvent| {
            let _: bool = value.update(|v: &mut f32| {
                let next: f32 = *v + step_amt;
                *v = if next > max_v { min_v } else { next };
            });
        });
        let readout: NodeId = ctx.reactive_text(
            Style::empty()
                .font_size(12.0)
                .color(p.get_muted_foreground()),
            move || format!("{:.1}", value.get()),
        );
        ctx.append(row, track);
        ctx.append(row, readout);
        row
    }
}

/// `Mountable` implementation for `EuvSelect`.
impl Mountable for EuvSelect {
    /// Mounts the select trigger and collapsible option list bound reactively to `open`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Select props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted select container handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let selected: Signal<usize> = self.get_selected();
        let open: Signal<bool> = self.get_open();
        let Self { options, .. } = self;
        let opts_for_label: Vec<String> = options.clone();
        let wrapper: NodeId = ctx.view(
            Style::empty()
                .column()
                .width(Length::Percent(100.0))
                .gap(SPACE_XS),
        );
        let trigger: NodeId = ctx.view(
            Style::empty()
                .row()
                .width(Length::Percent(100.0))
                .height(Length::Px(g.get_control_md()))
                .padding(Edges::symmetric(SPACE_SM, SPACE_MD))
                .justify(JustifyContent::SpaceBetween)
                .align(AlignItems::Center)
                .background(p.get_background())
                .border(g.get_border_width(), p.get_border())
                .radius(g.get_radius_md())
                .cursor(CursorIcon::Pointer),
        );
        let current_txt: NodeId = ctx.reactive_text(
            Style::empty().font_size(14.0).color(p.get_foreground()),
            move || {
                let idx: usize = selected.get();
                let label: &str = opts_for_label.get(idx).map(String::as_str).unwrap_or("");
                format!("{label}{SELECT_ARROW}")
            },
        );
        ctx.append(trigger, current_txt);
        ctx.on_click(trigger, move |_: &UiEvent| {
            let _: bool = open.update(|v: &mut bool| *v = !*v);
        });
        let menu: NodeId = ctx.view(Style::empty());
        for (idx, opt) in options.into_iter().enumerate() {
            let item: NodeId = ctx.view(
                Style::empty()
                    .row()
                    .width(Length::Percent(100.0))
                    .padding(Edges::symmetric(SPACE_SM, SPACE_MD))
                    .cursor(CursorIcon::Pointer),
            );
            let txt: NodeId = ctx.text(
                opt,
                Style::empty().font_size(13.0).color(p.get_foreground()),
            );
            ctx.append(item, txt);
            ctx.on_click(item, move |_: &UiEvent| {
                let _: bool = selected.set_if_changed(idx);
                let _: bool = open.set_if_changed(false);
            });
            ctx.append(menu, item);
        }
        let handle: SceneHandle = ctx.get_scene().clone();
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            handle.bind_style(menu, move || {
                let disp: Display = if open.get() {
                    Display::Flex
                } else {
                    Display::None
                };
                Style::empty()
                    .display(disp)
                    .column()
                    .width(Length::Percent(100.0))
                    .background(p.get_card())
                    .border(g.get_border_width(), p.get_border())
                    .radius(g.get_radius_md())
                    .z_index(Z_INDEX_FLOATING)
            })
        });
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                tree.attach_scope(menu, scope);
            },
            (),
        );
        ctx.append(wrapper, trigger);
        ctx.append(wrapper, menu);
        wrapper
    }
}

/// `Mountable` implementation for `EuvRating`.
impl Mountable for EuvRating {
    /// Mounts interactive star buttons updating `value` reactively.
    ///
    /// # Arguments
    ///
    /// - `Self` - Rating props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted rating row handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let value: Signal<usize> = self.get_value();
        let max_s: usize = self.get_max_stars();
        let row: NodeId = ctx.view(Style::empty().row().gap(SPACE_XS));
        for star_idx in 1..=max_s {
            let cell: NodeId = ctx.view(
                Style::empty()
                    .row()
                    .padding(Edges::all(SPACE_XS))
                    .cursor(CursorIcon::Pointer),
            );
            let txt: NodeId = ctx.reactive_text(
                Style::empty()
                    .font_size(16.0)
                    .font_weight(FontWeight::Bold)
                    .color(p.get_warning()),
                move || {
                    if value.get() >= star_idx {
                        STAR_FILLED.to_string()
                    } else {
                        STAR_EMPTY.to_string()
                    }
                },
            );
            ctx.append(cell, txt);
            ctx.on_click(cell, move |_: &UiEvent| {
                let _: bool = value.set_if_changed(star_idx);
            });
            ctx.append(row, cell);
        }
        row
    }
}
