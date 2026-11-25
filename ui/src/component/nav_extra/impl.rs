use super::*;

/// `Mountable` implementation for `EuvBreadcrumb`.
impl Mountable for EuvBreadcrumb {
    /// Mounts the horizontal breadcrumb trail into `ctx`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Breadcrumb props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted breadcrumb row handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let Self { segments, .. } = self;
        let last_idx: usize = segments.len().saturating_sub(1);
        let row: NodeId = ctx.view(Style::empty().row().align(AlignItems::Center).gap(SPACE_XS));
        for (idx, seg) in segments.into_iter().enumerate() {
            let color: Color = if idx == last_idx {
                p.get_foreground()
            } else {
                p.get_muted_foreground()
            };
            let txt: NodeId = ctx.text(seg, Style::empty().font_size(13.0).color(color));
            ctx.append(row, txt);
            if idx < last_idx {
                let sep: NodeId = ctx.text(
                    BREADCRUMB_SEP.to_string(),
                    Style::empty()
                        .font_size(12.0)
                        .color(p.get_muted_foreground()),
                );
                ctx.append(row, sep);
            }
        }
        row
    }
}

/// `Mountable` implementation for `EuvPagination`.
impl Mountable for EuvPagination {
    /// Mounts Prev, page readout, and Next controls bound reactively to `current`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Pagination props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted pagination row handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let tokens: ThemeTokens = self.get_tokens();
        let p: ThemePalette = tokens.get_palette();
        let current: Signal<usize> = self.get_current();
        let total: usize = self.get_total_pages();
        let row: NodeId = ctx.view(Style::empty().row().align(AlignItems::Center).gap(SPACE_SM));
        let mut prev_cfg: EuvButton =
            EuvButton::new(PAGE_PREV_LABEL.to_string(), tokens).on_click(move |_: &UiEvent| {
                let _: bool = current.update(|v: &mut usize| *v = v.saturating_sub(1).max(1));
            });
        prev_cfg
            .set_variant(EuvButtonVariant::Outline)
            .set_size(EuvButtonSize::Small);
        let prev_btn: NodeId = prev_cfg.mount(ctx);
        let info: NodeId = ctx.reactive_text(
            Style::empty()
                .font_size(13.0)
                .font_weight(FontWeight::Medium)
                .color(p.get_foreground()),
            move || format!("{} / {total}", current.get()),
        );
        let mut next_cfg: EuvButton =
            EuvButton::new(PAGE_NEXT_LABEL.to_string(), tokens).on_click(move |_: &UiEvent| {
                let _: bool = current.update(|v: &mut usize| *v = (*v + 1).min(total));
            });
        next_cfg
            .set_variant(EuvButtonVariant::Outline)
            .set_size(EuvButtonSize::Small);
        let next_btn: NodeId = next_cfg.mount(ctx);
        ctx.append(row, prev_btn);
        ctx.append(row, info);
        ctx.append(row, next_btn);
        row
    }
}

/// `Mountable` implementation for `EuvSteps`.
impl Mountable for EuvSteps {
    /// Mounts numbered step indicators updating highlight styles reactively.
    ///
    /// # Arguments
    ///
    /// - `Self` - Steps props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted steps row handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let active: Signal<usize> = self.get_active();
        let Self { labels, .. } = self;
        let row: NodeId = ctx.view(
            Style::empty()
                .row()
                .width(Length::Percent(100.0))
                .gap(SPACE_MD),
        );
        for (idx, lbl) in labels.into_iter().enumerate() {
            let step_box: NodeId = ctx.view(
                Style::empty()
                    .row()
                    .align(AlignItems::Center)
                    .gap(SPACE_SM)
                    .cursor(CursorIcon::Pointer),
            );
            let badge: NodeId = ctx.view(Style::empty());
            let num: NodeId = ctx.text(
                (idx + 1).to_string(),
                Style::empty()
                    .font_size(12.0)
                    .font_weight(FontWeight::Bold)
                    .color(p.get_primary_foreground()),
            );
            ctx.append(badge, num);
            let handle: SceneHandle = ctx.get_scene().clone();
            let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
                handle.bind_style(badge, move || {
                    let done: bool = active.get() >= idx;
                    Style::empty()
                        .row()
                        .width(Length::Px(24.0))
                        .height(Length::Px(24.0))
                        .justify(JustifyContent::Center)
                        .align(AlignItems::Center)
                        .radius(12.0)
                        .background(if done {
                            p.get_primary()
                        } else {
                            p.get_secondary()
                        })
                })
            });
            ctx.get_scene().with_write(
                |tree: &mut SceneTree| {
                    tree.attach_scope(badge, scope);
                },
                (),
            );
            ctx.on_click(step_box, move |_: &UiEvent| {
                let _: bool = active.set_if_changed(idx);
            });
            let txt: NodeId = ctx.text(
                lbl,
                Style::empty()
                    .font_size(13.0)
                    .font_weight(FontWeight::Medium)
                    .color(p.get_foreground()),
            );
            ctx.append(step_box, badge);
            ctx.append(step_box, txt);
            ctx.append(row, step_box);
        }
        row
    }
}

