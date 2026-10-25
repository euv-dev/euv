use super::*;

/// One browser case with actual execution outcome and diagnostic status.
#[derive(Clone, Debug, Getter, New, Serialize)]
#[new(pub(crate))]
pub(crate) struct CaseResult {
    /// Stable case identifier.
    #[get(pub(crate))]
    pub(crate) name: String,
    /// Whether the case's browser assertions succeeded.
    #[get(pub(crate), type(copy))]
    pub(crate) passed: bool,
    /// Contextual assertion or browser-operation failure.
    #[get(pub(crate))]
    pub(crate) error: Option<String>,
    /// Measured wall-clock case duration in milliseconds.
    #[get(pub(crate), type(copy))]
    pub(crate) elapsed_ms: u128,
    /// Diagnostic-capture failure, if screenshot or DOM collection failed.
    #[get(pub(crate))]
    pub(crate) capture_error: Option<String>,
}

/// Exact identities of resources created and reclaimed by this run.
#[derive(Clone, Debug, Getter, New, Serialize)]
#[new(pub(crate))]
pub(crate) struct Resources {
    /// PID of the ChromeDriver started by this runner.
    #[get(pub(crate), type(copy))]
    pub(crate) driver_pid: u32,
    /// Driver port selected by the operating system.
    #[get(pub(crate), type(copy))]
    pub(crate) driver_port: u16,
    /// Optional PID of the fixture server started by this runner.
    #[get(pub(crate))]
    pub(crate) server_pid: Option<u32>,
    /// Optional owned fixture-server port.
    #[get(pub(crate))]
    pub(crate) server_port: Option<u16>,
    /// Browser session identifier actually returned by ChromeDriver.
    #[get(pub(crate))]
    pub(crate) session: String,
    /// Whether the session and both helper processes were reclaimed.
    #[get(pub(crate), type(copy))]
    pub(crate) cleaned_up: bool,
    /// Unmodified browser capabilities identify the owned Chrome process and profile.
    #[get(pub(crate))]
    pub(crate) browser_capabilities: Value,
}

/// Raw browser automation report, including failures instead of silently skipping them.
#[derive(Clone, Debug, Getter, New, Serialize)]
#[new(pub(crate))]
pub(crate) struct SuiteReport {
    /// Report format revision.
    #[get(pub(crate), type(copy))]
    pub(crate) schema_version: u32,
    /// URL of the actual tested fixture.
    #[get(pub(crate))]
    pub(crate) url: String,
    /// Every executed browser case, in execution order.
    #[get(pub(crate))]
    pub(crate) cases: Vec<CaseResult>,
    /// Exact owned resource identities and teardown outcome.
    #[get(pub(crate))]
    pub(crate) resources: Resources,
}
