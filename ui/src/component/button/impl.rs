use super::*;

/// Styling and click handler helpers for `EuvButton`.
impl EuvButton {
    /// Attaches a click callback invoked when the button is activated.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source button.
    /// - `F` - Click callback closure.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated button.
    #[must_use]
    pub fn on_click<F>(mut self, callback: F) -> Self
    where
        F: FnMut(&UiEvent) + 'static,
    {
        self.set_on_click(Some(Rc::new(RefCell::new(callback))));
        self
    }

    /// Sets the visual emphasis variant fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source button.
    /// - `EuvButtonVariant` - Visual variant.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated button.
    #[must_use]
    pub fn variant(mut self, variant: EuvButtonVariant) -> Self {
        self.set_variant(variant);
        self
    }

    /// Sets the control size tier fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source button.
    /// - `EuvButtonSize` - Unified control size.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated button.
    #[must_use]
    pub fn size(mut self, size: EuvButtonSize) -> Self {
        self.set_size(size);
        self
    }

    /// Applies the `Secondary` button variant fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source button.
    ///
    /// # Returns
    ///
    /// - `Self` - Secondary button.
    #[must_use]
    pub fn secondary(self) -> Self {
        self.variant(EuvButtonVariant::Secondary)
    }

    /// Applies the `Outline` button variant fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source button.
    ///
    /// # Returns
    ///
    /// - `Self` - Outline button.
    #[must_use]
    pub fn outline(self) -> Self {
        self.variant(EuvButtonVariant::Outline)
    }

    /// Applies the `Ghost` button variant fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source button.
    ///
    /// # Returns
    ///
    /// - `Self` - Ghost button.
    #[must_use]
    pub fn ghost(self) -> Self {
        self.variant(EuvButtonVariant::Ghost)
    }

    /// Applies the `Destructive` button variant fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source button.
    ///
    /// # Returns
    ///
    /// - `Self` - Destructive button.
    #[must_use]
    pub fn destructive(self) -> Self {
        self.variant(EuvButtonVariant::Destructive)
    }

    /// Applies the `Link` button variant fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source button.
    ///
    /// # Returns
    ///
    /// - `Self` - Link button.
    #[must_use]
    pub fn link(self) -> Self {
        self.variant(EuvButtonVariant::Link)
    }

    /// Applies the `Small` unified control height fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source button.
    ///
    /// # Returns
    ///
    /// - `Self` - Small button.
    #[must_use]
    pub fn sm(self) -> Self {
        self.size(EuvButtonSize::Small)
    }

    /// Applies the `Large` unified control height fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source button.
    ///
    /// # Returns
    ///
    /// - `Self` - Large button.
    #[must_use]
    pub fn lg(self) -> Self {
        self.size(EuvButtonSize::Large)
    }

    /// Applies the square `Icon` control preset fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source button.
    ///
    /// # Returns
    ///
    /// - `Self` - Icon button.
    #[must_use]
    pub fn icon(self) -> Self {
        self.size(EuvButtonSize::Icon)
    }

