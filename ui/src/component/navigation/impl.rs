use super::*;

/// `Mountable` implementation for `EuvSidebarNav`.
impl Mountable for EuvSidebarNav {
    /// Mounts sidebar navigation items with reactive active highlighting.
    ///
    /// # Arguments
    ///
    /// - `Self` - Sidebar nav props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted nav list container handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let active_route: Signal<String> = self.get_active_route();
        let Self { items, .. } = self;
        let list: NodeId = ctx.view(
            Style::empty()
                .column()
                .width(Length::Percent(100.0))
                .gap(SPACE_XS),
        );
        for item in items.into_iter() {
            let target_path: String = item.get_path().clone();
            let click_path: String = target_path.clone();
            let row: NodeId = ctx.view(Style::empty());
            let lbl: NodeId = ctx.text(
                item.get_label().clone(),
                Style::empty()
                    .font_size(14.0)
                    .font_weight(FontWeight::Medium)
                    .color(p.get_foreground())
                    .text_overflow(TextOverflow::Clip),
            );
            ctx.append(row, lbl);
            let handle: SceneHandle = ctx.get_scene().clone();
            let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
                handle.bind_style(row, move || {
                    let is_active: bool = active_route.get() == target_path;
                    Style::empty()
                        .row()
                        .width(Length::Percent(100.0))
                        .height(Length::Px(g.get_control_md()))
                        .align(AlignItems::Center)
                        .padding(Edges::symmetric(0.0, SPACE_MD))
                        .radius(g.get_radius_md())
                        .background(if is_active {
                            p.get_accent()
                        } else {
                            COLOR_TRANSPARENT
                        })
                        .cursor(CursorIcon::Pointer)
                })
            });
            ctx.get_scene().with_write(
                |tree: &mut SceneTree| {
                    tree.attach_scope(row, scope);
                },
                (),
            );
            ctx.on_click(row, move |_: &UiEvent| {
                let _: bool = active_route.set_if_changed(click_path.clone());
            });
            ctx.append(list, row);
        }
        list
    }
}

/// `Mountable` implementation for `EuvTabs`.
impl Mountable for EuvTabs {
    /// Mounts the tab bar with reactive underline / active fill indicator.
    ///
    /// # Arguments
    ///
    /// - `Self` - Tabs props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted tab bar handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let active: Signal<usize> = self.get_active();
        let Self { labels, .. } = self;
        let bar: NodeId = ctx.view(
            Style::empty()
                .row()
                .wrap(FlexWrap::Wrap)
                .width(Length::Percent(100.0))
                .gap(SPACE_XS)
                .background(p.get_muted())
                .padding(Edges::all(SPACE_XS))
                .radius(g.get_radius_md()),
        );
        for (idx, label) in labels.into_iter().enumerate() {
            let tab: NodeId = ctx.view(Style::empty());
            let txt: NodeId = ctx.text(
                label,
                Style::empty()
                    .font_size(13.0)
                    .font_weight(FontWeight::Medium)
                    .color(p.get_foreground())
                    .text_overflow(TextOverflow::Clip),
            );
            ctx.append(tab, txt);
            let handle: SceneHandle = ctx.get_scene().clone();
            let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
                handle.bind_style(tab, move || {
                    let selected: bool = active.get() == idx;
                    let mut st: Style = Style::empty()
                        .row()
                        .height(Length::Px(g.get_control_sm()))
                        .justify(JustifyContent::Center)
                        .align(AlignItems::Center)
                        .padding(Edges::symmetric(0.0, SPACE_MD))
                        .radius(g.get_radius_sm())
                        .background(if selected {
                            p.get_card()
                        } else {
                            COLOR_TRANSPARENT
                        })
                        .cursor(CursorIcon::Pointer);
                    if selected {
                        st = st.shadow(Shadow::new(0.0, 1.0, 2.0, 0.0, Color::rgba(0, 0, 0, 16)));
                    }
                    st
                })
            });
            ctx.get_scene().with_write(
                |tree: &mut SceneTree| {
                    tree.attach_scope(tab, scope);
                },
                (),
            );
            ctx.on_click(tab, move |_: &UiEvent| {
                let _: bool = active.set_if_changed(idx);
            });
            ctx.append(bar, tab);
        }
        bar
    }
}
