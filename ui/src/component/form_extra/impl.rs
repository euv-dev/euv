use super::*;

/// Declarative `#[component]` implementation for `EuvLabel`.
#[component]
/// Inherent implementation of [`EuvLabel`].
impl EuvLabel {
    /// Renders `EuvLabel` into the ambient framework-managed `Context`.
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
        let required: bool = self.get_required();
        let Self { text, .. } = self.clone();
        view! {
            row {
                gap: 4.0
                text {
                    font_size: 13.0
                    font_weight: FontWeight::Medium
                    color: p.get_foreground()
                    { text }
                }
                if required {
                    text {
                        font_size: 13.0
                        font_weight: FontWeight::Bold
                        color: p.get_destructive()
                        { STAR_FILLED }
                    }
                }
            }
        }
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

/// Declarative `#[component]` implementation for `EuvForm`.
#[component]
/// Inherent implementation of [`EuvForm`].
impl EuvForm {
    /// Renders `EuvForm` into the ambient framework-managed `Context`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component descriptor reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted root scene node handle.
    fn render(&self) -> NodeId {
        let Self { fields, .. } = self.clone();
        view! {
            column {
                width: Length::Percent(100.0)
                gap: SPACE_MD
                ..fields
            }
        }
    }
}

/// Declarative `#[component]` implementation for `EuvTextarea`.
#[component]
/// Inherent implementation of [`EuvTextarea`].
impl EuvTextarea {
    /// Renders `EuvTextarea` into the ambient framework-managed `Context`.
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
        let sig: Signal<String> = self.get_value();
        let h: f32 = self.get_height();
        let initial: String = sig.get_untracked();
        let caret: usize = initial.len();
        let Self { placeholder, .. } = self.clone();
        view! {
            input {
                value: initial
                placeholder: placeholder
                caret: caret
                multiline: true
                width: Length::Percent(100.0)
                height: Length::Px(h)
                padding: Edges::all(SPACE_SM)
                background: p.get_card()
                border: (g.get_border_width(), p.get_border())
                radius: g.get_radius_md()
                focus_ring: Border::new(g.get_ring_width(), p.get_ring())
                color: p.get_foreground()
                font_size: g.get_font_body()
                role: A11yRole::TextInput
                on_change: move |ev: &UiEvent| {
                    if let UiEventKind::Change(val) = ev.get_kind() {
                        let _: bool = sig.set_if_changed(val.clone());
                    }
                }
            }
        }
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

/// Declarative `#[component]` implementation for `EuvNumberInput`.
#[component]
/// Inherent implementation of [`EuvNumberInput`].
impl EuvNumberInput {
    /// Renders `EuvNumberInput` into the ambient framework-managed `Context`.
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
        let sig: Signal<i64> = self.get_value();
        let step_v: i64 = if self.get_step() > 0 {
            self.get_step()
        } else {
            1
        };
        let Self { min, max, .. } = self.clone();
        let min_v: i64 = min.unwrap_or(i64::MIN / 2);
        let max_v: i64 = max.unwrap_or(i64::MAX / 2);
        view! {
            row {
                align: AlignItems::Center
                gap: SPACE_SM
                EuvButton {
                    label: STEP_MINUS.to_string()
                    tokens: tokens
                    outline()
                    sm()
                    on_click: move |_: &UiEvent| {
                        let _: bool = sig.update(|v: &mut i64| *v = (*v - step_v).clamp(min_v, max_v));
                    }
                }
                row {
                    width: Length::Px(72.0)
                    height: Length::Px(g.get_control_sm())
                    justify: JustifyContent::Center
                    align: AlignItems::Center
                    background: p.get_card()
                    border: (g.get_border_width(), p.get_border())
                    radius: g.get_radius_sm()
                    text {
                        font_size: 14.0
                        font_weight: FontWeight::Medium
                        color: p.get_foreground()
                        text_align: TextAlign::Center
                        { move || sig.get().to_string() }
                    }
                }
                EuvButton {
                    label: STEP_PLUS.to_string()
                    tokens: tokens
                    outline()
                    sm()
                    on_click: move |_: &UiEvent| {
                        let _: bool = sig.update(|v: &mut i64| *v = (*v + step_v).clamp(min_v, max_v));
                    }
                }
            }
        }
    }
}

/// Declarative `#[component]` implementation for `EuvRadioGroup`.
#[component]
/// Inherent implementation of [`EuvRadioGroup`].
impl EuvRadioGroup {
    /// Renders `EuvRadioGroup` into the ambient framework-managed `Context`.
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
            column {
                gap: SPACE_SM
                for (idx, opt) in options.into_iter().enumerate() {
                    row {
                        align: AlignItems::Center
                        padding: Edges::symmetric(SPACE_XS, SPACE_SM)
                        radius: g.get_radius_sm()
                        gap: SPACE_SM
                        cursor: CursorIcon::Pointer
                        on_click: move |_: &UiEvent| {
                            let _: bool = selected.set_if_changed(idx);
                        }
                        view {
                            bind_style: move || {
                                let active: bool = selected.get() == idx;
                                Style::default()
                                .width(Length::Px(18.0))
                                .height(Length::Px(18.0))
                                .radius(9.0)
                                .border(g.get_border_width(), p.get_primary())
                                .background(if active {
                                    p.get_primary()
                                    } else {
                                        p.get_background()
                                })
                            }
                        }
                        text {
                            flex_grow: 1.0
                            font_size: 14.0
                            color: p.get_foreground()
                            { opt }
                        }
                    }
                }
            }
        }
    }
}

