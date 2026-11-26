use super::*;

/// Mounts the cross-platform native documentation viewer for `site` using zero-`ctx` macros.
///
/// # Arguments
///
/// - `&DocSite` - Parsed documentation site.
/// - `Signal<String>` - Active route path signal.
/// - `Signal<String>` - Search query input signal.
/// - `ThemeTokens` - Active design tokens.
///
/// # Returns
///
/// - `NodeId` - Mounted documentation shell node.
pub fn mount_docs_viewer(
    site: &DocSite,
    active_route: Signal<String>,
    search_query: Signal<String>,
    tokens: ThemeTokens,
) -> NodeId {
    let query: String = search_query.get_untracked();
    let filtered: Vec<&DocPage> = site.search(&query);
    let nav_items: Vec<EuvNavItem> = filtered
        .iter()
        .take(24)
        .map(|page: &&DocPage| EuvNavItem::new(page.get_route().clone(), page.get_title().clone()))
        .collect();
    let sidebar_col: NodeId = view! {
        column {
            width: Length::Percent(100.0)
            gap: SPACE_MD
            { EuvInput::new(search_query, SEARCH_PLACEHOLDER.to_string(), tokens) }
            { EuvSidebarNav::new(nav_items, active_route, tokens) }
        }
    };
    let route: String = active_route.get_untracked();
    let active_page: Option<&DocPage> = site
        .find_by_route(&route)
        .or_else(|| site.get_pages().first());
    let main_col: NodeId = match active_page {
        Some(page) => mount_doc_page(page, tokens),
        None => EuvHeader::new(
            EMPTY_PAGE_TITLE.to_string(),
            EMPTY_PAGE_MSG.to_string(),
            tokens,
        )
        .mount_current(),
    };
    EuvAppShell::new(site.get_title().clone(), sidebar_col, main_col, tokens).mount_current()
}

/// Renders a single `DocPage` (header, section badge, TOC, and Markdown blocks) as native nodes.
///
/// # Arguments
///
/// - `&DocPage` - Documentation page to mount.
/// - `ThemeTokens` - Active design tokens.
///
/// # Returns
///
/// - `NodeId` - Mounted page container node.
pub fn mount_doc_page(page: &DocPage, tokens: ThemeTokens) -> NodeId {
    let p: ThemePalette = tokens.get_palette();
    let col: NodeId = view! {
        column {
            width: Length::Percent(100.0)
            gap: SPACE_MD
            { EuvBadge::new(page.get_section().clone(), EuvTone::Primary, tokens) }
            { EuvHeader::new(page.get_title().clone(), page.get_summary().clone(), tokens) }
        }
    };
    let mut ctx: BuildContext = current_build_context();
    if !page.get_toc().is_empty() {
        let mut toc_card: EuvCard = EuvCard::new(TOC_CARD_TITLE.to_string(), tokens);
        for item in page.get_toc().iter() {
            let entry: NodeId = view! {
                text {
                    font_size: 13.0
                    color: p.get_muted_foreground()
                    { item.get_title().clone() }
                }
            };
            toc_card = toc_card.child(entry);
        }
        let mounted_toc: NodeId = toc_card.mount_current();
        ctx.append(col, mounted_toc);
    }
    for block in page.get_blocks().iter() {
        let block_node: NodeId = match block {
            DocBlock::Heading { level, text, .. } => {
                let size: f32 = if *level <= 2 { 20.0 } else { 16.0 };
                view! {
                    text {
                        font_size: size
                        font_weight: FontWeight::Bold
                        color: p.get_foreground()
                        { text.clone() }
                    }
                }
            }
            DocBlock::Paragraph(text) => view! {
                text {
                    font_size: 14.0
                    color: p.get_foreground()
                    { text.clone() }
                }
            },
            DocBlock::ListItem(text) => view! {
                text {
                    font_size: 14.0
                    color: p.get_foreground()
                    { format!("{LIST_BULLET_PREFIX}{text}") }
                }
            },
            DocBlock::BlockQuote(text) => view! {
                column {
                    width: Length::Percent(100.0)
                    padding: Edges::all(SPACE_SM)
                    background: p.get_muted()
                    radius: 8.0
                    text {
                        font_size: 13.5
                        color: p.get_muted_foreground()
                        { text.clone() }
                    }
                }
            },
            DocBlock::Table { headers, rows } => {
                let summary: String = format!("{} ({} rows)", headers.join(" | "), rows.len());
                view! {
                    text {
                        font_size: 13.0
                        color: p.get_muted_foreground()
                        { summary }
                    }
                }
            }
            DocBlock::Image { alt, src } => {
                let caption: String = if alt.is_empty() {
                    src.clone()
                } else {
                    alt.clone()
                };
                view! {
                    text {
                        font_size: 13.0
                        color: p.get_muted_foreground()
                        { caption }
                    }
                }
            }
            DocBlock::HorizontalRule => view! {
                view {
                    width: Length::Percent(100.0)
                    height: Length::Px(1.0)
                    background: p.get_border()
                }
            },
            DocBlock::Code { code, .. } => EuvCodeBlock::new(code.clone(), tokens).mount_current(),
        };
        ctx.append(col, block_node);
    }
    col
}

/// Launches the documentation viewer app and steps one headless or native frame.
///
/// # Arguments
///
/// - `&DocSite` - Loaded documentation site.
/// - `bool` - When `true`, renders headlessly via `HeadlessPlatform`.
///
/// # Returns
///
/// - `Result<FrameStats, PlatformError>` - Frame render telemetry.
pub fn render_docs_smoke(site: &DocSite, headless: bool) -> Result<FrameStats, PlatformError> {
    render_docs_app(site, headless, false)
}

/// Launches the documentation viewer app in either single-frame or interactive event-loop mode.
///
/// # Arguments
///
/// - `&DocSite` - Loaded documentation site.
/// - `bool` - When `true`, renders headlessly via `HeadlessPlatform`.
/// - `bool` - When `true` and not headless, runs the interactive frame loop until window close.
///
/// # Returns
///
/// - `Result<FrameStats, PlatformError>` - Final rendered frame telemetry.
pub fn render_docs_app(
    site: &DocSite,
    headless: bool,
    interactive: bool,
) -> Result<FrameStats, PlatformError> {
    let mut window_cfg: WindowConfig = WindowConfig::default();
    window_cfg
        .set_title(DOCS_WINDOW_TITLE.to_string())
        .set_size(DOCS_WINDOW_WIDTH, DOCS_WINDOW_HEIGHT)
        .set_render(RenderConfig::software());
    let mut app_cfg: AppConfig = AppConfig::default();
    app_cfg.set_window(window_cfg);
    let mut app: App = if headless {
        App::headless(app_cfg)?
    } else {
        App::new(app_cfg)?
    };
    let initial_route: String = site
        .get_pages()
        .first()
        .map(|p: &DocPage| p.get_route().clone())
        .unwrap_or_else(|| "/".to_string());
    let active_route: Signal<String> = Signal::new(initial_route);
    let search_query: Signal<String> = Signal::new(String::new());
    let tokens: ThemeTokens = ThemeTokens::resolve(active_feature_scheme(), ThemeMode::Light);
    let _: NodeId = app.mount(|| mount_docs_viewer(site, active_route, search_query, tokens));
    let limit: Option<u64> = if interactive && !headless {
        None
    } else {
        Some(1)
    };
    app.run_frames(limit)
}
