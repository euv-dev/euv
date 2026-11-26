use super::*;

/// Optional site-level metadata declared under `site:` in root Markdown frontmatter.
#[derive(Clone, Debug, Default, Deserialize, Getter, PartialEq, Serialize)]
pub struct DocSiteFrontmatter {
    /// Optional site display title.
    pub(crate) title: Option<String>,
    /// Optional site summary tagline.
    pub(crate) description: Option<String>,
    /// Optional brand logo or monogram text.
    pub(crate) logo: Option<String>,
    /// Optional footer attribution line.
    pub(crate) footer: Option<String>,
}

/// Parsed YAML frontmatter metadata attached to a Markdown documentation file.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DocFrontmatter {
    /// Optional document title.
    pub(crate) title: Option<String>,
    /// Optional numeric sidebar sort order.
    pub(crate) order: Option<i32>,
    /// Optional tagline for home/landing pages.
    pub(crate) tagline: Option<String>,
    /// Optional section group name.
    pub(crate) section: Option<String>,
    /// Optional site-level configuration block.
    pub(crate) site: Option<DocSiteFrontmatter>,
}

/// Compact search index record serialized into `search-index.json` and embedded client search.
#[derive(Clone, Debug, Deserialize, Eq, Getter, New, PartialEq, Serialize)]
pub struct SearchIndexEntry {
    /// Canonical route path for navigation.
    pub(crate) route: String,
    /// Document title.
    pub(crate) title: String,
    /// Section category label.
    pub(crate) section: String,
    /// Short page description or lead excerpt.
    pub(crate) summary: String,
    /// Concatenated heading titles for keyword matching.
    pub(crate) headings: Vec<String>,
    /// Plain-text searchable body excerpt.
    pub(crate) excerpt: String,
}

/// Table-of-contents heading entry derived from a parsed document.
#[derive(Clone, Debug, Deserialize, Getter, New, PartialEq, Serialize)]
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
#[derive(Clone, Debug, Deserialize, Getter, New, PartialEq, Serialize)]
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
    /// Preserved Markdown body text (after frontmatter) for full-fidelity HTML generation.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    #[get(pub(crate))]
    #[new(skip)]
    pub(crate) markdown_body: String,
}

/// Collection of parsed documentation pages forming a complete site.
#[derive(Clone, Debug, Deserialize, Getter, New, PartialEq, Serialize)]
pub struct DocSite {
    /// Site brand title.
    pub(crate) title: String,
    /// Site description tagline.
    pub(crate) tagline: String,
    /// Ordered documentation pages.
    pub(crate) pages: Vec<DocPage>,
}
