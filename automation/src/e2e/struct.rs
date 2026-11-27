use super::*;

/// Target execution environment metadata stored inside a `UiE2eFlowV1`.
#[derive(Clone, Data, Debug, Deserialize, New, PartialEq, Serialize)]
pub struct UiE2eTargetV1 {
    /// Application entry name (e.g. `"showcase"`).
    pub(crate) entry: String,
    /// Target UI adapter identifier (e.g. `"headless"`, `"native-window"`, `"web"`).
    pub(crate) adapter: String,
    /// Logical viewport width in points.
    #[get(type(copy))]
    pub(crate) width: u32,
    /// Logical viewport height in points.
    #[get(type(copy))]
    pub(crate) height: u32,
    /// Display scale factor (`dpr`).
    #[get(type(copy))]
    pub(crate) dpr: f32,
}

/// Explicit approved retirement record required before a flow may be classified as `UiE2eVerdict::Retired`.
#[derive(Clone, Data, Debug, Deserialize, Eq, New, PartialEq, Serialize)]
pub struct UiE2eRetirementRecord {
    /// Reviewer or change-approval authority that approved feature retirement.
    pub(crate) approved_by: String,
    /// Application build or release version in which the feature was retired.
    pub(crate) effective_build: String,
    /// Human-readable reason explaining the retirement.
    pub(crate) reason: String,
    /// Optional replacement E2E flow identifier covering successor functionality.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[get(skip)]
    pub(crate) replacement_flow_id: Option<String>,
}

/// Single versioned step inside a `UiE2eFlowV1` with explicit step ID and bounded timeout.
#[derive(Clone, Data, Debug, Deserialize, New, PartialEq, Serialize)]
pub struct UiE2eStepV1 {
    /// Stable unique step identifier within the flow.
    pub(crate) id: String,
    /// Declarative `do`, `wait`, or `assert` operation.
    pub(crate) operation: UiE2eStepOp,
    /// Bounded step timeout in milliseconds (`1..=UI_E2E_MAX_TIMEOUT_MS`).
    #[get(type(copy))]
    pub(crate) timeout_ms: u64,
}

/// Versioned `euv.ui-e2e.v1` executable UI E2E flow shared across manual recording, AI runs, CLI, and CI.
#[derive(Clone, Data, Debug, Deserialize, PartialEq, Serialize)]
pub struct UiE2eFlowV1 {
    /// Protocol schema version (`"euv.ui-e2e.v1"`).
    pub(crate) schema: String,
    /// Stable flow identifier (e.g. `"login-to-orders"`).
    pub(crate) id: String,
    /// Monotonic flow revision number.
    #[get(type(copy))]
    pub(crate) revision: u32,
    /// Target viewport and adapter specification.
    pub(crate) target: UiE2eTargetV1,
    /// Ordered sequence of `do`, `wait`, and `assert` steps.
    pub(crate) steps: Vec<UiE2eStepV1>,
    /// Optional explicit retirement approval record (must be complete for `Retired` classification).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[get(skip)]
    pub(crate) retirement: Option<UiE2eRetirementRecord>,
}

/// Per-step execution result recorded into `UiE2eResultV1`, including `NotRun` for skipped trailing steps.
#[derive(Clone, Data, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct UiE2eStepResultV1 {
    /// Step identifier matching `UiE2eStepV1::id`.
    pub(crate) id: String,
    /// Step execution outcome (`Passed`, `Failed`, `NotRun`, `Blocked`, `Unsupported`).
    #[get(type(copy))]
    pub(crate) status: UiE2eStepStatus,
    /// Optional stable machine-readable error code when `status == Failed`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[get(skip)]
    pub(crate) code: Option<String>,
    /// Optional expected condition description on failure.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[get(skip)]
    pub(crate) expected: Option<String>,
    /// Optional observed runtime state description on failure.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[get(skip)]
    pub(crate) actual: Option<String>,
    /// Elapsed wall duration of the step in milliseconds.
    #[get(type(copy))]
    pub(crate) duration_ms: u64,
    /// Same-frame identifier captured when the step finished or failed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[get(skip)]
    pub(crate) frame_id: Option<u64>,
    /// Same-frame scene revision captured when the step finished or failed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[get(skip)]
    pub(crate) scene_revision: Option<u64>,
}

/// Single execution attempt record preserved inside `UiE2eResultV1::attempts` so retries never overwrite initial failures.
#[derive(Clone, Data, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct UiE2eAttemptRecord {
    /// One-based attempt number (`1` = initial run, `2+` = bounded clean retries).
    #[get(type(copy))]
    pub(crate) attempt: u32,
    /// Verdict produced by this individual attempt.
    #[get(type(copy))]
    pub(crate) verdict: UiE2eVerdict,
    /// Per-step results recorded during this attempt.
    pub(crate) steps: Vec<UiE2eStepResultV1>,
    /// Optional failing step ID for this attempt.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[get(skip)]
    pub(crate) first_failure_step_id: Option<String>,
    /// Optional failing error code for this attempt.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[get(skip)]
    pub(crate) first_failure_code: Option<String>,
}

