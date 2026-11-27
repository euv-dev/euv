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
        let ctx: Context = current_context();
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
        let mut root_st: Style = tokens.card_style().selectable(is_selectable);
        root_st.get_mut_layout().set_width(Length::Percent(100.0));
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                let root: NodeId = tree.create_view(root_st);
                let mut head_nodes: [NodeId; 2] = [NodeId::default(); 2];
                let mut head_len: usize = 0;
                if !title.is_empty() {
                    let mut title_st: Style = Style::default();
                    title_st.get_mut_layout().set_width(Length::Percent(100.0));
                    title_st
                        .get_mut_text()
                        .set_font_size(g.get_font_heading())
                        .set_weight(FontWeight::SemiBold)
                        .set_color(p.get_card_foreground());
                    head_nodes[head_len] = tree.create_text(title.clone(), title_st);
                    head_len += 1;
                }
                if !subtitle.is_empty() {
                    let mut sub_st: Style = Style::default();
                    sub_st.get_mut_layout().set_width(Length::Percent(100.0));
                    sub_st
                        .get_mut_text()
                        .set_font_size(13.0)
                        .set_color(p.get_muted_foreground());
                    head_nodes[head_len] = tree.create_text(subtitle.clone(), sub_st);
                    head_len += 1;
                }
                if head_len > 0 {
                    tree.append_children(root, &head_nodes[..head_len]);
                }
                if !children.is_empty() {
                    tree.append_children(root, children);
                }
                root
            },
            NodeId::default(),
        )
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
        let ctx: Context = current_context();
        let tokens: ThemeTokens = self.get_tokens();
        let p: ThemePalette = tokens.get_palette();
        let g: ThemeGeometry = tokens.get_geometry();
        let Self {
            title, subtitle, ..
        } = self;
        let mut a11y: A11yProps = A11yProps::new(A11yRole::Heading, title.clone());
        a11y.set_heading_level(1);
        let mut col_st: Style = Style::default();
        col_st
            .get_mut_layout()
            .set_flex_direction(FlexDirection::Column)
            .set_width(Length::Percent(100.0))
            .set_row_gap(SPACE_XS)
            .set_column_gap(SPACE_XS)
            .set_padding(Edges::new(0.0, 0.0, SPACE_MD, 0.0));
        let mut row_st: Style = Style::default();
        row_st
            .get_mut_layout()
            .set_flex_direction(FlexDirection::Row)
            .set_width(Length::Percent(100.0))
            .set_align_items(AlignItems::Center)
            .set_row_gap(SPACE_SM)
            .set_column_gap(SPACE_SM);
        let mut accent_st: Style = Style::default();
        accent_st
            .get_mut_layout()
            .set_width(Length::Px(4.0))
            .set_height(Length::Px(24.0));
        accent_st
            .get_mut_paint()
            .set_radius(CornerRadius::all(99.0))
            .set_background(Some(p.get_primary()));
        let mut title_st: Style = Style::default();
        title_st.get_mut_layout().set_flex_grow(1.0);
        title_st
            .get_mut_text()
            .set_font_size(g.get_font_title())
            .set_weight(FontWeight::Bold)
            .set_color(p.get_foreground());
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                let root: NodeId = tree.create_view(col_st);
                let header_row: NodeId = tree.create_view(row_st);
                let accent: NodeId = tree.create_view(accent_st);
                let title_id: NodeId = tree.create_text(title.clone(), title_st);
                if let Some(node) = tree.get_node_mut(title_id) {
                    node.set_a11y(Some(a11y));
                }
                tree.append_children(header_row, &[accent, title_id]);
                if !subtitle.is_empty() {
                    let mut sub_st: Style = Style::default();
                    sub_st.get_mut_layout().set_width(Length::Percent(100.0));
                    sub_st
                        .get_mut_text()
                        .set_font_size(g.get_font_body())
                        .set_color(p.get_muted_foreground());
                    let sub_id: NodeId = tree.create_text(subtitle.clone(), sub_st);
                    tree.append_children(root, &[header_row, sub_id]);
                } else {
                    tree.append_child(root, header_row);
                }
                root
            },
            NodeId::default(),
        )
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
        let ctx: Context = current_context();
        let tokens: ThemeTokens = self.get_tokens();
        let p: ThemePalette = tokens.get_palette();
        let g: ThemeGeometry = tokens.get_geometry();
        let Self {
            label,
            value,
            caption,
            ..
        } = self;
        let mut root_st: Style = tokens.card_style();
        root_st
            .get_mut_layout()
            .set_width(Length::Percent(100.0))
            .set_row_gap(SPACE_SM)
            .set_column_gap(SPACE_SM);
        let mut head_row_st: Style = Style::default();
        head_row_st
            .get_mut_layout()
            .set_flex_direction(FlexDirection::Row)
            .set_width(Length::Percent(100.0))
            .set_justify_content(JustifyContent::SpaceBetween)
            .set_align_items(AlignItems::Center);
        let mut lbl_st: Style = Style::default();
        lbl_st
            .get_mut_layout()
            .set_flex_shrink(1.0)
            .set_min_width(Length::Px(0.0));
        lbl_st
            .get_mut_text()
            .set_font_size(12.0)
            .set_weight(FontWeight::SemiBold)
            .set_color(p.get_muted_foreground());
        let mut bar_st: Style = Style::default();
        bar_st
            .get_mut_layout()
            .set_width(Length::Px(24.0))
            .set_height(Length::Px(4.0))
            .set_flex_shrink(0.0);
        bar_st
            .get_mut_paint()
            .set_radius(CornerRadius::all(99.0))
            .set_background(Some(p.get_primary()));
        let mut val_st: Style = Style::default();
        val_st.get_mut_layout().set_width(Length::Percent(100.0));
        val_st
            .get_mut_text()
            .set_font_size(26.0)
            .set_weight(FontWeight::Bold)
            .set_color(p.get_foreground());
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                let root: NodeId = tree.create_view(root_st);
                let head_row: NodeId = tree.create_view(head_row_st);
                let lbl_id: NodeId = tree.create_text(label.clone(), lbl_st);
                let bar_id: NodeId = tree.create_view(bar_st);
                tree.append_children(head_row, &[lbl_id, bar_id]);
                let val_id: NodeId = tree.create_text(value.clone(), val_st);
                if !caption.is_empty() {
                    let mut cap_row_st: Style = Style::default();
                    cap_row_st
                        .get_mut_layout()
                        .set_flex_direction(FlexDirection::Row)
                        .set_max_width(Length::Percent(100.0))
                        .set_padding(Edges::symmetric(4.0, 10.0))
                        .set_align_items(AlignItems::Center)
                        .set_overflow(Overflow::Hidden);
                    cap_row_st
                        .get_mut_paint()
                        .set_radius(CornerRadius::all(g.get_radius_sm().max(0.0)))
                        .set_background(Some(p.get_primary().with_alpha(24)));
                    let mut cap_txt_st: Style = Style::default();
                    cap_txt_st
                        .get_mut_layout()
                        .set_max_width(Length::Percent(100.0));
                    cap_txt_st
                        .get_mut_text()
                        .set_font_size(11.0)
                        .set_weight(FontWeight::SemiBold)
                        .set_color(p.get_primary());
                    let cap_row: NodeId = tree.create_view(cap_row_st);
                    let cap_txt: NodeId = tree.create_text(caption.clone(), cap_txt_st);
                    tree.append_child(cap_row, cap_txt);
                    tree.append_children(root, &[head_row, val_id, cap_row]);
                } else {
                    tree.append_children(root, &[head_row, val_id]);
                }
                root
            },
            NodeId::default(),
        )
    }
}
