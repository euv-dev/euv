---
title: Markdown Features
---

# Markdown Features

euv-docs renders a VuePress-flavored markdown subset at build time. This
page is the canonical reference for every block-level and inline
construct the parser supports, with the exact rendering it produces.

## Headings

Headings `h2` and `h3` appear in the right anchor TOC automatically. `h1`,
`h4`, `h5`, and `h6` render as plain headings and are not added to the TOC.

### h3 sub-heading

#### h4 sub-sub-heading

##### h5 sub-sub-sub-heading

###### h6 deepest heading

### Setext-style headings {#setext-h1}

Underlined text is also recognized as a heading. `=` underlines map
to `h1` and `-` underlines map to `h2`.

Setext h1
=========

Setext h2
---------

### Custom heading id {#custom-heading-id}

Append `{#slug}` to override the auto-generated slug. The slug
becomes the element id and the anchor permalink
(`#/guide/markdown.html#custom-heading-id` here).

## Inline formatting

**Bold**, *italic*, ~~strikethrough~~, and `inline code` are supported.

Bold and italic can nest: ***bold italic***, **bold with `inline code` inside**, and
*a mix of `code` and emphasis*.

Underscores work for italic too: _italic_, __bold__, and
*combined with asterisks like **this***.

Inline formatting can also nest with strikethrough:
**~~bold strikethrough~~**, *~~italic strikethrough~~*, and
***~~bold italic strikethrough~~***.

Inline code spans can contain a backtick when wrapped in a longer
fence: ``code with `backtick` inside`` renders as one span.

A footnote reference renders inline as `[^example]` — it shows as plain
`[^example]` text and points to the definition below.

## Hard break vs soft break

This paragraph ends with two trailing spaces, so the next line  
starts a new line in the same paragraph (hard break, `<br>`).

This paragraph ends with no trailing whitespace, so the next line
glues onto the previous one as a single paragraph (soft break, just a
space).

## Lists

- Unordered item
- Another item
  - Nested unordered item
  - Another nested item
    1. Nested ordered item under unordered
    2. Second nested ordered
  - Back to unordered

1. Ordered item
2. Second item
3. Third item with a [link](./getting-started.md)

- [x] Task list item (done)
- [ ] Task list item (todo)
- [x] Completed task with `inline code`

Mixing an ordered list with a task sublist:

1. Plan
   - [x] Outline
   - [ ] Draft
2. Review
   - [x] Self-review
   - [ ] Peer review

## Tables

| Feature | VuePress | euv-docs | Notes |
| :--- | :---: | ---: | --- |
| Frontmatter | ✅ | ✅ | parsed at build time |
| Custom containers | ✅ | ✅ | `::: tip / warning / danger / note` |
| Anchor TOC | ✅ | ✅ | h2 + h3 only |
| Table alignment | ✅ | ✅ | left / center / right |

Use a backslash to escape a literal pipe inside a cell: `col \| with pipe`
renders as `col | with pipe`.

## Code blocks

Fenced blocks with a language tag get syntax class on the wrapper:

```rust
fn fib(n: u64) -> u64 {
    match n {
        0 | 1 => n,
        _ => fib(n - 1) + fib(n - 2),
    }
}
```

```bash
euv build --release -- --target web --out-dir www/pkg
```

```toml
[site]
title = "euv-docs"

[[locales]]
prefix = "/"
lang = "en-US"
label = "English"
```

```text
plain text fence, no syntax class
```

```markdown
# markdown fence, no syntax class
```

Indented code blocks (4-space indent) also work, but fenced form is
preferred because the language tag drives the wrapper class.

    // 4-space indented block renders as code
    fn main() {}

A code block with an unknown language tag still fences correctly:

```haskell
main :: IO ()
main = putStrLn "unknown lang, but still fenced"
```

## Custom containers