    /// Sets the disabled interaction state fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source button.
    /// - `bool` - Whether the button is disabled.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated button.
    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.set_disabled(disabled);
        self
    }

    /// Resolves the deterministic compiled `Style` for this button.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Button configuration.
    ///
    /// # Returns
    ///
    /// - `Style` - Compiled cross-platform style.
    #[inline(always)]
    #[must_use]
    pub fn resolved_style(&self) -> Style {
        let tokens: ThemeTokens = self.get_tokens();
        let p: ThemePalette = tokens.get_palette();
        let g: ThemeGeometry = tokens.get_geometry();
        let size: EuvButtonSize = self.get_size();
        let disabled: bool = self.get_disabled();
        let (height, pad_x, font_sz, radius): (f32, f32, f32, f32) = match size {
            EuvButtonSize::Small => (g.get_control_sm(), SPACE_SM, 12.0, g.get_radius_sm()),
            EuvButtonSize::Default => (g.get_control_md(), SPACE_MD, 13.0, g.get_radius_md()),
            EuvButtonSize::Large => (g.get_control_lg(), SPACE_XL, 14.0, g.get_radius_md()),
            EuvButtonSize::Icon => (g.get_control_md(), 0.0, 13.0, g.get_radius_md()),
        };
        let mut style: Style = Style::default();
        let layout: &mut LayoutStyle = style.get_mut_layout();
        layout
            .set_flex_direction(FlexDirection::Row)
            .set_justify_content(JustifyContent::Center)
            .set_align_items(AlignItems::Center)
            .set_height(Length::Px(height))
            .set_padding(Edges::symmetric(0.0, pad_x))
            .set_overflow(Overflow::Hidden);
        if size == EuvButtonSize::Icon {
            layout.set_width(Length::Px(height));
        }
        let paint: &mut PaintStyle = style.get_mut_paint();
        paint
            .set_radius(CornerRadius::all(radius.max(0.0)))
            .set_focus_ring(Some(Border::new(g.get_ring_width(), p.get_ring())))
            .set_cursor(if disabled {
                CursorIcon::NotAllowed
            } else {
                CursorIcon::Pointer
            });
        let text_col: Color = match self.get_variant() {
            EuvButtonVariant::Primary => {
                paint
                    .set_background(Some(p.get_primary()))
                    .set_shadow(Some(g.get_shadow_xs()));
                p.get_primary_foreground()
            }
            EuvButtonVariant::Secondary => {
                paint.set_background(Some(p.get_secondary()));
                p.get_secondary_foreground()
            }
            EuvButtonVariant::Destructive => {
                paint
                    .set_background(Some(p.get_destructive()))
                    .set_shadow(Some(g.get_shadow_xs()));
                p.get_destructive_foreground()
            }
            EuvButtonVariant::Outline => {
                paint
                    .set_background(Some(p.get_card()))
                    .set_border(Border::new(g.get_border_width().max(0.0), p.get_border()))
                    .set_shadow(Some(g.get_shadow_xs()));
                p.get_foreground()
            }
            EuvButtonVariant::Ghost => p.get_foreground(),
            EuvButtonVariant::Link => p.get_primary(),
        };
        if disabled {
            paint.set_opacity(0.5);
        }
        style
            .get_mut_text()
            .set_font_size(font_sz)
            .set_weight(FontWeight::Medium)
            .set_color(text_col);
        style
    }
}

/// Declarative `#[component]` implementation for `EuvButton`.
#[component]
/// Inherent implementation of [`EuvButton`].
impl EuvButton {
    /// Renders `EuvButton` into the ambient framework-managed `Context`.
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
        let style: Style = self.resolved_style();
        let text_color: Color = style.get_text().get_color();
        let font_sz: f32 = style.get_text().get_font_size();
        let font_wt: FontWeight = style.get_text().get_weight();
        let disabled: bool = self.get_disabled();
        let Self {
            label, on_click, ..
        } = self;
        let mut a11y: A11yProps = A11yProps::new(A11yRole::Button, label.clone());
        a11y.set_disabled(disabled);
        let handler_opt: Option<UiEventHandler> = if !disabled {
            on_click.as_ref().map(Rc::clone)
        } else {
            None
        };
        let mut txt_st: Style = Style::default();
        txt_st
            .get_mut_layout()
            .set_max_width(Length::Percent(100.0));
        txt_st
            .get_mut_text()
            .set_color(text_color)
            .set_font_size(font_sz)
            .set_weight(font_wt)
            .set_align(TextAlign::Center)
            .set_overflow(TextOverflow::Ellipsis);
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                let btn_node: NodeId = tree.create_view(style);
                if let Some(node) = tree.get_node_mut(btn_node) {
                    node.set_a11y(Some(a11y));
                    if let Some(handler_rc) = handler_opt {
                        node.get_mut_handlers().set_on_click(Some(handler_rc));
                        node.get_mut_interaction().set_focusable(true);
                    }
                }
                let txt_node: NodeId = tree.create_text(label.clone(), txt_st);
                tree.append_child(btn_node, txt_node);
                btn_node
            },
            NodeId::default(),
        )
    }
}
