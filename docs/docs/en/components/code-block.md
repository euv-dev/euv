---
title: Code Block
order: 17
---

A syntax-highlighted source code block aligned verbatim with VS Code **Eva Light Bold** (`#EBEEF5`) and **Eva Dark Bold** (`#282C34`), complete with language badge, filename header, line numbers, highlighted lines, and a one-click Copy callback.

## Installation

```shell
cargo add euv euv-ui
```

## Declarative Macro Usage

```rust
view! {
    EuvCodeBlock(source.to_string(), tokens) {
        language: "rust"
        filename: "src/main.rs"
        line_numbers: true
        highlight_lines: vec![1, 4]
        on_copy: move |copied: &str| {
            let _: bool = status.set(format!("Copied {} bytes", copied.len()));
        }
    }
}
```

## Eva Bold Theme Palette

| Token Role | Eva Light Bold | Eva Dark Bold |
| :--- | :---: | :---: |
| Background | `#EBEEF5` | `#282C34` |
| Keyword | `#7C4DFF` | `#A78CFA` |
| Function | `#437AED` | `#6495EE` |
| Type | `#00BEC4` | `#56B7C3` |
| String | `#53A053` | `#98C379` |
| Number | `#FF6D12` | `#FF9070` |
| Comment | `#A9A9AA` | `#676E95` |

> [!TIP]
> See the full [Markdown Syntax & Eva Theme Showcase](../guide/markdown-syntax.md) for live examples across Rust, TypeScript, JSON, Diff, Shell, YAML, GitHub Alerts, tables, and disclosures.

## Composition

```text
EuvCodeBlock
├── Header Chrome (filename + language pill + Copy button)
├── Line-Number Gutter (Eva muted gutter)
└── Syntax-Highlighted Lines (CodeThemePalette::resolve(mode))
```

## Parts

| Part | Type | Purpose |
| :--- | :--- | :--- |
| Code Block | `EuvCodeBlock` | Multi-language Eva Light/Dark Bold code viewer |
| Theme Palette | `CodeThemePalette` | Exact VS Code Eva Light/Dark Bold token palette |
| Markdown Suite | `EuvMarkdown` | Complete native GFM renderer embedding `EuvCodeBlock` |
