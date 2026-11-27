---
name: euv-e2e-replay
description: 'Use when recording, authoring, replaying, migrating, or gating euv UI E2E flows (euv.ui-e2e.v1 / euv.ui-e2e.result.v1), 50-cycle exit/re-enter stress, or scripts/test/e2e-all.sh. Excludes live DevAutomationBridge setup.'
license: MIT
metadata:
  hermes:
    related_skills: [euv, euv-ai-automation, euv-design-restoration, rust-standards]
---

# euv UI E2E recording, replay, and verdict gating

## 1. Recording Contract (`UiE2eRecorder`, `automation/src/e2e/`)

- Construct with `UiE2eRecorder::new(flow_id, revision, UiE2eTargetV1::new(entry, adapter, width, height, dpr))` and export via `export_flow()` or `export_json()`.
- **Locator Priority**: Always prefer `UiE2eLocator::TestId` > `StableKey` > `A11yLabel` > `RoleAndLabel` (use `Text`, `Placeholder`, or `Selector` only when guaranteed unique).
- **Gesture Coalescing**: Consecutive `record_scroll` and `record_drag_move` calls targeting the same `UiE2eLocator` merge automatically into one cumulative `(dx, dy)` step.
- **Zero Arbitrary Sleeps**: `record_idle_pause(_pause_ms)` is intentionally a no-op so human thinking pauses never become flaky sleeps; synchronize state transitions exclusively via bounded `Wait` steps (`record_wait_visible`).
- **Automatic Secret Redaction**: `record_type_text(target, text, is_secret)` redacts whenever `is_secret || target.indicates_secret()`, emitting `secret_ref` (`E2E_SECRET_<NORMALIZED_TARGET>`) instead of plaintext and incrementing `redacted_count`. Never persist raw secrets or tokens in flow JSON.
- **Incomplete Recording Guard**: Call `mark_event_gap()` on dropped input queue events (`incomplete == true`) so partial traces cannot pass off as complete flows.

## 2. Versioned Flow Schema (`UiE2eFlowV1` — `"euv.ui-e2e.v1"`)

- Validate schema version via `UiE2eFlowV1::from_json(raw)` (`UI_E2E_FLOW_SCHEMA_V1 = "euv.ui-e2e.v1"`).
- Every `UiE2eStepV1` carries a stable `id`, a bounded `timeout_ms` clamped by `UiE2eStepV1::clamp_timeout_ms` to `[1, 30_000]` (`UI_E2E_DEFAULT_TIMEOUT_MS = 5_000`, `UI_E2E_MAX_TIMEOUT_MS = 30_000`), and one `UiE2eStepOp`:
  - `Do` (`UiE2eActionKind`): `Launch`, `Click`, `DoubleClick`, `Hover`, `Focus`, `TypeText` (with `text`, `secret_ref`, or `fixture_ref`), `ClearInput`, `PressKey`, `Scroll`, `Drag`, `Resize`.
  - `Wait` (`UiE2eWaitKind`): `Visible`, `NotVisible`, `Actionable`, `TextEquals`.
  - `Assert` (`UiE2eAssertKind`): `Visible`, `NotVisible`, `TextEquals`, `NoHorizontalOverflow`, `NoTextOverlap`.
- **Strict Unique-Locator Resolution (`UiE2eLocator::resolve_unique`)**:
  - `0` matching visible nodes fails with `LOCATOR_NOT_FOUND` (`E2E_ERR_LOCATOR_NOT_FOUND`).
  - `> 1` matching visible nodes fails with `AMBIGUOUS_LOCATOR` (`E2E_ERR_AMBIGUOUS_LOCATOR`) without clicking an arbitrary first match.

## 3. Deterministic Replay & 6-Verdict Engine (`UiE2eReplayer` / `UiE2eResultV1`)

