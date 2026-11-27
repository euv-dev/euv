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
    /// Renders the syntax-highlighted Eva-themed code block with header chrome, gutter, and interactive Copy pill.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Code block reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted code block container handle.
    fn render(&self) -> NodeId {
        let tokens: ThemeTokens = self.get_tokens();
        let g: ThemeGeometry = tokens.get_geometry();
        let eva: CodeThemePalette = CodeThemePalette::resolve(tokens.get_mode());
        let Self {
            code,
            language,
            filename,
            show_line_numbers,
            highlight_lines,
            on_copy,
            ..
        } = self.clone();
        let resolved_lang: String = if language.trim().is_empty() {
            infer_code_language(&code).to_string()
        } else {
            language.trim().to_string()
        };
        let display_lang: String = resolved_lang.to_ascii_uppercase();
        let code_lang: CodeLang = CodeLang::from_name(&resolved_lang);
        let header_title: String = filename.unwrap_or_else(|| eva.get_theme_name().to_string());
        let copied_sig: Signal<bool> = Signal::new(false);
        let code_for_copy: String = code.clone();
        let raw_lines: Vec<&str> = code.lines().collect();
        let lines_vec: Vec<String> = if raw_lines.is_empty() {
            vec![String::new()]
        } else {
            raw_lines.into_iter().map(str::to_string).collect()
        };
        let show_nums: bool = show_line_numbers || lines_vec.len() > 1;
        let code_font_size: f32 = 12.5;
        let mono_cell_w: f32 = code_font_size * MONOSPACE_ADVANCE_RATIO;
        let line_nodes: Vec<NodeId> = lines_vec
            .into_iter()
            .enumerate()
            .map(|(idx, line_text): (usize, String)| {
                let line_no: usize = idx + 1;
                let is_hl: bool = highlight_lines.contains(&line_no);
                let expanded_line: String = line_text.replace('\t', "  ");
                let leading_spaces: usize = expanded_line
                    .bytes()
                    .take_while(|b: &u8| *b == b' ')
                    .count();
                let trimmed_line: &str = &expanded_line[leading_spaces..];
                let indent_px: f32 = (leading_spaces as f32) * mono_cell_w;
                let spans: Vec<CodeSpan> = tokenize_code_line_with_lang(trimmed_line, code_lang);
                let mut merged: Vec<(String, Color, FontWeight, bool)> =
                    Vec::with_capacity(spans.len());
                for sp in spans.into_iter() {
                    let kind: CodeTokenKind = sp.get_kind();
                    let (fg, weight): (Color, FontWeight) = eva.token_style(kind);
                    let is_italic: bool = kind == CodeTokenKind::Comment;
                    if let Some(last) = merged.last_mut()
                        && last.1 == fg
                        && last.2 == weight
                        && last.3 == is_italic
                    {
                        last.0.push_str(sp.get_text());
                    } else {
                        merged.push((sp.get_text().clone(), fg, weight, is_italic));
                    }
                }
                let span_nodes: Vec<NodeId> = merged
                    .into_iter()
                    .map(|(txt, fg, weight, is_italic): (String, Color, FontWeight, bool)| {
                        view! {
                            text {
                                flex_shrink: 0.0
                                font_size: code_font_size
                                font_family: FontFamily::Monospace
                                font_weight: weight
                                italic: is_italic
                                color: fg
                                text_overflow: TextOverflow::Clip
                                { txt }
                            }
                        }
                    })
                    .collect();
                let row_bg: Color = if is_hl {
                    eva.get_line_highlight()
                } else {
                    COLOR_TRANSPARENT
                };
                view! {
                    row {
                        min_width: Length::Percent(100.0)
                        min_height: Length::Px(20.0)
                        padding: Edges::symmetric(1.0, SPACE_SM)
                        align: AlignItems::Center
                        background: row_bg
                        if show_nums {
                            row {
                                width: Length::Px(34.0)
                                flex_shrink: 0.0
                                padding: Edges::new(0.0, SPACE_SM, 0.0, 0.0)
                                justify: JustifyContent::End
                                align: AlignItems::Center
                                text {
                                    font_size: 11.5
                                    font_family: FontFamily::Monospace
                                    color: if is_hl { eva.get_foreground() } else { eva.get_line_number() }
                                    text_overflow: TextOverflow::Clip
                                    { format!("{line_no}") }
                                }
                            }
                        }
                        row {
                            flex_shrink: 0.0
                            padding: Edges::new(0.0, SPACE_MD, 0.0, indent_px)
                            align: AlignItems::Center
                            ..span_nodes
                        }
                    }
                }
            })
            .collect();
        view! {
            column {
                width: Length::Percent(100.0)
                background: eva.get_background()
                border: (1.0, eva.get_border())
                radius: g.get_radius_lg().max(10.0)
                overflow: Overflow::Hidden
                row {
                    width: Length::Percent(100.0)
                    padding: Edges::symmetric(SPACE_SM, SPACE_MD)
                    justify: JustifyContent::SpaceBetween
                    align: AlignItems::Center
                    gap: SPACE_SM
                    overflow: Overflow::Hidden
                    background: eva.get_header_bg()
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
                            { header_title }
                        }
                    }
                    row {
                        flex_shrink: 0.0
                        align: AlignItems::Center
                        gap: SPACE_SM
                        row {
                            padding: Edges::symmetric(3.0, 8.0)
                            radius: 99.0
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
                                { display_lang }
                            }
                        }
                        row {
                            padding: Edges::symmetric(3.0, 10.0)
                            radius: 6.0
                            justify: JustifyContent::Center
                            align: AlignItems::Center
                            cursor: CursorIcon::Pointer
                            background: eva.get_background()
                            border: (1.0, eva.get_border())
                            on_click: move |_: &UiEvent| {
                                let _: bool = copied_sig.set(true);
                                if let Some(cb) = on_copy.as_ref() {
                                    cb(&code_for_copy);
                                }
                            }
                            text {
                                font_size: 11.0
                                font_weight: FontWeight::SemiBold
                                color: eva.get_function()
                                text_align: TextAlign::Center
                                text_overflow: TextOverflow::Clip
                                { move || if copied_sig.get() { CODE_BTN_COPIED.to_string() } else { CODE_BTN_COPY.to_string() } }
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
                    style: Style::default().column()
                    width: Length::Percent(100.0)
                    padding: Edges::symmetric(SPACE_SM, SPACE_XS)
                    gap: 1.0
                    ..line_nodes
                }
            }
        }
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
        let title: String = self.get_text().clone();
        let show_rule: bool = lvl <= 2;
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
                        { title }
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
        let eva: CodeThemePalette = CodeThemePalette::resolve(tokens.get_mode());
        let snippet: String = self.get_code().clone();
        view! {
            row {
                max_width: Length::Percent(100.0)
                padding: Edges::symmetric(2.0, 6.0)
                radius: 5.0
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

/// Fluent builder helpers for `EuvMarkdownParagraph`.
impl EuvMarkdownParagraph {
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
}

/// Declarative `#[component]` implementation for `EuvMarkdownParagraph`.
#[component]
/// Inherent implementation of [`EuvMarkdownParagraph`].
impl EuvMarkdownParagraph {
    /// Renders rich inline Markdown spans (bold, italic, strike, inline code, links, `<kbd>`).
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
            inlines, on_link, ..
        } = self.clone();
        let has_rich: bool = inlines
            .iter()
            .any(|seg: &EuvMarkdownInline| !matches!(seg, EuvMarkdownInline::Text(_)));
        if !has_rich {
            let plain: String = inlines
                .into_iter()
                .map(|seg: EuvMarkdownInline| match seg {
                    EuvMarkdownInline::Text(s) => s,
                    _ => String::new(),
                })
                .collect();
            return view! {
                text {
                    width: Length::Percent(100.0)
                    font_size: 14.0
                    color: p.get_foreground()
                    { plain }
                }
            };
        }
        let nodes: Vec<NodeId> = inlines
            .into_iter()
            .map(|seg: EuvMarkdownInline| match seg {
                EuvMarkdownInline::Text(t) => view! {
                    text {
                        max_width: Length::Percent(100.0)
                        font_size: 14.0
                        color: p.get_foreground()
                        { t }
                    }
                },
                EuvMarkdownInline::Bold(t) => view! {
                    text {
                        max_width: Length::Percent(100.0)
                        font_size: 14.0
                        font_weight: FontWeight::Bold
                        color: p.get_foreground()
                        { t }
                    }
                },
                EuvMarkdownInline::Italic(t) => view! {
                    text {
                        max_width: Length::Percent(100.0)
                        font_size: 14.0
                        font_weight: FontWeight::Medium
                        color: p.get_muted_foreground()
                        { t }
                    }
                },
                EuvMarkdownInline::Strike(t) => view! {
                    text {
                        max_width: Length::Percent(100.0)
                        font_size: 14.0
                        color: p.get_muted_foreground()
                        { format!("~{t}~") }
                    }
                },
                EuvMarkdownInline::Code(code) => view! {
                    EuvMarkdownInlineCode {
                        code: code
                        tokens: tokens
                    }
                },
                EuvMarkdownInline::Kbd(key) => view! {
                    EuvKbd {
                        key: key
                        tokens: tokens
                    }
                },
                EuvMarkdownInline::Link { label, url } => {
                    let cb_opt: Option<MarkdownStrCallback> = on_link.clone();
                    view! {
                        row {
                            max_width: Length::Percent(100.0)
                            padding: Edges::symmetric(1.0, 3.0)
                            radius: 4.0
                            cursor: CursorIcon::Pointer
                            on_click: move |_: &UiEvent| {
                                if let Some(cb) = cb_opt.as_ref() {
                                    cb(&url);
                                }
                            }
                            text {
                                max_width: Length::Percent(100.0)
                                font_size: 14.0
                                font_weight: FontWeight::SemiBold
                                color: eva.get_link()
                                { label }
                            }
                        }
                    }
                }
            })
            .collect();
        view! {
            row {
                width: Length::Percent(100.0)
                min_width: Length::Px(0.0)
                flex_wrap: FlexWrap::Wrap
                align: AlignItems::Center
                overflow: Overflow::Hidden
                gap: 4.0
                ..nodes
            }
        }
    }
}

/// Declarative `#[component]` implementation for `EuvMarkdownQuote`.
#[component]
/// Inherent implementation of [`EuvMarkdownQuote`].
impl EuvMarkdownQuote {
    /// Renders a Markdown blockquote or GitHub Alert callout card with Eva-aligned border and tint.
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
        let lines: Vec<String> = self.get_lines().clone();
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
        view! {
            row {
                width: Length::Percent(100.0)
                background: bg
                border: (1.0, accent.with_alpha(96))
                radius: g.get_radius_md()
                overflow: Overflow::Hidden
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
                        text {
                            font_size: 11.5
                            font_weight: FontWeight::Bold
                            color: accent
                            { tag }
                        }
                    }
                    for line in lines.into_iter() {
                        EuvMarkdownParagraph {
                            inlines: parse_markdown_inlines(&line)
                            tokens: tokens
                        }
                    }
                }
            }
        }
    }
}

