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
    /// Mounts the fullscreen modal card for `EuvCodeBlock` when the user opens the fullscreen overlay.
    ///
    /// # Arguments
    ///
    /// - `&Context` - Active build context.
    /// - `ThemeTokens` - Base theme tokens.
    /// - `&Rc<str>` - Shared raw code string.
    /// - `&Rc<str>` - Uppercase language badge label.
    /// - `Option<&str>` - Optional display filename.
    /// - `(CodeLang, bool, usize, Signal<bool>, NodeId)` - Modal metadata `(code_lang, show_nums, line_count, fs_open, inline_copy_txt)`.
    /// - `(&[usize], &Rc<Cell<bool>>, Option<&MarkdownStrCallback>)` - Highlight lines, shared copied flag, and optional copy callback.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted modal card node handle.
    fn mount_fullscreen_modal_card(
        ctx: &Context,
        tokens: ThemeTokens,
        modal_code_rc: &Rc<str>,
        modal_lang: &Rc<str>,
        modal_filename: Option<&str>,
        meta: (CodeLang, bool, usize, Signal<bool>, NodeId),
        extra: (&[usize], &Rc<Cell<bool>>, Option<&MarkdownStrCallback>),
    ) -> NodeId {
        let (code_lang, show_nums, line_count, fs_open, inline_copy_txt): (
            CodeLang,
            bool,
            usize,
            Signal<bool>,
            NodeId,
        ) = meta;
        let (modal_hl, copied_cell, modal_on_copy): (
            &[usize],
            &Rc<Cell<bool>>,
            Option<&MarkdownStrCallback>,
        ) = extra;
        let live_tok: ThemeTokens = live_tokens_untracked(tokens);
        let live_p: ThemePalette = live_tok.get_palette();
        let live_g: ThemeGeometry = live_tok.get_geometry();
        let live_eva: CodeThemePalette = CodeThemePalette::resolve(live_tok.get_mode());
        let (modal_nat_w, _, modal_line_nodes): (f32, usize, Vec<NodeId>) = build_code_line_nodes(
            modal_code_rc,
            code_lang,
            live_eva,
            show_nums,
            modal_hl,
            CODE_MODAL_FONT_SIZE,
            true,
        );
        let m_copy_str: Rc<str> = Rc::clone(modal_code_rc);
        let m_copy_cb: Option<MarkdownStrCallback> = modal_on_copy.cloned();
        let m_ctx: Context = ctx.clone();
        let m_copied: Rc<Cell<bool>> = Rc::clone(copied_cell);
        let init_copy_label: &'static str = if copied_cell.get() {
            CODE_BTN_COPIED
        } else {
            CODE_BTN_COPY
        };
        let modal_copy_txt_cell: Rc<Cell<NodeId>> = Rc::new(Cell::new(NodeId::default()));
        let modal_copy_txt_cb: Rc<Cell<NodeId>> = Rc::clone(&modal_copy_txt_cell);
        let on_modal_copy = move |_: &UiEvent| {
            let _: bool = m_ctx.copy_to_clipboard(&m_copy_str);
            m_copied.set(true);
            let m_txt: NodeId = modal_copy_txt_cb.get();
            m_ctx.get_scene().with_write(
                |tree: &mut SceneTree| {
                    tree.set_text(inline_copy_txt, CODE_BTN_COPIED.to_string());
                    tree.set_text(m_txt, CODE_BTN_COPIED.to_string());
                },
                (),
            );
            if let Some(cb) = m_copy_cb.as_ref() {
                cb(&m_copy_str);
            }
        };
        let on_close_btn = move |_: &UiEvent| {
            let _: bool = fs_open.set_if_changed(false);
        };
        let m_title: &str = modal_filename.unwrap_or(live_eva.get_theme_name());
        let m_lang: &str = modal_lang;
        let lines_badge: String = format!("{line_count} L");
        let mut modal_scroll_st: Style = Style::default();
        modal_scroll_st
            .get_mut_layout()
            .set_flex_direction(FlexDirection::Column)
            .set_align_items(AlignItems::Start);
        let mut modal_copy_txt: NodeId = NodeId::default();
        let card: NodeId = view! {
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
                                node_ref: modal_copy_txt
                                font_size: 11.5
                                font_weight: FontWeight::SemiBold
                                color: live_eva.get_function()
                                text_overflow: TextOverflow::Clip
                                { init_copy_label }
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
        modal_copy_txt_cell.set(modal_copy_txt);
        card
    }

    /// Mounts the complete `EuvCodeBlock` subtree directly into an already-borrowed `SceneTree` with zero eager signals or effects.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Code block reference.
    /// - `&mut SceneTree` - Mutable scene tree.
    /// - `&Context` - Active build context captured by interactive click handlers.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted code block container handle.
    pub(crate) fn mount_in_tree(&self, tree: &mut SceneTree, ctx: &Context) -> NodeId {
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
        let code_rc: Rc<str> = Rc::from(code.as_str());
        let show_nums: bool = *show_line_numbers || code.lines().nth(1).is_some();
        let (natural_w, line_count, line_nodes): (f32, usize, Vec<NodeId>) =
            build_code_line_nodes_in_tree(
                tree,
                code,
                code_lang,
                eva,
                highlight_lines,
                (show_nums, CODE_FONT_SIZE, is_selectable),
            );
        let copied_cell: Rc<Cell<bool>> = Rc::new(Cell::new(false));
        let fs_open_slot: Rc<Cell<Option<Signal<bool>>>> = Rc::new(Cell::new(None));
        let radius_lg: CornerRadius = CornerRadius::all(g.get_radius_lg().max(0.0));
        let radius_sm: CornerRadius = CornerRadius::all(g.get_radius_sm().max(0.0));
        let eva_bg: Color = eva.get_background();
        let eva_border: Border = Border::new(1.0, eva.get_border());
        let mut root_st: Style = Style::default();
        root_st
            .get_mut_layout()
            .set_flex_direction(FlexDirection::Column)
            .set_width(Length::Percent(100.0))
            .set_overflow(Overflow::Hidden);
        root_st
            .get_mut_paint()
            .set_background(Some(eva_bg))
            .set_border(eva_border)
            .set_radius(radius_lg);
        let root: NodeId = tree.create_view(root_st);
        let mut header_st: Style = Style::default().selectable(false);
        header_st
            .get_mut_layout()
            .set_flex_direction(FlexDirection::Row)
            .set_width(Length::Percent(100.0))
            .set_padding(Edges::symmetric(SPACE_SM, SPACE_MD))
            .set_justify_content(JustifyContent::SpaceBetween)
            .set_align_items(AlignItems::Center)
            .set_row_gap(SPACE_SM)
            .set_column_gap(SPACE_SM)
            .set_overflow(Overflow::Hidden);
        header_st
            .get_mut_paint()
            .set_background(Some(eva.get_header_bg()));
        let header_row: NodeId = tree.create_view(header_st);
        let mut left_st: Style = Style::default();
        left_st
            .get_mut_layout()
            .set_flex_direction(FlexDirection::Row)
            .set_flex_grow(1.0)
            .set_flex_shrink(1.0)
            .set_min_width(Length::Px(0.0))
            .set_align_items(AlignItems::Center)
            .set_row_gap(SPACE_SM)
            .set_column_gap(SPACE_SM)
            .set_overflow(Overflow::Hidden);
        let left_row: NodeId = tree.create_view(left_st);
        let mut dots_st: Style = Style::default();
        dots_st
            .get_mut_layout()
            .set_flex_direction(FlexDirection::Row)
            .set_flex_shrink(0.0)
            .set_row_gap(5.0)
            .set_column_gap(5.0)
            .set_align_items(AlignItems::Center);
        let dots_row: NodeId = tree.create_view(dots_st);
        let mut dot_st: Style = Style::default();
        dot_st
            .get_mut_layout()
            .set_width(Length::Px(9.0))
            .set_height(Length::Px(9.0));
        dot_st.get_mut_paint().set_radius(CornerRadius::all(99.0));
        dot_st
            .get_mut_paint()
            .set_background(Some(Color::hex(CODE_DOT_RED)));
        let dot_r: NodeId = tree.create_view(dot_st);
        dot_st
            .get_mut_paint()
            .set_background(Some(Color::hex(CODE_DOT_YELLOW)));
        let dot_y: NodeId = tree.create_view(dot_st);
        dot_st
            .get_mut_paint()
            .set_background(Some(Color::hex(CODE_DOT_GREEN)));
        let dot_g: NodeId = tree.create_view(dot_st);
        tree.append_children(dots_row, &[dot_r, dot_y, dot_g]);
        let mut title_st: Style = Style::default();
        title_st
            .get_mut_layout()
            .set_flex_shrink(1.0)
            .set_min_width(Length::Px(0.0))
            .set_overflow(Overflow::Hidden);
        title_st
            .get_mut_text()
            .set_overflow(TextOverflow::Ellipsis)
            .set_font_size(12.0)
            .set_weight(FontWeight::SemiBold)
            .set_color(eva.get_foreground());
        let title_txt: NodeId = tree.create_text(header_title_str.to_string(), title_st);
        tree.append_children(left_row, &[dots_row, title_txt]);
        let mut right_st: Style = Style::default();
        right_st
            .get_mut_layout()
            .set_flex_direction(FlexDirection::Row)
            .set_flex_shrink(0.0)
            .set_align_items(AlignItems::Center)
            .set_row_gap(SPACE_XS)
            .set_column_gap(SPACE_XS);
        let right_row: NodeId = tree.create_view(right_st);
        let mut pill_st: Style = Style::default();
        pill_st
            .get_mut_layout()
            .set_flex_direction(FlexDirection::Row)
            .set_padding(Edges::symmetric(3.0, 8.0))
            .set_justify_content(JustifyContent::Center)
            .set_align_items(AlignItems::Center);
        pill_st
            .get_mut_paint()
            .set_radius(radius_sm)
            .set_background(Some(eva_bg))
            .set_border(eva_border);
        let lang_badge: NodeId = tree.create_view(pill_st);
        let mut lang_txt_st: Style = Style::default();
        lang_txt_st
            .get_mut_text()
            .set_font_size(10.5)
            .set_weight(FontWeight::Bold)
            .set_color(eva.get_keyword())
            .set_align(TextAlign::Center)
            .set_overflow(TextOverflow::Clip);
        let lang_txt: NodeId = tree.create_text(display_lang_rc.to_string(), lang_txt_st);
        tree.append_child(lang_badge, lang_txt);
        let mut fs_btn_st: Style = pill_st;
        fs_btn_st
            .get_mut_layout()
            .set_row_gap(4.0)
            .set_column_gap(4.0);
        fs_btn_st.get_mut_paint().set_cursor(CursorIcon::Pointer);
        let fs_btn: NodeId = tree.create_view(fs_btn_st);
        let mut icon_st: Style = Style::default();
        icon_st
            .get_mut_layout()
            .set_width(Length::Px(12.0))
            .set_height(Length::Px(12.0));
        let paint_fs_icon: CanvasDrawCallback =
            Rc::new(RefCell::new(move |bounds: Rect, dl: &mut DisplayList| {
                let live_eva: CodeThemePalette =
                    CodeThemePalette::resolve(live_tokens_untracked(tokens).get_mode());
                draw_code_fullscreen_icon(bounds, dl, live_eva.get_function());
            }));
        let fs_icon: NodeId = tree.create_canvas(icon_st, paint_fs_icon);
        let mut action_txt_st: Style = Style::default();
        action_txt_st
            .get_mut_text()
            .set_font_size(11.0)
            .set_weight(FontWeight::SemiBold)
            .set_color(eva.get_function())
            .set_align(TextAlign::Center)
            .set_overflow(TextOverflow::Clip);
        let fs_txt: NodeId = tree.create_text(CODE_BTN_FULLSCREEN.to_string(), action_txt_st);
        tree.append_children(fs_btn, &[fs_icon, fs_txt]);
        let mut copy_btn_st: Style = pill_st;
        copy_btn_st
            .get_mut_layout()
            .set_padding(Edges::symmetric(3.0, 10.0));
        copy_btn_st.get_mut_paint().set_cursor(CursorIcon::Pointer);
        let copy_btn: NodeId = tree.create_view(copy_btn_st);
        let copy_txt_node: NodeId = tree.create_text(CODE_BTN_COPY.to_string(), action_txt_st);
        tree.append_child(copy_btn, copy_txt_node);
        tree.append_children(right_row, &[lang_badge, fs_btn, copy_btn]);
        tree.append_children(header_row, &[left_row, right_row]);
        let mut div_st: Style = Style::default();
        div_st
            .get_mut_layout()
            .set_width(Length::Percent(100.0))
            .set_height(Length::Px(1.0));
        div_st
            .get_mut_paint()
            .set_background(Some(eva.get_border()));
        let div_node: NodeId = tree.create_view(div_st);
        let mut scroll_st: Style = Style::default();
        scroll_st
            .get_mut_layout()
            .set_flex_direction(FlexDirection::Column)
            .set_align_items(AlignItems::Start)
            .set_width(Length::Percent(100.0))
            .set_max_height(Length::Px(inline_max_h))
            .set_padding(Edges::symmetric(SPACE_SM, SPACE_XS));
        let scroll_node: NodeId = tree.create_scroll_view(scroll_st);
        let mut code_col_st: Style = Style::default().selectable(is_selectable);
        code_col_st
            .get_mut_layout()
            .set_flex_direction(FlexDirection::Column)
            .set_min_width(Length::Px(natural_w))
            .set_width(Length::Percent(100.0))
            .set_flex_shrink(0.0)
            .set_row_gap(1.0)
            .set_column_gap(1.0);
        let code_col: NodeId = tree.create_view(code_col_st);
        if !line_nodes.is_empty() {
            tree.append_children(code_col, &line_nodes);
        }
        tree.append_child(scroll_node, code_col);
        let mut portal_st: Style = Style::default();
        portal_st
            .get_mut_layout()
            .set_display(Display::None)
            .set_position(Position::Fixed)
            .set_top(Some(0.0))
            .set_right(Some(0.0))
            .set_bottom(Some(0.0))
            .set_left(Some(0.0))
            .set_width(Length::Percent(100.0))
            .set_height(Length::Percent(100.0))
            .set_padding(Edges::all(CODE_MODAL_PADDING))
            .set_justify_content(JustifyContent::Center)
            .set_align_items(AlignItems::Center)
            .set_z_index(Z_INDEX_MODAL);
        portal_st
            .get_mut_paint()
            .set_background(Some(tokens.get_palette().get_overlay()))
            .set_stop_scroll_propagation(true);
        let fs_portal: NodeId = tree.create_portal(portal_st);
        let ctx_copy: Context = ctx.clone();
        let on_copy_inline: Option<MarkdownStrCallback> = on_copy.clone();
        let code_for_copy: Rc<str> = Rc::clone(&code_rc);
        let copied_for_copy: Rc<Cell<bool>> = Rc::clone(&copied_cell);
        let on_copy_handler: UiEventHandler = Rc::new(RefCell::new(move |_: &UiEvent| {
            let _: bool = ctx_copy.copy_to_clipboard(&code_for_copy);
            copied_for_copy.set(true);
            ctx_copy.get_scene().with_write(
                |t: &mut SceneTree| {
                    t.set_text(copy_txt_node, CODE_BTN_COPIED.to_string());
                },
                (),
            );
            if let Some(cb) = on_copy_inline.as_ref() {
                cb(&code_for_copy);
            }
        }));
        if let Some(node) = tree.get_node_mut(copy_btn) {
            node.get_mut_handlers().set_on_click(Some(on_copy_handler));
            node.get_mut_interaction().set_focusable(true);
        }
        let fs_open_scrim: Rc<Cell<Option<Signal<bool>>>> = Rc::clone(&fs_open_slot);
        let on_scrim_handler: UiEventHandler = Rc::new(RefCell::new(move |_: &UiEvent| {
            if let Some(sig) = fs_open_scrim.get() {
                let _: bool = sig.set_if_changed(false);
            }
        }));
        if let Some(node) = tree.get_node_mut(fs_portal) {
            node.get_mut_handlers().set_on_click(Some(on_scrim_handler));
            node.get_mut_interaction().set_focusable(true);
        }
        let modal_hl: Vec<usize> = if highlight_lines.is_empty() {
            Vec::new()
        } else {
            highlight_lines.clone()
        };
        let modal_filename: Option<Rc<str>> = filename_rc;
        let modal_lang: Rc<str> = display_lang_rc;
        let modal_code_rc: Rc<str> = code_rc;
        let modal_on_copy: Option<MarkdownStrCallback> = on_copy.clone();
        let ctx_fs: Context = ctx.clone();
        let on_open_fs_handler: UiEventHandler = Rc::new(RefCell::new(move |_: &UiEvent| {
            if let Some(sig) = fs_open_slot.get() {
                let _: bool = sig.set_if_changed(true);
                return;
            }
            let fs_open: Signal<bool> = Signal::new(true);
            fs_open_slot.set(Some(fs_open));
            let handle_ov: SceneHandle = ctx_fs.get_scene().clone();
            let prev_open: Rc<Cell<bool>> = Rc::new(Cell::new(false));
            let ctx_build: Context = ctx_fs.clone();
            let code_build: Rc<str> = Rc::clone(&modal_code_rc);
            let lang_build: Rc<str> = Rc::clone(&modal_lang);
            let file_build: Option<Rc<str>> = modal_filename.clone();
            let hl_build: Vec<usize> = modal_hl.clone();
            let cb_build: Option<MarkdownStrCallback> = modal_on_copy.clone();
            let copied_build: Rc<Cell<bool>> = Rc::clone(&copied_cell);
            let (ov_scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
                let _: Effect = EuvOverlayStack::bind_signal(
                    fs_open,
                    CODE_FULLSCREEN_OVERLAY_NAME,
                    Z_INDEX_MODAL,
                );
                Effect::new(move || {
                    let is_open: bool = fs_open.get();
                    if is_open == prev_open.replace(is_open) {
                        return;
                    }
                    if is_open {
                        let modal_card: NodeId = with_context(ctx_build.clone(), || {
                            Self::mount_fullscreen_modal_card(
                                &ctx_build,
                                tokens,
                                &code_build,
                                &lang_build,
                                file_build.as_deref(),
                                (code_lang, show_nums, line_count, fs_open, copy_txt_node),
                                (&hl_build, &copied_build, cb_build.as_ref()),
                            )
                        });
                        handle_ov.with_write(
                            |t: &mut SceneTree| {
                                t.replace_children(fs_portal, vec![modal_card]);
                                if let Some(n) = t.get_node_mut(fs_portal) {
                                    n.get_mut_style()
                                        .get_mut_paint()
                                        .set_pointer_events(PointerEvents::Auto);
                                }
                                t.start_transition(
                                    NodeTransition::fade(fs_portal, 0.0, 1.0, 0.18, false)
                                        .with_easing(Easing::EaseOutCubic),
                                );
                            },
                            (),
                        );
                    } else {
                        handle_ov.with_write(
                            |t: &mut SceneTree| {
                                t.replace_children(fs_portal, Vec::new());
                                if let Some(n) = t.get_node_mut(fs_portal) {
                                    n.get_mut_style()
                                        .get_mut_paint()
                                        .set_pointer_events(PointerEvents::None);
                                }
                                t.start_transition(
                                    NodeTransition::fade(fs_portal, 1.0, 0.0, 0.16, true)
                                        .with_easing(Easing::EaseOutCubic),
                                );
                            },
                            (),
                        );
                    }
                })
            });
            ctx_fs.get_scene().with_write(
                |t: &mut SceneTree| {
                    t.attach_scope(root, ov_scope);
                },
                (),
            );
        }));
        if let Some(node) = tree.get_node_mut(fs_btn) {
            node.get_mut_handlers()
                .set_on_click(Some(on_open_fs_handler));
            node.get_mut_interaction().set_focusable(true);
        }
        tree.append_children(root, &[header_row, div_node, scroll_node, fs_portal]);
        root
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
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| self.mount_in_tree(tree, &ctx),
            NodeId::default(),
        )
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
    /// Mounts a Markdown heading (`H1`–`H6`) directly into an already-borrowed `SceneTree`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Heading component reference.
    /// - `&mut SceneTree` - Mutable scene tree.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted heading node handle.
    pub(crate) fn mount_in_tree(&self, tree: &mut SceneTree) -> NodeId {
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
        let heading_str: String = if text.contains(':') {
            expand_emoji_shortcodes(text.as_str())
        } else {
            text.clone()
        };
        let mut col_st: Style = Style::default();
        col_st
            .get_mut_layout()
            .set_flex_direction(FlexDirection::Column)
            .set_width(Length::Percent(100.0))
            .set_row_gap(SPACE_XS)
            .set_column_gap(SPACE_XS)
            .set_padding(Edges::new(
                if lvl <= 2 { SPACE_SM } else { SPACE_XS },
                0.0,
                0.0,
                0.0,
            ));
        let root: NodeId = tree.create_view(col_st);
        let mut row_st: Style = Style::default();
        row_st
            .get_mut_layout()
            .set_flex_direction(FlexDirection::Row)
            .set_width(Length::Percent(100.0))
            .set_align_items(AlignItems::Center)
            .set_row_gap(SPACE_SM)
            .set_column_gap(SPACE_SM);
        let row_id: NodeId = tree.create_view(row_st);
        let mut txt_st: Style = Style::default().selectable(is_selectable);
        txt_st
            .get_mut_layout()
            .set_flex_grow(1.0)
            .set_flex_shrink(1.0)
            .set_min_width(Length::Px(0.0));
        txt_st
            .get_mut_text()
            .set_font_size(size)
            .set_weight(FontWeight::Bold)
            .set_color(if lvl == 6 {
                p.get_muted_foreground()
            } else {
                p.get_foreground()
            });
        let txt_id: NodeId = tree.create_text(heading_str, txt_st);
        if lvl == 1 {
            let mut bar_st: Style = Style::default();
            bar_st
                .get_mut_layout()
                .set_width(Length::Px(4.0))
                .set_height(Length::Px(22.0));
            bar_st
                .get_mut_paint()
                .set_radius(CornerRadius::all(99.0))
                .set_background(Some(eva.get_keyword()));
            let bar_id: NodeId = tree.create_view(bar_st);
            tree.append_children(row_id, &[bar_id, txt_id]);
        } else {
            tree.append_child(row_id, txt_id);
        }
        if show_rule {
            let mut rule_st: Style = Style::default();
            rule_st
                .get_mut_layout()
                .set_width(Length::Percent(100.0))
                .set_height(Length::Px(1.0));
            rule_st.get_mut_paint().set_background(Some(p.get_border()));
            let rule_id: NodeId = tree.create_view(rule_st);
            tree.append_children(root, &[row_id, rule_id]);
        } else {
            tree.append_child(root, row_id);
        }
        root
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
        let ctx: Context = current_context();
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| self.mount_in_tree(tree),
            NodeId::default(),
        )
    }
}

