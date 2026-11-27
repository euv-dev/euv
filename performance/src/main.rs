use std::env;

use euv_performance::{
    BENCH_FLAG_FILTER, BENCH_FLAG_JSON, BenchReport, format_bench_report, report_to_json,
    run_filtered_benchmark_suite,
};

/// Body of the `main` free function.
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut filter: Option<String> = None;
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
        }
        idx += 1;
    }
    let report: BenchReport = run_filtered_benchmark_suite(filter.as_deref());
    if args.iter().any(|arg: &String| arg == BENCH_FLAG_JSON) {
        println!("{}", report_to_json(&report));
    } else {
        print!("{}", format_bench_report(&report));
    }
}
