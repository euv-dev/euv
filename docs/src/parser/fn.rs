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
    let mut active_heading: Option<u8> = None;
    let mut in_paragraph: bool = false;
    let mut in_list_item: bool = false;
    let mut active_code_lang: Option<String> = None;
    let mut buffer: String = String::new();
    let parser: Parser<'_> = Parser::new_ext(body, Options::all());
    for event in parser {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                active_heading = Some(heading_level_num(level));
                buffer.clear();
            }
            Event::End(TagEnd::Heading(_)) => {
                if let Some(level) = active_heading.take() {
                    let text: String = buffer.trim().to_string();
                    if !text.is_empty() {
                        let slug: String = slugify(&text);
                        toc.push(DocTocItem::new(level, slug.clone(), text.clone()));
                        blocks.push(DocBlock::Heading { level, slug, text });
                    }
                }
                buffer.clear();
            }
            Event::Start(Tag::Paragraph) => {
                in_paragraph = true;
                buffer.clear();
            }
            Event::End(TagEnd::Paragraph) => {
                if in_paragraph {
                    in_paragraph = false;
                    let text: String = buffer.trim().to_string();
                    if !text.is_empty() {
                        blocks.push(DocBlock::Paragraph(text));
                    }
                }
                buffer.clear();
            }
            Event::Start(Tag::Item) => {
                in_list_item = true;
                buffer.clear();
            }
            Event::End(TagEnd::Item) => {
                if in_list_item {
                    in_list_item = false;
                    let text: String = buffer.trim().to_string();
                    if !text.is_empty() {
                        blocks.push(DocBlock::ListItem(text));
                    }
                }
                buffer.clear();
            }
            Event::Start(Tag::CodeBlock(kind)) => {
                let lang: String = match kind {
                    CodeBlockKind::Fenced(tag) => {
                        let trimmed: &str = tag.trim();
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
            Event::Text(text) | Event::Code(text) => {
                buffer.push_str(&text);
            }
            Event::SoftBreak | Event::HardBreak => {
                if active_code_lang.is_some() {
                    buffer.push('\n');
                } else {
                    buffer.push(' ');
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
    } = frontmatter;
    let fallback_title: String = toc
        .first()
        .map(|item: &DocTocItem| item.get_title().clone())
        .unwrap_or_else(|| DEFAULT_DOC_TITLE.to_string());
    let title: String = fm_title.unwrap_or(fallback_title);
    let order: i32 = fm_order.unwrap_or(100);
    let section: String = fm_section.unwrap_or_else(|| default_section.to_string());
    let summary: String = fm_tagline.unwrap_or_else(|| {
        blocks
            .iter()
            .find_map(|b: &DocBlock| match b {
                DocBlock::Paragraph(p) => Some(p.clone()),
                _ => None,
            })
            .unwrap_or_default()
    });
    DocPage::new(
        route.to_string(),
        title,
        order,
        section,
        summary,
        blocks,
        toc,
    )
}

/// Recursively loads all `.md` files under `root_dir` into a sorted `DocSite`.
///
/// # Arguments
///
/// - `&Path` - Root Markdown directory.
///
/// # Returns
///
/// - `io::Result<DocSite>` - Loaded documentation site or I/O error.
pub fn load_doc_site(root_dir: &Path) -> io::Result<DocSite> {
    if !root_dir.exists() {
        return Err(Error::new(
            ErrorKind::NotFound,
            root_dir.display().to_string(),
        ));
    }
    let mut files: Vec<PathBuf> = Vec::new();
    collect_markdown_files(root_dir, &mut files)?;
    files.sort();
    let mut pages: Vec<DocPage> = Vec::new();
    for file_path in files.into_iter() {
        let raw: String = fs::read_to_string(&file_path)?;
        let rel: &Path = file_path.strip_prefix(root_dir).unwrap_or(&file_path);
        let (route, section): (String, String) = route_and_section_from_rel_path(rel);
        pages.push(parse_markdown_page(&route, &section, &raw));
    }
    pages.sort_by(|a: &DocPage, b: &DocPage| {
        a.get_order()
            .cmp(&b.get_order())
            .then_with(|| a.get_route().cmp(b.get_route()))
    });
    Ok(DocSite::new(
        DOCS_SITE_TITLE.to_string(),
        DOCS_SITE_TAGLINE.to_string(),
        pages,
    ))
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
fn heading_level_num(level: HeadingLevel) -> u8 {
    match level {
        HeadingLevel::H1 => 1,
        HeadingLevel::H2 => 2,
        HeadingLevel::H3 => 3,
        HeadingLevel::H4 => 4,
        HeadingLevel::H5 => 5,
        HeadingLevel::H6 => 6,
    }
}

/// Produces a lowercase ASCII URL anchor slug from heading text.
///
/// # Arguments
///
/// - `&str` - Heading text.
///
/// # Returns
///
/// - `String` - Hyphenated URL slug.
fn slugify(input: &str) -> String {
    let mut out: String = String::new();
    let mut prev_dash: bool = false;
    for ch in input.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
            prev_dash = false;
        } else if !prev_dash && !out.is_empty() {
            out.push('-');
            prev_dash = true;
        }
    }
    out.trim_end_matches('-').to_string()
}

/// Recursively collects `.md` file paths beneath `dir`.
///
/// # Arguments
///
/// - `&Path` - Directory to walk.
/// - `&mut Vec<PathBuf>` - Output path accumulator.
///
/// # Returns
///
/// - `io::Result<()>` - Traversal result.
fn collect_markdown_files(dir: &Path, out: &mut Vec<PathBuf>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry_val: DirEntry = entry?;
        let path: PathBuf = entry_val.path();
        if path.is_dir() {
            collect_markdown_files(&path, out)?;
        } else if path.extension().and_then(|s: &OsStr| s.to_str()) == Some(MARKDOWN_EXT) {
            out.push(path);
        }
    }
    Ok(())
}

/// Derives canonical route and section label from a relative Markdown path.
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
        parts[0].clone()
    } else {
        DEFAULT_SECTION.to_string()
    };
    let stem: String = rel
        .file_stem()
        .map(|s: &OsStr| s.to_string_lossy().into_owned())
        .unwrap_or_default();
    let mut route_segments: Vec<String> = parts[..parts.len().saturating_sub(1)].to_vec();
    if !stem.eq_ignore_ascii_case(README_STEM) {
        route_segments.push(stem);
    }
    let route: String = if route_segments.is_empty() {
        ROUTE_ROOT.to_string()
    } else {
        format!("/{}", route_segments.join("/"))
    };
    (route, section)
}
