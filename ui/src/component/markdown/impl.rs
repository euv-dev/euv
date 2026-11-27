use super::*;

/// Zero-allocation language resolver for `CodeLang`.
impl CodeLang {
    /// Resolves a raw language hint string into its `CodeLang` variant without heap allocation.
    ///
    /// # Arguments
    ///
    /// - `&str` - Raw language hint (e.g. `"rust"`, `"JSON"`, `"diff"`).
    ///
    /// # Returns
    ///
    /// - `Self` - Pre-resolved `CodeLang` variant.
    #[inline(always)]
    #[must_use]
    pub fn from_name(language: &str) -> Self {
        let trimmed: &str = language.trim();
        if trimmed.eq_ignore_ascii_case(CODE_LANG_RUST) {
            Self::Rust
        } else if trimmed.eq_ignore_ascii_case(CODE_LANG_DIFF) {
            Self::Diff
        } else if trimmed.eq_ignore_ascii_case(CODE_LANG_JSON) {
            Self::Json
        } else if trimmed.eq_ignore_ascii_case(CODE_LANG_TOML) {
            Self::Toml
        } else if trimmed.eq_ignore_ascii_case(CODE_LANG_YAML) {
            Self::Yaml
        } else {
            Self::Generic
        }
    }
}

/// Exact VS Code `Eva Light Bold` / `Eva Dark Bold` palette resolution for `CodeThemePalette`.
impl CodeThemePalette {
    /// Resolves the exact VS Code `Eva Light Bold` or `Eva Dark Bold` palette for `mode`.
    ///
    /// # Arguments
    ///
    /// - `ThemeMode` - Active light or dark theme mode.
    ///
    /// # Returns
    ///
    /// - `Self` - Exact Eva theme color palette.
    #[inline(always)]
    #[must_use]
    pub fn resolve(mode: ThemeMode) -> Self {
        match mode {
            ThemeMode::Light => Self {
                theme_name: CODE_THEME_LABEL_LIGHT,
                background: Color::hex(EVA_LIGHT_BG),
                header_bg: Color::hex(EVA_LIGHT_HEADER_BG),
                border: Color::hex(EVA_LIGHT_BORDER),
                foreground: Color::hex(EVA_LIGHT_FG),
                line_number: Color::hex(EVA_LIGHT_LINE_NUM),
                line_highlight: Color::hex(EVA_LIGHT_LINE_HL),
                comment: Color::hex(EVA_LIGHT_COMMENT),
                keyword: Color::hex(EVA_LIGHT_KEYWORD),
                control: Color::hex(EVA_LIGHT_CONTROL),
                operator: Color::hex(EVA_LIGHT_OPERATOR),
                string: Color::hex(EVA_LIGHT_STRING),
                number: Color::hex(EVA_LIGHT_NUMBER),
                constant: Color::hex(EVA_LIGHT_CONSTANT),
                function: Color::hex(EVA_LIGHT_FUNCTION),
                ty: Color::hex(EVA_LIGHT_TYPE),
                primitive: Color::hex(EVA_LIGHT_PRIMITIVE),
                property: Color::hex(EVA_LIGHT_PROPERTY),
                punctuation: Color::hex(EVA_LIGHT_PUNCT),
                inline_code_bg: Color::hex(EVA_LIGHT_INLINE_CODE_BG),
                inline_code_fg: Color::hex(EVA_LIGHT_INLINE_CODE_FG),
                quote_bg: Color::hex(EVA_LIGHT_QUOTE_BG),
                quote_border: Color::hex(EVA_LIGHT_QUOTE_BORDER),
                link: Color::hex(EVA_LIGHT_LINK),
                diff_add: Color::hex(EVA_LIGHT_DIFF_ADD),
                diff_del: Color::hex(EVA_LIGHT_DIFF_DEL),
            },
            ThemeMode::Dark => Self {
                theme_name: CODE_THEME_LABEL_DARK,
                background: Color::hex(EVA_DARK_BG),
                header_bg: Color::hex(EVA_DARK_HEADER_BG),
                border: Color::hex(EVA_DARK_BORDER),
                foreground: Color::hex(EVA_DARK_FG),
                line_number: Color::hex(EVA_DARK_LINE_NUM),
                line_highlight: Color::hex(EVA_DARK_LINE_HL),
                comment: Color::hex(EVA_DARK_COMMENT),
                keyword: Color::hex(EVA_DARK_KEYWORD),
                control: Color::hex(EVA_DARK_CONTROL),
                operator: Color::hex(EVA_DARK_OPERATOR),
                string: Color::hex(EVA_DARK_STRING),
                number: Color::hex(EVA_DARK_NUMBER),
                constant: Color::hex(EVA_DARK_CONSTANT),
                function: Color::hex(EVA_DARK_FUNCTION),
                ty: Color::hex(EVA_DARK_TYPE),
                primitive: Color::hex(EVA_DARK_PRIMITIVE),
                property: Color::hex(EVA_DARK_PROPERTY),
                punctuation: Color::hex(EVA_DARK_PUNCT),
                inline_code_bg: Color::hex(EVA_DARK_INLINE_CODE_BG),
                inline_code_fg: Color::hex(EVA_DARK_INLINE_CODE_FG),
                quote_bg: Color::hex(EVA_DARK_QUOTE_BG),
                quote_border: Color::hex(EVA_DARK_QUOTE_BORDER),
                link: Color::hex(EVA_DARK_LINK),
                diff_add: Color::hex(EVA_DARK_DIFF_ADD),
                diff_del: Color::hex(EVA_DARK_DIFF_DEL),
            },
        }
    }

    /// Returns `(Color, FontWeight)` matching the exact VS Code `Eva Light Bold` / `Eva Dark Bold` TextMate rule for `kind`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Resolved Eva palette reference.
    /// - `CodeTokenKind` - Token classification.
    ///
    /// # Returns
    ///
    /// - `(Color, FontWeight)` - Exact token color and font weight.
    #[inline(always)]
    #[must_use]
    pub fn token_style(&self, kind: CodeTokenKind) -> (Color, FontWeight) {
        match kind {
            CodeTokenKind::Plain => (self.get_foreground(), FontWeight::Normal),
            CodeTokenKind::Keyword => (self.get_keyword(), FontWeight::Bold),
            CodeTokenKind::Control | CodeTokenKind::Constant => {
                (self.get_control(), FontWeight::Bold)
            }
            CodeTokenKind::Operator => (self.get_operator(), FontWeight::Normal),
            CodeTokenKind::Type => (self.get_ty(), FontWeight::Normal),
            CodeTokenKind::PrimitiveType => (self.get_primitive(), FontWeight::Normal),
            CodeTokenKind::Function | CodeTokenKind::Macro => {
                (self.get_function(), FontWeight::Bold)
            }
            CodeTokenKind::String => (self.get_string(), FontWeight::Normal),
            CodeTokenKind::Number => (self.get_number(), FontWeight::Normal),
            CodeTokenKind::Comment => (self.get_comment(), FontWeight::Normal),
            CodeTokenKind::Property => (self.get_property(), FontWeight::Normal),
            CodeTokenKind::Punctuation => (self.get_punctuation(), FontWeight::Normal),
            CodeTokenKind::DiffAdd => (self.get_diff_add(), FontWeight::Normal),
            CodeTokenKind::DiffDel => (self.get_diff_del(), FontWeight::Normal),
        }
    }
}

/// Optional checkbox state accessor for `EuvMarkdownListItem`.
impl EuvMarkdownListItem {
    /// Returns the optional GFM checkbox state.
    ///
    /// # Arguments
    ///
    /// - `&Self` - List item reference.
    ///
    /// # Returns
    ///
    /// - `Option<bool>` - `Some(checked)` for checklist items, `None` otherwise.
    #[must_use]
    pub fn checked_state(&self) -> Option<bool> {
        let Self { checked, .. } = *self;
        checked
    }
}

/// Fluent builder helpers for `EuvCodeBlock`.
impl EuvCodeBlock {
    /// Sets the syntax-highlighting language tag (e.g. `"rust"`, `"typescript"`, `"json"`, `"diff"`).
    ///
    /// # Arguments
    ///
    /// - `Self` - Source code block.
    /// - `S` - Language hint convertible into `String`.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated code block.
    #[must_use]
    pub fn language<S>(mut self, lang: S) -> Self
    where
        S: Into<String>,
    {
        self.set_language(lang.into());
        self
    }

    /// Sets an optional filename displayed in the code block header bar.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source code block.
    /// - `S` - Display filename convertible into `String`.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated code block.
    #[must_use]
    pub fn filename<S>(mut self, name: S) -> Self
    where
        S: Into<String>,
    {
        self.set_filename(Some(name.into()));
        self
    }

    /// Enables or disables the left line-number gutter.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source code block.
    /// - `bool` - Whether line numbers are visible.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated code block.
    #[must_use]
    pub fn line_numbers(mut self, show: bool) -> Self {
        self.set_show_line_numbers(show);
        self
    }

    /// Marks specific 1-based line numbers with VS Code Eva active-line highlight tint.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source code block.
    /// - `Vec<usize>` - 1-based line indices to highlight.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated code block.
    #[must_use]
    pub fn highlight_lines(mut self, lines: Vec<usize>) -> Self {
        self.set_highlight_lines(lines);
        self
    }

    /// Sets the maximum inline viewport height before vertical scrolling activates.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source code block.
    /// - `f32` - Maximum viewport height in logical pixels.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated code block.
    #[must_use]
    pub fn max_height(mut self, height: f32) -> Self {
        self.set_max_height(height);
        self
    }

    /// Controls whether the code lines inside the block are selectable.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source code block.
    /// - `bool` - `true` to enable native text selection, `false` to disable.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated code block.
    #[must_use]
    pub fn selectable(mut self, enabled: bool) -> Self {
        self.set_selectable(Some(enabled));
        self
    }

    /// Registers a callback invoked when the user clicks the Copy button.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source code block.
    /// - `F` - Copy handler receiving the raw code string.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated code block.
    #[must_use]
    pub fn on_copy<F>(mut self, handler: F) -> Self
    where
        F: Fn(&str) + 'static,
    {
        self.set_on_copy(Some(Rc::new(handler)));
        self
    }
}

