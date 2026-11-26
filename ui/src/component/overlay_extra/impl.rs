use super::*;

/// Declarative `#[component]` implementation for `EuvDrawer`.
#[component]
/// Inherent implementation of [`EuvDrawer`].
impl EuvDrawer {
    /// Renders `EuvDrawer` into the ambient framework-managed `Context`.
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
        let open: Signal<bool> = self.get_open();
        let body: NodeId = self.get_body();
        let Self { title, .. } = self.clone();
        let init_open: bool = open.get_untracked();
        let mut panel: NodeId = NodeId::default();
        let scrim: NodeId = view! {
            portal {
                style: Style::default().row()
                display: if init_open { Display::Flex } else { Display::None }
                position: Position::Fixed
                inset: (Some(0.0), Some(0.0), Some(0.0), Some(0.0))
                width: Length::Percent(100.0)
                height: Length::Percent(100.0)
                justify: JustifyContent::End
                background: p.get_overlay()
                stop_scroll_propagation: true
                z_index: Z_INDEX_DRAWER
                column {
                    node_ref: panel
                    width: Length::Percent(85.0)
                    max_width: Length::Px(360.0)
                    height: Length::Percent(100.0)
                    padding: Edges::all(SPACE_XL)
                    gap: SPACE_MD
                    background: p.get_card()
                    border: (g.get_border_width(), p.get_border())
                    overflow: Overflow::Hidden
                    text {
                        font_size: g.get_font_heading()
                        font_weight: FontWeight::Bold
                        color: p.get_card_foreground()
                        { title }
                    }
                    { body }
                }
            }
        };
        ctx.on_click(scrim, move |ev: &UiEvent| {
            if ev.get_target() == scrim {
                let _: bool = open.set_if_changed(false);
            }
        });
        let handle: SceneHandle = ctx.get_scene().clone();
        let prev_open: Rc<Cell<bool>> = Rc::new(Cell::new(init_open));
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            let _: Effect = EuvOverlayStack::bind_signal(open, DRAWER_OVERLAY_NAME, Z_INDEX_DRAWER);
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
                    handle.start_transition(
                        NodeTransition::fade(scrim, 0.0, 1.0, 0.24, false)
                            .with_easing(Easing::EaseOutQuint),
                    );
                    handle.start_transition(
                        NodeTransition::slide_fade(
                            panel,
                            Point::new(360.0, 0.0),
                            Point::new(0.0, 0.0),
                            0.92,
                            1.0,
                            0.34,
                            false,
                        )
                        .with_scale(0.97, 1.0)
                        .with_easing(Easing::SpringGentle),
                    );
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
                    handle.start_transition(
                        NodeTransition::slide_fade(
                            panel,
                            Point::new(0.0, 0.0),
                            Point::new(360.0, 0.0),
                            1.0,
                            1.0,
                            0.20,
                            false,
                        )
                        .with_easing(Easing::EaseOutCubic),
                    );
                    handle.start_transition(
                        NodeTransition::fade(scrim, 1.0, 0.0, 0.20, true)
                            .with_easing(Easing::EaseOutCubic),
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

/// Declarative `#[component]` implementation for `EuvSheet`.
#[component]
/// Inherent implementation of [`EuvSheet`].
impl EuvSheet {
    /// Renders `EuvSheet` into the ambient framework-managed `Context`.
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
        let open: Signal<bool> = self.get_open();
        let body: NodeId = self.get_body();
        let Self { title, .. } = self.clone();
        let init_open: bool = open.get_untracked();
        let mut panel: NodeId = NodeId::default();
        let scrim: NodeId = view! {
            portal {
                style: Style::default().column()
                display: if init_open { Display::Flex } else { Display::None }
                position: Position::Fixed
                inset: (Some(0.0), Some(0.0), Some(0.0), Some(0.0))
                width: Length::Percent(100.0)
                height: Length::Percent(100.0)
                justify: JustifyContent::End
                background: p.get_overlay()
                stop_scroll_propagation: true
                z_index: Z_INDEX_DRAWER
                column {
                    node_ref: panel
                    width: Length::Percent(100.0)
                    padding: Edges::all(SPACE_XL)
                    gap: SPACE_MD
                    background: p.get_card()
                    border: (g.get_border_width(), p.get_border())
                    radius: g.get_radius_xl()
                    row {
                        width: Length::Percent(100.0)
                        justify: JustifyContent::Center
                        view {
                            width: Length::Px(38.0)
                            height: Length::Px(4.0)
                            radius: 99.0
                            background: p.get_border()
                        }
                    }
                    text {
                        font_size: g.get_font_heading()
                        font_weight: FontWeight::Bold
                        color: p.get_card_foreground()
                        { title }
                    }
                    { body }
                }
            }
        };
        ctx.on_click(scrim, move |ev: &UiEvent| {
            if ev.get_target() == scrim {
                let _: bool = open.set_if_changed(false);
            }
        });
        let handle: SceneHandle = ctx.get_scene().clone();
        let prev_open: Rc<Cell<bool>> = Rc::new(Cell::new(init_open));
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            let _: Effect = EuvOverlayStack::bind_signal(open, SHEET_OVERLAY_NAME, Z_INDEX_DRAWER);
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
                    handle.start_transition(
                        NodeTransition::fade(scrim, 0.0, 1.0, 0.24, false)
                            .with_easing(Easing::EaseOutQuint),
                    );
                    handle.start_transition(
                        NodeTransition::slide_fade(
                            panel,
                            Point::new(0.0, 240.0),
                            Point::new(0.0, 0.0),
                            0.92,
                            1.0,
                            0.34,
                            false,
                        )
                        .with_scale(0.97, 1.0)
                        .with_easing(Easing::SpringGentle),
                    );
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
                    handle.start_transition(
                        NodeTransition::slide_fade(
                            panel,
                            Point::new(0.0, 0.0),
                            Point::new(0.0, 240.0),
                            1.0,
                            1.0,
                            0.20,
                            false,
                        )
                        .with_easing(Easing::EaseOutCubic),
                    );
                    handle.start_transition(
                        NodeTransition::fade(scrim, 1.0, 0.0, 0.20, true)
                            .with_easing(Easing::EaseOutCubic),
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

/// Declarative `#[component]` implementation for `EuvPopover`.
#[component]
/// Inherent implementation of [`EuvPopover`].
impl EuvPopover {
    /// Renders `EuvPopover` into the ambient framework-managed `Context`.
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
        let open: Signal<bool> = self.get_open();
        let trigger: NodeId = self.get_trigger();
        let content: NodeId = self.get_content();
        let init_open: bool = open.get_untracked();
        let mut card: NodeId = NodeId::default();
        let wrap: NodeId = view! {
            column {
                align: AlignItems::Start
                gap: SPACE_XS
                { trigger }
                column {
                    node_ref: card
                    display: if init_open { Display::Flex } else { Display::None }
                    padding: Edges::all(SPACE_MD)
                    background: p.get_card()
                    border: (g.get_border_width(), p.get_border())
                    radius: g.get_radius_md()
                    shadow: g.get_shadow_md()
                    z_index: Z_INDEX_FLOATING
                    { content }
                }
            }
        };
        let handle: SceneHandle = ctx.get_scene().clone();
        let prev_open: Rc<Cell<bool>> = Rc::new(Cell::new(init_open));
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            let _: Effect =
                EuvOverlayStack::bind_signal(open, POPOVER_OVERLAY_NAME, Z_INDEX_DRAWER - 10);
            Effect::new(move || {
                let is_open: bool = open.get();
                let was_open: bool = prev_open.replace(is_open);
                if is_open == was_open {
                    return;
                }
                if is_open {
                    handle.start_transition(
                        NodeTransition::slide_fade(
                            card,
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
                            card,
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
                tree.attach_scope(card, scope);
            },
            (),
        );
        ctx.on_click(trigger, move |_: &UiEvent| {
            let _: bool = open.update(|v: &mut bool| *v = !*v);
        });
        wrap
    }
}

/// Fluent configuration builders for `EuvTooltip`.
impl EuvTooltip {
    /// Controls whether the tooltip stays permanently visible or toggles reactively on pointer hover.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source tooltip.
    /// - `bool` - When `true`, renders permanently visible.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated tooltip.
    #[must_use]
    pub fn always_visible(mut self, always_visible: bool) -> Self {
        self.set_always_visible(always_visible);
        self
    }
}

/// Declarative `#[component]` implementation for `EuvTooltip`.
#[component]
/// Inherent implementation of [`EuvTooltip`].
impl EuvTooltip {
    /// Renders `EuvTooltip` into the ambient framework-managed `Context`.
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
        let trigger: NodeId = self.get_trigger();
        let pinned: bool = self.get_always_visible();
        let Self { text, .. } = self.clone();
        let hovered: Signal<bool> = Signal::new(pinned);
        let mut tip: NodeId = NodeId::default();
        let wrap: NodeId = view! {
            column {
                align: AlignItems::Start
                gap: SPACE_XS
                { trigger }
                row {
                    node_ref: tip
                    display: if pinned { Display::Flex } else { Display::None }
                    padding: Edges::symmetric(SPACE_XS, SPACE_SM)
                    background: p.get_foreground()
                    radius: g.get_radius_sm()
                    shadow: g.get_shadow_md()
                    pointer_events: PointerEvents::None
                    z_index: Z_INDEX_TOOLTIP
                    text {
                        font_size: 11.0
                        color: p.get_background()
                        { text }
                    }
                }
            }
        };
        if !pinned {
            ctx.on_pointer_enter(wrap, move |_: &UiEvent| {
                let _: bool = hovered.set_if_changed(true);
            });
            ctx.on_pointer_leave(wrap, move |_: &UiEvent| {
                let _: bool = hovered.set_if_changed(false);
            });
            ctx.bind_style(tip, move || {
                let vis: Display = if hovered.get() {
                    Display::Flex
                } else {
                    Display::None
                };
                Style::default()
                    .row()
                    .display(vis)
                    .padding(Edges::symmetric(SPACE_XS, SPACE_SM))
                    .background(p.get_foreground())
                    .radius(g.get_radius_sm())
                    .shadow(g.get_shadow_md())
                    .pointer_events(PointerEvents::None)
                    .z_index(Z_INDEX_TOOLTIP)
            });
        }
        wrap
    }
}

/// Fluent controlled-state builders for `EuvDropdown`.
impl EuvDropdown {
    /// Binds a controlled reactive open flag signal fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source dropdown.
    /// - `Signal<bool>` - Controlled open state signal.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated dropdown.
    #[must_use]
    pub fn open(mut self, open: Signal<bool>) -> Self {
        self.set_open(Some(open));
        self
    }

    /// Binds a reactive signal capturing the index of the most recently clicked menu item.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source dropdown.
    /// - `Signal<Option<usize>>` - Controlled selected item index signal.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated dropdown.
    #[must_use]
    pub fn selected(mut self, last_selected: Signal<Option<usize>>) -> Self {
        self.set_last_selected(Some(last_selected));
        self
    }
}

/// Declarative `#[component]` implementation for `EuvDropdown`.
#[component]
/// Inherent implementation of [`EuvDropdown`].
impl EuvDropdown {
    /// Renders `EuvDropdown` into the ambient framework-managed `Context`.
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
            label,
            items,
            open: open_opt,
            last_selected: sel_opt,
            ..
        } = self.clone();
        let open: Signal<bool> = open_opt.unwrap_or_else(|| Signal::new(false));
        let last_selected: Signal<Option<usize>> = sel_opt.unwrap_or_else(|| Signal::new(None));
        let init_open: bool = open.get_untracked();
        let mut menu: NodeId = NodeId::default();
        let wrap: NodeId = view! {
            column {
                width: Length::Percent(100.0)
                max_width: Length::Px(280.0)
                gap: SPACE_XS
                EuvButton(label, tokens) {
                    outline()
                    on_click: move |_: &UiEvent| {
                        let _: bool = open.update(|v: &mut bool| *v = !*v);
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
                    z_index: Z_INDEX_FLOATING
                    role: A11yRole::Menu
                    for (idx, item_label) in items.into_iter().enumerate() {
                        row {
                            width: Length::Percent(100.0)
                            padding: Edges::symmetric(SPACE_SM, SPACE_MD)
                            radius: g.get_radius_sm()
                            cursor: CursorIcon::Pointer
                            role: A11yRole::MenuItem
                            a11y_label: item_label.clone()
                            on_click: move |_: &UiEvent| {
                                let _: bool = last_selected.set_if_changed(Some(idx));
                                let _: bool = open.set_if_changed(false);
                            }
                            text {
                                font_size: 13.0
                                color: p.get_foreground()
                                { item_label }
                            }
                        }
                    }
                }
            }
        };
        let handle: SceneHandle = ctx.get_scene().clone();
        let prev_open: Rc<Cell<bool>> = Rc::new(Cell::new(init_open));
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            let _: Effect =
                EuvOverlayStack::bind_signal(open, DROPDOWN_OVERLAY_NAME, Z_INDEX_DRAWER - 10);
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
        wrap
    }
}

/// Fluent tone builder for `EuvNotification`.
impl EuvNotification {
    /// Sets the semantic tone accent border fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source notification.
    /// - `EuvTone` - Target semantic tone.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated notification.
    #[must_use]
    pub fn tone(mut self, tone: EuvTone) -> Self {
        self.set_tone(tone);
        self
    }
}

/// Declarative `#[component]` implementation for `EuvNotification`.
#[component]
/// Inherent implementation of [`EuvNotification`].
impl EuvNotification {
    /// Renders `EuvNotification` into the ambient framework-managed `Context`.
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
        let (accent, _): (Color, Color) = self.get_tone().colors(p);
        let visible: Signal<bool> = self.get_visible();
        let Self { title, message, .. } = self.clone();
        let init_vis: bool = visible.get_untracked();
        let card: NodeId = view! {
            portal {
                style: Style::default().column()
                display: if init_vis { Display::Flex } else { Display::None }
                position: Position::Fixed
                inset: (Some(SPACE_LG), Some(SPACE_LG), None, None)
                width: Length::Percent(90.0)
                max_width: Length::Px(340.0)
                padding: Edges::all(SPACE_MD)
                gap: SPACE_XS
                background: p.get_card()
                border: (g.get_border_width(), accent)
                radius: g.get_radius_lg()
                shadow: g.get_shadow_lg()
                overflow: Overflow::Hidden
                z_index: Z_INDEX_TOAST
                role: A11yRole::Alert
                a11y_label: title.clone()
                text {
                    font_size: 14.0
                    font_weight: FontWeight::Bold
                    color: p.get_card_foreground()
                    { title }
                }
                text {
                    font_size: 12.0
                    color: p.get_muted_foreground()
                    { message }
                }
            }
        };
        let handle: SceneHandle = ctx.get_scene().clone();
        let prev_vis: Rc<Cell<bool>> = Rc::new(Cell::new(init_vis));
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            Effect::new(move || {
                let is_vis: bool = visible.get();
                let was_vis: bool = prev_vis.replace(is_vis);
                if is_vis == was_vis {
                    return;
                }
                if is_vis {
                    handle.start_transition(
                        NodeTransition::slide_fade(
                            card,
                            Point::new(36.0, 0.0),
                            Point::new(0.0, 0.0),
                            0.0,
                            1.0,
                            0.28,
                            false,
                        )
                        .with_scale(0.95, 1.0)
                        .with_easing(Easing::SpringSnappy),
                    );
                } else {
                    handle.start_transition(
                        NodeTransition::slide_fade(
                            card,
                            Point::new(0.0, 0.0),
                            Point::new(28.0, 0.0),
                            1.0,
                            0.0,
                            0.18,
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
                tree.attach_scope(card, scope);
            },
            (),
        );
        card
    }
}

/// Declarative `#[component]` implementation for `EuvLoading`.
#[component]
/// Inherent implementation of [`EuvLoading`].
impl EuvLoading {
    /// Renders `EuvLoading` into the ambient framework-managed `Context`.
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
        let active: Signal<bool> = self.get_active();
        let Self { label, .. } = self.clone();
        let init_active: bool = active.get_untracked();
        let portal: NodeId = view! {
            portal {
                style: Style::default().column()
                display: if init_active { Display::Flex } else { Display::None }
                position: Position::Fixed
                inset: (Some(0.0), Some(0.0), Some(0.0), Some(0.0))
                width: Length::Percent(100.0)
                height: Length::Percent(100.0)
                justify: JustifyContent::Center
                align: AlignItems::Center
                gap: SPACE_MD
                background: p.get_overlay()
                stop_scroll_propagation: true
                z_index: Z_INDEX_MODAL
                EuvSpinner(32.0, tokens)
                text {
                    font_size: 14.0
                    font_weight: FontWeight::Medium
                    color: p.get_foreground()
                    { label }
                }
            }
        };
        ctx.on_click(portal, move |ev: &UiEvent| {
            if ev.get_target() == portal {
                let _: bool = active.set_if_changed(false);
            }
        });
        let handle: SceneHandle = ctx.get_scene().clone();
        let prev_active: Rc<Cell<bool>> = Rc::new(Cell::new(init_active));
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            let _: Effect =
                EuvOverlayStack::bind_signal(active, LOADING_OVERLAY_NAME, Z_INDEX_MODAL);
            Effect::new(move || {
                let is_on: bool = active.get();
                let was_on: bool = prev_active.replace(is_on);
                if is_on == was_on {
                    return;
                }
                if is_on {
                    handle.with_write(
                        |tree: &mut SceneTree| {
                            if let Some(n) = tree.get_node_mut(portal) {
                                let mut st: Style = n.get_style();
                                st.get_mut_paint().set_pointer_events(PointerEvents::Auto);
                                n.set_style(st);
                            }
                        },
                        (),
                    );
                    handle.start_transition(NodeTransition::fade(portal, 0.0, 1.0, 0.18, false));
                } else {
                    handle.with_write(
                        |tree: &mut SceneTree| {
                            if let Some(n) = tree.get_node_mut(portal) {
                                let mut st: Style = n.get_style();
                                st.get_mut_paint().set_pointer_events(PointerEvents::None);
                                n.set_style(st);
                            }
                        },
                        (),
                    );
                    handle.start_transition(NodeTransition::fade(portal, 1.0, 0.0, 0.18, true));
                }
            })
        });
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                tree.attach_scope(portal, scope);
            },
            (),
        );
        portal
    }
}
