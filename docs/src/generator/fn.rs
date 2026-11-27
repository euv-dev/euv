use super::*;

/// Loads all Markdown files beneath `src_dir` (excluding `out_dir`) and builds a complete static docs site.
///
/// # Arguments
///
/// - `&Path` - Source project or Markdown directory.
/// - `&Path` - Output static site directory.
/// - `&DocsBuildOptions` - Site title, base URL, clean URL, asset copy, and optional dev HMR configuration.
///
/// # Returns
///
/// - `io::Result<GeneratedSiteReport>` - Summary of generated artifacts or I/O error.
pub fn build_docs_bundle(
    src_dir: &Path,
    out_dir: &Path,
    options: &DocsBuildOptions,
) -> io::Result<GeneratedSiteReport> {
    let site: DocSite =
        load_doc_site_excluding(src_dir, options.try_get_title().as_deref(), Some(out_dir))?;
    if site.get_pages().is_empty() {
        return Err(Error::new(ErrorKind::NotFound, ERR_NO_MARKDOWN_FOUND));
    }
    let mut resolved: DocsBuildOptions = options.clone();
    if resolved.try_get_assets_root().is_none() {
        resolved.set_assets_root(Some(src_dir.to_path_buf()));
    }
    generate_static_docs_with_options(&site, out_dir, &resolved)
}

/// Generates static HTML pages, search index, and sitemap for `site` into `out_dir` using default options.
///
/// # Arguments
///
/// - `&DocSite` - Loaded documentation site.
/// - `&Path` - Destination output directory.
///
/// # Returns
///
/// - `io::Result<GeneratedSiteReport>` - Summary of written artifacts or I/O error.
pub fn generate_static_docs(site: &DocSite, out_dir: &Path) -> io::Result<GeneratedSiteReport> {
    let opts: DocsBuildOptions = DocsBuildOptions::default();
    generate_static_docs_with_options(site, out_dir, &opts)
}

/// Generates a complete static documentation site with clean URLs, instant search, TOC, asset copying, and optional dev HMR.
///
/// # Arguments
///
/// - `&DocSite` - Loaded documentation site.
/// - `&Path` - Destination output directory.
/// - `&DocsBuildOptions` - Build options controlling base URL, asset copying, clean URLs, and dev HMR.
///
/// # Returns
///
/// - `io::Result<GeneratedSiteReport>` - Summary of written artifacts or I/O error.
pub fn generate_static_docs_with_options(
    site: &DocSite,
    out_dir: &Path,
    options: &DocsBuildOptions,
) -> io::Result<GeneratedSiteReport> {
    fs::create_dir_all(out_dir)?;
    let generated_at: String = Utc::now().to_rfc3339();
    let entries: Vec<SearchIndexEntry> = site.search_entries();
    let search_json_compact: String = match to_string(&entries) {
        Ok(text) => text.replace(SCRIPT_ESC_CLOSE_FROM, SCRIPT_ESC_CLOSE_TO),
        Err(err) => return Err(Error::other(err.to_string())),
    };
    let ordered_pages: Vec<&DocPage> = ordered_site_pages(site);
    for (idx, page) in ordered_pages.iter().enumerate() {
        let prev_page: Option<&DocPage> = if idx > 0 {
            ordered_pages.get(idx - 1).copied()
        } else {
            None
        };
        let next_page: Option<&DocPage> = ordered_pages.get(idx + 1).copied();
        let ctx: PageRenderContext<'_> = PageRenderContext {
            site,
            prev_page,
            next_page,
            generated_at: &generated_at,
            search_json_compact: &search_json_compact,
            options,
        };
        let html: String = render_page_html_with_context(page, &ctx);
        let rel_file: PathBuf = html_path_for_route(out_dir, page.get_route());
        if let Some(parent) = rel_file.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&rel_file, &html)?;
        if options.get_clean_urls() && page.get_route() != ROOT_ROUTE_STR {
            let clean_file: PathBuf = clean_html_path_for_route(out_dir, page.get_route());
            if let Some(parent) = clean_file.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(&clean_file, &html)?;
        }
    }
    if let Some(first_page) = ordered_pages.first().copied()
        && !out_dir.join(INDEX_HTML_NAME).exists()
    {
        let fallback_ctx: PageRenderContext<'_> = PageRenderContext {
            site,
            prev_page: None,
            next_page: ordered_pages.get(1).copied(),
            generated_at: &generated_at,
            search_json_compact: &search_json_compact,
            options,
        };
        let fallback_html: String = render_page_html_with_context(first_page, &fallback_ctx);
        fs::write(out_dir.join(INDEX_HTML_NAME), fallback_html)?;
    }
    let index_path: PathBuf = out_dir.join(INDEX_JSON_NAME);
    let json: String = match to_string_pretty(&entries) {
        Ok(text) => text,
        Err(err) => return Err(Error::other(err.to_string())),
    };
    fs::write(&index_path, json)?;
    let sitemap_xml: String = build_sitemap_xml(&ordered_pages, options.get_base_url());
    fs::write(out_dir.join(SITEMAP_XML_NAME), sitemap_xml)?;
    if let Some(assets_root) = options.try_get_assets_root() {
        copy_static_assets(assets_root, out_dir)?;
    }
    Ok(GeneratedSiteReport::new(
        site.get_pages().len(),
        generated_at,
        index_path.display().to_string(),
    ))
}

/// Renders a single `DocPage` into a self-contained static HTML document using default build options.
///
/// # Arguments
///
/// - `&DocSite` - Parent site for navigation links.
/// - `&DocPage` - Page to render.
/// - `&str` - Build timestamp string.
///
/// # Returns
///
/// - `String` - Complete HTML document string.
#[must_use]
pub fn render_page_html(site: &DocSite, page: &DocPage, generated_at: &str) -> String {
    let entries: Vec<SearchIndexEntry> = site.search_entries();
    let compact: String = to_string(&entries)
        .unwrap_or_else(|_| EMPTY_JSON_ARRAY.to_string())
        .replace(SCRIPT_ESC_CLOSE_FROM, SCRIPT_ESC_CLOSE_TO);
    let opts: DocsBuildOptions = DocsBuildOptions::default();
    let ctx: PageRenderContext<'_> = PageRenderContext {
        site,
        prev_page: None,
        next_page: None,
        generated_at,
        search_json_compact: &compact,
        options: &opts,
    };
    render_page_html_with_context(page, &ctx)
}

/// Returns all site pages in canonical sidebar and pagination order.
///
/// # Arguments
///
/// - `&DocSite` - Loaded documentation site.
///
/// # Returns
///
/// - `Vec<&DocPage>` - Pages ordered by section rank, section label, page order, and route.
fn ordered_site_pages(site: &DocSite) -> Vec<&DocPage> {
    let mut ordered: Vec<&DocPage> = site.get_pages().iter().collect();
    ordered.sort_by(|a: &&DocPage, b: &&DocPage| {
        section_sort_rank(a.get_section())
            .cmp(&section_sort_rank(b.get_section()))
            .then_with(|| a.get_section().cmp(b.get_section()))
            .then_with(|| a.get_order().cmp(&b.get_order()))
            .then_with(|| a.get_route().cmp(b.get_route()))
    });
    ordered
}

/// Renders a full styled HTML page with sidebar sections, TOC, live search index, pagination, and optional HMR.
///
/// # Arguments
///
/// - `&DocPage` - Active page being rendered.
/// - `&PageRenderContext<'_>` - Shared site, pagination, timestamp, search index, and build options context.
///
/// # Returns
///
/// - `String` - Complete HTML document string.
fn render_page_html_with_context(page: &DocPage, ctx: &PageRenderContext<'_>) -> String {
    let site: &DocSite = ctx.get_site();
    let options: &DocsBuildOptions = ctx.get_options();
    let base: &str = options.get_base_url().trim_end_matches('/');
    let home_href: String = href_for_route(base, ROOT_ROUTE_STR);
    let has_toc: bool = !page.get_toc().is_empty();
    let brand_char: char = brand_monogram(site.get_title());
    let mut out: String = String::with_capacity(12288);
    out.push_str(HTML_DOCTYPE);
    out.push_str(&escape_html(page.get_title()));
    out.push_str(HTML_TITLE_SEP);
    out.push_str(&escape_html(site.get_title()));
    out.push_str(HTML_STYLES_AND_HEAD_END);
    out.push_str(HTML_TOPBAR_START);
    out.push_str(&escape_html(&home_href));
    out.push_str(HTML_TOPBAR_MARK_OPEN);
    out.push(brand_char);
    out.push_str(HTML_TOPBAR_MARK_CLOSE);
    out.push_str(&escape_html(site.get_title()));
    out.push_str(HTML_TOPBAR_END);
    if has_toc {
        out.push_str(HTML_SHELL_OPEN);
    } else {
        out.push_str(HTML_SHELL_NO_TOC_OPEN);
    }
    out.push_str(HTML_SIDEBAR_OPEN);
    render_sidebar_nav(site, page, base, &mut out);
    out.push_str(HTML_SIDEBAR_CLOSE);
    out.push_str(HTML_BADGE_OPEN);
    out.push_str(&escape_html(page.get_section()));
    out.push_str(HTML_BADGE_CLOSE);
    out.push_str(&escape_html(page.get_title()));
    out.push_str(HTML_H1_CLOSE);
    if should_render_lead(page) {
        out.push_str(HTML_LEAD_OPEN);
        out.push_str(&escape_html(page.get_summary()));
        out.push_str(HTML_P_CLOSE);
    }
    if !page.get_markdown_body().is_empty() {
        render_markdown_body_html(page, base, &mut out);
    } else {
        render_fallback_blocks_html(page, &mut out);
    }
    render_pager_nav(ctx.get_prev_page(), ctx.get_next_page(), base, &mut out);
    out.push_str(HTML_FOOTER_PREFIX);
    out.push_str(&escape_html(ctx.get_generated_at()));
    out.push_str(HTML_FOOTER_SUFFIX);
    if has_toc {
        out.push_str(HTML_TOC_OPEN);
        for item in page.get_toc().iter() {
            let cls: &str = match item.get_level() {
                3 => TOC_LVL3_CLASS,
                lvl if lvl >= 4 => TOC_LVL4_CLASS,
                _ => "",
            };
            out.push_str(&format!(
                "<a class=\"{cls}\" href=\"#{}\">{}</a>",
                escape_html(item.get_slug()),
                escape_html(item.get_title())
            ));
        }
        out.push_str(HTML_TOC_CLOSE);
    }
    out.push_str(HTML_SHELL_CLOSE);
    let base_json: String = to_string(base).unwrap_or_else(|_| EMPTY_JSON_STRING.to_string());
    out.push_str(HTML_SCRIPT_PREFIX);
    out.push_str(ctx.get_search_json_compact());
    out.push_str(HTML_SCRIPT_BASE_PREFIX);
    out.push_str(&base_json);
    out.push_str(HTML_SCRIPT_BODY);
    if let Some(hmr_url) = options.try_get_hmr_endpoint() {
        out.push_str(HTML_HMR_PREFIX);
        out.push_str(&escape_html(hmr_url));
        out.push_str(HTML_HMR_SUFFIX);
    }
    out.push_str(HTML_BODY_END);
    out
}