/// Declarative `#[component]` implementation for `EuvCodeBlock`.
#[component]
/// Inherent implementation of [`EuvCodeBlock`].
impl EuvCodeBlock {
    /// Renders the syntax-highlighted Eva-themed code block with true H+V scrolling, header chrome, gutter, interactive Copy pill, and Fullscreen modal portal.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Code block reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted code block container handle.
    fn render(&self) -> NodeId {
        let ctx: Context = current_context();
        let tokens: ThemeTokens = self.get_tokens();
        let g: ThemeGeometry = tokens.get_geometry();
        let eva: CodeThemePalette = CodeThemePalette::resolve(tokens.get_mode());
        let Self {
            code,
            language,
            filename,
            show_line_numbers,
            highlight_lines,
            max_height,
            selectable,
            on_copy,
            ..
        } = self;
        let is_selectable: bool = selectable.unwrap_or(true);
        let inline_max_h: f32 = if *max_height > 0.0 {
            *max_height
        } else {
            CODE_DEFAULT_MAX_HEIGHT
        };
        let raw_lang: &str = language.trim();
        let resolved_lang: &str = if raw_lang.is_empty() {
            infer_code_language(code)
        } else {
            raw_lang
        };
        let display_lang_rc: Rc<str> = Rc::from(resolved_lang.to_ascii_uppercase());
        let code_lang: CodeLang = CodeLang::from_name(resolved_lang);
        let header_title_str: &str = filename.as_deref().unwrap_or(eva.get_theme_name());
        let filename_rc: Option<Rc<str>> = filename.as_deref().map(Rc::from);
        let copied_sig: Signal<bool> = Signal::new(false);
        let fs_open: Signal<bool> = Signal::new(false);
        let code_rc: Rc<str> = Rc::from(code.as_str());
        let show_nums: bool = *show_line_numbers || code.lines().nth(1).is_some();
        let (natural_w, line_count, line_nodes): (f32, usize, Vec<NodeId>) = build_code_line_nodes(
            code,
            code_lang,
            eva,
            show_nums,
            highlight_lines,
            CODE_FONT_SIZE,
            is_selectable,
        );
        let ctx_copy: Context = ctx.clone();
        let on_copy_inline: Option<MarkdownStrCallback> = on_copy.clone();
        let code_for_copy: Rc<str> = Rc::clone(&code_rc);
        let on_copy_click = move |_: &UiEvent| {
            let _: bool = ctx_copy.copy_to_clipboard(&code_for_copy);
            let _: bool = copied_sig.set(true);
            if let Some(cb) = on_copy_inline.as_ref() {
                cb(&code_for_copy);
            }
        };
        let copy_btn_text = move || -> String {
            if copied_sig.get() {
                CODE_BTN_COPIED.to_string()
            } else {
                CODE_BTN_COPY.to_string()
            }
        };
        let on_open_fs = move |_: &UiEvent| {
            let _: bool = fs_open.set_if_changed(true);
        };
        let paint_fs_icon = move |bounds: Rect, dl: &mut DisplayList| {
            let live_eva: CodeThemePalette =
                CodeThemePalette::resolve(live_tokens_untracked(tokens).get_mode());
            draw_code_fullscreen_icon(bounds, dl, live_eva.get_function());
        };
        let modal_hl: Vec<usize> = if highlight_lines.is_empty() {
            Vec::new()
        } else {
            highlight_lines.clone()
        };
        let modal_filename: Option<Rc<str>> = filename_rc;
        let modal_lang: Rc<str> = Rc::clone(&display_lang_rc);
        let modal_code_rc: Rc<str> = Rc::clone(&code_rc);
        let modal_on_copy: Option<MarkdownStrCallback> = on_copy.clone();
        let ctx_modal_copy: Context = ctx.clone();
        let build_fs_children = move || -> Vec<NodeId> {
            if !fs_open.get() {
                return Vec::new();
            }
            let live_tok: ThemeTokens = live_tokens_untracked(tokens);
            let live_p: ThemePalette = live_tok.get_palette();
            let live_g: ThemeGeometry = live_tok.get_geometry();
            let live_eva: CodeThemePalette = CodeThemePalette::resolve(live_tok.get_mode());
            let (modal_nat_w, _, modal_line_nodes): (f32, usize, Vec<NodeId>) =
                build_code_line_nodes(
                    &modal_code_rc,
                    code_lang,
                    live_eva,
                    show_nums,
                    &modal_hl,
                    CODE_MODAL_FONT_SIZE,
                    true,
                );
            let m_copy_str: Rc<str> = Rc::clone(&modal_code_rc);
            let m_copy_cb: Option<MarkdownStrCallback> = modal_on_copy.clone();
            let m_ctx: Context = ctx_modal_copy.clone();
            let on_modal_copy = move |_: &UiEvent| {
                let _: bool = m_ctx.copy_to_clipboard(&m_copy_str);
                let _: bool = copied_sig.set(true);
                if let Some(cb) = m_copy_cb.as_ref() {
                    cb(&m_copy_str);
                }
            };
            let modal_copy_label = move || -> String {
                if copied_sig.get() {
                    CODE_BTN_COPIED.to_string()
                } else {
                    CODE_BTN_COPY.to_string()
                }
            };
            let on_close_btn = move |_: &UiEvent| {
                let _: bool = fs_open.set_if_changed(false);
            };
            let m_title: &str = modal_filename
                .as_deref()
                .unwrap_or(live_eva.get_theme_name());
            let m_lang: &str = &modal_lang;
            let lines_badge: String = format!("{line_count} L");
            let mut modal_scroll_st: Style = Style::default();
            modal_scroll_st
                .get_mut_layout()
                .set_flex_direction(FlexDirection::Column)
                .set_align_items(AlignItems::Start);
            let modal_card: NodeId = view! {
                column {
                    width: Length::Percent(100.0)
                    max_width: Length::Px(CODE_MODAL_MAX_WIDTH)
                    height: Length::Percent(100.0)
                    max_height: Length::Px(CODE_MODAL_MAX_HEIGHT)
                    background: live_eva.get_background()
                    border: (1.0, live_eva.get_border())
                    radius: live_g.get_radius_lg()
                    shadow: live_g.get_shadow_lg()
                    overflow: Overflow::Hidden
                    stop_event_propagation: true
                    row {
                        width: Length::Percent(100.0)
                        padding: Edges::symmetric(SPACE_SM, SPACE_MD)
                        justify: JustifyContent::SpaceBetween
                        align: AlignItems::Center
                        gap: SPACE_SM
                        background: live_eva.get_header_bg()
                        selectable: false
                        row {
                            flex_grow: 1.0
                            flex_shrink: 1.0
                            min_width: Length::Px(0.0)
                            align: AlignItems::Center
                            gap: SPACE_SM
                            overflow: Overflow::Hidden
                            row {
                                flex_shrink: 0.0
                                gap: 5.0
                                align: AlignItems::Center
                                view {
                                    width: Length::Px(9.0)
                                    height: Length::Px(9.0)
                                    radius: 99.0
                                    background: Color::hex(CODE_DOT_RED)
                                }
                                view {
                                    width: Length::Px(9.0)
                                    height: Length::Px(9.0)
                                    radius: 99.0
                                    background: Color::hex(CODE_DOT_YELLOW)
                                }
                                view {
                                    width: Length::Px(9.0)
                                    height: Length::Px(9.0)
                                    radius: 99.0
                                    background: Color::hex(CODE_DOT_GREEN)
                                }
                            }
                            text {
                                flex_shrink: 1.0
                                min_width: Length::Px(0.0)
                                overflow: Overflow::Hidden
                                text_overflow: TextOverflow::Ellipsis
                                font_size: 13.0
                                font_weight: FontWeight::SemiBold
                                color: live_eva.get_foreground()
                                { m_title }
                            }
                        }
                        row {
                            flex_shrink: 0.0
                            align: AlignItems::Center
                            gap: SPACE_SM
                            row {
                                padding: Edges::symmetric(3.0, 8.0)
                                radius: live_g.get_radius_sm()
                                background: live_eva.get_background()
                                border: (1.0, live_eva.get_border())
                                text {
                                    font_size: 10.5
                                    font_weight: FontWeight::Bold
                                    color: live_eva.get_keyword()
                                    text_overflow: TextOverflow::Clip
                                    { m_lang }
                                }
                            }
                            row {
                                padding: Edges::symmetric(3.0, 8.0)
                                radius: live_g.get_radius_sm()
                                background: live_eva.get_background()
                                border: (1.0, live_eva.get_border())
                                text {
                                    font_size: 10.5
                                    font_family: FontFamily::Monospace
                                    color: live_eva.get_line_number()
                                    text_overflow: TextOverflow::Clip
                                    { lines_badge }
                                }
                            }
                            row {
                                padding: Edges::symmetric(4.0, 10.0)
                                radius: live_g.get_radius_sm()
                                cursor: CursorIcon::Pointer
                                background: live_eva.get_background()
                                border: (1.0, live_eva.get_border())
                                on_click: on_modal_copy
                                text {
                                    font_size: 11.5
                                    font_weight: FontWeight::SemiBold
                                    color: live_eva.get_function()
                                    text_overflow: TextOverflow::Clip
                                    { modal_copy_label }
                                }
                            }
                            row {
                                padding: Edges::symmetric(4.0, 10.0)
                                radius: live_g.get_radius_sm()
                                cursor: CursorIcon::Pointer
                                background: live_p.get_secondary()
                                border: (1.0, live_p.get_border())
                                on_click: on_close_btn
                                text {
                                    font_size: 11.5
                                    font_weight: FontWeight::SemiBold
                                    color: live_p.get_secondary_foreground()
                                    text_overflow: TextOverflow::Clip
                                    { CODE_BTN_CLOSE }
                                }
                            }
                        }
                    }
                    view {
                        width: Length::Percent(100.0)
                        height: Length::Px(1.0)
                        background: live_eva.get_border()
                    }
                    scroll {
                        style: modal_scroll_st
                        flex_grow: 1.0
                        flex_shrink: 1.0
                        min_height: Length::Px(0.0)
                        width: Length::Percent(100.0)
                        padding: Edges::symmetric(SPACE_MD, SPACE_SM)
                        column {
                            min_width: Length::Px(modal_nat_w)
                            width: Length::Percent(100.0)
                            flex_shrink: 0.0
                            gap: 2.0
                            selectable: true
                            ..modal_line_nodes
                        }
                    }
                }
            };
            vec![modal_card]
        };
        let on_scrim_click = move |_: &UiEvent| {
            let _: bool = fs_open.set_if_changed(false);
        };
        let inline_lang_str: &str = &display_lang_rc;
        let mut inline_scroll_st: Style = Style::default();
        inline_scroll_st
            .get_mut_layout()
            .set_flex_direction(FlexDirection::Column)
            .set_align_items(AlignItems::Start);
        let mut fs_portal: NodeId = NodeId::default();
        let root: NodeId = view! {
            column {
                width: Length::Percent(100.0)
                background: eva.get_background()
                border: (1.0, eva.get_border())
                radius: g.get_radius_lg()
                overflow: Overflow::Hidden
                row {
                    width: Length::Percent(100.0)
                    padding: Edges::symmetric(SPACE_SM, SPACE_MD)
                    justify: JustifyContent::SpaceBetween
                    align: AlignItems::Center
                    gap: SPACE_SM
                    overflow: Overflow::Hidden
                    background: eva.get_header_bg()
                    selectable: false
                    row {
                        flex_grow: 1.0
                        flex_shrink: 1.0
                        min_width: Length::Px(0.0)
                        align: AlignItems::Center
                        gap: SPACE_SM
                        overflow: Overflow::Hidden
                        row {
                            flex_shrink: 0.0
                            gap: 5.0
                            align: AlignItems::Center
                            view {
                                width: Length::Px(9.0)
                                height: Length::Px(9.0)
                                radius: 99.0
                                background: Color::hex(CODE_DOT_RED)
                            }
                            view {
                                width: Length::Px(9.0)
                                height: Length::Px(9.0)
                                radius: 99.0
                                background: Color::hex(CODE_DOT_YELLOW)
                            }
                            view {
                                width: Length::Px(9.0)
                                height: Length::Px(9.0)
                                radius: 99.0
                                background: Color::hex(CODE_DOT_GREEN)
                            }
                        }
                        text {
                            flex_shrink: 1.0
                            min_width: Length::Px(0.0)
                            overflow: Overflow::Hidden
                            text_overflow: TextOverflow::Ellipsis
                            font_size: 12.0
                            font_weight: FontWeight::SemiBold
                            color: eva.get_foreground()
                            { header_title_str }
                        }
                    }
                    row {
                        flex_shrink: 0.0
                        align: AlignItems::Center
                        gap: SPACE_XS
                        row {
                            padding: Edges::symmetric(3.0, 8.0)
                            radius: g.get_radius_sm()
                            justify: JustifyContent::Center
                            align: AlignItems::Center
                            background: eva.get_background()
                            border: (1.0, eva.get_border())
                            text {
                                font_size: 10.5
                                font_weight: FontWeight::Bold
                                color: eva.get_keyword()
                                text_align: TextAlign::Center
                                text_overflow: TextOverflow::Clip
                                { inline_lang_str }
                            }
                        }
                        row {
                            padding: Edges::symmetric(3.0, 8.0)
                            radius: g.get_radius_sm()
                            gap: 4.0
                            justify: JustifyContent::Center
                            align: AlignItems::Center
                            cursor: CursorIcon::Pointer
                            background: eva.get_background()
                            border: (1.0, eva.get_border())
                            on_click: on_open_fs
                            canvas {
                                width: Length::Px(12.0)
                                height: Length::Px(12.0)
                                painter: paint_fs_icon
                            }
                            text {
                                font_size: 11.0
                                font_weight: FontWeight::SemiBold
                                color: eva.get_function()
                                text_align: TextAlign::Center
                                text_overflow: TextOverflow::Clip
                                { CODE_BTN_FULLSCREEN }
                            }
                        }
                        row {
                            padding: Edges::symmetric(3.0, 10.0)
                            radius: g.get_radius_sm()
                            justify: JustifyContent::Center
                            align: AlignItems::Center
                            cursor: CursorIcon::Pointer
                            background: eva.get_background()
                            border: (1.0, eva.get_border())
                            on_click: on_copy_click
                            text {
                                font_size: 11.0
                                font_weight: FontWeight::SemiBold
                                color: eva.get_function()
                                text_align: TextAlign::Center
                                text_overflow: TextOverflow::Clip
                                { copy_btn_text }
                            }
                        }
                    }
                }
                view {
                    width: Length::Percent(100.0)
                    height: Length::Px(1.0)
                    background: eva.get_border()
                }
                scroll {
                    style: inline_scroll_st
                    width: Length::Percent(100.0)
                    max_height: Length::Px(inline_max_h)
                    padding: Edges::symmetric(SPACE_SM, SPACE_XS)
                    column {
                        min_width: Length::Px(natural_w)
                        width: Length::Percent(100.0)
                        flex_shrink: 0.0
                        gap: 1.0
                        selectable: is_selectable
                        ..line_nodes
                    }
                }
                portal {
                    node_ref: fs_portal
                    display: Display::None
                    position: Position::Fixed
                    inset: (Some(0.0), Some(0.0), Some(0.0), Some(0.0))
                    width: Length::Percent(100.0)
                    height: Length::Percent(100.0)
                    padding: Edges::all(CODE_MODAL_PADDING)
                    justify: JustifyContent::Center
                    align: AlignItems::Center
                    background: tokens.get_palette().get_overlay()
                    stop_scroll_propagation: true
                    z_index: Z_INDEX_MODAL
                    on_click: on_scrim_click
                    bind_children: build_fs_children
                }
            }
        };
        let handle: SceneHandle = ctx.get_scene().clone();
        let prev_open: Rc<Cell<bool>> = Rc::new(Cell::new(false));
        let (ov_scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            let _: Effect =
                EuvOverlayStack::bind_signal(fs_open, CODE_FULLSCREEN_OVERLAY_NAME, Z_INDEX_MODAL);
            Effect::new(move || {
                let is_open: bool = fs_open.get();
                if is_open == prev_open.replace(is_open) {
                    return;
                }
                if is_open {
                    handle.with_write(
                        |tree: &mut SceneTree| {
                            if let Some(n) = tree.get_node_mut(fs_portal) {
                                let mut st: Style = n.get_style();
                                st.get_mut_paint().set_pointer_events(PointerEvents::Auto);
                                n.set_style(st);
                            }
                        },
                        (),
                    );
                    handle.start_transition(
                        NodeTransition::fade(fs_portal, 0.0, 1.0, 0.18, false)
                            .with_easing(Easing::EaseOutCubic),
                    );
                } else {
                    handle.with_write(
                        |tree: &mut SceneTree| {
                            if let Some(n) = tree.get_node_mut(fs_portal) {
                                let mut st: Style = n.get_style();
                                st.get_mut_paint().set_pointer_events(PointerEvents::None);
                                n.set_style(st);
                            }
                        },
                        (),
                    );
                    handle.start_transition(
                        NodeTransition::fade(fs_portal, 1.0, 0.0, 0.16, true)
                            .with_easing(Easing::EaseOutCubic),
                    );
                }
            })
        });
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                tree.attach_scope(root, ov_scope);
            },
            (),
        );
        root
    }
}

