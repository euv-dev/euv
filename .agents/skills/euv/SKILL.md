---
name: euv
description: 'Use when implementing or debugging an euv cross-platform native Rust UI: view!/style! macros, signals, scene graph, mounting, 7-platform targets and rendering modes, or driving the euv CLI. euv-ui components are a separate crate.'
license: MIT
metadata:
  hermes:
    related_skills: [rust-standards, euv-ui-standards, git-standards]
---

# euv workspace standards and workflow

## Mandatory Standards

1. Load `rust-standards` before editing any `.rs` or `Cargo.toml` file; load `euv-ui-standards` for `euv-ui` work. Resolve authoritative crate membership and version from `Cargo.toml` / `cargo metadata --no-deps --format-version 1`.
2. `euv` is a **7-platform direct-OS-API native Rust UI framework** (Web, Android, iOS, iPadOS, macOS, Windows, Linux) with zero WebView, zero DOM, and zero virtual DOM. `euv-ui` is a separate crate not re-exported by root `euv`.
3. Enforce declarative macro discipline (`AGENTS.md` §1):
  - Never embed multi-line business logic, event handlers, canvas painters, or reactive closures inline inside `view!` or other macros; bind them to sibling `let` variables or helper `fn`s first.
  - Pass custom component parameters inside `view!` exclusively via self-documenting named attributes (`Component { prop: value }`, backed by `#[derive(Props)]`); positional `Component(...)` calls are rejected at compile time.
  - `style! { ... }` uses one rule per line with **no semicolons** (`;` is a compile error).
  - Preserve IDE go-to-definition bindings (`euv::view_elements`, `euv::style_props`, direct component type and prop field spans) and never emit `use` statements inside proc-macro bodies (`TextNodeContent` UFCS only).
  - Use ambient framework-managed `Context` (zero explicit `in ctx`), `#[component(retention = Freeze)]`, and `PageRetention::Freeze` for inactive routes with directional slide transitions.

4. Enforce UI & rendering invariants (`AGENTS.md` §2):
  - Zero-overshoot monotonic easing (`EaseOutQuint`/`EaseOutCubic`) and floating interactive drag grabbers on drawers and sheets.
  - Unified full-width dropdown triggers matching popup menu widths 1:1.
  - `Overflow::Hidden` on card/bubble/alert containers and explicit width/shrink constraints on wrapped text so zero horizontal overflow occurs from `360px` to `4K`.
  - Native monospace glyphs and exact VS Code Eva Light/Dark Bold styling in `EuvCodeBlock`; zero `#[inline]` in WASM `cdylib` crates.

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

Also run 7-target `RUSTFLAGS="-D warnings" cargo check -p euv-core -p euv -p euv-ui`, headless smokes (`euv-example --headless --software`, `euv-docs --headless`, `euv-cli doctor`), and visual screenshot verification on Web/Android/iOS for UI changes.

## Release Workflow

`euv` is **Tier 2** (depends on published `ctares` crates `crate-cli` and `lombok-macros`):

1. Wait until upstream `ctares` is live on `crates.io`, then upgrade `crate-cli` and `lombok-macros` in root `Cargo.toml`.
2. Bump `[workspace.package].version`, internal `[workspace.dependencies]` pins, and mobile/web build scripts (`scripts/build/build-android-apk.sh`, `build-ios-sim.sh`, `build-web.sh`).
3. Verify all local gates pass, validate English-only commit/PR text via `verify_english_only.py`, ship to `master`, and wait until `.github/workflows/rust.yml` (`check`, `tests`, `clippy`, `build`, `publish`, `release`) completes green.

## Documentation & Skill Governance

- Update root `README.md`, `<crate>/README.md`, `docs/`, and `AGENTS.md` whenever adding crates, macros, UI components, CLI flags, or scripts.
- Keep `.agents/skills/euv/SKILL.md` and `~/.agents/skills/euv/SKILL.md` synchronized and strictly limited to standards, requirements, and workflows (`<= 160` lines); run `audit_skills.py` and `update_skill_lock.py` after edits.

## Reference Index

Consult cached reference docs only as secondary lookup (source and compiler remain authoritative): [Macros](references/api-macros.md) · [Reactivity](references/reactive.md) · [Scene](references/scene.md) · [Layout](references/layout.md) · [Rendering](references/render.md) · [Platform](references/platform.md) · [UI](references/theme.md) · [Core API](references/api-core.md) · [UI API](references/api-ui.md) · [CLI API](references/api-cli.md) · [Docs API](references/api-docs.md) · [Automation](references/api-automation.md) · [Performance](references/api-performance.md) · [Pitfalls](references/pitfalls.md) · [UI Standards](../euv-ui-standards/SKILL.md).