/// Renders the grouped section-ordered sidebar links into `out`.
///
/// # Arguments
///
/// - `&DocSite` - Documentation site.
/// - `&DocPage` - Currently active page.
/// - `&str` - Normalized base URL without trailing slash.
/// - `&mut String` - HTML output buffer.
fn render_sidebar_nav(site: &DocSite, active_page: &DocPage, base: &str, out: &mut String) {
    let ordered: Vec<&DocPage> = ordered_site_pages(site);
    let mut current_section: Option<&str> = None;
    for nav_page in ordered.into_iter() {
        let sec: &str = nav_page.get_section().as_str();
        if current_section != Some(sec) {
            current_section = Some(sec);
            out.push_str(HTML_SEC_OPEN);
            out.push_str(&escape_html(sec));
            out.push_str(HTML_SEC_CLOSE);
        }
        let href: String = href_for_route(base, nav_page.get_route());
        if nav_page.get_route() == active_page.get_route() {
            out.push_str(HTML_NAV_LINK_ACTIVE_OPEN);
        } else {
            out.push_str(HTML_NAV_LINK_OPEN);
        }
        out.push_str(&escape_html(&href));
        out.push_str(HTML_LINK_MID);
        out.push_str(&escape_html(nav_page.get_title()));
        out.push_str(HTML_LINK_CLOSE);
    }
}

/// Renders previous and next pagination cards when either neighbor exists.
///
/// # Arguments
///
/// - `Option<&DocPage>` - Previous page in reading order.
/// - `Option<&DocPage>` - Next page in reading order.
/// - `&str` - Normalized base URL without trailing slash.
/// - `&mut String` - HTML output buffer.
fn render_pager_nav(
    prev_page: Option<&DocPage>,
    next_page: Option<&DocPage>,
    base: &str,
    out: &mut String,
) {
    if prev_page.is_none() && next_page.is_none() {
        return;
    }
    out.push_str(HTML_PAGER_OPEN);
    if let Some(prev) = prev_page {
        out.push_str(HTML_PAGER_PREV_OPEN);
        out.push_str(&escape_html(&href_for_route(base, prev.get_route())));
        out.push_str(HTML_PAGER_PREV_LABEL);
        out.push_str(&escape_html(prev.get_title()));
        out.push_str(HTML_PAGER_CARD_CLOSE);
    } else {
        out.push_str(HTML_PAGER_EMPTY);
    }
    if let Some(next) = next_page {
        out.push_str(HTML_PAGER_NEXT_OPEN);
        out.push_str(&escape_html(&href_for_route(base, next.get_route())));
        out.push_str(HTML_PAGER_NEXT_LABEL);
        out.push_str(&escape_html(next.get_title()));
        out.push_str(HTML_PAGER_CARD_CLOSE);
    } else {
        out.push_str(HTML_PAGER_EMPTY);
    }
    out.push_str(HTML_PAGER_CLOSE);
}

/// Returns `true` when `page.summary` is non-empty and does not duplicate the first body paragraph.
///
/// # Arguments
///
/// - `&DocPage` - Candidate documentation page.
///
/// # Returns
///
/// - `bool` - Whether a separate `<p class="lead">` should be rendered.
fn should_render_lead(page: &DocPage) -> bool {
    let summary: &str = page.get_summary().trim();
    if summary.is_empty() {
        return false;
    }
    let first_prose: Option<&str> = page.get_blocks().iter().find_map(|b: &DocBlock| match b {
        DocBlock::Paragraph(p) | DocBlock::BlockQuote(p) => Some(p.trim()),
        _ => None,
    });
    first_prose != Some(summary)
}

/// Computes the uppercase single-character brand monogram for the topbar badge.
///
/// # Arguments
///
/// - `&str` - Site brand title.
///
/// # Returns
///
/// - `char` - Uppercase alphanumeric initial or `'E'`.
fn brand_monogram(title: &str) -> char {
    title
        .chars()
        .find(|ch: &char| ch.is_alphanumeric())
        .and_then(|ch: char| ch.to_uppercase().next())
        .unwrap_or(DEFAULT_BRAND_MONOGRAM)
}

/// Renders full-fidelity Markdown HTML preserving inline formatting, links, images, lists, tables, code blocks, and custom containers.
///
/// # Arguments
///
/// - `&DocPage` - Parsed page carrying raw Markdown body text.
/// - `&str` - Normalized base URL prefix.
/// - `&mut String` - HTML output buffer.
fn render_markdown_body_html(page: &DocPage, base: &str, out: &mut String) {
    let segments: Vec<BodySegment> = split_custom_containers(page.get_markdown_body());
    let mut state: MarkdownRenderState<'_> = MarkdownRenderState {
        page,
        base,
        seen_slugs: BTreeMap::new(),
        skipped_title_h1: false,
        out,
    };
    for seg in segments.into_iter() {
        match seg {
            BodySegment::Markdown(md) => {
                render_markdown_chunk(&md, &mut state);
            }
            BodySegment::Callout { kind, title, body } => {
                if kind == CALLOUT_KIND_DETAILS {
                    state.out.push_str(&format!(
                        "<details class=\"callout callout-details\"><summary>{}</summary>",
                        escape_html(&title)
                    ));
                    render_markdown_chunk(&body, &mut state);
                    state.out.push_str(HTML_DETAILS_CLOSE);
                } else {
                    state.out.push_str(&format!(
                        "<div class=\"callout callout-{}\"><div class=\"callout-title\">{}</div>",
                        escape_html(&kind),
                        escape_html(&title)
                    ));
                    render_markdown_chunk(&body, &mut state);
                    state.out.push_str(HTML_DIV_CLOSE);
                }
            }
        }
    }
}

/// Splits VuePress-style `::: kind [title]` custom containers while ignoring fenced code blocks.
///
/// # Arguments
///
/// - `&str` - Raw Markdown document body.
///
/// # Returns
///
/// - `Vec<BodySegment>` - Ordered plain Markdown and callout segments.
fn split_custom_containers(body: &str) -> Vec<BodySegment> {
    let mut segments: Vec<BodySegment> = Vec::new();
    let mut plain: String = String::new();
    let mut active: Option<(String, String, String)> = None;
    let mut in_fence: bool = false;
    for line in body.lines() {
        let trimmed: &str = line.trim();
        if trimmed.starts_with(FENCE_BACKTICK) || trimmed.starts_with(FENCE_TILDE) {
            in_fence = !in_fence;
        }
        if let Some((kind, title, inner)) = active.as_mut() {
            if !in_fence && trimmed == CALLOUT_FENCE {
                let done_kind: String = take(kind);
                let done_title: String = take(title);
                let done_body: String = take(inner);
                active = None;
                segments.push(BodySegment::Callout {
                    kind: done_kind,
                    title: done_title,
                    body: done_body,
                });
                continue;
            }
            inner.push_str(line);
            inner.push('\n');
            continue;
        }
        if !in_fence && let Some(after) = trimmed.strip_prefix(CALLOUT_FENCE) {
            let spec: &str = after.trim();
            let (raw_kind, custom_title): (&str, &str) = match spec.find(char::is_whitespace) {
                Some(pos) => (&spec[..pos], spec[pos..].trim()),
                None => (spec, ""),
            };
            let normalized: Option<(&str, &str)> = match raw_kind.to_ascii_lowercase().as_str() {
                CALLOUT_KIND_TIP => Some((CALLOUT_KIND_TIP, CALLOUT_TITLE_TIP)),
                CALLOUT_KIND_WARNING => Some((CALLOUT_KIND_WARNING, CALLOUT_TITLE_WARNING)),
                CALLOUT_KIND_DANGER => Some((CALLOUT_KIND_DANGER, CALLOUT_TITLE_DANGER)),
                CALLOUT_KIND_INFO => Some((CALLOUT_KIND_INFO, CALLOUT_TITLE_INFO)),
                CALLOUT_KIND_NOTE => Some((CALLOUT_KIND_NOTE, CALLOUT_TITLE_NOTE)),
                CALLOUT_KIND_DETAILS => Some((CALLOUT_KIND_DETAILS, CALLOUT_TITLE_DETAILS)),
                _ => None,
            };
            if let Some((kind_str, default_title)) = normalized {
                if !plain.is_empty() {
                    segments.push(BodySegment::Markdown(take(&mut plain)));
                }
                let title: String = if custom_title.is_empty() {
                    default_title.to_string()
                } else {
                    custom_title.to_string()
                };
                active = Some((kind_str.to_string(), title, String::new()));
                continue;
            }
        }
        plain.push_str(line);
        plain.push('\n');
    }
    if let Some((kind, title, inner)) = active {
        segments.push(BodySegment::Callout {
            kind,
            title,
            body: inner,
        });
    }
    if !plain.is_empty() {
        segments.push(BodySegment::Markdown(plain));
    }
    segments
}

