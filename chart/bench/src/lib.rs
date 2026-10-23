//! euv-chart-bench
//!
//! The harness that compares euv-chart against Apache ECharts.
//!
//! The two libraries are measured the same way: a chart is created, an
//! option with a given number of points is applied, and the time it
//! takes to apply it is measured over a number of iterations. The
//! numbers are handed back to JavaScript, which puts them next to the
//! numbers ECharts produces on the same page.
//!
//! Two costs are measured, because they are the two costs an
//! application pays:
//!
//! * the *steady state*: the option is applied to a chart that is
//!   already there, over and over. This is the cost of one frame.
//! * the *cold start*: a chart is created from scratch for every
//!   measurement, so that nothing is carried over from the run before.
//!   This is the cost of opening a chart.

use euv_chart::{Chart, ChartOption, DataValue, GridOption, Position, SeriesOption};
use lombok_macros::Data;
use wasm_bindgen::prelude::*;

/// The result of one measurement.
#[derive(Clone, Copy, Data, Debug, Default)]
#[wasm_bindgen]
pub struct BenchResult {
    /// The time it took to create the chart, in milliseconds.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    setup_ms: f64,
    /// The average time it took to apply the option, in milliseconds.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    frame_ms: f64,
    /// The fastest run, in milliseconds.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    best_ms: f64,
    /// The number of points the chart was built over.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    count: u32,
    /// The number of display-list entries the chart produced.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    items: u32,
}

#[wasm_bindgen]
/// Inherent implementation of [`BenchResult`].
impl BenchResult {
    /// Returns the time it took to create the chart, in milliseconds.
    ///
    /// # Returns
    ///
    /// - `f64` - A 64-bit float.
    #[wasm_bindgen(getter)]
    pub fn setup_ms(&self) -> f64 {
        self.get_setup_ms()
    }

    /// Returns the average time it took to apply the option, in
    /// milliseconds.
    ///
    /// # Returns
    ///
    /// - `f64` - A 64-bit float.
    #[wasm_bindgen(getter)]
    pub fn frame_ms(&self) -> f64 {
        self.get_frame_ms()
    }

    /// Returns the fastest run, in milliseconds.
    ///
    /// # Returns
    ///
    /// - `f64` - A 64-bit float.
    #[wasm_bindgen(getter)]
    pub fn best_ms(&self) -> f64 {
        self.get_best_ms()
    }

    /// Returns the number of points the chart was built over.
    ///
    /// # Returns
    ///
    /// - `u32` - A 32-bit unsigned integer.
    #[wasm_bindgen(getter)]
    pub fn count(&self) -> u32 {
        self.get_count()
    }

    /// Returns the number of display-list entries the chart produced.
    ///
    /// # Returns
    ///
    /// - `u32` - A 32-bit unsigned integer.
    #[wasm_bindgen(getter)]
    pub fn items(&self) -> u32 {
        self.get_items()
    }
}

/// The kind of chart the benchmark builds.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Kind {
    /// A line chart.
    Line,
    /// A bar chart.
    Bar,
}

/// Builds the option the benchmark applies.
///
/// The option is the shape ECharts gets on the page: a category axis
/// with one category per point, and a value axis for the values.
///
/// # Arguments
///
/// - `Kind` - A `Kind` parameter.
/// - `usize` - A non-negative integer (`usize`).
///
/// # Returns
///
/// - `ChartOption` - A `ChartOption` value.
fn build_option(kind: Kind, count: usize) -> ChartOption {
    let mut option: ChartOption = ChartOption::new();
    let mut series: SeriesOption = match kind {
        Kind::Line => SeriesOption::line(),
        Kind::Bar => SeriesOption::bar(),
    };
    series.name = "throughput".to_string();
    for index in 0..count {
        let value: f64 =
            ((index % 1000) as f64 / 1000.0 * std::f64::consts::TAU).sin() * 50.0 + 50.0;
        match kind {
            Kind::Line => {
                series.add_data(DataValue::Number(value));
            }
            Kind::Bar => {
                series.add_data(DataValue::List(vec![
                    DataValue::Text(format!("category-{}", index)),
                    DataValue::Number(value),
                ]));
            }
        }
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

/// Returns the current time in milliseconds, with a fraction.
///
/// The clock of the host is used rather than the date: a frame over a
/// thousand points takes a fraction of a millisecond, and a clock that
/// only counts whole milliseconds would report every one of them as
/// zero.
///
/// # Returns
///
/// - `f64` - A 64-bit float.
fn now_ms() -> f64 {
    web_sys::window()
        .and_then(|window: web_sys::Window| window.performance())
        .map(|performance: web_sys::Performance| performance.now())
        .unwrap_or_else(js_sys::Date::now)
}

/// Measures the chart over the given number of points.
///
/// The measurement has three parts: the creation of the chart, the
/// application of the option once (which pays for the first layout),
/// and the average of the following applications (which is what a chart
/// costs per frame when the data changes).
///
/// The copies of the option are made *before* the clock starts. A
/// copy is a cost of the harness — JavaScript hands ECharts the very
/// same option every time — and charging it to the chart would make
/// the comparison meaningless.
///
/// # Arguments
///
/// - `web_sys::HtmlCanvasElement` - The `web_sys::HtmlCanvasElement` value.
/// - `&str` - The `&str` value.
/// - `u32` - The `u32` value.
///
/// # Returns
///
/// - `Result<BenchResult, JsValue>` - The `Result<BenchResult, JsValue>` value.
#[wasm_bindgen]
pub async fn bench(
    canvas: web_sys::HtmlCanvasElement,
    kind: &str,
    count: u32,
    iterations: u32,
) -> Result<BenchResult, JsValue> {
    let kind: Kind = match kind {
        "bar" => Kind::Bar,
        _ => Kind::Line,
    };
    let count: usize = count as usize;
    let iterations: usize = iterations.max(1) as usize;

    let setup_start: f64 = now_ms();
    let mut chart: Chart = Chart::create(canvas)
        .await
        .ok_or_else(|| JsValue::from_str("the chart could not be created"))?;
    let setup_ms: f64 = now_ms() - setup_start;

    let option: ChartOption = build_option(kind, count);

    // The first application pays for the backend's first frame.
    chart.set_option(option.clone());

    // The copies are made up front, so that the measurement below
    // times the chart and not the allocator.
    let mut prepared: Vec<ChartOption> = Vec::with_capacity(iterations);
    for _ in 0..iterations {
        prepared.push(option.clone());
    }

    let mut total: f64 = 0.0_f64;
    let mut best: f64 = f64::INFINITY;
    for item in prepared {
        let start: f64 = now_ms();
        chart.set_option(item);
        let elapsed: f64 = now_ms() - start;
        total += elapsed;
        if elapsed < best {
            best = elapsed;
        }
    }

    let items: u32 = chart.display_list().len() as u32;
    // The chart is released before the measurement is handed back: a
    // chart over a million points holds on to a lot of memory, and the
    // cold-start measurement creates one after the other.
    chart.dispose();

    Ok(BenchResult {
        setup_ms,
        frame_ms: total / iterations as f64,
        best_ms: best,
        count: count as u32,
        items,
    })
}

/// Returns the kind of the backend the host would pick.
///
/// The page shows it next to the ECharts numbers, because the backend is
/// the reason the two differ.
///
/// # Returns
///
/// - `String` - A `String` value.
#[wasm_bindgen]
pub fn backend_kind() -> String {
    euv_chart::BackendFactory::detect(euv_chart::BackendKind::WebGpu)
        .name()
        .to_string()
}
