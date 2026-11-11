---
title: Attachment
order: 5
---

A file that travels with a message, with its state and its size.

## Installation

```shell
cargo add euv-ui
```

## Composition

```text
Attachment
├── AttachmentMedia
├── AttachmentContent
├── AttachmentTitle
├── AttachmentDescription
├── AttachmentActions
├── AttachmentAction
├── AttachmentTrigger
```

## Parts

| Part | Component | Slot |
| --- | --- | --- |
| Attachment | `euv_attachment` | `attachment` |
| AttachmentAction | `euv_attachment_action` | `attachment-action` |
| AttachmentActions | `euv_attachment_actions` | `attachment-actions` |
| AttachmentContent | `euv_attachment_content` | `attachment-content` |
| AttachmentDescription | `euv_attachment_description` | `attachment-description` |
| AttachmentMedia | `euv_attachment_media` | `attachment-media` |
| AttachmentTitle | `euv_attachment_title` | `attachment-title` |
| AttachmentTrigger | `euv_attachment_trigger` | `attachment-trigger` |
