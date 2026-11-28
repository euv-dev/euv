---
name: euv-ai-automation
description: 'Use when automating, inspecting, or testing live euv apps via AI Schema v1/v2, DevAutomationBridge HTTP endpoints, UiAutomator, or layout anomaly checks. Excludes E2E recording/replay and visual mockup restoration.'
license: MIT
metadata:
  hermes:
    related_skills: [euv, euv-e2e-replay, euv-design-restoration, rust-standards]
---

# Euv AI UI Automation and DevAutomationBridge

## Mandatory Contracts

1. **Zero-Byte Release Stripping (`cfg(debug_assertions)`)**:
  - `DevAutomationBridge` (`automation/src/dev_gate/`) is compiled only under `#[cfg(all(debug_assertions, not(target_arch = "wasm32")))]`.
  - In release and WASM builds it compiles to a zero-sized no-op stub (`DevAutomationBridge::is_enabled() == cfg!(debug_assertions)`, `bound_address() == None`, `poll_budgeted(...) == 0`). Never ship live inspector ports or AI schema strings in release binaries.

2. **AI Schema v2 Same-Frame Protocol (`automation/src/schema/`)**:
  - Constants: `AI_SCHEMA_PROTOCOL_VERSION = "euv.ai.schema.v1"`, `AI_SCHEMA_V2_PROTOCOL_VERSION = "euv.ai.schema.v2"`, `AI_SCHEMA_REDACTED_VALUE = "[REDACTED]"`.
  - Use `AiSchemaProtocol::capture_same_frame(&app, include_layout_tree, include_rgba_hash, hooks)` (or `UiAutomator::capture_same_frame`) to return `AiSameFrameCapture { frame_id, scene_revision, snapshot, rgba_fnv1a, capture_source, synced }`.
  - `synced` is `true` only when pixel state and scene mutation revision match the same rendered frame (`last_rendered_revision == scene_revision && frame_id > 0` when `include_rgba_hash` is enabled). Always render (`step_frame` / `UiAutomator::step`) after mutations before capturing.
  - Every `AiLayoutNode` reports `layout_state: AiLayoutState::{Measured, Unmeasured}`, arena-independent `stable_key`, and author `test_id` (`test_id:` A11y hint or `SceneNode` key). Hidden (`Display::None`) or unlaid-out nodes MUST report `AiLayoutState::Unmeasured` with zeroed bounds, never fabricated geometry.

3. **Mandatory Secret & Password Redaction**:
  - Any `NodeKind::Input` with `password == true` or any `Input`/`Text` node whose key, `test_id`, `a11y_label`, or placeholder contains sensitive tokens (`password`, `passwd`, `secret`, `token`, `api_key`, `apikey`, `private_key`, `credential`, `ssn`, `cvv`) is automatically masked as `"[REDACTED]"` (`redacted: true`, counted in `redacted_field_count`) across `snapshot`, `snapshot_v2`, `accessibility_tree`, `query_nodes_direct`, and `hit_test_at`.

4. **Zero-Snapshot Direct `&SceneTree` Queries**:
  - `AiSchemaProtocol::query_nodes_direct`, `query_first_direct`, and `exists_direct` (wrapped by `UiAutomator::query_all`, `query`, `exists`) scan `SceneTree` directly without allocating an `AiRuntimeSnapshot` and maintain exact parity with `AiLayoutNode::find_matches`.
  - Supported `UiSelector` variants (`#[serde(tag = "type", content = "value")]`): `ByTestId`, `ByStableKey`, `ByA11yLabel`, `ByRole`, `ByKind`, `ByText`, `ByTextContains`, `ByPlaceholder`, `ByInputValue`, `ByNodeIndex`, `Interactive`, `Nth`, `All`. Prefer `ByTestId` / `ByStableKey` / `ByA11yLabel` over fragile text or index selectors.

## Non-Blocking Async `DevAutomationBridge`

- Start with `DevAutomationBridge::start(DEV_AI_INSPECTOR_ADDR)` (`"127.0.0.1:4174"`) or `start_with_capacity(addr, cap)` (`DEV_BRIDGE_QUEUE_CAPACITY = 16`).
- Attach host route, theme, and game controls via `.with_hooks(AiAppHooks::new().on_route(...).on_scheme(...).on_game_stats(...).on_navigate(...).on_set_theme(...).on_game_control(...))`.
- The resident background worker thread (`euv-dev-bridge`) isolates all TCP accept, non-blocking HTTP parsing (64-socket cap, 2 MiB max body), and JSON serialization off the UI thread.
- Saturated control queues reject immediately with HTTP `429 Too Many Requests` and `code: "BACKPRESSURE"` (`DEV_BRIDGE_ERR_BACKPRESSURE`, tracked by `backpressure_rejected()`).
- Drain commands on the UI thread strictly via `bridge.poll_budgeted(&mut app, budget_us)` (or `poll(&mut app)`, defaulting to `DEV_BRIDGE_DEFAULT_BUDGET_US = 2_000`; cap at `<= 500us` for 120Hz/144Hz frames so idle UI-thread poll `p99 <= 0.5ms`).

