use super::*;

class! {
    // The prose column of the article: it fills the width of the
    // content column and is allowed to shrink inside it, so a wide
    // table scrolls instead of pushing the article wide.
    pub c_docs_markdown {
        width: "100%";
        min-width: "0px";
    }
}

/// The prose class the shared typography stylesheet keys on.
///
/// It is the same class euv-ui's own markdown renderer puts on its
/// article, so the docs body keeps the shared prose rules.
pub(crate) const PROSE_CLASS: &str = "md-body";

/// The class prefix of a directive container block.
///
/// The trailing space separates the prefix from the container kind, so
/// a `tip` container carries the classes `docs-container` and `tip` —
/// the same pair the shared markdown renderer produces.
pub(crate) const CONTAINER_CLASS_PREFIX: &str = "docs-container ";

/// The class of the title line rendered inside a container block.
pub(crate) const CONTAINER_TITLE_CLASS: &str = "docs-container-title";

/// The fence language of a mermaid diagram.
pub(crate) const MERMAID_LANG: &str = "mermaid";
