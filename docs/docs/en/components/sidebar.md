---
title: Sidebar
order: 52
---

The column that carries the navigation of an application.

## Installation

```shell
cargo add euv-ui
```

## Composition

```text
SidebarProvider
├── Sidebar
├── SidebarHeader
├── SidebarContent
├── SidebarGroup
├── SidebarGroupLabel
├── SidebarGroupAction
├── SidebarGroupContent
├── SidebarMenu
├── SidebarMenuItem
├── SidebarMenuButton
├── SidebarMenuAction
├── SidebarMenuBadge
├── SidebarMenuSub
├── SidebarMenuSubItem
├── SidebarFooter
├── SidebarRail
├── SidebarInset
├── SidebarTrigger
```

## Parts

| Part                | Component                   | Slot                    |
| ------------------- | --------------------------- | ----------------------- |
| Sidebar             | `euv_sidebar`               | `sidebar`               |
| SidebarContent      | `euv_sidebar_content`       | `sidebar-content`       |
| SidebarFooter       | `euv_sidebar_footer`        | `sidebar-footer`        |
| SidebarGroup        | `euv_sidebar_item`          | `sidebar-group`         |
| SidebarGroupAction  | `euv_sidebar_group_action`  | `sidebar-group-action`  |
| SidebarGroupContent | `euv_sidebar_group_content` | `sidebar-group-content` |
| SidebarGroupLabel   | `euv_sidebar_group_label`   | `sidebar-group-label`   |
| SidebarHeader       | `euv_sidebar_header`        | `sidebar-header`        |
| SidebarInset        | `euv_sidebar_inset`         | `sidebar-inset`         |
| SidebarMenu         | `euv_sidebar_menu`          | `sidebar-menu`          |
| SidebarMenuAction   | `euv_sidebar_menu_action`   | `sidebar-menu-action`   |
| SidebarMenuBadge    | `euv_sidebar_menu_badge`    | `sidebar-menu-badge`    |
| SidebarMenuButton   | `euv_sidebar_item`          | `sidebar-menu-button`   |
| SidebarMenuItem     | `euv_sidebar_menu_item`     | `sidebar-menu-item`     |
| SidebarMenuSub      | `euv_sidebar_menu_sub`      | `sidebar-menu-sub`      |
| SidebarMenuSubItem  | `euv_sidebar_menu_sub_item` | `sidebar-menu-sub-item` |
| SidebarProvider     | `euv_sidebar_provider`      | `sidebar-provider`      |
| SidebarRail         | `euv_sidebar_rail`          | `sidebar-rail`          |
| SidebarTrigger      | `euv_sidebar_trigger`       | `sidebar-trigger`       |