/// `Mountable` implementation for `EuvAccordion`.
impl Mountable for EuvAccordion {
    /// Mounts accordion headers and reactively toggled content panels.
    ///
    /// # Arguments
    ///
    /// - `Self` - Accordion props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted accordion handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let open_index: Signal<Option<usize>> = self.get_open_index();
        let Self { items, .. } = self;
        let col: NodeId = ctx.view(
            Style::empty()
                .column()
                .width(Length::Percent(100.0))
                .gap(SPACE_XS),
        );
        for (idx, item) in items.into_iter().enumerate() {
            let section: NodeId = ctx.view(
                Style::empty()
                    .column()
                    .width(Length::Percent(100.0))
                    .background(p.get_card())
                    .border(g.get_border_width(), p.get_border())
                    .radius(g.get_radius_md()),
            );
            let header: NodeId = ctx.view(
                Style::empty()
                    .row()
                    .width(Length::Percent(100.0))
                    .padding(Edges::symmetric(SPACE_SM, SPACE_MD))
                    .cursor(CursorIcon::Pointer),
            );
            let title: NodeId = ctx.text(
                item.get_title().clone(),
                Style::empty()
                    .font_size(14.0)
                    .font_weight(FontWeight::SemiBold)
                    .color(p.get_foreground()),
            );
            ctx.append(header, title);
            ctx.on_click(header, move |_: &UiEvent| {
                let _: bool = open_index.update(|cur: &mut Option<usize>| {
                    *cur = if *cur == Some(idx) { None } else { Some(idx) };
                });
            });
            let body: NodeId = ctx.view(Style::empty());
            let body_txt: NodeId = ctx.text(
                item.get_content().clone(),
                Style::empty()
                    .font_size(13.0)
                    .color(p.get_muted_foreground()),
            );
            ctx.append(body, body_txt);
            let handle: SceneHandle = ctx.get_scene().clone();
            let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
                handle.bind_style(body, move || {
                    let disp: Display = if open_index.get() == Some(idx) {
                        Display::Flex
                    } else {
                        Display::None
                    };
                    Style::empty()
                        .column()
                        .display(disp)
                        .width(Length::Percent(100.0))
                        .padding(Edges::symmetric(SPACE_SM, SPACE_MD))
                })
            });
            ctx.get_scene().with_write(
                |tree: &mut SceneTree| {
                    tree.attach_scope(body, scope);
                },
                (),
            );
            ctx.append(section, header);
            ctx.append(section, body);
            ctx.append(col, section);
        }
        col
    }
}

/// `Mountable` implementation for `EuvToc`.
impl Mountable for EuvToc {
    /// Mounts the TOC heading list with reactive active highlighting.
    ///
    /// # Arguments
    ///
    /// - `Self` - TOC props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted TOC panel handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let active: Signal<usize> = self.get_active();
        let Self { headings, .. } = self;
        let box_node: NodeId = ctx.view(
            Style::empty()
                .column()
                .width(Length::Percent(100.0))
                .padding(Edges::all(SPACE_MD))
                .gap(SPACE_XS)
                .background(p.get_muted())
                .border(g.get_border_width(), p.get_border())
                .radius(g.get_radius_md()),
        );
        let title: NodeId = ctx.text(
            TOC_HEADING_DEFAULT.to_string(),
            Style::empty()
                .font_size(12.0)
                .font_weight(FontWeight::Bold)
                .color(p.get_muted_foreground()),
        );
        ctx.append(box_node, title);
        for (idx, h) in headings.into_iter().enumerate() {
            let item: NodeId = ctx.view(Style::empty());
            let txt: NodeId = ctx.text(h, Style::empty().font_size(13.0).color(p.get_foreground()));
            ctx.append(item, txt);
            let handle: SceneHandle = ctx.get_scene().clone();
            let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
                handle.bind_style(item, move || {
                    let is_cur: bool = active.get() == idx;
                    Style::empty()
                        .row()
                        .width(Length::Percent(100.0))
                        .padding(Edges::symmetric(SPACE_XS, SPACE_SM))
                        .radius(g.get_radius_sm())
                        .background(if is_cur {
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
                let _: bool = active.set_if_changed(idx);
            });
            ctx.append(box_node, item);
        }
        box_node
    }
}
