use super::*;

/// Builder and mounting methods for `EuvButton`.
impl EuvButton {
    /// Creates a new primary `EuvButton` with `label` and `tokens`.
    ///
    /// # Arguments
    ///
    /// - `String` - Button label.
    /// - `ThemeTokens` - Active design tokens.
    ///
    /// # Returns
    ///
    /// - `Self` - Constructed button props.
    #[must_use]
    pub fn create(label: String, tokens: ThemeTokens) -> Self {
        Self {
            label,
            variant: EuvButtonVariant::Primary,
            size: EuvButtonSize::Default,
            disabled: false,
            tokens,
            on_click: None,
        }
    }

    /// Sets the visual button variant.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source button.
    /// - `EuvButtonVariant` - Target variant.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated button.
    #[must_use]
    pub fn variant(mut self, variant: EuvButtonVariant) -> Self {
        self.set_variant(variant);
        self
    }

    /// Sets the button size preset.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source button.
    /// - `EuvButtonSize` - Target size.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated button.
    #[must_use]
    pub fn size(mut self, size: EuvButtonSize) -> Self {
        self.set_size(size);
        self
    }

    /// Marks the button disabled.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source button.
    /// - `bool` - Disabled flag.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated button.
    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.set_disabled(disabled);
        self
    }

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

    /// Resolves the deterministic compiled `Style` for this button.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Button configuration.
    ///
    /// # Returns
    ///
    /// - `Style` - Compiled cross-platform style.
    #[must_use]
    pub fn resolved_style(&self) -> Style {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let (height, pad_x, font_sz): (f32, f32, f32) = match self.get_size() {
            EuvButtonSize::Small => (g.get_control_sm(), SPACE_MD, 12.0),
            EuvButtonSize::Default => (g.get_control_md(), SPACE_LG, g.get_font_body()),
            EuvButtonSize::Large => (g.get_control_lg(), SPACE_2XL, 16.0),
            EuvButtonSize::Icon => (g.get_control_md(), SPACE_SM, g.get_font_body()),
        };
        let mut style: Style = Style::empty()
            .row()
            .justify(JustifyContent::Center)
            .align(AlignItems::Center)
            .height(Length::Px(height))
            .padding(Edges::symmetric(0.0, pad_x))
            .radius(g.get_radius_md())
            .font_size(font_sz)
            .font_weight(FontWeight::Medium)
            .focus_ring(Border::new(g.get_ring_width(), p.get_ring()))
            .cursor(if self.get_disabled() {
                CursorIcon::NotAllowed
            } else {
                CursorIcon::Pointer
            });
        style = match self.get_variant() {
            EuvButtonVariant::Primary => style
                .background(p.get_primary())
                .color(p.get_primary_foreground()),
            EuvButtonVariant::Secondary => style
                .background(p.get_secondary())
                .color(p.get_secondary_foreground()),
            EuvButtonVariant::Destructive => style
                .background(p.get_destructive())
                .color(p.get_destructive_foreground()),
            EuvButtonVariant::Outline => style
                .background(p.get_background())
                .border(g.get_border_width(), p.get_border())
                .color(p.get_foreground()),
            EuvButtonVariant::Ghost => style.color(p.get_foreground()),
            EuvButtonVariant::Link => style.color(p.get_primary()),
        };
        if self.get_disabled() {
            style = style.opacity(0.5);
        }
        style
    }
}

/// `Mountable` implementation mounting `EuvButton` directly into `SceneTree`.
impl Mountable for EuvButton {
    /// Mounts the button container and centered text child into `ctx`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Owned button props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted button node handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let style: Style = self.resolved_style();
        let text_style: Style = Style::empty()
            .color(style.get_text().get_color())
            .font_size(style.get_text().get_font_size())
            .font_weight(style.get_text().get_weight())
            .text_align(TextAlign::Center);
        let disabled: bool = self.get_disabled();
        let Self {
            label, on_click, ..
        } = self;
        let btn_node: NodeId = ctx.view(style);
        let label_node: NodeId = ctx.text(label, text_style);
        ctx.append(btn_node, label_node);
        if !disabled && let Some(handler) = on_click {
            ctx.get_scene().with_write(
                |tree: &mut SceneTree| {
                    if let Some(node) = tree.get_node_mut(btn_node) {
                        node.get_mut_handlers().set_on_click(Some(handler));
                        node.get_mut_interaction().set_focusable(true);
                    }
                },
                (),
            );
        }
        btn_node
    }
}
