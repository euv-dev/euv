use super::*;

/// Parses a single Markdown document with optional YAML frontmatter into a `DocPage`.
///
/// # Arguments
///
/// - `&str` - Canonical route path for this page.
/// - `&str` - Section label derived from directory hierarchy.
/// - `&str` - Raw Markdown source text.
///
/// # Returns
///
/// - `DocPage` - Parsed structured documentation page.
#[must_use]
pub fn parse_markdown_page(route: &str, default_section: &str, raw: &str) -> DocPage {
    let (frontmatter, body): (DocFrontmatter, &str) = split_frontmatter(raw);
    let mut blocks: Vec<DocBlock> = Vec::new();
    let mut toc: Vec<DocTocItem> = Vec::new();
    let mut seen_slugs: BTreeMap<String, usize> = BTreeMap::new();
    let mut active_heading: Option<(u8, Option<String>)> = None;
    let mut first_h1_title: Option<String> = None;
    let mut in_paragraph: bool = false;
    let mut list_stack: Vec<String> = Vec::new();
    let mut blockquote_depth: usize = 0;
    let mut in_table: bool = false;
    let mut in_table_head: bool = false;
    let mut in_table_cell: bool = false;
    let mut table_headers: Vec<String> = Vec::new();
    let mut table_rows: Vec<Vec<String>> = Vec::new();
    let mut current_row: Vec<String> = Vec::new();
    let mut active_code_lang: Option<String> = None;
    let mut active_image_src: Option<String> = None;
    let mut image_alt_buf: String = String::new();
    let mut buffer: String = String::new();
    let parser: Parser<'_> = Parser::new_ext(body, Options::all());
    for event in parser {
        match event {
            Event::Start(Tag::Heading { level, id, .. }) => {
                active_heading = Some((heading_level_num(level), id.map(CowStr::into_string)));
                buffer.clear();
            }
            Event::End(TagEnd::Heading(_)) => {
                if let Some((level, custom_id)) = active_heading.take() {
                    let text: String = buffer.trim().to_string();
                    if !text.is_empty() {
                        if level == 1 && first_h1_title.is_none() {
                            first_h1_title = Some(text.clone());
                        }
                        let raw_slug: String = custom_id
                            .map(|id_str: String| slugify(&id_str))
                            .filter(|s: &String| !s.is_empty())
                            .unwrap_or_else(|| slugify(&text));
                        let slug: String = deduplicate_slug(raw_slug, &mut seen_slugs);
                        if (2..=4).contains(&level) {
                            toc.push(DocTocItem::new(level, slug.clone(), text.clone()));
                        }
                        blocks.push(DocBlock::Heading { level, slug, text });
                    }
                }
                buffer.clear();
            }
            Event::Start(Tag::BlockQuote(_)) => {
                blockquote_depth += 1;
            }
            Event::End(TagEnd::BlockQuote(_)) => {
                blockquote_depth = blockquote_depth.saturating_sub(1);
            }
            Event::Start(Tag::Table(_)) => {
                in_table = true;
                table_headers.clear();
                table_rows.clear();
            }
            Event::End(TagEnd::Table) => {
                if in_table {
                    in_table = false;
                    let headers: Vec<String> = take(&mut table_headers);
                    let rows: Vec<Vec<String>> = take(&mut table_rows);
                    if !headers.is_empty() || !rows.is_empty() {
                        blocks.push(DocBlock::Table { headers, rows });
                    }
                }
            }
            Event::Start(Tag::TableHead) => {
                in_table_head = true;
                current_row.clear();
            }
            Event::End(TagEnd::TableHead) => {
                in_table_head = false;
                table_headers = take(&mut current_row);
            }
            Event::Start(Tag::TableRow) => {
                current_row.clear();
            }
            Event::End(TagEnd::TableRow) => {
                if !in_table_head {
                    table_rows.push(take(&mut current_row));
                }
            }
            Event::Start(Tag::TableCell) => {
                in_table_cell = true;
                buffer.clear();
            }
            Event::End(TagEnd::TableCell) => {
                if in_table_cell {
                    in_table_cell = false;
                    current_row.push(buffer.trim().to_string());
                }
                buffer.clear();
            }
            Event::Start(Tag::Paragraph) => {
                if list_stack.is_empty() {
                    in_paragraph = true;
                    buffer.clear();
                }
            }
            Event::End(TagEnd::Paragraph) => {
                if in_paragraph {
                    in_paragraph = false;
                    let text: String = buffer.trim().to_string();
                    if !text.is_empty() {
                        if blockquote_depth > 0 {
                            blocks.push(DocBlock::BlockQuote(text));
                        } else {
                            blocks.push(DocBlock::Paragraph(text));
                        }
                    }
                    buffer.clear();
                } else if let Some(top) = list_stack.last_mut()
                    && !top.is_empty()
                {
                    top.push(' ');
                }
            }
            Event::Start(Tag::Item) => {
                if let Some(parent_text) = list_stack.last_mut() {
                    let trimmed: String = parent_text.trim().to_string();
                    if !trimmed.is_empty() {
                        blocks.push(DocBlock::ListItem(trimmed));
                        parent_text.clear();
                    }
                }
                list_stack.push(String::new());
            }
            Event::End(TagEnd::Item) => {
                if let Some(item_buf) = list_stack.pop() {
                    let text: String = item_buf.trim().to_string();
                    if !text.is_empty() {
                        blocks.push(DocBlock::ListItem(text));
                    }
                }
            }
            Event::Start(Tag::CodeBlock(kind)) => {
                let lang: String = match kind {
                    CodeBlockKind::Fenced(tag) => {
                        let trimmed: &str = tag.split_whitespace().next().unwrap_or("");
                        if trimmed.is_empty() {
                            DEFAULT_CODE_LANG.to_string()
                        } else {
                            trimmed.to_string()
                        }
                    }
                    CodeBlockKind::Indented => DEFAULT_CODE_LANG.to_string(),
                };
                active_code_lang = Some(lang);
                buffer.clear();
            }
            Event::End(TagEnd::CodeBlock) => {
                if let Some(language) = active_code_lang.take() {
                    let code: String = buffer.trim_end().to_string();
                    blocks.push(DocBlock::Code { language, code });
                }
                buffer.clear();
            }
            Event::Start(Tag::Image { dest_url, .. }) => {
                active_image_src = Some(dest_url.into_string());
                image_alt_buf.clear();
            }
            Event::End(TagEnd::Image) => {
                if let Some(src) = active_image_src.take() {
                    let alt: String = take(&mut image_alt_buf).trim().to_string();
                    if !src.is_empty() {
                        blocks.push(DocBlock::Image { alt, src });
                    }
                }
            }
            Event::TaskListMarker(checked) => {
                let marker: &str = if checked { "[x] " } else { "[ ] " };
                if let Some(top) = list_stack.last_mut() {
                    top.push_str(marker);
                } else {
                    buffer.push_str(marker);
                }
            }
            Event::Rule => {
                blocks.push(DocBlock::HorizontalRule);
            }
            Event::Text(text) | Event::Code(text) => {
                if active_image_src.is_some() {
                    image_alt_buf.push_str(&text);
                } else if !list_stack.is_empty()
                    && active_code_lang.is_none()
                    && active_heading.is_none()
                    && !in_table_cell
                {
                    if let Some(top) = list_stack.last_mut() {
                        top.push_str(&text);
                    }
                } else {
                    buffer.push_str(&text);
                }
            }
            Event::SoftBreak | Event::HardBreak => {
                let sep: char = if active_code_lang.is_some() {
                    '\n'
                } else {
                    ' '
                };
                if active_image_src.is_some() {
                    image_alt_buf.push(sep);
                } else if !list_stack.is_empty()
                    && active_code_lang.is_none()
                    && active_heading.is_none()
                    && !in_table_cell
                {
                    if let Some(top) = list_stack.last_mut() {
                        top.push(sep);
                    }
                } else {
                    buffer.push(sep);
                }
            }
            _ => {}
        }
    }
    let DocFrontmatter {
        title: fm_title,
        order: fm_order,
        tagline: fm_tagline,
        section: fm_section,
        ..
    } = frontmatter;
    let fallback_title: String = first_h1_title
        .or_else(|| {
            toc.first()
                .map(|item: &DocTocItem| item.get_title().clone())
        })
        .unwrap_or_else(|| title_from_route(route));
    let title: String = fm_title.unwrap_or(fallback_title);
    if toc
        .first()
        .is_some_and(|item: &DocTocItem| item.get_title().trim().eq_ignore_ascii_case(title.trim()))
    {
        let _: DocTocItem = toc.remove(0);
    }
    let default_order: i32 = if route == ROUTE_ROOT { -1000 } else { 100 };
    let order: i32 = fm_order.unwrap_or(default_order);
    let section: String = fm_section.unwrap_or_else(|| default_section.to_string());
    let summary: String = fm_tagline.unwrap_or_else(|| {
        blocks
            .iter()
            .find_map(|b: &DocBlock| match b {
                DocBlock::Paragraph(p) | DocBlock::BlockQuote(p) => Some(p.clone()),
                _ => None,
            })
            .unwrap_or_default()
    });
    let mut page: DocPage = DocPage::new(
        route.to_string(),
        title,
        order,
        section,
        summary,
        blocks,
        toc,
    );
    page.set_markdown_body(body.to_string());
    page
}

