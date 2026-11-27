use super::*;

/// Declarative `#[component]` implementation for `EuvTag`.
#[component]
/// Inherent implementation of [`EuvTag`].
impl EuvTag {
    /// Renders `EuvTag` into the ambient framework-managed `Context`.
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
        let (accent, _): (Color, Color) = self.get_tone().colors(p);
        let Self { label, .. } = self;
        view! {
            row {
                height: Length::Px(24.0)
                justify: JustifyContent::Center
                align: AlignItems::Center
                padding: Edges::symmetric(0.0, SPACE_SM)
                overflow: Overflow::Hidden
                background: p.get_muted()
                border: (g.get_border_width(), accent)
                radius: g.get_radius_sm()
                text {
                    font_size: 12.0
                    font_weight: FontWeight::Medium
                    color: p.get_foreground()
                    text_align: TextAlign::Center
                    text_overflow: TextOverflow::Ellipsis
                    { label.as_str() }
                }
            }
        }
    }
}

/// Declarative `#[component]` implementation for `EuvAvatar`.
#[component]
/// Inherent implementation of [`EuvAvatar`].
impl EuvAvatar {
    /// Renders `EuvAvatar` into the ambient framework-managed `Context`.
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
        let sz: f32 = self.get_size();
        let Self { initials, .. } = self;
        view! {
            row {
                width: Length::Px(sz)
                height: Length::Px(sz)
                justify: JustifyContent::Center
                align: AlignItems::Center
                background: p.get_primary()
                radius: sz * 0.5
                text {
                    font_size: (sz * 0.42).max(10.0)
                    font_weight: FontWeight::Bold
                    color: p.get_primary_foreground()
                    text_align: TextAlign::Center
                    { initials.as_str() }
                }
            }
        }
    }
}

/// Declarative `#[component]` implementation for `EuvIcon`.
#[component]
/// Inherent implementation of [`EuvIcon`].
impl EuvIcon {
    /// Renders `EuvIcon` into the ambient framework-managed `Context`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component descriptor reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted root scene node handle.
    fn render(&self) -> NodeId {
        let sz: f32 = self.get_size();
        let color: Color = self.get_color();
        let Self { symbol, .. } = self;
        view! {
            row {
                width: Length::Px(sz)
                height: Length::Px(sz)
                justify: JustifyContent::Center
                align: AlignItems::Center
                text {
                    font_size: sz * 0.78
                    font_weight: FontWeight::SemiBold
                    color: color
                    { symbol.as_str() }
                }
            }
        }
    }
}

/// Declarative `#[component]` implementation for `EuvKbd`.
#[component]
/// Inherent implementation of [`EuvKbd`].
impl EuvKbd {
    /// Renders `EuvKbd` into the ambient framework-managed `Context`.
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
        let Self { key, .. } = self;
        view! {
            row {
                justify: JustifyContent::Center
                align: AlignItems::Center
                padding: Edges::symmetric(2.0, 6.0)
                background: p.get_muted()
                border: (g.get_border_width(), p.get_border())
                radius: g.get_radius_sm()
                text {
                    font_family: FontFamily::Monospace
                    font_size: 11.0
                    color: p.get_foreground()
                    text_align: TextAlign::Center
                    text_overflow: TextOverflow::Clip
                    { key.as_str() }
                }
            }
        }
    }
}

/// Declarative `#[component]` implementation for `EuvTypography`.
#[component]
/// Inherent implementation of [`EuvTypography`].
impl EuvTypography {
    /// Renders `EuvTypography` into the ambient framework-managed `Context`.
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
        let variant: EuvTypographyVariant = self.get_variant();
        let Self { text, .. } = self;
        let mut style: Style = Style::default();
        let ts: &mut TextStyle = style.get_mut_text();
        match variant {
            EuvTypographyVariant::H1 => {
                ts.set_font_size(g.get_font_title())
                    .set_weight(FontWeight::Bold)
                    .set_color(p.get_foreground());
            }
            EuvTypographyVariant::H2 => {
                ts.set_font_size(g.get_font_heading())
                    .set_weight(FontWeight::Bold)
                    .set_color(p.get_foreground());
            }
            EuvTypographyVariant::H3 => {
                ts.set_font_size(15.0)
                    .set_weight(FontWeight::SemiBold)
                    .set_color(p.get_foreground());
            }
            EuvTypographyVariant::Body => {
                ts.set_font_size(g.get_font_body())
                    .set_color(p.get_foreground());
            }
            EuvTypographyVariant::Caption => {
                ts.set_font_size(12.0).set_color(p.get_muted_foreground());
            }
        }
        view! {
            text {
                style: style
                { text.as_str() }
            }
        }
    }
}

