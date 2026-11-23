use super::*;

/// One block-level markdown node of the [`euv_markdown`] AST.
///
/// The tree is plain data so build tooling can generate it as `&'static`
/// values and the renderer can walk it without allocations.
#[derive(Clone, Copy, Debug)]
pub enum EuvMdBlock {
    /// `<h1>`–`<h6>`; `href` is the full permalink of the heading anchor.
    Heading {
        /// Heading level (1–6).
        level: u8,
        /// Slug id used for anchor scrolling.
        id: &'static str,
        /// Full permalink href.
        href: &'static str,
        /// Inline content.
        inline: &'static [EuvMdInline],
    },
    /// A paragraph.
    Paragraph(&'static [EuvMdInline]),
    /// A fenced code block.
    CodeBlock {
        /// Fence info string (language).
        lang: &'static str,
        /// Raw code.
        code: &'static str,
    },
    /// A diagram rendered to an inline SVG.
    ///
    /// The SVG is produced before the page data is generated — the
    /// diagram engine is a build-time dependency of the generator, not
    /// a runtime dependency of the browser bundle — so the browser only
    /// ever carries the finished markup.
    Diagram {
        /// The inline SVG markup of the diagram.
        svg: &'static str,
    },
    /// A block quote.
    BlockQuote(&'static [EuvMdBlock]),
    /// An ordered or unordered list.
    List {
        /// Ordered list flag.
        ordered: bool,
        /// List items.
        items: &'static [&'static [EuvMdBlock]],
    },
    /// A GFM table.
    Table {
        /// Header cells.
        head: &'static [&'static [EuvMdInline]],
        /// Body rows.
        rows: &'static [&'static [&'static [EuvMdInline]]],
    },
    /// A `:::` custom container.
    Container {
        /// Container kind (tip / warning / danger / …).
        kind: &'static str,
        /// Title label.
        title: &'static str,
        /// Inner blocks.
        blocks: &'static [EuvMdBlock],
    },
    /// A thematic break (`<hr>`).
    Rule,
    /// A raw HTML block (escape hatch, rendered via `inner_html`).
    Html(&'static str),
}

/// One inline markdown node of the [`euv_markdown`] AST.
#[derive(Clone, Copy, Debug)]
pub enum EuvMdInline {
    /// Plain text.
    Text(&'static str),
    /// Bold.
    Strong(&'static [EuvMdInline]),
    /// Italic.
    Em(&'static [EuvMdInline]),
    /// Strikethrough.
    Del(&'static [EuvMdInline]),
    /// Inline code.
    Code(&'static str),
    /// A link (internal route or external URL).
    Link {
        /// Resolved href.
        href: &'static str,
        /// External link flag (opens in a new tab).
        external: bool,
        /// Link text.
        children: &'static [EuvMdInline],
    },
    /// An image.
    Image {
        /// Image URL.
        src: &'static str,
        /// Alt text.
        alt: &'static str,
    },
    /// A task-list checkbox marker.
    TaskMarker(bool),
    /// A soft line break.
    SoftBreak,
    /// A hard line break.
    HardBreak,
    /// Raw inline HTML (escape hatch).
    Html(&'static str),
}