/// Fluent builder helpers for `EuvMarkdownHeading`.
impl EuvMarkdownHeading {
    /// Controls whether the heading text is selectable.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source heading.
    /// - `bool` - Whether selection is enabled.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated heading.
    #[must_use]
    pub fn selectable(mut self, enabled: bool) -> Self {
        self.set_selectable(Some(enabled));
        self
    }
}

/// Declarative `#[component]` implementation for `EuvMarkdownHeading`.
#[component]
/// Inherent implementation of [`EuvMarkdownHeading`].
impl EuvMarkdownHeading {
    /// Renders a Markdown heading (`H1`–`H6`) with crisp scale and subtle bottom rule for `H1`/`H2`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Heading component reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted heading node handle.
    fn render(&self) -> NodeId {
        let tokens: ThemeTokens = self.get_tokens();
        let p: ThemePalette = tokens.get_palette();
        let eva: CodeThemePalette = CodeThemePalette::resolve(tokens.get_mode());
        let lvl: u8 = self.get_level().clamp(1, 6);
        let size: f32 = match lvl {
            1 => 24.0,
            2 => 19.5,
            3 => 16.5,
            4 => 15.0,
            5 => 13.5,
            _ => 12.5,
        };
        let Self {
            text, selectable, ..
        } = self;
        let is_selectable: bool = selectable.unwrap_or(true);
        let show_rule: bool = lvl <= 2;
        let text_str: &str = text.as_str();
        view! {
            column {
                width: Length::Percent(100.0)
                gap: SPACE_XS
                padding: Edges::new(if lvl <= 2 { SPACE_SM } else { SPACE_XS }, 0.0, 0.0, 0.0)
                row {
                    width: Length::Percent(100.0)
                    align: AlignItems::Center
                    gap: SPACE_SM
                    if lvl == 1 {
                        view {
                            width: Length::Px(4.0)
                            height: Length::Px(22.0)
                            radius: 99.0
                            background: eva.get_keyword()
                        }
                    }
                    text {
                        flex_grow: 1.0
                        flex_shrink: 1.0
                        min_width: Length::Px(0.0)
                        font_size: size
                        font_weight: FontWeight::Bold
                        color: if lvl == 6 { p.get_muted_foreground() } else { p.get_foreground() }
                        selectable: is_selectable
                        { text_str }
                    }
                }
                if show_rule {
                    view {
                        width: Length::Percent(100.0)
                        height: Length::Px(1.0)
                        background: p.get_border()
                    }
                }
            }
        }
    }
}

/// Declarative `#[component]` implementation for `EuvMarkdownInlineCode`.
#[component]
/// Inherent implementation of [`EuvMarkdownInlineCode`].
impl EuvMarkdownInlineCode {
    /// Renders an inline monospace code pill styled with exact VS Code Eva inline-code colors.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Inline code component reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted inline code pill handle.
    fn render(&self) -> NodeId {
        let tokens: ThemeTokens = self.get_tokens();
        let g: ThemeGeometry = tokens.get_geometry();
        let eva: CodeThemePalette = CodeThemePalette::resolve(tokens.get_mode());
        let snippet: &str = self.get_code().as_str();
        view! {
            row {
                max_width: Length::Percent(100.0)
                padding: Edges::symmetric(2.0, 6.0)
                radius: g.get_radius_sm().min(6.0)
                background: eva.get_inline_code_bg()
                border: (1.0, eva.get_border())
                justify: JustifyContent::Center
                align: AlignItems::Center
                overflow: Overflow::Hidden
                text {
                    max_width: Length::Percent(100.0)
                    overflow: Overflow::Hidden
                    text_overflow: TextOverflow::Clip
                    text_align: TextAlign::Center
                    font_size: 12.0
                    font_family: FontFamily::Monospace
                    font_weight: FontWeight::Normal
                    color: eva.get_inline_code_fg()
                    { snippet }
                }
            }
        }
    }
}

/// Fluent builder helpers for `EuvMarkdownMath`.
impl EuvMarkdownMath {
    /// Controls whether the math expression text is selectable.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source math block.
    /// - `bool` - Whether selection is enabled.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated math block.
    #[must_use]
    pub fn selectable(mut self, enabled: bool) -> Self {
        self.set_selectable(Some(enabled));
        self
    }
}

