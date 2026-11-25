use super::*;

/// `Mountable` implementation for `EuvDrawer`.
impl Mountable for EuvDrawer {
    /// Mounts the fixed right-side drawer portal.
    ///
    /// # Arguments
    ///
    /// - `Self` - Drawer props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted drawer portal handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let open: Signal<bool> = self.get_open();
        let body: NodeId = self.get_body();
        let Self { title, .. } = self;
        let init_open: bool = open.get_untracked();
        let scrim: NodeId = ctx.portal(
            Style::empty()
                .row()
                .display(if init_open {
                    Display::Flex
                } else {
                    Display::None
                })
                .position(Position::Fixed)
                .inset(Some(0.0), Some(0.0), Some(0.0), Some(0.0))
                .width(Length::Percent(100.0))
                .height(Length::Percent(100.0))
                .justify(JustifyContent::End)
                .background(p.get_overlay())
                .z_index(Z_INDEX_DRAWER),
        );
        let panel: NodeId = ctx.view(
            Style::empty()
                .column()
                .width(Length::Percent(85.0))
                .max_width(Length::Px(360.0))
                .height(Length::Percent(100.0))
                .padding(Edges::all(SPACE_XL))
                .gap(SPACE_MD)
                .background(p.get_card())
                .border(g.get_border_width(), p.get_border())
                .overflow(Overflow::Hidden),
        );
        let heading: NodeId = ctx.text(
            title,
            Style::empty()
                .font_size(g.get_font_heading())
                .font_weight(FontWeight::Bold)
                .color(p.get_card_foreground()),
        );
        ctx.append(panel, heading);
        ctx.append(panel, body);
        ctx.append(scrim, panel);
        ctx.on_click(scrim, move |ev: &UiEvent| {
            if ev.get_target() == scrim {
                let _: bool = open.set_if_changed(false);
            }
        });
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
                    handle.start_transition(NodeTransition::fade(scrim, 0.0, 1.0, 0.22, false));
                    handle.start_transition(NodeTransition::slide_fade(
                        panel,
                        Point::new(360.0, 0.0),
                        Point::new(0.0, 0.0),
                        0.92,
                        1.0,
                        0.24,
                        false,
                    ));
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
                    handle.start_transition(NodeTransition::slide_fade(
                        panel,
                        Point::new(0.0, 0.0),
                        Point::new(360.0, 0.0),
                        1.0,
                        0.9,
                        0.22,
                        false,
                    ));
                    handle.start_transition(NodeTransition::fade(scrim, 1.0, 0.0, 0.22, true));
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

/// `Mountable` implementation for `EuvSheet`.
impl Mountable for EuvSheet {
    /// Mounts the bottom-anchored sheet portal into `ctx`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Sheet props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted sheet portal handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let open: Signal<bool> = self.get_open();
        let body: NodeId = self.get_body();
        let Self { title, .. } = self;
        let init_open: bool = open.get_untracked();
        let scrim: NodeId = ctx.portal(
            Style::empty()
                .column()
                .display(if init_open {
                    Display::Flex
                } else {
                    Display::None
                })
                .position(Position::Fixed)
                .inset(Some(0.0), Some(0.0), Some(0.0), Some(0.0))
                .width(Length::Percent(100.0))
                .height(Length::Percent(100.0))
                .justify(JustifyContent::End)
                .background(p.get_overlay())
                .z_index(Z_INDEX_DRAWER),
        );
        let panel: NodeId = ctx.view(
            Style::empty()
                .column()
                .width(Length::Percent(100.0))
                .padding(Edges::all(SPACE_XL))
                .gap(SPACE_MD)
                .background(p.get_card())
                .border(g.get_border_width(), p.get_border())
                .radius(g.get_radius_xl()),
        );
        let heading: NodeId = ctx.text(
            title,
            Style::empty()
                .font_size(g.get_font_heading())
                .font_weight(FontWeight::Bold)
                .color(p.get_card_foreground()),
        );
        ctx.append(panel, heading);
        ctx.append(panel, body);
        ctx.append(scrim, panel);
        ctx.on_click(scrim, move |ev: &UiEvent| {
            if ev.get_target() == scrim {
                let _: bool = open.set_if_changed(false);
            }
        });
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
                    handle.start_transition(NodeTransition::fade(scrim, 0.0, 1.0, 0.22, false));
                    handle.start_transition(NodeTransition::slide_fade(
                        panel,
                        Point::new(0.0, 240.0),
                        Point::new(0.0, 0.0),
                        0.92,
                        1.0,
                        0.24,
                        false,
                    ));
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
                    handle.start_transition(NodeTransition::slide_fade(
                        panel,
                        Point::new(0.0, 0.0),
                        Point::new(0.0, 240.0),
                        1.0,
                        0.9,
                        0.22,
                        false,
                    ));
                    handle.start_transition(NodeTransition::fade(scrim, 1.0, 0.0, 0.22, true));
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

/// `Mountable` implementation for `EuvPopover`.
impl Mountable for EuvPopover {
    /// Mounts the trigger and reactively shown popover card.
    ///
    /// # Arguments
    ///
    /// - `Self` - Popover props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted popover wrapper handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let open: Signal<bool> = self.get_open();
        let trigger: NodeId = self.get_trigger();
        let content: NodeId = self.get_content();
        let wrap: NodeId = ctx.view(
            Style::empty()
                .column()
                .align(AlignItems::Start)
                .gap(SPACE_XS),
        );
        let init_open: bool = open.get_untracked();
        let card: NodeId = ctx.view(
            Style::empty()
                .column()
                .display(if init_open {
                    Display::Flex
                } else {
                    Display::None
                })
                .padding(Edges::all(SPACE_MD))
                .background(p.get_card())
                .border(g.get_border_width(), p.get_border())
                .radius(g.get_radius_md())
                .shadow(Shadow::new(0.0, 4.0, 12.0, 0.0, Color::rgba(0, 0, 0, 22)))
                .z_index(Z_INDEX_FLOATING),
        );
        ctx.append(card, content);
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
                    handle.start_transition(NodeTransition::slide_fade(
                        card,
                        Point::new(0.0, -6.0),
                        Point::new(0.0, 0.0),
                        0.0,
                        1.0,
                        0.16,
                        false,
                    ));
                } else {
                    handle.start_transition(NodeTransition::slide_fade(
                        card,
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
                tree.attach_scope(card, scope);
            },
            (),
        );
        ctx.on_click(trigger, move |_: &UiEvent| {
            let _: bool = open.update(|v: &mut bool| *v = !*v);
        });
        ctx.append(wrap, trigger);
        ctx.append(wrap, card);
        wrap
    }
}

/// `Mountable` implementation for `EuvTooltip`.
impl Mountable for EuvTooltip {
    /// Mounts the trigger and compact hint surface into `ctx`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Tooltip props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted tooltip wrapper handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let trigger: NodeId = self.get_trigger();
        let Self { text, .. } = self;
        let wrap: NodeId = ctx.view(
            Style::empty()
                .column()
                .align(AlignItems::Start)
                .gap(SPACE_XS),
        );
        let tip: NodeId = ctx.view(
            Style::empty()
                .row()
                .padding(Edges::symmetric(SPACE_XS, SPACE_SM))
                .background(p.get_foreground())
                .radius(g.get_radius_sm()),
        );
        let txt: NodeId = ctx.text(
            text,
            Style::empty().font_size(11.0).color(p.get_background()),
        );
        ctx.append(tip, txt);
        ctx.append(wrap, trigger);
        ctx.append(wrap, tip);
        wrap
    }
}

