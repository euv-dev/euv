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
}

/// Query matching helpers for `DocPage`.
impl DocPage {
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
                DocBlock::Paragraph(text) | DocBlock::ListItem(text) => {
                    text.to_ascii_lowercase().contains(needle)
                }
                DocBlock::Code { code, .. } => code.to_ascii_lowercase().contains(needle),
            })
    }
}