/// Recursively loads all `.md` files under `root_dir` into a sorted `DocSite`.
///
/// # Arguments
///
/// - `&Path` - Root Markdown or project directory.
///
/// # Returns
///
/// - `io::Result<DocSite>` - Loaded documentation site or I/O error.
pub fn load_doc_site(root_dir: &Path) -> io::Result<DocSite> {
    load_doc_site_with_options(root_dir, None)
}

/// Recursively loads all `.md` files under `root_dir` with optional site title override and smart directory filtering.
///
/// # Arguments
///
/// - `&Path` - Root Markdown or project directory.
/// - `Option<&str>` - Optional explicit site title overriding frontmatter and folder heuristics.
///
/// # Returns
///
/// - `io::Result<DocSite>` - Loaded documentation site or I/O error.
pub fn load_doc_site_with_options(
    root_dir: &Path,
    title_override: Option<&str>,
) -> io::Result<DocSite> {
    load_doc_site_excluding(root_dir, title_override, None)
}

/// Recursively loads all `.md` files under `root_dir`, skipping `exclude_dir` when it sits inside `root_dir`.
///
/// # Arguments
///
/// - `&Path` - Root Markdown or project directory.
/// - `Option<&str>` - Optional explicit site title override.
/// - `Option<&Path>` - Optional output directory excluded from traversal.
///
/// # Returns
///
/// - `io::Result<DocSite>` - Loaded and section-ordered documentation site or I/O error.
pub(crate) fn load_doc_site_excluding(
    root_dir: &Path,
    title_override: Option<&str>,
    exclude_dir: Option<&Path>,
) -> io::Result<DocSite> {
    if !root_dir.exists() {
        return Err(Error::new(
            ErrorKind::NotFound,
            root_dir.display().to_string(),
        ));
    }
    let canonical_exclude: Option<PathBuf> =
        exclude_dir.map(|p: &Path| fs::canonicalize(p).unwrap_or_else(|_| p.to_path_buf()));
    let mut files: Vec<PathBuf> = Vec::new();
    collect_markdown_files(root_dir, canonical_exclude.as_deref(), &mut files)?;
    files.sort();
    let mut pages: Vec<DocPage> = Vec::new();
    let mut seen_routes: BTreeSet<String> = BTreeSet::new();
    let mut detected_title: Option<String> = None;
    let mut detected_tagline: Option<String> = None;
    for file_path in files.into_iter() {
        let raw: String = fs::read_to_string(&file_path)?;
        let rel: &Path = file_path.strip_prefix(root_dir).unwrap_or(&file_path);
        let (mut route, section): (String, String) = route_and_section_from_rel_path(rel);
        if route == ROUTE_ROOT {
            let (fm, _): (DocFrontmatter, &str) = split_frontmatter(&raw);
            if let Some(site_cfg) = fm.site {
                if detected_title.is_none() {
                    detected_title = site_cfg.try_get_title().clone();
                }
                if detected_tagline.is_none() {
                    detected_tagline = site_cfg.try_get_description().clone();
                }
            }
        }
        if seen_routes.contains(&route) {
            let stem: String = file_path
                .file_stem()
                .map(|s: &OsStr| slugify(&s.to_string_lossy()))
                .filter(|s: &String| !s.is_empty())
                .unwrap_or_else(|| FALLBACK_SLUG_PREFIX.to_string());
            let base_route: String = if route == ROUTE_ROOT {
                format!("/{stem}")
            } else {
                format!("{route}-{stem}")
            };
            route = base_route.clone();
            let mut suffix: usize = 2;
            while seen_routes.contains(&route) {
                route = format!("{base_route}-{suffix}");
                suffix += 1;
            }
        }
        let _: bool = seen_routes.insert(route.clone());
        pages.push(parse_markdown_page(&route, &section, &raw));
    }
    pages.sort_by(|a: &DocPage, b: &DocPage| {
        section_sort_rank(a.get_section())
            .cmp(&section_sort_rank(b.get_section()))
            .then_with(|| a.get_section().cmp(b.get_section()))
            .then_with(|| a.get_order().cmp(&b.get_order()))
            .then_with(|| a.get_route().cmp(b.get_route()))
    });
    let fallback_site_title: String = root_dir
        .file_name()
        .map(|s: &OsStr| s.to_string_lossy().into_owned())
        .filter(|raw: &String| {
            let low: String = raw.to_ascii_lowercase();
            raw.len() > 2
                && low != GENERIC_DOCS_DIR
                && low != GENERIC_SRC_DIR
                && raw != "."
                && raw != ".."
        })
        .map(|raw: String| format_section_label(&raw))
        .unwrap_or_else(|| DOCS_SITE_TITLE.to_string());
    let site_title: String = title_override
        .map(str::to_string)
        .or(detected_title)
        .unwrap_or(fallback_site_title);
    let site_tagline: String = detected_tagline.unwrap_or_else(|| DOCS_SITE_TAGLINE.to_string());
    Ok(DocSite::new(site_title, site_tagline, pages))
}

