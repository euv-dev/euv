---
title: Item
order: 33
---

One row of a list, with its actions.

## Installation

```shell
cargo add euv-ui
```

## Composition

```text
ItemGroup
├── Item
├── ItemHeader
├── ItemMedia
├── ItemContent
├── ItemTitle
├── ItemDescription
├── ItemActions
├── ItemFooter
```

## Parts

| Part | Component | Slot |
| --- | --- | --- |
| Item | `euv_item` | `item` |
| ItemActions | `euv_item_actions` | `item-actions` |
| ItemContent | `euv_item_content` | `item-content` |
| ItemDescription | `euv_item_description` | `item-description` |
| ItemFooter | `euv_item_footer` | `item-footer` |
| ItemGroup | `euv_item_group` | `item-group` |
| ItemHeader | `euv_item_header` | `item-header` |
| ItemMedia | `euv_item_media` | `item-media` |
| ItemTitle | `euv_item_title` | `item-title` |