### Loopback HTTP Endpoints (`automation/src/dev_gate/impl.rs`)

| Method & Path                       | Handler / Request Envelope                                                           |
| :---------------------------------- | :----------------------------------------------------------------------------------- |
| `OPTIONS *`                         | Off-thread CORS preflight (`204 No Content`)                                         |
| `GET /ai/health`                    | Off-thread JSON health (`status`, `requests_served`, `backpressure_rejected`, etc.)  |
| `GET /schema`, `GET /ai/schema`     | `AiSchemaRequest::DescribeSchema` (`AI_SCHEMA_JSON_SPEC`)                            |
| `GET /snapshot`, `GET /ai/snapshot` | `AiSchemaRequest::Snapshot { include_layout_tree: true }`                            |
| `GET /ai/snapshot/v2`               | `AiSchemaRequest::SnapshotV2 { include_layout_tree: true, include_rgba_hash: true }` |
| `GET /ai/a11y`                      | `AiSchemaRequest::AccessibilityTree` (`Vec<AiA11yNode>`)                             |
| `POST /query`, `POST /ai/query`     | JSON `AiSchemaRequest` (`QueryNodes`, `HitTest`, `SnapshotV2`, etc.)                 |
| `POST /ai/action`                   | JSON `AiAction` (`Click`, `TypeText`, `Gesture`, `Scroll`, `Resize`, `Navigate`...)  |
| `POST /ai/script`                   | JSON `UiTestScript` -> `UiTestReport`                                                |
| `POST /ai/e2e`                      | JSON `UiE2eFlowV1` -> `euv.ui-e2e.result.v1` (`AiSchemaResponse::success_v2`)        |

## Closed-Loop Headless AI Verification Workflow (`UiAutomator`)

1. Wrap `App` with `UiAutomator::new(&mut app)` (or drive a running debug app over `DevAutomationBridge`).
2. **Query**: Locate target via `auto.query(&UiSelector::test_id(...))` / `auto.exists(...)` (or `POST /ai/query` with `{"op":"QueryNodes","selector":{"type":"ByTestId","value":"..."}}`).
3. **Act**: Execute `click`, `click_at`, `double_click`, `hover`, `focus`, `type_text`, `clear_input`, `drag`, `drag_points`, `gesture` (`AiGestureKind::{Tap, DoubleTap, Swipe, Pinch, LongPress}`), `scroll`, `press_key_name`, `resize`, or `POST /ai/action` (`{"kind":"Click","selector":{"type":"ByA11yLabel","value":"..."}}`).
4. **Step**: Advance deterministic frames via `auto.step()` / `auto.step_frames(n)` / `auto.step_game(dt_secs, frames)` (or wait via `auto.wait_until(&selector, max_frames)`).
5. **Verify Same-Frame + Zero Anomalies**: Call `auto.capture_same_frame(true, true)` and assert:
  - `capture.get_synced() == true` and `capture.get_frame_id() == capture.get_snapshot().get_frame_id()`;
  - expected text/state (`auto.assert_visible`, `assert_not_visible`, `assert_text_equals`);
  - zero layout defects (`capture.get_snapshot().get_anomalies().is_empty()`, covering `AiAnomalyKind::{HorizontalOverflow, NegativeBounds, ZeroSizeVisible, TextOverlap}`, or `auto.assert_no_horizontal_overflow()` and `auto.assert_no_text_overlap()`).

6. For multi-step declarative checks without E2E verdict persistence, run `UiTestScript` (`UiTestStep`) via `auto.run_script(&script)` and assert `report.get_passed() && report.get_final_anomalies().is_empty()`.

## Verification Commands

```sh
cargo test -p euv-automation --all-features
RUSTFLAGS="-D warnings" cargo test -p euv-automation --release
cargo clippy -p euv-automation --all-features --all-targets -- -D warnings
```

Confirm release stripping via `./scripts/test/e2e-all.sh` (asserting `strings target/debug/euv-example | grep -c "euv.ai.schema.v1" >= 1` and `strings target/release/euv-example | grep -c "euv.ai.schema.v1" == 0`).

## Related Skills

- [Euv Core](../euv/SKILL.md) · [E2E Replay](../euv-e2e-replay/SKILL.md) · [Design Restoration](../euv-design-restoration/SKILL.md) · [Automation README](../../../automation/README.md)