/// Renders a single Markdown chunk through `pulldown_cmark` into semantic, XSS-safe HTML.
///
/// # Arguments
///
/// - `&str` - Markdown chunk text.
/// - `&mut MarkdownRenderState<'_>` - Mutable page rendering state and HTML output buffer.
fn render_markdown_chunk(chunk: &str, state: &mut MarkdownRenderState<'_>) {
    let parser: Parser<'_> = Parser::new_ext(chunk, Options::all());
    let mut active_heading: Option<(u8, Option<String>, String, String)> = None;
    let mut active_code: Option<(String, String)> = None;
    let mut active_image: Option<(String, String, String)> = None;
    let mut table_alignments: Vec<Alignment> = Vec::new();
    let mut blockquote_alerts: Vec<bool> = Vec::new();
    let mut in_table_head: bool = false;
    let mut cell_index: usize = 0;
    let mut pending_text: String = String::new();
    for event in parser {
        if let Some((_, _, ref mut heading_html, ref mut heading_plain)) = active_heading {
            match event {
                Event::End(TagEnd::Heading(_)) => {
                    if let Some((level, custom_id, inner_html, plain)) = active_heading.take() {
                        let trimmed_plain: &str = plain.trim();
                        if !trimmed_plain.is_empty() {
                            let raw_slug: String = custom_id
                                .map(|s: String| slugify(&s))
                                .filter(|s: &String| !s.is_empty())
                                .unwrap_or_else(|| slugify(trimmed_plain));
                            let slug: String = deduplicate_slug(raw_slug, &mut state.seen_slugs);
                            if level <= 2
                                && !state.skipped_title_h1
                                && state.seen_slugs.len() == 1
                                && trimmed_plain.eq_ignore_ascii_case(state.page.get_title().trim())
                            {
                                state.skipped_title_h1 = true;
                            } else {
                                let tag_lvl: u8 = level.clamp(2, 6);
                                let esc_slug: String = escape_html(&slug);
                                state.out.push_str(&format!(
                                    "<h{tag_lvl} id=\"{esc_slug}\">{inner_html}<a class=\"anchor\" href=\"#{esc_slug}\" aria-label=\"Anchor\">#</a></h{tag_lvl}>"
                                ));
                            }
                        }
                    }
                }
                Event::Text(text) => {
                    heading_plain.push_str(&text);
                    heading_html.push_str(&escape_html(&text));
                }
                Event::Code(text) => {
                    heading_plain.push_str(&text);
                    heading_html.push_str(HTML_INLINE_CODE_OPEN);
                    heading_html.push_str(&escape_html(&text));
                    heading_html.push_str(HTML_INLINE_CODE_CLOSE);
                }
                Event::Start(Tag::Strong) => heading_html.push_str(HTML_STRONG_OPEN),
                Event::End(TagEnd::Strong) => heading_html.push_str(HTML_STRONG_CLOSE),
                Event::Start(Tag::Emphasis) => heading_html.push_str(HTML_EM_OPEN),
                Event::End(TagEnd::Emphasis) => heading_html.push_str(HTML_EM_CLOSE),
                Event::Start(Tag::Strikethrough) => heading_html.push_str(HTML_DEL_OPEN),
                Event::End(TagEnd::Strikethrough) => heading_html.push_str(HTML_DEL_CLOSE),
                Event::SoftBreak | Event::HardBreak => {
                    heading_plain.push(' ');
                    heading_html.push(' ');
                }
                _ => {}
            }
            continue;
        }
        if let Some((_, ref mut code_buf)) = active_code {
            match event {
                Event::End(TagEnd::CodeBlock) => {
                    if let Some((lang, code)) = active_code.take() {
                        let trimmed: &str = code.trim_end_matches('\n');
                        if lang.eq_ignore_ascii_case(CODE_LANG_MERMAID) {
                            render_mermaid_svg_html(trimmed, state.out);
                        } else {
                            render_highlighted_code_html(&lang, trimmed, state.out);
                        }
                    }
                }
                Event::Text(text) | Event::Code(text) => code_buf.push_str(&text),
                Event::SoftBreak | Event::HardBreak => code_buf.push('\n'),
                _ => {}
            }
            continue;
        }
        if let Some((_, _, ref mut alt_buf)) = active_image {
            match event {
                Event::End(TagEnd::Image) => {
                    if let Some((src, title, alt)) = active_image.take()
                        && let Some(resolved) =
                            resolve_asset_src(state.base, state.page.get_route(), &src)
                    {
                        if title.is_empty() {
                            state.out.push_str(&format!(
                                "<img src=\"{}\" alt=\"{}\" loading=\"lazy\">",
                                escape_html(&resolved),
                                escape_html(alt.trim())
                            ));
                        } else {
                            state.out.push_str(&format!(
                                "<img src=\"{}\" alt=\"{}\" title=\"{}\" loading=\"lazy\">",
                                escape_html(&resolved),
                                escape_html(alt.trim()),
                                escape_html(&title)
                            ));
                        }
                    }
                }
                Event::Text(text) | Event::Code(text) => alt_buf.push_str(&text),
                Event::SoftBreak | Event::HardBreak => alt_buf.push(' '),
                _ => {}
            }
            continue;
        }
        if let Event::Text(text) = event {
            pending_text.push_str(&text);
            continue;
        }
        if !pending_text.is_empty() {
            render_extended_inline_text_html(&pending_text, state.out);
            pending_text.clear();
        }
        match event {
            Event::Start(Tag::Heading { level, id, .. }) => {
                active_heading = Some((
                    heading_level_num(level),
                    id.map(CowStr::into_string),
                    String::new(),
                    String::new(),
                ));
            }
            Event::Start(Tag::Paragraph) => state.out.push_str(HTML_P_OPEN),
            Event::End(TagEnd::Paragraph) => state.out.push_str(HTML_P_CLOSE),
            Event::Start(Tag::BlockQuote(kind_opt)) => {
                if let Some(kind) = kind_opt {
                    let (cls, title): (&str, &str) = match kind {
                        BlockQuoteKind::Note => (CALLOUT_KIND_NOTE, CALLOUT_TITLE_NOTE),
                        BlockQuoteKind::Tip => (CALLOUT_KIND_TIP, CALLOUT_TITLE_TIP),
                        BlockQuoteKind::Important => {
                            (CALLOUT_KIND_IMPORTANT, CALLOUT_TITLE_IMPORTANT)
                        }
                        BlockQuoteKind::Warning => (CALLOUT_KIND_WARNING, CALLOUT_TITLE_WARNING),
                        BlockQuoteKind::Caution => (CALLOUT_KIND_CAUTION, CALLOUT_TITLE_CAUTION),
                    };
                    state.out.push_str(&format!(
                        "<div class=\"callout callout-{cls}\"><div class=\"callout-title\">{title}</div>"
                    ));
                    blockquote_alerts.push(true);
                } else {
                    state.out.push_str(HTML_QUOTE_OPEN);
                    blockquote_alerts.push(false);
                }
            }
            Event::End(TagEnd::BlockQuote(_)) => {
                if blockquote_alerts.pop().unwrap_or(false) {
                    state.out.push_str(HTML_DIV_CLOSE);
                } else {
                    state.out.push_str(HTML_QUOTE_CLOSE);
                }
            }
            Event::Start(Tag::List(None)) => state.out.push_str(HTML_UL_OPEN),
            Event::Start(Tag::List(Some(start))) => {
                if start == 1 {
                    state.out.push_str(HTML_OL_OPEN);
                } else {
                    state.out.push_str(&format!("<ol start=\"{start}\">"));
                }
            }
            Event::End(TagEnd::List(ordered)) => {
                if ordered {
                    state.out.push_str(HTML_OL_CLOSE);
                } else {
                    state.out.push_str(HTML_UL_CLOSE);
                }
            }
            Event::Start(Tag::Item) => state.out.push_str(HTML_LI_OPEN),
            Event::End(TagEnd::Item) => state.out.push_str(HTML_LI_CLOSE),
            Event::Start(Tag::Strong) => state.out.push_str(HTML_STRONG_OPEN),
            Event::End(TagEnd::Strong) => state.out.push_str(HTML_STRONG_CLOSE),
            Event::Start(Tag::Emphasis) => state.out.push_str(HTML_EM_OPEN),
            Event::End(TagEnd::Emphasis) => state.out.push_str(HTML_EM_CLOSE),
            Event::Start(Tag::Strikethrough) => state.out.push_str(HTML_DEL_OPEN),
            Event::End(TagEnd::Strikethrough) => state.out.push_str(HTML_DEL_CLOSE),
            Event::Start(Tag::Superscript) => state.out.push_str(HTML_SUP_OPEN),
            Event::End(TagEnd::Superscript) => state.out.push_str(HTML_SUP_CLOSE),
            Event::Start(Tag::Subscript) => state.out.push_str(HTML_SUB_OPEN),
            Event::End(TagEnd::Subscript) => state.out.push_str(HTML_SUB_CLOSE),
            Event::Start(Tag::DefinitionList) => state.out.push_str(HTML_DL_OPEN),
            Event::End(TagEnd::DefinitionList) => state.out.push_str(HTML_DL_CLOSE),
            Event::Start(Tag::DefinitionListTitle) => state.out.push_str(HTML_DT_OPEN),
            Event::End(TagEnd::DefinitionListTitle) => state.out.push_str(HTML_DT_CLOSE),
            Event::Start(Tag::DefinitionListDefinition) => state.out.push_str(HTML_DD_OPEN),
            Event::End(TagEnd::DefinitionListDefinition) => state.out.push_str(HTML_DD_CLOSE),
            Event::Start(Tag::CodeBlock(kind)) => {
                let lang: String = match kind {
                    CodeBlockKind::Fenced(tag) => {
                        let first: &str = tag.split_whitespace().next().unwrap_or("");
                        if first.is_empty() {
                            DEFAULT_CODE_LABEL.to_string()
                        } else {
                            first.to_string()
                        }
                    }
                    CodeBlockKind::Indented => DEFAULT_CODE_LABEL.to_string(),
                };
                active_code = Some((lang, String::new()));
            }
            Event::Start(Tag::Link {
                dest_url, title, ..
            }) => {
                let resolved: String =
                    resolve_link_href(state.base, state.page.get_route(), &dest_url);
                let external: bool = is_external_url(&resolved);
                state.out.push_str(HTML_A_HREF_OPEN);
                state.out.push_str(&escape_html(&resolved));
                state.out.push('"');
                if !title.is_empty() {
                    state.out.push_str(HTML_TITLE_ATTR_OPEN);
                    state.out.push_str(&escape_html(&title));
                    state.out.push('"');
                }
                if external {
                    state.out.push_str(HTML_EXTERNAL_LINK_ATTRS);
                }
                state.out.push('>');
            }
            Event::End(TagEnd::Link) => state.out.push_str(HTML_LINK_CLOSE),
            Event::Start(Tag::Image {
                dest_url, title, ..
            }) => {
                active_image = Some((dest_url.into_string(), title.into_string(), String::new()));
            }
            Event::Start(Tag::Table(aligns)) => {
                table_alignments = aligns;
                in_table_head = false;
                state.out.push_str(HTML_TABLE_OPEN);
            }
            Event::End(TagEnd::Table) => {
                table_alignments.clear();
                state.out.push_str(HTML_TBODY_CLOSE);
                state.out.push_str(HTML_TABLE_CLOSE);
            }
            Event::Start(Tag::TableHead) => {
                in_table_head = true;
                cell_index = 0;
                state.out.push_str(HTML_THEAD_OPEN);
            }
            Event::End(TagEnd::TableHead) => {
                in_table_head = false;
                state.out.push_str(HTML_THEAD_CLOSE);
                state.out.push_str(HTML_TBODY_OPEN);
            }
            Event::Start(Tag::TableRow) => {
                cell_index = 0;
                state.out.push_str(HTML_TR_OPEN);
            }
            Event::End(TagEnd::TableRow) => state.out.push_str(HTML_TR_CLOSE),
            Event::Start(Tag::TableCell) => {
                let align_attr: &str = match table_alignments.get(cell_index).copied() {
                    Some(Alignment::Center) => HTML_ALIGN_CENTER,
                    Some(Alignment::Right) => HTML_ALIGN_RIGHT,
                    _ => "",
                };
                cell_index += 1;
                if in_table_head {
                    state.out.push_str(&format!("<th{align_attr}>"));
                } else {
                    state.out.push_str(&format!("<td{align_attr}>"));
                }
            }
            Event::End(TagEnd::TableCell) => {
                if in_table_head {
                    state.out.push_str(HTML_TH_CLOSE);
                } else {
                    state.out.push_str(HTML_TD_CLOSE);
                }
            }
            Event::Start(Tag::FootnoteDefinition(label)) => {
                state.out.push_str(&format!(
                    "<blockquote class=\"footnote\" id=\"fn-{}\"><strong>[^{}]</strong> ",
                    escape_html(&slugify(&label)),
                    escape_html(&label)
                ));
            }
            Event::End(TagEnd::FootnoteDefinition) => state.out.push_str(HTML_QUOTE_CLOSE),
            Event::FootnoteReference(label) => {
                state.out.push_str(&format!(
                    "<sup><a href=\"#fn-{}\">[^{}]</a></sup>",
                    escape_html(&slugify(&label)),
                    escape_html(&label)
                ));
            }
            Event::TaskListMarker(checked) => {
                if checked {
                    state.out.push_str(HTML_TASK_CHECKED);
                } else {
                    state.out.push_str(HTML_TASK_UNCHECKED);
                }
            }
            Event::Rule => state.out.push_str(HTML_HR),
            Event::Code(text) => {
                state.out.push_str(HTML_INLINE_CODE_OPEN);
                state.out.push_str(&escape_html(&text));
                state.out.push_str(HTML_INLINE_CODE_CLOSE);
            }
            Event::SoftBreak => state.out.push(' '),
            Event::HardBreak => state.out.push_str(HTML_BR),
            Event::InlineMath(math) => {
                state.out.push_str(HTML_MATH_INLINE_OPEN);
                state.out.push_str(&escape_html(&format_latex_math(&math)));
                state.out.push_str(HTML_MATH_INLINE_CLOSE);
            }
            Event::DisplayMath(math) => {
                state.out.push_str(HTML_MATH_DISPLAY_OPEN);
                state.out.push_str(&escape_html(&format_latex_math(&math)));
                state.out.push_str(HTML_MATH_DISPLAY_CLOSE);
            }
            Event::Html(raw_html) | Event::InlineHtml(raw_html) => {
                if is_safe_raw_html(&raw_html) {
                    state.out.push_str(&raw_html);
                } else {
                    state.out.push_str(&escape_html(&raw_html));
                }
            }
            _ => {}
        }
    }
    if !pending_text.is_empty() {
        render_extended_inline_text_html(&pending_text, state.out);
    }
}

