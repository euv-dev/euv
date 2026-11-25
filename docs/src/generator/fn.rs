use super::*;

/// Generates static HTML pages and a JSON search index for `site` into `out_dir`.
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
    fs::create_dir_all(out_dir)?;
    let generated_at: String = Utc::now().to_rfc3339();
    for page in site.get_pages().iter() {
        let html: String = render_page_html(site, page, &generated_at);
        let rel_file: PathBuf = html_path_for_route(out_dir, page.get_route());
        if let Some(parent) = rel_file.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&rel_file, html)?;
    }
    let index_path: PathBuf = out_dir.join(INDEX_JSON_NAME);
    let json: String = match to_string_pretty(site) {
        Ok(text) => text,
        Err(err) => return Err(Error::other(err.to_string())),
    };
    fs::write(&index_path, json)?;
    Ok(GeneratedSiteReport::new(
        site.get_pages().len(),
        generated_at,
        index_path.display().to_string(),
    ))
}

/// Renders a single `DocPage` into a self-contained static HTML document.
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
    let mut out: String = String::new();
    out.push_str(HTML_DOCTYPE);
    out.push_str(&escape_html(page.get_title()));
    out.push_str(" - ");
    out.push_str(&escape_html(site.get_title()));
    out.push_str(HTML_HEAD_CLOSE);
    out.push_str(HTML_NAV_OPEN);
    for nav_page in site.get_pages().iter().take(12) {
        let href: String = if nav_page.get_route() == "/" {
            INDEX_HTML_NAME.to_string()
        } else {
            format!("{}{HTML_EXT}", nav_page.get_route().trim_start_matches('/'))
        };
        out.push_str(HTML_LINK_PREFIX);
        out.push_str(&escape_html(&href));
        out.push_str(HTML_LINK_MID);
        out.push_str(&escape_html(nav_page.get_title()));
        out.push_str(HTML_LINK_CLOSE);
    }
    out.push_str(HTML_NAV_CLOSE_MAIN_H1);
    out.push_str(&escape_html(page.get_title()));
    out.push_str(HTML_H1_CLOSE);
    for block in page.get_blocks().iter() {
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
                out.push_str("<p>");
                out.push_str(&escape_html(text));
                out.push_str(HTML_P_CLOSE);
            }
            DocBlock::ListItem(text) => {
                out.push_str(HTML_LI_OPEN);
                out.push_str(&escape_html(text));
                out.push_str(HTML_LI_CLOSE);
            }
            DocBlock::Code { language, code } => {
                out.push_str(HTML_CODE_OPEN);
                out.push_str(&escape_html(language));
                out.push_str(HTML_LINK_MID);
                out.push_str(&escape_html(code));
                out.push_str(HTML_CODE_CLOSE);
            }
        }
    }
    out.push_str(HTML_MAIN_CLOSE);
    out.push_str(HTML_FOOTER_PREFIX);
    out.push_str(&escape_html(generated_at));
    out.push_str(HTML_FOOTER_SUFFIX);
    out
}

/// Computes the filesystem output HTML path for a canonical documentation route.
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
