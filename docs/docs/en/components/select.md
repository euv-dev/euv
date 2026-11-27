---
title: Select
order: 49
---

The control that picks one of the options it holds.

## Installation

```shell
cargo add euv-ui
```

## Composition

```text
Select
├── SelectTrigger
├── SelectValue
├── SelectContent
├── SelectGroup
├── SelectLabel
├── SelectItem
├── SelectSeparator
```

## Parts

| Part            | Component             | Slot               |
| --------------- | --------------------- | ------------------ |
| Select          | `euv_select`          | `select`           |
| SelectContent   | `euv_select`          | `select-content`   |
| SelectGroup     | `select_option_group` | `select-group`     |
| SelectItem      | `select_option_row`   | `select-item`      |
| SelectLabel     | `select_option_group` | `select-label`     |
| SelectSeparator | `euv_select`          | `select-separator` |
| SelectTrigger   | `euv_select`          | `select-trigger`   |
| SelectValue     | `euv_select`          | `select-value`     |

## Accessibility

The trigger is a combobox and its `aria-controls` references the internal viewport listbox. `aria-expanded` reflects the panel state. Group wrappers use `role="group"` and reference their `SelectLabel` with `aria-labelledby`.

## Form Integration

Set `id` to associate a label with the trigger. A non-empty `name` or `required: true` adds a visually hidden native text control that follows the selected value and participates in form submission and required validation.

## Keyboard Typeahead

With the panel open, typing a label prefix moves focus to the next enabled matching option. Matching is case-insensitive, consecutive characters build a prefix for 700ms, and repeated initials cycle through matches. Press Enter to choose the focused option or Escape to close the panel.

## Invalid State

Set `EuvSelectProps.invalid` to `true` to mark the Select root with `data-invalid="true"` and its trigger with `aria-invalid="true"`. When used inside `euv_form_field`, set the field and Select invalid props from the same validation state.

## Align Item With Trigger

`align_item_with_trigger` controls whether the selected option is centered over the trigger when the panel opens. It defaults to `true` with `theme-shadcn` and `false` with other themes. Set it to `false` to keep the panel edge-aligned below the trigger; if the aligned panel would cross the viewport gutter, it falls back to the edge-aligned position.

## Scrollable Options

Long option lists scroll in an internal viewport. The Select shows an up or down scroll indicator only while more options remain in that direction; the list itself keeps native wheel, touch and keyboard scrolling. The indicators are decorative and do not take focus.