/// Renders structured `DocBlock` entries when a `DocPage` was constructed without raw Markdown text.
///
/// # Arguments
///
/// - `&DocPage` - Documentation page containing structured blocks.
/// - `&mut String` - HTML output buffer.
fn render_fallback_blocks_html(page: &DocPage, out: &mut String) {
    let mut in_ul: bool = false;
    for block in page.get_blocks().iter() {
        let is_item: bool = matches!(block, DocBlock::ListItem(_));
        if is_item && !in_ul {
            out.push_str(HTML_UL_OPEN);
            in_ul = true;
        } else if !is_item && in_ul {
            out.push_str(HTML_UL_CLOSE);
            in_ul = false;
        }
        match block {
            DocBlock::Heading { level, slug, text } => {
                let tag_lvl: u8 = (*level).clamp(2, 6);
                out.push_str(&format!(
                    "<h{tag_lvl} id=\"{}\">{}</h{tag_lvl}>",
                    escape_html(slug),
                    escape_html(text)
                ));
            }
            DocBlock::Paragraph(text) => {
                out.push_str(HTML_P_OPEN);
                out.push_str(&escape_html(text));
                out.push_str(HTML_P_CLOSE);
            }
            DocBlock::ListItem(text) => {
                out.push_str(HTML_LI_OPEN);
                out.push_str(&escape_html(text));
                out.push_str(HTML_LI_CLOSE);
            }
            DocBlock::BlockQuote(text) => {
                out.push_str(HTML_QUOTE_OPEN);
                out.push_str(&escape_html(text));
                out.push_str(HTML_QUOTE_CLOSE);
            }
            DocBlock::Image { alt, src } => {
                out.push_str(&format!(
                    "<p><img src=\"{}\" alt=\"{}\" loading=\"lazy\"></p>",
                    escape_html(src),
                    escape_html(alt)
                ));
            }
            DocBlock::HorizontalRule => out.push_str(HTML_HR),
            DocBlock::Code { language, code } => {
                render_highlighted_code_html(language, code, out);
            }
            DocBlock::Table { headers, rows } => {
                out.push_str(HTML_TABLE_OPEN);
                if !headers.is_empty() {
                    out.push_str(HTML_THEAD_OPEN);
                    for h in headers.iter() {
                        out.push_str(HTML_TH_OPEN);
                        out.push_str(&escape_html(h));
                        out.push_str(HTML_TH_CLOSE);
                    }
                    out.push_str(HTML_THEAD_CLOSE);
                }
                out.push_str(HTML_TBODY_OPEN);
                for row in rows.iter() {
                    out.push_str(HTML_TR_OPEN);
                    for cell in row.iter() {
                        out.push_str(HTML_TD_OPEN);
                        out.push_str(&escape_html(cell));
                        out.push_str(HTML_TD_CLOSE);
                    }
                    out.push_str(HTML_TR_CLOSE);
                }
                out.push_str(HTML_TBODY_CLOSE);
                out.push_str(HTML_TABLE_CLOSE);
            }
        }
    }
    if in_ul {
        out.push_str(HTML_UL_CLOSE);
    }
}

/// Checks whether a raw Markdown HTML fragment is free of active script or event-handler vectors.
///
/// # Arguments
///
/// - `&str` - Raw HTML fragment emitted by `pulldown_cmark`.
///
/// # Returns
///
/// - `bool` - `true` when safe to pass through verbatim.
fn is_safe_raw_html(raw: &str) -> bool {
    let low: String = raw.to_ascii_lowercase();
    if UNSAFE_HTML_TAGS.iter().any(|tag: &&str| low.contains(tag)) {
        return false;
    }
    if low.contains(SCHEME_JAVASCRIPT) || low.contains(SCHEME_VBSCRIPT) {
        return false;
    }
    let bytes: &[u8] = low.as_bytes();
    for i in 0..bytes.len().saturating_sub(3) {
        let before_ok: bool = i == 0 || bytes[i - 1].is_ascii_whitespace();
        if before_ok
            && bytes[i] == BYTE_LOWER_O
            && bytes[i + 1] == BYTE_LOWER_N
            && bytes[i + 2].is_ascii_alphabetic()
        {
            let rest: &str = &low[i + 2..];
            let attr_len: usize = rest
                .bytes()
                .take_while(|b: &u8| b.is_ascii_alphabetic())
                .count();
            if rest[attr_len..].trim_start().starts_with('=') {
                return false;
            }
        }
    }
    true
}

/// Resolves Markdown link targets, rewriting relative `.md` links to clean URLs and blocking unsafe schemes.
///
/// # Arguments
///
/// - `&str` - Normalized base URL prefix.
/// - `&str` - Current page canonical route.
/// - `&str` - Raw Markdown link destination.
///
/// # Returns
///
/// - `String` - Safe navigable HTML `href` attribute value.
fn resolve_link_href(base: &str, current_route: &str, raw_dest: &str) -> String {
    let dest: &str = raw_dest.trim();
    let low: String = dest.to_ascii_lowercase();
    if low.starts_with(SCHEME_JAVASCRIPT) || low.starts_with(SCHEME_VBSCRIPT) {
        return "#".to_string();
    }
    if dest.starts_with('#')
        || low.starts_with(SCHEME_HTTP)
        || low.starts_with(SCHEME_HTTPS)
        || low.starts_with(SCHEME_MAILTO)
    {
        return dest.to_string();
    }
    let (path_part, fragment): (&str, Option<&str>) = match dest.split_once('#') {
        Some((p, f)) => (p, Some(f)),
        None => (dest, None),
    };
    let path_low: String = path_part.to_ascii_lowercase();
    if path_low.ends_with(MARKDOWN_EXT_SUFFIX) {
        let joined: String = join_relative_route(current_route, path_part);
        let stripped: &str = joined
            .strip_suffix(MARKDOWN_EXT_SUFFIX)
            .or_else(|| joined.strip_suffix(MARKDOWN_EXT_UPPER))
            .unwrap_or(&joined);
        let mut segs: Vec<String> = stripped
            .split('/')
            .filter(|s: &&str| !s.is_empty())
            .map(|seg: &str| {
                let cleaned: String = slugify(seg);
                if cleaned.is_empty() {
                    seg.to_ascii_lowercase()
                } else {
                    cleaned
                }
            })
            .collect();
        if let Some(last) = segs.last()
            && (last.eq_ignore_ascii_case(STEM_README_LOWER)
                || last.eq_ignore_ascii_case(STEM_INDEX_LOWER))
        {
            let _: Option<String> = segs.pop();
        }
        let route: String = if segs.is_empty() {
            ROOT_ROUTE_STR.to_string()
        } else {
            format!("/{}", segs.join("/"))
        };
        let clean: String = href_for_route(base, &route);
        return match fragment {
            Some(frag) if !frag.is_empty() => format!("{clean}#{frag}"),
            _ => clean,
        };
    }
    if dest.starts_with('/') {
        if base.is_empty() {
            dest.to_string()
        } else {
            format!("{base}{dest}")
        }
    } else {
        dest.to_string()
    }
}

/// Resolves Markdown image `src` attributes against `current_route` and `base`, blocking unsafe schemes.
///
/// # Arguments
///
/// - `&str` - Normalized base URL prefix.
/// - `&str` - Current page canonical route.
/// - `&str` - Raw Markdown image destination.
///
/// # Returns
///
/// - `Option<String>` - Safe `src` attribute value or `None` when blocked.
fn resolve_asset_src(base: &str, current_route: &str, raw_src: &str) -> Option<String> {
    let src: &str = raw_src.trim();
    let low: String = src.to_ascii_lowercase();
    if src.is_empty() || low.starts_with(SCHEME_JAVASCRIPT) || low.starts_with(SCHEME_VBSCRIPT) {
        return None;
    }
    if low.starts_with(SCHEME_HTTP) || low.starts_with(SCHEME_HTTPS) || low.starts_with(SCHEME_DATA)
    {
        return Some(src.to_string());
    }
    if src.starts_with('/') {
        return Some(if base.is_empty() {
            src.to_string()
        } else {
            format!("{base}{src}")
        });
    }
    let resolved: String = join_relative_route(current_route, src);
    Some(if base.is_empty() {
        resolved
    } else {
        format!("{base}{resolved}")
    })
}

/// Resolves a relative link or asset path against a page's canonical directory route.
///
/// # Arguments
///
/// - `&str` - Canonical page route (for example `/guide/markdown`).
/// - `&str` - Relative path reference (for example `./getting-started.md` or `../img/logo.png`).
///
/// # Returns
///
/// - `String` - Normalized root-relative path beginning with `/`.
fn join_relative_route(current_route: &str, rel_path: &str) -> String {
    let mut stack: Vec<String> = current_route
        .trim_matches('/')
        .split('/')
        .filter(|s: &&str| !s.is_empty())
        .map(str::to_string)
        .collect();
    if !stack.is_empty() {
        let _: Option<String> = stack.pop();
    }
    for seg in rel_path.split('/') {
        if seg.is_empty() || seg == "." {
            continue;
        }
        if seg == ".." {
            let _: Option<String> = stack.pop();
        } else {
            stack.push(seg.to_string());
        }
    }
    if stack.is_empty() {
        ROOT_ROUTE_STR.to_string()
    } else {
        format!("/{}", stack.join("/"))
    }
}

/// Returns `true` when `href` points to an external HTTP(S) origin.
///
/// # Arguments
///
/// - `&str` - Resolved href string.
///
/// # Returns
///
/// - `bool` - Whether the link should open in a new tab.
fn is_external_url(href: &str) -> bool {
    let low: String = href.to_ascii_lowercase();
    low.starts_with(SCHEME_HTTP) || low.starts_with(SCHEME_HTTPS)
}

/// Builds a standard XML sitemap containing every generated page route in canonical order.
///
/// # Arguments
///
/// - `&[&DocPage]` - Ordered site pages.
/// - `&str` - Optional base URL prefix.
///
/// # Returns
///
/// - `String` - Complete `sitemap.xml` document.
fn build_sitemap_xml(pages: &[&DocPage], base_url: &str) -> String {
    let base: &str = base_url.trim_end_matches('/');
    let mut xml: String = String::from(SITEMAP_HEADER);
    for page in pages.iter() {
        let loc: String = href_for_route(base, page.get_route());
        xml.push_str(SITEMAP_URL_OPEN);
        xml.push_str(&escape_html(&loc));
        xml.push_str(SITEMAP_URL_CLOSE);
    }
    xml.push_str(SITEMAP_FOOTER);
    xml
}

/// Copies recognized static asset directories and co-located media assets into `out_dir`.
///
/// # Arguments
///
/// - `&Path` - Source project or docs root directory.
/// - `&Path` - Output site directory.
///
/// # Returns
///
/// - `io::Result<()>` - Copy result.
fn copy_static_assets(src_root: &Path, out_dir: &Path) -> io::Result<()> {
    let canonical_out: PathBuf =
        fs::canonicalize(out_dir).unwrap_or_else(|_| out_dir.to_path_buf());
    for dir_name in ASSET_COPY_DIRS.iter() {
        let candidate: PathBuf = src_root.join(dir_name);
        if candidate.is_dir() {
            copy_dir_recursive(&candidate, &out_dir.join(dir_name), &canonical_out)?;
        }
    }
    copy_colocated_media(src_root, src_root, out_dir, &canonical_out)
}

