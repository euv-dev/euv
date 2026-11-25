use super::*;

/// `Mountable` implementation for `EuvAlertDialog`.
impl Mountable for EuvAlertDialog {
    /// Mounts the confirmation dialog portal with responsive width and Cancel/Confirm controls.
    ///
    /// # Arguments
    ///
    /// - `Self` - Alert dialog props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted alert dialog portal handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
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
        let init_open: bool = open.get_untracked();
        let scrim: NodeId = ctx.portal(
            Style::empty()
                .display(if init_open {
                    Display::Flex
                } else {
                    Display::None
                })
                .position(Position::Fixed)
                .inset(Some(0.0), Some(0.0), Some(0.0), Some(0.0))
                .width(Length::Percent(100.0))
                .height(Length::Percent(100.0))
                .justify(JustifyContent::Center)
                .align(AlignItems::Center)
                .background(p.get_overlay())
                .stop_scroll_propagation(true)
                .z_index(Z_INDEX_MODAL),
        );
        let card: NodeId = ctx.view(
            Style::empty()
                .column()
                .width(Length::Percent(92.0))
                .max_width(Length::Px(440.0))
                .padding(Edges::all(SPACE_XL))
                .gap(SPACE_MD)
                .background(p.get_card())
                .border(g.get_border_width(), p.get_border())
                .radius(g.get_radius_lg())
                .overflow(Overflow::Hidden)
                .shadow(Shadow::new(0.0, 10.0, 18.0, 0.0, Color::rgba(0, 0, 0, 28))),
        );
        let h_node: NodeId = ctx.text(
            title,
            Style::empty()
                .font_size(g.get_font_heading())
                .font_weight(FontWeight::SemiBold)
                .color(p.get_card_foreground()),
        );
        let d_node: NodeId = ctx.text(
            description,
            Style::empty()
                .font_size(13.0)
                .color(p.get_muted_foreground()),
        );
        let actions: NodeId = ctx.view(
            Style::empty()
                .row()
                .wrap(FlexWrap::Wrap)
                .width(Length::Percent(100.0))
                .justify(JustifyContent::End)
                .gap(SPACE_SM),
        );
        let mut cancel_btn: EuvButton =
            EuvButton::new(cancel_label, tokens).on_click(move |_: &UiEvent| {
                let _: bool = confirmed.set_if_changed(false);
                let _: bool = open.set_if_changed(false);
            });
        cancel_btn.set_variant(EuvButtonVariant::Outline);
        let mut confirm_btn: EuvButton =
            EuvButton::new(confirm_label, tokens).on_click(move |_: &UiEvent| {
                let _: bool = confirmed.set_if_changed(true);
                let _: bool = open.set_if_changed(false);
            });
        confirm_btn.set_variant(EuvButtonVariant::Destructive);
        let cancel_node: NodeId = cancel_btn.mount(ctx);
        let confirm_node: NodeId = confirm_btn.mount(ctx);
        ctx.append(actions, cancel_node);
        ctx.append(actions, confirm_node);
        ctx.append(card, h_node);
        ctx.append(card, d_node);
        ctx.append(card, actions);
        ctx.append(scrim, card);
        ctx.on_click(scrim, move |ev: &UiEvent| {
            if ev.get_target() == scrim {
                let _: bool = confirmed.set_if_changed(false);
                let _: bool = open.set_if_changed(false);
            }
        });
        let handle: SceneHandle = ctx.get_scene().clone();
        let prev_open: Rc<Cell<bool>> = Rc::new(Cell::new(init_open));
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            let _: Effect = EuvOverlayStack::bind_signal(open, "EuvAlertDialog", Z_INDEX_MODAL);
            Effect::new(move || {
                let is_open: bool = open.get();
                let was_open: bool = prev_open.replace(is_open);
                if is_open == was_open {
                    return;
                }
                if is_open {
                    handle.with_write(
                        |tree: &mut SceneTree| {
                            if let Some(n) = tree.get_node_mut(scrim) {
                                let mut st: Style = n.get_style();
                                st.get_mut_paint().set_pointer_events(PointerEvents::Auto);
                                n.set_style(st);
                            }
                        },
                        (),
                    );
                    handle.start_transition(NodeTransition::fade(scrim, 0.0, 1.0, 0.20, false));
                    handle.start_transition(NodeTransition::modal_enter(card));
                } else {
                    handle.with_write(
                        |tree: &mut SceneTree| {
                            if let Some(n) = tree.get_node_mut(scrim) {
                                let mut st: Style = n.get_style();
                                st.get_mut_paint().set_pointer_events(PointerEvents::None);
                                n.set_style(st);
                            }
                        },
                        (),
                    );
                    handle.start_transition(NodeTransition::modal_exit(card));
                    handle.start_transition(NodeTransition::fade(scrim, 1.0, 0.0, 0.18, true));
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

/// `Mountable` implementation for `EuvCollapsible`.
impl Mountable for EuvCollapsible {
    /// Mounts the disclosure trigger header and reactively toggled content container.
    ///
    /// # Arguments
    ///
    /// - `Self` - Collapsible props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted collapsible container handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let open: Signal<bool> = self.get_open();
        let content: NodeId = self.get_content();
        let Self { title, .. } = self;
        let root: NodeId = ctx.view(
            Style::empty()
                .column()
                .width(Length::Percent(100.0))
                .gap(SPACE_XS)
                .background(p.get_card())
                .border(g.get_border_width(), p.get_border())
                .radius(g.get_radius_md())
                .overflow(Overflow::Hidden),
        );
        let header: NodeId = ctx.view(
            Style::empty()
                .row()
                .width(Length::Percent(100.0))
                .padding(Edges::symmetric(SPACE_SM, SPACE_MD))
                .justify(JustifyContent::SpaceBetween)
                .align(AlignItems::Center)
                .cursor(CursorIcon::Pointer),
        );
        let title_node: NodeId = ctx.text(
            title,
            Style::empty()
                .font_size(13.0)
                .font_weight(FontWeight::SemiBold)
                .color(p.get_foreground()),
        );
        let state_badge: NodeId = ctx.reactive_text(
            Style::empty()
                .font_size(12.0)
                .font_weight(FontWeight::Medium)
                .color(p.get_muted_foreground()),
            move || {
                if open.get() {
                    COLLAPSIBLE_HIDE_LABEL.to_string()
                } else {
                    COLLAPSIBLE_SHOW_LABEL.to_string()
                }
            },
        );
        ctx.append(header, title_node);
        ctx.append(header, state_badge);
        ctx.on_click(header, move |_: &UiEvent| {
            let _: bool = open.update(|v: &mut bool| *v = !*v);
        });
        let body: NodeId = ctx.view(Style::empty());
        ctx.append(body, content);
        let handle: SceneHandle = ctx.get_scene().clone();
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            handle.bind_style(body, move || {
                let disp: Display = if open.get() {
                    Display::Flex
                } else {
                    Display::None
                };
                Style::empty()
                    .column()
                    .display(disp)
                    .width(Length::Percent(100.0))
                    .padding(Edges::all(SPACE_MD))
                    .border(g.get_border_width(), p.get_border())
            })
        });
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                tree.attach_scope(body, scope);
            },
            (),
        );
        ctx.append(root, header);
        ctx.append(root, body);
        root
    }
}