/// Declarative `#[component]` implementation for `EuvEmpty`.
#[component]
/// Inherent implementation of [`EuvEmpty`].
impl EuvEmpty {
    /// Renders `EuvEmpty` into the ambient framework-managed `Context`.
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
            title, description, ..
        } = self;
        view! {
            column {
                width: Length::Percent(100.0)
                align: AlignItems::Center
                padding: Edges::all(SPACE_2XL)
                gap: SPACE_SM
                border: (g.get_border_width(), p.get_border())
                radius: g.get_radius_lg()
                text {
                    font_size: 20.0
                    color: p.get_muted_foreground()
                    { EMPTY_ICON_GLYPH }
                }
                text {
                    font_size: 15.0
                    font_weight: FontWeight::SemiBold
                    color: p.get_foreground()
                    { title.as_str() }
                }
                text {
                    font_size: 13.0
                    color: p.get_muted_foreground()
                    { description.as_str() }
                }
            }
        }
    }
}

/// Declarative `#[component]` implementation for `EuvSkeleton`.
#[component]
/// Inherent implementation of [`EuvSkeleton`].
impl EuvSkeleton {
    /// Renders `EuvSkeleton` into the ambient framework-managed `Context`.
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
        let w: f32 = self.get_width();
        let h: f32 = self.get_height();
        view! {
            view {
                width: Length::Px(w)
                max_width: Length::Percent(100.0)
                height: Length::Px(h)
                background: p.get_secondary()
                radius: g.get_radius_sm()
            }
        }
    }
}

/// Declarative `#[component]` implementation for `EuvSpinner`.
#[component]
/// Inherent implementation of [`EuvSpinner`].
impl EuvSpinner {
    /// Renders `EuvSpinner` into the ambient framework-managed `Context`.
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
        let sz: f32 = self.get_size();
        view! {
            canvas {
                width: Length::Px(sz)
                height: Length::Px(sz)
                painter: move |bounds: Rect, dl: &mut DisplayList| {
                    let cx: f32 = bounds.get_x() + bounds.get_width() * 0.5;
                    let cy: f32 = bounds.get_y() + bounds.get_height() * 0.5;
                    let outer_r: f32 = (bounds.get_width().min(bounds.get_height()) * 0.42).max(4.0);
                    let inner_r: f32 = outer_r * 0.56;
                    let stroke_w: f32 = (outer_r * 0.24).clamp(1.6, 3.2);
                    let spokes: usize = 8;
                    for i in 0..spokes {
                        let angle: f32 = (i as f32) * (TAU / (spokes as f32));
                        let (sin_a, cos_a): (f32, f32) = angle.sin_cos();
                        let alpha: f32 = 0.22 + 0.78 * ((i + 1) as f32 / (spokes as f32));
                        dl.push(DrawCommand::Line {
                            from: Point::new(cx + cos_a * inner_r, cy + sin_a * inner_r),
                            to: Point::new(cx + cos_a * outer_r, cy + sin_a * outer_r),
                            color: p.get_primary().multiply_alpha(alpha),
                            width: stroke_w,
                        });
                    }
                }
            }
        }
    }
}