/// Declarative `#[component]` implementation for `EuvMarkdownMath`.
#[component]
/// Inherent implementation of [`EuvMarkdownMath`].
impl EuvMarkdownMath {
    /// Renders a display LaTeX/math formula card formatted with crisp Unicode mathematical typography.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Math block component reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted display math card handle.
    fn render(&self) -> NodeId {
        let tokens: ThemeTokens = self.get_tokens();
        let p: ThemePalette = tokens.get_palette();
        let g: ThemeGeometry = tokens.get_geometry();
        let eva: CodeThemePalette = CodeThemePalette::resolve(tokens.get_mode());
        let Self {
            expression,
            selectable,
            ..
        } = self;
        let is_selectable: bool = selectable.unwrap_or(true);
        let formatted_lines: Vec<String> = expression
            .lines()
            .map(format_latex_math)
            .filter(|s: &String| !s.is_empty())
            .collect();
        let display_lines: Vec<String> = if formatted_lines.is_empty() {
            vec![format_latex_math(expression)]
        } else {
            formatted_lines
        };
        let line_nodes: Vec<NodeId> = display_lines
            .into_iter()
            .map(|line_str: String| {
                view! {
                    text {
                        width: Length::Percent(100.0)
                        font_size: 16.0
                        font_family: FontFamily::Monospace
                        font_weight: FontWeight::Medium
                        color: p.get_foreground()
                        text_align: TextAlign::Center
                        selectable: is_selectable
                        { line_str }
                    }
                }
            })
            .collect();
        view! {
            column {
                width: Length::Percent(100.0)
                padding: Edges::symmetric(SPACE_MD, SPACE_MD)
                gap: SPACE_XS
                background: eva.get_header_bg()
                border: (1.0, eva.get_border())
                radius: g.get_radius_md()
                overflow: Overflow::Hidden
                row {
                    width: Length::Percent(100.0)
                    justify: JustifyContent::End
                    selectable: false
                    row {
                        padding: Edges::symmetric(2.0, 6.0)
                        radius: g.get_radius_sm()
                        background: eva.get_background()
                        border: (1.0, eva.get_border())
                        text {
                            font_size: 9.5
                            font_weight: FontWeight::Bold
                            color: eva.get_keyword()
                            { MD_MATH_BADGE }
                        }
                    }
                }
                ..line_nodes
            }
        }
    }
}

/// Fluent builder helpers for `EuvMarkdownParagraph`.
impl EuvMarkdownParagraph {
    /// Controls whether paragraph text is selectable.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source paragraph.
    /// - `bool` - Whether text selection is enabled.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated paragraph.
    #[must_use]
    pub fn selectable(mut self, enabled: bool) -> Self {
        self.set_selectable(Some(enabled));
        self
    }

    /// Registers a hyperlink click handler.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source paragraph.
    /// - `F` - Callback invoked with clicked link URL.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated paragraph.
    #[must_use]
    pub fn on_link<F>(mut self, handler: F) -> Self
    where
        F: Fn(&str) + 'static,
    {
        self.set_on_link(Some(Rc::new(handler)));
        self
    }

    /// Mounts a plain body text paragraph node directly without allocating an inline AST.
    ///
    /// # Arguments
    ///
    /// - `&str` - Plain body text slice.
    /// - `ThemePalette` - Resolved active theme palette.
    /// - `bool` - Whether text selection is enabled.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted body text node handle.
    #[inline(always)]
    pub(crate) fn mount_plain_text(text: &str, p: ThemePalette, is_selectable: bool) -> NodeId {
        view! {
            text {
                width: Length::Percent(100.0)
                font_size: 14.0
                color: p.get_foreground()
                selectable: is_selectable
                { text }
            }
        }
    }

    /// Mounts a single parsed `EuvMarkdownInline` segment by reference without cloning the parent vector.
    ///
    /// # Arguments
    ///
    /// - `&EuvMarkdownInline` - Borrowed inline segment.
    /// - `ThemeTokens` - Active theme tokens.
    /// - `CodeThemePalette` - Resolved Eva palette.
    /// - `(Color, Color)` - Highlight background and foreground colors.
    /// - `bool` - Whether text is selectable.
    /// - `Option<&MarkdownStrCallback>` - Optional link-click handler.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted inline node handle.
    fn render_inline_node(
        seg: &EuvMarkdownInline,
        tokens: ThemeTokens,
        eva: CodeThemePalette,
        hl_colors: (Color, Color),
        is_selectable: bool,
        on_link: Option<&MarkdownStrCallback>,
    ) -> NodeId {
        let p: ThemePalette = tokens.get_palette();
        let g: ThemeGeometry = tokens.get_geometry();
        let (hl_bg, hl_fg): (Color, Color) = hl_colors;
        match seg {
            EuvMarkdownInline::Text(t) => view! {
                text {
                    max_width: Length::Percent(100.0)
                    font_size: 14.0
                    color: p.get_foreground()
                    selectable: is_selectable
                    { t.as_str() }
                }
            },
            EuvMarkdownInline::Bold(t) => view! {
                text {
                    max_width: Length::Percent(100.0)
                    font_size: 14.0
                    font_weight: FontWeight::Bold
                    color: p.get_foreground()
                    selectable: is_selectable
                    { t.as_str() }
                }
            },
            EuvMarkdownInline::Italic(t) => view! {
                text {
                    max_width: Length::Percent(100.0)
                    font_size: 14.0
                    italic: true
                    color: p.get_foreground()
                    selectable: is_selectable
                    { t.as_str() }
                }
            },
            EuvMarkdownInline::BoldItalic(t) => view! {
                text {
                    max_width: Length::Percent(100.0)
                    font_size: 14.0
                    font_weight: FontWeight::Bold
                    italic: true
                    color: p.get_foreground()
                    selectable: is_selectable
                    { t.as_str() }
                }
            },
            EuvMarkdownInline::Strike(t) => {
                let mut strike_st: Style = Style::default();
                strike_st
                    .get_mut_layout()
                    .set_max_width(Length::Percent(100.0));
                strike_st
                    .get_mut_text()
                    .set_font_size(14.0)
                    .set_color(p.get_muted_foreground())
                    .set_strikethrough(true);
                view! {
                    text {
                        style: strike_st
                        selectable: is_selectable
                        { t.as_str() }
                    }
                }
            }
            EuvMarkdownInline::Highlight(t) => view! {
                row {
                    max_width: Length::Percent(100.0)
                    padding: Edges::symmetric(1.0, 5.0)
                    radius: g.get_radius_sm().min(5.0)
                    background: hl_bg
                    overflow: Overflow::Hidden
                    text {
                        max_width: Length::Percent(100.0)
                        font_size: 13.5
                        font_weight: FontWeight::Medium
                        color: hl_fg
                        selectable: is_selectable
                        { t.as_str() }
                    }
                }
            },
            EuvMarkdownInline::Superscript(t) => view! {
                row {
                    padding: Edges::new(0.0, 1.0, 6.0, 1.0)
                    text {
                        font_size: 10.5
                        font_weight: FontWeight::SemiBold
                        color: p.get_foreground()
                        selectable: is_selectable
                        { t.as_str() }
                    }
                }
            },
            EuvMarkdownInline::Subscript(t) => view! {
                row {
                    padding: Edges::new(6.0, 1.0, 0.0, 1.0)
                    text {
                        font_size: 10.5
                        font_weight: FontWeight::SemiBold
                        color: p.get_foreground()
                        selectable: is_selectable
                        { t.as_str() }
                    }
                }
            },
            EuvMarkdownInline::Code(code) => view! {
                EuvMarkdownInlineCode {
                    code: code.clone()
                    tokens: tokens
                }
            },
            EuvMarkdownInline::Math(expr) => view! {
                row {
                    max_width: Length::Percent(100.0)
                    padding: Edges::symmetric(1.0, 6.0)
                    radius: g.get_radius_sm().min(5.0)
                    background: eva.get_inline_code_bg()
                    border: (1.0, eva.get_border())
                    overflow: Overflow::Hidden
                    text {
                        max_width: Length::Percent(100.0)
                        font_size: 13.0
                        font_family: FontFamily::Monospace
                        color: eva.get_keyword()
                        selectable: is_selectable
                        { expr.as_str() }
                    }
                }
            },
            EuvMarkdownInline::FootnoteRef(id) => view! {
                row {
                    padding: Edges::new(0.0, 4.0, 5.0, 4.0)
                    radius: g.get_radius_sm().min(4.0)
                    background: p.get_primary().with_alpha(28)
                    text {
                        font_size: 10.0
                        font_weight: FontWeight::Bold
                        color: p.get_primary()
                        { format!("[{id}]") }
                    }
                }
            },
            EuvMarkdownInline::Image { alt, url } => {
                let caption: &str = if alt.is_empty() {
                    url.as_str()
                } else {
                    alt.as_str()
                };
                view! {
                    row {
                        max_width: Length::Percent(100.0)
                        padding: Edges::symmetric(2.0, 6.0)
                        gap: 4.0
                        radius: g.get_radius_sm()
                        background: eva.get_header_bg()
                        border: (1.0, eva.get_border())
                        align: AlignItems::Center
                        overflow: Overflow::Hidden
                        text {
                            font_size: 10.0
                            font_weight: FontWeight::Bold
                            color: eva.get_keyword()
                            { MD_IMAGE_BADGE }
                        }
                        text {
                            max_width: Length::Percent(100.0)
                            font_size: 12.5
                            color: p.get_foreground()
                            { caption }
                        }
                    }
                }
            }
            EuvMarkdownInline::Kbd(key) => view! {
                EuvKbd {
                    key: key.clone()
                    tokens: tokens
                }
            },
            EuvMarkdownInline::Link { label, url } => {
                let cb_opt: Option<MarkdownStrCallback> = on_link.cloned();
                let url_owned: String = url.clone();
                let on_link_click = move |_: &UiEvent| {
                    if let Some(cb) = cb_opt.as_ref() {
                        cb(&url_owned);
                    }
                };
                let mut link_st: Style = Style::default();
                link_st
                    .get_mut_layout()
                    .set_max_width(Length::Percent(100.0));
                link_st
                    .get_mut_text()
                    .set_font_size(14.0)
                    .set_weight(FontWeight::SemiBold)
                    .set_color(eva.get_link())
                    .set_underline(true);
                view! {
                    row {
                        max_width: Length::Percent(100.0)
                        padding: Edges::symmetric(1.0, 3.0)
                        radius: 4.0
                        cursor: CursorIcon::Pointer
                        on_click: on_link_click
                        text {
                            style: link_st
                            selectable: is_selectable
                            { label.as_str() }
                        }
                    }
                }
            }
            EuvMarkdownInline::HardBreak => view! {
                view {
                    width: Length::Px(0.0)
                    height: Length::Px(0.0)
                }
            },
        }
    }
}

