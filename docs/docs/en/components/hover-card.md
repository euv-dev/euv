---
title: Hover Card
order: 29
---

A card that opens while the pointer rests on a trigger.

## Installation

```shell
cargo add euv-ui
```

## Composition

```text
HoverCard
├── HoverCardTrigger
├── HoverCardContent
```

## Parts

| Part             | Component                | Slot                 |
| ---------------- | ------------------------ | -------------------- |
| HoverCard        | `euv_hover_card`         | `hover-card`         |
| HoverCardContent | `euv_hover_card_content` | `hover-card-content` |
| HoverCardTrigger | `euv_hover_card_trigger` | `hover-card-trigger` |
