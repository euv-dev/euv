---
title: Tabs
order: 59
---

The set of panels that share one place on the page.

## Installation

```shell
cargo add euv-ui
```

## Composition

```text
Tabs
├── TabsList
├── TabsTrigger
├── TabsContent
```

## Parts

| Part        | Component    | Slot           |
| ----------- | ------------ | -------------- |
| Tabs        | `euv_tabs`   | `tabs`         |
| TabsContent | `euv_tabs`   | `tabs-content` |
| TabsList    | `euv_tabs`   | `tabs-list`    |
| TabsTrigger | `tab_button` | `tabs-trigger` |