/// Declarative `#[component]` implementation for `EuvMarkdownParagraph`.
#[component]
/// Inherent implementation of [`EuvMarkdownParagraph`].
impl EuvMarkdownParagraph {
    /// Renders rich inline Markdown spans (bold, italic, bold-italic, strike, highlight, sup/sub, inline code, math, footnotes, images, links, hard breaks, `<kbd>`).
    ///
    /// # Arguments
    ///
    /// - `&Self` - Paragraph component reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted paragraph container handle.
    fn render(&self) -> NodeId {
        let tokens: ThemeTokens = self.get_tokens();
        let p: ThemePalette = tokens.get_palette();
        let eva: CodeThemePalette = CodeThemePalette::resolve(tokens.get_mode());
        let Self {
            inlines,
            selectable,
            on_link,
            ..
        } = self;
        let is_selectable: bool = selectable.unwrap_or(true);
        if inlines.len() == 1
            && let EuvMarkdownInline::Text(plain) = &inlines[0]
        {
            return Self::mount_plain_text(plain.as_str(), p, is_selectable);
        }
        let has_rich: bool = inlines
            .iter()
            .any(|seg: &EuvMarkdownInline| !matches!(seg, EuvMarkdownInline::Text(_)));
        if !has_rich {
            let total_len: usize = inlines
                .iter()
                .map(|seg: &EuvMarkdownInline| match seg {
                    EuvMarkdownInline::Text(s) => s.len(),
                    _ => 0,
                })
                .sum();
            let mut plain: String = String::with_capacity(total_len);
            for seg in inlines {
                if let EuvMarkdownInline::Text(s) = seg {
                    plain.push_str(s);
                }
            }
            return Self::mount_plain_text(&plain, p, is_selectable);
        }
        let hl_colors: (Color, Color) = match tokens.get_mode() {
            ThemeMode::Light => (
                Color::hex(MD_HIGHLIGHT_LIGHT_BG),
                Color::hex(MD_HIGHLIGHT_LIGHT_FG),
            ),
            ThemeMode::Dark => (
                Color::hex(MD_HIGHLIGHT_DARK_BG),
                Color::hex(MD_HIGHLIGHT_DARK_FG),
            ),
        };
        let on_link_ref: Option<&MarkdownStrCallback> = on_link.as_ref();
        let has_hard_break: bool = inlines
            .iter()
            .any(|seg: &EuvMarkdownInline| matches!(seg, EuvMarkdownInline::HardBreak));
        if !has_hard_break {
            let nodes: Vec<NodeId> = inlines
                .iter()
                .map(|seg: &EuvMarkdownInline| {
                    Self::render_inline_node(
                        seg,
                        tokens,
                        eva,
                        hl_colors,
                        is_selectable,
                        on_link_ref,
                    )
                })
                .collect();
            return view! {
                row {
                    width: Length::Percent(100.0)
                    min_width: Length::Px(0.0)
                    flex_wrap: FlexWrap::Wrap
                    align: AlignItems::Center
                    overflow: Overflow::Hidden
                    selectable: is_selectable
                    gap: 4.0
                    ..nodes
                }
            };
        }
        let mut line_rows: Vec<NodeId> = Vec::new();
        for line_segs in
            inlines.split(|seg: &EuvMarkdownInline| matches!(seg, EuvMarkdownInline::HardBreak))
        {
            let nodes: Vec<NodeId> = line_segs
                .iter()
                .map(|seg: &EuvMarkdownInline| {
                    Self::render_inline_node(
                        seg,
                        tokens,
                        eva,
                        hl_colors,
                        is_selectable,
                        on_link_ref,
                    )
                })
                .collect();
            line_rows.push(view! {
                row {
                    width: Length::Percent(100.0)
                    min_width: Length::Px(0.0)
                    flex_wrap: FlexWrap::Wrap
                    align: AlignItems::Center
                    overflow: Overflow::Hidden
                    selectable: is_selectable
                    gap: 4.0
                    ..nodes
                }
            });
        }
        if line_rows.len() == 1 {
            return line_rows.remove(0);
        }
        view! {
            column {
                width: Length::Percent(100.0)
                gap: 4.0
                selectable: is_selectable
                ..line_rows
            }
        }
    }
}

/// Fluent builder helpers for `EuvMarkdownQuote`.
impl EuvMarkdownQuote {
    /// Controls whether quote text is selectable.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source quote.
    /// - `bool` - Whether selection is enabled.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated quote.
    #[must_use]
    pub fn selectable(mut self, enabled: bool) -> Self {
        self.set_selectable(Some(enabled));
        self
    }

    /// Registers a link-click handler for hyperlinks inside the quote.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source quote.
    /// - `F` - Callback invoked with clicked link URL.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated quote.
    #[must_use]
    pub fn on_link<F>(mut self, handler: F) -> Self
    where
        F: Fn(&str) + 'static,
    {
        self.set_on_link(Some(Rc::new(handler)));
        self
    }
}

/// Declarative `#[component]` implementation for `EuvMarkdownQuote`.
#[component]
/// Inherent implementation of [`EuvMarkdownQuote`].
impl EuvMarkdownQuote {
    /// Renders a Markdown blockquote (including nested `> >` quotes) or GitHub Alert callout card with Eva-aligned border and tint.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Quote component reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted callout node handle.
    fn render(&self) -> NodeId {
        let tokens: ThemeTokens = self.get_tokens();
        let p: ThemePalette = tokens.get_palette();
        let g: ThemeGeometry = tokens.get_geometry();
        let eva: CodeThemePalette = CodeThemePalette::resolve(tokens.get_mode());
        let kind: EuvMarkdownAlertKind = self.get_kind();
        let Self {
            lines,
            selectable,
            on_link,
            ..
        } = self;
        let is_selectable: bool = selectable.unwrap_or(true);
        let (accent, badge_opt): (Color, Option<&str>) = match kind {
            EuvMarkdownAlertKind::Quote => (eva.get_quote_border(), None),
            EuvMarkdownAlertKind::Note => (eva.get_link(), Some(MD_ALERT_LABEL_NOTE)),
            EuvMarkdownAlertKind::Tip => (eva.get_diff_add(), Some(MD_ALERT_LABEL_TIP)),
            EuvMarkdownAlertKind::Important => (eva.get_keyword(), Some(MD_ALERT_LABEL_IMPORTANT)),
            EuvMarkdownAlertKind::Warning => (p.get_warning(), Some(MD_ALERT_LABEL_WARNING)),
            EuvMarkdownAlertKind::Caution => (eva.get_diff_del(), Some(MD_ALERT_LABEL_CAUTION)),
        };
        let bg: Color = if kind == EuvMarkdownAlertKind::Quote {
            eva.get_quote_bg()
        } else {
            accent.with_alpha(26)
        };
        let mut body_nodes: Vec<NodeId> = Vec::with_capacity(lines.len());
        let mut nested_buf: Vec<String> = Vec::new();
        for line in lines {
            if let Some(after_gt) = line.trim_start().strip_prefix('>') {
                nested_buf.push(after_gt.trim_start().to_string());
            } else {
                if !nested_buf.is_empty() {
                    let inner_lines: Vec<String> = take(&mut nested_buf);
                    let (inner_kind, clean_inner): (EuvMarkdownAlertKind, Vec<String>) =
                        classify_quote_alert(inner_lines);
                    let mut nested: EuvMarkdownQuote =
                        EuvMarkdownQuote::new(inner_kind, clean_inner, tokens)
                            .selectable(is_selectable);
                    if let Some(cb) = on_link.clone() {
                        nested.set_on_link(Some(cb));
                    }
                    body_nodes.push(nested.mount());
                }
                if !line.trim().is_empty() {
                    if !has_inline_markdown_trigger(line) {
                        body_nodes.push(EuvMarkdownParagraph::mount_plain_text(
                            line,
                            p,
                            is_selectable,
                        ));
                    } else {
                        let mut para: EuvMarkdownParagraph =
                            EuvMarkdownParagraph::new(parse_markdown_inlines(line), tokens)
                                .selectable(is_selectable);
                        if let Some(cb) = on_link.clone() {
                            para.set_on_link(Some(cb));
                        }
                        body_nodes.push(para.mount());
                    }
                }
            }
        }
        if !nested_buf.is_empty() {
            let inner_lines: Vec<String> = take(&mut nested_buf);
            let (inner_kind, clean_inner): (EuvMarkdownAlertKind, Vec<String>) =
                classify_quote_alert(inner_lines);
            let mut nested: EuvMarkdownQuote =
                EuvMarkdownQuote::new(inner_kind, clean_inner, tokens).selectable(is_selectable);
            if let Some(cb) = on_link.clone() {
                nested.set_on_link(Some(cb));
            }
            body_nodes.push(nested.mount());
        }
        view! {
            row {
                width: Length::Percent(100.0)
                background: bg
                border: (1.0, accent.with_alpha(96))
                radius: g.get_radius_md()
                overflow: Overflow::Hidden
                selectable: is_selectable
                view {
                    width: Length::Px(4.0)
                    flex_shrink: 0.0
                    background: accent
                }
                column {
                    flex_grow: 1.0
                    flex_shrink: 1.0
                    min_width: Length::Px(0.0)
                    padding: Edges::symmetric(SPACE_SM, SPACE_MD)
                    gap: SPACE_XS
                    overflow: Overflow::Hidden
                    if let Some(tag) = badge_opt {
                        row {
                            align: AlignItems::Center
                            gap: 6.0
                            view {
                                width: Length::Px(7.0)
                                height: Length::Px(7.0)
                                radius: 99.0
                                background: accent
                            }
                            text {
                                font_size: 11.5
                                font_weight: FontWeight::Bold
                                color: accent
                                { tag }
                            }
                        }
                    }
                    ..body_nodes
                }
            }
        }
    }
}

/// Fluent builder helpers for `EuvMarkdownList`.
impl EuvMarkdownList {
    /// Sets the starting ordinal index for ordered lists.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source list.
    /// - `usize` - Starting number (defaults to `1` when `0`).
    ///
    /// # Returns
    ///
    /// - `Self` - Updated list.
    #[must_use]
    pub fn start(mut self, start: usize) -> Self {
        self.set_start(start);
        self
    }

    /// Controls whether list item text is selectable.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source list.
    /// - `bool` - Whether selection is enabled.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated list.
    #[must_use]
    pub fn selectable(mut self, enabled: bool) -> Self {
        self.set_selectable(Some(enabled));
        self
    }