/// Declarative `#[component]` implementation for `EuvSlider`.
#[component]
/// Inherent implementation of [`EuvSlider`].
impl EuvSlider {
    /// Renders `EuvSlider` into the ambient framework-managed `Context`.
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
        let value: Signal<f32> = self.get_value();
        let min_v: f32 = self.get_min();
        let max_v: f32 = self.get_max();
        let span: f32 = (max_v - min_v).max(0.001);
        let mut track: NodeId = NodeId::default();
        let row: NodeId = view! {
            row {
                width: Length::Percent(100.0)
                height: Length::Px(42.0)
                align: AlignItems::Center
                gap: SPACE_SM
                stop_scroll_propagation: true
                stop_event_propagation: true
                canvas {
                    node_ref: track
                    flex_grow: 1.0
                    height: Length::Px(42.0)
                    stop_scroll_propagation: true
                    stop_event_propagation: true
                    cursor: CursorIcon::Pointer
                    painter: move |bounds: Rect, dl: &mut DisplayList| {
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
                    }
                }
                row {
                    width: Length::Px(40.0)
                    justify: JustifyContent::End
                    align: AlignItems::Center
                    text {
                        font_size: 12.0
                        font_weight: FontWeight::Medium
                        color: p.get_muted_foreground()
                        { move || format!("{:.1}", value.get()) }
                    }
                }
            }
        };
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
        row
    }
}

