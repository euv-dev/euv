//! The host-side benchmark of the chart crate.
//!
//! The benchmark measures the two costs that compete with ECharts': the
//! layout of a chart and the building of its display list. Both are pure
//! computation, so they can be measured without a browser — which is
//! exactly what makes them comparable.
//!
//! Run it with:
//!
//! ```text
//! cargo bench -p euv-chart
//! ```

mod r#const;

use std::time::{Duration, Instant};

use euv_chart::{
    ChartLayout, ChartOption, ChartRenderer, DataValue, DisplayList, GridOption, Position,
    SeriesOption, TextMeasurer,
};

use r#const::SERIES_NAME;

/// The number of iterations each measurement runs.
const ITERATIONS: usize = 20;

/// The dataset sizes the benchmark runs over.
const SIZES: &[usize] = &[1_000, 10_000, 100_000, 1_000_000];

/// Builds a line chart over the given number of points.
///
/// # Arguments
///
/// - `usize` - A non-negative integer (`usize`).
///
/// # Returns
///
/// - `ChartOption` - A `ChartOption` value.
fn line_option(count: usize) -> ChartOption {
    let mut option: ChartOption = ChartOption::new();
    let mut series: SeriesOption = SeriesOption::line();
    series.name = SERIES_NAME.to_string();
    for index in 0..count {
        let value: f64 = ((index % 1000) as f64 / 1000.0 * 100.0).sin() * 50.0 + 50.0;
        series.add_data(DataValue::Number(value));
    }
    option.series.push(series);
    option.grid = Some(GridOption {
        left: Some(Position::Offset(40.0)),
        right: Some(Position::Offset(20.0)),
        top: Some(Position::Offset(20.0)),
        bottom: Some(Position::Offset(30.0)),
        ..Default::default()
    });
    option
}

/// Builds a bar chart over the given number of categories.
///
/// # Arguments
///
/// - `usize` - A non-negative integer (`usize`).
///
/// # Returns
///
/// - `ChartOption` - A `ChartOption` value.
fn bar_option(count: usize) -> ChartOption {
    let mut option: ChartOption = ChartOption::new();
    let mut series: SeriesOption = SeriesOption::bar();
    series.name = SERIES_NAME.to_string();
    for index in 0..count {
        let value: f64 = ((index % 997) as f64 / 997.0 * 100.0).cos() * 50.0 + 50.0;
        series.add_data(DataValue::List(vec![
            DataValue::Text(format!("category-{}", index)),
            DataValue::Number(value),
        ]));
    }
    option.series.push(series);
    option
}

/// Runs a measurement and returns the fastest run.
///
/// # Arguments
///
/// - `impl FnMut()` - A `impl FnMut()` parameter.
///
/// # Returns
///
/// - `Duration` - A `Duration` value.
fn measure(mut action: impl FnMut()) -> Duration {
    let mut best: Duration = Duration::from_secs(u64::MAX);
    for _ in 0..ITERATIONS {
        let start: Instant = Instant::now();
        action();
        let elapsed: Duration = start.elapsed();
        if elapsed < best {
            best = elapsed;
        }
    }
    best
}

/// Prints one row of the report.
///
/// # Arguments
///
/// - `&str` - Shared reference to a `str`.
/// - `usize` - A non-negative integer (`usize`).
/// - `Duration` - A `Duration` parameter.
fn report(name: &str, count: usize, layout: Duration, display: Duration, items: usize) {
    let layout_ms: f64 = layout.as_secs_f64() * 1000.0;
    let display_ms: f64 = display.as_secs_f64() * 1000.0;
    let total_ms: f64 = layout_ms + display_ms;
    println!(
        "{:<10} {:>9} points  {:>9.3} ms layout  {:>9.3} ms display  {:>9.3} ms total  {:>7} items",
        name, count, layout_ms, display_ms, total_ms, items
    );
}

/// Runs the benchmark.
fn main() {
    let measurer: TextMeasurer = TextMeasurer::approximate();
    let mut renderer: ChartRenderer = ChartRenderer::new();
    let mut list: DisplayList = DisplayList::new();

    println!(
        "euv-chart benchmark — the fastest of {} runs per measurement",
        ITERATIONS
    );
    println!();

    for count in SIZES {
        let option: ChartOption = line_option(*count);
        let layout: Duration = measure(|| {
            let _: ChartLayout = ChartLayout::compute(&option, 1200.0, 800.0, &measurer);
        });
        let computed: ChartLayout = ChartLayout::compute(&option, 1200.0, 800.0, &measurer);
        let display: Duration = measure(|| {
            renderer.build(&mut list, &option, &computed);
        });
        report("line", *count, layout, display, list.len());
    }

    println!();

    for count in SIZES {
        let option: ChartOption = bar_option(*count);
        let layout: Duration = measure(|| {
            let _: ChartLayout = ChartLayout::compute(&option, 1200.0, 800.0, &measurer);
        });
        let computed: ChartLayout = ChartLayout::compute(&option, 1200.0, 800.0, &measurer);
        let display: Duration = measure(|| {
            renderer.build(&mut list, &option, &computed);
        });
        report("bar", *count, layout, display, list.len());
    }

    println!();
    println!("The 'items' column is the number of display-list entries: the");
    println!("GPU backends turn a run of them into a handful of draw calls,");
    println!("and the Canvas2D backend replays them one by one.");
}
