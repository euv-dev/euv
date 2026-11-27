---
title: Native Select
order: 40
---

A select that uses the control of the platform.

## Installation

```shell
cargo add euv-ui
```

## Composition

```text
NativeSelect
├── NativeSelectOption
```

## Parts

| Part               | Component                  | Slot                   |
| ------------------ | -------------------------- | ---------------------- |
| NativeSelect       | `euv_native_select`        | `native-select`        |
| NativeSelectOption | `euv_native_select_option` | `native-select-option` |

## Disabled Options

Set `disabled: true` on `euv_native_select_option` to exclude a single option, or on `euv_native_select_opt_group` to disable every option in the group. The browser's native select keeps its built-in keyboard and picker behavior.
