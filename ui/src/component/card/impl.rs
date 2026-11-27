use super::*;

/// Child composition helpers for `EuvCard`.
impl EuvCard {
    /// Sets the card subtitle text fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source card.
    /// - `impl Into<String>` - Muted subtitle string.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated card.
    #[must_use]
    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.set_subtitle(subtitle.into());
        self
    }

    /// Appends a child `NodeId` to the card body.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source card.
    /// - `NodeId` - Mounted child node.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated card.
    #[must_use]
    pub fn child(mut self, child: NodeId) -> Self {
        self.get_mut_children().push(child);
        self
    }

    /// Controls whether text inside the card is selectable.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source card.
    /// - `bool` - `true` to enable text selection across the card subtree.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated card.
    #[must_use]
    pub fn selectable(mut self, enabled: bool) -> Self {
        self.set_selectable(enabled);
        self
    }
}

/// Declarative `#[component]` implementation for `EuvCard`.
#[component]
/// Inherent implementation of [`EuvCard`].
impl EuvCard {
    /// Renders `EuvCard` into the ambient framework-managed `Context`.
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
        let g: ThemeGeometry = tokens.get_geometry();
        let is_selectable: bool = self.get_selectable();
        let Self {
            title,
            subtitle,
            children,
            ..
        } = self;
        let root: NodeId = view! {
            view {
                style: tokens.card_style()
                width: Length::Percent(100.0)
                selectable: is_selectable
                if !title.is_empty() {
                    text {
                        width: Length::Percent(100.0)
                        font_size: g.get_font_heading()
                        font_weight: FontWeight::SemiBold
                        color: p.get_card_foreground()
                        { title.as_str() }
                    }
                }
                if !subtitle.is_empty() {
                    text {
                        width: Length::Percent(100.0)
                        font_size: 13.0
                        color: p.get_muted_foreground()
                        { subtitle.as_str() }
                    }
                }
            }
        };
        if !children.is_empty() {
            let mut ctx: Context = current_context();
            ctx.append_many(root, children);
        }
        root
    }
}

/// Declarative `#[component]` implementation for `EuvHeader`.
#[component]
/// Inherent implementation of [`EuvHeader`].
impl EuvHeader {
    /// Renders `EuvHeader` into the ambient framework-managed `Context`.
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
        let g: ThemeGeometry = tokens.get_geometry();
        let Self {
            title, subtitle, ..
        } = self;
        let mut a11y: A11yProps = A11yProps::new(A11yRole::Heading, title.clone());
        a11y.set_heading_level(1);
        view! {
            column {
                width: Length::Percent(100.0)
                gap: SPACE_XS
                padding: Edges::new(0.0, 0.0, SPACE_MD, 0.0)
                row {
                    width: Length::Percent(100.0)
                    align: AlignItems::Center
                    gap: SPACE_SM
                    view {
                        width: Length::Px(4.0)
                        height: Length::Px(24.0)
                        radius: 99.0
                        background: p.get_primary()
                    }
                    text {
                        flex_grow: 1.0
                        font_size: g.get_font_title()
                        font_weight: FontWeight::Bold
                        color: p.get_foreground()
                        a11y: a11y
                        { title.as_str() }
                    }
                }
                if !subtitle.is_empty() {
                    text {
                        width: Length::Percent(100.0)
                        font_size: g.get_font_body()
                        color: p.get_muted_foreground()
                        { subtitle.as_str() }
                    }
                }
            }
        }
    }
}

/// Declarative `#[component]` implementation for `EuvStatCard`.
#[component]
/// Inherent implementation of [`EuvStatCard`].
impl EuvStatCard {
    /// Renders `EuvStatCard` into the ambient framework-managed `Context`.
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
        let g: ThemeGeometry = tokens.get_geometry();
        let Self {
            label,
            value,
            caption,
            ..
        } = self;
        view! {
            view {
                style: tokens.card_style()
                width: Length::Percent(100.0)
                gap: SPACE_SM
                row {
                    width: Length::Percent(100.0)
                    justify: JustifyContent::SpaceBetween
                    align: AlignItems::Center
                    text {
                        flex_shrink: 1.0
                        min_width: Length::Px(0.0)
                        font_size: 12.0
                        font_weight: FontWeight::SemiBold
                        color: p.get_muted_foreground()
                        { label.as_str() }
                    }
                    view {
                        width: Length::Px(24.0)
                        height: Length::Px(4.0)
                        flex_shrink: 0.0
                        radius: 99.0
                        background: p.get_primary()
                    }
                }
                text {
                    width: Length::Percent(100.0)
                    font_size: 26.0
                    font_weight: FontWeight::Bold
                    color: p.get_foreground()
                    { value.as_str() }
                }
                if !caption.is_empty() {
                    row {
                        max_width: Length::Percent(100.0)
                        padding: Edges::symmetric(4.0, 10.0)
                        radius: g.get_radius_sm()
                        background: p.get_primary().with_alpha(24)
                        align: AlignItems::Center
                        overflow: Overflow::Hidden
                        text {
                            max_width: Length::Percent(100.0)
                            font_size: 11.0
                            font_weight: FontWeight::SemiBold
                            color: p.get_primary()
                            { caption.as_str() }
                        }
                    }
                }
            }
        }
    }
}
