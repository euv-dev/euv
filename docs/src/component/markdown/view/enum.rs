/// How a fenced code block is rendered.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CodeBlockRoute {
    /// The fence carries a diagram, rendered as an inline SVG.
    Diagram,
    /// The fence carries source code, rendered as a code block.
    Code,
}
