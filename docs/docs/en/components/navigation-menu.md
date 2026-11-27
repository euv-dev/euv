---
title: Navigation Menu
order: 41
---

The menu that carries the sections of a site.

## Installation

```shell
cargo add euv-ui
```

## Composition

```text
NavigationMenu
├── NavigationMenuList
├── NavigationMenuItem
├── NavigationMenuTrigger
├── NavigationMenuContent
├── NavigationMenuLink
├── NavigationMenuIndicator
```

## Parts

| Part                    | Component                       | Slot                        |
| ----------------------- | ------------------------------- | --------------------------- |
| NavigationMenu          | `euv_navigation_menu`           | `navigation-menu`           |
| NavigationMenuContent   | `euv_navigation_menu_content`   | `navigation-menu-content`   |
| NavigationMenuIndicator | `euv_navigation_menu_indicator` | `navigation-menu-indicator` |
| NavigationMenuItem      | `euv_navigation_menu_item`      | `navigation-menu-item`      |
| NavigationMenuLink      | `euv_navigation_menu_link`      | `navigation-menu-link`      |
| NavigationMenuList      | `euv_navigation_menu_list`      | `navigation-menu-list`      |
| NavigationMenuTrigger   | `euv_navigation_menu_trigger`   | `navigation-menu-trigger`   |