    /// Registers a link-click handler for hyperlinks inside list items.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source list.
    /// - `F` - Callback invoked with clicked URL.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated list.
    #[must_use]
    pub fn on_link<F>(mut self, handler: F) -> Self
    where
        F: Fn(&str) + 'static,
    {
        self.set_on_link(Some(Rc::new(handler)));
        self
    }

    /// Registers an interactive task-checkbox toggle handler.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source list.
    /// - `F` - Callback receiving `(item_index, next_checked)`.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated list.
    #[must_use]
    pub fn on_toggle_task<F>(mut self, handler: F) -> Self
    where
        F: Fn(usize, bool) + 'static,
    {
        self.set_on_toggle_task(Some(Rc::new(handler)));
        self
    }
}

/// Declarative `#[component]` implementation for `EuvMarkdownList`.
#[component]
/// Inherent implementation of [`EuvMarkdownList`].
impl EuvMarkdownList {
    /// Renders unordered, ordered, nested, or interactive task-checklist Markdown items.
    ///
    /// # Arguments
    ///
    /// - `&Self` - List component reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted list column handle.
    fn render(&self) -> NodeId {
        let tokens: ThemeTokens = self.get_tokens();
        let p: ThemePalette = tokens.get_palette();
        let g: ThemeGeometry = tokens.get_geometry();
        let eva: CodeThemePalette = CodeThemePalette::resolve(tokens.get_mode());
        let ordered: bool = self.get_ordered();
        let start_num: usize = self.get_start().max(1);
        let Self {
            items,
            selectable,
            on_link,
            on_toggle_task,
            ..
        } = self;
        let is_selectable: bool = selectable.unwrap_or(true);
        let row_nodes: Vec<NodeId> = items
            .iter()
            .enumerate()
            .map(|(idx, item): (usize, &EuvMarkdownListItem)| {
                let indent: f32 = (item.get_depth() as f32) * 18.0;
                let text_ref: &str = item.get_text().as_str();
                let para_node: NodeId = if !has_inline_markdown_trigger(text_ref) {
                    EuvMarkdownParagraph::mount_plain_text(text_ref, p, is_selectable)
                } else {
                    let mut para: EuvMarkdownParagraph =
                        EuvMarkdownParagraph::new(parse_markdown_inlines(text_ref), tokens)
                            .selectable(is_selectable);
                    if let Some(cb) = on_link.clone() {
                        para.set_on_link(Some(cb));
                    }
                    para.mount()
                };
                if let Some(initial_checked) = item.checked_state() {
                    let chk_sig: Signal<bool> = Signal::new(initial_checked);
                    let cb_opt: Option<MarkdownTaskCallback> = on_toggle_task.clone();
                    let on_toggle_click = move |_: &UiEvent| {
                        let next: bool = !chk_sig.get_untracked();
                        let _: bool = chk_sig.set(next);
                        if let Some(cb) = cb_opt.as_ref() {
                            cb(idx, next);
                        }
                    };
                    let chk_label = move || -> String {
                        if chk_sig.get() {
                            MD_CHECKBOX_ON.to_string()
                        } else {
                            MD_CHECKBOX_OFF.to_string()
                        }
                    };
                    view! {
                        row {
                            width: Length::Percent(100.0)
                            padding: Edges::new(2.0, 0.0, 2.0, indent)
                            align: AlignItems::Center
                            gap: SPACE_SM
                            cursor: CursorIcon::Pointer
                            on_click: on_toggle_click
                            row {
                                width: Length::Px(28.0)
                                height: Length::Px(22.0)
                                radius: g.get_radius_sm().min(5.0)
                                justify: JustifyContent::Center
                                align: AlignItems::Center
                                background: eva.get_inline_code_bg()
                                border: (1.0, eva.get_border())
                                selectable: false
                                text {
                                    font_size: 11.0
                                    font_family: FontFamily::Monospace
                                    font_weight: FontWeight::Bold
                                    color: eva.get_keyword()
                                    text_align: TextAlign::Center
                                    text_overflow: TextOverflow::Clip
                                    { chk_label }
                                }
                            }
                            column {
                                flex_grow: 1.0
                                flex_shrink: 1.0
                                min_width: Length::Px(0.0)
                                overflow: Overflow::Hidden
                                { para_node }
                            }
                        }
                    }
                } else {
                    let marker_node: NodeId = if ordered {
                        let num_marker: String = format!("{}.", start_num + idx);
                        view! {
                            text {
                                font_size: 14.0
                                font_weight: FontWeight::Bold
                                color: eva.get_keyword()
                                selectable: false
                                { num_marker }
                            }
                        }
                    } else {
                        let bullet: &str = if item.get_depth() % 2 == 1 {
                            "◦"
                        } else {
                            "•"
                        };
                        view! {
                            text {
                                font_size: 14.0
                                font_weight: FontWeight::Bold
                                color: eva.get_keyword()
                                selectable: false
                                { bullet }
                            }
                        }
                    };
                    view! {
                        row {
                            width: Length::Percent(100.0)
                            padding: Edges::new(2.0, 0.0, 2.0, indent)
                            align: AlignItems::Start
                            gap: SPACE_SM
                            { marker_node }
                            column {
                                flex_grow: 1.0
                                flex_shrink: 1.0
                                min_width: Length::Px(0.0)
                                overflow: Overflow::Hidden
                                { para_node }
                            }
                        }
                    }
                }
            })
            .collect();
        view! {
            column {
                width: Length::Percent(100.0)
                gap: SPACE_XS
                ..row_nodes
            }
        }
    }
}

/// Fluent builder helpers for `EuvMarkdownTable`.
impl EuvMarkdownTable {
    /// Controls whether table cell text is selectable.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source table.
    /// - `bool` - Whether selection is enabled.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated table.
    #[must_use]
    pub fn selectable(mut self, enabled: bool) -> Self {
        self.set_selectable(Some(enabled));
        self
    }

    /// Registers a link-click handler for hyperlinks inside table cells.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source table.
    /// - `F` - Callback invoked with clicked URL.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated table.
    #[must_use]
    pub fn on_link<F>(mut self, handler: F) -> Self
    where
        F: Fn(&str) + 'static,
    {
        self.set_on_link(Some(Rc::new(handler)));
        self
    }

    /// Maps a column index to its resolved `(TextAlign, JustifyContent)` pair without heap allocation.
    ///
    /// # Arguments
    ///
    /// - `&[EuvMarkdownAlign]` - Parsed column alignments.
    /// - `usize` - Zero-based column index.
    ///
    /// # Returns
    ///
    /// - `(TextAlign, JustifyContent)` - Text alignment and flex justification pair.
    #[inline(always)]
    fn align_pair(alignments: &[EuvMarkdownAlign], col_idx: usize) -> (TextAlign, JustifyContent) {
        match alignments.get(col_idx).copied().unwrap_or_default() {
            EuvMarkdownAlign::Left => (TextAlign::Left, JustifyContent::Start),
            EuvMarkdownAlign::Center => (TextAlign::Center, JustifyContent::Center),
            EuvMarkdownAlign::Right => (TextAlign::Right, JustifyContent::End),
        }
    }
}

/// Declarative `#[component]` implementation for `EuvMarkdownTable`.
#[component]
/// Inherent implementation of [`EuvMarkdownTable`].
impl EuvMarkdownTable {
    /// Renders a GFM aligned Markdown table with crisp hairline borders, rich cell inlines, horizontal scroll on narrow viewports, and zebra row tint.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Table component reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted table container handle.
    fn render(&self) -> NodeId {
        let tokens: ThemeTokens = self.get_tokens();
        let p: ThemePalette = tokens.get_palette();
        let g: ThemeGeometry = tokens.get_geometry();
        let eva: CodeThemePalette = CodeThemePalette::resolve(tokens.get_mode());
        let Self {
            headers,
            alignments,
            rows,
            selectable,
            on_link,
            ..
        } = self;
        let is_selectable: bool = selectable.unwrap_or(true);
        let col_count: usize = headers.len().max(1);
        let min_table_w: f32 = (col_count as f32) * MD_TABLE_COL_MIN_WIDTH;
        let body_rows: Vec<NodeId> = rows
            .iter()
            .enumerate()
            .map(|(ri, r): (usize, &Vec<String>)| {
                let cells: Vec<NodeId> = (0..col_count)
                    .map(|ci: usize| {
                        let cell_val: &str = r.get(ci).map(String::as_str).unwrap_or("");
                        let (cell_align, cell_justify): (TextAlign, JustifyContent) =
                            Self::align_pair(alignments, ci);
                        if !has_inline_markdown_trigger(cell_val) {
                            return view! {
                                row {
                                    flex_grow: 1.0
                                    flex_shrink: 1.0
                                    min_width: Length::Px(MD_TABLE_COL_MIN_WIDTH)
                                    padding: Edges::symmetric(SPACE_SM, SPACE_SM)
                                    justify: cell_justify
                                    overflow: Overflow::Hidden
                                    text {
                                        width: Length::Percent(100.0)
                                        font_size: 13.0
                                        color: p.get_foreground()
                                        text_align: cell_align
                                        selectable: is_selectable
                                        { cell_val }
                                    }
                                }
                            };
                        }
                        let inlines: Vec<EuvMarkdownInline> = parse_markdown_inlines(cell_val);
                        let is_plain: bool = inlines
                            .iter()
                            .all(|seg: &EuvMarkdownInline| matches!(seg, EuvMarkdownInline::Text(_)));
                        if is_plain {
                            let plain_val: String = if inlines.len() == 1
                                && let EuvMarkdownInline::Text(s) = &inlines[0]
                            {
                                s.clone()
                            } else {
                                inlines
                                    .into_iter()
                                    .map(|seg: EuvMarkdownInline| match seg {
                                        EuvMarkdownInline::Text(s) => s,
                                        _ => String::new(),
                                    })
                                    .collect()
                            };
                            view! {
                                row {
                                    flex_grow: 1.0
                                    flex_shrink: 1.0
                                    min_width: Length::Px(MD_TABLE_COL_MIN_WIDTH)
                                    padding: Edges::symmetric(SPACE_SM, SPACE_SM)
                                    justify: cell_justify
                                    overflow: Overflow::Hidden
                                    text {
                                        width: Length::Percent(100.0)
                                        font_size: 13.0
                                        color: p.get_foreground()
                                        text_align: cell_align
                                        selectable: is_selectable
                                        { plain_val }
                                    }
                                }
                            }
                        } else {
                            let mut para: EuvMarkdownParagraph =
                                EuvMarkdownParagraph::new(inlines, tokens)
                                    .selectable(is_selectable);
                            if let Some(cb) = on_link.clone() {
                                para.set_on_link(Some(cb));
                            }
                            let cell_node: NodeId = para.mount();
                            view! {
                                row {
                                    flex_grow: 1.0
                                    flex_shrink: 1.0
                                    min_width: Length::Px(MD_TABLE_COL_MIN_WIDTH)
                                    padding: Edges::symmetric(SPACE_SM, SPACE_SM)
                                    justify: cell_justify
                                    overflow: Overflow::Hidden
                                    { cell_node }
                                }
                            }
                        }
                    })
                    .collect();
                view! {
                    column {
                        width: Length::Percent(100.0)
                        view {
                            width: Length::Percent(100.0)
                            height: Length::Px(1.0)
                            background: p.get_border()
                        }
                        row {
                            width: Length::Percent(100.0)
                            background: if ri % 2 == 1 { p.get_muted().with_alpha(96) } else { p.get_card() }
                            ..cells
                        }
                    }
                }
            })
            .collect();
        let mut table_scroll_st: Style = Style::default();
        table_scroll_st
            .get_mut_layout()
            .set_flex_direction(FlexDirection::Column)
            .set_align_items(AlignItems::Start);
        view! {
            scroll {
                style: table_scroll_st
                width: Length::Percent(100.0)
                border: (1.0, p.get_border())
                radius: g.get_radius_md()
                column {
                    min_width: Length::Px(min_table_w)
                    width: Length::Percent(100.0)
                    flex_shrink: 0.0
                    selectable: is_selectable
                    row {
                        width: Length::Percent(100.0)
                        background: eva.get_header_bg()
                        for (ci, header) in headers.iter().enumerate() {
                            row {
                                flex_grow: 1.0
                                flex_shrink: 1.0
                                min_width: Length::Px(MD_TABLE_COL_MIN_WIDTH)
                                padding: Edges::symmetric(SPACE_SM, SPACE_SM)
                                overflow: Overflow::Hidden
                                text {
                                    width: Length::Percent(100.0)
                                    font_size: 12.5
                                    font_weight: FontWeight::Bold
                                    color: p.get_foreground()
                                    text_align: Self::align_pair(alignments, ci).0
                                    selectable: is_selectable
                                    { header.as_str() }
                                }
                            }
                        }
                    }
                    ..body_rows
                }
            }
        }
    }
}

