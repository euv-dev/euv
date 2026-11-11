---
title: Field
order: 27
---

A label, a description, an error and the control they belong to.

## Installation

```shell
cargo add euv-ui
```

## Composition

```text
Field
├── FieldLabel
├── Input
├── FieldDescription
├── FieldError
```

## Parts

| Part | Component | Slot |
| --- | --- | --- |
| Field | `euv_field` | `field` |
| FieldDescription | `euv_field_description` | `field-description` |
| FieldError | `euv_field_error` | `field-error` |
| FieldLabel | `euv_field_label` | `field-label` |
| Input | `euv_field` | `input` |