/// `Mountable` implementation for `EuvDropdown`.
impl Mountable for EuvDropdown {
    /// Mounts the dropdown trigger button and collapsible action list.
    ///
    /// # Arguments
    ///
    /// - `Self` - Dropdown props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted dropdown wrapper handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let tokens: ThemeTokens = self.get_tokens();
        let p: ThemePalette = tokens.get_palette();
        let g: ThemeGeometry = tokens.get_geometry();
        let open: Signal<bool> = self.get_open();
        let last_selected: Signal<Option<usize>> = self.get_last_selected();
        let Self { label, items, .. } = self;
        let wrap: NodeId = ctx.view(
            Style::empty()
                .column()
                .align(AlignItems::Start)
                .gap(SPACE_XS),
        );
        let mut trigger_btn: EuvButton =
            EuvButton::new(label, tokens).on_click(move |_: &UiEvent| {
                let _: bool = open.update(|v: &mut bool| *v = !*v);
            });
        trigger_btn.set_variant(EuvButtonVariant::Outline);
        let trigger: NodeId = trigger_btn.mount(ctx);
        let init_open: bool = open.get_untracked();
        let menu: NodeId = ctx.view(
            Style::empty()
                .column()
                .display(if init_open {
                    Display::Flex
                } else {
                    Display::None
                })
                .background(p.get_card())
                .border(g.get_border_width(), p.get_border())
                .radius(g.get_radius_md())
                .shadow(Shadow::new(0.0, 4.0, 12.0, 0.0, Color::rgba(0, 0, 0, 22)))
                .z_index(Z_INDEX_FLOATING),
        );
        for (idx, item_label) in items.into_iter().enumerate() {
            let row: NodeId = ctx.view(
                Style::empty()
                    .row()
                    .padding(Edges::symmetric(SPACE_SM, SPACE_MD))
                    .cursor(CursorIcon::Pointer),
            );
            let txt: NodeId = ctx.text(
                item_label,
                Style::empty().font_size(13.0).color(p.get_foreground()),
            );
            ctx.append(row, txt);
            ctx.on_click(row, move |_: &UiEvent| {
                let _: bool = last_selected.set_if_changed(Some(idx));
                let _: bool = open.set_if_changed(false);
            });
            ctx.append(menu, row);
        }
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
        ctx.append(wrap, trigger);
        ctx.append(wrap, menu);
        wrap
    }
}

