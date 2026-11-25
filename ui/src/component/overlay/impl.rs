use super::*;

/// `Mountable` implementation for `EuvModal`.
impl Mountable for EuvModal {
    /// Mounts the fixed scrim backdrop and centered dialog surface, toggling `Display::Flex`/`None` reactively.
    ///
    /// # Arguments
    ///
    /// - `Self` - Modal props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted overlay portal node.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let open: Signal<bool> = self.get_open();
        let body: NodeId = self.get_body();
        let Self { title, .. } = self;
        let backdrop: NodeId = ctx.portal(Style::empty());
        let dialog: NodeId = ctx.view(
            Style::empty()
                .column()
                .width(Length::Percent(92.0))
                .max_width(Length::Px(460.0))
                .padding(Edges::all(SPACE_XL))
                .gap(SPACE_MD)
                .background(p.get_card())
                .border(g.get_border_width(), p.get_border())
                .radius(g.get_radius_xl())
                .overflow(Overflow::Hidden)
                .shadow(Shadow::new(0.0, 12.0, 24.0, 0.0, Color::rgba(0, 0, 0, 72))),
        );
        let heading: NodeId = ctx.text(
            title,
            Style::empty()
                .font_size(g.get_font_heading())
                .font_weight(FontWeight::Bold)
                .color(p.get_card_foreground()),
        );
        ctx.append(dialog, heading);
        ctx.append(dialog, body);
        ctx.append(backdrop, dialog);
        let handle: SceneHandle = ctx.get_scene().clone();
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            handle.bind_style(backdrop, move || {
                let disp: Display = if open.get() {
                    Display::Flex
                } else {
                    Display::None
                };
                Style::empty()
                    .display(disp)
                    .position(Position::Fixed)
                    .inset(Some(0.0), Some(0.0), Some(0.0), Some(0.0))
                    .width(Length::Percent(100.0))
                    .height(Length::Percent(100.0))
                    .justify(JustifyContent::Center)
                    .align(AlignItems::Center)
                    .background(p.get_overlay())
                    .z_index(Z_INDEX_MODAL)
            })
        });
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                tree.attach_scope(backdrop, scope);
            },
            (),
        );
        backdrop
    }
}

/// `Mountable` implementation for `EuvToast`.
impl Mountable for EuvToast {
    /// Mounts the fixed floating toast banner at the bottom-right corner.
    ///
    /// # Arguments
    ///
    /// - `Self` - Toast props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted toast portal node.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let (bg, fg): (Color, Color) = self.get_tone().colors(p);
        let visible: Signal<bool> = self.get_visible();
        let Self { message, .. } = self;
        let toast: NodeId = ctx.portal(Style::empty());
        let txt: NodeId = ctx.text(
            message,
            Style::empty()
                .font_size(13.0)
                .font_weight(FontWeight::Medium)
                .color(fg),
        );
        ctx.append(toast, txt);
        let handle: SceneHandle = ctx.get_scene().clone();
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            handle.bind_style(toast, move || {
                let disp: Display = if visible.get() {
                    Display::Flex
                } else {
                    Display::None
                };
                Style::empty()
                    .display(disp)
                    .position(Position::Fixed)
                    .inset(None, Some(SPACE_LG), Some(SPACE_LG), None)
                    .padding(Edges::symmetric(SPACE_SM, SPACE_LG))
                    .background(bg)
                    .radius(g.get_radius_md())
                    .z_index(Z_INDEX_TOAST)
            })
        });
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                tree.attach_scope(toast, scope);
            },
            (),
        );
        toast
    }
}
