use super::*;

/// Fluent configuration builders for `EuvAlertDialog`.
impl EuvAlertDialog {
    /// Sets the explanatory body description fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source alert dialog.
    /// - `S` - Body description string.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated alert dialog.
    #[must_use]
    pub fn description<S>(mut self, description: S) -> Self
    where
        S: Into<String>,
    {
        self.set_description(description.into());
        self
    }

    /// Sets custom cancel and confirm button labels fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source alert dialog.
    /// - `S1` - Cancel action label.
    /// - `S2` - Confirm action label.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated alert dialog.
    #[must_use]
    pub fn labels<S1, S2>(mut self, cancel_label: S1, confirm_label: S2) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        self.set_cancel_label(cancel_label.into());
        self.set_confirm_label(confirm_label.into());
        self
    }
}

/// Declarative `#[component]` implementation for `EuvAlertDialog`.
#[component]
/// Inherent implementation of [`EuvAlertDialog`].
impl EuvAlertDialog {
    /// Renders `EuvAlertDialog` into the ambient framework-managed `Context`.
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
        let open: Signal<bool> = self.get_open();
        let confirmed: Signal<bool> = self.get_confirmed();
        let Self {
            title,
            description,
            cancel_label,
            confirm_label,
            ..
        } = self.clone();
        let resolved_cancel: String = if cancel_label.is_empty() {
            ALERT_DEFAULT_CANCEL_LABEL.to_string()
        } else {
            cancel_label
        };
        let resolved_confirm: String = if confirm_label.is_empty() {
            ALERT_DEFAULT_CONFIRM_LABEL.to_string()
        } else {
            confirm_label
        };
        let init_open: bool = open.get_untracked();
        let mut card: NodeId = NodeId::default();
        let scrim: NodeId = view! {
            portal {
                display: if init_open { Display::Flex } else { Display::None }
                position: Position::Fixed
                inset: (Some(0.0), Some(0.0), Some(0.0), Some(0.0))
                width: Length::Percent(100.0)
                height: Length::Percent(100.0)
                justify: JustifyContent::Center
                align: AlignItems::Center
                background: p.get_overlay()
                stop_scroll_propagation: true
                z_index: Z_INDEX_MODAL
                column {
                    node_ref: card
                    width: Length::Percent(92.0)
                    max_width: Length::Px(440.0)
                    padding: Edges::all(SPACE_XL)
                    gap: SPACE_MD
                    background: p.get_card()
                    border: (g.get_border_width(), p.get_border())
                    radius: g.get_radius_lg()
                    overflow: Overflow::Hidden
                    shadow: g.get_shadow_lg()
                    role: A11yRole::Dialog
                    a11y_label: title.clone()
                    text {
                        width: Length::Percent(100.0)
                        font_size: g.get_font_heading()
                        font_weight: FontWeight::SemiBold
                        color: p.get_card_foreground()
                        { title }
                    }
                    text {
                        width: Length::Percent(100.0)
                        font_size: 13.0
                        color: p.get_muted_foreground()
                        { description }
                    }
                    row {
                        flex_wrap: FlexWrap::Wrap
                        width: Length::Percent(100.0)
                        justify: JustifyContent::End
                        gap: SPACE_SM
                        EuvButton(resolved_cancel, tokens) {
                            outline()
                            on_click: move |_: &UiEvent| {
                                let _: bool = confirmed.set_if_changed(false);
                                let _: bool = open.set_if_changed(false);
                            }
                        }
                        EuvButton(resolved_confirm, tokens) {
                            destructive()
                            on_click: move |_: &UiEvent| {
                                let _: bool = confirmed.set_if_changed(true);
                                let _: bool = open.set_if_changed(false);
                            }
                        }
                    }
                }
            }
        };
        ctx.on_click(scrim, move |ev: &UiEvent| {
            if ev.get_target() == scrim {
                let _: bool = confirmed.set_if_changed(false);
                let _: bool = open.set_if_changed(false);
            }
        });
        let handle: SceneHandle = ctx.get_scene().clone();
        let prev_open: Rc<Cell<bool>> = Rc::new(Cell::new(init_open));
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            let _: Effect =
                EuvOverlayStack::bind_signal(open, ALERT_DIALOG_OVERLAY_NAME, Z_INDEX_MODAL);
            Effect::new(move || {
                let is_open: bool = open.get();
                let was_open: bool = prev_open.replace(is_open);
                if is_open == was_open {
                    return;
                }
                if is_open {
                    handle.with_write(
                        |tree: &mut SceneTree| {
                            if let Some(n) = tree.get_node_mut(scrim) {
                                let mut st: Style = n.get_style();
                                st.get_mut_paint().set_pointer_events(PointerEvents::Auto);
                                n.set_style(st);
                            }
                        },
                        (),
                    );
                    handle.start_transition(NodeTransition::fade(scrim, 0.0, 1.0, 0.20, false));
                    handle.start_transition(NodeTransition::modal_enter(card));
                } else {
                    handle.with_write(
                        |tree: &mut SceneTree| {
                            if let Some(n) = tree.get_node_mut(scrim) {
                                let mut st: Style = n.get_style();
                                st.get_mut_paint().set_pointer_events(PointerEvents::None);
                                n.set_style(st);
                            }
                        },
                        (),
                    );
                    handle.start_transition(NodeTransition::modal_exit(card));
                    handle.start_transition(NodeTransition::fade(scrim, 1.0, 0.0, 0.18, true));
                }
            })
        });
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                tree.attach_scope(scrim, scope);
            },
            (),
        );
        scrim
    }
}

