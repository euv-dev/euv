---
title: Breadcrumb
order: 8
---

The trail of the pages that lead to the one being read.

## Installation

```shell
cargo add euv-ui
```

## Composition

```text
Breadcrumb
├── BreadcrumbList
├── BreadcrumbItem
├── BreadcrumbLink
├── BreadcrumbSeparator
├── BreadcrumbPage
```

## Parts

| Part                | Component                        | Slot                   |
| ------------------- | -------------------------------- | ---------------------- |
| Breadcrumb          | `euv_breadcrumb`                 | `breadcrumb`           |
| BreadcrumbItem      | `euv_breadcrumb_item`            | `breadcrumb-item`      |
| BreadcrumbLink      | `breadcrumb_link_node`           | `breadcrumb-link`      |
| BreadcrumbList      | `euv_breadcrumb_list`            | `breadcrumb-list`      |
| BreadcrumbPage      | `breadcrumb_page_node_with`      | `breadcrumb-page`      |
| BreadcrumbSeparator | `breadcrumb_separator_node_with` | `breadcrumb-separator` |