/// Declarative `#[component]` implementation for `EuvTimeline`.
#[component]
/// Inherent implementation of [`EuvTimeline`].
impl EuvTimeline {
    /// Renders `EuvTimeline` into the ambient framework-managed `Context`.
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
        let Self { items, .. } = self;
        view! {
            column {
                width: Length::Percent(100.0)
                gap: SPACE_MD
                for item in items.iter() {
                    row {
                        width: Length::Percent(100.0)
                        gap: SPACE_MD
                        view {
                            width: Length::Px(10.0)
                            height: Length::Px(10.0)
                            background: p.get_primary()
                            radius: 5.0
                        }
                        column {
                            flex_grow: 1.0
                            gap: 2.0
                            text {
                                font_size: 14.0
                                font_weight: FontWeight::SemiBold
                                color: p.get_foreground()
                                { item.get_title().as_str() }
                            }
                            text {
                                font_size: 11.0
                                color: p.get_muted_foreground()
                                { item.get_timestamp().as_str() }
                            }
                            text {
                                font_size: 13.0
                                color: p.get_muted_foreground()
                                { item.get_detail().as_str() }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Declarative `#[component]` implementation for `EuvList`.
#[component]
/// Inherent implementation of [`EuvList`].
impl EuvList {
    /// Renders `EuvList` into the ambient framework-managed `Context`.
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
        let Self { items, .. } = self;
        view! {
            column {
                width: Length::Percent(100.0)
                background: p.get_card()
                border: (g.get_border_width(), p.get_border())
                radius: g.get_radius_md()
                overflow: Overflow::Hidden
                for (idx, item) in items.iter().enumerate() {
                    if idx > 0 {
                        view {
                            width: Length::Percent(100.0)
                            height: Length::Px(g.get_border_width().max(1.0))
                            background: p.get_border()
                        }
                    }
                    row {
                        width: Length::Percent(100.0)
                        padding: Edges::symmetric(SPACE_SM, SPACE_MD)
                        text {
                            font_size: 13.0
                            color: p.get_foreground()
                            { item.as_str() }
                        }
                    }
                }
            }
        }
    }
}

/// Builder and recursive rendering methods for `EuvTree`.
impl EuvTree {
    /// Binds an optional controlled signal receiving the most recently selected tree node label.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source tree component.
    /// - `Signal<Option<String>>` - Selected node label signal.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated interactive tree.
    #[must_use]
    pub fn selected(mut self, selected: Signal<Option<String>>) -> Self {
        self.set_selected(Some(selected));
        self
    }

    /// Recursively builds interactive indented tree row nodes for `nodes`.
    ///
    /// # Arguments
    ///
    /// - `&mut Context` - Active build context.
    /// - `&[EuvTreeNode]` - Sibling tree nodes.
    /// - `usize` - Current indentation depth.
    /// - `(ThemePalette, ThemeGeometry)` - Active theme palette and geometry.
    /// - `Option<Signal<Option<String>>>` - Optional active selection signal.
    ///
    /// # Returns
    ///
    /// - `Vec<NodeId>` - Mounted sibling tree nodes ready for spread attachment.
    fn build_branch(
        ctx: &mut Context,
        nodes: &[EuvTreeNode],
        depth: usize,
        theme: (ThemePalette, ThemeGeometry),
        selected: Option<Signal<Option<String>>>,
    ) -> Vec<NodeId> {
        let (palette, geom): (ThemePalette, ThemeGeometry) = theme;
        let mut out: Vec<NodeId> = Vec::with_capacity(nodes.len() * 2);
        for node in nodes.iter() {
            let has_children: bool = !node.get_children().is_empty();
            let expanded: Signal<bool> = Signal::new(true);
            let indent: f32 = (depth as f32) * 16.0;
            let label_str: String = node.get_label().clone();
            let lbl_for_txt: String = label_str.clone();
            let lbl_click: String = label_str.clone();
            let row: NodeId = view! {
                row {
                    width: Length::Percent(100.0)
                    padding: Edges::new(SPACE_XS, SPACE_SM, SPACE_XS, SPACE_SM + indent)
                    radius: geom.get_radius_sm()
                    cursor: CursorIcon::Pointer
                    on_click: move |_: &UiEvent| {
                        if has_children {
                            let _: bool = expanded.update(|v: &mut bool| *v = !*v);
                        }
                        if let Some(sel_sig) = selected {
                            let _: bool = sel_sig.set_if_changed(Some(lbl_click.clone()));
                        }
                    }
                    if has_children {
                        text {
                            font_size: 13.0
                            font_weight: FontWeight::Medium
                            color: palette.get_foreground()
                            { move || {
                                let pfx: &str = if expanded.get() {
                                    TREE_EXPAND_PREFIX
                                } else {
                                    TREE_COLLAPSED_PREFIX
                                };
                                format!("{pfx}{lbl_for_txt}")
                            } }
                        }
                    } else {
                        text {
                            font_size: 13.0
                            color: palette.get_foreground()
                            { format!("{TREE_LEAF_PREFIX}{label_str}") }
                        }
                    }
                }
            };
            if let Some(sel_sig) = selected {
                let lbl_cmp: String = node.get_label().clone();
                ctx.bind_style(row, move || {
                    let is_sel: bool = sel_sig.get().as_deref() == Some(lbl_cmp.as_str());
                    Style::default()
                        .row()
                        .width(Length::Percent(100.0))
                        .padding(Edges::new(SPACE_XS, SPACE_SM, SPACE_XS, SPACE_SM + indent))
                        .radius(geom.get_radius_sm())
                        .background(if is_sel {
                            palette.get_accent()
                        } else {
                            COLOR_TRANSPARENT
                        })
                        .cursor(CursorIcon::Pointer)
                });
            }
            out.push(row);
            if has_children {
                let child_nodes: Vec<NodeId> =
                    Self::build_branch(ctx, node.get_children(), depth + 1, theme, selected);
                let branch_box: NodeId = view! {
                    column {
                        bind_style: move || {
                            Style::default()
                            .column()
                            .display(if expanded.get() {
                                Display::Flex
                                } else {
                                    Display::None
                            })
                            .width(Length::Percent(100.0))
                        }
                        ..child_nodes
                    }
                };
                out.push(branch_box);
            }
        }
        out
    }
}

/// Declarative `#[component]` implementation for `EuvTree`.
#[component]
/// Inherent implementation of [`EuvTree`].
impl EuvTree {
    /// Renders `EuvTree` into the ambient framework-managed `Context`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component descriptor reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted root scene node handle.
    fn render(&self) -> NodeId {
        let mut ctx: Context = current_context();
        let tokens: ThemeTokens = self.get_tokens();
        let p: ThemePalette = tokens.get_palette();
        let g: ThemeGeometry = tokens.get_geometry();
        let Self {
            roots, selected, ..
        } = self;
        let branch_nodes: Vec<NodeId> = Self::build_branch(&mut ctx, roots, 0, (p, g), *selected);
        view! {
            column {
                width: Length::Percent(100.0)
                padding: Edges::all(SPACE_SM)
                background: p.get_card()
                border: (g.get_border_width(), p.get_border())
                radius: g.get_radius_md()
                overflow: Overflow::Hidden
                ..branch_nodes
            }
        }
    }
}
