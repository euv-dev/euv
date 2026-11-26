use super::*;

/// Builder and mounting methods for `EuvPageContainer`.
impl EuvPageContainer {
    /// Appends a child node to the container.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source container.
    /// - `NodeId` - Mounted child node.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated container.
    #[must_use]
    pub fn child(mut self, node: NodeId) -> Self {
        self.get_mut_children().push(node);
        self
    }
}

/// Declarative `#[component]` implementation for `EuvPageContainer`.
#[component]
/// Inherent implementation of [`EuvPageContainer`].
impl EuvPageContainer {
    /// Renders `EuvPageContainer` into the ambient framework-managed `Context`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component descriptor reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted root scene node handle.
    fn render(&self) -> NodeId {
        let max_w: f32 = if self.get_max_width() > 0.0 {
            self.get_max_width()
        } else {
            1120.0
        };
        let Self { children, .. } = self.clone();
        view! {
            column {
                width: Length::Percent(100.0)
                max_width: Length::Px(max_w)
                padding: Edges::all(SPACE_XL)
                gap: SPACE_LG
                ..children
            }
        }
    }
}

/// Builder and mounting methods for `EuvPanel`.
impl EuvPanel {
    /// Appends a child node to the panel.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source panel.
    /// - `NodeId` - Child handle.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated panel.
    #[must_use]
    pub fn child(mut self, node: NodeId) -> Self {
        self.get_mut_children().push(node);
        self
    }
}

/// Declarative `#[component]` implementation for `EuvPanel`.
#[component]
/// Inherent implementation of [`EuvPanel`].
impl EuvPanel {
    /// Renders `EuvPanel` into the ambient framework-managed `Context`.
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
            title, children, ..
        } = self.clone();
        view! {
            column {
                width: Length::Percent(100.0)
                padding: Edges::all(SPACE_MD)
                gap: SPACE_SM
                background: p.get_muted()
                border: (g.get_border_width(), p.get_border())
                radius: g.get_radius_md()
                if !title.is_empty() {
                    text {
                        font_size: 14.0
                        font_weight: FontWeight::SemiBold
                        color: p.get_foreground()
                        { title }
                    }
                }
                ..children
            }
        }
    }
}

/// Builder methods for `EuvSpace`.
impl EuvSpace {
    /// Creates a horizontal `EuvSpace` with `gap`.
    ///
    /// # Arguments
    ///
    /// - `f32` - Gap in logical pixels.
    ///
    /// # Returns
    ///
    /// - `Self` - Horizontal spacing container.
    #[must_use]
    pub fn row(gap: f32) -> Self {
        Self::new(false, gap)
    }
    /// Creates a vertical `EuvSpace` with `gap`.
    ///
    /// # Arguments
    ///
    /// - `f32` - Gap in logical pixels.
    ///
    /// # Returns
    ///
    /// - `Self` - Vertical spacing container.
    #[must_use]
    pub fn column(gap: f32) -> Self {
        Self::new(true, gap)
    }
    /// Appends a child node.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source container.
    /// - `NodeId` - Child handle.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated container.
    #[must_use]
    pub fn child(mut self, node: NodeId) -> Self {
        self.get_mut_children().push(node);
        self
    }
}

/// Declarative `#[component]` implementation for `EuvSpace`.
#[component]
/// Inherent implementation of [`EuvSpace`].
impl EuvSpace {
    /// Renders `EuvSpace` into the ambient framework-managed `Context`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component descriptor reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted root scene node handle.
    fn render(&self) -> NodeId {
        let base: Style = if self.get_vertical() {
            Style::default().column()
        } else {
            Style::default()
                .row()
                .wrap(FlexWrap::Wrap)
                .align(AlignItems::Center)
        };
        let gap: f32 = self.get_gap();
        let Self { children, .. } = self.clone();
        view! {
            view {
                style: base
                gap: gap
                ..children
            }
        }
    }
}

/// Builder methods for `EuvGrid`.
impl EuvGrid {
    /// Appends a child cell to the grid.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source grid.
    /// - `NodeId` - Child node handle.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated grid.
    #[must_use]
    pub fn child(mut self, node: NodeId) -> Self {
        self.get_mut_children().push(node);
        self
    }
}

/// Declarative `#[component]` implementation for `EuvGrid`.
#[component]
/// Inherent implementation of [`EuvGrid`].
impl EuvGrid {
    /// Renders `EuvGrid` into the ambient framework-managed `Context`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component descriptor reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted root scene node handle.
    fn render(&self) -> NodeId {
        let cols: u16 = self.get_columns();
        let gap: f32 = self.get_gap();
        let Self { children, .. } = self.clone();
        view! {
            grid {
                columns: cols
                width: Length::Percent(100.0)
                gap: gap
                ..children
            }
        }
    }
}

/// Builder methods for `EuvScrollArea`.
impl EuvScrollArea {
    /// Appends a child node into the scroll area.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source scroll area.
    /// - `NodeId` - Child handle.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated scroll area.
    #[must_use]
    pub fn child(mut self, node: NodeId) -> Self {
        self.get_mut_children().push(node);
        self
    }
}

