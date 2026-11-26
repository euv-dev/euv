use super::*;

/// Search and lookup utilities for `DocSite`.
impl DocSite {
    /// Searches pages matching `query` (case-insensitive) across title, route, and content blocks.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Documentation site reference.
    /// - `&str` - Search query text.
    ///
    /// # Returns
    ///
    /// - `Vec<&DocPage>` - Matching pages in display order.
    #[must_use]
    pub fn search(&self, query: &str) -> Vec<&DocPage> {
        let needle: String = query.trim().to_ascii_lowercase();
        if needle.is_empty() {
            return self.get_pages().iter().collect();
        }
        self.get_pages()
            .iter()
            .filter(|page: &&DocPage| page.matches_query(&needle))
            .collect()
    }

    /// Looks up a page by its canonical route path.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Documentation site reference.
    /// - `&str` - Route path to locate.
    ///
    /// # Returns
    ///
    /// - `Option<&DocPage>` - Matching page if present.
    #[must_use]
    pub fn find_by_route(&self, route: &str) -> Option<&DocPage> {
        self.get_pages()
            .iter()
            .find(|page: &&DocPage| page.get_route() == route)
    }

    /// Builds compact full-text search index entries for every page in this site.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Documentation site reference.
    ///
    /// # Returns
    ///
    /// - `Vec<SearchIndexEntry>` - Ordered search records ready for JSON serialization.
    #[must_use]
    pub fn search_entries(&self) -> Vec<SearchIndexEntry> {
        self.get_pages()
            .iter()
            .map(|page: &DocPage| {
                let headings: Vec<String> = page
                    .get_toc()
                    .iter()
                    .map(|item: &DocTocItem| item.get_title().clone())
                    .collect();
                let mut excerpt: String = String::new();
                for block in page.get_blocks().iter() {
                    if excerpt.len() >= 360 {
                        break;
                    }
                    match block {
                        DocBlock::Paragraph(text)
                        | DocBlock::ListItem(text)
                        | DocBlock::BlockQuote(text) => {
                            if !excerpt.is_empty() {
                                excerpt.push(' ');
                            }
                            excerpt.push_str(text);
                        }
                        DocBlock::Table { headers, rows } => {
                            for h in headers.iter() {
                                if !excerpt.is_empty() {
                                    excerpt.push(' ');
                                }
                                excerpt.push_str(h);
                            }
                            for r in rows.iter().take(3) {
                                for cell in r.iter() {
                                    if !excerpt.is_empty() {
                                        excerpt.push(' ');
                                    }
                                    excerpt.push_str(cell);
                                }
                            }
                        }
                        DocBlock::Image { alt, .. } => {
                            if !alt.is_empty() {
                                if !excerpt.is_empty() {
                                    excerpt.push(' ');
                                }
                                excerpt.push_str(alt);
                            }
                        }
                        DocBlock::Heading { .. }
                        | DocBlock::Code { .. }
                        | DocBlock::HorizontalRule => {}
                    }
                }
                let trimmed: String = excerpt.chars().take(360).collect();
                SearchIndexEntry::new(
                    page.get_route().clone(),
                    page.get_title().clone(),
                    page.get_section().clone(),
                    page.get_summary().clone(),
                    headings,
                    trimmed,
                )
            })
            .collect()
    }
}

/// Query matching and body attachment helpers for `DocPage`.
impl DocPage {
    /// Stores the raw Markdown body slice for full-fidelity HTML generation.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable documentation page reference.
    /// - `String` - Raw Markdown body text without frontmatter.
    pub(crate) fn set_markdown_body(&mut self, body: String) {
        self.markdown_body = body;
    }

    /// Returns `true` if `needle` (already lowercased) matches the title, route, summary, or any block.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Documentation page reference.
    /// - `&str` - Lowercased search query.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the page matches.
    #[must_use]
    pub fn matches_query(&self, needle: &str) -> bool {
        if self.get_title().to_ascii_lowercase().contains(needle)
            || self.get_route().to_ascii_lowercase().contains(needle)
            || self.get_summary().to_ascii_lowercase().contains(needle)
        {
            return true;
        }
        self.get_blocks()
            .iter()
            .any(|block: &DocBlock| match block {
                DocBlock::Heading { text, .. } => text.to_ascii_lowercase().contains(needle),
                DocBlock::Paragraph(text)
                | DocBlock::ListItem(text)
                | DocBlock::BlockQuote(text) => text.to_ascii_lowercase().contains(needle),
                DocBlock::Code { code, .. } => code.to_ascii_lowercase().contains(needle),
                DocBlock::Image { alt, .. } => alt.to_ascii_lowercase().contains(needle),
                DocBlock::Table { headers, rows } => {
                    let in_headers: bool = headers
                        .iter()
                        .any(|h: &String| h.to_ascii_lowercase().contains(needle));
                    let in_rows: bool = rows.iter().any(|row: &Vec<String>| {
                        row.iter()
                            .any(|cell: &String| cell.to_ascii_lowercase().contains(needle))
                    });
                    in_headers || in_rows
                }
                DocBlock::HorizontalRule => false,
            })
    }
}
