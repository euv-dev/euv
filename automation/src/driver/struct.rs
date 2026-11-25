use super::*;

/// Declarative JSON-serializable end-to-end UI automation test script.
#[derive(Clone, Data, Debug, Deserialize, New, PartialEq, Serialize)]
pub struct UiTestScript {
    /// Human-readable test suite or scenario name.
    pub(crate) name: String,
    /// Ordered sequence of actions and assertions to execute.
    pub(crate) steps: Vec<UiTestStep>,
}

/// Execution telemetry and pass/fail result for a single `UiTestStep`.
#[derive(Clone, Data, Debug, Deserialize, New, PartialEq, Serialize)]
pub struct UiStepResult {
    /// Zero-based step index inside the script.
    #[get(type(copy))]
    pub(crate) step_index: usize,
    /// Step label summary.
    pub(crate) label: String,
    /// Whether the step succeeded.
    #[get(type(copy))]
    pub(crate) passed: bool,
    /// Elapsed wall time in microseconds.
    #[get(type(copy))]
    pub(crate) elapsed_us: u64,
    /// Optional diagnostic failure message.
    #[get(skip)]
    pub(crate) error: Option<String>,
}

/// Aggregated UI automation test run report.
#[derive(Clone, Data, Debug, Deserialize, New, PartialEq, Serialize)]
pub struct UiTestReport {
    /// Script name.
    pub(crate) name: String,
    /// Overall pass flag (`true` iff all steps passed).
    #[get(type(copy))]
    pub(crate) passed: bool,
    /// Number of steps that passed.
    #[get(type(copy))]
    pub(crate) passed_steps: usize,
    /// Number of steps that failed.
    #[get(type(copy))]
    pub(crate) failed_steps: usize,
    /// Per-step detailed execution results.
    pub(crate) results: Vec<UiStepResult>,
    /// Final detected layout anomalies after completing the script.
    pub(crate) final_anomalies: Vec<AiLayoutAnomaly>,
}

/// High-level UI automation driver operating directly over a live `euv::App` instance.
#[derive(CustomDebug)]
pub struct UiAutomator<'a> {
    /// Mutable reference to the application runtime under test.
    #[debug(skip)]
    pub(crate) app: &'a mut App,
}
