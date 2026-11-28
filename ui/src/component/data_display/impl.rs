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
        let ctx: Context = current_context();
        let tokens: ThemeTokens = self.get_tokens();
        let p: ThemePalette = tokens.get_palette();
        let g: ThemeGeometry = tokens.get_geometry();
        let (accent, _): (Color, Color) = self.get_tone().colors(p);
        let Self { label, .. } = self;
        let mut row_st: Style = Style::default();
        row_st
            .get_mut_layout()
            .set_flex_direction(FlexDirection::Row)
            .set_height(Length::Px(24.0))
            .set_justify_content(JustifyContent::Center)
            .set_align_items(AlignItems::Center)
            .set_padding(Edges::symmetric(0.0, SPACE_SM))
            .set_overflow(Overflow::Hidden);
        row_st
            .get_mut_paint()
            .set_background(Some(p.get_muted()))
            .set_border(Border::new(g.get_border_width().max(0.0), accent))
            .set_radius(CornerRadius::all(g.get_radius_sm().max(0.0)));
        let mut txt_st: Style = Style::default();
        txt_st
            .get_mut_text()
            .set_font_size(12.0)
            .set_weight(FontWeight::Medium)
            .set_color(p.get_foreground())
            .set_align(TextAlign::Center)
            .set_overflow(TextOverflow::Ellipsis);
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                let root: NodeId = tree.create_view(row_st);
                let txt_id: NodeId = tree.create_text(label.clone(), txt_st);
                tree.append_child(root, txt_id);
                root
            },
            NodeId::default(),
        )
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
        let ctx: Context = current_context();
        let p: ThemePalette = self.get_tokens().get_palette();
        let sz: f32 = self.get_size();
        let Self { initials, .. } = self;
        let mut row_st: Style = Style::default();
        row_st
            .get_mut_layout()
            .set_flex_direction(FlexDirection::Row)
            .set_width(Length::Px(sz))
            .set_height(Length::Px(sz))
            .set_justify_content(JustifyContent::Center)
            .set_align_items(AlignItems::Center);
        row_st
            .get_mut_paint()
            .set_background(Some(p.get_primary()))
            .set_radius(CornerRadius::all((sz * 0.5).max(0.0)));
        let mut txt_st: Style = Style::default();
        txt_st
            .get_mut_text()
            .set_font_size((sz * 0.42).max(10.0))
            .set_weight(FontWeight::Bold)
            .set_color(p.get_primary_foreground())
            .set_align(TextAlign::Center);
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                let root: NodeId = tree.create_view(row_st);
                let txt_id: NodeId = tree.create_text(initials.clone(), txt_st);
                tree.append_child(root, txt_id);
                root
            },
            NodeId::default(),
        )
    }
}

