/// Canonical schema version identifier for versioned UI E2E flow files.
pub const UI_E2E_FLOW_SCHEMA_V1: &str = "euv.ui-e2e.v1";
/// Canonical schema version identifier for UI E2E verdict result artifacts.
pub const UI_E2E_RESULT_SCHEMA_V1: &str = "euv.ui-e2e.result.v1";
/// Canonical schema version identifier for locator-only migration proposal artifacts.
pub const UI_E2E_MIGRATION_SCHEMA_V1: &str = "euv.ui-e2e.migration.v1";
/// Error code emitted when zero visible nodes match a strict unique locator.
pub const E2E_ERR_LOCATOR_NOT_FOUND: &str = "LOCATOR_NOT_FOUND";
/// Error code emitted when more than one visible node matches a strict unique locator.
pub const E2E_ERR_AMBIGUOUS_LOCATOR: &str = "AMBIGUOUS_LOCATOR";
/// Error code emitted when a bounded wait condition expires without becoming satisfied.
pub const E2E_ERR_WAIT_TIMEOUT: &str = "WAIT_TIMEOUT";
/// Error code emitted when an explicit E2E assertion fails.
pub const E2E_ERR_ASSERTION_FAILED: &str = "ASSERTION_FAILED";
/// Error code emitted when a flow declares an unsupported schema version.
pub const E2E_ERR_UNSUPPORTED_SCHEMA: &str = "UNSUPPORTED_SCHEMA";
/// Error code emitted when the requested execution adapter is unsupported.
pub const E2E_ERR_UNSUPPORTED_ADAPTER: &str = "UNSUPPORTED_ADAPTER";
/// Error code emitted when the execution environment is blocked before running steps.
pub const E2E_ERR_ENVIRONMENT_BLOCKED: &str = "ENVIRONMENT_BLOCKED";
/// Error code emitted when a migration proposal attempts to alter original assertions.
pub const E2E_ERR_ASSERTION_MUTATION_FORBIDDEN: &str = "ASSERTION_MUTATION_FORBIDDEN";
/// Error code emitted when runtime stepping or action dispatch fails.
pub const E2E_ERR_RUNTIME_ERROR: &str = "RUNTIME_ERROR";
/// Error code emitted when a flow carries an approved retirement record.
pub const E2E_ERR_FLOW_RETIRED: &str = "FLOW_RETIRED";
/// Default bounded per-step timeout in milliseconds.
pub const UI_E2E_DEFAULT_TIMEOUT_MS: u64 = 5_000;
/// Maximum permitted bounded per-step timeout in milliseconds.
pub const UI_E2E_MAX_TIMEOUT_MS: u64 = 30_000;
/// Maximum permitted bounded clean retry count for transient failures.
pub const UI_E2E_MAX_RETRIES: u32 = 3;

pub(crate) const E2E_SECRET_REF_PREFIX: &str = "E2E_SECRET_";
pub(crate) const E2E_STEP_PREFIX_CLICK: &str = "click";
pub(crate) const E2E_STEP_PREFIX_TYPE: &str = "type";
pub(crate) const E2E_STEP_PREFIX_SCROLL: &str = "scroll";
pub(crate) const E2E_STEP_PREFIX_DRAG: &str = "drag";
pub(crate) const E2E_STEP_PREFIX_KEY: &str = "key";
pub(crate) const E2E_STEP_PREFIX_ASSERT: &str = "assert";
pub(crate) const E2E_STEP_PREFIX_WAIT: &str = "wait";
pub(crate) const E2E_MSG_ZERO_MATCHES: &str = "zero matches";
pub(crate) const E2E_MSG_MISSING_TARGET: &str = "step requires a target locator";
pub(crate) const E2E_MSG_UNSUPPORTED_SCHEMA: &str = "unsupported ui-e2e schema version";
pub(crate) const E2E_MSG_MISSING_SECRET: &str = "referenced secret value was not supplied";
pub(crate) const E2E_MSG_MISSING_FIXTURE: &str = "referenced fixture value was not supplied";
pub(crate) const E2E_MSG_NODE_STILL_VISIBLE: &str = "node remained visible";
pub(crate) const E2E_MSG_ASSERTION_MUTATED: &str = "locator migration must not modify assertions";
pub(crate) const E2E_MSG_LAUNCH_STEP: &str = "launch frame step";
pub(crate) const E2E_MSG_NOT_ACTIONABLE: &str = "node visible but not actionable";
pub(crate) const E2E_MSG_ZERO_OVERFLOW: &str = "zero horizontal overflow";
pub(crate) const E2E_MSG_ZERO_OVERLAP: &str = "zero text overlap";
pub(crate) const E2E_DEFAULT_ADAPTER: &str = "headless";
