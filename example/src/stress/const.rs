/// Route path for the interactive UI stress & performance lab page.
pub const ROUTE_STRESS: &str = "/stress";
/// Scenario identifier for the 100,000-row virtualized list scroll benchmark.
pub const STRESS_ID_VLIST_100K: &str = "vlist_100k";
/// Scenario identifier for the 2,000-node Flex/Grid complex layout benchmark.
pub const STRESS_ID_LAYOUT_2000: &str = "layout_2000";
/// Scenario identifier for the 1,000-component bulk reactive signal/memo benchmark.
pub const STRESS_ID_REACTIVE_1000: &str = "reactive_1000";
/// Scenario identifier for the dynamic `.wasm` verification, hot-swap, and rollback benchmark.
pub const STRESS_ID_WASM_HOTSWAP: &str = "wasm_hotswap";
/// Scenario identifier for the 30-frame shadowed drawer & modal animation trace benchmark.
pub const STRESS_ID_DRAWER_ANIM: &str = "drawer_anim";
/// Scenario identifier for the 240Hz multi-touch pan and 1,000-node hit-test benchmark.
pub const STRESS_ID_TOUCH_HITTEST: &str = "touch_hittest";
/// Scenario identifier for the HiDPI 3x/4x Gaussian shadow and SDF rasterizer benchmark.
pub const STRESS_ID_HIDPI_SHADOW: &str = "hidpi_shadow";
/// Scenario identifier for the Unicode + CJK glyph rasterization and RGBA8 atlas packing benchmark.
pub const STRESS_ID_CJK_ATLAS: &str = "cjk_atlas";
/// Scenario identifier for the 2D physics (256 bodies) + 3D BVH ray-tracing benchmark.
pub const STRESS_ID_GAME_2D_3D: &str = "game_2d_3d";
/// Scenario identifier for the declarative `view!` macro tree mount/unmount benchmark.
pub const STRESS_ID_MACRO_MOUNT: &str = "macro_mount";

pub(crate) const NAV_LABEL_STRESS: &str = "Stress Lab";
pub(crate) const STRESS_PAGE_TITLE: &str = "Interactive UI Stress & Perf Lab";
pub(crate) const STRESS_PAGE_SUBTITLE: &str = "Click any scenario below to execute real live workloads on this device and inspect measured latency, P95, and throughput.";
pub(crate) const STRESS_CTRL_TITLE: &str = "Stress Suite Controls";
pub(crate) const STRESS_CTRL_SUBTITLE: &str =
    "All scenarios run isolated scenes inside this process with zero fake numbers or mock timings.";
pub(crate) const STRESS_BTN_RUN_ALL: &str = "Run All 10 Stress Tests";
pub(crate) const STRESS_BTN_RUN_CORE: &str = "Run Core UI Suite (1-5)";
pub(crate) const STRESS_BTN_RESET: &str = "Reset Telemetry";
pub(crate) const STRESS_KPI_COMPLETED_LABEL: &str = "Completed Cases";
pub(crate) const STRESS_KPI_BUDGET_LABEL: &str = "Budget Compliance";
pub(crate) const STRESS_KPI_LATEST_LABEL: &str = "Latest Measured";
pub(crate) const STRESS_KPI_IDLE_VAL: &str = "Idle";
pub(crate) const STRESS_KPI_IDLE_CAP: &str = "Tap any scenario button to benchmark";
pub(crate) const STRESS_KPI_BUDGET_CAP: &str = "Real per-frame latency vs target budget";
pub(crate) const STRESS_SUMMARY_READY: &str =
    "Ready: click any scenario button or Run All 10 Stress Tests to collect real device metrics.";
pub(crate) const STRESS_STATUS_IDLE: &str = "IDLE";
pub(crate) const STRESS_STATUS_PASS: &str = "PASS";
pub(crate) const STRESS_STATUS_WARN: &str = "OVER";
pub(crate) const STRESS_UNIT_NA: &str = "Pending";
pub(crate) const STRESS_CAT_LIST: &str = "LIST";
pub(crate) const STRESS_CAT_LAYOUT: &str = "LAYOUT";
pub(crate) const STRESS_CAT_REACTIVE: &str = "REACTIVE";
pub(crate) const STRESS_CAT_WASM: &str = "WASM OTA";
pub(crate) const STRESS_CAT_ANIM: &str = "ANIMATION";
pub(crate) const STRESS_CAT_INPUT: &str = "INPUT";
pub(crate) const STRESS_CAT_RASTER: &str = "RASTER";
pub(crate) const STRESS_CAT_TEXT: &str = "GLYPH";
pub(crate) const STRESS_CAT_ENGINE: &str = "2D / 3D";
pub(crate) const STRESS_CAT_MACRO: &str = "MACRO";
pub(crate) const STRESS_TITLE_VLIST: &str = "1. 100k Virtual List Scroll";
pub(crate) const STRESS_DESC_VLIST: &str =
    "Scrolls a 100,000-row EuvVirtualList across 24 software-rasterized frames (+144px/step).";