/// Returns the canonical sidebar and pagination ordering rank for a section label.
///
/// # Arguments
///
/// - `&str` - Section display label.
///
/// # Returns
///
/// - `u8` - Priority rank (`0` General, `1` Guide, `2` all other sections alphabetically).
pub(crate) fn section_sort_rank(section: &str) -> u8 {
    if section.eq_ignore_ascii_case(DEFAULT_SECTION) {
        0
    } else if section.eq_ignore_ascii_case(GUIDE_SECTION) {
        1
    } else {
        2
    }
}

/// Extracts optional YAML frontmatter delimited by `---` lines.
///
/// # Arguments
///
/// - `&str` - Full raw Markdown document.
///
/// # Returns
///
/// - `(DocFrontmatter, &str)` - Parsed frontmatter and remaining Markdown body slice.
fn split_frontmatter(raw: &str) -> (DocFrontmatter, &str) {
    let trimmed: &str = raw.trim_start_matches('\u{feff}');
    let Some(rest) = trimmed.strip_prefix(FRONTMATTER_DELIMITER) else {
        return (DocFrontmatter::default(), trimmed);
    };
    let rest: &str = rest
        .strip_prefix("\r\n")
        .or_else(|| rest.strip_prefix('\n'))
        .unwrap_or(rest);
    let mut offset: usize = 0;
    for line in rest.split_inclusive('\n') {
        let clean: &str = line.trim_end_matches(['\r', '\n']);
        if clean.trim() == FRONTMATTER_DELIMITER {
            let yaml_slice: &str = &rest[..offset];
            let body_slice: &str = &rest[(offset + line.len())..];
            let parsed: DocFrontmatter = from_str::<DocFrontmatter>(yaml_slice).unwrap_or_default();
            return (parsed, body_slice);
        }
        offset += line.len();
    }
    (DocFrontmatter::default(), trimmed)
}