/// `Mountable` implementation for `EuvToggleGroup`.
impl Mountable for EuvToggleGroup {
    /// Mounts the wrapping multi-option toggle group bound reactively to `selected`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Toggle group props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted toggle group container handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let selected: Signal<usize> = self.get_selected();
        let Self { options, .. } = self;
        let row: NodeId = ctx.view(
            Style::empty()
                .row()
                .wrap(FlexWrap::Wrap)
                .width(Length::Percent(100.0))
                .gap(SPACE_XS),
        );
        for (idx, opt) in options.into_iter().enumerate() {
            let item: NodeId = ctx.view(Style::empty());
            let txt: NodeId = ctx.text(
                opt,
                Style::empty()
                    .font_size(13.0)
                    .font_weight(FontWeight::Medium)
                    .color(p.get_foreground())
                    .text_overflow(TextOverflow::Clip),
            );
            ctx.append(item, txt);
            let handle: SceneHandle = ctx.get_scene().clone();
            let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
                handle.bind_style(item, move || {
                    let on: bool = selected.get() == idx;
                    Style::empty()
                        .row()
                        .align(AlignItems::Center)
                        .padding(Edges::symmetric(6.0, SPACE_MD))
                        .radius(g.get_radius_md())
                        .border(
                            g.get_border_width(),
                            if on { p.get_primary() } else { p.get_border() },
                        )
                        .background(if on {
                            p.get_accent()
                        } else {
                            p.get_background()
                        })
                        .cursor(CursorIcon::Pointer)
                })
            });
            ctx.get_scene().with_write(
                |tree: &mut SceneTree| {
                    tree.attach_scope(item, scope);
                },
                (),
            );
            ctx.on_click(item, move |_: &UiEvent| {
                let _: bool = selected.set_if_changed(idx);
            });
            ctx.append(row, item);
        }
        row
    }
}

