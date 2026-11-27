---
name: euv
description: 'Use when implementing or debugging an euv cross-platform native Rust UI: view!/style! macros, signals, scene graph, 7-platform targets, SystemEvent, or euv CLI. Excludes AI bridge, design restoration, and E2E replay.'
license: MIT
metadata:
  hermes:
    related_skills: [rust-standards, euv-ui-standards, git-standards, euv-ai-automation, euv-design-restoration, euv-e2e-replay]
---

# euv workspace standards and workflow

## Mandatory Standards

1. Load `rust-standards` before editing any `.rs` or `Cargo.toml` file; load `euv-ui-standards` for `euv-ui` work. Resolve authoritative crate membership and version from `Cargo.toml` / `cargo metadata --no-deps --format-version 1`.
2. `euv` is a **7-platform direct-OS-API native Rust UI framework** (`Web`, `Android`, `iOS`, `iPadOS`, `macOS`, `Windows`, `Linux`) with zero WebView, zero DOM, and zero virtual DOM. `euv-ui` is a separate crate not re-exported by root `euv`.
3. Enforce declarative macro discipline (`AGENTS.md` §2):
  - Never embed multi-line business logic, event handlers, canvas painters, or reactive closures inline inside `view!` or other macros; bind them to sibling `let` variables or outer helper `fn`s first.
  - Pass custom component parameters inside `view!` exclusively via named attributes (`Component { prop: value }`, backed by `#[derive(Props)]`); positional `Component(...)` calls are rejected at compile time.
  - `style! { ... }` uses one rule per line with **no semicolons** (`;` is a compile error).
  - Preserve IDE Go-to-Definition bindings (`euv::view_elements`, `euv::style_props`, direct component type and prop field spans) and never emit `use` statements inside proc-macro bodies (`TextNodeContent` UFCS only).
  - Use ambient framework-managed `Context`, `#[component(retention = Freeze)]`, and `PageRetention::Freeze` for inactive routes with directional slide transitions.

4. Enforce runtime, lifecycle, gesture, and UI invariants (`AGENTS.md` §3):
  - **7-Platform `SystemEvent` & Background 0% CPU/GPU Freeze**: Normalize lifecycle/OS signals via `SystemEvent` (`Appear`, `Disappear`, `Foreground`, `Background`, `Inactive`, `Terminate`, `SafeAreaChanged`, `ThemeChanged`, `MemoryWarning`, `BackPressed`, `DeepLink`); suspend rendering, timers, media, and host frame pumps (`App::is_background_frozen`, `should_pump_frames`, `is_platform_frame_pump_paused`) on `Background` and restore state losslessly on `Foreground`.
  - **True Edge-to-Edge Fullscreen & Reactive SafeArea**: Consume reactive `Context::get_safe_area()` (`Signal<SafeAreaInsets>`) updated by `WindowEvent::SafeAreaChanged` (clearing notches and Dynamic/Atomic Island pills) and coordinate immersive fullscreen via `WindowCommand::SetFullscreen` / reactive fullscreen state.
  - **GestureArena & Single-Owner Scroll Containment**: Enforce exclusive gesture arbitration (`ActiveGesture`) with strict axis lock so route swipes and vertical page scrolls never leak deltas into nested horizontal scrollables (`EuvCodeBlock`, tables, tabs).
  - **Zero-Overshoot Overlays, Unified Controls & Charting**: Use strictly monotonic easing (`Easing::EaseOutQuint` / `Easing::EaseOutCubic`) and floating drag grabbers on `EuvDrawer`/`EuvSheet`; unified full-width dropdown triggers matching popup widths 1:1; `Overflow::Hidden` on cards/bubbles/alerts with explicit wrapped text width (`Length::Percent(100.0)` or `flex_shrink: 1.0` + `min_width: Length::Px(0.0)`); native monospace Eva Light/Dark Bold `EuvCodeBlock`; native vector charting (`EuvEChart` / `EuvChart`); zero `#[inline]` in WASM `cdylib` crates.

5. Keep `euv fmt` layered on `crate-cli`: `euv fmt` MUST invoke `crate_cli::execute_unified_fmt` (Prettier-aligned `.md`/`.yml`/`.json`/`.toml` + Rust derives + `cargo fmt`) first, then `format_workspace_macros` and `cargo fmt --all`.

## Verification Workflow

Run every gate locally before committing or pushing:

```sh
cargo run -p euv-cli -- fmt
cargo run -p euv-cli -- fmt
cargo run -p euv-cli -- fmt --check
cargo fmt --all -- --check
cargo clippy -p euv-ui --no-default-features --all-targets -- -D warnings
cargo clippy --all-features --workspace --all-targets -- -D warnings
cargo clippy --manifest-path performance/Cargo.toml --all-targets -- -D warnings
cargo test -p euv-ui --no-default-features
cargo test -p euv-ui --features theme-shadcn
cargo test --all-features --workspace
cargo test --manifest-path performance/Cargo.toml
python3 ~/.agents/skills/rust-standards/scripts/staged_file_gate.py .
python3 ~/.agents/skills/rust-standards/scripts/verify_dep_order.py .
```

For performance, stress-matrix, and cross-platform checks:

- Run canonical release benchmarks & interleaved A/B partial-damage parity via `python3 scripts/bench/run-stress-report.py --runs 3 --write-report` (or `./scripts/bench/stress-bench.sh --runs 3 --write-report`), updating only `performance/BENCHMARK_REPORT.md`.
- Run 7-target `RUSTFLAGS="-D warnings" cargo check -p euv-core -p euv -p euv-ui`, headless smokes (`euv-example --headless --software`, `euv-docs --headless`, `euv-cli doctor`), `./scripts/test/e2e-all.sh`, and visual screenshot verification on Web/Android/iOS for UI changes.

## Release Workflow

`euv` is **Tier 2** (depends on published `ctares` crates `crate-cli` and `lombok-macros`):

1. Wait until upstream `ctares` is live on `crates.io`, then upgrade `crate-cli` and `lombok-macros` in root `Cargo.toml`.
2. Bump `[workspace.package].version`, internal `[workspace.dependencies]` pins, and mobile/web build scripts (`scripts/build/build-android-apk.sh`, `scripts/build/build-ios-sim.sh`, `scripts/build/build-web.sh`).
3. Verify all local gates pass, validate English-only commit/PR text via `verify_english_only.py`, ship to `master`, and wait until `.github/workflows/rust.yml` (`check`, `tests`, `clippy`, `build`, `publish`, `release`) completes green.

## Documentation & Skill Governance

- Update root `README.md`, `<crate>/README.md`, `docs/`, `AGENTS.md`, and affected project skills under `.agents/skills/` **in the same commit/PR** per `AGENTS.md` §4.
- Keep every `SKILL.md` strictly to standards, contracts, and workflows (`<= 160` lines, pure English, valid YAML frontmatter), synchronized with any `~/.agents/skills/` mirror, and verified with `audit_skills.py` and `cargo run -p euv-cli -- fmt --check`.

## Reference Index

- Companion project skills: [AI Automation](../euv-ai-automation/SKILL.md) · [Design Restoration](../euv-design-restoration/SKILL.md) · [E2E Replay](../euv-e2e-replay/SKILL.md)
- Workspace docs: [AGENTS Governance](../../../AGENTS.md) · [Workspace README](../../../README.md) · [Automation README](../../../automation/README.md) · [Benchmark Report](../../../performance/BENCHMARK_REPORT.md)
- User-global prerequisite skills (loaded by skill name): `rust-standards` · `euv-ui-standards` · `git-standards`