pub(crate) const STRESS_BTN_VLIST: &str = "Run 100k List";
pub(crate) const STRESS_TITLE_LAYOUT: &str = "2. 2,000-Node Complex Layout";
pub(crate) const STRESS_DESC_LAYOUT: &str =
    "Builds 50x4 Flex/Grid sections (~2,000 nodes) and profiles 40 dirty-subtree relayout passes.";
pub(crate) const STRESS_BTN_LAYOUT: &str = "Run 2k Layout";
pub(crate) const STRESS_TITLE_REACTIVE: &str = "3. 1,000-Component Bulk Signals";
pub(crate) const STRESS_DESC_REACTIVE: &str =
    "Batches 1,000 live Signal + Memo reactive text nodes across 16 glitch-free flush frames.";
pub(crate) const STRESS_BTN_REACTIVE: &str = "Run 1k Signals";
pub(crate) const STRESS_TITLE_WASM: &str = "4. Dynamic WASM Load & Hot-Swap";
pub(crate) const STRESS_DESC_WASM: &str =
    "Verifies, executes, hot-swaps (v1 -> v2), and rolls back real .wasm binaries 12 cycles.";
pub(crate) const STRESS_BTN_WASM: &str = "Run WASM OTA";
pub(crate) const STRESS_TITLE_ANIM: &str = "5. Drawer & Scrim Animation (30f)";
pub(crate) const STRESS_DESC_ANIM: &str = "Animates a shadowed portal drawer over a multi-card base scene with backdrop cache + LUT scrim.";
pub(crate) const STRESS_BTN_ANIM: &str = "Run Drawer Anim";
pub(crate) const STRESS_TITLE_TOUCH: &str = "6. 240Hz Multi-Touch & Hit-Test";
pub(crate) const STRESS_DESC_TOUCH: &str = "Injects 120 high-rate touch + tap-jitter events over 1,000 interactive nodes and checks 0px drift.";
pub(crate) const STRESS_BTN_TOUCH: &str = "Run 240Hz Touch";
pub(crate) const STRESS_TITLE_HIDPI: &str = "7. HiDPI 3x Shadow & SDF Raster";
pub(crate) const STRESS_DESC_HIDPI: &str =
    "Renders 12 shadowed rounded-rect cards at 1080x1920 @ 3.0x with analytical SDF edges.";
pub(crate) const STRESS_BTN_HIDPI: &str = "Run 3x Shadow";
pub(crate) const STRESS_TITLE_CJK: &str = "8. Unicode & CJK Glyph Atlas";
pub(crate) const STRESS_DESC_CJK: &str =
    "Rasterizes Latin + CJK glyphs across sizes/weights and shelf-packs GPU RGBA8 atlases (16x).";
pub(crate) const STRESS_BTN_CJK: &str = "Run CJK Atlas";
pub(crate) const STRESS_TITLE_GAME: &str = "9. 2D Physics + 3D BVH Ray-Trace";
pub(crate) const STRESS_DESC_GAME: &str =
    "Steps 256 2D rigid bodies + 1,024 particles (60 ticks) plus 4 frames of 3D BVH ray tracing.";
pub(crate) const STRESS_BTN_GAME: &str = "Run 2D+3D Engine";
pub(crate) const STRESS_TITLE_MACRO: &str = "10. Declarative view! Macro Mount";
pub(crate) const STRESS_DESC_MACRO: &str =
    "Mounts, lays out, and disposes 160 declarative view! macro component trees across 8 cycles.";
pub(crate) const STRESS_BTN_MACRO: &str = "Run Macro Mount";
pub(crate) const STRESS_ROW_PREFIX: &str = "Stress Row";
pub(crate) const STRESS_CELL_PREFIX: &str = "Cell";
pub(crate) const STRESS_CARD_PREFIX: &str = "Card";
pub(crate) const STRESS_CARD_SUB: &str = "Shadowed HiDPI SDF surface";
pub(crate) const STRESS_BADGE_TXT: &str = "Lossless";
pub(crate) const STRESS_DRAWER_TITLE: &str = "Mobile Drawer";
pub(crate) const STRESS_DRAWER_ITEM: &str = "Nav Item";
pub(crate) const STRESS_WASM_SLOT: &str = "stress_wasm_slot";
pub(crate) const STRESS_WASM_URL_V1: &str = "euv-stress://partial-v1.wasm";
pub(crate) const STRESS_WASM_URL_V2: &str = "euv-stress://partial-v2.wasm";
pub(crate) const STRESS_WASM_ETAG_V1: &str = "etag-stress-v1";
pub(crate) const STRESS_WASM_ETAG_V2: &str = "etag-stress-v2";
pub(crate) const STRESS_WASM_TITLE_V1: &str = "Stress WASM v1";
pub(crate) const STRESS_WASM_TITLE_V2: &str = "Stress WASM v2";
pub(crate) const STRESS_WASM_KEY: &str = "euv-stress-sign-key";
pub(crate) const STRESS_WASM_RB_REASON: &str = "stress rollback";
pub(crate) const STRESS_WASM_FALLBACK: &str = "WASM Slot Ready";
pub(crate) const STRESS_SCENARIOS_TITLE: &str = "Live Stress Benchmark Scenarios";
pub(crate) const STRESS_SCENARIOS_SUB: &str = "Each button triggers an individual benchmark immediately and records avg, P95, max, and throughput.";
