---
title: Context Menu
order: 21
---

The menu that opens on a right click.

## Installation

```shell
cargo add euv-ui
```

## Composition

```text
ContextMenu
├── ContextMenuTrigger
├── ContextMenuContent
├── ContextMenuGroup
├── ContextMenuLabel
├── ContextMenuItem
├── ContextMenuSeparator
├── ContextMenuCheckboxItem
├── ContextMenuRadioGroup
├── ContextMenuRadioItem
├── ContextMenuSub
├── ContextMenuSubTrigger
├── ContextMenuSubContent
```

## Parts

| Part                    | Component                        | Slot                         |
| ----------------------- | -------------------------------- | ---------------------------- |
| ContextMenu             | `euv_context_menu`               | `context-menu`               |
| ContextMenuCheckboxItem | `euv_context_menu_checkbox_item` | `context-menu-checkbox-item` |
| ContextMenuContent      | `euv_context_menu_content`       | `context-menu-content`       |
| ContextMenuGroup        | `euv_context_menu_group`         | `context-menu-group`         |
| ContextMenuItem         | `euv_context_menu_item`          | `context-menu-item`          |
| ContextMenuLabel        | `euv_context_menu_label`         | `context-menu-label`         |
| ContextMenuRadioGroup   | `euv_context_menu_radio_group`   | `context-menu-radio-group`   |
| ContextMenuRadioItem    | `euv_context_menu_radio_item`    | `context-menu-radio-item`    |
| ContextMenuSeparator    | `euv_context_menu_separator`     | `context-menu-separator`     |
| ContextMenuSub          | `euv_context_menu_sub`           | `context-menu-sub`           |
| ContextMenuSubContent   | `euv_context_menu_sub_content`   | `context-menu-sub-content`   |
| ContextMenuSubTrigger   | `euv_context_menu_sub_trigger`   | `context-menu-sub-trigger`   |
| ContextMenuTrigger      | `euv_context_menu_trigger`       | `context-menu-trigger`       |
