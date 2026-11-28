/// Minimum candle body height in logical pixels so doji candles stay crisp.
pub(crate) const CARTESIAN_CANDLE_MIN_BODY: f32 = 1.5;

/// Default tick mark stub length in logical pixels.
pub(crate) const CARTESIAN_TICK_LEN: f32 = 4.5;

/// Minimum horizontal spacing between rendered X category labels in logical pixels.
pub(crate) const CARTESIAN_MIN_X_LABEL_STEP: f32 = 42.0;

/// Area fill vertical gradient slice count.
pub(crate) const CARTESIAN_AREA_GRADIENT_BANDS: usize = 4;

/// Default mark-point fallback prefix for average label.
pub(crate) const CARTESIAN_MARK_AVG: &str = "Avg";
