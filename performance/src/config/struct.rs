use super::*;

/// Configuration shared by every isolated and mixed measurement.
#[derive(Clone, Data, Debug, New, Serialize)]
#[new(pub(crate))]
pub(crate) struct Config {
    /// Ordered isolated and mixed workloads selected for this run.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) suites: Vec<Suite>,
    /// Measured frames executed by every round.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) ticks: usize,
    /// Number of raw measurement rounds retained per workload.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) rounds: usize,
    /// Frames executed before steady-state recording begins.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) warmup: usize,
    /// Multiplier applied consistently to every fixture population.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) scale: usize,
    /// Source identifier preserved in the raw report.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) label: String,
    /// Optional JSON destination; absence selects standard output.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    pub(crate) output: Option<PathBuf>,
    /// Prints usage without constructing measurement fixtures.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    pub(crate) help: bool,
}
