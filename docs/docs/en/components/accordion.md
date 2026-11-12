---
title: Accordion
order: 1
---

A vertical stack of headings that each reveal a section of content.

## Installation

```shell
cargo add euv-ui
```

## Composition

```text
Accordion
├── AccordionItem
├── AccordionTrigger
├── AccordionContent
```

## Parts

| Part | Component | Slot |
| --- | --- | --- |
| Accordion | `euv_accordion` | `accordion` |
| AccordionContent | `accordion_section` | `accordion-content` |
| AccordionItem | `accordion_section` | `accordion-item` |
| AccordionTrigger | `accordion_section` | `accordion-trigger` |
