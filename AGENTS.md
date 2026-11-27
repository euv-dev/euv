# AGENTS.md — euv Workspace Engineering & Documentation Governance

Mandatory architectural, macro authoring, UI/layout, formatting, release, and documentation governance rules for all contributors and AI coding agents working in the `euv` repository.

---

## 1. Repository Architecture & Crate Responsibility Matrix

`euv` is a unified Rust 2024 monorepo (`[workspace.package].version` in root `Cargo.toml`) delivering a zero-WebView, zero-VDOM, 7-platform native UI framework.

| Crate             | Path           | Primary Responsibility                                                                                                                                                                                                                                                                                                                                                                                                           |
| :---------------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `euv`             | `.` (`src/`)   | Umbrella facade crate re-exporting `euv-core` and `euv-macros` (does NOT re-export `euv-ui`)                                                                                                                                                                                                                                                                                                                                     |
| `euv-core`        | `core/`        | Generation-indexed reactive arena, Flexbox/Grid layout with O(1) clean-subtree cache & surgical partial-layout damage, flat retained scene graph, zero-cost DSL Go-to-Definition schemas (`view_elements`, `style_props`), OS fonts & exact `TextCaret`, CPU/GPU/Hybrid & Terminal TUI/CLI renderer, 2D/3D ray-traced game engine, pure-Rust media codecs, dynamic WASM OTA, and 7 OS backends with 0% CPU/GPU background freeze |
| `euv-macros`      | `macros/`      | Zero-overhead procedural macros: `view!`, `style!`, `tokens!`, `watch!`, `computed!`, `#[component]`, `#[event]`, `emit!`, `on_event!`, `#[global_state_store]`, `local_state!`, `global_state!`, `euv_main!`                                                                                                                                                                                                                    |
| `euv-ui`          | `ui/`          | Separate adaptive UI component library: 60+ components, 17 zero-CSS compiled Light/Dark themes, `PageRetention::Freeze` router, `EuvBottomNav`, gesture-draggable `EuvDrawer`/`EuvSheet`, unified 1:1 width dropdowns, GFM `EuvMarkdown` (vector Mermaid, Math, Alerts, Footnotes), and VS Code Eva Light/Dark Bold `EuvCodeBlock`                                                                                               |
| `euv-cli`         | `cli/`         | Developer CLI (`euv doctor`, `euv new`, `euv dev`, `euv build`, `euv docs`, parallel `euv fmt` layered on `crate-cli`)                                                                                                                                                                                                                                                                                                           |
| `euv-docs`        | `docs/`        | Zero-JS static documentation site generator powered by `euv` + `euv-ui` with Eva Bold highlighting, inline Mermaid SVGs, and live search                                                                                                                                                                                                                                                                                         |
| `euv-automation`  | `automation/`  | Deterministic headless `UiAutomator`, `euv.ui-e2e.v1` recorder/replayer, and dev-only async AI Schema v2 bridge (`cfg(debug_assertions)`, 0 bytes in release)                                                                                                                                                                                                                                                                    |
| `euv-performance` | `performance/` | Reproducible 27-benchmark suite, `--stress-matrix` A/B partial-damage & tail-latency verifier, canonical `scripts/bench/run-stress-report.py` runner, and single latest `performance/BENCHMARK_REPORT.md`                                                                                                                                                                                                                        |
| `euv-example`     | `example/`     | Full 15-route cross-platform reference showcase (Bottom Nav, swipe routing, live FPS telemetry, 3D RT, Media Studio, Markdown Studio, Stress Lab)                                                                                                                                                                                                                                                                                |

---

## 2. Declarative Macro Authoring Rules (`view!`, `style!`, `watch!`, `computed!`)

### 2.1 Never Embed Full Logic or Multi-Line Closures Directly Inside Macros

- **Do NOT write business logic, multi-statement event handlers, canvas painters, or reactive text/children builder closures inline inside `view! { ... }` or other `euv` macros.**
- **Instead**: define the handler or reactive closure as a sibling `let` binding immediately before the macro invocation (or as a standalone outer helper `fn`), and pass only the identifier into the macro.
- **Good**:

  ```rust
  let on_save = move |_: &UiEvent| {
      let next: String = draft.get_untracked().trim().to_string();
      if !next.is_empty() {
          let _: bool = saved.set_if_changed(next);
      }
  };
  let summary_text = move || -> String {
      let entries: Vec<EuvRouteLifecycleEntry> = entries_sig.get();
      format!("Cached: {}", entries.len())
  };
  view! {
      EuvButton {
          label: BTN_SAVE
          tokens: tokens
          on_click: on_save
      }
      text {
          { summary_text }
      }
  }
  ```

### 2.2 `style!` Macro Syntax: One Rule Per Line, No Semicolons

- Every statement inside `style! { ... }` must sit on its own line **without** trailing semicolons `;`.
- Placing two rules on the same line or adding trailing `;` is a compile-time error.

