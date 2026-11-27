<div align="center">

# euv-performance

**Reproducible CPU, Layout, Reactivity, Virtual List & 3D Benchmark Harness**

[![Crates.io Version](https://img.shields.io/crates/v/euv-performance.svg?style=flat-square)](https://crates.io/crates/euv-performance)
[![Crates.io Downloads](https://img.shields.io/crates/d/euv-performance.svg?style=flat-square)](https://crates.io/crates/euv-performance)
[![Docs.rs](https://img.shields.io/docsrs/euv-performance?style=flat-square)](https://docs.rs/euv-performance)
[![CI Status](https://img.shields.io/github/actions/workflow/status/euv-dev/euv/rust.yml?style=flat-square&label=CI)](https://github.com/euv-dev/euv/actions)
[![Rust 2024](https://img.shields.io/badge/edition-2024-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/crates/l/euv-performance.svg?style=flat-square)](../LICENSE)

[Workspace Overview](../README.md) · [API Docs](https://docs.rs/euv-performance/latest/) · [Agent Engineering Rules](../AGENTS.md)

</div>

---

## Overview

Reproducible CPU, layout, reactivity, virtual list, batch mutation, WASM OTA hot-swap, and 3D ray-tracing benchmark harness for the `euv` framework.

## Responsibilities & Module Layout

- `bench` — Deterministic micro-benchmarks covering signal diamond propagation, O(1) `Selector` switching, clean-subtree layout caching, 100k-row virtual scrolling, and 3D ray-tracing
- `scripts/bench/stress-bench.sh` — Automated workspace benchmark runner

## Installation & Usage

```sh
cargo run --manifest-path performance/Cargo.toml --release
./scripts/bench/stress-bench.sh
```

## Verification

```sh
cargo run -p euv-cli -- fmt --check
cargo clippy --manifest-path performance/Cargo.toml --all-targets -- -D warnings
cargo test --manifest-path performance/Cargo.toml
```

## Documentation & Governance

- Update this `README.md` alongside any public API, macro, component, CLI, or module-boundary change per [`../AGENTS.md`](../AGENTS.md#4-unified-documentation--skill-maintenance-policy).
- Project standards and workflows live in [`../.agents/skills/euv/SKILL.md`](../.agents/skills/euv/SKILL.md).

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

## Contact

For inquiries, please contact the author at [root@devhub.vip](mailto:root@devhub.vip).