/// Recursively copies non-Markdown static asset files from `src` into `dst`.
///
/// # Arguments
///
/// - `&Path` - Source directory.
/// - `&Path` - Destination directory.
/// - `&Path` - Canonical output root skipped to avoid self-recursion.
///
/// # Returns
///
/// - `io::Result<()>` - Copy result.
fn copy_dir_recursive(src: &Path, dst: &Path, canonical_out: &Path) -> io::Result<()> {
    let src_canon: PathBuf = fs::canonicalize(src).unwrap_or_else(|_| src.to_path_buf());
    if src_canon == canonical_out || src_canon.starts_with(canonical_out) {
        return Ok(());
    }
    fs::create_dir_all(dst)?;
    for entry_res in fs::read_dir(src)? {
        let entry: DirEntry = entry_res?;
        let path: PathBuf = entry.path();
        let target: PathBuf = dst.join(entry.file_name());
        if path.is_dir() {
            copy_dir_recursive(&path, &target, canonical_out)?;
        } else if path
            .extension()
            .and_then(|s: &OsStr| s.to_str())
            .is_none_or(|ext: &str| !ext.eq_ignore_ascii_case(MARKDOWN_EXT_CHECK))
        {
            let _: u64 = fs::copy(&path, &target)?;
        }
    }
    Ok(())
}

/// Recursively copies co-located media files alongside Markdown documents while skipping ignored dirs and `out_dir`.
///
/// # Arguments
///
/// - `&Path` - Original source root directory.
/// - `&Path` - Current directory being walked.
/// - `&Path` - Output site directory.
/// - `&Path` - Canonical output root skipped to avoid self-recursion.
///
/// # Returns
///
/// - `io::Result<()>` - Copy result.
fn copy_colocated_media(
    src_root: &Path,
    cur_dir: &Path,
    out_dir: &Path,
    canonical_out: &Path,
) -> io::Result<()> {
    for entry_res in fs::read_dir(cur_dir)? {
        let entry: DirEntry = entry_res?;
        let path: PathBuf = entry.path();
        if path.is_dir() {
            let name: String = entry.file_name().to_string_lossy().into_owned();
            if should_skip_dir(&name) {
                continue;
            }
            if cur_dir == src_root
                && ASSET_COPY_DIRS
                    .iter()
                    .any(|d: &&str| name.eq_ignore_ascii_case(d))
            {
                continue;
            }
            let canon: PathBuf = fs::canonicalize(&path).unwrap_or_else(|_| path.clone());
            if canon == canonical_out || canon.starts_with(canonical_out) {
                continue;
            }
            copy_colocated_media(src_root, &path, out_dir, canonical_out)?;
        } else if path
            .extension()
            .and_then(|s: &OsStr| s.to_str())
            .is_some_and(|ext: &str| {
                CO_LOCATED_ASSET_EXTS
                    .iter()
                    .any(|allowed: &&str| ext.eq_ignore_ascii_case(allowed))
            })
            && let Ok(rel) = path.strip_prefix(src_root)
        {
            let target: PathBuf = out_dir.join(rel);
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }
            let _: u64 = fs::copy(&path, &target)?;
        }
    }
    Ok(())
}

/// Formats a clean link href for `route` under `base`.
///
/// # Arguments
///
/// - `&str` - Normalized base URL without trailing slash.
/// - `&str` - Canonical route path.
///
/// # Returns
///
/// - `String` - Clean navigable href string.
fn href_for_route(base: &str, route: &str) -> String {
    let trimmed: &str = route.trim_matches('/');
    if trimmed.is_empty() {
        if base.is_empty() {
            ROOT_ROUTE_STR.to_string()
        } else {
            format!("{base}/")
        }
    } else if base.is_empty() {
        format!("/{trimmed}/")
    } else {
        format!("{base}/{trimmed}/")
    }
}

/// Computes the filesystem output `.html` path for a canonical documentation route.
///
/// # Arguments
///
/// - `&Path` - Output root directory.
/// - `&str` - Canonical route path.
///
/// # Returns
///
/// - `PathBuf` - Destination `.html` file path.
fn html_path_for_route(out_dir: &Path, route: &str) -> PathBuf {
    let trimmed: &str = route.trim_matches('/');
    if trimmed.is_empty() {
        out_dir.join(INDEX_HTML_NAME)
    } else {
        out_dir.join(format!("{trimmed}{HTML_EXT}"))
    }
}

/// Computes the clean directory-index `<route>/index.html` path for a canonical route.
///
/// # Arguments
///
/// - `&Path` - Output root directory.
/// - `&str` - Canonical route path.
///
/// # Returns
///
/// - `PathBuf` - Destination `<route>/index.html` path.
fn clean_html_path_for_route(out_dir: &Path, route: &str) -> PathBuf {
    let trimmed: &str = route.trim_matches('/');
    if trimmed.is_empty() {
        out_dir.join(INDEX_HTML_NAME)
    } else {
        out_dir.join(trimmed).join(INDEX_HTML_NAME)
    }
}

/// Renders a fenced code block with line numbers and exact VS Code Eva Light/Dark Bold syntax classes.
///
/// # Arguments
///
/// - `&str` - Language label from the fenced code info string.
/// - `&str` - Trimmed source code body.
/// - `&mut String` - HTML output buffer.
fn render_highlighted_code_html(language: &str, code: &str, out: &mut String) {
    out.push_str(HTML_CODE_WRAP_OPEN);
    out.push_str(&escape_html(language));
    out.push_str(HTML_CODE_HEAD_MID);
    out.push_str(&escape_html(language));
    out.push_str(HTML_LINK_MID);
    for (idx, line) in code.split('\n').enumerate() {
        out.push_str(HTML_CODE_LINE_OPEN);
        out.push_str(&(idx + 1).to_string());
        out.push_str(HTML_CODE_LINE_MID);
        let spans: Vec<CodeSpan> = tokenize_code_line(line, language);
        for span in spans.iter() {
            let class_opt: Option<&str> = match span.get_kind() {
                CodeTokenKind::Keyword => Some(TOK_CLASS_KW),
                CodeTokenKind::Control => Some(TOK_CLASS_CTRL),
                CodeTokenKind::Operator => Some(TOK_CLASS_OP),
                CodeTokenKind::Function => Some(TOK_CLASS_FN),
                CodeTokenKind::Type => Some(TOK_CLASS_TY),
                CodeTokenKind::PrimitiveType => Some(TOK_CLASS_PRIM),
                CodeTokenKind::String => Some(TOK_CLASS_STR),
                CodeTokenKind::Number => Some(TOK_CLASS_NUM),
                CodeTokenKind::Comment => Some(TOK_CLASS_CMT),
                CodeTokenKind::Macro => Some(TOK_CLASS_MAC),
                CodeTokenKind::Property => Some(TOK_CLASS_PROP),
                CodeTokenKind::Constant => Some(TOK_CLASS_CONST),
                CodeTokenKind::Punctuation => Some(TOK_CLASS_PUNC),
                CodeTokenKind::DiffAdd => Some(TOK_CLASS_ADD),
                CodeTokenKind::DiffDel => Some(TOK_CLASS_DEL),
                CodeTokenKind::Plain => None,
            };
            if let Some(cls) = class_opt {
                out.push_str(&format!(
                    "<span class=\"{cls}\">{}",
                    escape_html(span.get_text())
                ));
                out.push_str(HTML_SPAN_CLOSE);
            } else {
                out.push_str(&escape_html(span.get_text()));
            }
        }
        out.push_str(HTML_CODE_LINE_CLOSE);
    }
    out.push_str(HTML_CODE_CLOSE);
}

/// Escapes special HTML characters in text content.
///
/// # Arguments
///
/// - `&str` - Raw UTF-8 text.
///
/// # Returns
///
/// - `String` - Escaped HTML-safe string.
fn escape_html(input: &str) -> String {
    let mut out: String = String::with_capacity(input.len());
    for ch in input.chars() {
        push_escaped_html_char(ch, &mut out);
    }
    out
}

/// Pushes a single HTML-escaped character into `out`.
///
/// # Arguments
///
/// - `char` - Source UTF-8 character.
/// - `&mut String` - Destination HTML buffer.
fn push_escaped_html_char(ch: char, out: &mut String) {
    match ch {
        '&' => out.push_str(HTML_ESC_AMP),
        '<' => out.push_str(HTML_ESC_LT),
        '>' => out.push_str(HTML_ESC_GT),
        '"' => out.push_str(HTML_ESC_QUOT),
        _ => out.push(ch),
    }
}

