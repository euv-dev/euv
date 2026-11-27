---
title: Table
order: 58
---

Rows and columns of data.

## Installation

```shell
cargo add euv-ui
```

## Composition

```text
Table
├── TableCaption
├── TableHeader
├── TableRow
├── TableHead
├── TableBody
├── TableCell
├── TableFooter
```

## Parts

| Part         | Component           | Slot            |
| ------------ | ------------------- | --------------- |
| Table        | `euv_table`         | `table`         |
| TableBody    | `euv_table_body`    | `table-body`    |
| TableCaption | `euv_table_caption` | `table-caption` |
| TableCell    | `euv_table_cell`    | `table-cell`    |
| TableFooter  | `euv_table_footer`  | `table-footer`  |
| TableHead    | `euv_table_head`    | `table-head`    |
| TableHeader  | `euv_table_header`  | `table-header`  |
| TableRow     | `euv_table_row`     | `table-row`     |