/// Fluent builder helpers for `EuvMarkdownDefList`.
impl EuvMarkdownDefList {
    /// Controls whether definition list text is selectable.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source definition list.
    /// - `bool` - Whether selection is enabled.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated definition list.
    #[must_use]
    pub fn selectable(mut self, enabled: bool) -> Self {
        self.set_selectable(Some(enabled));
        self
    }

    /// Registers a link-click handler for hyperlinks inside definitions.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source definition list.
    /// - `F` - Callback invoked with clicked URL.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated definition list.
    #[must_use]
    pub fn on_link<F>(mut self, handler: F) -> Self
    where
        F: Fn(&str) + 'static,
    {
        self.set_on_link(Some(Rc::new(handler)));
        self
    }
}

/// Declarative `#[component]` implementation for `EuvMarkdownDefList`.
#[component]
/// Inherent implementation of [`EuvMarkdownDefList`].
impl EuvMarkdownDefList {
    /// Renders an extended Markdown definition list with bold term headers and indented descriptions.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Definition list component reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted definition list column handle.
    fn render(&self) -> NodeId {
        let tokens: ThemeTokens = self.get_tokens();
        let p: ThemePalette = tokens.get_palette();
        let eva: CodeThemePalette = CodeThemePalette::resolve(tokens.get_mode());
        let Self {
            items,
            selectable,
            on_link,
            ..
        } = self;
        let is_selectable: bool = selectable.unwrap_or(true);
        let entry_nodes: Vec<NodeId> = items
            .iter()
            .map(|entry: &EuvMarkdownDefItem| {
                let term: &str = entry.get_term().as_str();
                let def_nodes: Vec<NodeId> = entry
                    .get_definitions()
                    .iter()
                    .map(|desc: &String| {
                        let desc_str: &str = desc.as_str();
                        if !has_inline_markdown_trigger(desc_str) {
                            return EuvMarkdownParagraph::mount_plain_text(
                                desc_str,
                                p,
                                is_selectable,
                            );
                        }
                        let mut para: EuvMarkdownParagraph =
                            EuvMarkdownParagraph::new(parse_markdown_inlines(desc_str), tokens)
                                .selectable(is_selectable);
                        if let Some(cb) = on_link.clone() {
                            para.set_on_link(Some(cb));
                        }
                        para.mount()
                    })
                    .collect();
                view! {
                    column {
                        width: Length::Percent(100.0)
                        gap: 3.0
                        text {
                            width: Length::Percent(100.0)
                            font_size: 14.0
                            font_weight: FontWeight::Bold
                            color: p.get_foreground()
                            selectable: is_selectable
                            { term }
                        }
                        row {
                            width: Length::Percent(100.0)
                            padding: Edges::new(0.0, 0.0, 0.0, SPACE_SM)
                            gap: SPACE_SM
                            view {
                                width: Length::Px(2.0)
                                flex_shrink: 0.0
                                background: eva.get_border()
                            }
                            column {
                                flex_grow: 1.0
                                flex_shrink: 1.0
                                min_width: Length::Px(0.0)
                                gap: 3.0
                                ..def_nodes
                            }
                        }
                    }
                }
            })
            .collect();
        view! {
            column {
                width: Length::Percent(100.0)
                gap: SPACE_SM
                selectable: is_selectable
                ..entry_nodes
            }
        }
    }
}

/// Fluent builder helpers for `EuvMarkdownFootnotes`.
impl EuvMarkdownFootnotes {
    /// Controls whether footnote text is selectable.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source footnotes section.
    /// - `bool` - Whether selection is enabled.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated footnotes section.
    #[must_use]
    pub fn selectable(mut self, enabled: bool) -> Self {
        self.set_selectable(Some(enabled));
        self
    }

    /// Registers a link-click handler for hyperlinks inside footnotes.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source footnotes section.
    /// - `F` - Callback invoked with clicked URL.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated footnotes section.
    #[must_use]
    pub fn on_link<F>(mut self, handler: F) -> Self
    where
        F: Fn(&str) + 'static,
    {
        self.set_on_link(Some(Rc::new(handler)));
        self
    }
}

/// Declarative `#[component]` implementation for `EuvMarkdownFootnotes`.
#[component]
/// Inherent implementation of [`EuvMarkdownFootnotes`].
impl EuvMarkdownFootnotes {
    /// Renders the bottom Markdown footnotes list separated by a subtle hairline rule.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Footnotes component reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted footnotes section handle.
    fn render(&self) -> NodeId {
        let tokens: ThemeTokens = self.get_tokens();
        let p: ThemePalette = tokens.get_palette();
        let g: ThemeGeometry = tokens.get_geometry();
        let Self {
            footnotes,
            selectable,
            on_link,
            ..
        } = self;
        let is_selectable: bool = selectable.unwrap_or(true);
        let rows: Vec<NodeId> = footnotes
            .iter()
            .map(|entry: &EuvMarkdownFootnote| {
                let id_lbl: String = format!("[{}]", entry.get_id());
                let content_str: &str = entry.get_content().as_str();
                let para_node: NodeId = if !has_inline_markdown_trigger(content_str) {
                    EuvMarkdownParagraph::mount_plain_text(content_str, p, is_selectable)
                } else {
                    let mut para: EuvMarkdownParagraph =
                        EuvMarkdownParagraph::new(parse_markdown_inlines(content_str), tokens)
                            .selectable(is_selectable);
                    if let Some(cb) = on_link.clone() {
                        para.set_on_link(Some(cb));
                    }
                    para.mount()
                };
                view! {
                    row {
                        width: Length::Percent(100.0)
                        align: AlignItems::Start
                        gap: SPACE_SM
                        row {
                            padding: Edges::symmetric(2.0, 6.0)
                            radius: g.get_radius_sm().min(4.0)
                            background: p.get_primary().with_alpha(24)
                            flex_shrink: 0.0
                            text {
                                font_size: 11.0
                                font_family: FontFamily::Monospace
                                font_weight: FontWeight::Bold
                                color: p.get_primary()
                                { id_lbl }
                            }
                        }
                        column {
                            flex_grow: 1.0
                            flex_shrink: 1.0
                            min_width: Length::Px(0.0)
                            overflow: Overflow::Hidden
                            { para_node }
                        }
                    }
                }
            })
            .collect();
        view! {
            column {
                width: Length::Percent(100.0)
                gap: SPACE_XS
                padding: Edges::new(SPACE_SM, 0.0, 0.0, 0.0)
                selectable: is_selectable
                view {
                    width: Length::Percent(100.0)
                    height: Length::Px(1.0)
                    background: p.get_border()
                }
                text {
                    font_size: 12.0
                    font_weight: FontWeight::Bold
                    color: p.get_muted_foreground()
                    { MD_FOOTNOTES_TITLE }
                }
                ..rows
            }
        }
    }
}

/// Declarative `#[component]` implementation for `EuvMarkdownDivider`.
#[component]
/// Inherent implementation of [`EuvMarkdownDivider`].
impl EuvMarkdownDivider {
    /// Renders a subtle thematic break across the full container width.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Divider component reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted divider handle.
    fn render(&self) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        view! {
            view {
                width: Length::Percent(100.0)
                height: Length::Px(1.0)
                margin: Edges::symmetric(SPACE_SM, 0.0)
                background: p.get_border()
            }
        }
    }
}

/// Declarative `#[component]` implementation for `EuvMarkdownImage`.
#[component]
/// Inherent implementation of [`EuvMarkdownImage`].
impl EuvMarkdownImage {
    /// Renders a framed Markdown figure card with semantic alt caption and source badge.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Image figure component reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted image card handle.
    fn render(&self) -> NodeId {
        let tokens: ThemeTokens = self.get_tokens();
        let p: ThemePalette = tokens.get_palette();
        let g: ThemeGeometry = tokens.get_geometry();
        let eva: CodeThemePalette = CodeThemePalette::resolve(tokens.get_mode());
        let alt: &str = self.get_alt().as_str();
        let url: &str = self.get_url().as_str();
        view! {
            column {
                width: Length::Percent(100.0)
                padding: Edges::all(SPACE_MD)
                gap: SPACE_XS
                background: eva.get_header_bg()
                border: (1.0, eva.get_border())
                radius: g.get_radius_md()
                overflow: Overflow::Hidden
                row {
                    width: Length::Percent(100.0)
                    justify: JustifyContent::SpaceBetween
                    align: AlignItems::Center
                    gap: SPACE_SM
                    overflow: Overflow::Hidden
                    text {
                        flex_grow: 1.0
                        flex_shrink: 1.0
                        min_width: Length::Px(0.0)
                        overflow: Overflow::Hidden
                        text_overflow: TextOverflow::Ellipsis
                        font_size: 13.5
                        font_weight: FontWeight::SemiBold
                        color: p.get_foreground()
                        { alt }
                    }
                    EuvBadge {
                        label: MD_IMAGE_BADGE.to_string()
                        tone: EuvTone::Primary
                        tokens: tokens
                    }
                }
                text {
                    width: Length::Percent(100.0)
                    overflow: Overflow::Hidden
                    text_overflow: TextOverflow::Ellipsis
                    font_size: 12.0
                    font_family: FontFamily::Monospace
                    color: eva.get_link()
                    { url }
                }
            }
        }
    }
}

