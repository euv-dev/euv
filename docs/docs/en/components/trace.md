---
title: Trace
order: 65
---

The steps of one run of work, with their timings.

## Installation

```shell
cargo add euv-ui
```

## Composition

```text
Trace
├── TraceLine
├── TraceIcon
├── TraceTitle
├── TraceDescription
```

## Parts

| Part             | Component               | Slot                |
| ---------------- | ----------------------- | ------------------- |
| Trace            | `euv_trace`             | `trace`             |
| TraceDescription | `euv_trace_description` | `trace-description` |
| TraceIcon        | `euv_trace_icon`        | `trace-icon`        |
| TraceLine        | `euv_trace_line`        | `trace-line`        |
| TraceTitle       | `euv_trace_title`       | `trace-title`       |
