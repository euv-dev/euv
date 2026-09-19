---
home: true
heroText: euv-docs
tagline: A VuePress-style documentation site powered by euv + euv-ui, compiled to WebAssembly.
actions:
  - text: Get Started →
    link: /guide/getting-started.html
    type: primary
  - text: Introduction
    link: /guide/
    type: secondary
features:
  - icon: 📝
    title: Markdown-driven
    details: Write plain .md files under docs/ — pages, sidebars, and anchor TOCs are generated automatically at build time.
  - icon: 🧭
    title: VuePress layout
    details: Home hero, navbar, multi-level collapsible sidebar, right anchor TOC, prev/next links, and footer — the layout you already know.
  - icon: 🦀
    title: Rust + WASM
    details: The whole site is a single euv WASM app — reactive, themeable (light/dark), and fast.
footer: MIT Licensed | Built with euv + euv-ui
---

## Hello, euv-docs

This is the home page body. Everything above the horizontal rule comes from
the frontmatter of this very file; everything below is regular markdown.

```rust
fn main() {
    println!("Docs as code, rendered as WASM.");
}
```