/// Fluent builder helpers for `EuvMarkdownDetails`.
impl EuvMarkdownDetails {
    /// Controls whether content inside `<details>` is selectable.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source disclosure block.
    /// - `bool` - Whether selection is enabled.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated disclosure block.
    #[must_use]
    pub fn selectable(mut self, enabled: bool) -> Self {
        self.set_selectable(Some(enabled));
        self
    }

    /// Registers a link-click handler for hyperlinks inside expanded details.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source disclosure block.
    /// - `F` - Callback invoked with clicked URL.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated disclosure block.
    #[must_use]
    pub fn on_link<F>(mut self, handler: F) -> Self
    where
        F: Fn(&str) + 'static,
    {
        self.set_on_link(Some(Rc::new(handler)));
        self
    }

    /// Registers a copy handler for code blocks inside expanded details.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source disclosure block.
    /// - `F` - Callback invoked with copied code text.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated disclosure block.
    #[must_use]
    pub fn on_copy<F>(mut self, handler: F) -> Self
    where
        F: Fn(&str) + 'static,
    {
        self.set_on_copy(Some(Rc::new(handler)));
        self
    }

    /// Registers a task-toggle handler for checklists inside expanded details.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source disclosure block.
    /// - `F` - Callback invoked with `(index, checked)`.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated disclosure block.
    #[must_use]
    pub fn on_toggle_task<F>(mut self, handler: F) -> Self
    where
        F: Fn(usize, bool) + 'static,
    {
        self.set_on_toggle_task(Some(Rc::new(handler)));
        self
    }
}

/// Declarative `#[component]` implementation for `EuvMarkdownDetails`.
#[component]
/// Inherent implementation of [`EuvMarkdownDetails`].
impl EuvMarkdownDetails {
    /// Renders an interactive collapsible `<details><summary>` disclosure block supporting rich nested Markdown.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Disclosure component reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted disclosure container handle.
    fn render(&self) -> NodeId {
        let tokens: ThemeTokens = self.get_tokens();
        let open: Signal<bool> = self.get_open();
        let Self {
            summary,
            body,
            selectable,
            on_link,
            on_copy,
            on_toggle_task,
            ..
        } = self;
        let is_selectable: bool = selectable.unwrap_or(true);
        let nested_src: String = body.join("\n");
        let mut inner_md: EuvMarkdown =
            EuvMarkdown::new(nested_src, tokens).selectable(is_selectable);
        if let Some(cb) = on_link.clone() {
            inner_md.set_on_link(Some(cb));
        }
        if let Some(cb) = on_copy.clone() {
            inner_md.set_on_copy(Some(cb));
        }
        if let Some(cb) = on_toggle_task.clone() {
            inner_md.set_on_toggle_task(Some(cb));
        }
        let body_node: NodeId = inner_md.mount();
        let summary_title: String = summary.clone();
        view! {
            EuvCollapsible {
                open: open
                title: summary_title
                content: body_node
                tokens: tokens
            }
        }
    }
}

/// Fluent callback and selection builders for `EuvMarkdown`.
impl EuvMarkdown {
    /// Controls whether text inside the rendered Markdown document is selectable.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source Markdown renderer.
    /// - `bool` - `true` to enable native text selection across blocks, `false` to disable.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated Markdown renderer.
    #[must_use]
    pub fn selectable(mut self, enabled: bool) -> Self {
        self.set_selectable(Some(enabled));
        self
    }

    /// Attaches a link-click handler for hyperlinks inside the Markdown document.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source Markdown renderer.
    /// - `F` - Callback invoked with clicked link URL.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated Markdown renderer.
    #[must_use]
    pub fn on_link<F>(mut self, handler: F) -> Self
    where
        F: Fn(&str) + 'static,
    {
        self.set_on_link(Some(Rc::new(handler)));
        self
    }

    /// Attaches a copy handler invoked when any fenced `EuvCodeBlock` or `EuvMermaid` diagram is copied.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source Markdown renderer.
    /// - `F` - Callback invoked with copied code text.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated Markdown renderer.
    #[must_use]
    pub fn on_copy<F>(mut self, handler: F) -> Self
    where
        F: Fn(&str) + 'static,
    {
        self.set_on_copy(Some(Rc::new(handler)));
        self
    }

    /// Attaches a checkbox toggle handler for GFM task checklists inside the document.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source Markdown renderer.
    /// - `F` - Callback invoked with `(item_index, checked)`.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated Markdown renderer.
    #[must_use]
    pub fn on_toggle_task<F>(mut self, handler: F) -> Self
    where
        F: Fn(usize, bool) + 'static,
    {
        self.set_on_toggle_task(Some(Rc::new(handler)));
        self
    }
}

/// Declarative `#[component]` implementation for `EuvMarkdown`.
#[component]
/// Inherent implementation of [`EuvMarkdown`].
impl EuvMarkdown {
    /// Parses and mounts a full GitHub-Flavored & Extended Markdown document natively using `view!`, including native vector Mermaid diagrams and Unicode LaTeX math.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Markdown document component reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted Markdown document column handle.
    fn render(&self) -> NodeId {
        let tokens: ThemeTokens = self.get_tokens();
        let Self {
            source,
            selectable,
            on_link,
            on_copy,
            on_toggle_task,
            ..
        } = self;
        let is_selectable: bool = selectable.unwrap_or(true);
        let blocks: Vec<EuvMarkdownBlock> = parse_markdown_blocks(source);
        let children: Vec<NodeId> = blocks
            .into_iter()
            .map(|blk: EuvMarkdownBlock| match blk {
                EuvMarkdownBlock::Heading { level, text } => view! {
                    EuvMarkdownHeading {
                        level: level
                        text: text
                        tokens: tokens
                        selectable: is_selectable
                    }
                },
                EuvMarkdownBlock::Paragraph(inlines) => {
                    let mut para: EuvMarkdownParagraph =
                        EuvMarkdownParagraph::new(inlines, tokens).selectable(is_selectable);
                    if let Some(cb) = on_link.clone() {
                        para.set_on_link(Some(cb));
                    }
                    para.mount()
                }
                EuvMarkdownBlock::CodeBlock { language, code } => {
                    let mut cb_block: EuvCodeBlock = EuvCodeBlock::new(code, tokens)
                        .language(language)
                        .line_numbers(true)
                        .selectable(is_selectable);
                    if let Some(cb) = on_copy.clone() {
                        cb_block.set_on_copy(Some(cb));
                    }
                    cb_block.mount()
                }
                EuvMarkdownBlock::Mermaid { source: diag_src } => {
                    let mut diagram: EuvMermaid = EuvMermaid::new(diag_src, tokens);
                    if let Some(cb) = on_copy.clone() {
                        diagram.set_on_copy(Some(cb));
                    }
                    diagram.mount()
                }
                EuvMarkdownBlock::MathBlock(expr) => view! {
                    EuvMarkdownMath {
                        expression: expr
                        tokens: tokens
                        selectable: is_selectable
                    }
                },
                EuvMarkdownBlock::Quote { kind, lines, .. } => {
                    let mut quote: EuvMarkdownQuote =
                        EuvMarkdownQuote::new(kind, lines, tokens).selectable(is_selectable);
                    if let Some(cb) = on_link.clone() {
                        quote.set_on_link(Some(cb));
                    }
                    quote.mount()
                }
                EuvMarkdownBlock::List {
                    ordered,
                    start,
                    items,
                } => {
                    let mut list: EuvMarkdownList = EuvMarkdownList::new(ordered, items, tokens)
                        .start(start)
                        .selectable(is_selectable);
                    if let Some(cb) = on_link.clone() {
                        list.set_on_link(Some(cb));
                    }
                    if let Some(cb) = on_toggle_task.clone() {
                        list.set_on_toggle_task(Some(cb));
                    }
                    list.mount()
                }
                EuvMarkdownBlock::Table {
                    headers,
                    alignments,
                    rows,
                } => {
                    let mut tbl: EuvMarkdownTable =
                        EuvMarkdownTable::new(headers, alignments, rows, tokens)
                            .selectable(is_selectable);
                    if let Some(cb) = on_link.clone() {
                        tbl.set_on_link(Some(cb));
                    }
                    tbl.mount()
                }
                EuvMarkdownBlock::DefinitionList(items) => {
                    let mut def_list: EuvMarkdownDefList =
                        EuvMarkdownDefList::new(items, tokens).selectable(is_selectable);
                    if let Some(cb) = on_link.clone() {
                        def_list.set_on_link(Some(cb));
                    }
                    def_list.mount()
                }
                EuvMarkdownBlock::Footnotes(entries) => {
                    let mut fn_sec: EuvMarkdownFootnotes =
                        EuvMarkdownFootnotes::new(entries, tokens).selectable(is_selectable);
                    if let Some(cb) = on_link.clone() {
                        fn_sec.set_on_link(Some(cb));
                    }
                    fn_sec.mount()
                }
                EuvMarkdownBlock::Divider => view! {
                    EuvMarkdownDivider {
                        tokens: tokens
                    }
                },
                EuvMarkdownBlock::Image { alt, url } => view! {
                    EuvMarkdownImage {
                        alt: alt
                        url: url
                        tokens: tokens
                    }
                },
                EuvMarkdownBlock::Details { summary, body } => {
                    let open_sig: Signal<bool> = Signal::new(false);
                    let mut details: EuvMarkdownDetails =
                        EuvMarkdownDetails::new(summary, body, open_sig, tokens)
                            .selectable(is_selectable);
                    if let Some(cb) = on_link.clone() {
                        details.set_on_link(Some(cb));
                    }
                    if let Some(cb) = on_copy.clone() {
                        details.set_on_copy(Some(cb));
                    }
                    if let Some(cb) = on_toggle_task.clone() {
                        details.set_on_toggle_task(Some(cb));
                    }
                    details.mount()
                }
            })
            .collect();
        view! {
            column {
                width: Length::Percent(100.0)
                gap: SPACE_MD
                selectable: is_selectable
                ..children
            }
        }
    }
}
