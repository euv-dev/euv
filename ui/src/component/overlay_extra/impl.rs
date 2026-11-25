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
        let scrim: NodeId = ctx.portal(Style::empty());
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
        let handle: SceneHandle = ctx.get_scene().clone();
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            handle.bind_style(scrim, move || {
                let disp: Display = if open.get() {
                    Display::Flex
                } else {
                    Display::None
                };
                Style::empty()
                    .row()
                    .display(disp)
                    .position(Position::Fixed)
                    .inset(Some(0.0), Some(0.0), Some(0.0), Some(0.0))
                    .width(Length::Percent(100.0))
                    .height(Length::Percent(100.0))
                    .justify(JustifyContent::End)
                    .background(p.get_overlay())
                    .z_index(Z_INDEX_DRAWER)
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
        let scrim: NodeId = ctx.portal(Style::empty());
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
        let handle: SceneHandle = ctx.get_scene().clone();
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            handle.bind_style(scrim, move || {
                let disp: Display = if open.get() {
                    Display::Flex
                } else {
                    Display::None
                };
                Style::empty()
                    .column()
                    .display(disp)
                    .position(Position::Fixed)
                    .inset(Some(0.0), Some(0.0), Some(0.0), Some(0.0))
                    .width(Length::Percent(100.0))
                    .height(Length::Percent(100.0))
                    .justify(JustifyContent::End)
                    .background(p.get_overlay())
                    .z_index(Z_INDEX_DRAWER)
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
        let wrap: NodeId = ctx.view(Style::empty().column().gap(SPACE_XS));
        let card: NodeId = ctx.view(Style::empty());
        ctx.append(card, content);
        let handle: SceneHandle = ctx.get_scene().clone();
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            handle.bind_style(card, move || {
                let disp: Display = if open.get() {
                    Display::Flex
                } else {
                    Display::None
                };
                Style::empty()
                    .column()
                    .display(disp)
                    .padding(Edges::all(SPACE_MD))
                    .background(p.get_card())
                    .border(g.get_border_width(), p.get_border())
                    .radius(g.get_radius_md())
                    .z_index(Z_INDEX_FLOATING)
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
        let wrap: NodeId = ctx.view(Style::empty().column().gap(SPACE_XS));
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
        let wrap: NodeId = ctx.view(Style::empty().column().gap(SPACE_XS));
        let mut trigger_btn: EuvButton =
            EuvButton::new(label, tokens).on_click(move |_: &UiEvent| {
                let _: bool = open.update(|v: &mut bool| *v = !*v);
            });
        trigger_btn.set_variant(EuvButtonVariant::Outline);
        let trigger: NodeId = trigger_btn.mount(ctx);
        let menu: NodeId = ctx.view(Style::empty());
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
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            handle.bind_style(menu, move || {
                let disp: Display = if open.get() {
                    Display::Flex
                } else {
                    Display::None
                };
                Style::empty()
                    .column()
                    .display(disp)
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
        let card: NodeId = ctx.portal(Style::empty());
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
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            handle.bind_style(card, move || {
                let disp: Display = if visible.get() {
                    Display::Flex
                } else {
                    Display::None
                };
                Style::empty()
                    .column()
                    .display(disp)
                    .position(Position::Fixed)
                    .inset(Some(SPACE_LG), Some(SPACE_LG), None, None)
                    .width(Length::Percent(90.0))
                    .max_width(Length::Px(340.0))
                    .padding(Edges::all(SPACE_MD))
                    .gap(SPACE_XS)
                    .background(p.get_card())
                    .border(g.get_border_width(), accent)
                    .radius(g.get_radius_md())
                    .overflow(Overflow::Hidden)
                    .z_index(Z_INDEX_TOAST)
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
        let portal: NodeId = ctx.portal(Style::empty());
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
        let handle: SceneHandle = ctx.get_scene().clone();
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            handle.bind_style(portal, move || {
                let disp: Display = if active.get() {
                    Display::Flex
                } else {
                    Display::None
                };
                Style::empty()
                    .column()
                    .display(disp)
                    .position(Position::Fixed)
                    .inset(Some(0.0), Some(0.0), Some(0.0), Some(0.0))
                    .width(Length::Percent(100.0))
                    .height(Length::Percent(100.0))
                    .justify(JustifyContent::Center)
                    .align(AlignItems::Center)
                    .gap(SPACE_MD)
                    .background(p.get_overlay())
                    .z_index(Z_INDEX_MODAL)
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
