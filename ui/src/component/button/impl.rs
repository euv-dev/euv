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
    #[must_use]
    pub fn resolved_style(&self) -> Style {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let (height, pad_x, font_sz, radius): (f32, f32, f32, f32) = match self.get_size() {
            EuvButtonSize::Small => (g.get_control_sm(), SPACE_SM, 12.0, g.get_radius_sm()),
            EuvButtonSize::Default => (g.get_control_md(), SPACE_MD, 13.0, g.get_radius_md()),
            EuvButtonSize::Large => (g.get_control_lg(), SPACE_XL, 14.0, g.get_radius_md()),
            EuvButtonSize::Icon => (g.get_control_md(), 0.0, 13.0, g.get_radius_md()),
        };
        let mut style: Style = Style::default()
            .row()
            .justify(JustifyContent::Center)
            .align(AlignItems::Center)
            .height(Length::Px(height))
            .padding(Edges::symmetric(0.0, pad_x))
            .overflow(Overflow::Hidden)
            .radius(radius)
            .font_size(font_sz)
            .font_weight(FontWeight::Medium)
            .focus_ring(Border::new(g.get_ring_width(), p.get_ring()))
            .cursor(if self.get_disabled() {
                CursorIcon::NotAllowed
            } else {
                CursorIcon::Pointer
            });
        if self.get_size() == EuvButtonSize::Icon {
            style = style.width(Length::Px(height));
        }
        style = match self.get_variant() {
            EuvButtonVariant::Primary => style
                .background(p.get_primary())
                .color(p.get_primary_foreground())
                .shadow(g.get_shadow_xs()),
            EuvButtonVariant::Secondary => style
                .background(p.get_secondary())
                .color(p.get_secondary_foreground()),
            EuvButtonVariant::Destructive => style
                .background(p.get_destructive())
                .color(p.get_destructive_foreground())
                .shadow(g.get_shadow_xs()),
            EuvButtonVariant::Outline => style
                .background(p.get_card())
                .border(g.get_border_width(), p.get_border())
                .color(p.get_foreground())
                .shadow(g.get_shadow_xs()),
            EuvButtonVariant::Ghost => style.color(p.get_foreground()),
            EuvButtonVariant::Link => style.color(p.get_primary()),
        };
        if self.get_disabled() {
            style = style.opacity(0.5);
        }
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
        } = self.clone();
        let btn_node: NodeId = view! {
            view {
                style: style
                role: A11yRole::Button
                a11y_label: label.clone()
                a11y_disabled: disabled
                text {
                    max_width: Length::Percent(100.0)
                    color: text_color
                    font_size: font_sz
                    font_weight: font_wt
                    text_align: TextAlign::Center
                    text_overflow: TextOverflow::Ellipsis
                    { label }
                }
            }
        };
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