/// Declarative `#[component]` implementation for `EuvCollapsible`.
#[component]
/// Inherent implementation of [`EuvCollapsible`].
impl EuvCollapsible {
    /// Renders `EuvCollapsible` into the ambient framework-managed `Context`.
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
        let open: Signal<bool> = self.get_open();
        let content: NodeId = self.get_content();
        let Self { title, .. } = self.clone();
        view! {
            column {
                width: Length::Percent(100.0)
                gap: SPACE_XS
                background: p.get_card()
                border: (g.get_border_width(), p.get_border())
                radius: g.get_radius_md()
                overflow: Overflow::Hidden
                row {
                    width: Length::Percent(100.0)
                    padding: Edges::symmetric(SPACE_SM, SPACE_MD)
                    justify: JustifyContent::SpaceBetween
                    align: AlignItems::Center
                    cursor: CursorIcon::Pointer
                    on_click: move |_: &UiEvent| {
                        let _: bool = open.update(|v: &mut bool| *v = !*v);
                    }
                    text {
                        font_size: 13.0
                        font_weight: FontWeight::SemiBold
                        color: p.get_foreground()
                        { title }
                    }
                    text {
                        font_size: 12.0
                        font_weight: FontWeight::Medium
                        color: p.get_muted_foreground()
                        { move || {
                                if open.get() {
                                    COLLAPSIBLE_HIDE_LABEL.to_string()
                                } else {
                                    COLLAPSIBLE_SHOW_LABEL.to_string()
                                }
                        } }
                    }
                }
                column {
                    bind_style: move || {
                        let disp: Display = if open.get() {
                            Display::Flex
                        } else {
                            Display::None
                        };
                        Style::default()
                        .column()
                        .display(disp)
                        .width(Length::Percent(100.0))
                        .padding(Edges::all(SPACE_MD))
                        .border(g.get_border_width(), p.get_border())
                    }
                    { content }
                }
            }
        }
    }
}

/// Declarative `#[component]` implementation for `EuvToggleGroup`.
#[component]
/// Inherent implementation of [`EuvToggleGroup`].
impl EuvToggleGroup {
    /// Renders `EuvToggleGroup` into the ambient framework-managed `Context`.
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
        let selected: Signal<usize> = self.get_selected();
        let Self { options, .. } = self.clone();
        view! {
            row {
                flex_wrap: FlexWrap::Wrap
                width: Length::Percent(100.0)
                gap: SPACE_XS
                for (idx, opt) in options.into_iter().enumerate() {
                    row {
                        bind_style: move || {
                            let on: bool = selected.get() == idx;
                            Style::default()
                            .row()
                            .height(Length::Px(g.get_control_sm()))
                            .justify(JustifyContent::Center)
                            .align(AlignItems::Center)
                            .padding(Edges::symmetric(0.0, SPACE_MD))
                            .overflow(Overflow::Hidden)
                            .radius(g.get_radius_md())
                            .border(
                                g.get_border_width(),
                                if on { p.get_primary() } else { p.get_border() },
                            )
                            .background(if on {
                                    p.get_accent()
                                } else {
                                    p.get_background()
                            })
                            .cursor(CursorIcon::Pointer)
                        }
                        on_click: move |_: &UiEvent| {
                            let _: bool = selected.set_if_changed(idx);
                        }
                        text {
                            font_size: 13.0
                            font_weight: FontWeight::Medium
                            color: p.get_foreground()
                            text_align: TextAlign::Center
                            text_overflow: TextOverflow::Ellipsis
                            { opt }
                        }
                    }
                }
            }
        }
    }
}

