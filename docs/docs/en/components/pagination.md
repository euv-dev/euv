---
title: Pagination
order: 42
---

The controls that move between the pages of a list.

## Installation

```shell
cargo add euv-ui
```

## Composition

```text
Pagination
├── PaginationContent
├── PaginationItem
├── PaginationPrevious
├── PaginationLink
├── PaginationEllipsis
├── PaginationNext
```

## Parts

| Part | Component | Slot |
| --- | --- | --- |
| Pagination | `euv_pagination` | `pagination` |
| PaginationContent | `euv_pagination` | `pagination-content` |
| PaginationEllipsis | `euv_pagination_ellipsis` | `pagination-ellipsis` |
| PaginationItem | `euv_pagination_item` | `pagination-item` |
| PaginationLink | `euv_pagination_link` | `pagination-link` |
| PaginationNext | `euv_pagination` | `pagination-next` |
| PaginationPrevious | `euv_pagination` | `pagination-previous` |
