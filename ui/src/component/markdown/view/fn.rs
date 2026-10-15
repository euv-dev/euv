use super::*;

/// A generic markdown renderer component.
///
/// Renders a block AST (for example generated at build time) into euv
/// VirtualNodes inside an `<article>` carrying the `md-body` prose class;
/// inject [`EUV_MD_CSS`] once at startup for the typography styles.
///
/// # Arguments
///
/// - `VirtualNode<EuvMarkdownProps>` - The props node containing the blocks.
///
/// # Returns
///
/// - `VirtualNode` - The rendered markdown article.
#[component]
pub fn euv_markdown(node: VirtualNode<EuvMarkdownProps>) -> VirtualNode {
    let EuvMarkdownProps { blocks }: EuvMarkdownProps = node.try_get_props().unwrap_or_default();
    let content: VirtualNode = euv_markdown_blocks(blocks);
    html! {
        article {
            class: c_euv_markdown()
            class: MD_BODY_PROSE_CLASS
            content
        }
    }
}

/// Renders a markdown block slice into euv VirtualNodes.
///
/// Pure render function (no component): the AST is `&'static` data, so
/// rendering is a direct recursive tree walk and the framework's diffing
/// applies to the produced virtual DOM.
///
/// # Arguments
///
/// - `&'static [EuvMdBlock]` - The blocks to render.
///
/// # Returns
///
/// - `VirtualNode` - The rendered block sequence.
pub fn euv_markdown_blocks(blocks: &'static [EuvMdBlock]) -> VirtualNode {
    html! {
        for block in blocks.iter() {
            { render_md_block(block) }
        }
    }
}

