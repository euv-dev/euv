---
title: Sheet
order: 51
---

A panel that slides over the page from the edge.

## Installation

```shell
cargo add euv-ui
```

## Composition

```text
Sheet
├── SheetTrigger
├── SheetContent
├── SheetHeader
├── SheetTitle
├── SheetDescription
├── SheetFooter
```

## Parts

| Part | Component | Slot |
| --- | --- | --- |
| Sheet | `euv_sheet` | `sheet` |
| SheetContent | `euv_sheet_content` | `sheet-content` |
| SheetDescription | `euv_sheet_description` | `sheet-description` |
| SheetFooter | `euv_sheet_footer` | `sheet-footer` |
| SheetHeader | `euv_sheet_header` | `sheet-header` |
| SheetTitle | `euv_sheet_title` | `sheet-title` |
| SheetTrigger | `euv_sheet_trigger` | `sheet-trigger` |
