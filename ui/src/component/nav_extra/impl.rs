use super::*;

/// Fluent interactive selection builder for `EuvBreadcrumb`.
impl EuvBreadcrumb {
    /// Binds a reactive signal capturing the selected breadcrumb segment index when clicked.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source breadcrumb.
    /// - `Signal<usize>` - Active segment index signal.
    ///
    /// # Returns
    ///
    /// - `Self` - Interactive breadcrumb trail.
    #[must_use]
    pub fn active(mut self, active: Signal<usize>) -> Self {
        self.set_active(Some(active));
        self
    }
}

/// Declarative `#[component]` implementation for `EuvBreadcrumb`.
#[component]
/// Inherent implementation of [`EuvBreadcrumb`].
impl EuvBreadcrumb {
    /// Renders `EuvBreadcrumb` into the ambient framework-managed `Context`.
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
        let Self {
            segments, active, ..
        } = self.clone();
        let last_idx: usize = segments.len().saturating_sub(1);
        view! {
            row {
                flex_wrap: FlexWrap::Wrap
                width: Length::Percent(100.0)
                align: AlignItems::Center
                gap: SPACE_XS
                for (idx, seg) in segments.into_iter().enumerate() {
                    if let Some(active_sig) = active {
                        row {
                            padding: Edges::symmetric(2.0, 6.0)
                            radius: g.get_radius_sm()
                            cursor: CursorIcon::Pointer
                            on_click: move |_: &UiEvent| {
                                let _: bool = active_sig.set_if_changed(idx);
                            }
                            text {
                                bind_style: move || {
                                    let is_cur: bool = active_sig.get() == idx;
                                    Style::default()
                                    .font_size(13.0)
                                    .font_weight(if is_cur {
                                        FontWeight::SemiBold
                                        } else {
                                            FontWeight::Normal
                                    })
                                    .color(if is_cur {
                                        p.get_foreground()
                                        } else {
                                            p.get_muted_foreground()
                                    })
                                }
                                { seg }
                            }
                        }
                    } else {
                        text {
                            font_size: 13.0
                            color: if idx == last_idx {
                                p.get_foreground()
                            } else {
                                p.get_muted_foreground()
                            }
                            { seg }
                        }
                    }
                    if idx < last_idx {
                        text {
                            font_size: 12.0
                            color: p.get_muted_foreground()
                            { BREADCRUMB_SEP }
                        }
                    }
                }
            }
        }
    }
}

/// Declarative `#[component]` implementation for `EuvPagination`.
#[component]
/// Inherent implementation of [`EuvPagination`].
impl EuvPagination {
    /// Renders `EuvPagination` into the ambient framework-managed `Context`.
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
        let current: Signal<usize> = self.get_current();
        let total: usize = self.get_total_pages();
        view! {
            row {
                flex_wrap: FlexWrap::Wrap
                align: AlignItems::Center
                gap: SPACE_SM
                EuvButton(PAGE_PREV_LABEL.to_string(), tokens) {
                    outline()
                    sm()
                    on_click: move |_: &UiEvent| {
                        let _: bool = current.update(|v: &mut usize| *v = v.saturating_sub(1).max(1));
                    }
                }
                text {
                    font_size: 13.0
                    font_weight: FontWeight::Medium
                    color: p.get_foreground()
                    { move || format!("{} / {total}", current.get()) }
                }
                EuvButton(PAGE_NEXT_LABEL.to_string(), tokens) {
                    outline()
                    sm()
                    on_click: move |_: &UiEvent| {
                        let _: bool = current.update(|v: &mut usize| *v = (*v + 1).min(total));
                    }
                }
            }
        }
    }
}

