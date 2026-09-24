use super::*;

/// A generic documentation page layout aligned with common site frameworks
/// (VuePress default theme `Page`, Docusaurus doc item): a fluid content
/// column with the page body as children, `euv_pagination` prev/next links
/// and an optional footer, plus a sticky right `euv_toc` anchor column that
/// collapses on narrow viewports.
///
/// # Arguments
///
/// - `VirtualNode<EuvDocLayoutProps>` - The props node.
///
/// # Returns
///
/// - `VirtualNode` - The doc page layout virtual DOM tree.
#[component]
pub fn euv_doc_layout(node: VirtualNode<EuvDocLayoutProps>) -> VirtualNode {
    let EuvDocLayoutProps {
        toc_title,
        toc_items,
        prev_label,
        next_label,
        prev,
        next,
        footer,
    }: EuvDocLayoutProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            class: c_euv_doc_layout()
            div {
                class: c_euv_doc_content()
                // Two parallel children:
                //   1. the page body (article) — grows to its natural height;
                //   2. a wrapper containing pagination + footer — sits beside
                //      the body. Combined with `justify-content: space-between`
                //      on this flex column (see `c_euv_doc_content`), the
                //      wrapper drops to the bottom of the scroll area when the
                //      body is short, and follows the body in normal flow
                //      when the body is long enough to fill the viewport.
                children
                div {
                    class: c_euv_doc_tail()
                    euv_pagination {
                        prev_label
                        next_label
                        prev
                        next
                    }
                    if { !footer.is_empty() } {
                        footer {
                            class: c_euv_footer()
                            {
                                footer
                            }
                        }
                    }
                }
            }
            if { !toc_items.is_empty() } {
                div {
                    class: c_euv_doc_toc()
                    euv_toc {
                        title: toc_title
                        items: toc_items
                    }
                }
            }
        }
    }
}