/// Renders extended inline Markdown syntax (`==mark==`, `^sup^`, `~sub~`, `:emoji:`) into XSS-safe HTML.
///
/// # Arguments
///
/// - `&str` - Raw inline Markdown text slice.
/// - `&mut String` - Destination HTML buffer.
fn render_extended_inline_text_html(raw: &str, out: &mut String) {
    let bytes: &[u8] = raw.as_bytes();
    let len: usize = bytes.len();
    let mut i: usize = 0;
    while i < len {
        if i + 1 < len && bytes[i] == b'=' && bytes[i + 1] == b'=' {
            let rest: &str = &raw[i + 2..];
            if let Some(close_rel) = rest.find("==")
                && !rest[..close_rel].trim().is_empty()
            {
                out.push_str(HTML_MARK_OPEN);
                render_extended_inline_text_html(&rest[..close_rel], out);
                out.push_str(HTML_MARK_CLOSE);
                i += 2 + close_rel + 2;
                continue;
            }
        }
        if bytes[i] == b'^' {
            let rest: &str = &raw[i + 1..];
            if let Some(close_rel) = rest.find('^') {
                let inner: &str = &rest[..close_rel];
                if !inner.is_empty() && !inner.chars().any(char::is_whitespace) {
                    out.push_str(HTML_SUP_OPEN);
                    out.push_str(&escape_html(inner));
                    out.push_str(HTML_SUP_CLOSE);
                    i += 1 + close_rel + 1;
                    continue;
                }
            }
        }
        if bytes[i] == b'~' && !(i + 1 < len && bytes[i + 1] == b'~') {
            let rest: &str = &raw[i + 1..];
            if let Some(close_rel) = rest.find('~') {
                let inner: &str = &rest[..close_rel];
                if !inner.is_empty() && !inner.chars().any(char::is_whitespace) {
                    out.push_str(HTML_SUB_OPEN);
                    out.push_str(&escape_html(inner));
                    out.push_str(HTML_SUB_CLOSE);
                    i += 1 + close_rel + 1;
                    continue;
                }
            }
        }
        if bytes[i] == b':' {
            let rest: &str = &raw[i + 1..];
            if let Some(close_rel) = rest.find(':') {
                let code: &str = &rest[..close_rel];
                if !code.is_empty()
                    && code.len() <= 32
                    && code
                        .bytes()
                        .all(|b: u8| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
                    && let Some(emoji) = resolve_emoji_shortcode(code)
                {
                    out.push_str(emoji);
                    i += 1 + close_rel + 1;
                    continue;
                }
            }
        }
        if let Some(ch) = raw[i..].chars().next() {
            push_escaped_html_char(ch, out);
            i += ch.len_utf8();
        } else {
            break;
        }
    }
}

/// Renders a fenced Mermaid diagram block into a zero-JS themed inline SVG card.
///
/// # Arguments
///
/// - `&str` - Raw Mermaid diagram source text.
/// - `&mut String` - Destination HTML output buffer.
fn render_mermaid_svg_html(source: &str, out: &mut String) {
    let diagram: MermaidDiagram = parse_mermaid_diagram(source);
    let badge: &str = match &diagram {
        MermaidDiagram::Flowchart(_) => MERMAID_BADGE_FLOWCHART,
        MermaidDiagram::Sequence(_) => MERMAID_BADGE_SEQUENCE,
        MermaidDiagram::State(_) => MERMAID_BADGE_STATE,
        MermaidDiagram::Class(_) => MERMAID_BADGE_CLASS,
        MermaidDiagram::Er(_) => MERMAID_BADGE_ER,
        MermaidDiagram::Pie(_) => MERMAID_BADGE_PIE,
        MermaidDiagram::Gantt(_) => MERMAID_BADGE_GANTT,
    };
    out.push_str(MERMAID_CARD_OPEN);
    out.push_str(badge);
    out.push_str(MERMAID_CARD_MID);
    match &diagram {
        MermaidDiagram::Flowchart(fc) | MermaidDiagram::State(fc) => {
            render_mermaid_flow_svg(fc, out);
        }
        MermaidDiagram::Sequence(seq) => {
            render_mermaid_sequence_svg(seq, out);
        }
        MermaidDiagram::Class(cls) => {
            render_mermaid_class_svg(cls, out);
        }
        MermaidDiagram::Er(er) => {
            render_mermaid_er_svg(er, out);
        }
        MermaidDiagram::Pie(pie) => {
            render_mermaid_pie_svg(pie, out);
        }
        MermaidDiagram::Gantt(gantt) => {
            render_mermaid_gantt_svg(gantt, out);
        }
    }
    out.push_str(MERMAID_CARD_CLOSE);
}

/// Runs 3-color DFS from `u` to collect acyclic forward edges for static Mermaid SVG layout.
///
/// # Arguments
///
/// - `usize` - Current node index.
/// - `&[Vec<usize>]` - Directed adjacency lists.
/// - `&mut [u8]` - Visitation state (`0` unvisited, `1` active stack, `2` done).
/// - `&mut Vec<(usize, usize)>` - Collected DAG edges.
fn dfs_svg_dag_edges(
    u: usize,
    adj: &[Vec<usize>],
    state: &mut [u8],
    dag_edges: &mut Vec<(usize, usize)>,
) {
    state[u] = 1;
    for &v in adj[u].iter() {
        if u == v || state[v] == 1 {
            continue;
        }
        dag_edges.push((u, v));
        if state[v] == 0 {
            dfs_svg_dag_edges(v, adj, state, dag_edges);
        }
    }
    state[u] = 2;
}

/// Renders a Mermaid flowchart or state machine AST as a themed inline SVG.
///
/// # Arguments
///
/// - `&MermaidFlowchart` - Parsed flowchart or state graph.
/// - `&mut String` - Destination HTML/SVG buffer.
fn render_mermaid_flow_svg(fc: &MermaidFlowchart, out: &mut String) {
    let n: usize = fc.get_nodes().len();
    if n == 0 {
        return;
    }
    let mut id_map: HashMap<&str, usize> = HashMap::with_capacity(n);
    for (idx, node) in fc.get_nodes().iter().enumerate() {
        let _: Option<usize> = id_map.insert(node.get_id().as_str(), idx);
    }
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut in_deg: Vec<usize> = vec![0; n];
    for edge in fc.get_edges().iter() {
        if let (Some(&u), Some(&v)) = (
            id_map.get(edge.get_from().as_str()),
            id_map.get(edge.get_to().as_str()),
        ) && u != v
        {
            adj[u].push(v);
            in_deg[v] += 1;
        }
    }
    let mut visit_state: Vec<u8> = vec![0; n];
    let mut dag_edges: Vec<(usize, usize)> = Vec::with_capacity(fc.get_edges().len());
    for (idx, node) in fc.get_nodes().iter().enumerate() {
        if node.get_shape() == MermaidNodeShape::StateStart && visit_state[idx] == 0 {
            dfs_svg_dag_edges(idx, &adj, &mut visit_state, &mut dag_edges);
        }
    }
    for (idx, node) in fc.get_nodes().iter().enumerate() {
        if in_deg[idx] == 0
            && node.get_shape() != MermaidNodeShape::StateEnd
            && visit_state[idx] == 0
        {
            dfs_svg_dag_edges(idx, &adj, &mut visit_state, &mut dag_edges);
        }
    }
    for idx in 0..n {
        if visit_state[idx] == 0 {
            dfs_svg_dag_edges(idx, &adj, &mut visit_state, &mut dag_edges);
        }
    }
    let mut layers: Vec<usize> = vec![0; n];
    for _ in 0..n {
        let mut changed: bool = false;
        for &(u, v) in dag_edges.iter() {
            if layers[v] <= layers[u] && layers[u] + 1 < n {
                layers[v] = layers[u] + 1;
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    for edge in fc.get_edges().iter() {
        if let (Some(&u), Some(&v)) = (
            id_map.get(edge.get_from().as_str()),
            id_map.get(edge.get_to().as_str()),
        ) && u != v
            && fc.get_nodes()[v].get_shape() == MermaidNodeShape::StateEnd
            && layers[v] <= layers[u]
        {
            layers[v] = layers[u] + 1;
        }
    }
    let max_layer: usize = layers.iter().copied().max().unwrap_or(0);
    let mut buckets: Vec<Vec<usize>> = vec![Vec::new(); max_layer + 1];
    for (idx, &lay) in layers.iter().enumerate() {
        buckets[lay].push(idx);
    }
    let horiz: bool = matches!(
        fc.get_direction(),
        MermaidFlowDirection::LeftRight | MermaidFlowDirection::RightLeft
    );
    let reversed: bool = matches!(
        fc.get_direction(),
        MermaidFlowDirection::RightLeft | MermaidFlowDirection::BottomTop
    );
    let node_sizes: Vec<(f32, f32)> = fc
        .get_nodes()
        .iter()
        .map(|node: &MermaidFlowNode| match node.get_shape() {
            MermaidNodeShape::StateStart | MermaidNodeShape::StateEnd => (24.0, 24.0),
            MermaidNodeShape::Circle => {
                let d: f32 =
                    ((node.get_label().chars().count() as f32) * 7.6 + 26.0).clamp(52.0, 96.0);
                (d, d)
            }
            MermaidNodeShape::Diamond => {
                let w: f32 =
                    ((node.get_label().chars().count() as f32) * 8.0 + 46.0).clamp(108.0, 196.0);
                (w, 58.0)
            }
            _ => {
                let w: f32 =
                    ((node.get_label().chars().count() as f32) * 7.6 + 34.0).clamp(96.0, 210.0);
                (w, 40.0)
            }
        })
        .collect();
    let pad: f32 = 28.0;
    let main_gap: f32 = if horiz { 54.0 } else { 44.0 };
    let cross_gap: f32 = 24.0;
    let layer_count: usize = buckets.len();
    let mut layer_main: Vec<f32> = vec![0.0; layer_count];
    let mut layer_cross: Vec<f32> = vec![0.0; layer_count];
    for (li, items) in buckets.iter().enumerate() {
        let mut max_m: f32 = 0.0;
        let mut sum_c: f32 = 0.0;
        for (pos, &ni) in items.iter().enumerate() {
            let (nw, nh): (f32, f32) = node_sizes[ni];
            if horiz {
                max_m = max_m.max(nw);
                sum_c += nh;
            } else {
                max_m = max_m.max(nh);
                sum_c += nw;
            }
            if pos + 1 < items.len() {
                sum_c += cross_gap;
            }
        }
        layer_main[li] = max_m;
        layer_cross[li] = sum_c;
    }
    let total_main: f32 =
        layer_main.iter().sum::<f32>() + (layer_count.saturating_sub(1) as f32) * main_gap;
    let total_cross: f32 = layer_cross.iter().copied().fold(0.0_f32, f32::max);
    let mut rects: Vec<(f32, f32, f32, f32)> = vec![(0.0, 0.0, 0.0, 0.0); n];
    let mut main_cur: f32 = pad;
    let order: Vec<usize> = if reversed {
        (0..layer_count).rev().collect()
    } else {
        (0..layer_count).collect()
    };
    for li in order.into_iter() {
        let items: &[usize] = &buckets[li];
        let band_m: f32 = layer_main[li];
        let band_c: f32 = layer_cross[li];
        let mut cross_cur: f32 = pad + (total_cross - band_c) * 0.5;
        for &ni in items.iter() {
            let (nw, nh): (f32, f32) = node_sizes[ni];
            if horiz {
                rects[ni] = (main_cur + (band_m - nw) * 0.5, cross_cur, nw, nh);
                cross_cur += nh + cross_gap;
            } else {
                rects[ni] = (cross_cur, main_cur + (band_m - nh) * 0.5, nw, nh);
                cross_cur += nw + cross_gap;
            }
        }
        main_cur += band_m + main_gap;
    }
    let (svg_w, svg_h): (f32, f32) = if horiz {
        (
            (total_main + pad * 2.0).max(360.0),
            (total_cross + pad * 2.0).max(140.0),
        )
    } else {
        (
            (total_cross + pad * 2.0).max(360.0),
            (total_main + pad * 2.0).max(140.0),
        )
    };
    out.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {svg_w:.1} {svg_h:.1}\" width=\"{svg_w:.0}\" height=\"{svg_h:.0}\" role=\"img\" aria-label=\"Mermaid diagram\">"
    ));
    out.push_str(MERMAID_SVG_ARROW_MARKER);
    for edge in fc.get_edges().iter() {
        if let (Some(&u), Some(&v)) = (
            id_map.get(edge.get_from().as_str()),
            id_map.get(edge.get_to().as_str()),
        ) {
            let (ux, uy, uw, uh): (f32, f32, f32, f32) = rects[u];
            let (vx, vy, vw, vh): (f32, f32, f32, f32) = rects[v];
            let (x1, y1, x2, y2): (f32, f32, f32, f32) = if horiz {
                let sx: f32 = if vx >= ux { ux + uw } else { ux };
                let tx: f32 = if vx >= ux { vx } else { vx + vw };
                (sx, uy + uh * 0.5, tx, vy + vh * 0.5)
            } else {
                let sy: f32 = if vy >= uy { uy + uh } else { uy };
                let ty: f32 = if vy >= uy { vy } else { vy + vh };
                (ux + uw * 0.5, sy, vx + vw * 0.5, ty)
            };
            let dash: &str = match edge.get_style() {
                MermaidEdgeStyle::Dashed => MERMAID_SVG_DASH_ATTR,
                _ => "",
            };
            let sw: f32 = if edge.get_style() == MermaidEdgeStyle::Thick {
                2.6
            } else {
                1.6
            };
            let marker: &str = if edge.get_style() == MermaidEdgeStyle::Plain {
                ""
            } else {
                MERMAID_SVG_MARKER_ATTR
            };
            out.push_str(&format!(
                "<line x1=\"{x1:.1}\" y1=\"{y1:.1}\" x2=\"{x2:.1}\" y2=\"{y2:.1}\" stroke=\"var(--muted-fg)\" stroke-width=\"{sw:.1}\"{dash}{marker}/>"
            ));
            let lbl: &str = edge.get_label().trim();
            if !lbl.is_empty() {
                let mx: f32 = (x1 + x2) * 0.5;
                let my: f32 = (y1 + y2) * 0.5;
                let bw: f32 = ((lbl.chars().count() as f32) * 6.8 + 12.0).max(28.0);
                let bx: f32 = mx - bw * 0.5;
                let by: f32 = my - 10.0;
                out.push_str(&format!(
                    "<rect x=\"{bx:.1}\" y=\"{by:.1}\" width=\"{bw:.1}\" height=\"20\" rx=\"4\" fill=\"var(--muted)\" stroke=\"var(--border)\" stroke-width=\"1\"/>"
                ));
                out.push_str(&format!(
                    "<text x=\"{mx:.1}\" y=\"{:.1}\" text-anchor=\"middle\" font-size=\"11\" font-weight=\"600\" fill=\"var(--muted-fg)\">{}</text>",
                    my + 4.0,
                    escape_html(lbl)
                ));
            }
        }
    }
    for (idx, node) in fc.get_nodes().iter().enumerate() {
        let (rx, ry, rw, rh): (f32, f32, f32, f32) = rects[idx];
        let cx: f32 = rx + rw * 0.5;
        let cy: f32 = ry + rh * 0.5;
        let accent: &str = MERMAID_SVG_PALETTE[idx % MERMAID_SVG_PALETTE.len()];
        match node.get_shape() {
            MermaidNodeShape::StateStart => {
                out.push_str(&format!(
                    "<circle cx=\"{cx:.1}\" cy=\"{cy:.1}\" r=\"9\" fill=\"var(--fg)\"/>"
                ));
            }
            MermaidNodeShape::StateEnd => {
                out.push_str(&format!(
                    "<circle cx=\"{cx:.1}\" cy=\"{cy:.1}\" r=\"10\" fill=\"none\" stroke=\"var(--fg)\" stroke-width=\"2\"/><circle cx=\"{cx:.1}\" cy=\"{cy:.1}\" r=\"5.5\" fill=\"var(--fg)\"/>"
                ));
            }
            MermaidNodeShape::Diamond => {
                let pts: String = format!(
                    "{cx:.1},{ry:.1} {:.1},{cy:.1} {cx:.1},{:.1} {rx:.1},{cy:.1}",
                    rx + rw,
                    ry + rh
                );
                out.push_str(&format!(
                    "<polygon points=\"{pts}\" fill=\"var(--muted)\" stroke=\"{accent}\" stroke-width=\"1.8\"/>"
                ));
            }
            MermaidNodeShape::Circle => {
                let r: f32 = rw.min(rh) * 0.5;
                out.push_str(&format!(
                    "<circle cx=\"{cx:.1}\" cy=\"{cy:.1}\" r=\"{r:.1}\" fill=\"var(--muted)\" stroke=\"{accent}\" stroke-width=\"1.8\"/>"
                ));
            }
            MermaidNodeShape::Stadium => {
                let rad: f32 = rh * 0.5;
                out.push_str(&format!(
                    "<rect x=\"{rx:.1}\" y=\"{ry:.1}\" width=\"{rw:.1}\" height=\"{rh:.1}\" rx=\"{rad:.1}\" fill=\"var(--muted)\" stroke=\"{accent}\" stroke-width=\"1.8\"/>"
                ));
            }
            MermaidNodeShape::Rounded => {
                out.push_str(&format!(
                    "<rect x=\"{rx:.1}\" y=\"{ry:.1}\" width=\"{rw:.1}\" height=\"{rh:.1}\" rx=\"10\" fill=\"var(--muted)\" stroke=\"{accent}\" stroke-width=\"1.8\"/>"
                ));
            }
            MermaidNodeShape::Box => {
                out.push_str(&format!(
                    "<rect x=\"{rx:.1}\" y=\"{ry:.1}\" width=\"{rw:.1}\" height=\"{rh:.1}\" rx=\"5\" fill=\"var(--muted)\" stroke=\"{accent}\" stroke-width=\"1.8\"/>"
                ));
            }
        }
        let lbl: &str = node.get_label().trim();
        if !lbl.is_empty() {
            out.push_str(&format!(
                "<text x=\"{cx:.1}\" y=\"{:.1}\" text-anchor=\"middle\" font-size=\"12\" font-weight=\"600\" fill=\"var(--fg)\">{}</text>",
                cy + 4.0,
                escape_html(lbl)
            ));
        }
    }
    out.push_str(MERMAID_SVG_CLOSE);
}

/// Renders a Mermaid `sequenceDiagram` AST as a themed inline SVG.
///
/// # Arguments
///
/// - `&MermaidSequence` - Parsed sequence diagram.
/// - `&mut String` - Destination HTML/SVG buffer.
fn render_mermaid_sequence_svg(seq: &MermaidSequence, out: &mut String) {
    let p_count: usize = seq.get_participants().len();
    if p_count == 0 {
        return;
    }
    let col_w: f32 = 176.0;
    let pad: f32 = 24.0;
    let box_w: f32 = 132.0;
    let box_h: f32 = 34.0;
    let row_h: f32 = 44.0;
    let msg_count: usize = seq.get_messages().len();
    let svg_w: f32 = ((p_count as f32) * col_w + pad * 2.0).max(380.0);
    let top_y: f32 = pad;
    let life_top: f32 = top_y + box_h;
    let life_bot: f32 = life_top + 22.0 + (msg_count as f32) * row_h + 18.0;
    let svg_h: f32 = life_bot + box_h + pad;
    out.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {svg_w:.1} {svg_h:.1}\" width=\"{svg_w:.0}\" height=\"{svg_h:.0}\" role=\"img\" aria-label=\"Mermaid sequence diagram\">"
    ));
    out.push_str(MERMAID_SVG_ARROW_MARKER);
    let mut centers: Vec<f32> = Vec::with_capacity(p_count);
    for (idx, (_, label)) in seq.get_participants().iter().enumerate() {
        let cx: f32 = pad + (idx as f32) * col_w + col_w * 0.5;
        centers.push(cx);
        let accent: &str = MERMAID_SVG_PALETTE[idx % MERMAID_SVG_PALETTE.len()];
        out.push_str(&format!(
            "<line x1=\"{cx:.1}\" y1=\"{life_top:.1}\" x2=\"{cx:.1}\" y2=\"{life_bot:.1}\" stroke=\"var(--border)\" stroke-width=\"1.4\" stroke-dasharray=\"5 4\"/>"
        ));
        for by in [top_y, life_bot].into_iter() {
            let bx: f32 = cx - box_w * 0.5;
            out.push_str(&format!(
                "<rect x=\"{bx:.1}\" y=\"{by:.1}\" width=\"{box_w:.1}\" height=\"{box_h:.1}\" rx=\"7\" fill=\"var(--muted)\" stroke=\"{accent}\" stroke-width=\"1.6\"/>"
            ));
            out.push_str(&format!(
                "<text x=\"{cx:.1}\" y=\"{:.1}\" text-anchor=\"middle\" font-size=\"12\" font-weight=\"600\" fill=\"var(--fg)\">{}</text>",
                by + 21.0,
                escape_html(label)
            ));
        }
    }
    for (idx, msg) in seq.get_messages().iter().enumerate() {
        let y: f32 = life_top + 28.0 + (idx as f32) * row_h;
        let x1: f32 = centers.get(msg.get_from_idx()).copied().unwrap_or(pad);
        let x2: f32 = centers.get(msg.get_to_idx()).copied().unwrap_or(pad);
        if msg.get_is_note() {
            let nx: f32 = x1.min(x2) - 48.0;
            let nw: f32 = (x2 - x1).abs().max(96.0) + 96.0;
            out.push_str(&format!(
                "<rect x=\"{nx:.1}\" y=\"{:.1}\" width=\"{nw:.1}\" height=\"26\" rx=\"5\" fill=\"var(--muted)\" stroke=\"var(--warn)\" stroke-width=\"1.4\"/>",
                y - 13.0
            ));
            out.push_str(&format!(
                "<text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\" font-size=\"11.5\" fill=\"var(--fg)\">{}</text>",
                nx + nw * 0.5,
                y + 4.0,
                escape_html(msg.get_text())
            ));
        } else {
            let dash: &str = match msg.get_style() {
                MermaidEdgeStyle::Dashed => MERMAID_SVG_DASH_ATTR,
                _ => "",
            };
            out.push_str(&format!(
                "<line x1=\"{x1:.1}\" y1=\"{y:.1}\" x2=\"{x2:.1}\" y2=\"{y:.1}\" stroke=\"var(--muted-fg)\" stroke-width=\"1.6\"{dash} marker-end=\"url(#euv-m-arrow)\"/>"
            ));
            out.push_str(&format!(
                "<text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\" font-size=\"11.5\" font-weight=\"500\" fill=\"var(--fg)\">{}</text>",
                (x1 + x2) * 0.5,
                y - 7.0,
                escape_html(msg.get_text())
            ));
        }
    }
    out.push_str(MERMAID_SVG_CLOSE);
}

