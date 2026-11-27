---
title: Alert Dialog
order: 3
---

A modal that interrupts the reader with a message and two answers.

## Installation

```shell
cargo add euv-ui
```

## Composition

```text
AlertDialog
├── AlertDialogTrigger
├── AlertDialogContent
├── AlertDialogHeader
├── AlertDialogMedia
├── AlertDialogTitle
├── AlertDialogDescription
├── AlertDialogFooter
├── AlertDialogCancel
├── AlertDialogAction
```

## Parts

| Part                   | Component                      | Slot                       |
| ---------------------- | ------------------------------ | -------------------------- |
| AlertDialog            | `euv_alert_dialog`             | `alert-dialog`             |
| AlertDialogAction      | `euv_alert_dialog_action`      | `alert-dialog-action`      |
| AlertDialogCancel      | `euv_alert_dialog_cancel`      | `alert-dialog-cancel`      |
| AlertDialogContent     | `euv_alert_dialog_content`     | `alert-dialog-content`     |
| AlertDialogDescription | `euv_alert_dialog_description` | `alert-dialog-description` |
| AlertDialogFooter      | `euv_alert_dialog_footer`      | `alert-dialog-footer`      |
| AlertDialogHeader      | `euv_alert_dialog_header`      | `alert-dialog-header`      |
| AlertDialogMedia       | `euv_alert_dialog_media`       | `alert-dialog-media`       |
| AlertDialogTitle       | `euv_alert_dialog_title`       | `alert-dialog-title`       |
| AlertDialogTrigger     | `euv_alert_dialog_trigger`     | `alert-dialog-trigger`     |