### 2.3 Named Component Props, IDE Go-to-Definition & Zero Macro-Emitted `use`

- Built-in `view!` tags (`view`, `row`, `column`, `col`, `grid`, `scroll`, `portal`, `fragment`, `text`, `input`, `image`, `canvas`, `slot`, `page_slot`) and properties resolve directly to zero-cost schemas in `euv::view_elements`; `style!` keys resolve to `euv::style_props`.
- Custom components in `view!` MUST derive `Props` (implementing `ComponentProps`) and receive parameters exclusively as self-documenting named attributes inside `Component { prop: value }` blocks; positional `Component(...)` calls are rejected at compile time.
- Custom component invocations preserve the type path span (`<#path as ::euv::ComponentProps>::init_props()`) and field spans on generated setters, keeping `::euv::Component::mount(...)` on `Span::call_site()`.
- Proc-macro expansions MUST NEVER emit `use` statements inside generated blocks; use absolute `::crate::Item` paths and UFCS traits (`TextNodeContent`).

---

## 3. UI Component, Layout & Quality Standards

- **Zero-Overshoot Overlays & Gesture Drawers**: Drawers (`EuvDrawer`, `EuvAppShell` left drawer) and sheets (`EuvSheet`) must use strictly monotonic easing (`Easing::EaseOutQuint` / `Easing::EaseOutCubic`, never overshooting springs) and provide interactive floating grabber handles supporting drag-to-close and drag-to-expand.
- **Unified Dropdown & Trigger Widths**: Dropdowns (`EuvDropdown`, `EuvSelect`, `EuvCombobox`, `EuvNativeSelect`, `EuvDatePicker`) must use the unified full-width control trigger (`height: g.get_control_md()`, `padding: Edges::symmetric(0.0, SPACE_MD)`, `background: p.get_background()`, crisp vector chevron) and match popup list width 1:1 with the trigger width.
- **No Card Text Overflow**: All card/bubble/alert containers must set `overflow: Overflow::Hidden`, and wrapped `text` nodes inside cards must explicitly specify `width: Length::Percent(100.0)` (or `flex_shrink: 1.0` + `min_width: Length::Px(0.0)` inside rows) so text wraps cleanly across `360px` to `4K`.
- **Strict Code & Theme Fidelity**: Code blocks (`EuvCodeBlock`) use native monospace glyphs, preserve leading indentation via `TextOverflow::Clip`, and align strictly to VS Code `Eva Light Bold` / `Eva Dark Bold` palettes.
- **Unified Layered Formatting**: `euv fmt` executes `crate_cli::execute_unified_fmt` (Prettier-aligned `.md`/`.yml`/`.json`/`.toml` + Rust derives + `cargo fmt`) first, then normalizes `view!`/`style!`/`tokens!` macro blocks and runs `cargo fmt --all`. Always verify double-run idempotency (`euv fmt` twice, then `euv fmt --check` and `cargo fmt --all -- --check`).
- **Rust Standards & Gates**: Enforce 9-keyword filenames, 3-section `mod.rs`, centralized imports, explicit types on `let` and closure parameters, zero `#[allow]`, zero `self.field` outside accessors/Debug/Display/constructors, zero `#[inline]` in WASM `cdylib` crates, zero warnings under `cargo clippy --all-features --workspace --all-targets -- -D warnings`, and zero new violations under `staged_file_gate.py` & `verify_dep_order.py`.

---

## 4. Unified Documentation & Skill Maintenance Policy

All workspace documentation and project skills under `.agents/skills/` are first-class engineering artifacts governed by a single maintenance contract.

### 4.1 Project Skill Registry

| Skill Name               | Path                                             | Authoritative Domain                                                                                                                                        |
| :----------------------- | :----------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `euv`                    | `.agents/skills/euv/SKILL.md`                    | Workspace architecture, declarative macro discipline (§2), 7-platform `SystemEvent`/bg-freeze, fullscreen/SafeArea, GestureArena, verification & releases   |
| `euv-ai-automation`      | `.agents/skills/euv-ai-automation/SKILL.md`      | AI Schema v1/v2 (`euv.ai.schema.v2`), non-blocking `DevAutomationBridge`, `UiAutomator`, secret redaction, direct `SceneTree` queries, anomaly verification |
| `euv-design-restoration` | `.agents/skills/euv-design-restoration/SKILL.md` | Static mockup/image restoration, video/motion keyframe restoration, `euv-ui` mapping (`EuvEChart`, `EuvCodeBlock`, `EuvDrawer`/`EuvSheet`), visual QA       |
| `euv-e2e-replay`         | `.agents/skills/euv-e2e-replay/SKILL.md`         | `UiE2eRecorder`, `euv.ui-e2e.v1` flows, `UiE2eReplayer` 6-verdict engine (`euv.ui-e2e.result.v1`), `UiE2eMigrationProposal`, 50-cycle stress, `e2e-all.sh`  |