/// Renders a Mermaid `classDiagram` AST as a themed inline SVG.
///
/// # Arguments
///
/// - `&MermaidClassDiagram` - Parsed class diagram.
/// - `&mut String` - Destination HTML/SVG buffer.
fn render_mermaid_class_svg(cls: &MermaidClassDiagram, out: &mut String) {
    let count: usize = cls.get_classes().len();
    if count == 0 {
        return;
    }
    let pad: f32 = 24.0;
    let gap: f32 = 28.0;
    let box_w: f32 = 180.0;
    let mut max_h: f32 = 64.0;
    for c in cls.get_classes().iter() {
        let h: f32 = 34.0 + (c.get_members().len() as f32) * 18.0 + 12.0;
        max_h = max_h.max(h);
    }
    let svg_w: f32 = pad * 2.0 + (count as f32) * box_w + (count.saturating_sub(1) as f32) * gap;
    let svg_h: f32 = pad * 2.0 + max_h + 24.0;
    out.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {svg_w:.1} {svg_h:.1}\" width=\"{svg_w:.0}\" height=\"{svg_h:.0}\" role=\"img\" aria-label=\"Mermaid class diagram\">"
    ));
    out.push_str(MERMAID_SVG_ARROW_MARKER);
    let mut pos_map: HashMap<&str, (f32, f32, f32)> = HashMap::with_capacity(count);
    for (idx, c) in cls.get_classes().iter().enumerate() {
        let x: f32 = pad + (idx as f32) * (box_w + gap);
        let h: f32 = 34.0 + (c.get_members().len() as f32) * 18.0 + 12.0;
        let _: Option<(f32, f32, f32)> = pos_map.insert(c.get_name().as_str(), (x, pad, h));
        let accent: &str = MERMAID_SVG_PALETTE[idx % MERMAID_SVG_PALETTE.len()];
        out.push_str(&format!(
            "<rect x=\"{x:.1}\" y=\"{pad:.1}\" width=\"{box_w:.1}\" height=\"{h:.1}\" rx=\"7\" fill=\"var(--muted)\" stroke=\"{accent}\" stroke-width=\"1.6\"/>"
        ));
        out.push_str(&format!(
            "<line x1=\"{x:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"var(--border)\" stroke-width=\"1\"/>",
            pad + 30.0,
            x + box_w,
            pad + 30.0
        ));
        out.push_str(&format!(
            "<text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\" font-size=\"12.5\" font-weight=\"700\" fill=\"var(--fg)\">{}</text>",
            x + box_w * 0.5,
            pad + 20.0,
            escape_html(c.get_name())
        ));
        for (mi, mem) in c.get_members().iter().enumerate() {
            out.push_str(&format!(
                "<text x=\"{:.1}\" y=\"{:.1}\" font-size=\"11\" fill=\"var(--muted-fg)\">{}</text>",
                x + 10.0,
                pad + 48.0 + (mi as f32) * 18.0,
                escape_html(mem)
            ));
        }
    }
    for rel in cls.get_relations().iter() {
        if let (Some(&(x1, y1, h1)), Some(&(x2, y2, h2))) = (
            pos_map.get(rel.get_from().as_str()),
            pos_map.get(rel.get_to().as_str()),
        ) {
            let sx: f32 = if x2 >= x1 { x1 + box_w } else { x1 };
            let tx: f32 = if x2 >= x1 { x2 } else { x2 + box_w };
            out.push_str(&format!(
                "<line x1=\"{sx:.1}\" y1=\"{:.1}\" x2=\"{tx:.1}\" y2=\"{:.1}\" stroke=\"var(--muted-fg)\" stroke-width=\"1.5\" marker-end=\"url(#euv-m-arrow)\"/>",
                y1 + h1 * 0.5,
                y2 + h2 * 0.5
            ));
        }
    }
    out.push_str(MERMAID_SVG_CLOSE);
}

