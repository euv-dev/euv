use super::*;

/// Executes the CLI and writes raw measurements without swallowing failures.
///
/// # Returns
///
/// - `Result<(), Box<dyn Error>>` - Success or the configuration, measurement or output error.
pub fn run() -> Result<(), Box<dyn Error>> {
    let config: Config = Config::parse(env::args().skip(1))?;
    if config.get_help() {
        println!("{USAGE}");
        return Ok(());
    }
    let Output {
        status,
        stdout,
        stderr,
    }: Output = Command::new(COMPILER).arg(COMPILER_FLAG).output()?;
    if !status.success() {
        return Err(format!(
            "compiler metadata failed: {}",
            String::from_utf8_lossy(&stderr)
        )
        .into());
    }
    let mut measurements: Vec<Measurement> = Vec::with_capacity(config.get_suites().len());
    for &suite in config.get_suites() {
        measurements.push(measure(&config, suite)?);
    }
    if config.get_suites().as_slice() == DEFAULT_SUITES.as_slice()
        && measurements.len() != DEFAULT_SUITES.len()
    {
        return Err(format!("incomplete default suite coverage: {}", measurements.len()).into());
    }
    let mode: &'static str = if cfg!(feature = "allocation-tracking") {
        MODE_ALLOCATIONS
    } else {
        MODE_TIMING
    };
    let environment: Environment = Environment::new(
        SOURCE_ROOT.to_string(),
        OS.to_string(),
        ARCH.to_string(),
        available_parallelism().map(NonZeroUsize::get).ok(),
        String::from_utf8(stdout)?,
        SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
        LIMITATIONS.to_string(),
    );
    let report: Report = Report::new(Header::new(2, mode, environment), config, measurements);
    for measurement in report.get_measurements() {
        for sample in measurement.get_samples() {
            eprintln!(
                "{:?} round={} elapsed_ns={:?} p50_ns={:?} p95_ns={:?} p99_ns={:?} allocations={:?} work=({},{},{}) checksum={}",
                measurement.get_suite(),
                sample.get_round(),
                sample.get_timings().try_get_elapsed_ns(),
                sample.get_timings().try_get_p50_ns(),
                sample.get_timings().try_get_p95_ns(),
                sample.get_timings().try_get_p99_ns(),
                sample.try_get_allocations(),
                sample.get_work().get_core_ticks(),
                sample.get_work().get_engine_ticks(),
                sample.get_work().get_chart_ticks(),
                sample.get_work().get_checksum()
            );
        }
    }
    let json: String = to_string_pretty(&report)?;
    if let Some(path) = report.get_config().try_get_output() {
        if let Some(parent) = path.parent()
            && !parent.as_os_str().is_empty()
        {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, json)?;
        eprintln!("report={}", path.display());
    } else {
        println!("{json}");
    }
    Ok(())
}