/// Direct tree mounting helper for `EuvMarkdownInlineCode`.
impl EuvMarkdownInlineCode {
    /// Mounts an inline monospace code pill directly into an already-borrowed `SceneTree`.
    ///
    /// # Arguments
    ///
    /// - `&mut SceneTree` - Mutable scene tree.
    /// - `&str` - Inline code snippet slice.
    /// - `ThemeTokens` - Active theme tokens.
    /// - `CodeThemePalette` - Resolved Eva palette.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted inline code pill handle.
    pub(crate) fn mount_in_tree(
        tree: &mut SceneTree,
        snippet: &str,
        tokens: ThemeTokens,
        eva: CodeThemePalette,
    ) -> NodeId {
        let g: ThemeGeometry = tokens.get_geometry();
        let mut row_st: Style = Style::default();
        row_st
            .get_mut_layout()
            .set_flex_direction(FlexDirection::Row)
            .set_max_width(Length::Percent(100.0))
            .set_padding(Edges::symmetric(2.0, 6.0))
            .set_justify_content(JustifyContent::Center)
            .set_align_items(AlignItems::Center)
            .set_overflow(Overflow::Hidden);
        row_st
            .get_mut_paint()
            .set_radius(CornerRadius::all(g.get_radius_sm().clamp(0.0, 6.0)))
            .set_background(Some(eva.get_inline_code_bg()))
            .set_border(Border::new(1.0, eva.get_border()));
        let mut txt_st: Style = Style::default();
        txt_st
            .get_mut_layout()
            .set_max_width(Length::Percent(100.0))
            .set_overflow(Overflow::Hidden);
        txt_st
            .get_mut_text()
            .set_overflow(TextOverflow::Clip)
            .set_align(TextAlign::Center)
            .set_font_size(12.0)
            .set_family(FontFamily::Monospace)
            .set_weight(FontWeight::Normal)
            .set_color(eva.get_inline_code_fg());
        let root: NodeId = tree.create_view(row_st);
        let txt_id: NodeId = tree.create_text(snippet.to_string(), txt_st);
        tree.append_child(root, txt_id);
        root
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
        let ctx: Context = current_context();
        let tokens: ThemeTokens = self.get_tokens();
        let eva: CodeThemePalette = CodeThemePalette::resolve(tokens.get_mode());
        let snippet: &str = self.get_code().as_str();
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| Self::mount_in_tree(tree, snippet, tokens, eva),
            NodeId::default(),
        )
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
                        flex_shrink: 0.0
                        padding: Edges::symmetric(2.0, 6.0)
                        radius: g.get_radius_sm()
                        background: eva.get_background()
                        border: (1.0, eva.get_border())
                        text {
                            font_size: 9.5
                            font_weight: FontWeight::Bold
                            color: eva.get_keyword()
                            text_overflow: TextOverflow::Clip
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

    /// Mounts a plain body text paragraph node directly into an already-borrowed `SceneTree`.
    ///
    /// # Arguments
    ///
    /// - `&mut SceneTree` - Mutable scene tree.
    /// - `&str` - Plain body text slice.
    /// - `ThemePalette` - Resolved active theme palette.
    /// - `bool` - Whether text selection is enabled.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted body text node handle.
    #[inline(always)]
    pub(crate) fn mount_plain_in_tree(
        tree: &mut SceneTree,
        text: &str,
        p: ThemePalette,
        is_selectable: bool,
    ) -> NodeId {
        let mut st: Style = Style::default().selectable(is_selectable);
        st.get_mut_layout().set_width(Length::Percent(100.0));
        st.get_mut_text()
            .set_font_size(14.0)
            .set_color(p.get_foreground());
        tree.create_text(text.to_string(), st)
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
        let ctx: Context = current_context();
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| Self::mount_plain_in_tree(tree, text, p, is_selectable),
            NodeId::default(),
        )
    }

