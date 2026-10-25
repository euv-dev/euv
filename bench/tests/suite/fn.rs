use super::*;

#[test]
#[cfg(feature = "allocation-tracking")]
fn reactive_snapshots_allocate_only_their_final_output_strings() {
    let mut workload: Workload = Workload::create(Suite::Core, 1);
    workload.tick(0);
    let before: AllocationStats = AllocationStats::capture(true);
    let work: Work = workload.tick(1);
    let delta: AllocationDelta =
        AllocationStats::delta(before, AllocationStats::capture(false)).unwrap();
    assert_eq!(work.get_core_ticks(), 1);
    assert_eq!(delta.get_calls(), 2);
    assert_eq!(delta.get_reallocations(), 0);
    assert_eq!(delta.get_live_delta_bytes(), 0);
}

#[test]
fn default_suite_includes_every_subsystem_and_a_real_mixed_frame() {
    let config: Config = Config::default();
    assert_eq!(
        config.get_suites().as_slice(),
        &[Suite::Core, Suite::Engine, Suite::Chart, Suite::Mixed]
    );
    let mut workload: Workload = Workload::create(Suite::Mixed, 1);
    let work: Work = workload.tick(1);
    assert_eq!(
        (
            work.get_core_ticks(),
            work.get_engine_ticks(),
            work.get_chart_ticks()
        ),
        (1, 1, 1)
    );
    assert!(work.get_checksum() > 0);
    assert!(
        workload
            .try_get_core()
            .as_ref()
            .unwrap()
            .get_notifications()
            .get()
            > 0
    );
    let engine: &EngineLoad = workload.try_get_engine().as_ref().unwrap();
    assert_eq!(engine.get_emitter().alive_count(), ENGINE_PARTICLE_COUNT);
    assert_eq!(engine.get_draw().len(), ENGINE_PARTICLE_COUNT);
    let chart: &ChartLoad = workload.try_get_chart().as_ref().unwrap();
    assert!(!chart.get_list().is_empty());
    assert_eq!(chart.get_point_count(), CHART_POINT_COUNT * 3);
}

#[test]
fn incomplete_mixed_work_is_rejected_instead_of_reporting_partial_success() {
    let work: Work = Work::new(1, 0, 1, 99);
    assert!(runner::validate_work(Suite::Mixed, 1, work).is_err());
}

#[test]
fn individual_suites_do_not_silently_execute_or_skip_other_subsystems() {
    for suite in [Suite::Core, Suite::Engine, Suite::Chart] {
        let mut workload: Workload = Workload::create(suite, 1);
        let work: Work = workload.tick(1);
        assert!(runner::validate_work(suite, 1, work).is_ok());
    }
}

#[test]
fn invalid_options_and_empty_measurements_are_rejected() {
    for arguments in [
        vec!["--suite", "echarts"],
        vec!["--unknown", "1"],
        vec!["--ticks", "0"],
        vec!["--rounds", "0"],
        vec!["--scale", "0"],
        vec!["--scale", "17"],
        vec!["--output"],
    ] {
        assert!(Config::parse(arguments.into_iter().map(str::to_string)).is_err());
    }
}

#[test]
fn every_default_measurement_contains_all_requested_rounds_and_real_work() {
    let mut config: Config = Config::default();
    config.set_ticks(2).set_rounds(2).set_warmup(1);
    for suite in config.get_suites().iter().copied() {
        let measurement: runner::Measurement = runner::measure(&config, suite).unwrap();
        assert_eq!(measurement.get_samples().len(), config.get_rounds());
        for sample in measurement.get_samples() {
            assert!(runner::validate_work(suite, config.get_ticks(), sample.get_work()).is_ok());
        }
    }
}

#[test]
#[cfg(feature = "allocation-tracking")]
fn allocator_accounts_for_alloc_realloc_and_dealloc_requested_sizes() {
    let before: AllocationStats = AllocationStats::capture(true);
    let mut values: Vec<u64> = Vec::with_capacity(16);
    values.resize(16, 1);
    black_box(&values);
    values.reserve_exact(16);
    black_box(&values);
    drop(values);
    let after: AllocationStats = AllocationStats::capture(false);
    let delta: AllocationDelta = AllocationStats::delta(before, after).unwrap();
    assert_eq!(delta.get_calls(), 2);
    assert_eq!(delta.get_reallocations(), 1);
    assert_eq!(delta.get_live_delta_bytes(), 0);
    assert_eq!(delta.get_allocated_bytes(), (16 + 32) * 8);
    assert_eq!(delta.get_peak_extra_bytes(), 32 * 8);
}