/// Converts a `pulldown_cmark::HeadingLevel` into its `u8` depth (`1..=6`).
///
/// # Arguments
///
/// - `HeadingLevel` - Heading level enum.
///
/// # Returns
///
/// - `u8` - Numeric heading depth.
pub(crate) fn heading_level_num(level: HeadingLevel) -> u8 {
    match level {
        HeadingLevel::H1 => 1,
        HeadingLevel::H2 => 2,
        HeadingLevel::H3 => 3,
        HeadingLevel::H4 => 4,
        HeadingLevel::H5 => 5,
        HeadingLevel::H6 => 6,
    }
}

/// Produces a lowercase Unicode-safe URL anchor slug from heading text or path segments.
///
/// # Arguments
///
/// - `&str` - Heading text or path segment.
///
/// # Returns
///
/// - `String` - Hyphenated URL slug.
pub(crate) fn slugify(input: &str) -> String {
    let mut out: String = String::new();
    let mut prev_dash: bool = false;
    for ch in input.chars() {
        if ch.is_alphanumeric() {
            for lower in ch.to_lowercase() {
                out.push(lower);
            }
            prev_dash = false;
        } else if !prev_dash && !out.is_empty() {
            out.push('-');
            prev_dash = true;
        }
    }
    out.trim_end_matches('-').to_string()
}

