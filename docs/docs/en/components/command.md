---
title: Command
order: 20
---

A list of commands that a reader can search through.

## Installation

```shell
cargo add euv-ui
```

## Composition

```text
Command
├── CommandInput
├── CommandList
├── CommandEmpty
├── CommandGroup
├── CommandItem
├── CommandSeparator
```

## Parts

| Part | Component | Slot |
| --- | --- | --- |
| Command | `euv_command` | `command` |
| CommandEmpty | `euv_command_empty` | `command-empty` |
| CommandGroup | `euv_command_group` | `command-group` |
| CommandInput | `euv_command_input` | `command-input` |
| CommandItem | `euv_command_item` | `command-item` |
| CommandList | `euv_command_list` | `command-list` |
| CommandSeparator | `euv_command_separator` | `command-separator` |