/// Single locator update entry inside a `UiE2eMigrationProposal`.
#[derive(Clone, Data, Debug, Deserialize, Eq, New, PartialEq, Serialize)]
pub struct UiE2eLocatorChange {
    /// Target step identifier whose locator changed.
    pub(crate) step_id: String,
    /// Previous locator that failed to resolve.
    pub(crate) old_locator: UiE2eLocator,
    /// Proposed replacement locator verified against the updated UI.
    pub(crate) new_locator: UiE2eLocator,
    /// Human-readable evidence justifying the locator mapping.
    pub(crate) evidence: String,
}

/// Reviewable locator-only migration proposal carrying a mandatory `assertions_changed: false` invariant.
#[derive(Clone, Data, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct UiE2eMigrationProposal {
    /// Migration artifact schema version (`"euv.ui-e2e.migration.v1"`).
    pub(crate) schema: String,
    /// Target flow identifier.
    pub(crate) flow_id: String,
    /// Original flow revision being migrated.
    #[get(type(copy))]
    pub(crate) from_revision: u32,
    /// Application build tag where the flow previously passed.
    pub(crate) old_build: String,
    /// Application build tag where the UI locator changed.
    pub(crate) new_build: String,
    /// Review status (`Proposed`, `Approved`, `Rejected`).
    #[get(type(copy))]
    pub(crate) status: UiE2eMigrationStatus,
    /// Must remain `false` for locator-only migration proposals; mutating assertions is forbidden.
    #[get(type(copy))]
    pub(crate) assertions_changed: bool,
    /// Ordered list of proposed locator replacements.
    pub(crate) changes: Vec<UiE2eLocatorChange>,
    /// Verdict observed when replaying the candidate migrated flow from a clean environment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[get(skip)]
    pub(crate) candidate_rerun_verdict: Option<UiE2eVerdict>,
}

/// Executor-produced `euv.ui-e2e.result.v1` verdict artifact preserving initial failure records and bounded retry history.
#[derive(Clone, Data, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct UiE2eResultV1 {
    /// Result schema version (`"euv.ui-e2e.result.v1"`).
    pub(crate) schema: String,
    /// Unique execution run identifier.
    pub(crate) run_id: String,
    /// Executed flow identifier.
    pub(crate) flow_id: String,
    /// Executed flow revision.
    #[get(type(copy))]
    pub(crate) flow_revision: u32,
    /// Application build tag tested during this run.
    pub(crate) app_build: String,
    /// Final verdict (`Passed`, `Failed`, `Flaky`, `Blocked`, `Unsupported`, `Retired`).
    #[get(type(copy))]
    pub(crate) verdict: UiE2eVerdict,
    /// Per-step results from the primary attempt (preserving `NotRun` on skipped trailing steps).
    pub(crate) steps: Vec<UiE2eStepResultV1>,
    /// Bounded attempt history preserving attempt #1's failure without overwriting on retry.
    pub(crate) attempts: Vec<UiE2eAttemptRecord>,
    /// Preserved initial failure step record whenever attempt #1 failed (even if retry produced `Flaky`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[get(skip)]
    pub(crate) first_failure: Option<UiE2eStepResultV1>,
    /// Optional locator-only migration proposal attached separately from the original failed verdict.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[get(skip)]
    pub(crate) migration: Option<UiE2eMigrationProposal>,
}

/// Semantic UI E2E action recorder merging consecutive scroll/drag moves, redacting secrets, and never emitting idle sleeps.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct UiE2eRecorder {
    /// Flow identifier assigned to the recorded flow.
    pub(crate) flow_id: String,
    /// Monotonic flow revision number.
    #[get(type(copy))]
    pub(crate) revision: u32,
    /// Target viewport and adapter specification.
    pub(crate) target: UiE2eTargetV1,
    /// Accumulated semantic E2E steps.
    pub(crate) steps: Vec<UiE2eStepV1>,
    /// Monotonic step sequence counter for deterministic step IDs.
    #[get(type(copy))]
    pub(crate) next_seq: u32,
    /// Number of sensitive/password inputs redacted into `secret_ref` placeholders during recording.
    #[get(type(copy))]
    pub(crate) redacted_count: usize,
    /// Flag raised if any input event gap occurred, preventing incomplete recordings from masquerading as complete flows.
    #[get(type(copy))]
    pub(crate) incomplete: bool,
}

/// Deterministic `euv.ui-e2e.v1` flow replayer and `euv.ui-e2e.result.v1` verdict engine.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct UiE2eReplayer {
    /// Application build identifier embedded in result artifacts.
    pub(crate) app_build: String,
    /// Maximum bounded clean retry attempts for transient `WAIT_TIMEOUT` failures (`0..=UI_E2E_MAX_RETRIES`).
    #[get(type(copy))]
    pub(crate) max_retries: u32,
    /// Maximum frames stepped per wait loop when evaluating bounded wait conditions in deterministic headless mode.
    #[get(type(copy))]
    pub(crate) max_wait_frames: usize,
    /// Runtime secret values mapped by `secret_ref` name (never written back to flow files or logs).
    pub(crate) secrets: HashMap<String, String>,
    /// Runtime fixture values mapped by `fixture_ref` name.
    pub(crate) fixtures: HashMap<String, String>,
    /// Whitelist of supported adapter names (empty means accept all adapters).
    pub(crate) supported_adapters: Vec<String>,
    /// Optional environment blocker message causing runs to classify as `UiE2eVerdict::Blocked`.
    #[get(skip)]
    pub(crate) blocked_reason: Option<String>,
}
