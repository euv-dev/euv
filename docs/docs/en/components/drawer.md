---
title: Drawer
order: 24
---

A panel that slides in from the edge of the page.

## Installation

```shell
cargo add euv-ui
```

## Composition

```text
Drawer
├── DrawerTrigger
├── DrawerContent
├── DrawerHeader
├── DrawerTitle
├── DrawerDescription
├── DrawerFooter
```

## Parts

| Part | Component | Slot |
| --- | --- | --- |
| Drawer | `euv_drawer` | `drawer` |
| DrawerContent | `euv_drawer_content` | `drawer-content` |
| DrawerDescription | `euv_drawer_description` | `drawer-description` |
| DrawerFooter | `euv_drawer_footer` | `drawer-footer` |
| DrawerHeader | `euv_drawer_header` | `drawer-header` |
| DrawerTitle | `euv_drawer_title` | `drawer-title` |
| DrawerTrigger | `euv_drawer_trigger` | `drawer-trigger` |