/// Measures a real suite, separating warmup and fixture allocation from ticks.
///
/// # Arguments
///
/// - `&Config` - Shared timing and scaling options.
/// - `Suite` - Subsystems to execute.
///
/// # Returns
///
/// - `Result<Measurement, Box<dyn Error>>` - Verified samples or a coverage failure.
pub(crate) fn measure(config: &Config, suite: Suite) -> Result<Measurement, Box<dyn Error>> {
    let mut samples: Vec<Sample> = Vec::with_capacity(config.get_rounds());
    let mut latencies: Vec<u64> = Vec::with_capacity(config.get_ticks());
    let before: AllocationStats = AllocationStats::capture(true);
    let started: Option<Instant> = start_clock();
    let mut workload: Workload = Workload::create(suite, config.get_scale());
    let initialization_ns: Option<u64> = elapsed(started);
    let initialized: AllocationStats = AllocationStats::capture(false);
    let initialization_allocations: Option<AllocationDelta> =
        AllocationStats::delta(before, initialized);
    let warmup_start: AllocationStats = AllocationStats::capture(true);
    let mut frame: u64 = 0;
    for _ in 0..config.get_warmup() {
        black_box(workload.tick(frame));
        frame += 1;
    }
    let warmup_allocations: Option<AllocationDelta> =
        AllocationStats::delta(warmup_start, AllocationStats::capture(false));
    for round in 0..config.get_rounds() {
        latencies.clear();
        let mut work: Work = Work::default();
        let start: AllocationStats = AllocationStats::capture(true);
        let started: Option<Instant> = start_clock();
        for _ in 0..config.get_ticks() {
            let tick_start: Option<Instant> = start_clock();
            let tick_work: Work = workload.tick(frame);
            if let Some(tick_ns) = elapsed(tick_start) {
                latencies.push(tick_ns);
            }
            work.add(tick_work);
            frame += 1;
        }
        let elapsed_ns: Option<u64> = elapsed(started);
        let end: AllocationStats = AllocationStats::capture(false);
        validate_work(suite, config.get_ticks(), work)?;
        latencies.sort_unstable();
        let timings: Timings = Timings::new(
            elapsed_ns,
            percentile(&latencies, 50),
            percentile(&latencies, 95),
            percentile(&latencies, 99),
            elapsed_ns.map(|nanoseconds: u64| {
                config.get_ticks() as f64 * 1_000_000_000.0 / nanoseconds.max(1) as f64
            }),
        );
        samples.push(Sample::new(
            round,
            config.get_ticks(),
            timings,
            AllocationStats::delta(start, end),
            work,
        ));
    }
    drop(workload);
    let after_drop: AllocationStats = AllocationStats::capture(false);
    Ok(Measurement::new(
        suite,
        initialization_ns,
        initialization_allocations,
        warmup_allocations,
        cfg!(feature = "allocation-tracking")
            .then_some(after_drop.get_live() as i64 - before.get_live() as i64),
        samples,
    ))
}

/// Rejects a frame loop that skipped one of its required subsystems.
///
/// # Arguments
///
/// - `Suite` - Expected subsystems.
/// - `usize` - Expected frames per subsystem.
/// - `Work` - Observed work accumulated by the actual frame loop.
///
/// # Returns
///
/// - `Result<(), Box<dyn Error>>` - Success only when every expected invocation ran.
pub(crate) fn validate_work(suite: Suite, ticks: usize, work: Work) -> Result<(), Box<dyn Error>> {
    let expected_core: u64 = if matches!(suite, Suite::Core | Suite::Mixed) {
        ticks as u64
    } else {
        0
    };
    let expected_engine: u64 = if matches!(suite, Suite::Engine | Suite::Mixed) {
        ticks as u64
    } else {
        0
    };
    let expected_chart: u64 = if matches!(suite, Suite::Chart | Suite::Mixed) {
        ticks as u64
    } else {
        0
    };
    if (
        work.get_core_ticks(),
        work.get_engine_ticks(),
        work.get_chart_ticks(),
    ) != (expected_core, expected_engine, expected_chart)
        || work.get_checksum() == 0
    {
        return Err(format!("missing or unexpected subsystem work in {suite:?}: {work:?}").into());
    }
    Ok(())
}

/// Returns a nearest-rank percentile of already sorted frame latencies.
///
/// # Arguments
///
/// - `&[u64]` - Sorted nanosecond measurements.
/// - `usize` - Percentile in the range one through one hundred.
///
/// # Returns
///
/// - `Option<u64>` - Observed latency, absent when no clock samples were taken.
fn percentile(sorted: &[u64], percent: usize) -> Option<u64> {
    let index: usize = (sorted.len() * percent).div_ceil(100).saturating_sub(1);
    sorted.get(index).copied()
}

/// Starts a frame clock only in an uninstrumented timing build.
///
/// # Returns
///
/// - `Option<Instant>` - Clock sample, absent in allocation mode.
fn start_clock() -> Option<Instant> {
    if cfg!(feature = "allocation-tracking") {
        None
    } else {
        Some(Instant::now())
    }
}

/// Reads elapsed time only when timing was enabled for the phase.
///
/// # Arguments
///
/// - `Option<Instant>` - Optional clock sample.
///
/// # Returns
///
/// - `Option<u64>` - Elapsed nanoseconds, absent in allocation mode.
fn elapsed(started: Option<Instant>) -> Option<u64> {
    started.map(|started: Instant| started.elapsed().as_nanos() as u64)
}