/// `Mountable` implementation for `EuvInputOtp`.
impl Mountable for EuvInputOtp {
    /// Mounts segmented monospace OTP digit slots reacting live to `value`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Input OTP props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted OTP container handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let value: Signal<String> = self.get_value();
        let slots: usize = self.get_length().clamp(4, 8);
        let row: NodeId = ctx.view(
            Style::empty()
                .row()
                .wrap(FlexWrap::Wrap)
                .align(AlignItems::Center)
                .gap(SPACE_XS),
        );
        for slot_idx in 0..slots {
            let cell: NodeId = ctx.view(
                Style::empty()
                    .row()
                    .width(Length::Px(36.0))
                    .height(Length::Px(40.0))
                    .justify(JustifyContent::Center)
                    .align(AlignItems::Center)
                    .background(p.get_background())
                    .border(g.get_border_width(), p.get_border())
                    .radius(g.get_radius_md())
                    .cursor(CursorIcon::Pointer),
            );
            let digit: NodeId = ctx.reactive_text(
                Style::empty()
                    .font_family(FontFamily::Monospace)
                    .font_size(15.0)
                    .font_weight(FontWeight::SemiBold)
                    .color(p.get_foreground()),
                move || {
                    let cur: String = value.get();
                    cur.chars()
                        .nth(slot_idx)
                        .map(|c: char| c.to_string())
                        .unwrap_or_else(|| OTP_EMPTY_SLOT.to_string())
                },
            );
            ctx.on_click(cell, move |_: &UiEvent| {
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
            });
            ctx.append(cell, digit);
            ctx.append(row, cell);
        }
        row
    }
}