/// Vector glyph constructor for `EuvIcon`.
impl EuvIcon {
    /// Creates a crisp native vector `EuvIcon` rendered directly via `DisplayList` primitives.
    ///
    /// # Arguments
    ///
    /// - `EuvIconName` - Built-in vector glyph variant.
    /// - `f32` - Square icon box size in logical pixels.
    /// - `Color` - Foreground stroke color.
    ///
    /// # Returns
    ///
    /// - `Self` - Vector icon descriptor.
    #[must_use]
    pub fn vector(icon: EuvIconName, size: f32, color: Color) -> Self {
        Self {
            symbol: String::new(),
            size,
            color,
            icon: Some(icon),
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
        let ctx: Context = current_context();
        let sz: f32 = self.get_size();
        let color: Color = self.get_color();
        if let Some(icon_name) = self.get_icon() {
            let paint_icon = move |bounds: Rect, dl: &mut DisplayList| {
                draw_euv_icon(icon_name, bounds, dl, color);
            };
            return view! {
                canvas {
                    width: Length::Px(sz)
                    height: Length::Px(sz)
                    painter: paint_icon
                }
            };
        }
        let Self { symbol, .. } = self;
        let mut row_st: Style = Style::default();
        row_st
            .get_mut_layout()
            .set_flex_direction(FlexDirection::Row)
            .set_width(Length::Px(sz))
            .set_height(Length::Px(sz))
            .set_justify_content(JustifyContent::Center)
            .set_align_items(AlignItems::Center);
        let mut txt_st: Style = Style::default();
        txt_st
            .get_mut_text()
            .set_font_size(sz * 0.78)
            .set_weight(FontWeight::SemiBold)
            .set_color(color);
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                let root: NodeId = tree.create_view(row_st);
                let txt_id: NodeId = tree.create_text(symbol.clone(), txt_st);
                tree.append_child(root, txt_id);
                root
            },
            NodeId::default(),
        )
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
        let ctx: Context = current_context();
        let tokens: ThemeTokens = self.get_tokens();
        let p: ThemePalette = tokens.get_palette();
        let g: ThemeGeometry = tokens.get_geometry();
        let Self { key, .. } = self;
        let mut row_st: Style = Style::default();
        row_st
            .get_mut_layout()
            .set_flex_direction(FlexDirection::Row)
            .set_justify_content(JustifyContent::Center)
            .set_align_items(AlignItems::Center)
            .set_padding(Edges::symmetric(2.0, 6.0));
        row_st
            .get_mut_paint()
            .set_background(Some(p.get_muted()))
            .set_border(Border::new(g.get_border_width().max(0.0), p.get_border()))
            .set_radius(CornerRadius::all(g.get_radius_sm().max(0.0)));
        let mut txt_st: Style = Style::default();
        txt_st
            .get_mut_text()
            .set_family(FontFamily::Monospace)
            .set_font_size(11.0)
            .set_color(p.get_foreground())
            .set_align(TextAlign::Center)
            .set_overflow(TextOverflow::Clip);
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                let root: NodeId = tree.create_view(row_st);
                let txt_id: NodeId = tree.create_text(key.clone(), txt_st);
                tree.append_child(root, txt_id);
                root
            },
            NodeId::default(),
        )
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
        let primary: Color = p.get_primary();
        let paint_spinner = move |bounds: Rect, dl: &mut DisplayList| {
            let cx: f32 = bounds.get_x() + bounds.get_width() * 0.5;
            let cy: f32 = bounds.get_y() + bounds.get_height() * 0.5;
            let outer_r: f32 = (bounds.get_width().min(bounds.get_height()) * 0.42).max(4.0);
            let inner_r: f32 = outer_r * 0.56;
            let stroke_w: f32 = (outer_r * 0.24).clamp(1.6, 3.2);
            for &(cos_a, sin_a, alpha) in SPINNER_SPOKES.iter() {
                dl.push(DrawCommand::Line {
                    from: Point::new(cx + cos_a * inner_r, cy + sin_a * inner_r),
                    to: Point::new(cx + cos_a * outer_r, cy + sin_a * outer_r),
                    color: primary.multiply_alpha(alpha),
                    width: stroke_w,
                });
            }
        };
        view! {
            canvas {
                width: Length::Px(sz)
                height: Length::Px(sz)
                painter: paint_spinner
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
    /// - `&[EuvTreeNode]` - Sibling tree nodes.
    /// - `usize` - Current indentation depth.
    /// - `(ThemePalette, ThemeGeometry)` - Active theme palette and geometry.
    /// - `Option<Signal<Option<String>>>` - Optional active selection signal.
    /// - `&mut Vec<(NodeId, String)>` - Collector of `(row_node, label)` pairs for O(1) selector binding.
    ///
    /// # Returns
    ///
    /// - `Vec<NodeId>` - Mounted sibling tree nodes ready for spread attachment.
    fn build_branch(
        nodes: &[EuvTreeNode],
        depth: usize,
        theme: (ThemePalette, ThemeGeometry),
        selected: Option<Signal<Option<String>>>,
        bound_rows: &mut Vec<(NodeId, String)>,
    ) -> Vec<NodeId> {
        let (palette, geom): (ThemePalette, ThemeGeometry) = theme;
        let mut out: Vec<NodeId> = Vec::with_capacity(nodes.len() * 2);
        for node in nodes.iter() {
            let has_children: bool = !node.get_children().is_empty();
            let expanded: Signal<bool> = Signal::new(true);
            let indent: f32 = (depth as f32) * 16.0;
            let label_str: &str = node.get_label().as_str();
            let lbl_click: String = node.get_label().clone();
            let on_row_click = move |_: &UiEvent| {
                if has_children {
                    let _: bool = expanded.update(|v: &mut bool| *v = !*v);
                }
                if let Some(sel_sig) = selected {
                    let _: bool = sel_sig.set_if_changed(Some(lbl_click.clone()));
                }
            };
            let label_node: NodeId = if has_children {
                let lbl_for_txt: String = node.get_label().clone();
                let branch_label = move || -> String {
                    let pfx: &str = if expanded.get() {
                        TREE_EXPAND_PREFIX
                    } else {
                        TREE_COLLAPSED_PREFIX
                    };
                    format!("{pfx}{lbl_for_txt}")
                };
                view! {
                    text {
                        font_size: 13.0
                        font_weight: FontWeight::Medium
                        color: palette.get_foreground()
                        { branch_label }
                    }
                }
            } else {
                let leaf_label: String = format!("{TREE_LEAF_PREFIX}{label_str}");
                view! {
                    text {
                        font_size: 13.0
                        color: palette.get_foreground()
                        { leaf_label }
                    }
                }
            };
            let row: NodeId = view! {
                row {
                    width: Length::Percent(100.0)
                    padding: Edges::new(SPACE_XS, SPACE_SM, SPACE_XS, SPACE_SM + indent)
                    radius: geom.get_radius_sm()
                    background: COLOR_TRANSPARENT
                    cursor: CursorIcon::Pointer
                    on_click: on_row_click
                    { label_node }
                }
            };
            if selected.is_some() {
                bound_rows.push((row, node.get_label().clone()));
            }
            out.push(row);
            if has_children {
                let child_nodes: Vec<NodeId> =
                    Self::build_branch(node.get_children(), depth + 1, theme, selected, bound_rows);
                let mut branch_base: Style = Style::default();
                branch_base
                    .get_mut_layout()
                    .set_flex_direction(FlexDirection::Column)
                    .set_width(Length::Percent(100.0));
                let branch_style = move || -> Style {
                    let mut st: Style = branch_base;
                    st.get_mut_layout().set_display(if expanded.get() {
                        Display::Flex
                    } else {
                        Display::None
                    });
                    st
                };
                let branch_box: NodeId = view! {
                    column {
                        bind_style: branch_style
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
        let tokens: ThemeTokens = self.get_tokens();
        let p: ThemePalette = tokens.get_palette();
        let g: ThemeGeometry = tokens.get_geometry();
        let Self {
            roots, selected, ..
        } = self;
        let mut bound_rows: Vec<(NodeId, String)> = Vec::new();
        let branch_nodes: Vec<NodeId> =
            Self::build_branch(roots, 0, (p, g), *selected, &mut bound_rows);
        let root: NodeId = view! {
            column {
                width: Length::Percent(100.0)
                padding: Edges::all(SPACE_SM)
                background: p.get_card()
                border: (g.get_border_width(), p.get_border())
                radius: g.get_radius_md()
                overflow: Overflow::Hidden
                ..branch_nodes
            }
        };
        if let Some(sel_sig) = *selected
            && !bound_rows.is_empty()
        {
            let ctx: Context = current_context();
            let scene: SceneHandle = ctx.get_scene().clone();
            let accent_col: Color = p.get_accent();
            let (tree_scope, _): (Scope, ()) = Scope::run(|_: Scope| {
                let sel: Rc<Selector<Option<String>>> =
                    Rc::new(Selector::new(move || sel_sig.get()));
                let sel_cleanup: Rc<Selector<Option<String>>> = Rc::clone(&sel);
                on_cleanup(move || {
                    sel_cleanup.dispose();
                });
                for (row_node, lbl_cmp) in bound_rows.into_iter() {
                    let sel_row: Rc<Selector<Option<String>>> = Rc::clone(&sel);
                    let scene_row: SceneHandle = scene.clone();
                    let mut prev_sel: bool = false;
                    let mut has_synced: bool = false;
                    let _: Effect = Effect::new(move || {
                        let is_sel: bool = sel_row.is_selected(Some(lbl_cmp.clone()));
                        if has_synced && is_sel == prev_sel {
                            return;
                        }
                        prev_sel = is_sel;
                        has_synced = true;
                        let bg: Color = if is_sel {
                            accent_col
                        } else {
                            COLOR_TRANSPARENT
                        };
                        scene_row.with_write(
                            |tree: &mut SceneTree| {
                                if let Some(node) = tree.get_node_mut(row_node) {
                                    let mut st: Style = node.get_style();
                                    st.get_mut_paint().set_background(Some(bg));
                                    node.set_style(st);
                                }
                                tree.mark_dirty(row_node, DIRTY_PAINT);
                            },
                            (),
                        );
                    });
                }
            });
            ctx.get_scene().with_write(
                |tree: &mut SceneTree| {
                    tree.attach_scope(root, tree_scope);
                },
                (),
            );
        }
        root
    }
}