/// Ensures every heading anchor slug within a single page is unique (`slug`, `slug-2`, ...).
///
/// # Arguments
///
/// - `String` - Candidate slug string.
/// - `&mut BTreeMap<String, usize>` - Per-page occurrence map.
///
/// # Returns
///
/// - `String` - Unique anchor slug.
pub(crate) fn deduplicate_slug(raw_slug: String, seen: &mut BTreeMap<String, usize>) -> String {
    let base: String = if raw_slug.is_empty() {
        FALLBACK_SLUG_PREFIX.to_string()
    } else {
        raw_slug
    };
    let count: &mut usize = seen.entry(base.clone()).or_insert(0);
    *count += 1;
    if *count == 1 {
        base
    } else {
        format!("{base}-{}", *count)
    }
}

/// Recursively collects `.md` file paths beneath `dir`, skipping build artifacts, hidden folders, and `exclude_dir`.
///
/// # Arguments
///
/// - `&Path` - Directory to walk.
/// - `Option<&Path>` - Optional canonical output directory to skip.
/// - `&mut Vec<PathBuf>` - Output path accumulator.
///
/// # Returns
///
/// - `io::Result<()>` - Traversal result.
fn collect_markdown_files(
    dir: &Path,
    exclude_dir: Option<&Path>,
    out: &mut Vec<PathBuf>,
) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry_val: DirEntry = entry?;
        let path: PathBuf = entry_val.path();
        if path.is_dir() {
            let name: String = entry_val.file_name().to_string_lossy().into_owned();
            if should_skip_dir(&name) {
                continue;
            }
            if let Some(excluded) = exclude_dir {
                let candidate: PathBuf = fs::canonicalize(&path).unwrap_or_else(|_| path.clone());
                if candidate == excluded || candidate.starts_with(excluded) {
                    continue;
                }
            }
            collect_markdown_files(&path, exclude_dir, out)?;
        } else if path
            .extension()
            .and_then(|s: &OsStr| s.to_str())
            .is_some_and(|ext: &str| ext.eq_ignore_ascii_case(MARKDOWN_EXT))
        {
            out.push(path);
        }
    }
    Ok(())
}

