use super::*;

/// Construction and mounting for `EuvSidebarNav`.
impl EuvSidebarNav {
    /// Creates a sidebar navigation list bound to `active_route`.
    ///
    /// # Arguments
    ///
    /// - `Vec<EuvNavItem>` - Navigation destinations.
    /// - `Signal<String>` - Active route path signal.
    /// - `ThemeTokens` - Active design tokens.
    ///
    /// # Returns
    ///
    /// - `Self` - Constructed sidebar nav.
    #[must_use]
    pub fn create(
        items: Vec<EuvNavItem>,
        active_route: Signal<String>,
        tokens: ThemeTokens,
    ) -> Self {
        Self {
            items,
            active_route,
            tokens,
        }
    }
}

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
                    .color(p.get_foreground()),
            );
            ctx.append(row, lbl);
            let handle: SceneHandle = ctx.get_scene().clone();
            let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
                handle.bind_style(row, move || {
                    let is_active: bool = active_route.get() == target_path;
                    Style::empty()
                        .row()
                        .width(Length::Percent(100.0))
                        .padding(Edges::symmetric(SPACE_SM, SPACE_MD))
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

/// Construction and mounting for `EuvTabs`.
impl EuvTabs {
    /// Creates a horizontal tab bar bound to `active`.
    ///
    /// # Arguments
    ///
    /// - `Vec<String>` - Ordered tab titles.
    /// - `Signal<usize>` - Active tab index signal.
    /// - `ThemeTokens` - Active design tokens.
    ///
    /// # Returns
    ///
    /// - `Self` - Constructed tab bar.
    #[must_use]
    pub fn create(labels: Vec<String>, active: Signal<usize>, tokens: ThemeTokens) -> Self {
        Self {
            labels,
            active,
            tokens,
        }
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
                .width(Length::Percent(100.0))
                .gap(SPACE_SM)
                .border(g.get_border_width(), p.get_border())
                .padding(Edges::all(SPACE_XS))
                .radius(g.get_radius_md()),
        );
        for (idx, label) in labels.into_iter().enumerate() {
            let tab: NodeId = ctx.view(Style::empty());
            let txt: NodeId = ctx.text(
                label,
                Style::empty()
                    .font_size(14.0)
                    .font_weight(FontWeight::Medium)
                    .color(p.get_foreground()),
            );
            ctx.append(tab, txt);
            let handle: SceneHandle = ctx.get_scene().clone();
            let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
                handle.bind_style(tab, move || {
                    let selected: bool = active.get() == idx;
                    Style::empty()
                        .row()
                        .padding(Edges::symmetric(SPACE_SM, SPACE_MD))
                        .radius(g.get_radius_sm())
                        .background(if selected {
                            p.get_secondary()
                        } else {
                            COLOR_TRANSPARENT
                        })
                        .cursor(CursorIcon::Pointer)
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
