use super::*;

/// A generic sticky anchor table-of-contents aligned with common docs
/// frameworks.
///
/// Renders nothing when `items` is empty. The caller controls the column
/// width and responsive visibility; the component provides the sticky inner
/// stack.
///
/// # Arguments
///
/// - `VirtualNode<EuvTocProps>` - The props node.
///
/// # Returns
///
/// - `VirtualNode` - The TOC virtual DOM tree.
#[component]
pub fn euv_toc(node: VirtualNode<EuvTocProps>) -> VirtualNode {
    let EuvTocProps {
        title,
        items,
        active,
    }: EuvTocProps = node.try_get_props().unwrap_or_default();
    if items.is_empty() {
        return html! {
            ""
        };
    }
    let active_href: String = active.get();
    // Computed outside the template: the `html!` parser has no grammar for
    // a `let` inside a `for` body, and the class choice is a plain lookup.
    let entry_classes: Vec<fn() -> &'static Css> = items
        .iter()
        .map(|item: &'static EuvTocItem| {
            if item.href == active_href {
                if item.level > TOC_NESTED_LEVEL {
                    c_euv_toc_link_nested_active
                } else {
                    c_euv_toc_link_active
                }
            } else if item.level > TOC_NESTED_LEVEL {
                c_euv_toc_link_nested
            } else {
                c_euv_toc_link
            }
        })
        .collect();
    html! {
        div {
            class: c_euv_toc()
            div {
                class: c_euv_toc_title()
                {
                    title
                }
            }
            for (item, entry_class) in items.iter().zip(entry_classes.iter()) {
                a {
                    key: item.href
                    class: entry_class()
                    href: item.href
                    {
                        item.text
                    }
                }
            }
        }
    }
}