/// Declarative `#[component]` implementation for `EuvInputOtp`.
#[component]
/// Inherent implementation of [`EuvInputOtp`].
impl EuvInputOtp {
    /// Renders `EuvInputOtp` into the ambient framework-managed `Context`.
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
        let value: Signal<String> = self.get_value();
        let slots: usize = self.get_length().clamp(4, 8);
        view! {
            row {
                flex_wrap: FlexWrap::Wrap
                align: AlignItems::Center
                gap: SPACE_XS
                for slot_idx in 0..slots {
                    row {
                        width: Length::Px(36.0)
                        height: Length::Px(40.0)
                        justify: JustifyContent::Center
                        align: AlignItems::Center
                        background: p.get_background()
                        border: (g.get_border_width(), p.get_border())
                        radius: g.get_radius_md()
                        cursor: CursorIcon::Pointer
                        on_click: move |_: &UiEvent| {
                            let _: bool = value.update(|s: &mut String| {
                                    let mut chars: Vec<char> = s.chars().collect();
                                    while chars.len() <= slot_idx {
                                        chars.push('0');
                                    }
                                    let next_digit: u32 = chars[slot_idx]
                                    .to_digit(10)
                                    .map(|d: u32| (d + 1) % 10)
                                    .unwrap_or(1);
                                    chars[slot_idx] = char::from_digit(next_digit, 10).unwrap_or('0');
                                    chars.truncate(slots);
                                    *s = chars.into_iter().collect();
                            });
                        }
                        text {
                            font_family: FontFamily::Monospace
                            font_size: 15.0
                            font_weight: FontWeight::SemiBold
                            color: p.get_foreground()
                            text_align: TextAlign::Center
                            { move || {
                                    let cur: String = value.get();
                                    cur.chars()
                                    .nth(slot_idx)
                                    .map(|c: char| c.to_string())
                                    .unwrap_or_else(|| OTP_EMPTY_SLOT.to_string())
                            } }
                        }
                    }
                }
            }
        }
    }
}

/// Fluent search query and placeholder builders for `EuvCommand`.
impl EuvCommand {
    /// Binds an external reactive query signal fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source command palette.
    /// - `Signal<String>` - Controlled query signal.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated command palette.
    #[must_use]
    pub fn query(mut self, query: Signal<String>) -> Self {
        self.set_query(Some(query));
        self
    }

    /// Sets the search input placeholder text fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source command palette.
    /// - `S` - Placeholder prompt string.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated command palette.
    #[must_use]
    pub fn placeholder<S>(mut self, placeholder: S) -> Self
    where
        S: Into<String>,
    {
        self.set_placeholder(placeholder.into());
        self
    }
}