    /// Mounts a single parsed `EuvMarkdownInline` segment directly into an already-borrowed `SceneTree`.
    ///
    /// # Arguments
    ///
    /// - `&mut SceneTree` - Mutable scene tree.
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
    fn render_inline_in_tree(
        tree: &mut SceneTree,
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
            EuvMarkdownInline::Text(t) => {
                let mut st: Style = Style::default().selectable(is_selectable);
                st.get_mut_layout().set_max_width(Length::Percent(100.0));
                st.get_mut_text()
                    .set_font_size(14.0)
                    .set_color(p.get_foreground());
                tree.create_text(t.clone(), st)
            }
            EuvMarkdownInline::Bold(t) => {
                let mut st: Style = Style::default().selectable(is_selectable);
                st.get_mut_layout().set_max_width(Length::Percent(100.0));
                st.get_mut_text()
                    .set_font_size(14.0)
                    .set_weight(FontWeight::Bold)
                    .set_color(p.get_foreground());
                tree.create_text(t.clone(), st)
            }
            EuvMarkdownInline::Italic(t) => {
                let mut st: Style = Style::default().selectable(is_selectable);
                st.get_mut_layout().set_max_width(Length::Percent(100.0));
                st.get_mut_text()
                    .set_font_size(14.0)
                    .set_italic(true)
                    .set_color(p.get_foreground());
                tree.create_text(t.clone(), st)
            }
            EuvMarkdownInline::BoldItalic(t) => {
                let mut st: Style = Style::default().selectable(is_selectable);
                st.get_mut_layout().set_max_width(Length::Percent(100.0));
                st.get_mut_text()
                    .set_font_size(14.0)
                    .set_weight(FontWeight::Bold)
                    .set_italic(true)
                    .set_color(p.get_foreground());
                tree.create_text(t.clone(), st)
            }
            EuvMarkdownInline::Strike(t) => {
                let mut st: Style = Style::default().selectable(is_selectable);
                st.get_mut_layout().set_max_width(Length::Percent(100.0));
                st.get_mut_text()
                    .set_font_size(14.0)
                    .set_color(p.get_muted_foreground())
                    .set_strikethrough(true);
                tree.create_text(t.clone(), st)
            }
            EuvMarkdownInline::Highlight(t) => {
                let mut row_st: Style = Style::default();
                row_st
                    .get_mut_layout()
                    .set_flex_direction(FlexDirection::Row)
                    .set_max_width(Length::Percent(100.0))
                    .set_padding(Edges::symmetric(1.0, 5.0))
                    .set_overflow(Overflow::Hidden);
                row_st
                    .get_mut_paint()
                    .set_radius(CornerRadius::all(g.get_radius_sm().clamp(0.0, 5.0)))
                    .set_background(Some(hl_bg));
                let mut txt_st: Style = Style::default().selectable(is_selectable);
                txt_st
                    .get_mut_layout()
                    .set_max_width(Length::Percent(100.0));
                txt_st
                    .get_mut_text()
                    .set_font_size(13.5)
                    .set_weight(FontWeight::Medium)
                    .set_color(hl_fg);
                let row_id: NodeId = tree.create_view(row_st);
                let txt_id: NodeId = tree.create_text(t.clone(), txt_st);
                tree.append_child(row_id, txt_id);
                row_id
            }
            EuvMarkdownInline::Superscript(t) => {
                let mut row_st: Style = Style::default();
                row_st
                    .get_mut_layout()
                    .set_flex_direction(FlexDirection::Row)
                    .set_padding(Edges::new(0.0, 1.0, 6.0, 1.0));
                let mut txt_st: Style = Style::default().selectable(is_selectable);
                txt_st
                    .get_mut_text()
                    .set_font_size(10.5)
                    .set_weight(FontWeight::SemiBold)
                    .set_color(p.get_foreground());
                let row_id: NodeId = tree.create_view(row_st);
                let txt_id: NodeId = tree.create_text(t.clone(), txt_st);
                tree.append_child(row_id, txt_id);
                row_id
            }
            EuvMarkdownInline::Subscript(t) => {
                let mut row_st: Style = Style::default();
                row_st
                    .get_mut_layout()
                    .set_flex_direction(FlexDirection::Row)
                    .set_padding(Edges::new(6.0, 1.0, 0.0, 1.0));
                let mut txt_st: Style = Style::default().selectable(is_selectable);
                txt_st
                    .get_mut_text()
                    .set_font_size(10.5)
                    .set_weight(FontWeight::SemiBold)
                    .set_color(p.get_foreground());
                let row_id: NodeId = tree.create_view(row_st);
                let txt_id: NodeId = tree.create_text(t.clone(), txt_st);
                tree.append_child(row_id, txt_id);
                row_id
            }
            EuvMarkdownInline::Code(code) => {
                EuvMarkdownInlineCode::mount_in_tree(tree, code.as_str(), tokens, eva)
            }
            EuvMarkdownInline::Math(expr) => {
                let mut row_st: Style = Style::default();
                row_st
                    .get_mut_layout()
                    .set_flex_direction(FlexDirection::Row)
                    .set_max_width(Length::Percent(100.0))
                    .set_padding(Edges::symmetric(1.0, 6.0))
                    .set_overflow(Overflow::Hidden);
                row_st
                    .get_mut_paint()
                    .set_radius(CornerRadius::all(g.get_radius_sm().clamp(0.0, 5.0)))
                    .set_background(Some(eva.get_inline_code_bg()))
                    .set_border(Border::new(1.0, eva.get_border()));
                let mut txt_st: Style = Style::default().selectable(is_selectable);
                txt_st
                    .get_mut_layout()
                    .set_max_width(Length::Percent(100.0));
                txt_st
                    .get_mut_text()
                    .set_font_size(13.0)
                    .set_family(FontFamily::Monospace)
                    .set_color(eva.get_keyword());
                let row_id: NodeId = tree.create_view(row_st);
                let txt_id: NodeId = tree.create_text(expr.clone(), txt_st);
                tree.append_child(row_id, txt_id);
                row_id
            }
            EuvMarkdownInline::FootnoteRef(id) => {
                let mut row_st: Style = Style::default();
                row_st
                    .get_mut_layout()
                    .set_flex_direction(FlexDirection::Row)
                    .set_padding(Edges::new(0.0, 4.0, 5.0, 4.0));
                row_st
                    .get_mut_paint()
                    .set_radius(CornerRadius::all(g.get_radius_sm().clamp(0.0, 4.0)))
                    .set_background(Some(p.get_primary().with_alpha(28)));
                let mut txt_st: Style = Style::default();
                txt_st
                    .get_mut_text()
                    .set_font_size(10.0)
                    .set_weight(FontWeight::Bold)
                    .set_color(p.get_primary());
                let row_id: NodeId = tree.create_view(row_st);
                let txt_id: NodeId = tree.create_text(format!("[{id}]"), txt_st);
                tree.append_child(row_id, txt_id);
                row_id
            }
            EuvMarkdownInline::Image { alt, url } => {
                let caption: &str = if alt.is_empty() {
                    url.as_str()
                } else {
                    alt.as_str()
                };
                let mut row_st: Style = Style::default();
                row_st
                    .get_mut_layout()
                    .set_flex_direction(FlexDirection::Row)
                    .set_max_width(Length::Percent(100.0))
                    .set_padding(Edges::symmetric(2.0, 6.0))
                    .set_row_gap(4.0)
                    .set_column_gap(4.0)
                    .set_align_items(AlignItems::Center)
                    .set_overflow(Overflow::Hidden);
                row_st
                    .get_mut_paint()
                    .set_radius(CornerRadius::all(g.get_radius_sm().max(0.0)))
                    .set_background(Some(eva.get_header_bg()))
                    .set_border(Border::new(1.0, eva.get_border()));
                let mut badge_st: Style = Style::default();
                badge_st
                    .get_mut_text()
                    .set_font_size(10.0)
                    .set_weight(FontWeight::Bold)
                    .set_color(eva.get_keyword());
                let mut cap_st: Style = Style::default();
                cap_st
                    .get_mut_layout()
                    .set_max_width(Length::Percent(100.0));
                cap_st
                    .get_mut_text()
                    .set_font_size(12.5)
                    .set_color(p.get_foreground());
                let row_id: NodeId = tree.create_view(row_st);
                let badge_id: NodeId = tree.create_text(MD_IMAGE_BADGE.to_string(), badge_st);
                let cap_id: NodeId = tree.create_text(caption.to_string(), cap_st);
                tree.append_children(row_id, &[badge_id, cap_id]);
                row_id
            }
            EuvMarkdownInline::Kbd(key) => {
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
                let row_id: NodeId = tree.create_view(row_st);
                let txt_id: NodeId = tree.create_text(key.clone(), txt_st);
                tree.append_child(row_id, txt_id);
                row_id
            }
            EuvMarkdownInline::Link { label, url } => {
                let cb_opt: Option<MarkdownStrCallback> = on_link.cloned();
                let url_owned: String = url.clone();
                let click_handler: UiEventHandler = Rc::new(RefCell::new(move |_: &UiEvent| {
                    if let Some(cb) = cb_opt.as_ref() {
                        cb(&url_owned);
                    }
                }));
                let mut row_st: Style = Style::default();
                row_st
                    .get_mut_layout()
                    .set_flex_direction(FlexDirection::Row)
                    .set_max_width(Length::Percent(100.0))
                    .set_padding(Edges::symmetric(1.0, 3.0));
                row_st
                    .get_mut_paint()
                    .set_radius(CornerRadius::all(4.0))
                    .set_cursor(CursorIcon::Pointer);
                let mut link_st: Style = Style::default().selectable(is_selectable);
                link_st
                    .get_mut_layout()
                    .set_max_width(Length::Percent(100.0));
                link_st
                    .get_mut_text()
                    .set_font_size(14.0)
                    .set_weight(FontWeight::SemiBold)
                    .set_color(eva.get_link())
                    .set_underline(true);
                let row_id: NodeId = tree.create_view(row_st);
                if let Some(node) = tree.get_node_mut(row_id) {
                    node.get_mut_handlers().set_on_click(Some(click_handler));
                    node.get_mut_interaction().set_focusable(true);
                }
                let txt_id: NodeId = tree.create_text(label.clone(), link_st);
                tree.append_child(row_id, txt_id);
                row_id
            }
            EuvMarkdownInline::HardBreak => {
                let mut st: Style = Style::default();
                st.get_mut_layout()
                    .set_width(Length::Px(0.0))
                    .set_height(Length::Px(0.0));
                tree.create_view(st)
            }
        }
    }

    /// Mounts this paragraph directly into an already-borrowed `SceneTree`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Paragraph component reference.
    /// - `&mut SceneTree` - Mutable scene tree.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted paragraph handle.
    pub(crate) fn mount_in_tree(&self, tree: &mut SceneTree) -> NodeId {
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
            return Self::mount_plain_in_tree(tree, plain.as_str(), p, is_selectable);
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
            return Self::mount_plain_in_tree(tree, &plain, p, is_selectable);
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
        let mut wrap_row_st: Style = Style::default().selectable(is_selectable);
        wrap_row_st
            .get_mut_layout()
            .set_flex_direction(FlexDirection::Row)
            .set_width(Length::Percent(100.0))
            .set_min_width(Length::Px(0.0))
            .set_flex_wrap(FlexWrap::Wrap)
            .set_align_items(AlignItems::Center)
            .set_overflow(Overflow::Hidden)
            .set_row_gap(4.0)
            .set_column_gap(4.0);
        let has_hard_break: bool = inlines
            .iter()
            .any(|seg: &EuvMarkdownInline| matches!(seg, EuvMarkdownInline::HardBreak));
        if !has_hard_break {
            let mut nodes: Vec<NodeId> = Vec::with_capacity(inlines.len());
            for seg in inlines {
                nodes.push(Self::render_inline_in_tree(
                    tree,
                    seg,
                    tokens,
                    eva,
                    hl_colors,
                    is_selectable,
                    on_link_ref,
                ));
            }
            let row_id: NodeId = tree.create_view(wrap_row_st);
            if !nodes.is_empty() {
                tree.append_children(row_id, &nodes);
            }
            return row_id;
        }
        let mut line_rows: Vec<NodeId> = Vec::new();
        let mut line_nodes: Vec<NodeId> = Vec::new();
        for line_segs in
            inlines.split(|seg: &EuvMarkdownInline| matches!(seg, EuvMarkdownInline::HardBreak))
        {
            line_nodes.clear();
            line_nodes.reserve(line_segs.len());
            for seg in line_segs {
                line_nodes.push(Self::render_inline_in_tree(
                    tree,
                    seg,
                    tokens,
                    eva,
                    hl_colors,
                    is_selectable,
                    on_link_ref,
                ));
            }
            let row_id: NodeId = tree.create_view(wrap_row_st);
            if !line_nodes.is_empty() {
                tree.append_children(row_id, &line_nodes);
            }
            line_rows.push(row_id);
        }
        if line_rows.len() == 1 {
            return line_rows.remove(0);
        }
        let mut col_st: Style = Style::default().selectable(is_selectable);
        col_st
            .get_mut_layout()
            .set_flex_direction(FlexDirection::Column)
            .set_width(Length::Percent(100.0))
            .set_row_gap(4.0)
            .set_column_gap(4.0);
        let col_id: NodeId = tree.create_view(col_st);
        if !line_rows.is_empty() {
            tree.append_children(col_id, &line_rows);
        }
        col_id
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
        let ctx: Context = current_context();
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| self.mount_in_tree(tree),
            NodeId::default(),
        )
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
    /// Mounts this quote or GitHub Alert callout directly into an already-borrowed `SceneTree`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Quote component reference.
    /// - `&mut SceneTree` - Mutable scene tree.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted callout node handle.
    pub(crate) fn mount_in_tree(&self, tree: &mut SceneTree) -> NodeId {
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
        let mut body_nodes: Vec<NodeId> = Vec::with_capacity(lines.len() + 1);
        if let Some(tag) = badge_opt {
            let mut tag_row_st: Style = Style::default();
            tag_row_st
                .get_mut_layout()
                .set_flex_direction(FlexDirection::Row)
                .set_align_items(AlignItems::Center)
                .set_row_gap(6.0)
                .set_column_gap(6.0);
            let mut dot_st: Style = Style::default();
            dot_st
                .get_mut_layout()
                .set_width(Length::Px(7.0))
                .set_height(Length::Px(7.0));
            dot_st
                .get_mut_paint()
                .set_radius(CornerRadius::all(99.0))
                .set_background(Some(accent));
            let mut tag_txt_st: Style = Style::default();
            tag_txt_st
                .get_mut_text()
                .set_font_size(11.5)
                .set_weight(FontWeight::Bold)
                .set_color(accent);
            let tag_row: NodeId = tree.create_view(tag_row_st);
            let dot_id: NodeId = tree.create_view(dot_st);
            let tag_txt: NodeId = tree.create_text(tag.to_string(), tag_txt_st);
            tree.append_children(tag_row, &[dot_id, tag_txt]);
            body_nodes.push(tag_row);
        }
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
                    body_nodes.push(nested.mount_in_tree(tree));
                }
                if !line.trim().is_empty() {
                    if !has_inline_markdown_trigger(line) {
                        body_nodes.push(EuvMarkdownParagraph::mount_plain_in_tree(
                            tree,
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
                        body_nodes.push(para.mount_in_tree(tree));
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
            body_nodes.push(nested.mount_in_tree(tree));
        }
        let mut root_st: Style = Style::default().selectable(is_selectable);
        root_st
            .get_mut_layout()
            .set_flex_direction(FlexDirection::Row)
            .set_width(Length::Percent(100.0))
            .set_overflow(Overflow::Hidden);
        root_st
            .get_mut_paint()
            .set_background(Some(bg))
            .set_border(Border::new(1.0, accent.with_alpha(96)))
            .set_radius(CornerRadius::all(g.get_radius_md().max(0.0)));
        let mut bar_st: Style = Style::default();
        bar_st
            .get_mut_layout()
            .set_width(Length::Px(4.0))
            .set_flex_shrink(0.0);
        bar_st.get_mut_paint().set_background(Some(accent));
        let mut col_st: Style = Style::default();
        col_st
            .get_mut_layout()
            .set_flex_direction(FlexDirection::Column)
            .set_flex_grow(1.0)
            .set_flex_shrink(1.0)
            .set_min_width(Length::Px(0.0))
            .set_padding(Edges::symmetric(SPACE_SM, SPACE_MD))
            .set_row_gap(SPACE_XS)
            .set_column_gap(SPACE_XS)
            .set_overflow(Overflow::Hidden);
        let root: NodeId = tree.create_view(root_st);
        let bar_id: NodeId = tree.create_view(bar_st);
        let col_id: NodeId = tree.create_view(col_st);
        if !body_nodes.is_empty() {
            tree.append_children(col_id, &body_nodes);
        }
        tree.append_children(root, &[bar_id, col_id]);
        root
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
        let ctx: Context = current_context();
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| self.mount_in_tree(tree),
            NodeId::default(),
        )
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
    /// Mounts this list directly into an already-borrowed `SceneTree` with zero per-checkbox signals or effects.
    ///
    /// # Arguments
    ///
    /// - `&Self` - List component reference.
    /// - `&mut SceneTree` - Mutable scene tree.
    /// - `&SceneHandle` - Shared scene handle captured by task-checkbox toggle handlers.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted list column handle.
    pub(crate) fn mount_in_tree(&self, tree: &mut SceneTree, scene: &SceneHandle) -> NodeId {
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
        let mut para_col_st: Style = Style::default();
        para_col_st
            .get_mut_layout()
            .set_flex_direction(FlexDirection::Column)
            .set_flex_grow(1.0)
            .set_flex_shrink(1.0)
            .set_min_width(Length::Px(0.0))
            .set_overflow(Overflow::Hidden);
        let mut chk_box_st: Style = Style::default().selectable(false);
        chk_box_st
            .get_mut_layout()
            .set_flex_direction(FlexDirection::Row)
            .set_width(Length::Px(28.0))
            .set_height(Length::Px(22.0))
            .set_justify_content(JustifyContent::Center)
            .set_align_items(AlignItems::Center);
        chk_box_st
            .get_mut_paint()
            .set_radius(CornerRadius::all(g.get_radius_sm().clamp(0.0, 5.0)))
            .set_background(Some(eva.get_inline_code_bg()))
            .set_border(Border::new(1.0, eva.get_border()));
        let mut chk_txt_st: Style = Style::default();
        chk_txt_st
            .get_mut_text()
            .set_font_size(11.0)
            .set_family(FontFamily::Monospace)
            .set_weight(FontWeight::Bold)
            .set_color(eva.get_keyword())
            .set_align(TextAlign::Center)
            .set_overflow(TextOverflow::Clip);
        let mut marker_st: Style = Style::default().selectable(false);
        marker_st
            .get_mut_text()
            .set_font_size(14.0)
            .set_weight(FontWeight::Bold)
            .set_color(eva.get_keyword());
        let mut row_nodes: Vec<NodeId> = Vec::with_capacity(items.len());
        for (idx, item) in items.iter().enumerate() {
            let indent: f32 = (item.get_depth() as f32) * 18.0;
            let text_ref: &str = item.get_text().as_str();
            let para_node: NodeId = if !has_inline_markdown_trigger(text_ref) {
                EuvMarkdownParagraph::mount_plain_in_tree(tree, text_ref, p, is_selectable)
            } else {
                let mut para: EuvMarkdownParagraph =
                    EuvMarkdownParagraph::new(parse_markdown_inlines(text_ref), tokens)
                        .selectable(is_selectable);
                if let Some(cb) = on_link.clone() {
                    para.set_on_link(Some(cb));
                }
                para.mount_in_tree(tree)
            };
            let para_col: NodeId = tree.create_view(para_col_st);
            tree.append_child(para_col, para_node);
            if let Some(initial_checked) = item.checked_state() {
                let chk_text: NodeId = tree.create_text(
                    if initial_checked {
                        MD_CHECKBOX_ON.to_string()
                    } else {
                        MD_CHECKBOX_OFF.to_string()
                    },
                    chk_txt_st,
                );
                let chk_box: NodeId = tree.create_view(chk_box_st);
                tree.append_child(chk_box, chk_text);
                let mut item_row_st: Style = Style::default();
                item_row_st
                    .get_mut_layout()
                    .set_flex_direction(FlexDirection::Row)
                    .set_width(Length::Percent(100.0))
                    .set_padding(Edges::new(2.0, 0.0, 2.0, indent))
                    .set_align_items(AlignItems::Center)
                    .set_row_gap(SPACE_SM)
                    .set_column_gap(SPACE_SM);
                item_row_st.get_mut_paint().set_cursor(CursorIcon::Pointer);
                let item_row: NodeId = tree.create_view(item_row_st);
                let chk_cell: Rc<Cell<bool>> = Rc::new(Cell::new(initial_checked));
                let cb_opt: Option<MarkdownTaskCallback> = on_toggle_task.clone();
                let scene_toggle: SceneHandle = scene.clone();
                let toggle_handler: UiEventHandler = Rc::new(RefCell::new(move |_: &UiEvent| {
                    let next: bool = !chk_cell.get();
                    chk_cell.set(next);
                    let next_label: &'static str = if next {
                        MD_CHECKBOX_ON
                    } else {
                        MD_CHECKBOX_OFF
                    };
                    scene_toggle.with_write(
                        |t: &mut SceneTree| {
                            t.set_text(chk_text, next_label.to_string());
                        },
                        (),
                    );
                    if let Some(cb) = cb_opt.as_ref() {
                        cb(idx, next);
                    }
                }));
                if let Some(node) = tree.get_node_mut(item_row) {
                    node.get_mut_handlers().set_on_click(Some(toggle_handler));
                    node.get_mut_interaction().set_focusable(true);
                }
                tree.append_children(item_row, &[chk_box, para_col]);
                row_nodes.push(item_row);
            } else {
                let marker_txt: String = if ordered {
                    format!("{}.", start_num + idx)
                } else if item.get_depth() % 2 == 1 {
                    "◦".to_string()
                } else {
                    "•".to_string()
                };
                let marker_node: NodeId = tree.create_text(marker_txt, marker_st);
                let mut item_row_st: Style = Style::default();
                item_row_st
                    .get_mut_layout()
                    .set_flex_direction(FlexDirection::Row)
                    .set_width(Length::Percent(100.0))
                    .set_padding(Edges::new(2.0, 0.0, 2.0, indent))
                    .set_align_items(AlignItems::Start)
                    .set_row_gap(SPACE_SM)
                    .set_column_gap(SPACE_SM);
                let item_row: NodeId = tree.create_view(item_row_st);
                tree.append_children(item_row, &[marker_node, para_col]);
                row_nodes.push(item_row);
            }
        }
        let mut col_st: Style = Style::default();
        col_st
            .get_mut_layout()
            .set_flex_direction(FlexDirection::Column)
            .set_width(Length::Percent(100.0))
            .set_row_gap(SPACE_XS)
            .set_column_gap(SPACE_XS);
        let root: NodeId = tree.create_view(col_st);
        if !row_nodes.is_empty() {
            tree.append_children(root, &row_nodes);
        }
        root
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
        let ctx: Context = current_context();
        let scene: SceneHandle = ctx.get_scene().clone();
        scene.with_write(
            |tree: &mut SceneTree| self.mount_in_tree(tree, &scene),
            NodeId::default(),
        )
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
    /// Mounts the complete GFM aligned table directly into an already-borrowed `SceneTree`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Table component reference.
    /// - `&mut SceneTree` - Mutable scene tree.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted table scroll container handle.
    pub(crate) fn mount_in_tree(&self, tree: &mut SceneTree) -> NodeId {
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
        let mut base_cell_st: Style = Style::default();
        base_cell_st
            .get_mut_layout()
            .set_flex_direction(FlexDirection::Row)
            .set_flex_grow(1.0)
            .set_flex_shrink(1.0)
            .set_min_width(Length::Px(MD_TABLE_COL_MIN_WIDTH))
            .set_padding(Edges::symmetric(SPACE_SM, SPACE_SM))
            .set_overflow(Overflow::Hidden);
        let mut base_cell_txt_st: Style = Style::default().selectable(is_selectable);
        base_cell_txt_st
            .get_mut_layout()
            .set_width(Length::Percent(100.0));
        base_cell_txt_st
            .get_mut_text()
            .set_font_size(13.0)
            .set_color(p.get_foreground());
        let mut row_wrap_col_st: Style = Style::default();
        row_wrap_col_st
            .get_mut_layout()
            .set_flex_direction(FlexDirection::Column)
            .set_width(Length::Percent(100.0));
        let mut row_div_st: Style = Style::default();
        row_div_st
            .get_mut_layout()
            .set_width(Length::Percent(100.0))
            .set_height(Length::Px(1.0));
        row_div_st
            .get_mut_paint()
            .set_background(Some(p.get_border()));
        let mut base_data_row_st: Style = Style::default();
        base_data_row_st
            .get_mut_layout()
            .set_flex_direction(FlexDirection::Row)
            .set_width(Length::Percent(100.0));
        let even_bg: Color = p.get_card();
        let odd_bg: Color = p.get_muted().with_alpha(96);
        let mut table_scroll_st: Style = Style::default();
        table_scroll_st
            .get_mut_layout()
            .set_flex_direction(FlexDirection::Column)
            .set_align_items(AlignItems::Start)
            .set_width(Length::Percent(100.0));
        table_scroll_st
            .get_mut_paint()
            .set_border(Border::new(1.0, p.get_border()))
            .set_radius(CornerRadius::all(g.get_radius_md().max(0.0)));
        let scroll_id: NodeId = tree.create_scroll_view(table_scroll_st);
        let mut inner_col_st: Style = Style::default().selectable(is_selectable);
        inner_col_st
            .get_mut_layout()
            .set_flex_direction(FlexDirection::Column)
            .set_min_width(Length::Px(min_table_w))
            .set_width(Length::Percent(100.0))
            .set_flex_shrink(0.0);
        let inner_col: NodeId = tree.create_view(inner_col_st);
        let mut head_row_st: Style = Style::default();
        head_row_st
            .get_mut_layout()
            .set_flex_direction(FlexDirection::Row)
            .set_width(Length::Percent(100.0));
        head_row_st
            .get_mut_paint()
            .set_background(Some(eva.get_header_bg()));
        let head_row: NodeId = tree.create_view(head_row_st);
        let mut base_hdr_txt_st: Style = Style::default().selectable(is_selectable);
        base_hdr_txt_st
            .get_mut_layout()
            .set_width(Length::Percent(100.0));
        base_hdr_txt_st
            .get_mut_text()
            .set_font_size(12.5)
            .set_weight(FontWeight::Bold)
            .set_color(p.get_foreground());
        let mut cell_buf: Vec<NodeId> = Vec::with_capacity(col_count);
        for (ci, header) in headers.iter().enumerate() {
            let h_cell: NodeId = tree.create_view(base_cell_st);
            let mut hdr_txt_st: Style = base_hdr_txt_st;
            hdr_txt_st
                .get_mut_text()
                .set_align(Self::align_pair(alignments, ci).0);
            let h_txt: NodeId = tree.create_text(header.clone(), hdr_txt_st);
            tree.append_child(h_cell, h_txt);
            cell_buf.push(h_cell);
        }
        if !cell_buf.is_empty() {
            tree.append_children(head_row, &cell_buf);
        }
        let mut table_children: Vec<NodeId> = Vec::with_capacity(rows.len() + 1);
        table_children.push(head_row);
        for (ri, r) in rows.iter().enumerate() {
            cell_buf.clear();
            for ci in 0..col_count {
                let cell_val: &str = r.get(ci).map(String::as_str).unwrap_or("");
                let (cell_align, cell_justify): (TextAlign, JustifyContent) =
                    Self::align_pair(alignments, ci);
                let mut c_st: Style = base_cell_st;
                c_st.get_mut_layout().set_justify_content(cell_justify);
                let cell_id: NodeId = tree.create_view(c_st);
                if !has_inline_markdown_trigger(cell_val) {
                    let mut t_st: Style = base_cell_txt_st;
                    t_st.get_mut_text().set_align(cell_align);
                    let t_id: NodeId = tree.create_text(cell_val.to_string(), t_st);
                    tree.append_child(cell_id, t_id);
                } else {
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
                        let mut t_st: Style = base_cell_txt_st;
                        t_st.get_mut_text().set_align(cell_align);
                        let t_id: NodeId = tree.create_text(plain_val, t_st);
                        tree.append_child(cell_id, t_id);
                    } else {
                        let mut para: EuvMarkdownParagraph =
                            EuvMarkdownParagraph::new(inlines, tokens).selectable(is_selectable);
                        if let Some(cb) = on_link.clone() {
                            para.set_on_link(Some(cb));
                        }
                        let cell_node: NodeId = para.mount_in_tree(tree);
                        tree.append_child(cell_id, cell_node);
                    }
                }
                cell_buf.push(cell_id);
            }
            let wrap_col: NodeId = tree.create_view(row_wrap_col_st);
            let div_id: NodeId = tree.create_view(row_div_st);
            let mut d_row_st: Style = base_data_row_st;
            d_row_st
                .get_mut_paint()
                .set_background(Some(if ri % 2 == 1 { odd_bg } else { even_bg }));
            let d_row: NodeId = tree.create_view(d_row_st);
            if !cell_buf.is_empty() {
                tree.append_children(d_row, &cell_buf);
            }
            tree.append_children(wrap_col, &[div_id, d_row]);
            table_children.push(wrap_col);
        }
        tree.append_children(inner_col, &table_children);
        tree.append_child(scroll_id, inner_col);
        scroll_id
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
        let ctx: Context = current_context();
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| self.mount_in_tree(tree),
            NodeId::default(),
        )
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
                let term_node: NodeId = if !has_inline_markdown_trigger(term) {
                    view! {
                        text {
                            width: Length::Percent(100.0)
                            font_size: 14.0
                            font_weight: FontWeight::Bold
                            color: p.get_foreground()
                            selectable: is_selectable
                            { term }
                        }
                    }
                } else {
                    let mut term_para: EuvMarkdownParagraph =
                        EuvMarkdownParagraph::new(parse_markdown_inlines(term), tokens)
                            .selectable(is_selectable);
                    if let Some(cb) = on_link.clone() {
                        term_para.set_on_link(Some(cb));
                    }
                    term_para.mount()
                };
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
                        { term_node }
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

/// Direct tree mounting helper for `EuvMarkdownDivider`.
impl EuvMarkdownDivider {
    /// Mounts a full-width thematic break directly into an already-borrowed `SceneTree`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Divider component reference.
    /// - `&mut SceneTree` - Mutable scene tree.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted divider handle.
    pub(crate) fn mount_in_tree(&self, tree: &mut SceneTree) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let mut st: Style = Style::default();
        st.get_mut_layout()
            .set_width(Length::Percent(100.0))
            .set_height(Length::Px(1.0))
            .set_margin(Edges::symmetric(SPACE_SM, 0.0));
        st.get_mut_paint().set_background(Some(p.get_border()));
        tree.create_view(st)
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
        let ctx: Context = current_context();
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| self.mount_in_tree(tree),
            NodeId::default(),
        )
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
        let ctx: Context = current_context();
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
        let all_pure: bool = blocks.iter().all(|blk: &EuvMarkdownBlock| {
            matches!(
                blk,
                EuvMarkdownBlock::Heading { .. }
                    | EuvMarkdownBlock::Paragraph(_)
                    | EuvMarkdownBlock::CodeBlock { .. }
                    | EuvMarkdownBlock::Quote { .. }
                    | EuvMarkdownBlock::List { .. }
                    | EuvMarkdownBlock::Table { .. }
                    | EuvMarkdownBlock::Divider
            )
        });
        let mut col_st: Style = Style::default().selectable(is_selectable);
        col_st
            .get_mut_layout()
            .set_flex_direction(FlexDirection::Column)
            .set_width(Length::Percent(100.0))
            .set_row_gap(SPACE_MD)
            .set_column_gap(SPACE_MD);
        if all_pure {
            let scene: SceneHandle = ctx.get_scene().clone();
            return scene.with_write(
                |tree: &mut SceneTree| {
                    let mut children: Vec<NodeId> = Vec::with_capacity(blocks.len());
                    for blk in blocks.into_iter() {
                        let node_id: NodeId = match blk {
                            EuvMarkdownBlock::Heading { level, text } => {
                                EuvMarkdownHeading::new(level, text, tokens)
                                    .selectable(is_selectable)
                                    .mount_in_tree(tree)
                            }
                            EuvMarkdownBlock::Paragraph(inlines) => {
                                let mut para: EuvMarkdownParagraph =
                                    EuvMarkdownParagraph::new(inlines, tokens)
                                        .selectable(is_selectable);
                                if let Some(cb) = on_link.clone() {
                                    para.set_on_link(Some(cb));
                                }
                                para.mount_in_tree(tree)
                            }
                            EuvMarkdownBlock::CodeBlock { language, code } => {
                                let mut cb_block: EuvCodeBlock = EuvCodeBlock::new(code, tokens)
                                    .language(language)
                                    .line_numbers(true)
                                    .selectable(is_selectable);
                                if let Some(cb) = on_copy.clone() {
                                    cb_block.set_on_copy(Some(cb));
                                }
                                cb_block.mount_in_tree(tree, &ctx)
                            }
                            EuvMarkdownBlock::Quote { kind, lines, .. } => {
                                let mut quote: EuvMarkdownQuote =
                                    EuvMarkdownQuote::new(kind, lines, tokens)
                                        .selectable(is_selectable);
                                if let Some(cb) = on_link.clone() {
                                    quote.set_on_link(Some(cb));
                                }
                                quote.mount_in_tree(tree)
                            }
                            EuvMarkdownBlock::List {
                                ordered,
                                start,
                                items,
                            } => {
                                let mut list: EuvMarkdownList =
                                    EuvMarkdownList::new(ordered, items, tokens)
                                        .start(start)
                                        .selectable(is_selectable);
                                if let Some(cb) = on_link.clone() {
                                    list.set_on_link(Some(cb));
                                }
                                if let Some(cb) = on_toggle_task.clone() {
                                    list.set_on_toggle_task(Some(cb));
                                }
                                list.mount_in_tree(tree, &scene)
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
                                tbl.mount_in_tree(tree)
                            }
                            EuvMarkdownBlock::Divider => {
                                EuvMarkdownDivider::new(tokens).mount_in_tree(tree)
                            }
                            _ => continue,
                        };
                        children.push(node_id);
                    }
                    let root: NodeId = tree.create_view(col_st);
                    if !children.is_empty() {
                        tree.append_children(root, &children);
                    }
                    root
                },
                NodeId::default(),
            );
        }
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
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                let root: NodeId = tree.create_view(col_st);
                if !children.is_empty() {
                    tree.append_children(root, &children);
                }
                root
            },
            NodeId::default(),
        )
    }
}