/// Declarative `#[component]` implementation for `EuvSelect`.
#[component]
/// Inherent implementation of [`EuvSelect`].
impl EuvSelect {
    /// Renders `EuvSelect` into the ambient framework-managed `Context`.
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
        let selected: Signal<usize> = self.get_selected();
        let open: Signal<bool> = self.get_open();
        let Self { options, .. } = self.clone();
        let opts_for_label: Vec<String> = options.clone();
        let init_open: bool = open.get_untracked();
        let mut menu: NodeId = NodeId::default();
        let chevron_color: Color = p.get_muted_foreground();
        let on_toggle = move |_: &UiEvent| {
            let _: bool = open.update(|v: &mut bool| *v = !*v);
        };
        let selected_text = move || -> String {
            let idx: usize = selected.get();
            opts_for_label.get(idx).cloned().unwrap_or_default()
        };
        let paint_chevron = move |bounds: Rect, dl: &mut DisplayList| {
            let cx: f32 = bounds.get_x() + bounds.get_width() * 0.5;
            let cy: f32 = bounds.get_y() + bounds.get_height() * 0.5;
            dl.push(DrawCommand::Line {
                from: Point::new(cx - 4.0, cy - 2.0),
                to: Point::new(cx, cy + 2.5),
                color: chevron_color,
                width: 1.6,
            });
            dl.push(DrawCommand::Line {
                from: Point::new(cx, cy + 2.5),
                to: Point::new(cx + 4.0, cy - 2.0),
                color: chevron_color,
                width: 1.6,
            });
        };
        let option_rows: Vec<NodeId> = options
            .into_iter()
            .enumerate()
            .map(|(idx, opt): (usize, String)| {
                let on_pick = move |_: &UiEvent| {
                    let _: bool = selected.set_if_changed(idx);
                    let _: bool = open.set_if_changed(false);
                };
                view! {
                    row {
                        width: Length::Percent(100.0)
                        height: Length::Px(g.get_control_sm())
                        padding: Edges::symmetric(0.0, SPACE_MD)
                        align: AlignItems::Center
                        radius: g.get_radius_sm()
                        overflow: Overflow::Hidden
                        cursor: CursorIcon::Pointer
                        on_click: on_pick
                        text {
                            width: Length::Percent(100.0)
                            font_size: 13.0
                            color: p.get_foreground()
                            text_overflow: TextOverflow::Ellipsis
                            { opt }
                        }
                    }
                }
            })
            .collect();
        let wrapper: NodeId = view! {
            column {
                width: Length::Percent(100.0)
                gap: SPACE_XS
                row {
                    width: Length::Percent(100.0)
                    height: Length::Px(g.get_control_md())
                    padding: Edges::symmetric(0.0, SPACE_MD)
                    justify: JustifyContent::SpaceBetween
                    align: AlignItems::Center
                    gap: SPACE_SM
                    background: p.get_background()
                    border: (g.get_border_width(), p.get_border())
                    radius: g.get_radius_md()
                    overflow: Overflow::Hidden
                    cursor: CursorIcon::Pointer
                    on_click: on_toggle
                    text {
                        flex_grow: 1.0
                        flex_shrink: 1.0
                        min_width: Length::Px(0.0)
                        font_size: 14.0
                        color: p.get_foreground()
                        text_overflow: TextOverflow::Ellipsis
                        { selected_text }
                    }
                    canvas {
                        width: Length::Px(12.0)
                        height: Length::Px(12.0)
                        flex_shrink: 0.0
                        painter: paint_chevron
                    }
                }
                column {
                    node_ref: menu
                    display: if init_open { Display::Flex } else { Display::None }
                    width: Length::Percent(100.0)
                    padding: Edges::all(SPACE_XS)
                    gap: 2.0
                    background: p.get_card()
                    border: (g.get_border_width(), p.get_border())
                    radius: g.get_radius_md()
                    shadow: g.get_shadow_md()
                    overflow: Overflow::Hidden
                    z_index: Z_INDEX_FLOATING
                    ..option_rows
                }
            }
        };
        let handle: SceneHandle = ctx.get_scene().clone();
        let prev_open: Rc<Cell<bool>> = Rc::new(Cell::new(init_open));
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            Effect::new(move || {
                let is_open: bool = open.get();
                let was_open: bool = prev_open.replace(is_open);
                if is_open == was_open {
                    return;
                }
                if is_open {
                    handle.start_transition(
                        NodeTransition::slide_fade(
                            menu,
                            Point::new(0.0, -8.0),
                            Point::new(0.0, 0.0),
                            0.0,
                            1.0,
                            0.24,
                            false,
                        )
                        .with_scale(0.95, 1.0)
                        .with_easing(Easing::SpringSnappy),
                    );
                } else {
                    handle.start_transition(
                        NodeTransition::slide_fade(
                            menu,
                            Point::new(0.0, 0.0),
                            Point::new(0.0, -5.0),
                            1.0,
                            0.0,
                            0.14,
                            true,
                        )
                        .with_scale(1.0, 0.96)
                        .with_easing(Easing::EaseOutCubic),
                    );
                }
            })
        });
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                tree.attach_scope(menu, scope);
            },
            (),
        );
        wrapper
    }
}

