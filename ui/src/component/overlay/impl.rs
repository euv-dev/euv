use super::*;

/// Declarative `#[component]` implementation for `EuvModal`.
#[component]
/// Inherent implementation of [`EuvModal`].
impl EuvModal {
    /// Renders `EuvModal` into the ambient framework-managed `Context`.
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
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let open: Signal<bool> = self.get_open();
        let body: NodeId = self.get_body();
        let Self { title, .. } = self.clone();
        let init_open: bool = open.get_untracked();
        let mut dialog: NodeId = NodeId::default();
        let backdrop: NodeId = view! {
            portal {
                position: Position::Fixed
                inset: (Some(0.0), Some(0.0), Some(0.0), Some(0.0))
                width: Length::Percent(100.0)
                height: Length::Percent(100.0)
                justify: JustifyContent::Center
                align: AlignItems::Center
                background: p.get_overlay()
                stop_scroll_propagation: true
                z_index: Z_INDEX_MODAL
                display: if init_open { Display::Flex } else { Display::None }
                column {
                    node_ref: dialog
                    width: Length::Percent(92.0)
                    max_width: Length::Px(460.0)
                    padding: Edges::all(SPACE_XL)
                    gap: SPACE_MD
                    background: p.get_card()
                    border: (g.get_border_width(), p.get_border())
                    radius: g.get_radius_xl()
                    overflow: Overflow::Hidden
                    shadow: g.get_shadow_lg()
                    role: A11yRole::Dialog
                    a11y_label: title.clone()
                    text {
                        font_size: g.get_font_heading()
                        font_weight: FontWeight::Bold
                        color: p.get_card_foreground()
                        { title }
                    }
                    { body }
                }
            }
        };
        ctx.on_click(backdrop, move |ev: &UiEvent| {
            if ev.get_target() == backdrop {
                let _: bool = open.set_if_changed(false);
            }
        });
        let handle: SceneHandle = ctx.get_scene().clone();
        let prev_open: Rc<Cell<bool>> = Rc::new(Cell::new(init_open));
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            let _: Effect = EuvOverlayStack::bind_signal(open, MODAL_OVERLAY_NAME, Z_INDEX_MODAL);
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
                    handle.start_transition(
                        NodeTransition::fade(backdrop, 0.0, 1.0, 0.22, false)
                            .with_easing(Easing::EaseOutQuint),
                    );
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
                    handle.start_transition(
                        NodeTransition::modal_exit(dialog).with_easing(Easing::EaseOutCubic),
                    );
                    handle.start_transition(
                        NodeTransition::fade(backdrop, 1.0, 0.0, 0.18, true)
                            .with_easing(Easing::EaseOutCubic),
                    );
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

/// Declarative `#[component]` implementation for `EuvToast`.
#[component]
/// Inherent implementation of [`EuvToast`].
impl EuvToast {
    /// Renders `EuvToast` into the ambient framework-managed `Context`.
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
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let (bg, fg): (Color, Color) = self.get_tone().colors(p);
        let visible: Signal<bool> = self.get_visible();
        let Self { message, .. } = self.clone();
        let init_vis: bool = visible.get_untracked();
        let toast: NodeId = view! {
            portal {
                style: Style::default().row()
                align: AlignItems::Center
                display: if init_vis { Display::Flex } else { Display::None }
                position: Position::Fixed
                inset: (None, Some(SPACE_LG), Some(SPACE_LG), None)
                padding: Edges::symmetric(SPACE_SM, SPACE_LG)
                background: bg
                radius: g.get_radius_md()
                shadow: g.get_shadow_md()
                pointer_events: PointerEvents::None
                z_index: Z_INDEX_TOAST
                role: A11yRole::Alert
                a11y_label: message.clone()
                text {
                    font_size: 13.0
                    font_weight: FontWeight::Medium
                    color: fg
                    { message }
                }
            }
        };
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
                    handle.start_transition(
                        NodeTransition::slide_fade(
                            toast,
                            Point::new(0.0, 22.0),
                            Point::new(0.0, 0.0),
                            0.0,
                            1.0,
                            0.28,
                            false,
                        )
                        .with_scale(0.92, 1.0)
                        .with_easing(Easing::SpringSnappy),
                    );
                } else {
                    handle.start_transition(
                        NodeTransition::slide_fade(
                            toast,
                            Point::new(0.0, 0.0),
                            Point::new(0.0, 14.0),
                            1.0,
                            0.0,
                            0.18,
                            true,
                        )
                        .with_scale(1.0, 0.96)
                        .with_easing(Easing::EaseOutCubic),
                    );
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
