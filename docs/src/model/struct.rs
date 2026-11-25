use super::*;

/// Parsed YAML frontmatter metadata attached to a Markdown documentation file.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DocFrontmatter {
    /// Optional document title.
    pub(crate) title: Option<String>,
    /// Optional numeric sidebar sort order.
    pub(crate) order: Option<i32>,
    /// Optional tagline for home/landing pages.
    pub(crate) tagline: Option<String>,
    /// Optional section group name.
    pub(crate) section: Option<String>,
}

/// Table-of-contents heading entry derived from a parsed document.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Getter, New)]
pub struct DocTocItem {
    /// Heading depth from 1 to 6.
    #[get(type(copy))]
    pub(crate) level: u8,
    /// Anchor slug for jump navigation.
    pub(crate) slug: String,
    /// Display title text.
    pub(crate) title: String,
}

/// Fully parsed documentation page ready for native rendering or static generation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Getter, New)]
pub struct DocPage {
    /// Canonical route path (for example `/guide/getting-started`).
    pub(crate) route: String,
    /// Resolved page title.
    pub(crate) title: String,
    /// Sidebar sort order.
    #[get(type(copy))]
    pub(crate) order: i32,
    /// Section category label.
    pub(crate) section: String,
    /// Short summary or tagline.
    pub(crate) summary: String,
    /// Structured Markdown content blocks.
    pub(crate) blocks: Vec<DocBlock>,
    /// Table-of-contents headings.
    pub(crate) toc: Vec<DocTocItem>,
}

/// Collection of parsed documentation pages forming a complete site.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Getter, New)]
pub struct DocSite {
    /// Site brand title.
    pub(crate) title: String,
    /// Site description tagline.
    pub(crate) tagline: String,
    /// Ordered documentation pages.
    pub(crate) pages: Vec<DocPage>,
}
