/// Top-level Markdown body segment distinguishing plain Markdown from VuePress `:::` callout blocks.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum BodySegment {
    /// Standard Markdown chunk rendered directly through `pulldown_cmark`.
    Markdown(String),
    /// Custom callout or expandable `<details>` container block.
    Callout {
        /// Normalized callout kind (`tip`, `warning`, `danger`, `info`, `note`, or `details`).
        kind: String,
        /// Display header or summary title for the callout.
        title: String,
        /// Inner Markdown body rendered inside the container.
        body: String,
    },
}
