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
                .background(p.get_card())
                .border(g.get_border_width(), p.get_border())
                .radius(g.get_radius_md())
                .focus_ring(Border::new(g.get_ring_width(), p.get_ring()))
                .color(p.get_foreground())
                .font_size(g.get_font_body()),
        );
        ctx.set_a11y(node, A11yProps::new(A11yRole::TextInput, String::new()));
        ctx.on_change(node, move |ev: &UiEvent| {
            if let UiEventKind::Change(val) = ev.get_kind() {
                let _: bool = sig.set_if_changed(val.clone());
            }
        });
        node
    }
}

/// Fluent range and step builders for `EuvNumberInput`.
impl EuvNumberInput {
    /// Configures inclusive `[min, max]` clamp bounds fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source number input.
    /// - `i64` - Inclusive minimum value.
    /// - `i64` - Inclusive maximum value.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated number input.
    #[must_use]
    pub fn range(mut self, min: i64, max: i64) -> Self {
        self.set_min(Some(min));
        self.set_max(Some(max));
        self
    }

    /// Configures the increment/decrement step size fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source number input.
    /// - `i64` - Positive step delta.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated number input.
    #[must_use]
    pub fn step(mut self, step: i64) -> Self {
        self.set_step(step.max(1));
        self
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
        let step_v: i64 = if self.get_step() > 0 {
            self.get_step()
        } else {
            1
        };
        let Self { min, max, .. } = self;
        let min_v: i64 = min.unwrap_or(i64::MIN / 2);
        let max_v: i64 = max.unwrap_or(i64::MAX / 2);
        let row: NodeId = ctx.view(Style::empty().row().align(AlignItems::Center).gap(SPACE_SM));
        let dec: NodeId = EuvButton::new(STEP_MINUS.to_string(), tokens)
            .outline()
            .sm()
            .on_click(move |_: &UiEvent| {
                let _: bool = sig.update(|v: &mut i64| *v = (*v - step_v).clamp(min_v, max_v));
            })
            .mount(ctx);
        let display: NodeId = ctx.view(
            Style::empty()
                .row()
                .width(Length::Px(72.0))
                .height(Length::Px(g.get_control_sm()))
                .justify(JustifyContent::Center)
                .align(AlignItems::Center)
                .background(p.get_card())
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
        let inc: NodeId = EuvButton::new(STEP_PLUS.to_string(), tokens)
            .outline()
            .sm()
            .on_click(move |_: &UiEvent| {
                let _: bool = sig.update(|v: &mut i64| *v = (*v + step_v).clamp(min_v, max_v));
            })
            .mount(ctx);
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
                    .padding(Edges::symmetric(SPACE_XS, SPACE_SM))
                    .radius(g.get_radius_sm())
                    .gap(SPACE_SM)
                    .cursor(CursorIcon::Pointer),
            );
            let ring: NodeId = ctx.view(Style::empty());
            let handle: SceneHandle = ctx.get_scene().clone();
            let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
                handle.bind_style(ring, move || {
                    let active: bool = selected.get() == idx;
                    Style::empty()
                        .width(Length::Px(18.0))
                        .height(Length::Px(18.0))
                        .radius(9.0)
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
                Style::empty()
                    .flex_grow(1.0)
                    .font_size(14.0)
                    .color(p.get_foreground()),
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
                .height(Length::Px(42.0))
                .align(AlignItems::Center)
                .gap(SPACE_SM)
                .stop_scroll_propagation(true)
                .stop_event_propagation(true),
        );
        let track: NodeId = ctx.canvas(
            Style::empty()
                .flex_grow(1.0)
                .height(Length::Px(42.0))
                .stop_scroll_propagation(true)
                .stop_event_propagation(true)
                .cursor(CursorIcon::Pointer),
            move |bounds: Rect, dl: &mut DisplayList| {
                let pct: f32 = ((value.get_untracked() - min_v) / span).clamp(0.0, 1.0);
                let track_h: f32 = 6.0;
                let track_y: f32 = bounds.get_y() + (bounds.get_height() - track_h) * 0.5;
                let pad_x: f32 = 9.0;
                let track_w: f32 = (bounds.get_width() - pad_x * 2.0).max(8.0);
                let track_x: f32 = bounds.get_x() + pad_x;
                dl.push(DrawCommand::FillRoundedRect {
                    rect: Rect::new(track_x, track_y, track_w, track_h),
                    color: p.get_secondary(),
                    radius: CornerRadius::all(3.0),
                    border: Border::default(),
                });
                let fill_w: f32 = (track_w * pct).max(0.0);
                if fill_w > 0.5 {
                    dl.push(DrawCommand::FillRoundedRect {
                        rect: Rect::new(track_x, track_y, fill_w, track_h),
                        color: p.get_primary(),
                        radius: CornerRadius::all(3.0),
                        border: Border::default(),
                    });
                }
                let thumb_cx: f32 = track_x + fill_w;
                let thumb_cy: f32 = bounds.get_y() + bounds.get_height() * 0.5;
                dl.push(DrawCommand::Circle {
                    center: Point::new(thumb_cx, thumb_cy),
                    radius: 9.0,
                    color: p.get_primary(),
                });
                dl.push(DrawCommand::Circle {
                    center: Point::new(thumb_cx, thumb_cy),
                    radius: 6.5,
                    color: p.get_background(),
                });
            },
        );
        let dragging: Rc<Cell<bool>> = Rc::new(Cell::new(false));
        let scene_down: SceneHandle = ctx.get_scene().clone();
        let drag_down: Rc<Cell<bool>> = dragging.clone();
        ctx.on_pointer_down(track, move |ev: &UiEvent| {
            drag_down.set(true);
            if let UiEventKind::PointerDown(pos) = ev.get_kind()
                && let Some(bounds) = scene_down.node_screen_bounds(track)
            {
                let pad_x: f32 = 9.0;
                let track_w: f32 = (bounds.get_width() - pad_x * 2.0).max(1.0);
                let frac: f32 =
                    ((pos.get_x() - (bounds.get_x() + pad_x)) / track_w).clamp(0.0, 1.0);
                let next: f32 = (min_v + frac * span).clamp(min_v, max_v);
                let _: bool = value.set_if_changed(next);
            }
        });
        let scene_move: SceneHandle = ctx.get_scene().clone();
        let drag_move: Rc<Cell<bool>> = dragging.clone();
        ctx.on_pointer_move(track, move |ev: &UiEvent| {
            if !drag_move.get() {
                return;
            }
            if let UiEventKind::PointerMove(pos) = ev.get_kind()
                && let Some(bounds) = scene_move.node_screen_bounds(track)
            {
                let pad_x: f32 = 9.0;
                let track_w: f32 = (bounds.get_width() - pad_x * 2.0).max(1.0);
                let frac: f32 =
                    ((pos.get_x() - (bounds.get_x() + pad_x)) / track_w).clamp(0.0, 1.0);
                let next: f32 = (min_v + frac * span).clamp(min_v, max_v);
                let _: bool = value.set_if_changed(next);
            }
        });
        let drag_up: Rc<Cell<bool>> = dragging.clone();
        ctx.on_pointer_up(track, move |_: &UiEvent| {
            drag_up.set(false);
        });
        let scene_watch: SceneHandle = ctx.get_scene().clone();
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            Effect::new(move || {
                let _: f32 = value.get();
                scene_watch.mark_dirty(track, DIRTY_PAINT);
            })
        });
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                tree.attach_scope(track, scope);
            },
            (),
        );
        let readout_box: NodeId = ctx.view(
            Style::empty()
                .row()
                .width(Length::Px(40.0))
                .justify(JustifyContent::End)
                .align(AlignItems::Center),
        );
        let readout: NodeId = ctx.reactive_text(
            Style::empty()
                .font_size(12.0)
                .font_weight(FontWeight::Medium)
                .color(p.get_muted_foreground()),
            move || format!("{:.1}", value.get()),
        );
        ctx.append(readout_box, readout);
        ctx.append(row, track);
        ctx.append(row, readout_box);
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
                opts_for_label.get(idx).cloned().unwrap_or_default()
            },
        );
        let chevron: NodeId = ctx.canvas(
            Style::empty()
                .width(Length::Px(12.0))
                .height(Length::Px(12.0)),
            move |bounds: Rect, dl: &mut DisplayList| {
                let cx: f32 = bounds.get_x() + bounds.get_width() * 0.5;
                let cy: f32 = bounds.get_y() + bounds.get_height() * 0.5;
                dl.push(DrawCommand::Line {
                    from: Point::new(cx - 4.0, cy - 2.0),
                    to: Point::new(cx, cy + 2.5),
                    color: p.get_muted_foreground(),
                    width: 1.6,
                });
                dl.push(DrawCommand::Line {
                    from: Point::new(cx, cy + 2.5),
                    to: Point::new(cx + 4.0, cy - 2.0),
                    color: p.get_muted_foreground(),
                    width: 1.6,
                });
            },
        );
        ctx.append(trigger, current_txt);
        ctx.append(trigger, chevron);
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
        let prev_open: Rc<Cell<bool>> = Rc::new(Cell::new(open.get_untracked()));
        let base_menu_style: Style = Style::empty()
            .column()
            .width(Length::Percent(100.0))
            .padding(Edges::all(SPACE_XS))
            .gap(2.0)
            .background(p.get_card())
            .border(g.get_border_width(), p.get_border())
            .radius(g.get_radius_md())
            .shadow(g.get_shadow_md())
            .z_index(Z_INDEX_FLOATING);
        handle.with_write(
            |tree: &mut SceneTree| {
                let init_disp: Display = if open.get_untracked() {
                    Display::Flex
                } else {
                    Display::None
                };
                tree.set_style(menu, base_menu_style.display(init_disp));
            },
            (),
        );
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            Effect::new(move || {
                let is_open: bool = open.get();
                let was_open: bool = prev_open.replace(is_open);
                if is_open == was_open {
                    return;
                }
                if is_open {
                    handle.start_transition(NodeTransition::slide_fade(
                        menu,
                        Point::new(0.0, -6.0),
                        Point::new(0.0, 0.0),
                        0.0,
                        1.0,
                        0.16,
                        false,
                    ));
                } else {
                    handle.start_transition(NodeTransition::slide_fade(
                        menu,
                        Point::new(0.0, 0.0),
                        Point::new(0.0, -4.0),
                        1.0,
                        0.0,
                        0.14,
                        true,
                    ));
                }
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
        let max_s: usize = self.get_max_stars().max(1);
        let row: NodeId = ctx.view(Style::empty().row().align(AlignItems::Center).gap(SPACE_XS));
        let mut badges: Vec<NodeId> = Vec::with_capacity(max_s);
        for star_idx in 1..=max_s {
            let cell: NodeId = ctx.view(
                Style::empty()
                    .row()
                    .padding(Edges::all(2.0))
                    .radius(4.0)
                    .cursor(CursorIcon::Pointer),
            );
            ctx.set_a11y(
                cell,
                A11yProps::new(A11yRole::Button, format!("Rate {star_idx} of {max_s}")),
            );
            let badge: NodeId = ctx.canvas(
                Style::empty()
                    .width(Length::Px(20.0))
                    .height(Length::Px(20.0)),
                move |bounds: Rect, dl: &mut DisplayList| {
                    let active: bool = value.get_untracked() >= star_idx;
                    let cx: f32 = bounds.get_x() + bounds.get_width() * 0.5;
                    let cy: f32 = bounds.get_y() + bounds.get_height() * 0.5;
                    let outer_r: f32 =
                        (bounds.get_width().min(bounds.get_height()) * 0.44).max(4.0);
                    let inner_r: f32 = outer_r * 0.44;
                    let col: Color = if active {
                        p.get_warning()
                    } else {
                        p.get_border()
                    };
                    let center: Point = Point::new(cx, cy);
                    for i in 0..10 {
                        let a0: f32 = (i as f32) * (TAU / 10.0) - FRAC_PI_2;
                        let a1: f32 = ((i + 1) as f32) * (TAU / 10.0) - FRAC_PI_2;
                        let r0: f32 = if i % 2 == 0 { outer_r } else { inner_r };
                        let r1: f32 = if i % 2 == 0 { inner_r } else { outer_r };
                        let (s0, c0): (f32, f32) = a0.sin_cos();
                        let (s1, c1): (f32, f32) = a1.sin_cos();
                        dl.push(DrawCommand::Triangle {
                            p0: center,
                            p1: Point::new(cx + c0 * r0, cy + s0 * r0),
                            p2: Point::new(cx + c1 * r1, cy + s1 * r1),
                            color: col,
                        });
                    }
                },
            );
            badges.push(badge);
            ctx.append(cell, badge);
            ctx.on_click(cell, move |_: &UiEvent| {
                let _: bool = value.set_if_changed(star_idx);
            });
            ctx.append(row, cell);
        }
        let handle: SceneHandle = ctx.get_scene().clone();
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            Effect::new(move || {
                let _: usize = value.get();
                handle.with_write(
                    |tree: &mut SceneTree| {
                        for b in badges.iter().copied() {
                            tree.mark_dirty(b, DIRTY_PAINT);
                        }
                    },
                    (),
                );
            })
        });
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                tree.attach_scope(row, scope);
            },
            (),
        );
        row
    }
}