/// `Mountable` implementation for `EuvCommand`.
impl Mountable for EuvCommand {
    /// Mounts the searchable shadcn command palette with reactive filtering.
    ///
    /// # Arguments
    ///
    /// - `Self` - Command palette props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted command palette handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let tokens: ThemeTokens = self.get_tokens();
        let p: ThemePalette = tokens.get_palette();
        let g: ThemeGeometry = tokens.get_geometry();
        let query: Signal<String> = self.get_query();
        let selected: Signal<Option<usize>> = self.get_selected();
        let Self {
            placeholder, items, ..
        } = self;
        let palette_box: NodeId = ctx.view(
            Style::empty()
                .column()
                .width(Length::Percent(100.0))
                .max_width(Length::Px(520.0))
                .padding(Edges::all(SPACE_SM))
                .gap(SPACE_SM)
                .background(p.get_card())
                .border(g.get_border_width(), p.get_border())
                .radius(g.get_radius_lg())
                .overflow(Overflow::Hidden)
                .shadow(Shadow::new(0.0, 4.0, 12.0, 0.0, Color::rgba(0, 0, 0, 18))),
        );
        let search_input: NodeId = EuvInput::new(query, placeholder, tokens).mount(ctx);
        let list_host: NodeId = ctx.view(
            Style::empty()
                .column()
                .width(Length::Percent(100.0))
                .gap(SPACE_XS),
        );
        ctx.bind_children(list_host, move || {
            let needle: String = query.get().to_lowercase();
            let mut inner_ctx: BuildContext = current_build_context();
            let mut nodes: Vec<NodeId> = Vec::new();
            for (idx, item) in items.iter().enumerate() {
                if !needle.is_empty() && !item.to_lowercase().contains(&needle) {
                    continue;
                }
                let is_sel: bool = selected.get() == Some(idx);
                let row: NodeId = inner_ctx.view(
                    Style::empty()
                        .row()
                        .width(Length::Percent(100.0))
                        .padding(Edges::symmetric(6.0, SPACE_MD))
                        .radius(g.get_radius_sm())
                        .background(if is_sel {
                            p.get_accent()
                        } else {
                            COLOR_TRANSPARENT
                        })
                        .cursor(CursorIcon::Pointer),
                );
                let txt: NodeId = inner_ctx.text(
                    item.clone(),
                    Style::empty()
                        .font_size(13.0)
                        .font_weight(if is_sel {
                            FontWeight::SemiBold
                        } else {
                            FontWeight::Normal
                        })
                        .color(p.get_foreground()),
                );
                inner_ctx.append(row, txt);
                inner_ctx.on_click(row, move |_: &UiEvent| {
                    let _: bool = selected.set_if_changed(Some(idx));
                });
                nodes.push(row);
            }
            if nodes.is_empty() {
                let empty: NodeId = inner_ctx.text(
                    COMMAND_EMPTY_LABEL.to_string(),
                    Style::empty()
                        .font_size(12.0)
                        .color(p.get_muted_foreground()),
                );
                nodes.push(empty);
            }
            nodes
        });
        ctx.append(palette_box, search_input);
        ctx.append(palette_box, list_host);
        palette_box
    }
}

