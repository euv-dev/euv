use super::*;

/// Fluent section builder for `EuvNavItem`.
impl EuvNavItem {
    /// Assigns a sidebar category group label rendered above the first item in that group.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source navigation item.
    /// - `&str` - Category section title.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated navigation item.
    #[must_use]
    pub fn section(mut self, group: &str) -> Self {
        self.set_section(group.to_string());
        self
    }
}

/// Declarative `#[component]` implementation for `EuvSidebarNav`.
#[component]
/// Inherent implementation of [`EuvSidebarNav`].
impl EuvSidebarNav {
    /// Renders `EuvSidebarNav` into the ambient framework-managed `Context`.
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
        let active_route: Signal<String> = self.get_active_route();
        let Self { items, .. } = self.clone();
        let mut last_section: String = String::new();
        let mut entries: Vec<SidebarNavEntry> = Vec::with_capacity(items.len());
        for (idx, item) in items.into_iter().enumerate() {
            let sec: &str = item.get_section().as_str();
            let sec_header: Option<(String, f32)> =
                if !sec.is_empty() && sec != last_section.as_str() {
                    last_section = sec.to_string();
                    Some((sec.to_string(), if idx == 0 { 2.0 } else { 12.0 }))
                } else {
                    None
                };
            let target_path: String = item.get_path().clone();
            let click_path: String = target_path.clone();
            let indicator_path: String = target_path.clone();
            let label: String = item.get_label().clone();
            entries.push((sec_header, label, target_path, click_path, indicator_path));
        }
        view! {
            column {
                width: Length::Percent(100.0)
                gap: 2.0
                for (sec_header, label, target_path, click_path, indicator_path) in entries.into_iter() {
                    if let Some((sec_title, top_pad)) = sec_header {
                        row {
                            width: Length::Percent(100.0)
                            padding: Edges::new(top_pad, SPACE_SM, 4.0, SPACE_SM)
                            text {
                                font_size: 11.0
                                font_weight: FontWeight::SemiBold
                                color: p.get_muted_foreground()
                                { sec_title }
                            }
                        }
                    }
                    row {
                        bind_style: move || {
                            let is_active: bool = active_route.get() == target_path;
                            Style::default()
                            .row()
                            .width(Length::Percent(100.0))
                            .height(Length::Px(g.get_control_sm().max(34.0)))
                            .align(AlignItems::Center)
                            .gap(SPACE_SM)
                            .padding(Edges::symmetric(0.0, SPACE_SM))
                            .radius(g.get_radius_md())
                            .background(if is_active {
                                    p.get_primary().with_alpha(24)
                                } else {
                                    COLOR_TRANSPARENT
                            })
                            .cursor(CursorIcon::Pointer)
                        }
                        on_click: move |_: &UiEvent| {
                            let _: bool = active_route.set_if_changed(click_path.clone());
                        }
                        view {
                            bind_style: move || {
                                let is_active: bool = active_route.get() == indicator_path;
                                Style::default()
                                .width(Length::Px(3.5))
                                .height(Length::Px(18.0))
                                .radius(99.0)
                                .background(if is_active {
                                        p.get_primary()
                                    } else {
                                        COLOR_TRANSPARENT
                                })
                            }
                        }
                        text {
                            font_size: 13.5
                            font_weight: FontWeight::SemiBold
                            color: p.get_foreground()
                            text_overflow: TextOverflow::Clip
                            { label }
                        }
                    }
                }
            }
        }
    }
}

/// Declarative `#[component]` implementation for `EuvTabs`.
#[component]
/// Inherent implementation of [`EuvTabs`].
impl EuvTabs {
    /// Renders `EuvTabs` into the ambient framework-managed `Context`.
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
        let active: Signal<usize> = self.get_active();
        let Self { labels, .. } = self.clone();
        view! {
            row {
                flex_wrap: FlexWrap::Wrap
                width: Length::Percent(100.0)
                gap: SPACE_XS
                background: p.get_muted()
                padding: Edges::all(SPACE_XS)
                radius: g.get_radius_md()
                for (idx, label) in labels.into_iter().enumerate() {
                    row {
                        bind_style: move || {
                            let selected: bool = active.get() == idx;
                            let mut st: Style = Style::default()
                            .row()
                            .height(Length::Px(g.get_control_sm()))
                            .justify(JustifyContent::Center)
                            .align(AlignItems::Center)
                            .padding(Edges::symmetric(0.0, SPACE_MD))
                            .overflow(Overflow::Hidden)
                            .radius(g.get_radius_sm())
                            .background(if selected {
                                    p.get_card()
                                } else {
                                    COLOR_TRANSPARENT
                            })
                            .border(
                                if selected { g.get_border_width() } else { 0.0 },
                                p.get_border(),
                            )
                            .cursor(CursorIcon::Pointer);
                            if selected {
                                st = st.shadow(g.get_shadow_xs());
                            }
                            st
                        }
                        on_click: move |_: &UiEvent| {
                            let _: bool = active.set_if_changed(idx);
                        }
                        text {
                            font_size: 13.0
                            font_weight: FontWeight::Medium
                            color: p.get_foreground()
                            text_align: TextAlign::Center
                            text_overflow: TextOverflow::Ellipsis
                            { label }
                        }
                    }
                }
            }
        }
    }
}
