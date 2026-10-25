use super::*;

/// Prints CLI usage without running benchmark workloads.
pub(crate) const FLAG_HELP: &str = "--help";

/// Selects an isolated subsystem or the combined workload suite.
pub(crate) const FLAG_SUITE: &str = "--suite";

/// Overrides the number of measured frames in each round.
pub(crate) const FLAG_TICKS: &str = "--ticks";

/// Assigns a source identifier to the generated report.
pub(crate) const FLAG_LABEL: &str = "--label";

/// Multiplies the fixture population sizes.
pub(crate) const FLAG_SCALE: &str = "--scale";

/// Writes the structured JSON report to an output file.
pub(crate) const FLAG_OUTPUT: &str = "--output";

/// Overrides the number of repeated measurement rounds.
pub(crate) const FLAG_ROUNDS: &str = "--rounds";

/// Sets the number of unmeasured warmup frames.
pub(crate) const FLAG_WARMUP: &str = "--warmup";

/// Selects every isolated workload and the combined frame.
pub(crate) const SUITE_ALL: &str = "all";

/// Names the reactive framework workload.
pub(crate) const SUITE_CORE: &str = "core";

/// Names the chart layout and display-list workload.
pub(crate) const SUITE_CHART: &str = "chart";

/// Names the physics and particle-recording workload.
pub(crate) const SUITE_ENGINE: &str = "engine";

/// Names a frame that executes all three subsystems together.
pub(crate) const SUITE_MIXED: &str = "mixed";

/// Identifies the source when the caller does not provide a label.
pub(crate) const DEFAULT_LABEL: &str = "worktree";

/// Defines the default execution order, including the real mixed frame.
pub(crate) const DEFAULT_SUITES: [Suite; 4] =
    [Suite::Core, Suite::Engine, Suite::Chart, Suite::Mixed];

/// Lists the supported workload-selection and measurement options.
pub(crate) const USAGE: &str = "euv-performance [--suite all|core|engine|chart|mixed] [--ticks N] [--rounds N] [--warmup N] [--scale N] [--label TEXT] [--output FILE]";