/// `Mountable` implementation for `EuvHoverCard`.
impl Mountable for EuvHoverCard {
    /// Mounts the trigger link and collapsible preview card surface.
    ///
    /// # Arguments
    ///
    /// - `Self` - Hover card props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted hover card wrapper handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let tokens: ThemeTokens = self.get_tokens();
        let p: ThemePalette = tokens.get_palette();
        let g: ThemeGeometry = tokens.get_geometry();
        let open: Signal<bool> = self.get_open();
        let Self {
            trigger_label,
            title,
            description,
            ..
        } = self;
        let wrap: NodeId = ctx.view(
            Style::empty()
                .column()
                .align(AlignItems::Start)
                .gap(SPACE_XS),
        );
        let mut trig_btn: EuvButton =
            EuvButton::new(trigger_label, tokens).on_click(move |_: &UiEvent| {
                let _: bool = open.set_if_changed(true);
            });
        trig_btn
            .set_variant(EuvButtonVariant::Link)
            .set_size(EuvButtonSize::Small);
        let trig_node: NodeId = trig_btn.mount(ctx);
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                if let Some(node) = tree.get_node_mut(wrap) {
                    node.get_mut_handlers()
                        .set_on_pointer_enter(Some(Rc::new(RefCell::new(move |_: &UiEvent| {
                            let _: bool = open.set_if_changed(true);
                        }))));
                    node.get_mut_handlers()
                        .set_on_pointer_leave(Some(Rc::new(RefCell::new(move |_: &UiEvent| {
                            let _: bool = open.set_if_changed(false);
                        }))));
                }
            },
            (),
        );
        let preview: NodeId = ctx.view(Style::empty());
        let h_node: NodeId = ctx.text(
            title,
            Style::empty()
                .font_size(13.0)
                .font_weight(FontWeight::SemiBold)
                .color(p.get_card_foreground()),
        );
        let d_node: NodeId = ctx.text(
            description,
            Style::empty()
                .font_size(12.0)
                .color(p.get_muted_foreground()),
        );
        ctx.append(preview, h_node);
        ctx.append(preview, d_node);
        let handle: SceneHandle = ctx.get_scene().clone();
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            handle.bind_style(preview, move || {
                let disp: Display = if open.get() {
                    Display::Flex
                } else {
                    Display::None
                };
                Style::empty()
                    .column()
                    .display(disp)
                    .width(Length::Percent(100.0))
                    .max_width(Length::Px(320.0))
                    .padding(Edges::all(SPACE_MD))
                    .gap(SPACE_XS)
                    .background(p.get_card())
                    .border(g.get_border_width(), p.get_border())
                    .radius(g.get_radius_lg())
                    .overflow(Overflow::Hidden)
                    .shadow(Shadow::new(0.0, 6.0, 16.0, 0.0, Color::rgba(0, 0, 0, 24)))
                    .z_index(Z_INDEX_FLOATING)
            })
        });
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                tree.attach_scope(preview, scope);
            },
            (),
        );
        ctx.append(wrap, trig_node);
        ctx.append(wrap, preview);
        wrap
    }
}

/// `Mountable` implementation for `EuvCalendar`.
impl Mountable for EuvCalendar {
    /// Mounts the 7-column monthly calendar grid with reactive day selection.
    ///
    /// # Arguments
    ///
    /// - `Self` - Calendar props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted calendar card handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let selected_day: Signal<u32> = self.get_selected_day();
        let total_days: u32 = self.get_days_in_month().clamp(28, 31);
        let Self { month_label, .. } = self;
        let card: NodeId = ctx.view(
            Style::empty()
                .column()
                .width(Length::Percent(100.0))
                .max_width(Length::Px(320.0))
                .padding(Edges::all(SPACE_MD))
                .gap(SPACE_SM)
                .background(p.get_card())
                .border(g.get_border_width(), p.get_border())
                .radius(g.get_radius_lg())
                .overflow(Overflow::Hidden),
        );
        let header: NodeId = ctx.text(
            month_label,
            Style::empty()
                .font_size(14.0)
                .font_weight(FontWeight::SemiBold)
                .color(p.get_foreground())
                .text_align(TextAlign::Center),
        );
        let weekdays: NodeId = ctx.view(
            Style::empty()
                .grid(7)
                .width(Length::Percent(100.0))
                .gap(2.0),
        );
        for wd in CALENDAR_WEEKDAYS.iter() {
            let cell: NodeId = ctx.view(
                Style::empty()
                    .row()
                    .justify(JustifyContent::Center)
                    .align(AlignItems::Center)
                    .padding(Edges::symmetric(2.0, 0.0)),
            );
            let txt: NodeId = ctx.text(
                (*wd).to_string(),
                Style::empty()
                    .font_size(11.0)
                    .font_weight(FontWeight::Medium)
                    .color(p.get_muted_foreground()),
            );
            ctx.append(cell, txt);
            ctx.append(weekdays, cell);
        }
        let days_grid: NodeId = ctx.view(
            Style::empty()
                .grid(7)
                .width(Length::Percent(100.0))
                .gap(2.0),
        );
        for day in 1..=total_days {
            let cell: NodeId = ctx.view(Style::empty());
            let txt: NodeId = ctx.text(
                day.to_string(),
                Style::empty()
                    .font_size(12.0)
                    .font_weight(FontWeight::Medium)
                    .color(p.get_foreground()),
            );
            ctx.append(cell, txt);
            let handle: SceneHandle = ctx.get_scene().clone();
            let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
                handle.bind_style(cell, move || {
                    let is_sel: bool = selected_day.get() == day;
                    Style::empty()
                        .row()
                        .height(Length::Px(28.0))
                        .justify(JustifyContent::Center)
                        .align(AlignItems::Center)
                        .radius(g.get_radius_sm())
                        .background(if is_sel {
                            p.get_accent()
                        } else {
                            COLOR_TRANSPARENT
                        })
                        .border(
                            if is_sel { g.get_border_width() } else { 0.0 },
                            p.get_primary(),
                        )
                        .cursor(CursorIcon::Pointer)
                })
            });
            ctx.get_scene().with_write(
                |tree: &mut SceneTree| {
                    tree.attach_scope(cell, scope);
                },
                (),
            );
            ctx.on_click(cell, move |_: &UiEvent| {
                let _: bool = selected_day.set_if_changed(day);
            });
            ctx.append(days_grid, cell);
        }
        ctx.append(card, header);
        ctx.append(card, weekdays);
        ctx.append(card, days_grid);
        card
    }
}

