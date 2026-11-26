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
    let mut in_table_head: bool = false;
    let mut cell_index: usize = 0;
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
                        state.out.push_str(HTML_CODE_WRAP_OPEN);
                        state.out.push_str(&escape_html(&lang));
                        state.out.push_str(HTML_CODE_HEAD_MID);
                        state.out.push_str(&escape_html(&lang));
                        state.out.push_str(HTML_LINK_MID);
                        state.out.push_str(&escape_html(trimmed));
                        state.out.push_str(HTML_CODE_CLOSE);
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
            Event::Start(Tag::BlockQuote(_)) => state.out.push_str(HTML_QUOTE_OPEN),
            Event::End(TagEnd::BlockQuote(_)) => state.out.push_str(HTML_QUOTE_CLOSE),
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
            Event::Text(text) => state.out.push_str(&escape_html(&text)),
            Event::Code(text) => {
                state.out.push_str(HTML_INLINE_CODE_OPEN);
                state.out.push_str(&escape_html(&text));
                state.out.push_str(HTML_INLINE_CODE_CLOSE);
            }
            Event::SoftBreak => state.out.push(' '),
            Event::HardBreak => state.out.push_str(HTML_BR),
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
                out.push_str(HTML_CODE_WRAP_OPEN);
                out.push_str(&escape_html(language));
                out.push_str(HTML_CODE_HEAD_MID);
                out.push_str(&escape_html(language));
                out.push_str(HTML_LINK_MID);
                out.push_str(&escape_html(code));
                out.push_str(HTML_CODE_CLOSE);
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
            && bytes[i] == b'o'
            && bytes[i + 1] == b'n'
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
        match ch {
            '&' => out.push_str(HTML_ESC_AMP),
            '<' => out.push_str(HTML_ESC_LT),
            '>' => out.push_str(HTML_ESC_GT),
            '"' => out.push_str(HTML_ESC_QUOT),
            _ => out.push(ch),
        }
    }
    out
}
