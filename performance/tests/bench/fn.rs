use super::*;

#[test]
fn benchmark_suite_produces_valid_metrics_and_json() {
    let reactive: BenchMetric = bench_reactive_signals_and_memos(200);
    let selector: BenchMetric = bench_selector_o1(200);
    let layout: BenchMetric = bench_layout_cache(20);
    let sw: BenchMetric = bench_software_rasterizer(2);
    let hw: BenchMetric = bench_hardware_gpu_batching(10);
    let report: BenchReport = BenchReport::new(vec![reactive, selector, layout, sw, hw]);
    assert_eq!(report.get_metrics().len(), 5);
    for m in report.get_metrics().iter() {
        assert!(m.get_iterations() > 0);
        assert!(m.get_ops_per_sec() > 0.0);
    }
    let json: String = report_to_json(&report);
    assert!(json.contains("reactive_signal_memo_diamond"));
    assert!(json.contains("render_software_sdf_800x600"));
    assert!(json.contains("render_hardware_gpu_batching"));
}
