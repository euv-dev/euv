---
title: Code Block
order: 17
---

A block of source code, with a toolbar and a copy button.

## Installation

```shell
cargo add euv-ui
```

## Composition

```text
CodeBlock
├── CodeBlockHeader
├── CodeBlockTitle
├── CodeBlockActions
├── CodeBlockCopyButton
├── CodeBlockContent
├── CodeBlockLine
├── CodeBlockFooter
```

## Parts

| Part | Component | Slot |
| --- | --- | --- |
| CodeBlock | `euv_code_block` | `code-block` |
| CodeBlockActions | `euv_code_block_actions` | `code-block-actions` |
| CodeBlockContent | `euv_code_block_content` | `code-block-content` |
| CodeBlockCopyButton | `euv_code_block_copy_button` | `code-block-copy-button` |
| CodeBlockFooter | `euv_code_block_footer` | `code-block-footer` |
| CodeBlockHeader | `euv_code_block_header` | `code-block-header` |
| CodeBlockLine | `euv_code_block_line` | `code-block-line` |
| CodeBlockTitle | `euv_code_block_title` | `code-block-title` |