The full set of supported kinds is `tip`, `warning`, `danger`, `info`,
`note`, and `details` (defined in euv-ui's container CSS).

::: tip
Tips are rendered with a solid border.
:::

::: warning
Warnings use a dashed border.
:::

::: danger
Danger containers use a double border.
:::

::: info
Info containers use a dotted border.
:::

::: note Custom title
Containers accept an optional custom title after the kind.
:::

A container may also hold several blocks at once:

::: tip Multiple blocks
This container holds **two paragraphs** and a list.

- First inner item
- Second inner item
:::

Containers do not nest — a second `:::` inside a container is rendered
as literal text.

::: details Click to expand
Hidden content rendered inside an expandable block. The block can hold
**any block-level content** supported by markdown: code, lists, even
nested blockquotes.

```rust
fn main() {
    println!("inside details");
}
```
:::

## Blockquotes

> Documentation is a love letter to your future self.

Nested blockquotes:

> First level.
>
> > Second level.
> >
> > Even **bold** and `code` work inside.
>
> Back to first level.

A blockquote can also contain other blocks:

> - A list inside a quote
> - Second item
>
> ```
> code inside a quote
> ```

## Horizontal rules

Three or more of `-`, `_`, or `*` on a line:

---

***

___

## Links

- [Internal link to the guide index](./README.md)
- [External link to euv](https://github.com/euv-dev/euv)
- [Jump to Tables](#tables)
- [Jump to Setext headings](#setext-h1)
- [Jump to custom heading id](#custom-heading-id)
- [Mail link](mailto:nobody@example.com)
- [Link with title](https://github.com/euv-dev/euv "euv repository on GitHub")

Bare URLs and email addresses are autolinked: <https://github.com/euv-dev/euv>
and <nobody@example.com> render as clickable links without any
markdown syntax around them.

Reference-style links work too. Define an id once and reuse it:

The [euv repo][euv-repo] and [its issue tracker][euv-issues] both live
on GitHub. Reusing the same id multiple times is fine: see [the euv
repo][euv-repo] again.

[euv-repo]: https://github.com/euv-dev/euv "euv on GitHub"
[euv-issues]: https://github.com/euv-dev/euv/issues "issue tracker"

## Footnotes

A footnote reference points to a definition anywhere on the same page.
The definition syntax is `[^name]: body`, and the reference is
`[^name]` inline.[^syntax] The renderer does not auto-number or
generate a footnote list — the definition is rendered as a blockquote
prefixed with the id, and the reference shows the literal `[^name]`.

[^syntax]: This is the footnote body. It can contain **formatting**,
    `inline code`, and even multiple paragraphs.

    A second paragraph inside the same footnote is supported.

    > Blockquotes work inside footnotes too.

## Images

Images render with the alt text and the source URL preserved:

![euv logo placeholder](https://dummyimage.com/120x40/eee/aaa.png)

Use a base64-encoded `data:` URL when you want the image to render
fully offline without any external request:

![inline base64](data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyNDAiIGhlaWdodD0iNTAiPjxyZWN0IHdpZHRoPSIyNDAiIGhlaWdodD0iNTAiIGZpbGw9IiM0NDQiLz48dGV4dCB4PSIxMjAiIHk9IjMyIiB0ZXh0LWFuY2hvcj0ibWlkZGxlIiBmaWxsPSJ3aGl0ZSIgZm9udC1mYW1pbHk9InNhbnMtc2VyaWYiIGZvbnQtc2l6ZT0iMTYiPmlubGluZSBzdmc8L3RleHQ+PC9zdmc+)

## Escaped characters

Backslash escapes the next character: \*literal asterisks\*,
\[not a link\], \<not a tag\>, \`not code\`. The backslash is consumed
and the character renders as plain text.

## Inline and block HTML

A span of inline HTML is allowed inside paragraphs:

This paragraph has a <sup>superscript</sup> and a <sub>subscript</sub>
inlined with raw HTML tags.

Raw HTML attributes are preserved too: <kbd>Ctrl</kbd>+<kbd>S</kbd>
renders keyboard shortcuts, and <mark>highlighted text</mark> uses the
native `<mark>` element.

A block of raw HTML passes through verbatim:

<div style="border: 1px dashed currentColor; padding: 8px 12px;">
  This block is rendered as raw HTML, not markdown.
</div>

<details>
<summary>Native HTML <code>&lt;details&gt;</code></summary>

A `<details>` block is also passed through verbatim. This is different
from the `::: details` custom container — the container is a markdown
construct with the same look, but this one uses native HTML directly.

</details>