/// Declarative `#[component]` implementation for `EuvSteps`.
#[component]
/// Inherent implementation of [`EuvSteps`].
impl EuvSteps {
    /// Renders `EuvSteps` into the ambient framework-managed `Context`.
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
        let active: Signal<usize> = self.get_active();
        let Self { labels, .. } = self.clone();
        view! {
            row {
                flex_wrap: FlexWrap::Wrap
                width: Length::Percent(100.0)
                gap: SPACE_MD
                for (idx, lbl) in labels.into_iter().enumerate() {
                    row {
                        align: AlignItems::Center
                        gap: SPACE_SM
                        cursor: CursorIcon::Pointer
                        on_click: move |_: &UiEvent| {
                            let _: bool = active.set_if_changed(idx);
                        }
                        row {
                            bind_style: move || {
                                let done: bool = active.get() >= idx;
                                Style::default()
                                .row()
                                .width(Length::Px(24.0))
                                .height(Length::Px(24.0))
                                .justify(JustifyContent::Center)
                                .align(AlignItems::Center)
                                .radius(12.0)
                                .border(if done { 0.0 } else { 1.0 }, p.get_border())
                                .background(if done {
                                    p.get_primary()
                                    } else {
                                        p.get_secondary()
                                })
                            }
                            text {
                                bind_style: move || {
                                    let done: bool = active.get() >= idx;
                                    Style::default()
                                    .font_size(12.0)
                                    .font_weight(FontWeight::Bold)
                                    .color(if done {
                                        p.get_primary_foreground()
                                        } else {
                                            p.get_muted_foreground()
                                    })
                                }
                                { (idx + 1).to_string() }
                            }
                        }
                        text {
                            font_size: 13.0
                            font_weight: FontWeight::Medium
                            color: p.get_foreground()
                            { lbl }
                        }
                    }
                }
            }
        }
    }
}

/// Declarative `#[component]` implementation for `EuvAccordion`.
#[component]
/// Inherent implementation of [`EuvAccordion`].
impl EuvAccordion {
    /// Renders `EuvAccordion` into the ambient framework-managed `Context`.
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
        let open_index: Signal<Option<usize>> = self.get_open_index();
        let Self { items, .. } = self.clone();
        view! {
            column {
                width: Length::Percent(100.0)
                gap: SPACE_XS
                for (idx, item) in items.into_iter().enumerate() {
                    column {
                        width: Length::Percent(100.0)
                        background: p.get_card()
                        border: (g.get_border_width(), p.get_border())
                        radius: g.get_radius_md()
                        row {
                            width: Length::Percent(100.0)
                            padding: Edges::symmetric(SPACE_SM, SPACE_MD)
                            cursor: CursorIcon::Pointer
                            on_click: move |_: &UiEvent| {
                                let _: bool = open_index.update(|cur: &mut Option<usize>| {
                                    *cur = if *cur == Some(idx) { None } else { Some(idx) };
                                });
                            }
                            text {
                                font_size: 14.0
                                font_weight: FontWeight::SemiBold
                                color: p.get_foreground()
                                { item.get_title().clone() }
                            }
                        }
                        column {
                            bind_style: move || {
                                let disp: Display = if open_index.get() == Some(idx) {
                                    Display::Flex
                                } else {
                                    Display::None
                                };
                                Style::default()
                                .column()
                                .display(disp)
                                .width(Length::Percent(100.0))
                                .padding(Edges::symmetric(SPACE_SM, SPACE_MD))
                            }
                            text {
                                font_size: 13.0
                                color: p.get_muted_foreground()
                                { item.get_content().clone() }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Declarative `#[component]` implementation for `EuvToc`.
#[component]
/// Inherent implementation of [`EuvToc`].
impl EuvToc {
    /// Renders `EuvToc` into the ambient framework-managed `Context`.
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
        let Self { headings, .. } = self.clone();
        view! {
            column {
                width: Length::Percent(100.0)
                padding: Edges::all(SPACE_MD)
                gap: SPACE_XS
                background: p.get_muted()
                border: (g.get_border_width(), p.get_border())
                radius: g.get_radius_md()
                text {
                    font_size: 12.0
                    font_weight: FontWeight::Bold
                    color: p.get_muted_foreground()
                    { TOC_HEADING_DEFAULT }
                }
                for (idx, h) in headings.into_iter().enumerate() {
                    row {
                        bind_style: move || {
                            let is_cur: bool = active.get() == idx;
                            Style::default()
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
                        }
                        on_click: move |_: &UiEvent| {
                            let _: bool = active.set_if_changed(idx);
                        }
                        text {
                            font_size: 13.0
                            color: p.get_foreground()
                            { h }
                        }
                    }
                }
            }
        }
    }
}
