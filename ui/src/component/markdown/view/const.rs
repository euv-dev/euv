/// Prose typography class carried by the root `<article>` of a rendered
/// markdown document.
pub(crate) const MD_BODY_PROSE_CLASS: &str = "md-body";

/// Class prefix concatenated with a fenced code block's language token so a
/// highlighter can pick the grammar up from the class name alone.
pub(crate) const MD_CODE_LANG_CLASS_PREFIX: &str = "language-";

/// Class prefix for a directive container block; the trailing space separates
/// it from the concatenated container kind.
pub(crate) const MD_DOCS_CONTAINER_CLASS_PREFIX: &str = "docs-container ";

/// Heading class of the title line rendered inside a container block.
pub(crate) const MD_DOCS_CONTAINER_TITLE_CLASS: &str = "docs-container-title";

/// Class marking the `#` permalink anchor appended to every heading.
pub(crate) const MD_HEADING_ANCHOR_CLASS: &str = "header-anchor";

/// Attribute flipped on an image once it has settled (loaded or failed).
///
/// The stylesheet keeps a not-yet-loaded image at `height: 0` so a slow
/// image cannot flash at the wrong size and then reflow the page under
/// the reader. Before this was an injected script plus a
/// `MutationObserver` watching the whole body; the renderer marks the
/// image itself instead, which is both cheaper and immune to the
/// observer missing an image that mounted in the same tick.
pub(crate) const MD_IMAGE_LOADED_ATTR: &str = "data-loaded";

/// Value written to [`MD_IMAGE_LOADED_ATTR`]; presence is what CSS matches.
pub(crate) const MD_IMAGE_LOADED_VALUE: &str = "1";

/// Class on the glyph span of a GFM task-list item, checked or unchecked.
pub(crate) const MD_TASK_MARKER_CLASS: &str = "task-marker";
