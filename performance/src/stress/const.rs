/// CLI flag to execute the §03/§08 stress-test verification matrix.
pub const BENCH_FLAG_STRESS_MATRIX: &str = "--stress-matrix";

pub(crate) const STRESS_PARTIAL_ROW_AB_NAME: &str = "stress_partial_single_row_update_ab";
pub(crate) const STRESS_VIRTUAL_LIST_TAIL_NAME: &str = "stress_virtual_list_scroll_tail";
pub(crate) const STRESS_WINDOW_RESIZE_NAME: &str = "stress_window_resize_reflow";
pub(crate) const STRESS_TERMINAL_TUI_DIFF_NAME: &str = "stress_terminal_tui_diff";

pub(crate) const BUDGET_144HZ_US: f64 = 6_944.444_444_444_444;
pub(crate) const BUDGET_60HZ_US: f64 = 16_666.666_666_666_668;
pub(crate) const STRESS_PARTIAL_DEFAULT_ITERS: usize = 60;
pub(crate) const STRESS_VLIST_TAIL_DEFAULT_ITERS: usize = 120;
pub(crate) const STRESS_RESIZE_DEFAULT_ITERS: usize = 60;
pub(crate) const STRESS_TERMINAL_DEFAULT_ITERS: usize = 200;
pub(crate) const STRESS_INIT_FAILED: &str = "init_failed";
pub(crate) const STRESS_TUI_HEADER: &str = "euv Terminal TUI Telemetry — 120x40";
pub(crate) const STRESS_TUI_TICK_ZERO: &str = "tick=0";
pub(crate) const STRESS_REPORT_HEADER: &str = "euv Stress-Test Verification Matrix (§03/§08)\n-------------------------------------------------------------------------------\n";
