/// Default high-contrast Apache ECharts categorical palette for Light mode (RGB hex).
pub const ECHART_PALETTE_LIGHT: [u32; 10] = [
    0x5470C6, 0x91CC75, 0xFAC858, 0xEE6666, 0x73C0DE, 0x3BA272, 0xFC8452, 0x9A60B4, 0xEA7CCC,
    0x2EC7C9,
];

/// Default luminous Apache ECharts categorical palette for Dark mode (RGB hex).
pub const ECHART_PALETTE_DARK: [u32; 10] = [
    0x6C8CFF, 0x8CE071, 0xFFD166, 0xFF7B7B, 0x67D4F5, 0x4AD493, 0xFF9A68, 0xC084FC, 0xF472B6,
    0x38E2E0,
];

/// Default continuous visual-map gradient stops for Light mode (RGB hex).
pub const ECHART_VISUAL_RAMP_LIGHT: [u32; 4] = [0xEFF6FF, 0x60A5FA, 0xF59E0B, 0xDC2626];

/// Default continuous visual-map gradient stops for Dark mode (RGB hex).
pub const ECHART_VISUAL_RAMP_DARK: [u32; 4] = [0x1E293B, 0x38BDF8, 0xFBBF24, 0xF87171];

/// Default bullish candlestick color (RGB hex).
pub const ECHART_CANDLE_BULL_HEX: u32 = 0x22C55E;

/// Default bearish candlestick color (RGB hex).
pub const ECHART_CANDLE_BEAR_HEX: u32 = 0xEF4444;

/// Default logical canvas width in pixels when unspecified.
pub const ECHART_DEFAULT_WIDTH: f32 = 640.0;

/// Default logical canvas height in pixels when unspecified.
pub const ECHART_DEFAULT_HEIGHT: f32 = 340.0;

/// Default grid left margin in logical pixels.
pub(crate) const ECHART_GRID_DEFAULT_LEFT: f32 = 48.0;

/// Default grid right margin in logical pixels.
pub(crate) const ECHART_GRID_DEFAULT_RIGHT: f32 = 22.0;

/// Default grid top margin in logical pixels.
pub(crate) const ECHART_GRID_DEFAULT_TOP: f32 = 28.0;

/// Default grid bottom margin in logical pixels.
pub(crate) const ECHART_GRID_DEFAULT_BOTTOM: f32 = 34.0;

/// Default axis split line interval count.
pub(crate) const ECHART_DEFAULT_SPLIT_NUMBER: usize = 5;

/// Default pie chart outer radius ratio relative to half plot minimum side.
pub(crate) const ECHART_PIE_DEFAULT_OUTER_RATIO: f32 = 0.72;

/// Default pie chart start angle in degrees (`90.0` = 12 o'clock).
pub(crate) const ECHART_PIE_DEFAULT_START_ANGLE: f32 = 90.0;

/// Default clicked pie slice pop-out distance in logical pixels.
pub(crate) const ECHART_PIE_DEFAULT_POPOUT: f32 = 10.0;

/// Default gauge sweep start angle in degrees.
pub(crate) const ECHART_GAUGE_DEFAULT_START_ANGLE: f32 = 225.0;

/// Default gauge sweep end angle in degrees.
pub(crate) const ECHART_GAUGE_DEFAULT_END_ANGLE: f32 = -45.0;

/// Default gauge segment low-band color (RGB hex).
pub(crate) const ECHART_GAUGE_LOW_HEX: u32 = 0x22C55E;

/// Default gauge segment mid-band color (RGB hex).
pub(crate) const ECHART_GAUGE_MID_HEX: u32 = 0xF59E0B;

/// Default gauge segment high-band color (RGB hex).
pub(crate) const ECHART_GAUGE_HIGH_HEX: u32 = 0xEF4444;

/// Default calendar grid week count across one year.
pub(crate) const ECHART_CALENDAR_DEFAULT_WEEKS: usize = 26;

/// Default force-directed graph iteration count.
pub(crate) const ECHART_GRAPH_DEFAULT_ITERS: usize = 48;

/// Default minimum data zoom window span percentage.
pub(crate) const ECHART_ZOOM_DEFAULT_MIN_SPAN: f32 = 8.0;
