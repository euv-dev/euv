use std::env;

use euv_performance::{
    BENCH_FLAG_JSON, BenchReport, format_bench_report, report_to_json, run_full_benchmark_suite,
};

/// Body of the `main` free function.
fn main() {
    let args: Vec<String> = env::args().collect();
    let report: BenchReport = run_full_benchmark_suite();
    if args.iter().any(|arg: &String| arg == BENCH_FLAG_JSON) {
        println!("{}", report_to_json(&report));
    } else {
        print!("{}", format_bench_report(&report));
    }
}