### 4.2 When to Update Skills & Docs (Mandatory Trigger Matrix)

Update the listed skills and documentation **in the exact same commit or PR** whenever any trigger below occurs:

| Change Trigger                                                                       | Target Skill(s) & Document(s)                                                                                                                           | Mandatory Update Action                                                                                                           |
| :----------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------ | :-------------------------------------------------------------------------------------------------------------------------------- |
| Add, rename, or remove a workspace crate, 7-platform backend, or `SystemEvent` hook  | `.agents/skills/euv/SKILL.md`, root `README.md`, `<crate>/README.md`, `AGENTS.md`                                                                       | Update crate matrix, lifecycle/freeze contracts, and cross-platform verification commands.                                        |
| Add or change macros (`view!`/`style!`/`tokens!`), `euv-ui` components, or UI rules  | `.agents/skills/euv/SKILL.md`, `.agents/skills/euv-design-restoration/SKILL.md`, `macros/README.md`, `ui/README.md`, `docs/docs/en/guide/`, `AGENTS.md` | Document exact syntax, component mappings, SafeArea/GestureArena invariants, and minimal §2-compliant examples.                   |
| Add or change AI Schema v1/v2 fields, `DevAutomationBridge` endpoints, or redaction  | `.agents/skills/euv-ai-automation/SKILL.md`, `automation/README.md`, `AGENTS.md`                                                                        | Sync protocol version constants, HTTP routes, queue/latency budgets, `UiSelector`/`AiAction` variants, and release-strip checks.  |
| Add or change `euv.ui-e2e.v1` / `result.v1` / `migration.v1` schemas or `e2e-all.sh` | `.agents/skills/euv-e2e-replay/SKILL.md`, `automation/README.md`, `AGENTS.md`                                                                           | Sync recorder/replayer rules, verdict codes, `assertions_changed: false` guard, and full E2E verification commands.               |
| Add or change CLI subcommands (`euv fmt`), build scripts, benchmarks, or motion QA   | `.agents/skills/euv/SKILL.md`, `.agents/skills/euv-design-restoration/SKILL.md`, `cli/README.md`, `scripts/README.md`, `AGENTS.md`                      | Sync CLI commands, `scripts/bench/run-stress-report.py` / `performance/BENCHMARK_REPORT.md` workflow, and visual/latency budgets. |
| Fix a recurring bug class, review rejection, or CI/hook failure mode                 | Relevant `.agents/skills/<skill>/SKILL.md`, `AGENTS.md`                                                                                                 | Codify the root-cause invariant and deterministic regression gate so the defect class cannot recur.                               |
| Upgrade upstream `ctares` dependencies or cut a minor release                        | Root `Cargo.toml`, `scripts/build/build-{android-apk,ios-sim,web}.sh`, `.agents/skills/euv/SKILL.md`                                                    | Wait until `ctares` is live on `crates.io`, bump versions, run `euv fmt` (2x) + all local gates, and verify green CI + publish.   |

### 4.3 How to Update Skills (Mandatory 6-Step Procedure)

1. **Read Authoritative Source First**: Inspect the real Rust source, tests, schemas, and scripts before editing any skill. Never invent or guess struct names, method signatures, HTTP routes, CLI flags, or file paths.
2. **Enforce Concise Structure & Budgets**: Keep every `.agents/skills/<skill>/SKILL.md` strictly to standards, contracts, requirements, and executable workflows in **pure English**, with valid YAML frontmatter (`name` matching folder, concise trigger `description` `<= 240` chars / `<= 60` words) and **`<= 160` lines**. Place bulky lookup tables in `references/` only when genuinely required, and prune obsolete or duplicated prose.
3. **Keep User-Global Mirrors Synchronized**: When maintaining a matching user-global skill under `~/.agents/skills/<skill>/SKILL.md`, synchronize its standards and workflows while keeping every relative link valid in both trees.
4. **Verify Every Referenced Symbol & Path**: Confirm every Rust type, schema identifier, HTTP endpoint, CLI command, relative link, and script path cited in the updated skill exists in the repository (preserve YAML frontmatter in `docs/README.md` and `cli/README.md`).
5. **Run Formatting, English & Structural Audit Gates**:

  ```sh
  wc -l .agents/skills/*/SKILL.md
  cargo run -p euv-cli -- fmt
  cargo run -p euv-cli -- fmt
  cargo run -p euv-cli -- fmt --check
  python3 ~/.agents/skills/git-standards/scripts/verify_english_only.py text AGENTS.md .agents/skills/*/SKILL.md
  python3 ~/.agents/scripts/audit_skills.py --repo .agents
  ```
6. **Land Atomically in the Same Commit / PR**: Always commit `.agents/skills/**` and `AGENTS.md` changes in the **same commit or PR** as the triggering engineering change — never defer skill updates to a follow-up PR.
