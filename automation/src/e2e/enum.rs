use super::*;

/// Semantic unique-target locator used by `euv.ui-e2e.v1` steps (matching 0 or >1 nodes always fails).
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(tag = "by", content = "value")]
pub enum UiE2eLocator {
    /// Matches a node by its author-declared `test_id`.
    TestId(String),
    /// Matches a node by its arena-independent `stable_key`.
    StableKey(String),
    /// Matches a node by its explicit accessibility label.
    A11yLabel(String),
    /// Matches a node by its exact rendered text or input value.
    Text(String),
    /// Matches an input node by its exact placeholder text.
    Placeholder(String),
    /// Matches a node satisfying both accessibility role and label.
    RoleAndLabel {
        /// Accessibility role name (e.g. `"Button"`).
        role: String,
        /// Accessibility label or exact text.
        label: String,
    },
    /// Matches a node via a general `UiSelector`.
    Selector(UiSelector),
}

/// Interactive action verb executed inside a `UiE2eStepOp::Do` step.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UiE2eActionKind {
    /// Verifies app launch / steps an initial frame.
    Launch,
    /// Clicks the strictly unique target node.
    Click,
    /// Double-clicks the strictly unique target node.
    DoubleClick,
    /// Hovers pointer over the strictly unique target node.
    Hover,
    /// Focuses the strictly unique target node.
    Focus,
    /// Types non-secret or referenced text into the strictly unique target input.
    TypeText,
    /// Clears all characters in the strictly unique target input.
    ClearInput,
    /// Dispatches a keyboard key press by name.
    PressKey,
    /// Scrolls the strictly unique target container by `(dx, dy)`.
    Scroll,
    /// Drags starting from the strictly unique target center by `(dx, dy)`.
    Drag,
    /// Resizes the logical viewport and DPI scale.
    Resize,
}

/// Bounded wait predicate evaluated inside a `UiE2eStepOp::Wait` step.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UiE2eWaitKind {
    /// Waits until a unique matching node becomes visible.
    Visible,
    /// Waits until zero visible nodes match the target locator.
    NotVisible,
    /// Waits until a unique matching node is visible, interactive, and not disabled.
    Actionable,
    /// Waits until a unique matching node's text or input value equals `expected`.
    TextEquals,
}

/// Observable verification predicate evaluated inside a `UiE2eStepOp::Assert` step.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UiE2eAssertKind {
    /// Asserts that a unique matching node is visible.
    Visible,
    /// Asserts that zero visible nodes match the target locator.
    NotVisible,
    /// Asserts that a unique matching node has text or input value equal to `expected`.
    TextEquals,
    /// Asserts zero horizontal viewport overflow anomalies exist.
    NoHorizontalOverflow,
    /// Asserts zero sibling text overlap anomalies exist.
    NoTextOverlap,
}

/// Structured step operation discriminated cleanly into `do`, `wait`, or `assert` (never arbitrary sleeps).
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum UiE2eStepOp {
    /// Executes a user-level UI action.
    Do {
        /// Action verb to execute.
        action: UiE2eActionKind,
        /// Optional unique target locator.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        target: Option<UiE2eLocator>,
        /// Optional literal non-secret text payload.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        text: Option<String>,
        /// Optional secret environment/store reference name (never plaintext secret).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        secret_ref: Option<String>,
        /// Optional test fixture reference name.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        fixture_ref: Option<String>,
        /// Optional key name for `PressKey`.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        key: Option<String>,
        /// Horizontal scroll or drag delta.
        #[serde(default)]
        dx: f32,
        /// Vertical scroll or drag delta.
        #[serde(default)]
        dy: f32,
        /// Target logical width for `Resize`.
        #[serde(default)]
        width: u32,
        /// Target logical height for `Resize`.
        #[serde(default)]
        height: u32,
        /// Target DPI scale for `Resize`.
        #[serde(default)]
        scale: f32,
    },
    /// Waits up to bounded `timeout_ms` for an observable condition.
    Wait {
        /// Observable wait condition.
        condition: UiE2eWaitKind,
        /// Target unique locator.
        target: UiE2eLocator,
        /// Optional expected text value for `TextEquals`.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        expected: Option<String>,
    },
    /// Evaluates an explicit observable UI assertion.
    Assert {
        /// Assertion kind.
        assertion: UiE2eAssertKind,
        /// Optional target unique locator.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        target: Option<UiE2eLocator>,
        /// Optional expected text value for `TextEquals`.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        expected: Option<String>,
    },
}

/// Top-level execution verdict classified by the `euv.ui-e2e.result.v1` verdict engine.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UiE2eVerdict {
    /// All steps succeeded on the first attempt with zero failures.
    Passed,
    /// Flow failed due to locator error, assertion failure, or unrecovered timeout.
    #[default]
    Failed,
    /// First attempt failed with a transient error and a clean bounded retry subsequently passed.
    Flaky,
    /// Execution could not run due to an environment or infrastructure blocker.
    Blocked,
    /// Flow schema version or target adapter is unsupported by this executor.
    Unsupported,
    /// Flow carries an approved retirement record and was not treated as passed.
    Retired,
}

/// Per-step execution status inside `UiE2eResultV1`.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UiE2eStepStatus {
    /// Step completed and satisfied its precondition or assertion.
    Passed,
    /// Step failed with a structured error code and same-frame evidence.
    Failed,
    /// Step was skipped because an earlier step failed or the flow was retired/blocked.
    #[default]
    NotRun,
    /// Step was blocked by environment failure.
    Blocked,
    /// Step was unsupported on the active adapter.
    Unsupported,
}

/// Lifecycle review status of a locator-only `UiE2eMigrationProposal`.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UiE2eMigrationStatus {
    /// Migration patch has been generated and verified, awaiting human review.
    #[default]
    Proposed,
    /// Migration patch has been explicitly approved.
    Approved,
    /// Migration patch was rejected (e.g. attempted assertion mutation or failed candidate rerun).
    Rejected,
}
