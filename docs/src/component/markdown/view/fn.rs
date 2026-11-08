use super::*;

/// Renders the body of a documentation page.
///
/// The body is the shared markdown article, with one exception: the
/// code blocks are the docs' own, because they carry a toolbar and a
/// copy button that the shared renderer does not provide. Every other
/// block is handed to the shared renderer unchanged, so the prose keeps
/// the shared typography, the shared anchors and the shared behaviour.
///
/// The shared renderer is fed one block at a time — a one-element slice
/// of the page's own block list, which is as long-lived as the list
/// itself. A quote, a list or a container that holds a code block is
/// rendered here instead, so that the code block below it is the docs'
/// own.
///
/// # Arguments
///
/// - `&'static [EuvMdBlock]` - The blocks of the page.
///
/// # Returns
///
/// - `VirtualNode` - The rendered body.
pub(crate) fn docs_markdown(blocks: &'static [EuvMdBlock]) -> VirtualNode {
    html! {
        article {
            class: c_docs_markdown()
            class: PROSE_CLASS
            {
                render_docs_blocks(blocks)
            }
        }
    }
}

/// Renders a run of markdown blocks.
///
/// # Arguments
///
/// - `&'static [EuvMdBlock]` - The blocks to render.
///
/// # Returns
///
/// - `VirtualNode` - The rendered blocks.
fn render_docs_blocks(blocks: &'static [EuvMdBlock]) -> VirtualNode {
    html! {
        for (index, block) in blocks.iter().enumerate() {
            {
                render_docs_block(blocks, index, block)
            }
        }
    }
}

/// Renders one markdown block.
///
/// # Arguments
///
/// - `&'static [EuvMdBlock]` - The whole block list of the page.
/// - `usize` - The index of the block in that list.
/// - `&'static EuvMdBlock` - The block to render.
///
/// # Returns
///
/// - `VirtualNode` - The rendered block.
fn render_docs_block(
    blocks: &'static [EuvMdBlock],
    index: usize,
    block: &'static EuvMdBlock,
) -> VirtualNode {
    match block {
        EuvMdBlock::CodeBlock { lang, code } => html! {
            docs_code_block {
                lang: *lang
                code: *code
            }
        },
        EuvMdBlock::BlockQuote(inner) => {
            if contains_code(inner) {
                html! {
                    blockquote {
                        {
                            render_docs_blocks(inner)
                        }
                    }
                }
            } else {
                euv_ui::euv_markdown_blocks(&blocks[index..index + 1])
            }
        }
        EuvMdBlock::List { ordered, items } => {
            if items
                .iter()
                .any(|item: &&'static [EuvMdBlock]| contains_code(item))
            {
                if *ordered {
                    html! {
                        ol {
                            for item in items.iter() {
                                li {
                                    {
                                        render_docs_blocks(item)
                                    }
                                }
                            }
                        }
                    }
                } else {
                    html! {
                        ul {
                            for item in items.iter() {
                                li {
                                    {
                                        render_docs_blocks(item)
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                euv_ui::euv_markdown_blocks(&blocks[index..index + 1])
            }
        }
        EuvMdBlock::Container {
            kind,
            title,
            blocks,
        } => {
            if contains_code(blocks) {
                html! {
                    div {
                        class: {
                            let mut class: String =
                                String::with_capacity(CONTAINER_CLASS_PREFIX.len() + kind.len());
                            class.push_str(CONTAINER_CLASS_PREFIX);
                            class.push_str(kind);
                            class
                        }
                        p {
                            class: CONTAINER_TITLE_CLASS
                            {
                                *title
                            }
                        }
                        {
                            render_docs_blocks(blocks)
                        }
                    }
                }
            } else {
                euv_ui::euv_markdown_blocks(&blocks[index..index + 1])
            }
        }
        _ => euv_ui::euv_markdown_blocks(&blocks[index..index + 1]),
    }
}

/// Whether a run of blocks holds a code block anywhere below it.
///
/// # Arguments
///
/// - `&'static [EuvMdBlock]` - The blocks to test.
///
/// # Returns
///
/// - `bool` - `true` when a code block is reachable from the run.
fn contains_code(blocks: &'static [EuvMdBlock]) -> bool {
    blocks.iter().any(|block: &'static EuvMdBlock| match block {
        EuvMdBlock::CodeBlock { .. } => true,
        EuvMdBlock::BlockQuote(inner) => contains_code(inner),
        EuvMdBlock::List { items, .. } => items
            .iter()
            .any(|item: &&'static [EuvMdBlock]| contains_code(item)),
        EuvMdBlock::Container { blocks, .. } => contains_code(blocks),
        _ => false,
    })
}