/// `Mountable` implementation for `EuvAspectRatio`.
impl Mountable for EuvAspectRatio {
    /// Mounts a full-width container constrained to `ratio` (`width / height`).
    ///
    /// # Arguments
    ///
    /// - `Self` - Aspect ratio props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted aspect ratio container handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let ratio: f32 = if self.get_ratio() > 0.0 && self.get_ratio().is_finite() {
            self.get_ratio()
        } else {
            16.0 / 9.0
        };
        let child: NodeId = self.get_child();
        let frame: NodeId = ctx.view(
            Style::empty()
                .column()
                .width(Length::Percent(100.0))
                .aspect_ratio(ratio)
                .justify(JustifyContent::Center)
                .align(AlignItems::Center)
                .background(p.get_muted())
                .border(g.get_border_width(), p.get_border())
                .radius(g.get_radius_md())
                .overflow(Overflow::Hidden),
        );
        ctx.append(frame, child);
        frame
    }
}

/// `Mountable` implementation for `EuvMenubar`.
impl Mountable for EuvMenubar {
    /// Mounts the wrapping horizontal menubar with reactive active item state.
    ///
    /// # Arguments
    ///
    /// - `Self` - Menubar props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted menubar handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let active_menu: Signal<Option<usize>> = self.get_active_menu();
        let Self { menus, .. } = self;
        let bar: NodeId = ctx.view(
            Style::empty()
                .row()
                .wrap(FlexWrap::Wrap)
                .width(Length::Percent(100.0))
                .padding(Edges::all(SPACE_XS))
                .gap(SPACE_XS)
                .background(p.get_card())
                .border(g.get_border_width(), p.get_border())
                .radius(g.get_radius_md()),
        );
        for (idx, label) in menus.into_iter().enumerate() {
            let item: NodeId = ctx.view(Style::empty());
            let txt: NodeId = ctx.text(
                label,
                Style::empty()
                    .font_size(13.0)
                    .font_weight(FontWeight::Medium)
                    .color(p.get_foreground())
                    .text_overflow(TextOverflow::Clip),
            );
            ctx.append(item, txt);
            let handle: SceneHandle = ctx.get_scene().clone();
            let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
                handle.bind_style(item, move || {
                    let is_open: bool = active_menu.get() == Some(idx);
                    Style::empty()
                        .row()
                        .padding(Edges::symmetric(5.0, SPACE_MD))
                        .radius(g.get_radius_sm())
                        .background(if is_open {
                            p.get_accent()
                        } else {
                            COLOR_TRANSPARENT
                        })
                        .cursor(CursorIcon::Pointer)
                })
            });
            ctx.get_scene().with_write(
                |tree: &mut SceneTree| {
                    tree.attach_scope(item, scope);
                },
                (),
            );
            ctx.on_click(item, move |_: &UiEvent| {
                let _: bool = active_menu.update(|cur: &mut Option<usize>| {
                    *cur = if *cur == Some(idx) { None } else { Some(idx) };
                });
            });
            ctx.append(bar, item);
        }
        bar
    }
}