/// Declarative `#[component]` implementation for `EuvScrollArea`.
#[component]
/// Inherent implementation of [`EuvScrollArea`].
impl EuvScrollArea {
    /// Renders `EuvScrollArea` into the ambient framework-managed `Context`.
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
        let h: f32 = self.get_height();
        let Self { children, .. } = self.clone();
        view! {
            scroll {
                style: Style::default().column()
                width: Length::Percent(100.0)
                height: Length::Px(h)
                padding: Edges::all(SPACE_SM)
                gap: SPACE_SM
                background: p.get_card()
                border: (g.get_border_width(), p.get_border())
                radius: g.get_radius_md()
                ..children
            }
        }
    }
}

/// Declarative `#[component]` implementation for `EuvSplitter`.
#[component]
/// Inherent implementation of [`EuvSplitter`].
impl EuvSplitter {
    /// Renders `EuvSplitter` into the ambient framework-managed `Context`.
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
        let ratio: f32 = self.get_ratio_percent();
        let primary: NodeId = self.get_primary();
        let secondary: NodeId = self.get_secondary();
        view! {
            row {
                width: Length::Percent(100.0)
                column {
                    width: Length::Percent(ratio)
                    padding: Edges::all(SPACE_SM)
                    { primary }
                }
                view {
                    width: Length::Px(4.0)
                    background: p.get_border()
                }
                column {
                    flex_grow: 1.0
                    padding: Edges::all(SPACE_SM)
                    { secondary }
                }
            }
        }
    }
}

/// Builder methods for `EuvNavbar`.
impl EuvNavbar {
    /// Appends a trailing action node to the navbar.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source navbar.
    /// - `NodeId` - Action node handle.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated navbar.
    #[must_use]
    pub fn action(mut self, node: NodeId) -> Self {
        self.get_mut_actions().push(node);
        self
    }
}

/// Declarative `#[component]` implementation for `EuvNavbar`.
#[component]
/// Inherent implementation of [`EuvNavbar`].
impl EuvNavbar {
    /// Renders `EuvNavbar` into the ambient framework-managed `Context`.
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
        let Self { brand, actions, .. } = self.clone();
        view! {
            row {
                width: Length::Percent(100.0)
                height: Length::Px(MOBILE_HEADER_HEIGHT)
                padding: Edges::symmetric(SPACE_SM, SPACE_LG)
                justify: JustifyContent::SpaceBetween
                align: AlignItems::Center
                background: p.get_card()
                border: (g.get_border_width(), p.get_border())
                text {
                    font_size: 16.0
                    font_weight: FontWeight::Bold
                    color: p.get_foreground()
                    { brand }
                }
                row {
                    align: AlignItems::Center
                    gap: SPACE_SM
                    ..actions
                }
            }
        }
    }
}

/// Declarative `#[component]` implementation for `EuvFooter`.
#[component]
/// Inherent implementation of [`EuvFooter`].
impl EuvFooter {
    /// Renders `EuvFooter` into the ambient framework-managed `Context`.
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
        let Self { text, .. } = self.clone();
        view! {
            row {
                width: Length::Percent(100.0)
                padding: Edges::symmetric(SPACE_MD, SPACE_LG)
                justify: JustifyContent::Center
                background: p.get_muted()
                border: (g.get_border_width(), p.get_border())
                text {
                    font_size: 12.0
                    color: p.get_muted_foreground()
                    { text }
                }
            }
        }
    }
}

/// Declarative `#[component]` implementation for `EuvButtonGroup`.
#[component]
/// Inherent implementation of [`EuvButtonGroup`].
impl EuvButtonGroup {
    /// Renders `EuvButtonGroup` into the ambient framework-managed `Context`.
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
        let Self { children, .. } = self.clone();
        view! {
            row {
                flex_wrap: FlexWrap::Wrap
                align: AlignItems::Center
                gap: SPACE_XS
                padding: Edges::all(SPACE_XS)
                border: (g.get_border_width(), p.get_border())
                radius: g.get_radius_md()
                ..children
            }
        }
    }
}

/// Declarative `#[component]` implementation for `EuvToggle`.
#[component]
/// Inherent implementation of [`EuvToggle`].
impl EuvToggle {
    /// Renders `EuvToggle` into the ambient framework-managed `Context`.
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
        let pressed: Signal<bool> = self.get_pressed();
        let Self { label, .. } = self.clone();
        view! {
            row {
                bind_style: move || {
                    let on: bool = pressed.get();
                    Style::default()
                    .row()
                    .height(Length::Px(g.get_control_sm()))
                    .justify(JustifyContent::Center)
                    .align(AlignItems::Center)
                    .padding(Edges::symmetric(0.0, SPACE_MD))
                    .overflow(Overflow::Hidden)
                    .radius(g.get_radius_md())
                    .border(g.get_border_width(), p.get_border())
                    .background(if on { p.get_accent() } else { p.get_card() })
                    .cursor(CursorIcon::Pointer)
                }
                on_click: move |_: &UiEvent| {
                    let _: bool = pressed.update(|v: &mut bool| *v = !*v);
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
