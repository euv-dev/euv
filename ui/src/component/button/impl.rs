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
            EuvButtonSize::Default => (g.get_control_md(), SPACE_LG, 13.0),
            EuvButtonSize::Large => (g.get_control_lg(), SPACE_2XL, 14.0),
            EuvButtonSize::Icon => (g.get_control_md(), SPACE_SM, 13.0),
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
                .color(p.get_primary_foreground())
                .shadow(Shadow::new(0.0, 1.0, 2.0, 0.0, Color::rgba(0, 0, 0, 20))),
            EuvButtonVariant::Secondary => style
                .background(p.get_secondary())
                .color(p.get_secondary_foreground()),
            EuvButtonVariant::Destructive => style
                .background(p.get_destructive())
                .color(p.get_destructive_foreground())
                .shadow(Shadow::new(0.0, 1.0, 2.0, 0.0, Color::rgba(0, 0, 0, 20))),
            EuvButtonVariant::Outline => style
                .background(p.get_background())
                .border(g.get_border_width(), p.get_border())
                .color(p.get_foreground())
                .shadow(Shadow::new(0.0, 1.0, 2.0, 0.0, Color::rgba(0, 0, 0, 10))),
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
        let mut a11y: A11yProps = A11yProps::new(A11yRole::Button, label.clone());
        a11y.set_disabled(disabled);
        ctx.set_a11y(btn_node, a11y);
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
