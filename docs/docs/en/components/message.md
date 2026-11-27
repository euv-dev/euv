---
title: Message
order: 38
---

One turn of a conversation.

## Installation

```shell
cargo add euv-ui
```

## Composition

```text
Message
├── MessageAvatar
├── MessageContent
├── MessageHeader
├── Bubble
├── MessageFooter
```

## Parts

| Part           | Component             | Slot              |
| -------------- | --------------------- | ----------------- |
| Bubble         | `euv_message_bubble`  | `bubble`          |
| Message        | `euv_message`         | `message`         |
| MessageAvatar  | `euv_message_avatar`  | `message-avatar`  |
| MessageContent | `euv_message_content` | `message-content` |
| MessageFooter  | `euv_message_footer`  | `message-footer`  |
| MessageHeader  | `euv_message_header`  | `message-header`  |
