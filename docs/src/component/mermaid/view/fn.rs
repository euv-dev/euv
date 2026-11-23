use super::*;

/// Renders a mermaid diagram as inline SVG.
///
/// The diagram is rendered at build time by `merman`, a pure-Rust
/// mermaid engine, so the published site ships no JavaScript runtime
/// and no `.js` or `.css` asset: the SVG is produced while the page
/// data is generated and inlined into the page.
///
/// The colours come from a scoped stylesheet the engine appends to the
/// SVG itself. Every colour it names is a docs theme token, and CSS
/// custom properties inherit into an inline SVG, so the diagram follows
/// the root theme class — light or dark — with no re-render and no
/// `!important`.
///
/// A diagram the engine could not lay out never reaches this
/// component: the build keeps the ordinary code block instead, so the
/// source stays readable.
///
/// # Arguments
///
/// - `VirtualNode<DocsMermaidProps>` - The props node carrying the
///   rendered SVG of the diagram.
///
/// # Returns
///
/// - `VirtualNode` - The diagram virtual DOM tree.
#[component]
pub(crate) fn docs_mermaid(node: VirtualNode<DocsMermaidProps>) -> VirtualNode {
    let DocsMermaidProps { svg }: DocsMermaidProps = node.try_get_props().unwrap_or_default();
    html! {
        figure {
            class: c_docs_mermaid()
            div {
                class: c_docs_mermaid_frame()
                inner_html: svg
            }
        }
    }
}