/// Renders a single markdown block.
///
/// # Arguments
///
/// - `&'static EuvMdBlock` - The block to render.
///
/// # Returns
///
/// - `VirtualNode` - The rendered block.
fn render_md_block(block: &'static EuvMdBlock) -> VirtualNode {
    match block {
        EuvMdBlock::Heading {
            level,
            id,
            href,
            inline,
        } => render_md_heading(*level, id, href, inline),
        EuvMdBlock::Paragraph(inline) => {
            let content: VirtualNode = render_md_inlines(inline);
            html! {
                p {
                    content
                }
            }
        }
        EuvMdBlock::CodeBlock { lang, code } => html! {
            pre {
                // OPT 30: avoid `format!("language-{lang}")` per render;
                // concatenate the constant prefix with the dynamic lang
                // segment using a single pre-sized buffer.
                class: {
                    let mut class: String =
                        String::with_capacity(MD_CODE_LANG_CLASS_PREFIX.len() + lang.len());
                    class.push_str(MD_CODE_LANG_CLASS_PREFIX);
                    class.push_str(lang);
                    class
                }
                code {
                    { *code }
                }
            }
        },
        EuvMdBlock::BlockQuote(blocks) => {
            let content: VirtualNode = euv_markdown_blocks(blocks);
            html! {
                blockquote {
                    content
                }
            }
        }
        EuvMdBlock::List { ordered, items } => {
            if *ordered {
                html! {
                    ol {
                        for item in items.iter() {
                            li {
                                { euv_markdown_blocks(item) }
                            }
                        }
                    }
                }
            } else {
                html! {
                    ul {
                        for item in items.iter() {
                            li {
                                { euv_markdown_blocks(item) }
                            }
                        }
                    }
                }
            }
        }
        EuvMdBlock::Table { head, rows } => html! {
            // The scroll container has to be a *wrapper*, not the table
            // itself: `overflow-x` on a `display: table` box is ignored
            // by the browser, and setting `display: block` on the table
            // (the previous workaround) drops it out of table layout, so
            // the anonymous table shrink-wraps its content instead of
            // filling the column. The wrapper scrolls; the table inside
            // keeps real table layout and expands to the column width.
            div {
                class: c_euv_markdown_table_scroll()
                table {
                    thead {
                        tr {
                            for cell in head.iter() {
                                th {
                                    { render_md_inlines(cell) }
                                }
                            }
                        }
                    }
                    tbody {
                        for row in rows.iter() {
                            tr {
                                for cell in row.iter() {
                                    td {
                                        { render_md_inlines(cell) }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        },
        EuvMdBlock::Container {
            kind,
            title,
            blocks,
        } => {
            let content: VirtualNode = euv_markdown_blocks(blocks);
            html! {
                div {
                    // OPT 30: same concat-with-constant pattern as the
                    // `CodeBlock` branch above.
                    class: {
                        let mut class: String =
                            String::with_capacity(MD_DOCS_CONTAINER_CLASS_PREFIX.len() + kind.len());
                        class.push_str(MD_DOCS_CONTAINER_CLASS_PREFIX);
                        class.push_str(kind);
                        class
                    }
                    p {
                        class: MD_DOCS_CONTAINER_TITLE_CLASS
                        { *title }
                    }
                    content
                }
            }
        }
        EuvMdBlock::Rule => html! {
            hr {}
        },
        EuvMdBlock::Html(raw) => html! {
            div {
                inner_html: *raw
            }
        },
    }
}

/// Renders one heading block with its permalink anchor.
///
/// # Arguments
///
/// - `u8` - The heading level (1–6).
/// - `&'static str` - The slug id.
/// - `&'static str` - The full permalink href.
/// - `&'static [EuvMdInline]` - The heading content.
///
/// # Returns
///
/// - `VirtualNode` - The rendered heading.
fn render_md_heading(
    level: u8,
    id: &'static str,
    href: &'static str,
    inline: &'static [EuvMdInline],
) -> VirtualNode {
    let anchor: VirtualNode = html! {
        a {
            class: MD_HEADING_ANCHOR_CLASS
            href: href
            span { "#" }
        }
    };
    let content: VirtualNode = render_md_inlines(inline);
    match level {
        1 => html! {
            h1 {
                id: id
                anchor
                content
            }
        },
        2 => html! {
            h2 {
                id: id
                anchor
                content
            }
        },
        3 => html! {
            h3 {
                id: id
                anchor
                content
            }
        },
        4 => html! {
            h4 {
                id: id
                anchor
                content
            }
        },
        5 => html! {
            h5 {
                id: id
                anchor
                content
            }
        },
        _ => html! {
            h6 {
                id: id
                anchor
                content
            }
        },
    }
}

/// Renders an inline slice.
///
/// # Arguments
///
/// - `&'static [EuvMdInline]` - The inlines to render.
///
/// # Returns
///
/// - `VirtualNode` - The rendered inline sequence.
fn render_md_inlines(inlines: &'static [EuvMdInline]) -> VirtualNode {
    html! {
        for inline in inlines.iter() {
            { render_md_inline(inline) }
        }
    }
}

/// Renders a single inline node.
///
/// # Arguments
///
/// - `&'static EuvMdInline` - The inline to render.
///
/// # Returns
///
/// - `VirtualNode` - The rendered inline.
fn render_md_inline(inline: &'static EuvMdInline) -> VirtualNode {
    match inline {
        EuvMdInline::Text(text) => html! {
            { *text }
        },
        EuvMdInline::Strong(children) => {
            let content: VirtualNode = render_md_inlines(children);
            html! {
                strong {
                    content
                }
            }
        }
        EuvMdInline::Em(children) => {
            let content: VirtualNode = render_md_inlines(children);
            html! {
                em {
                    content
                }
            }
        }
        EuvMdInline::Del(children) => {
            let content: VirtualNode = render_md_inlines(children);
            html! {
                del {
                    content
                }
            }
        }
        EuvMdInline::Code(code) => html! {
            code {
                { *code }
            }
        },
        EuvMdInline::Link {
            href,
            external,
            children,
        } => {
            let content: VirtualNode = render_md_inlines(children);
            if *external {
                html! {
                    a {
                        href: *href
                        target: "_blank"
                        content
                    }
                }
            } else {
                html! {
                    a {
                        href: *href
                        content
                    }
                }
            }
        }
        EuvMdInline::Image { src, alt } => html! {
            img {
                src: *src
                alt: *alt
                // The stylesheet holds an unsettled image at `height: 0`
                // so a slow image cannot flash at the wrong size and then
                // reflow the page. Marking it here on both outcomes — and
                // for a cached image through `mark_image_loaded` at mount —
                // replaces an injected script plus a body-wide
                // MutationObserver with two handlers on the element itself.
                onload: mark_image_loaded()
                onerror: mark_image_loaded()
            }
        },
        EuvMdInline::TaskMarker(checked) => {
            if *checked {
                html! {
                    span {
                        class: MD_TASK_MARKER_CLASS
                        "☑"
                    }
                }
            } else {
                html! {
                    span {
                        class: MD_TASK_MARKER_CLASS
                        "☐"
                    }
                }
            }
        }
        EuvMdInline::SoftBreak => html! {
            " "
        },
        EuvMdInline::HardBreak => html! {
            br {}
        },
        EuvMdInline::Html(raw) => html! {
            span {
                inner_html: *raw
            }
        },
    }
}

/// Marks the pressed image as settled so the stylesheet stops holding it at
/// zero height.
///
/// Both outcomes set the attribute: an image that failed to load is just
/// as settled as one that succeeded, and leaving a broken image collapsed
/// to `height: 0` hides the fact that anything was there at all.
///
/// The attribute is set through `set_attribute` rather than by toggling a
/// class because the stylesheet matches `[data-loaded]` — presence is the
/// whole signal.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The handler, bound to `load` and `error`.
pub(crate) fn mark_image_loaded() -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(|event: Event| {
        let target: JsValue = event
            .target()
            .map_or(JsValue::NULL, |event_target: EventTarget| {
                event_target.into()
            });
        let _marked: Result<(), JsValue> =
            Element::from(target).set_attribute(MD_IMAGE_LOADED_ATTR, MD_IMAGE_LOADED_VALUE);
    }))
}