/// `Mountable` implementation for `EuvCarousel`.
impl Mountable for EuvCarousel {
    /// Mounts the responsive slide viewport with Prev/Next controls and slide indicator.
    ///
    /// # Arguments
    ///
    /// - `Self` - Carousel props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted carousel container handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let tokens: ThemeTokens = self.get_tokens();
        let p: ThemePalette = tokens.get_palette();
        let g: ThemeGeometry = tokens.get_geometry();
        let active: Signal<usize> = self.get_active_slide();
        let Self { slides, .. } = self;
        let total: usize = slides.len();
        let slides_for_text: Vec<String> = slides;
        let root: NodeId = ctx.view(
            Style::empty()
                .column()
                .width(Length::Percent(100.0))
                .gap(SPACE_SM),
        );
        let stage: NodeId = ctx.view(
            Style::empty()
                .column()
                .width(Length::Percent(100.0))
                .padding(Edges::all(SPACE_XL))
                .justify(JustifyContent::Center)
                .align(AlignItems::Center)
                .background(p.get_muted())
                .border(g.get_border_width(), p.get_border())
                .radius(g.get_radius_lg())
                .overflow(Overflow::Hidden),
        );
        let slide_txt: NodeId = ctx.reactive_text(
            Style::empty()
                .font_size(14.0)
                .font_weight(FontWeight::SemiBold)
                .color(p.get_foreground())
                .text_align(TextAlign::Center),
            move || {
                if total == 0 {
                    CAROUSEL_EMPTY_LABEL.to_string()
                } else {
                    let idx: usize = active.get() % total;
                    slides_for_text[idx].clone()
                }
            },
        );
        ctx.append(stage, slide_txt);
        let controls: NodeId = ctx.view(
            Style::empty()
                .row()
                .wrap(FlexWrap::Wrap)
                .width(Length::Percent(100.0))
                .justify(JustifyContent::SpaceBetween)
                .align(AlignItems::Center)
                .gap(SPACE_SM),
        );
        let mut prev_btn: EuvButton =
            EuvButton::new(PAGE_PREV_LABEL.to_string(), tokens).on_click(move |_: &UiEvent| {
                if total > 0 {
                    let _: bool = active.update(|v: &mut usize| {
                        *v = if *v == 0 { total - 1 } else { *v - 1 };
                    });
                }
            });
        prev_btn
            .set_variant(EuvButtonVariant::Outline)
            .set_size(EuvButtonSize::Small);
        let mut next_btn: EuvButton =
            EuvButton::new(PAGE_NEXT_LABEL.to_string(), tokens).on_click(move |_: &UiEvent| {
                if total > 0 {
                    let _: bool = active.update(|v: &mut usize| *v = (*v + 1) % total);
                }
            });
        next_btn
            .set_variant(EuvButtonVariant::Outline)
            .set_size(EuvButtonSize::Small);
        let counter: NodeId = ctx.reactive_text(
            Style::empty()
                .font_size(12.0)
                .color(p.get_muted_foreground()),
            move || {
                if total == 0 {
                    "0 / 0".to_string()
                } else {
                    format!("{} / {total}", (active.get() % total) + 1)
                }
            },
        );
        let prev_node: NodeId = prev_btn.mount(ctx);
        let next_node: NodeId = next_btn.mount(ctx);
        ctx.append(controls, prev_node);
        ctx.append(controls, counter);
        ctx.append(controls, next_node);
        ctx.append(root, stage);
        ctx.append(root, controls);
        root
    }
}
