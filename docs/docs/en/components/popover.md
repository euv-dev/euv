---
title: Popover
order: 43
---

A panel that opens over the page.

## Installation

```shell
cargo add euv-ui
```

## Composition

```text
Popover
├── PopoverTrigger
├── PopoverContent
```

## Parts

| Part           | Component             | Slot              |
| -------------- | --------------------- | ----------------- |
| Popover        | `euv_popover`         | `popover`         |
| PopoverContent | `euv_popover`         | `popover-content` |
| PopoverTrigger | `euv_popover_trigger` | `popover-trigger` |
