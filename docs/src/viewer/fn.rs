use super::*;

/// Mounts the cross-platform native documentation viewer for `site` into `ctx`.
///
/// # Arguments
///
/// - `&mut BuildContext` - Active build context.
/// - `&DocSite` - Parsed documentation site.
/// - `Signal<String>` - Active route path signal.
/// - `Signal<String>` - Search query input signal.
/// - `ThemeTokens` - Active design tokens.
///
/// # Returns
///
/// - `NodeId` - Mounted documentation shell node.
pub fn mount_docs_viewer(
    ctx: &mut BuildContext,
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
    let sidebar_col: NodeId = ctx.view(
        Style::empty()
            .column()
            .width(Length::Percent(100.0))
            .gap(SPACE_MD),
    );
    let search_input: NodeId =
        EuvInput::create(search_query, SEARCH_PLACEHOLDER.to_string(), tokens).mount(ctx);
    let nav: NodeId = EuvSidebarNav::create(nav_items, active_route, tokens).mount(ctx);
    ctx.append(sidebar_col, search_input);
    ctx.append(sidebar_col, nav);
    let route: String = active_route.get_untracked();
    let active_page: Option<&DocPage> = site
        .find_by_route(&route)
        .or_else(|| site.get_pages().first());
    let main_col: NodeId = match active_page {
        Some(page) => mount_doc_page(ctx, page, tokens),
        None => EuvHeader::create(
            EMPTY_PAGE_TITLE.to_string(),
            EMPTY_PAGE_MSG.to_string(),
            tokens,
        )
        .mount(ctx),
    };
    EuvAppShell::create(site.get_title().clone(), sidebar_col, main_col, tokens).mount(ctx)
}

/// Renders a single `DocPage` (header, section badge, TOC, and Markdown blocks) as native nodes.
///
/// # Arguments
///
/// - `&mut BuildContext` - Active build context.
/// - `&DocPage` - Documentation page to mount.
/// - `ThemeTokens` - Active design tokens.
///
/// # Returns
///
/// - `NodeId` - Mounted page container node.
pub fn mount_doc_page(ctx: &mut BuildContext, page: &DocPage, tokens: ThemeTokens) -> NodeId {
    let p: ThemePalette = tokens.get_palette();
    let col: NodeId = ctx.view(
        Style::empty()
            .column()
            .width(Length::Percent(100.0))
            .gap(SPACE_MD),
    );
    let badge: NodeId =
        EuvBadge::create(page.get_section().clone(), EuvTone::Primary, tokens).mount(ctx);
    let header: NodeId =
        EuvHeader::create(page.get_title().clone(), page.get_summary().clone(), tokens).mount(ctx);
    ctx.append(col, badge);
    ctx.append(col, header);
    if !page.get_toc().is_empty() {
        let mut toc_card: EuvCard = EuvCard::create(TOC_CARD_TITLE.to_string(), tokens);
        for item in page.get_toc().iter() {
            let entry: NodeId = ctx.text(
                item.get_title().clone(),
                Style::empty()
                    .font_size(13.0)
                    .color(p.get_muted_foreground()),
            );
            toc_card = toc_card.child(entry);
        }
        let mounted_toc: NodeId = toc_card.mount(ctx);
        ctx.append(col, mounted_toc);
    }
    for block in page.get_blocks().iter() {
        let block_node: NodeId = match block {
            DocBlock::Heading { level, text, .. } => {
                let size: f32 = if *level <= 2 { 20.0 } else { 16.0 };
                ctx.text(
                    text.clone(),
                    Style::empty()
                        .font_size(size)
                        .font_weight(FontWeight::Bold)
                        .color(p.get_foreground()),
                )
            }
            DocBlock::Paragraph(text) => ctx.text(
                text.clone(),
                Style::empty().font_size(14.0).color(p.get_foreground()),
            ),
            DocBlock::ListItem(text) => ctx.text(
                format!("{LIST_BULLET_PREFIX}{text}"),
                Style::empty().font_size(14.0).color(p.get_foreground()),
            ),
            DocBlock::Code { code, .. } => EuvCodeBlock::create(code.clone(), tokens).mount(ctx),
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
    let window_cfg: WindowConfig = WindowConfig::default()
        .with_title(DOCS_WINDOW_TITLE.to_string())
        .with_size(DOCS_WINDOW_WIDTH, DOCS_WINDOW_HEIGHT)
        .with_render(RenderConfig::software());
    let app_cfg: AppConfig = AppConfig::default().with_window(window_cfg);
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
    let _: NodeId = app.mount(|ctx: &mut BuildContext| {
        mount_docs_viewer(ctx, site, active_route, search_query, tokens)
    });
    app.step_frame()
}