/// Returns `true` when directory `name` is an ignored build, cache, or hidden metadata directory.
///
/// # Arguments
///
/// - `&str` - Directory base name.
///
/// # Returns
///
/// - `bool` - Whether traversal should skip this directory.
pub(crate) fn should_skip_dir(name: &str) -> bool {
    name.starts_with('.')
        || IGNORED_SCAN_DIRS
            .iter()
            .any(|ignored: &&str| name.eq_ignore_ascii_case(ignored))
}

/// Derives canonical route and formatted section label from a relative Markdown path.
///
/// # Arguments
///
/// - `&Path` - Relative path inside the docs root directory.
///
/// # Returns
///
/// - `(String, String)` - Canonical route path and section label.
fn route_and_section_from_rel_path(rel: &Path) -> (String, String) {
    let parts: Vec<String> = rel
        .components()
        .map(|c: Component<'_>| c.as_os_str().to_string_lossy().into_owned())
        .collect();
    if parts.is_empty() {
        return (ROUTE_ROOT.to_string(), DEFAULT_SECTION.to_string());
    }
    let section: String = if parts.len() > 1 {
        format_section_label(&parts[0])
    } else {
        DEFAULT_SECTION.to_string()
    };
    let stem: String = rel
        .file_stem()
        .map(|s: &OsStr| s.to_string_lossy().into_owned())
        .unwrap_or_default();
    let mut route_segments: Vec<String> = parts[..parts.len().saturating_sub(1)]
        .iter()
        .map(|seg: &String| {
            let cleaned: String = slugify(seg);
            if cleaned.is_empty() {
                seg.to_ascii_lowercase()
            } else {
                cleaned
            }
        })
        .collect();
    if !stem.eq_ignore_ascii_case(README_STEM) && !stem.eq_ignore_ascii_case(INDEX_STEM) {
        let cleaned_stem: String = slugify(&stem);
        route_segments.push(if cleaned_stem.is_empty() {
            stem.to_ascii_lowercase()
        } else {
            cleaned_stem
        });
    }
    let route: String = if route_segments.is_empty() {
        ROUTE_ROOT.to_string()
    } else {
        format!("/{}", route_segments.join("/"))
    };
    (route, section)
}

/// Formats a directory or filename slug into a readable Title-Case label.
///
/// # Arguments
///
/// - `&str` - Raw path segment or filename stem.
///
/// # Returns
///
/// - `String` - Formatted display label.
fn format_section_label(raw: &str) -> String {
    let mut words: Vec<String> = Vec::new();
    for part in raw.split(['-', '_']) {
        let trimmed: &str = part.trim();
        if trimmed.is_empty() {
            continue;
        }
        let mut chars: Chars<'_> = trimmed.chars();
        if let Some(first) = chars.next() {
            let mut word: String = String::new();
            for up in first.to_uppercase() {
                word.push(up);
            }
            word.push_str(chars.as_str());
            words.push(word);
        }
    }
    if words.is_empty() {
        DEFAULT_SECTION.to_string()
    } else {
        words.join(" ")
    }
}

/// Derives a human-readable fallback page title from a canonical route string.
///
/// # Arguments
///
/// - `&str` - Canonical route path.
///
/// # Returns
///
/// - `String` - Display title fallback.
fn title_from_route(route: &str) -> String {
    let trimmed: &str = route.trim_matches('/');
    if trimmed.is_empty() {
        return DEFAULT_DOC_TITLE.to_string();
    }
    trimmed
        .rsplit('/')
        .next()
        .map(format_section_label)
        .unwrap_or_else(|| DEFAULT_DOC_TITLE.to_string())
}