#[test]
#[cfg(feature = "allocation-tracking")]
fn warmed_particle_count_burst_and_render_do_not_clone_the_particle_vector() {
    let mut workload: Workload = Workload::create(Suite::Engine, 1);
    let engine: &mut EngineLoad = workload.get_mut_engine().as_mut().unwrap();
    let mut draw: DrawList = take(engine.get_mut_draw());
    engine.get_emitter().render(&mut draw);
    draw.clear();
    let before: AllocationStats = AllocationStats::capture(true);
    assert_eq!(engine.get_emitter().alive_count(), ENGINE_PARTICLE_COUNT);
    engine.get_mut_emitter().burst(0);
    engine.get_emitter().render(&mut draw);
    let delta: AllocationDelta =
        AllocationStats::delta(before, AllocationStats::capture(false)).unwrap();
    assert_eq!(delta.get_calls(), 0);
    assert_eq!(draw.len(), ENGINE_PARTICLE_COUNT);
    engine.set_draw(draw);
}

#[test]
fn timing_and_allocation_modes_do_not_claim_each_others_metrics() {
    let mut config: Config = Config::default();
    config.set_ticks(1).set_rounds(1).set_warmup(0);
    let measurement: runner::Measurement = runner::measure(&config, Suite::Core).unwrap();
    let sample: &runner::Sample = &measurement.get_samples()[0];
    let allocation_mode: bool = cfg!(feature = "allocation-tracking");
    assert_eq!(sample.try_get_allocations().is_some(), allocation_mode);
    assert_eq!(
        measurement.try_get_initialization_allocations().is_some(),
        allocation_mode
    );
    assert_eq!(
        measurement.try_get_warmup_allocations().is_some(),
        allocation_mode
    );
    assert_eq!(
        sample.get_timings().try_get_elapsed_ns().is_none(),
        allocation_mode
    );
    assert_eq!(
        sample.get_timings().try_get_p50_ns().is_none(),
        allocation_mode
    );
    assert_eq!(
        sample.get_timings().try_get_p95_ns().is_none(),
        allocation_mode
    );
    assert_eq!(
        sample.get_timings().try_get_p99_ns().is_none(),
        allocation_mode
    );
    assert_eq!(
        sample.get_timings().try_get_ticks_per_second().is_none(),
        allocation_mode
    );
    assert_eq!(
        measurement.try_get_initialization_ns().is_none(),
        allocation_mode
    );
}

#[test]
fn flattened_records_preserve_the_existing_report_schema() {
    let environment: runner::Environment = runner::Environment::new(
        String::from("source"),
        String::from("host"),
        String::from("arch"),
        Some(1),
        String::from("compiler"),
        1,
        String::from("native CPU"),
    );
    let timings: runner::Timings = runner::Timings::new(
        Some(100),
        Some(100),
        Some(100),
        Some(100),
        Some(1_000_000_000.0 / 100.0),
    );
    let sample: runner::Sample = runner::Sample::new(0, 1, timings, None, Work::new(1, 1, 1, 1));
    let measurement: runner::Measurement =
        runner::Measurement::new(Suite::Mixed, Some(1), None, None, None, vec![sample]);
    let report: runner::Report = runner::Report::new(
        runner::Header::new(2, "timing", environment),
        Config::default(),
        vec![measurement],
    );
    let json: String = to_string_pretty(&report).unwrap();
    let value: Value = from_str(&json).unwrap();
    assert_eq!(value.get("schema_version").and_then(Value::as_u64), Some(2));
    assert_eq!(value.get("mode").and_then(Value::as_str), Some("timing"));
    assert_eq!(
        value.get("source_root").and_then(Value::as_str),
        Some("source")
    );
    assert!(value.get("header").is_none());
    let sample: &Value = &value["measurements"][0]["samples"][0];
    assert_eq!(sample.get("elapsed_ns").and_then(Value::as_u64), Some(100));
    assert!(sample.get("timings").is_none());
    assert!(sample.get("allocations").unwrap().is_null());
}
