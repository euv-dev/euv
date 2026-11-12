---
title: Resizable
order: 47
---

Panels that share the space between them.

## Installation

```shell
cargo add euv-ui
```

## Composition

```text
ResizablePanelGroup
├── ResizablePanel
├── ResizableHandle
```

## Parts

| Part | Component | Slot |
| --- | --- | --- |
| ResizableHandle | `euv_resizable_handle` | `resizable-handle` |
| ResizablePanel | `euv_resizable_panel` | `resizable-panel` |
| ResizablePanelGroup | `euv_resizable_panel_group` | `resizable-panel-group` |