/// `Mountable` implementation for `EuvNotification`.
impl Mountable for EuvNotification {
    /// Mounts the top-right notification portal into `ctx`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Notification props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted notification portal handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let (accent, _): (Color, Color) = self.get_tone().colors(p);
        let visible: Signal<bool> = self.get_visible();
        let Self { title, message, .. } = self;
        let init_vis: bool = visible.get_untracked();
        let card: NodeId = ctx.portal(
            Style::empty()
                .column()
                .display(if init_vis {
                    Display::Flex
                } else {
                    Display::None
                })
                .position(Position::Fixed)
                .inset(Some(SPACE_LG), Some(SPACE_LG), None, None)
                .width(Length::Percent(90.0))
                .max_width(Length::Px(340.0))
                .padding(Edges::all(SPACE_MD))
                .gap(SPACE_XS)
                .background(p.get_card())
                .border(g.get_border_width(), accent)
                .radius(g.get_radius_lg())
                .shadow(Shadow::new(0.0, 6.0, 16.0, 0.0, Color::rgba(0, 0, 0, 28)))
                .overflow(Overflow::Hidden)
                .z_index(Z_INDEX_TOAST),
        );
        let t_node: NodeId = ctx.text(
            title,
            Style::empty()
                .font_size(14.0)
                .font_weight(FontWeight::Bold)
                .color(p.get_card_foreground()),
        );
        let m_node: NodeId = ctx.text(
            message,
            Style::empty()
                .font_size(12.0)
                .color(p.get_muted_foreground()),
        );
        ctx.append(card, t_node);
        ctx.append(card, m_node);
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
                    handle.start_transition(NodeTransition::slide_fade(
                        card,
                        Point::new(24.0, 0.0),
                        Point::new(0.0, 0.0),
                        0.0,
                        1.0,
                        0.20,
                        false,
                    ));
                } else {
                    handle.start_transition(NodeTransition::slide_fade(
                        card,
                        Point::new(0.0, 0.0),
                        Point::new(24.0, 0.0),
                        1.0,
                        0.0,
                        0.18,
                        true,
                    ));
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

/// `Mountable` implementation for `EuvLoading`.
impl Mountable for EuvLoading {
    /// Mounts the centered loading scrim and spinner portal.
    ///
    /// # Arguments
    ///
    /// - `Self` - Loading props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted loading portal handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let tokens: ThemeTokens = self.get_tokens();
        let p: ThemePalette = tokens.get_palette();
        let active: Signal<bool> = self.get_active();
        let Self { label, .. } = self;
        let init_active: bool = active.get_untracked();
        let portal: NodeId = ctx.portal(
            Style::empty()
                .column()
                .display(if init_active {
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
                .gap(SPACE_MD)
                .background(p.get_overlay())
                .z_index(Z_INDEX_MODAL),
        );
        let spinner: NodeId = EuvSpinner::new(32.0, tokens).mount(ctx);
        let txt: NodeId = ctx.text(
            label,
            Style::empty()
                .font_size(14.0)
                .font_weight(FontWeight::Medium)
                .color(p.get_foreground()),
        );
        ctx.append(portal, spinner);
        ctx.append(portal, txt);
        ctx.on_click(portal, move |ev: &UiEvent| {
            if ev.get_target() == portal {
                let _: bool = active.set_if_changed(false);
            }
        });
        let handle: SceneHandle = ctx.get_scene().clone();
        let prev_active: Rc<Cell<bool>> = Rc::new(Cell::new(init_active));
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
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
