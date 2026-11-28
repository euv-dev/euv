use super::*;

/// Callback invoked when the user clicks a plotted chart datum or slice.
pub type EuvChartHitCallback = Rc<dyn Fn(&EuvChartDatumHit)>;

/// Callback invoked when the user toggles a series visibility chip in the legend `(series_name, now_visible)`.
pub type EuvChartLegendCallback = Rc<dyn Fn(&str, bool)>;

/// Callback invoked when interactive data-zoom changes the active percentage window `(start_pct, end_pct)`.
pub type EuvChartZoomCallback = Rc<dyn Fn(f32, f32)>;

/// Custom series vector draw hook receiving plot bounds, coordinate context, and target `DisplayList`.
pub type EuvChartCustomPainter = Rc<dyn Fn(Rect, &EuvChartCustomDrawContext, &mut DisplayList)>;