- Configure `UiE2eReplayer::for_build(app_build)` with `.with_max_retries(0..=3)` (`UI_E2E_MAX_RETRIES = 3`), `.with_max_wait_frames(n)`, `.with_secret(key, val)`, `.with_fixture(key, val)`, `.with_supported_adapters(list)`, and `.with_blocked_reason(reason)`.
- Execute single attempt via `replay_once(&mut app, &flow, run_id)` (or HTTP `POST /ai/e2e` / `AiSchemaRequest::RunE2eFlow`) or bounded clean retries via `replay_with_clean_retries(&flow, run_id, build_clean_app)`.
- Output artifact schema is `UI_E2E_RESULT_SCHEMA_V1 = "euv.ui-e2e.result.v1"` classifying into 6 mutually exclusive `UiE2eVerdict` values:

| Verdict       | Meaning & Mandatory Invariant                                                                                                                   |
| :------------ | :---------------------------------------------------------------------------------------------------------------------------------------------- |
| `Passed`      | All steps passed on attempt 1 with zero failures.                                                                                               |
| `Failed`      | Deterministic failure (`LOCATOR_NOT_FOUND`, `AMBIGUOUS_LOCATOR`, `ASSERTION_FAILED`, `RUNTIME_ERROR`) or unrecovered `WAIT_TIMEOUT`.            |
| `Flaky`       | Attempt 1 failed with transient `WAIT_TIMEOUT` and a clean retry (`<= 3`) passed; attempt 1's `first_failure` & `attempts` history stay intact. |
| `Blocked`     | Pre-execution blocker (`ENVIRONMENT_BLOCKED`) or missing runtime `secret_ref` / `fixture_ref`.                                                  |
| `Unsupported` | Unsupported flow schema (`UNSUPPORTED_SCHEMA`) or adapter (`UNSUPPORTED_ADAPTER`).                                                              |
| `Retired`     | Requires approved `UiE2eRetirementRecord` (`approved_by` and `effective_build` non-empty); unapproved records still execute and fail.           |

- **Trailing `NotRun` & Same-Frame Evidence**: Every step after the first failure (and all steps on `Blocked`/`Unsupported`/`Retired`) MUST be marked `UiE2eStepStatus::NotRun`. Executed and failed steps preserve `(frame_id, scene_revision, code, expected, actual, duration_ms)` alongside `first_failure`.

## 4. Locator-Only Migration Guard (`UiE2eMigrationProposal`)

- Schema: `UI_E2E_MIGRATION_SCHEMA_V1 = "euv.ui-e2e.migration.v1"`.
- When a UI refactor renames a locator, generate and verify a candidate patch via `UiE2eMigrationProposal::propose_and_verify(&flow, old_build, new_build, changes, &mut clean_app, run_id)` and attach it separately via `failed_res.attach_migration(proposal)` without overwriting the original `Failed` verdict.
- **Hard Guard (`assertions_changed: false`)**: `validate_preserves_assertions` and `propose_and_verify` reject any attempt to edit, add, or drop `UiE2eStepOp::Assert` steps with `ASSERTION_MUTATION_FORBIDDEN`.

## 5. Full E2E & Resilience Verification Workflow

1. **Automation & E2E Unit/Replay Suites** (debug + release):

  ```sh
  cargo test -p euv-automation --all-features
  RUSTFLAGS="-D warnings" cargo test -p euv-automation --release
  ```
2. **50-Cycle Exit/Re-Enter & Full Showcase Interactivity Stress**:
  - Run `cargo test --workspace --all-features` to verify real interactive showcase flows (`example/tests/showcase/fn.rs`), multi-touch handoff, background freeze/restore, and 50-cycle rapid same-process exit/background/terminate -> re-enter -> click/scroll responsiveness (`core/tests/`, `ui/tests/`, `example/tests/`).

3. **Full End-to-End Gate & Release Stripping Proof**:

  ```sh
  ./scripts/test/e2e-all.sh
  ```
  Verifies `cargo fmt --all -- --check`, workspace `clippy -D warnings`, `--all-features` + `--no-default-features` + `euv-automation --release` tests, all 7 cross-platform target triples, debug-present / release-zero `euv.ai.schema.v1` binary stripping, and release `euv-performance`.

## Related Skills

- [euv Core](../euv/SKILL.md) · [AI Automation](../euv-ai-automation/SKILL.md) · [Design Restoration](../euv-design-restoration/SKILL.md) · [Automation README](../../../automation/README.md)
