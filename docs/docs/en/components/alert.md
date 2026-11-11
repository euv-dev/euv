---
title: Alert
order: 2
---

A callout that draws attention to a short piece of feedback.

## Installation

```shell
cargo add euv-ui
```

## Composition

```text
Alert
├── Icon
├── AlertTitle
├── AlertDescription
├── AlertAction
```

## Parts

| Part | Component | Slot |
| --- | --- | --- |
| Alert | `euv_alert` | `alert` |
| AlertAction | `euv_alert_action` | `alert-action` |
| AlertDescription | `euv_alert_description` | `alert-description` |
| AlertTitle | `euv_alert_title` | `alert-title` |
| Icon | `euv_alert_icon` | `icon` |