/// Declarative `#[component]` implementation for `EuvCommand`.
#[component]
/// Inherent implementation of [`EuvCommand`].
impl EuvCommand {
    /// Renders `EuvCommand` into the ambient framework-managed `Context`.
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
        let selected: Signal<Option<usize>> = self.get_selected();
        let Self {
            query: query_opt,
            placeholder,
            items,
            ..
        } = self.clone();
        let query: Signal<String> = query_opt.unwrap_or_else(|| Signal::new(String::new()));
        let resolved_ph: String = if placeholder.is_empty() {
            COMMAND_DEFAULT_PLACEHOLDER.to_string()
        } else {
            placeholder
        };
        view! {
            column {
                width: Length::Percent(100.0)
                max_width: Length::Px(520.0)
                padding: Edges::all(SPACE_SM)
                gap: SPACE_SM
                background: p.get_card()
                border: (g.get_border_width(), p.get_border())
                radius: g.get_radius_lg()
                overflow: Overflow::Hidden
                shadow: g.get_shadow_md()
                EuvInput(query, resolved_ph, tokens)
                column {
                    width: Length::Percent(100.0)
                    gap: SPACE_XS
                    bind_children: move || {
                        let needle: String = query.get().to_lowercase();
                        let mut nodes: Vec<NodeId> = Vec::new();
                        for (idx, item) in items.iter().enumerate() {
                            if !needle.is_empty() && !item.to_lowercase().contains(&needle) {
                                continue;
                            }
                            let is_sel: bool = selected.get() == Some(idx);
                            let row: NodeId = view! {
                                row {
                                    width: Length::Percent(100.0)
                                    padding: Edges::symmetric(6.0, SPACE_MD)
                                    radius: g.get_radius_sm()
                                    background: if is_sel {
                                        p.get_accent()
                                    } else {
                                        COLOR_TRANSPARENT
                                    }
                                    cursor: CursorIcon::Pointer
                                    on_click: move |_: &UiEvent| {
                                        let _: bool = selected.set_if_changed(Some(idx));
                                    }
                                    text {
                                        font_size: 13.0
                                        font_weight: if is_sel {
                                            FontWeight::SemiBold
                                        } else {
                                            FontWeight::Normal
                                        }
                                        color: p.get_foreground()
                                        { item.clone() }
                                    }
                                }
                            };
                            nodes.push(row);
                        }
                        if nodes.is_empty() {
                            nodes.push(view! {
                                    text {
                                        font_size: 12.0
                                        color: p.get_muted_foreground()
                                        { COMMAND_EMPTY_LABEL }
                                    }
                            });
                        }
                        nodes
                    }
                }
            }
        }
    }
}

/// Fluent controlled-state and body builders for `EuvHoverCard`.
impl EuvHoverCard {
    /// Binds a controlled reactive open visibility signal fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source hover card.
    /// - `Signal<bool>` - Controlled open flag signal.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated hover card.
    #[must_use]
    pub fn open(mut self, open: Signal<bool>) -> Self {
        self.set_open(Some(open));
        self
    }

    /// Sets the preview card description text fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source hover card.
    /// - `S` - Preview description string.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated hover card.
    #[must_use]
    pub fn description<S>(mut self, description: S) -> Self
    where
        S: Into<String>,
    {
        self.set_description(description.into());
        self
    }
}

/// Declarative `#[component]` implementation for `EuvHoverCard`.
#[component]
/// Inherent implementation of [`EuvHoverCard`].
impl EuvHoverCard {
    /// Renders `EuvHoverCard` into the ambient framework-managed `Context`.
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
            open: open_opt,
            trigger_label,
            title,
            description,
            ..
        } = self.clone();
        let open: Signal<bool> = open_opt.unwrap_or_else(|| Signal::new(false));
        let opened_by_hover: Rc<Cell<bool>> = Rc::new(Cell::new(false));
        let hover_click: Rc<Cell<bool>> = opened_by_hover.clone();
        let hover_enter: Rc<Cell<bool>> = opened_by_hover.clone();
        let hover_leave: Rc<Cell<bool>> = opened_by_hover;
        view! {
            column {
                align: AlignItems::Start
                gap: SPACE_XS
                on_pointer_enter: move |_: &UiEvent| {
                    if !open.get_untracked() {
                        hover_enter.set(true);
                    }
                    let _: bool = open.set_if_changed(true);
                }
                on_pointer_leave: move |_: &UiEvent| {
                    hover_leave.set(false);
                    let _: bool = open.set_if_changed(false);
                }
                EuvButton(trigger_label, tokens) {
                    link()
                    sm()
                    on_click: move |_: &UiEvent| {
                        if hover_click.replace(false) {
                            let _: bool = open.set_if_changed(true);
                        } else {
                            let _: bool = open.update(|v: &mut bool| *v = !*v);
                        }
                    }
                }
                column {
                    bind_style: move || {
                        let disp: Display = if open.get() {
                            Display::Flex
                        } else {
                            Display::None
                        };
                        Style::default()
                        .column()
                        .display(disp)
                        .width(Length::Percent(100.0))
                        .max_width(Length::Px(320.0))
                        .padding(Edges::all(SPACE_MD))
                        .gap(SPACE_XS)
                        .background(p.get_card())
                        .border(g.get_border_width(), p.get_border())
                        .radius(g.get_radius_lg())
                        .overflow(Overflow::Hidden)
                        .shadow(g.get_shadow_lg())
                        .z_index(Z_INDEX_FLOATING)
                    }
                    text {
                        width: Length::Percent(100.0)
                        font_size: 13.0
                        font_weight: FontWeight::SemiBold
                        color: p.get_card_foreground()
                        { title }
                    }
                    if !description.is_empty() {
                        text {
                            width: Length::Percent(100.0)
                            font_size: 12.0
                            color: p.get_muted_foreground()
                            { description }
                        }
                    }
                }
            }
        }
    }
}

