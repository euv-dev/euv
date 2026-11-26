use super::*;

#[test]
fn benchmark_suite_produces_valid_metrics_and_json() {
    let reactive: BenchMetric = bench_reactive_signals_and_memos(200);
    let selector: BenchMetric = bench_selector_o1(200);
    let layout: BenchMetric = bench_layout_cache(20);
    let sw: BenchMetric = bench_software_rasterizer(2);
    let hw: BenchMetric = bench_hardware_gpu_batching(10);
    let long_list: BenchMetric = bench_stress_long_list_100k(4);
    let complex_layout: BenchMetric = bench_stress_complex_layout_2000(4);
    let bulk_reactive: BenchMetric = bench_stress_bulk_reactive_1000(3);
    let wasm_ota: BenchMetric = bench_stress_wasm_dynamic_hotswap(3);
    let hidpi_4x: BenchMetric = bench_stress_hidpi_4x_1440x3168(2);
    let input_hit: BenchMetric = bench_stress_scroll_physics_and_hittest(20);
    let report: BenchReport = BenchReport::new(vec![
        reactive,
        selector,
        layout,
        sw,
        hw,
        long_list,
        complex_layout,
        bulk_reactive,
        wasm_ota,
        hidpi_4x,
        input_hit,
    ]);
    assert_eq!(report.get_metrics().len(), 11);
    for m in report.get_metrics().iter() {
        assert!(m.get_iterations() > 0);
        assert!(m.get_ops_per_sec() > 0.0);
    }
    let json: String = report_to_json(&report);
    assert!(json.contains("reactive_signal_memo_diamond"));
    assert!(json.contains("render_software_sdf_800x600"));
    assert!(json.contains("render_hardware_gpu_batching"));
    assert!(json.contains("stress_long_list_virtual_100k"));
    assert!(json.contains("stress_complex_layout_2000_nodes"));
    assert!(json.contains("stress_bulk_reactive_1000_nodes"));
    assert!(json.contains("stress_wasm_dynamic_load_hotswap"));
    assert!(json.contains("stress_hidpi_4x_1440x3168_scroll"));
    assert!(json.contains("stress_240hz_scroll_and_hittest"));
}
