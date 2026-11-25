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
        let backdrop_base: Style = Style::empty()
            .position(Position::Fixed)
            .inset(Some(0.0), Some(0.0), Some(0.0), Some(0.0))
            .width(Length::Percent(100.0))
            .height(Length::Percent(100.0))
            .justify(JustifyContent::Center)
            .align(AlignItems::Center)
            .background(p.get_overlay())
            .z_index(Z_INDEX_MODAL);
        let init_open: bool = open.get_untracked();
        let backdrop: NodeId = ctx.portal(backdrop_base.display(if init_open {
            Display::Flex
        } else {
            Display::None
        }));
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
        ctx.on_click(backdrop, move |ev: &UiEvent| {
            if ev.get_target() == backdrop {
                let _: bool = open.set_if_changed(false);
            }
        });
        let handle: SceneHandle = ctx.get_scene().clone();
        let prev_open: Rc<Cell<bool>> = Rc::new(Cell::new(init_open));
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            Effect::new(move || {
                let is_open: bool = open.get();
                let was_open: bool = prev_open.replace(is_open);
                if is_open == was_open {
                    return;
                }
                if is_open {
                    handle.with_write(
                        |tree: &mut SceneTree| {
                            if let Some(n) = tree.get_node_mut(backdrop) {
                                let mut st: Style = n.get_style();
                                st.get_mut_paint().set_pointer_events(PointerEvents::Auto);
                                n.set_style(st);
                            }
                        },
                        (),
                    );
                    handle.start_transition(NodeTransition::fade(backdrop, 0.0, 1.0, 0.20, false));
                    handle.start_transition(NodeTransition::modal_enter(dialog));
                } else {
                    handle.with_write(
                        |tree: &mut SceneTree| {
                            if let Some(n) = tree.get_node_mut(backdrop) {
                                let mut st: Style = n.get_style();
                                st.get_mut_paint().set_pointer_events(PointerEvents::None);
                                n.set_style(st);
                            }
                        },
                        (),
                    );
                    handle.start_transition(NodeTransition::modal_exit(dialog));
                    handle.start_transition(NodeTransition::fade(backdrop, 1.0, 0.0, 0.18, true));
                }
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
        let init_vis: bool = visible.get_untracked();
        let toast: NodeId = ctx.portal(
            Style::empty()
                .row()
                .align(AlignItems::Center)
                .display(if init_vis {
                    Display::Flex
                } else {
                    Display::None
                })
                .position(Position::Fixed)
                .inset(None, Some(SPACE_LG), Some(SPACE_LG), None)
                .padding(Edges::symmetric(SPACE_SM, SPACE_LG))
                .background(bg)
                .radius(g.get_radius_md())
                .shadow(Shadow::new(0.0, 6.0, 16.0, 0.0, Color::rgba(0, 0, 0, 32)))
                .pointer_events(PointerEvents::None)
                .z_index(Z_INDEX_TOAST),
        );
        let txt: NodeId = ctx.text(
            message,
            Style::empty()
                .font_size(13.0)
                .font_weight(FontWeight::Medium)
                .color(fg),
        );
        ctx.append(toast, txt);
        let handle: SceneHandle = ctx.get_scene().clone();
        let prev_vis: Rc<Cell<bool>> = Rc::new(Cell::new(init_vis));
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            Effect::new(move || {
                let is_vis: bool = visible.get();
                let was_vis: bool = prev_vis.replace(is_vis);
                if is_vis == was_vis {
                    return;
                }
                if is_vis {
                    handle.start_transition(NodeTransition::slide_fade(
                        toast,
                        Point::new(0.0, 16.0),
                        Point::new(0.0, 0.0),
                        0.0,
                        1.0,
                        0.20,
                        false,
                    ));
                } else {
                    handle.start_transition(NodeTransition::slide_fade(
                        toast,
                        Point::new(0.0, 0.0),
                        Point::new(0.0, 14.0),
                        1.0,
                        0.0,
                        0.18,
                        true,
                    ));
                }
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
