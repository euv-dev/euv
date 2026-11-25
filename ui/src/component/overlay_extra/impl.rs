use super::*;

/// Builder methods for `EuvDrawer`.
impl EuvDrawer {
    /// Creates a right-edge slide-over drawer.
    ///
    /// # Arguments
    ///
    /// - `Signal<bool>` - Reactive open flag.
    /// - `String` - Drawer title.
    /// - `NodeId` - Mounted body node.
    /// - `ThemeTokens` - Active design tokens.
    ///
    /// # Returns
    ///
    /// - `Self` - Constructed drawer.
    #[must_use]
    pub fn create(open: Signal<bool>, title: String, body: NodeId, tokens: ThemeTokens) -> Self {
        Self {
            open,
            title,
            body,
            tokens,
        }
    }
}

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
                .width(Length::Px(360.0))
                .height(Length::Percent(100.0))
                .padding(Edges::all(SPACE_XL))
                .gap(SPACE_MD)
                .background(p.get_card())
                .border(g.get_border_width(), p.get_border()),
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
                    .display(disp)
                    .row()
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

/// Builder methods for `EuvSheet`.
impl EuvSheet {
    /// Creates a bottom sheet overlay.
    ///
    /// # Arguments
    ///
    /// - `Signal<bool>` - Reactive open flag.
    /// - `String` - Sheet title.
    /// - `NodeId` - Mounted body node.
    /// - `ThemeTokens` - Active design tokens.
    ///
    /// # Returns
    ///
    /// - `Self` - Constructed sheet.
    #[must_use]
    pub fn create(open: Signal<bool>, title: String, body: NodeId, tokens: ThemeTokens) -> Self {
        Self {
            open,
            title,
            body,
            tokens,
        }
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
                    .display(disp)
                    .column()
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

/// Builder methods for `EuvPopover`.
impl EuvPopover {
    /// Creates a reactive popover attached to `trigger`.
    ///
    /// # Arguments
    ///
    /// - `Signal<bool>` - Open visibility flag.
    /// - `NodeId` - Trigger node handle.
    /// - `NodeId` - Popover content handle.
    /// - `ThemeTokens` - Active design tokens.
    ///
    /// # Returns
    ///
    /// - `Self` - Constructed popover.
    #[must_use]
    pub fn create(
        open: Signal<bool>,
        trigger: NodeId,
        content: NodeId,
        tokens: ThemeTokens,
    ) -> Self {
        Self {
            open,
            trigger,
            content,
            tokens,
        }
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
                    .display(disp)
                    .column()
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

/// Builder methods for `EuvTooltip`.
impl EuvTooltip {
    /// Creates a tooltip container wrapping `trigger`.
    ///
    /// # Arguments
    ///
    /// - `String` - Tooltip hint text.
    /// - `NodeId` - Trigger node handle.
    /// - `ThemeTokens` - Active design tokens.
    ///
    /// # Returns
    ///
    /// - `Self` - Constructed tooltip.
    #[must_use]
    pub fn create(text: String, trigger: NodeId, tokens: ThemeTokens) -> Self {
        Self {
            text,
            trigger,
            tokens,
        }
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

/// Builder methods for `EuvDropdown`.
impl EuvDropdown {
    /// Creates a click-triggered dropdown action menu.
    ///
    /// # Arguments
    ///
    /// - `String` - Menu button label.
    /// - `Vec<String>` - Menu action labels.
    /// - `Signal<bool>` - Reactive open state.
    /// - `Signal<Option<usize>>` - Reactive selected action signal.
    /// - `ThemeTokens` - Active design tokens.
    ///
    /// # Returns
    ///
    /// - `Self` - Constructed dropdown menu.
    #[must_use]
    pub fn create(
        label: String,
        items: Vec<String>,
        open: Signal<bool>,
        last_selected: Signal<Option<usize>>,
        tokens: ThemeTokens,
    ) -> Self {
        Self {
            label,
            items,
            open,
            last_selected,
            tokens,
        }
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
        let trigger: NodeId = EuvButton::create(label, tokens)
            .variant(EuvButtonVariant::Outline)
            .on_click(move |_: &UiEvent| {
                let _: bool = open.update(|v: &mut bool| *v = !*v);
            })
            .mount(ctx);
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
                    .display(disp)
                    .column()
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

/// Builder methods for `EuvNotification`.
impl EuvNotification {
    /// Creates a corner notification card.
    ///
    /// # Arguments
    ///
    /// - `Signal<bool>` - Reactive visibility signal.
    /// - `String` - Notification title.
    /// - `String` - Notification detail message.
    /// - `EuvTone` - Semantic tone.
    /// - `ThemeTokens` - Active design tokens.
    ///
    /// # Returns
    ///
    /// - `Self` - Constructed notification card.
    #[must_use]
    pub fn create(
        visible: Signal<bool>,
        title: String,
        message: String,
        tone: EuvTone,
        tokens: ThemeTokens,
    ) -> Self {
        Self {
            visible,
            title,
            message,
            tone,
            tokens,
        }
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
                    .display(disp)
                    .column()
                    .position(Position::Fixed)
                    .inset(Some(SPACE_LG), Some(SPACE_LG), None, None)
                    .width(Length::Px(320.0))
                    .padding(Edges::all(SPACE_MD))
                    .gap(SPACE_XS)
                    .background(p.get_card())
                    .border(g.get_border_width(), accent)
                    .radius(g.get_radius_md())
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

/// Builder methods for `EuvLoading`.
impl EuvLoading {
    /// Creates a full-surface loading overlay.
    ///
    /// # Arguments
    ///
    /// - `Signal<bool>` - Reactive active flag.
    /// - `String` - Loading caption.
    /// - `ThemeTokens` - Active design tokens.
    ///
    /// # Returns
    ///
    /// - `Self` - Constructed loading overlay.
    #[must_use]
    pub fn create(active: Signal<bool>, label: String, tokens: ThemeTokens) -> Self {
        Self {
            active,
            label,
            tokens,
        }
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
        let spinner: NodeId = EuvSpinner::create(32.0, tokens).mount(ctx);
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
                    .display(disp)
                    .column()
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