/// Declarative `#[component]` implementation for `EuvCalendar`.
#[component]
/// Inherent implementation of [`EuvCalendar`].
impl EuvCalendar {
    /// Renders `EuvCalendar` into the ambient framework-managed `Context`.
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
        let selected_day: Signal<u32> = self.get_selected_day();
        let total_days: u32 = self.get_days_in_month().clamp(28, 31);
        let Self { month_label, .. } = self.clone();
        view! {
            column {
                width: Length::Percent(100.0)
                max_width: Length::Px(320.0)
                padding: Edges::all(SPACE_MD)
                gap: SPACE_SM
                background: p.get_card()
                border: (g.get_border_width(), p.get_border())
                radius: g.get_radius_lg()
                overflow: Overflow::Hidden
                text {
                    font_size: 14.0
                    font_weight: FontWeight::SemiBold
                    color: p.get_foreground()
                    text_align: TextAlign::Center
                    { month_label }
                }
                grid {
                    columns: 7
                    width: Length::Percent(100.0)
                    gap: 2.0
                    for wd in CALENDAR_WEEKDAYS.iter().copied() {
                        row {
                            justify: JustifyContent::Center
                            align: AlignItems::Center
                            padding: Edges::symmetric(2.0, 0.0)
                            text {
                                font_size: 11.0
                                font_weight: FontWeight::Medium
                                color: p.get_muted_foreground()
                                text_align: TextAlign::Center
                                { wd }
                            }
                        }
                    }
                }
                grid {
                    columns: 7
                    width: Length::Percent(100.0)
                    gap: 2.0
                    for day in 1..=total_days {
                        row {
                            bind_style: move || {
                                let is_sel: bool = selected_day.get() == day;
                                Style::default()
                                .row()
                                .height(Length::Px(28.0))
                                .justify(JustifyContent::Center)
                                .align(AlignItems::Center)
                                .radius(g.get_radius_sm())
                                .background(if is_sel {
                                        p.get_accent()
                                    } else {
                                        COLOR_TRANSPARENT
                                })
                                .border(
                                    if is_sel { g.get_border_width() } else { 0.0 },
                                    p.get_primary(),
                                )
                                .cursor(CursorIcon::Pointer)
                            }
                            on_click: move |_: &UiEvent| {
                                let _: bool = selected_day.set_if_changed(day);
                            }
                            text {
                                font_size: 12.0
                                font_weight: FontWeight::Medium
                                color: p.get_foreground()
                                text_align: TextAlign::Center
                                { day.to_string() }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Declarative `#[component]` implementation for `EuvAspectRatio`.
#[component]
/// Inherent implementation of [`EuvAspectRatio`].
impl EuvAspectRatio {
    /// Renders `EuvAspectRatio` into the ambient framework-managed `Context`.
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
        let ratio: f32 = if self.get_ratio() > 0.0 && self.get_ratio().is_finite() {
            self.get_ratio()
        } else {
            16.0 / 9.0
        };
        let child: NodeId = self.get_child();
        view! {
            column {
                width: Length::Percent(100.0)
                aspect_ratio: ratio
                justify: JustifyContent::Center
                align: AlignItems::Center
                background: p.get_muted()
                border: (g.get_border_width(), p.get_border())
                radius: g.get_radius_md()
                overflow: Overflow::Hidden
                { child }
            }
        }
    }
}

/// Declarative `#[component]` implementation for `EuvMenubar`.
#[component]
/// Inherent implementation of [`EuvMenubar`].
impl EuvMenubar {
    /// Renders `EuvMenubar` into the ambient framework-managed `Context`.
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
        let active_menu: Signal<Option<usize>> = self.get_active_menu();
        let Self { menus, .. } = self.clone();
        view! {
            row {
                flex_wrap: FlexWrap::Wrap
                width: Length::Percent(100.0)
                padding: Edges::all(SPACE_XS)
                gap: SPACE_XS
                background: p.get_card()
                border: (g.get_border_width(), p.get_border())
                radius: g.get_radius_md()
                for (idx, label) in menus.into_iter().enumerate() {
                    row {
                        bind_style: move || {
                            let is_open: bool = active_menu.get() == Some(idx);
                            Style::default()
                            .row()
                            .height(Length::Px(g.get_control_sm()))
                            .justify(JustifyContent::Center)
                            .align(AlignItems::Center)
                            .padding(Edges::symmetric(0.0, SPACE_MD))
                            .overflow(Overflow::Hidden)
                            .radius(g.get_radius_sm())
                            .background(if is_open {
                                    p.get_accent()
                                } else {
                                    COLOR_TRANSPARENT
                            })
                            .cursor(CursorIcon::Pointer)
                        }
                        on_click: move |_: &UiEvent| {
                            let _: bool = active_menu.update(|cur: &mut Option<usize>| {
                                    *cur = if *cur == Some(idx) { None } else { Some(idx) };
                            });
                        }
                        text {
                            font_size: 13.0
                            font_weight: FontWeight::Medium
                            color: p.get_foreground()
                            text_align: TextAlign::Center
                            text_overflow: TextOverflow::Ellipsis
                            { label }
                        }
                    }
                }
            }
        }
    }
}

/// Declarative `#[component]` implementation for `EuvCarousel`.
#[component]
/// Inherent implementation of [`EuvCarousel`].
impl EuvCarousel {
    /// Renders `EuvCarousel` into the ambient framework-managed `Context`.
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
        let active: Signal<usize> = self.get_active_slide();
        let Self { slides, .. } = self.clone();
        let total: usize = slides.len();
        let slides_for_text: Vec<String> = slides;
        view! {
            column {
                width: Length::Percent(100.0)
                gap: SPACE_SM
                column {
                    width: Length::Percent(100.0)
                    padding: Edges::all(SPACE_XL)
                    justify: JustifyContent::Center
                    align: AlignItems::Center
                    background: p.get_muted()
                    border: (g.get_border_width(), p.get_border())
                    radius: g.get_radius_lg()
                    overflow: Overflow::Hidden
                    text {
                        font_size: 14.0
                        font_weight: FontWeight::SemiBold
                        color: p.get_foreground()
                        text_align: TextAlign::Center
                        { move || {
                                if total == 0 {
                                    CAROUSEL_EMPTY_LABEL.to_string()
                                } else {
                                    let idx: usize = active.get() % total;
                                    slides_for_text[idx].clone()
                                }
                        } }
                    }
                }
                row {
                    flex_wrap: FlexWrap::Wrap
                    width: Length::Percent(100.0)
                    justify: JustifyContent::SpaceBetween
                    align: AlignItems::Center
                    gap: SPACE_SM
                    EuvButton(PAGE_PREV_LABEL.to_string(), tokens) {
                        outline()
                        sm()
                        on_click: move |_: &UiEvent| {
                            if total > 0 {
                                let _: bool = active.update(|v: &mut usize| {
                                        *v = if *v == 0 { total - 1 } else { *v - 1 };
                                });
                            }
                        }
                    }
                    text {
                        font_size: 12.0
                        color: p.get_muted_foreground()
                        { move || {
                                if total == 0 {
                                    CAROUSEL_ZERO_COUNTER.to_string()
                                } else {
                                    format!("{} / {total}", (active.get() % total) + 1)
                                }
                        } }
                    }
                    EuvButton(PAGE_NEXT_LABEL.to_string(), tokens) {
                        outline()
                        sm()
                        on_click: move |_: &UiEvent| {
                            if total > 0 {
                                let _: bool = active.update(|v: &mut usize| *v = (*v + 1) % total);
                            }
                        }
                    }
                }
            }
        }
    }
}