/// Declarative `#[component]` implementation for `EuvRating`.
#[component]
/// Inherent implementation of [`EuvRating`].
impl EuvRating {
    /// Renders `EuvRating` into the ambient framework-managed `Context`.
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
        let value: Signal<usize> = self.get_value();
        let max_s: usize = self.get_max_stars().max(1);
        let mut badges: Vec<NodeId> = Vec::with_capacity(max_s);
        let row: NodeId = view! {
            row {
                align: AlignItems::Center
                gap: SPACE_XS
                for star_idx in 1..=max_s {
                    row {
                        padding: Edges::all(2.0)
                        radius: 4.0
                        cursor: CursorIcon::Pointer
                        role: A11yRole::Button
                        a11y_label: format!("Rate {star_idx} of {max_s}")
                        on_click: move |_: &UiEvent| {
                            let _: bool = value.set_if_changed(star_idx);
                        }
                        canvas {
                            width: Length::Px(20.0)
                            height: Length::Px(20.0)
                            painter: move |bounds: Rect, dl: &mut DisplayList| {
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
                            }
                        }
                    }
                }
            }
        };
        ctx.get_scene().with_read(
            |tree: &SceneTree| {
                if let Some(root_node) = tree.get_node(row) {
                    for cell_id in root_node.get_children().iter().copied() {
                        if let Some(cell_node) = tree.get_node(cell_id)
                            && let Some(b) = cell_node.get_children().first().copied()
                        {
                            badges.push(b);
                        }
                    }
                }
            },
            (),
        );
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

/// Fluent swatch preset builder for `EuvColorPicker`.
impl EuvColorPicker {
    /// Overrides the clickable color swatch presets fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source color picker.
    /// - `Vec<Color>` - Custom swatch palette colors.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated color picker.
    #[must_use]
    pub fn presets(mut self, presets: Vec<Color>) -> Self {
        self.set_presets(presets);
        self
    }
}

/// Declarative `#[component]` implementation for `EuvColorPicker`.
#[component]
/// Inherent implementation of [`EuvColorPicker`].
impl EuvColorPicker {
    /// Renders `EuvColorPicker` into the ambient framework-managed `Context`.
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
        let sig: Signal<Color> = self.get_value();
        let Self { presets, .. } = self.clone();
        let swatches: Vec<Color> = if presets.is_empty() {
            COLOR_PICKER_DEFAULTS
                .iter()
                .copied()
                .map(Color::hex)
                .collect()
        } else {
            presets
        };
        view! {
            column {
                width: Length::Percent(100.0)
                gap: SPACE_SM
                row {
                    align: AlignItems::Center
                    gap: SPACE_SM
                    view {
                        bind_style: move || {
                            Style::default()
                            .width(Length::Px(24.0))
                            .height(Length::Px(24.0))
                            .radius(g.get_radius_sm())
                            .background(sig.get())
                            .border(g.get_border_width(), p.get_border())
                        }
                    }
                    text {
                        font_family: FontFamily::Monospace
                        font_size: 13.0
                        font_weight: FontWeight::Medium
                        color: p.get_foreground()
                        { move || {
                            let c: Color = sig.get();
                            format!("#{:02X}{:02X}{:02X}", c.get_r(), c.get_g(), c.get_b())
                        } }
                    }
                }
                row {
                    flex_wrap: FlexWrap::Wrap
                    width: Length::Percent(100.0)
                    gap: SPACE_XS
                    for swatch_color in swatches.into_iter() {
                        view {
                            bind_style: move || {
                                let active: bool = sig.get() == swatch_color;
                                Style::default()
                                .width(Length::Px(26.0))
                                .height(Length::Px(26.0))
                                .radius(g.get_radius_sm())
                                .background(swatch_color)
                                .border(
                                    if active { 2.0 } else { g.get_border_width() },
                                    if active {
                                        p.get_foreground()
                                    } else {
                                        p.get_border()
                                    },
                                )
                                .cursor(CursorIcon::Pointer)
                            }
                            on_click: move |_: &UiEvent| {
                                let _: bool = sig.set_if_changed(swatch_color);
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Declarative `#[component]` implementation for `EuvTimePicker`.
#[component]
/// Inherent implementation of [`EuvTimePicker`].
impl EuvTimePicker {
    /// Renders `EuvTimePicker` into the ambient framework-managed `Context`.
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
        let sig: Signal<(u8, u8)> = self.get_value();
        let time_readout_text = move || -> String {
            let (hh, mm): (u8, u8) = sig.get();
            format!("{:02}:{:02}", hh % 24, mm % 60)
        };
        view! {
            column {
                width: Length::Percent(100.0)
                gap: SPACE_SM
                row {
                    flex_wrap: FlexWrap::Wrap
                    align: AlignItems::Center
                    gap: SPACE_XS
                    text {
                        font_size: 11.0
                        color: p.get_muted_foreground()
                        { TIME_HOUR_LABEL }
                    }
                    EuvButton {
                        label: STEP_MINUS.to_string()
                        tokens: tokens
                        outline()
                        sm()
                        on_click: move |_: &UiEvent| {
                            let _: bool = sig.update(|v: &mut (u8, u8)| {
                                v.0 = if v.0 == 0 { 23 } else { v.0 - 1 };
                            });
                        }
                    }
                    EuvButton {
                        label: STEP_PLUS.to_string()
                        tokens: tokens
                        outline()
                        sm()
                        on_click: move |_: &UiEvent| {
                            let _: bool = sig.update(|v: &mut (u8, u8)| {
                                v.0 = (v.0 + 1) % 24;
                            });
                        }
                    }
                    row {
                        height: Length::Px(g.get_control_sm())
                        min_width: Length::Px(76.0)
                        flex_shrink: 0.0
                        padding: Edges::symmetric(0.0, SPACE_MD)
                        justify: JustifyContent::Center
                        align: AlignItems::Center
                        background: p.get_card()
                        border: (g.get_border_width(), p.get_border())
                        radius: g.get_radius_sm()
                        text {
                            font_family: FontFamily::Monospace
                            font_size: 14.0
                            font_weight: FontWeight::SemiBold
                            color: p.get_foreground()
                            text_align: TextAlign::Center
                            text_overflow: TextOverflow::Clip
                            { time_readout_text }
                        }
                    }
                    EuvButton {
                        label: STEP_MINUS.to_string()
                        tokens: tokens
                        outline()
                        sm()
                        on_click: move |_: &UiEvent| {
                            let _: bool = sig.update(|v: &mut (u8, u8)| {
                                v.1 = (v.1 + 55) % 60;
                            });
                        }
                    }
                    EuvButton {
                        label: STEP_PLUS.to_string()
                        tokens: tokens
                        outline()
                        sm()
                        on_click: move |_: &UiEvent| {
                            let _: bool = sig.update(|v: &mut (u8, u8)| {
                                v.1 = (v.1 + 5) % 60;
                            });
                        }
                    }
                    text {
                        font_size: 11.0
                        color: p.get_muted_foreground()
                        { TIME_MIN_LABEL }
                    }
                }
                row {
                    flex_wrap: FlexWrap::Wrap
                    width: Length::Percent(100.0)
                    gap: SPACE_XS
                    for (ph, pm) in TIME_PRESETS.iter().copied() {
                        EuvButton {
                            label: format!("{ph:02}:{pm:02}")
                            tokens: tokens
                            secondary()
                            sm()
                            on_click: move |_: &UiEvent| {
                                let _: bool = sig.set_if_changed((ph, pm));
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Fluent placeholder builder for `EuvTagInput`.
impl EuvTagInput {
    /// Sets the tag input field placeholder text fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source tag input.
    /// - `S` - Placeholder string.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated tag input.
    #[must_use]
    pub fn placeholder<S>(mut self, placeholder: S) -> Self
    where
        S: Into<String>,
    {
        self.set_placeholder(placeholder.into());
        self
    }
}

/// Declarative `#[component]` implementation for `EuvTagInput`.
#[component]
/// Inherent implementation of [`EuvTagInput`].
impl EuvTagInput {
    /// Renders `EuvTagInput` into the ambient framework-managed `Context`.
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
        let tags_sig: Signal<Vec<String>> = self.get_tags();
        let Self { placeholder, .. } = self.clone();
        let resolved_ph: String = if placeholder.is_empty() {
            TAG_DEFAULT_PLACEHOLDER.to_string()
        } else {
            placeholder
        };
        let draft: Signal<String> = Signal::new(String::new());
        view! {
            column {
                width: Length::Percent(100.0)
                gap: SPACE_SM
                row {
                    flex_wrap: FlexWrap::Wrap
                    width: Length::Percent(100.0)
                    gap: SPACE_XS
                    bind_children: move || {
                        tags_sig
                        .get()
                        .into_iter()
                        .enumerate()
                        .map(|(idx, tag): (usize, String)| {
                            view! {
                                row {
                                    justify: JustifyContent::Center
                                    align: AlignItems::Center
                                    padding: Edges::symmetric(4.0, SPACE_SM)
                                    gap: 6.0
                                    background: p.get_secondary()
                                    border: (g.get_border_width(), p.get_border())
                                    radius: g.get_radius_sm()
                                    cursor: CursorIcon::Pointer
                                    on_click: move |_: &UiEvent| {
                                        let _: bool = tags_sig.update(|list: &mut Vec<String>| {
                                            if idx < list.len() {
                                                let _: String = list.remove(idx);
                                            }
                                        });
                                    }
                                    text {
                                        font_size: 12.0
                                        font_weight: FontWeight::Medium
                                        color: p.get_secondary_foreground()
                                        text_align: TextAlign::Center
                                        { format!("{tag}  x") }
                                    }
                                }
                            }
                        })
                        .collect()
                    }
                }
                row {
                    width: Length::Percent(100.0)
                    align: AlignItems::Center
                    gap: SPACE_XS
                    column {
                        flex_grow: 1.0
                        min_width: Length::Px(80.0)
                        EuvInput {
                            value: draft
                            placeholder: resolved_ph
                            tokens: tokens
                        }
                    }
                    EuvButton {
                        label: TAG_ADD_LABEL.to_string()
                        tokens: tokens
                        outline()
                        on_click: move |_: &UiEvent| {
                            let raw: String = draft.get_untracked().trim().to_string();
                            if !raw.is_empty() {
                                let _: bool = tags_sig.update(|list: &mut Vec<String>| {
                                    if !list.iter().any(|existing: &String| existing == &raw) {
                                        list.push(raw.clone());
                                    }
                                });
                                let _: bool = draft.set_if_changed(String::new());
                            }
                        }
                    }
                }
            }
        }
    }
}