/// Fluent builder helpers for `EuvMarkdownList`.
impl EuvMarkdownList {
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
        let eva: CodeThemePalette = CodeThemePalette::resolve(tokens.get_mode());
        let ordered: bool = self.get_ordered();
        let Self {
            items,
            on_toggle_task,
            ..
        } = self.clone();
        let row_nodes: Vec<NodeId> = items
            .into_iter()
            .enumerate()
            .map(|(idx, item): (usize, EuvMarkdownListItem)| {
                let indent: f32 = (item.get_depth() as f32) * 18.0;
                let text_str: String = item.get_text().clone();
                if let Some(initial_checked) = item.checked_state() {
                    let chk_sig: Signal<bool> = Signal::new(initial_checked);
                    let cb_opt: Option<MarkdownTaskCallback> = on_toggle_task.clone();
                    view! {
                        row {
                            width: Length::Percent(100.0)
                            padding: Edges::new(2.0, 0.0, 2.0, indent)
                            align: AlignItems::Center
                            gap: SPACE_SM
                            cursor: CursorIcon::Pointer
                            on_click: move |_: &UiEvent| {
                                let next: bool = !chk_sig.get_untracked();
                                let _: bool = chk_sig.set(next);
                                if let Some(cb) = cb_opt.as_ref() {
                                    cb(idx, next);
                                }
                            }
                            row {
                                width: Length::Px(28.0)
                                height: Length::Px(22.0)
                                radius: 5.0
                                justify: JustifyContent::Center
                                align: AlignItems::Center
                                background: eva.get_inline_code_bg()
                                border: (1.0, eva.get_border())
                                text {
                                    font_size: 11.0
                                    font_family: FontFamily::Monospace
                                    font_weight: FontWeight::Bold
                                    color: eva.get_keyword()
                                    text_align: TextAlign::Center
                                    text_overflow: TextOverflow::Clip
                                    { move || if chk_sig.get() { MD_CHECKBOX_ON.to_string() } else { MD_CHECKBOX_OFF.to_string() } }
                                }
                            }
                            column {
                                flex_grow: 1.0
                                flex_shrink: 1.0
                                min_width: Length::Px(0.0)
                                overflow: Overflow::Hidden
                                EuvMarkdownParagraph {
                                    inlines: parse_markdown_inlines(&text_str)
                                    tokens: tokens
                                }
                            }
                        }
                    }
                } else {
                    let marker: String = if ordered {
                        format!("{}.", idx + 1)
                    } else {
                        "•".to_string()
                    };
                    view! {
                        row {
                            width: Length::Percent(100.0)
                            padding: Edges::new(2.0, 0.0, 2.0, indent)
                            align: AlignItems::Start
                            gap: SPACE_SM
                            text {
                                font_size: 14.0
                                font_weight: FontWeight::Bold
                                color: eva.get_keyword()
                                { marker }
                            }
                            column {
                                flex_grow: 1.0
                                flex_shrink: 1.0
                                min_width: Length::Px(0.0)
                                overflow: Overflow::Hidden
                                EuvMarkdownParagraph {
                                    inlines: parse_markdown_inlines(&text_str)
                                    tokens: tokens
                                }
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

/// Declarative `#[component]` implementation for `EuvMarkdownTable`.
#[component]
/// Inherent implementation of [`EuvMarkdownTable`].
impl EuvMarkdownTable {
    /// Renders a GFM aligned Markdown table with crisp hairline borders and zebra row tint.
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
            ..
        } = self.clone();
        let col_count: usize = headers.len().max(1);
        let col_aligns: Vec<TextAlign> = (0..col_count)
            .map(
                |col_idx: usize| match alignments.get(col_idx).copied().unwrap_or_default() {
                    EuvMarkdownAlign::Left => TextAlign::Left,
                    EuvMarkdownAlign::Center => TextAlign::Center,
                    EuvMarkdownAlign::Right => TextAlign::Right,
                },
            )
            .collect();
        let body_rows: Vec<NodeId> = rows
            .into_iter()
            .enumerate()
            .map(|(ri, r): (usize, Vec<String>)| {
                let cells: Vec<NodeId> = (0..col_count)
                    .map(|ci: usize| {
                        let cell_val: String = r.get(ci).cloned().unwrap_or_default();
                        let cell_align: TextAlign =
                            col_aligns.get(ci).copied().unwrap_or(TextAlign::Left);
                        view! {
                            row {
                                flex_grow: 1.0
                                flex_shrink: 1.0
                                min_width: Length::Px(0.0)
                                padding: Edges::symmetric(SPACE_SM, SPACE_SM)
                                overflow: Overflow::Hidden
                                text {
                                    width: Length::Percent(100.0)
                                    font_size: 13.0
                                    color: p.get_foreground()
                                    text_align: cell_align
                                    { cell_val }
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
        view! {
            column {
                width: Length::Percent(100.0)
                border: (1.0, p.get_border())
                radius: g.get_radius_md()
                overflow: Overflow::Hidden
                row {
                    width: Length::Percent(100.0)
                    background: eva.get_header_bg()
                    for (ci, header) in headers.into_iter().enumerate() {
                        row {
                            flex_grow: 1.0
                            flex_shrink: 1.0
                            min_width: Length::Px(0.0)
                            padding: Edges::symmetric(SPACE_SM, SPACE_SM)
                            overflow: Overflow::Hidden
                            text {
                                width: Length::Percent(100.0)
                                font_size: 12.5
                                font_weight: FontWeight::Bold
                                color: p.get_foreground()
                                text_align: col_aligns.get(ci).copied().unwrap_or(TextAlign::Left)
                                { header }
                            }
                        }
                    }
                }
                ..body_rows
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
        let alt: String = self.get_alt().clone();
        let url: String = self.get_url().clone();
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

/// Declarative `#[component]` implementation for `EuvMarkdownDetails`.
#[component]
/// Inherent implementation of [`EuvMarkdownDetails`].
impl EuvMarkdownDetails {
    /// Renders an interactive collapsible `<details><summary>` disclosure block.
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
        let summary: String = self.get_summary().clone();
        let body: Vec<String> = self.get_body().clone();
        let body_node: NodeId = view! {
            column {
                width: Length::Percent(100.0)
                gap: SPACE_XS
                for line in body.into_iter() {
                    EuvMarkdownParagraph {
                        inlines: parse_markdown_inlines(&line)
                        tokens: tokens
                    }
                }
            }
        };
        view! {
            EuvCollapsible {
                open: open
                title: summary
                content: body_node
                tokens: tokens
            }
        }
    }
}

/// Fluent callback builders for `EuvMarkdown`.
impl EuvMarkdown {
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

    /// Attaches a copy handler invoked when any fenced `EuvCodeBlock` is copied.
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
    /// Parses and mounts a full GitHub-Flavored Markdown document natively using `view!`.
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
            on_link,
            on_copy,
            on_toggle_task,
            ..
        } = self.clone();
        let blocks: Vec<EuvMarkdownBlock> = parse_markdown_blocks(&source);
        let children: Vec<NodeId> = blocks
            .into_iter()
            .map(|blk: EuvMarkdownBlock| match blk {
                EuvMarkdownBlock::Heading { level, text } => view! {
                    EuvMarkdownHeading {
                        level: level
                        text: text
                        tokens: tokens
                    }
                },
                EuvMarkdownBlock::Paragraph(inlines) => {
                    let mut para: EuvMarkdownParagraph = EuvMarkdownParagraph::new(inlines, tokens);
                    if let Some(cb) = on_link.clone() {
                        para.set_on_link(Some(cb));
                    }
                    para.mount()
                }
                EuvMarkdownBlock::CodeBlock { language, code } => {
                    let mut cb_block: EuvCodeBlock = EuvCodeBlock::new(code, tokens)
                        .language(language)
                        .line_numbers(true);
                    if let Some(cb) = on_copy.clone() {
                        cb_block.set_on_copy(Some(cb));
                    }
                    cb_block.mount()
                }
                EuvMarkdownBlock::Quote { kind, lines } => view! {
                    EuvMarkdownQuote {
                        kind: kind
                        lines: lines
                        tokens: tokens
                    }
                },
                EuvMarkdownBlock::List { ordered, items } => {
                    let mut list: EuvMarkdownList = EuvMarkdownList::new(ordered, items, tokens);
                    if let Some(cb) = on_toggle_task.clone() {
                        list.set_on_toggle_task(Some(cb));
                    }
                    list.mount()
                }
                EuvMarkdownBlock::Table {
                    headers,
                    alignments,
                    rows,
                } => view! {
                    EuvMarkdownTable {
                        headers: headers
                        alignments: alignments
                        rows: rows
                        tokens: tokens
                    }
                },
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
                    view! {
                        EuvMarkdownDetails {
                            summary: summary
                            body: body
                            open: open_sig
                            tokens: tokens
                        }
                    }
                }
            })
            .collect();
        view! {
            column {
                width: Length::Percent(100.0)
                gap: SPACE_MD
                ..children
            }
        }
    }
}