/// Renders a Mermaid `erDiagram` AST as a themed inline SVG.
///
/// # Arguments
///
/// - `&MermaidErDiagram` - Parsed ER diagram.
/// - `&mut String` - Destination HTML/SVG buffer.
fn render_mermaid_er_svg(er: &MermaidErDiagram, out: &mut String) {
    let count: usize = er.get_entities().len();
    if count == 0 {
        return;
    }
    let pad: f32 = 24.0;
    let gap: f32 = 52.0;
    let box_w: f32 = 160.0;
    let mut max_h: f32 = 56.0;
    for ent in er.get_entities().iter() {
        let h: f32 = 34.0 + (ent.get_attributes().len() as f32) * 18.0 + 10.0;
        max_h = max_h.max(h);
    }
    let svg_w: f32 = pad * 2.0 + (count as f32) * box_w + (count.saturating_sub(1) as f32) * gap;
    let svg_h: f32 = pad * 2.0 + max_h + 16.0;
    out.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {svg_w:.1} {svg_h:.1}\" width=\"{svg_w:.0}\" height=\"{svg_h:.0}\" role=\"img\" aria-label=\"Mermaid ER diagram\">"
    ));
    let mut pos_map: HashMap<&str, (f32, f32, f32)> = HashMap::with_capacity(count);
    for (idx, ent) in er.get_entities().iter().enumerate() {
        let x: f32 = pad + (idx as f32) * (box_w + gap);
        let h: f32 = 34.0 + (ent.get_attributes().len() as f32) * 18.0 + 10.0;
        let _: Option<(f32, f32, f32)> = pos_map.insert(ent.get_name().as_str(), (x, pad, h));
        let accent: &str = MERMAID_SVG_PALETTE[idx % MERMAID_SVG_PALETTE.len()];
        out.push_str(&format!(
            "<rect x=\"{x:.1}\" y=\"{pad:.1}\" width=\"{box_w:.1}\" height=\"{h:.1}\" rx=\"6\" fill=\"var(--muted)\" stroke=\"{accent}\" stroke-width=\"1.6\"/>"
        ));
        out.push_str(&format!(
            "<text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\" font-size=\"12.5\" font-weight=\"700\" fill=\"var(--fg)\">{}</text>",
            x + box_w * 0.5,
            pad + 21.0,
            escape_html(ent.get_name())
        ));
        for (ai, attr) in ent.get_attributes().iter().enumerate() {
            out.push_str(&format!(
                "<text x=\"{:.1}\" y=\"{:.1}\" font-size=\"11\" fill=\"var(--muted-fg)\">{}</text>",
                x + 10.0,
                pad + 44.0 + (ai as f32) * 18.0,
                escape_html(attr)
            ));
        }
    }
    for rel in er.get_relations().iter() {
        if let (Some(&(x1, y1, h1)), Some(&(x2, y2, h2))) = (
            pos_map.get(rel.get_left().as_str()),
            pos_map.get(rel.get_right().as_str()),
        ) {
            let sx: f32 = if x2 >= x1 { x1 + box_w } else { x1 };
            let tx: f32 = if x2 >= x1 { x2 } else { x2 + box_w };
            let sy: f32 = y1 + h1 * 0.5;
            let ty: f32 = y2 + h2 * 0.5;
            out.push_str(&format!(
                "<line x1=\"{sx:.1}\" y1=\"{sy:.1}\" x2=\"{tx:.1}\" y2=\"{ty:.1}\" stroke=\"var(--muted-fg)\" stroke-width=\"1.5\"/>"
            ));
            if !rel.get_label().is_empty() {
                out.push_str(&format!(
                    "<text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\" font-size=\"10.5\" fill=\"var(--muted-fg)\">{}</text>",
                    (sx + tx) * 0.5,
                    (sy + ty) * 0.5 - 6.0,
                    escape_html(rel.get_label())
                ));
            }
        }
    }
    out.push_str(MERMAID_SVG_CLOSE);
}

/// Renders a Mermaid `pie` chart AST as a proportional inline SVG pie + legend.
///
/// # Arguments
///
/// - `&MermaidPieChart` - Parsed pie chart.
/// - `&mut String` - Destination HTML/SVG buffer.
fn render_mermaid_pie_svg(pie: &MermaidPieChart, out: &mut String) {
    let total: f32 = pie
        .get_slices()
        .iter()
        .map(|s: &MermaidPieSlice| s.get_value())
        .sum();
    if total <= 0.0 {
        return;
    }
    let has_title: bool = !pie.get_title().is_empty();
    let title_off: f32 = if has_title { 28.0 } else { 8.0 };
    let r: f32 = 76.0;
    let cx: f32 = 112.0;
    let cy: f32 = title_off + r + 14.0;
    let legend_x: f32 = cx + r + 36.0;
    let legend_h: f32 = (pie.get_slices().len() as f32) * 24.0;
    let svg_h: f32 = (cy + r + 20.0).max(title_off + legend_h + 36.0);
    let svg_w: f32 = 460.0;
    out.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {svg_w:.1} {svg_h:.1}\" width=\"{svg_w:.0}\" height=\"{svg_h:.0}\" role=\"img\" aria-label=\"Mermaid pie chart\">"
    ));
    if has_title {
        out.push_str(&format!(
            "<text x=\"{:.1}\" y=\"22\" text-anchor=\"middle\" font-size=\"13.5\" font-weight=\"700\" fill=\"var(--fg)\">{}</text>",
            svg_w * 0.5,
            escape_html(pie.get_title())
        ));
    }
    let mut angle: f32 = -FRAC_PI_2;
    let legend_y0: f32 = cy - legend_h * 0.5 + 12.0;
    for (idx, slice) in pie.get_slices().iter().enumerate() {
        let color: &str = MERMAID_SVG_PALETTE[idx % MERMAID_SVG_PALETTE.len()];
        let frac: f32 = slice.get_value() / total;
        let sweep: f32 = frac * TAU;
        if pie.get_slices().len() == 1 {
            out.push_str(&format!(
                "<circle cx=\"{cx:.1}\" cy=\"{cy:.1}\" r=\"{r:.1}\" fill=\"{color}\"/>"
            ));
        } else {
            let x1: f32 = cx + r * angle.cos();
            let y1: f32 = cy + r * angle.sin();
            let end_a: f32 = angle + sweep;
            let x2: f32 = cx + r * end_a.cos();
            let y2: f32 = cy + r * end_a.sin();
            let large: u8 = if sweep > PI { 1 } else { 0 };
            out.push_str(&format!(
                "<path d=\"M {cx:.1} {cy:.1} L {x1:.1} {y1:.1} A {r:.1} {r:.1} 0 {large} 1 {x2:.1} {y2:.1} Z\" fill=\"{color}\" stroke=\"var(--card)\" stroke-width=\"1.5\"/>"
            ));
            angle = end_a;
        }
        let ly: f32 = legend_y0 + (idx as f32) * 24.0;
        let pct: f32 = frac * 100.0;
        out.push_str(&format!(
            "<rect x=\"{legend_x:.1}\" y=\"{:.1}\" width=\"12\" height=\"12\" rx=\"3\" fill=\"{color}\"/>",
            ly - 10.0
        ));
        out.push_str(&format!(
            "<text x=\"{:.1}\" y=\"{ly:.1}\" font-size=\"12\" fill=\"var(--fg)\">{} ({pct:.0}%)</text>",
            legend_x + 18.0,
            escape_html(slice.get_label())
        ));
    }
    out.push_str(MERMAID_SVG_CLOSE);
}

/// Renders a Mermaid `gantt` chart AST as a themed inline SVG timeline.
///
/// # Arguments
///
/// - `&MermaidGanttChart` - Parsed Gantt timeline.
/// - `&mut String` - Destination HTML/SVG buffer.
fn render_mermaid_gantt_svg(gantt: &MermaidGanttChart, out: &mut String) {
    let count: usize = gantt.get_tasks().len();
    if count == 0 {
        return;
    }
    let has_title: bool = !gantt.get_title().is_empty();
    let top: f32 = if has_title { 38.0 } else { 16.0 };
    let label_w: f32 = 148.0;
    let track_w: f32 = 340.0;
    let row_h: f32 = 30.0;
    let svg_w: f32 = label_w + track_w + 28.0;
    let svg_h: f32 = top + (count as f32) * row_h + 16.0;
    let max_end: f32 = gantt
        .get_tasks()
        .iter()
        .map(|t: &MermaidGanttTask| t.get_start_unit() + t.get_duration_unit())
        .fold(1.0_f32, f32::max);
    out.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {svg_w:.1} {svg_h:.1}\" width=\"{svg_w:.0}\" height=\"{svg_h:.0}\" role=\"img\" aria-label=\"Mermaid Gantt chart\">"
    ));
    if has_title {
        out.push_str(&format!(
            "<text x=\"{:.1}\" y=\"22\" text-anchor=\"middle\" font-size=\"13.5\" font-weight=\"700\" fill=\"var(--fg)\">{}</text>",
            svg_w * 0.5,
            escape_html(gantt.get_title())
        ));
    }
    for (idx, task) in gantt.get_tasks().iter().enumerate() {
        let y: f32 = top + (idx as f32) * row_h;
        let bx: f32 = label_w + (task.get_start_unit() / max_end) * track_w;
        let bw: f32 = ((task.get_duration_unit() / max_end) * track_w).max(16.0);
        let fill: &str = match task.get_status() {
            MermaidGanttStatus::Done => MERMAID_SVG_VAR_TIP,
            MermaidGanttStatus::Active => MERMAID_SVG_VAR_LINK,
            MermaidGanttStatus::Critical => MERMAID_SVG_VAR_DANGER,
            MermaidGanttStatus::Normal => MERMAID_SVG_VAR_IMPORTANT,
        };
        out.push_str(&format!(
            "<text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"end\" font-size=\"11.5\" fill=\"var(--fg)\">{}</text>",
            label_w - 10.0,
            y + 15.0,
            escape_html(task.get_label())
        ));
        out.push_str(&format!(
            "<rect x=\"{bx:.1}\" y=\"{:.1}\" width=\"{bw:.1}\" height=\"18\" rx=\"5\" fill=\"{fill}\"/>",
            y + 2.0
        ));
    }
    out.push_str(MERMAID_SVG_CLOSE);
}
