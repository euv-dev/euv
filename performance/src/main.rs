use std::env;

use euv_performance::{
    BENCH_FLAG_FILTER, BENCH_FLAG_JSON, BENCH_FLAG_STRESS_MATRIX, BenchReport, StressMatrixReport,
    format_bench_report, format_stress_matrix_report, report_to_json, run_filtered_benchmark_suite,
    run_filtered_stress_matrix, stress_matrix_to_json,
};

/// Body of the `main` free function.
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut filter: Option<String> = None;
    let mut stress_matrix: bool = false;
    let mut emit_json: bool = false;
    let mut idx: usize = 1;
    while idx < args.len() {
        let arg: &str = args[idx].as_str();
        if arg == BENCH_FLAG_FILTER {
            if let Some(next_val) = args.get(idx + 1) {
                filter = Some(next_val.clone());
                idx += 1;
            }
        } else if let Some(rest) = arg.strip_prefix(BENCH_FLAG_FILTER)
            && let Some(val) = rest.strip_prefix('=')
            && !val.is_empty()
        {
            filter = Some(val.to_string());
        } else if arg == BENCH_FLAG_STRESS_MATRIX {
            stress_matrix = true;
        } else if arg == BENCH_FLAG_JSON {
            emit_json = true;
        }
        idx += 1;
    }
    if stress_matrix {
        let report: StressMatrixReport = run_filtered_stress_matrix(filter.as_deref());
        if emit_json {
            println!("{}", stress_matrix_to_json(&report));
        } else {
            print!("{}", format_stress_matrix_report(&report));
        }
    } else {
        let report: BenchReport = run_filtered_benchmark_suite(filter.as_deref());
        if emit_json {
            println!("{}", report_to_json(&report));
        } else {
            print!("{}", format_bench_report(&report));
        }
    }
}
