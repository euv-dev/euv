---
title: Tooltip
order: 64
---

The note that names a control while the pointer rests on it.

## Installation

```shell
cargo add euv-ui
```

## Composition

```text
Tooltip
├── TooltipTrigger
├── TooltipContent
```

## Parts

| Part | Component | Slot |
| --- | --- | --- |
| Tooltip | `euv_tooltip` | `tooltip` |
| TooltipContent | `euv_tooltip` | `tooltip-content` |
| TooltipTrigger | `euv_tooltip` | `tooltip-trigger` |
