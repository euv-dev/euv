<div align="center">

# Euv-performance

**Reproducible CPU, Layout, Reactivity, Virtual List & 3D Benchmark Harness**

[![Crates.io Version](https://flat.badgen.net/crates/v/euv-performance)](https://crates.io/crates/euv-performance)
[![Crates.io Downloads](https://flat.badgen.net/crates/d/euv-performance)](https://crates.io/crates/euv-performance)
[![Docs.rs](https://img.shields.io/docsrs/euv-performance?style=flat-square)](https://docs.rs/euv-performance)
[![CI Status](https://img.shields.io/github/actions/workflow/status/euv-dev/euv/rust.yml?style=flat-square&label=CI)](https://github.com/euv-dev/euv/actions)
[![Rust 2024](https://img.shields.io/badge/edition-2024-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](../LICENSE)

[Workspace Overview](../README.md) · [API Docs](https://docs.rs/euv-performance/latest/) · [Agent Engineering Rules](../AGENTS.md)

</div>

---

## Overview

Reproducible CPU, layout, reactivity, virtual list, batch mutation, WASM OTA hot-swap, and 3D ray-tracing benchmark harness for the `euv` framework.

## Responsibilities & Module Layout

- `suite` — Deterministic 27-benchmark release suite covering signal diamond propagation, O(1) `Selector` switching, clean-subtree layout caching, software/hardware/GPU rendering, 100k-row virtual scrolling, WASM OTA hot-swap, and 3D ray-tracing
- `stress` — Opt-in `--stress-matrix` verification harness measuring tail percentiles (`p50`/`p95`/`p99`/`max`), 144Hz/60Hz over-budget frame counts, interleaved A/B partial-layout damage bit-exact parity, window resize reflow, and Terminal TUI cell-diff/idle flush invariants
- `scripts/bench/run-stress-report.py` — Canonical zero-dependency runner aggregating N warmed release runs, verifying `--stress-matrix` and `euv-automation` release tests, enforcing performance budgets, and atomically updating the single latest report at `performance/BENCHMARK_REPORT.md` (`--write-report`)
- `scripts/bench/stress-bench.sh` — Backward-compatible shell wrapper invoking `scripts/bench/run-stress-report.py`

## Installation & Usage

```sh
cargo run --manifest-path performance/Cargo.toml --release
cargo run --manifest-path performance/Cargo.toml --release -- --stress-matrix
./scripts/bench/stress-bench.sh
./scripts/bench/stress-bench.sh --runs 3 --write-report
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
