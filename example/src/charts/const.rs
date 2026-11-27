/// Route path for the interactive Apache ECharts 20-series vector showcase page.
pub const ROUTE_CHARTS: &str = "/charts";

/// Page heading for the ECharts Studio showcase.
pub(crate) const CHARTS_PAGE_TITLE: &str = "ECharts Studio";
/// Page subtitle describing all 20 native Apache ECharts series types.
pub(crate) const CHARTS_PAGE_SUBTITLE: &str = "All 20 Apache ECharts series types rendered natively in pure Rust with DataZoom, VisualMap, Tooltip, Legend, Toolbox, and edge-to-edge Fullscreen.";
/// Filter card heading.
pub(crate) const CHARTS_FILTER_TITLE: &str = "Series Category Explorer";
/// Filter card subtitle.
pub(crate) const CHARTS_FILTER_SUBTITLE: &str = "Filter across all 20 series types, toggle legends, hover tooltips, drag DataZoom, or inspect any chart fullscreen.";
/// Initial status message before any chart interaction event fires.
pub(crate) const CHARTS_EVENT_READY: &str =
    "Interactive: hover tooltips, toggle legend chips, drag DataZoom, or tap Fullscreen.";
/// Category button label displaying all 20 series cards.
pub(crate) const CHARTS_CAT_ALL: &str = "All (20)";
/// Category button label for Cartesian Line, Bar, and Scatter charts.
pub(crate) const CHARTS_CAT_CARTESIAN: &str = "Cartesian";
/// Category button label for Financial and Matrix charts.
pub(crate) const CHARTS_CAT_FINANCIAL: &str = "Financial";
/// Category button label for Pie, Radar, and Gauge charts.
pub(crate) const CHARTS_CAT_RADIAL: &str = "Radial";
/// Category button label for Hierarchical and Relational charts.
pub(crate) const CHARTS_CAT_HIERARCHY: &str = "Hierarchy";
/// Category button label for Geo, Flow, Parallel, ThemeRiver, and Custom charts.
pub(crate) const CHARTS_CAT_GEO_FLOW: &str = "Geo & Flow";

/// Card 1 heading (Line series).
pub(crate) const CARD_LINE_TITLE: &str = "1. Line (Smooth Area, Step, Stack, Marks & DataZoom)";
/// Card 1 subtitle.
pub(crate) const CARD_LINE_SUB: &str = "Smooth stacked area + dashed step line with MarkPoint, MarkLine, MarkArea, Inside+Slider DataZoom, Cross AxisPointer, and Toolbox.";
/// Card 2 heading (Bar + Dual-Axis Dataset).
pub(crate) const CARD_BAR_TITLE: &str = "2. Bar + Dual-Axis Line & Dataset Encode";
/// Card 2 subtitle.
pub(crate) const CARD_BAR_SUB: &str = "Tabular Dataset mapped via Encode onto grouped stacked Bars and a secondary right-hand Margin Line axis.";
/// Card 3 heading (Horizontal Bar).
pub(crate) const CARD_HBAR_TITLE: &str = "3. Horizontal Category Bar";
/// Card 3 subtitle.
pub(crate) const CARD_HBAR_SUB: &str =
    "Category Y-axis horizontal bars with background tracks and direct numeric value labels.";
/// Card 4 heading (Scatter & EffectScatter).
pub(crate) const CARD_SCATTER_TITLE: &str = "4. Scatter & EffectScatter (Bubble + VisualMap)";
/// Card 4 subtitle.
pub(crate) const CARD_SCATTER_SUB: &str = "Variable bubble radii paired with pulsing EffectScatter hotspots tinted by a Continuous VisualMap.";
/// Card 5 heading (Candlestick K-Line).
pub(crate) const CARD_CANDLE_TITLE: &str = "5. Candlestick (K-Line OHLC) + MA5 Line";
/// Card 5 subtitle.
pub(crate) const CARD_CANDLE_SUB: &str = "Bull/bear OHLC candlesticks overlaid with a smooth moving-average Line and interactive DataZoom Slider.";
/// Card 6 heading (Boxplot).
pub(crate) const CARD_BOXPLOT_TITLE: &str = "6. Boxplot (5-Number Distribution & Outliers)";
/// Card 6 subtitle.
pub(crate) const CARD_BOXPLOT_SUB: &str = "Min, Q1, median, Q3, and max whiskers with highlighted latency outlier dots across render backends.";
/// Card 7 heading (Cartesian Heatmap).
pub(crate) const CARD_HEATMAP_TITLE: &str = "7. Cartesian 2D Heatmap (Continuous VisualMap)";
/// Card 7 subtitle.
pub(crate) const CARD_HEATMAP_SUB: &str =
    "Hour-by-day compute load matrix shaded smoothly via a Continuous VisualMap gradient bar.";
/// Card 8 heading (Calendar Heatmap).
pub(crate) const CARD_CALENDAR_TITLE: &str = "8. Calendar Heatmap (Piecewise VisualMap)";
/// Card 8 subtitle.
pub(crate) const CARD_CALENDAR_SUB: &str =
    "Week-by-day 2026 commit matrix shaded by discrete Piecewise VisualMap buckets.";
/// Card 9 heading (Pie / Donut / Nightingale Rose).
pub(crate) const CARD_PIE_TITLE: &str = "9. Pie / Donut / Nightingale Rose";
/// Card 9 subtitle.
pub(crate) const CARD_PIE_SUB: &str =
    "Interactive donut rose chart with leader lines, clickable Legend toggles, and slice pop-out.";
/// Card 10 heading (Radar).
pub(crate) const CARD_RADAR_TITLE: &str = "10. Radar (Multi-Series Polygon Web)";
/// Card 10 subtitle.
pub(crate) const CARD_RADAR_SUB: &str =
    "Multi-indicator radar polygon comparing native euv metrics against legacy WebView stacks.";
/// Card 11 heading (Gauge).
pub(crate) const CARD_GAUGE_TITLE: &str = "11. Gauge (Multi-Zone Telemetry Dial)";
/// Card 11 subtitle.
pub(crate) const CARD_GAUGE_SUB: &str = "270-degree dial gauge with multi-segment zone arcs, progress sweep, ticks, and live pointer needle.";
/// Card 12 heading (Treemap).
pub(crate) const CARD_TREEMAP_TITLE: &str = "12. Treemap (Squarified Hierarchy)";
/// Card 12 subtitle.
pub(crate) const CARD_TREEMAP_SUB: &str =
    "Area-proportional squarified rectangles with group headers and contrast-safe labels.";
/// Card 13 heading (Sunburst).
pub(crate) const CARD_SUNBURST_TITLE: &str = "13. Sunburst (Multi-Ring Taxonomy)";
/// Card 13 subtitle.
pub(crate) const CARD_SUNBURST_SUB: &str =
    "Concentric hierarchical wedge rings displaying proportional native and web backend shares.";
/// Card 14 heading (Sankey).
pub(crate) const CARD_SANKEY_TITLE: &str = "14. Sankey (Topological Flow Ribbons)";
/// Card 14 subtitle.
pub(crate) const CARD_SANKEY_SUB: &str =
    "Multi-column topological layout connected by smooth value-weighted cubic Bezier ribbons.";
/// Card 15 heading (Funnel).
pub(crate) const CARD_FUNNEL_TITLE: &str = "15. Funnel (Conversion Pipeline)";
/// Card 15 subtitle.
pub(crate) const CARD_FUNNEL_SUB: &str =
    "Descending stage trapezoids with automatic conversion percentage annotations.";
/// Card 16 heading (Graph Force & Circular).
pub(crate) const CARD_GRAPH_TITLE: &str = "16. Graph (Force & Circular Network)";
/// Card 16 subtitle.
pub(crate) const CARD_GRAPH_SUB: &str = "Switch live between deterministic Fruchterman-Reingold Force and Circular layouts via reactive option_signal.";
/// Card 17 heading (Tree Orthogonal & Radial).
pub(crate) const CARD_TREE_TITLE: &str = "17. Tree (Orthogonal & Radial Hierarchy)";
/// Card 17 subtitle.
pub(crate) const CARD_TREE_SUB: &str = "Switch live between Left-to-Right Orthogonal and Radial node-link tree layouts via reactive option_signal.";
/// Card 18 heading (GeoMap + Lines + EffectScatter).
pub(crate) const CARD_GEO_TITLE: &str = "18. GeoMap + Lines + EffectScatter";
/// Card 18 subtitle.
pub(crate) const CARD_GEO_SUB: &str = "Vector world choropleth colored by VisualMap with curved directional Lines arcs and pulsing hub hotspots.";
/// Card 19 heading (Parallel).
pub(crate) const CARD_PARALLEL_TITLE: &str = "19. Parallel Coordinates";
/// Card 19 subtitle.
pub(crate) const CARD_PARALLEL_SUB: &str =
    "Multi-axis engine telemetry polylines comparing TTFF, FPS, RAM, and Score simultaneously.";
/// Card 20 heading (ThemeRiver).
pub(crate) const CARD_RIVER_TITLE: &str = "20. ThemeRiver (Centered Streamgraph)";
/// Card 20 subtitle.
pub(crate) const CARD_RIVER_SUB: &str =
    "Centered stacked streamgraph ribbons tracking ecosystem adoption across release cycles.";
/// Card 21 heading (PictorialBar & Custom).
pub(crate) const CARD_PIC_CUSTOM_TITLE: &str = "21. PictorialBar & Custom DrawCommand";
/// Card 21 subtitle.
pub(crate) const CARD_PIC_CUSTOM_SUB: &str =
    "Repeated capsule PictorialBar paired with a native Custom series DrawCommand overlay painter.";

/// Interactive Graph force-layout button caption.
pub(crate) const BTN_GRAPH_FORCE: &str = "Force Layout";
/// Interactive Graph circular-layout button caption.
pub(crate) const BTN_GRAPH_CIRCULAR: &str = "Circular Layout";
/// Interactive Tree orthogonal-layout button caption.
pub(crate) const BTN_TREE_ORTHO: &str = "Orthogonal Tree";
/// Interactive Tree radial-layout button caption.
pub(crate) const BTN_TREE_RADIAL: &str = "Radial Tree";

/// Chart title inside Line chart canvas.
pub(crate) const TITLE_LINE_CHART: &str = "Weekly Traffic & Throughput";
/// Chart subtitle inside Line chart canvas.
pub(crate) const SUB_LINE_CHART: &str = "Smooth Area + Step + Marks + DataZoom";
/// Chart title inside Bar Dataset chart canvas.
pub(crate) const TITLE_BAR_CHART: &str = "Revenue & Margin (Dataset)";
/// Chart title inside Horizontal Bar chart canvas.
pub(crate) const TITLE_HBAR_CHART: &str = "Platform Share";
/// Chart title inside Scatter chart canvas.
pub(crate) const TITLE_SCATTER_CHART: &str = "Latency vs Throughput Clusters";
/// Chart title inside Candlestick chart canvas.
pub(crate) const TITLE_CANDLE_CHART: &str = "EUV Index K-Line + MA5";
/// Chart title inside Boxplot chart canvas.
pub(crate) const TITLE_BOXPLOT_CHART: &str = "Frame Latency Distribution";
/// Chart title inside Cartesian Heatmap canvas.
pub(crate) const TITLE_HEATMAP_CHART: &str = "Compute Cluster Load";
/// Chart title inside Calendar Heatmap canvas.
pub(crate) const TITLE_CALENDAR_CHART: &str = "2026 Commit Activity";
/// Chart title inside Pie Rose canvas.
pub(crate) const TITLE_PIE_CHART: &str = "Traffic Sources (Rose)";
/// Chart title inside Radar canvas.
pub(crate) const TITLE_RADAR_CHART: &str = "Native vs Legacy Profile";
/// Chart title inside Gauge canvas.
pub(crate) const TITLE_GAUGE_CHART: &str = "GPU Frame Efficiency";
/// Chart title inside Treemap canvas.
pub(crate) const TITLE_TREEMAP_CHART: &str = "Workspace Footprint (KB)";
/// Chart title inside Sunburst canvas.
pub(crate) const TITLE_SUNBURST_CHART: &str = "Renderer Taxonomy";
/// Chart title inside Sankey canvas.
pub(crate) const TITLE_SANKEY_CHART: &str = "Acquisition Flow";
/// Chart title inside Funnel canvas.
pub(crate) const TITLE_FUNNEL_CHART: &str = "Onboarding Funnel";
/// Chart title inside Graph canvas.
pub(crate) const TITLE_GRAPH_CHART: &str = "Microservice Topology";
/// Chart title inside Tree canvas.
pub(crate) const TITLE_TREE_CHART: &str = "Scene Graph Hierarchy";
/// Chart title inside GeoMap canvas.
pub(crate) const TITLE_GEO_CHART: &str = "Global Edge Traffic";
/// Chart title inside Parallel canvas.
pub(crate) const TITLE_PARALLEL_CHART: &str = "Runtime Benchmark Axes";
/// Chart title inside ThemeRiver canvas.
pub(crate) const TITLE_RIVER_CHART: &str = "Target Adoption Streams";
/// Chart title inside PictorialBar & Custom canvas.
pub(crate) const TITLE_PIC_CUSTOM_CHART: &str = "Capsule Quota & Custom Target";

/// Sample 8-day category labels for Line and Candlestick charts.
pub(crate) const DAYS_EIGHT: [&str; 8] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun", "Next"];
/// Sample 6-month category labels for Dataset Bar chart.
pub(crate) const MONTHS_SIX: [&str; 6] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun"];
/// Sample platform labels for Horizontal Bar chart.
pub(crate) const PLATFORMS_FIVE: [&str; 5] = ["WebGPU", "macOS", "iOS", "Android", "Linux"];
/// Sample render backend names for Boxplot chart.
pub(crate) const BACKENDS_THREE: [&str; 3] = ["Software", "Hybrid", "Hardware"];
/// Sample 4-hour bucket labels for Cartesian Heatmap X axis.
pub(crate) const HOURS_SIX: [&str; 6] = ["00h", "04h", "08h", "12h", "16h", "20h"];
/// Sample weekday labels for Cartesian Heatmap Y axis.
pub(crate) const WEEKDAYS_FOUR: [&str; 4] = ["Mon", "Tue", "Wed", "Thu"];
/// Sample tier labels for PictorialBar & Custom chart.
pub(crate) const TIERS_FOUR: [&str; 4] = ["Alpha", "Beta", "Gamma", "Delta"];

/// Series and dimension label strings (`>= 4` chars centralized for rust-standards §1.3c).
pub(crate) const SER_DIRECT: &str = "Direct";
/// Series name for stacked organic traffic.
pub(crate) const SER_ORGANIC: &str = "Organic";
/// Series name for dashed step search line.
pub(crate) const SER_SEARCH: &str = "Step Search";
/// Stack group key for Line series.
pub(crate) const STACK_TRAFFIC: &str = "traffic";
/// MarkPoint peak label.
pub(crate) const MARK_PEAK: &str = "Peak";
/// MarkLine average label.
pub(crate) const MARK_AVG: &str = "Average";
/// MarkArea highlight label.
pub(crate) const MARK_PROMO: &str = "Launch";
/// Dataset dimension name for month column.
pub(crate) const DIM_MONTH: &str = "Month";
/// Dataset dimension name for revenue column.
pub(crate) const DIM_REVENUE: &str = "Revenue";
/// Dataset dimension name for profit column.
pub(crate) const DIM_PROFIT: &str = "Profit";
/// Dataset dimension name for margin column.
pub(crate) const DIM_MARGIN: &str = "Margin";
/// Stack key for dataset bar series.
pub(crate) const STACK_REV: &str = "rev_stack";
/// Series name for horizontal bar share.
pub(crate) const SER_SHARE: &str = "Share";
/// Series name for scatter clusters.
pub(crate) const SER_CLUSTERS: &str = "Clusters";
/// Series name for effect scatter hotspots.
pub(crate) const SER_HOTSPOTS: &str = "Hotspots";
/// Scatter point annotation label.
pub(crate) const PT_EDGE_A: &str = "Edge-A";
/// Scatter point annotation label.
pub(crate) const PT_EDGE_B: &str = "Edge-B";
/// Scatter point annotation label.
pub(crate) const PT_CORE_C: &str = "Core-C";
/// Scatter point annotation label.
pub(crate) const PT_BURST_D: &str = "Burst";
/// Candlestick series name.
pub(crate) const SER_KLINE: &str = "KLine";
/// Moving-average line series name.
pub(crate) const SER_MA5: &str = "MA5 Line";
/// Boxplot series name.
pub(crate) const SER_LATENCY: &str = "Latency";
/// Heatmap series name.
pub(crate) const SER_LOAD: &str = "Load";
/// Calendar heatmap series name.
pub(crate) const SER_COMMITS: &str = "Commits";
/// Piecewise VisualMap label for low band.
pub(crate) const VM_QUIET: &str = "Quiet";
/// Piecewise VisualMap label for medium band.
pub(crate) const VM_ACTIVE: &str = "Active";
/// Piecewise VisualMap label for peak band.
pub(crate) const VM_PEAK: &str = "Peak";
/// Pie series name.
pub(crate) const SER_TRAFFIC: &str = "Traffic";
/// Pie slice label for Search.
pub(crate) const SLICE_SEARCH: &str = "Search";
/// Pie slice label for Referral.
pub(crate) const SLICE_REFERRAL: &str = "Referral";
/// Pie slice label for Social.
pub(crate) const SLICE_SOCIAL: &str = "Social";
/// Pie slice label for Email.
pub(crate) const SLICE_EMAIL: &str = "Email";
/// Radar indicator label for frame rate.
pub(crate) const IND_FPS: &str = "Frame FPS";
/// Radar indicator label for memory.
pub(crate) const IND_MEM: &str = "Memory";
/// Radar indicator label for startup.
pub(crate) const IND_STARTUP: &str = "Startup";
/// Radar indicator label for binary size.
pub(crate) const IND_BINARY: &str = "Binary";
/// Radar indicator label for battery efficiency.
pub(crate) const IND_BATTERY: &str = "Battery";
/// Radar series name.
pub(crate) const SER_PROFILE: &str = "Profile";
/// Radar item name for native euv.
pub(crate) const ITEM_EUV_NATIVE: &str = "euv Native";
/// Radar item name for legacy WebView.
pub(crate) const ITEM_WEBVIEW: &str = "WebView";
/// Gauge series name.
pub(crate) const SER_BUDGET: &str = "Frame Budget";
/// Gauge inner title.
pub(crate) const GAUGE_EFFICIENCY: &str = "Efficiency";
/// Treemap series name.
pub(crate) const SER_FOOTPRINT: &str = "Footprint";
/// Hierarchy node label for Core.
pub(crate) const H_CORE: &str = "Core";
/// Hierarchy node label for Layout.
pub(crate) const H_LAYOUT: &str = "Layout";
/// Hierarchy node label for Render.
pub(crate) const H_RENDER: &str = "Render";
/// Hierarchy node label for Signals.
pub(crate) const H_SIGNALS: &str = "Signals";
/// Hierarchy node label for Charts.
pub(crate) const H_CHARTS: &str = "Charts";
/// Hierarchy node label for Markdown.
pub(crate) const H_MARKDOWN: &str = "Markdown";
/// Hierarchy node label for Router.
pub(crate) const H_ROUTER: &str = "Router";
/// Hierarchy node label for UI Suite.
pub(crate) const H_UI_SUITE: &str = "UI Suite";
/// Sunburst series name.
pub(crate) const SER_TAXONOMY: &str = "Taxonomy";
/// Sunburst branch label for Native.
pub(crate) const H_NATIVE: &str = "Native";
/// Sunburst leaf label for Metal.
pub(crate) const H_METAL: &str = "Metal";
/// Sunburst leaf label for Vulkan.
pub(crate) const H_VULKAN: &str = "Vulkan";
/// Sunburst leaf label for DX12.
pub(crate) const H_DX12: &str = "Direct3D12";
/// Sunburst branch label for Browser.
pub(crate) const H_BROWSER: &str = "Browser";
/// Sunburst leaf label for Canvas2D.
pub(crate) const H_CANVAS2D: &str = "Canvas2D";
/// Sankey series name.
pub(crate) const SER_FLOW: &str = "Traffic Flow";
/// Sankey node label for Paid Ads.
pub(crate) const SK_PAID_ADS: &str = "Paid Ads";
/// Sankey node label for Landing.
pub(crate) const SK_LANDING: &str = "Landing";
/// Sankey node label for Trial.
pub(crate) const SK_TRIAL: &str = "Trial";
/// Sankey node label for Signup.
pub(crate) const SK_SIGNUP: &str = "Signup";
/// Sankey node label for Enterprise.
pub(crate) const SK_ENTERPRISE: &str = "Enterprise";
/// Funnel series name.
pub(crate) const SER_CONVERSION: &str = "Conversion";
/// Funnel stage label for Visit.
pub(crate) const FN_VISIT: &str = "Visit";
/// Funnel stage label for Explore.
pub(crate) const FN_EXPLORE: &str = "Explore";
/// Funnel stage label for Evaluate.
pub(crate) const FN_EVAL: &str = "Evaluate";
/// Funnel stage label for Adopt.
pub(crate) const FN_ADOPT: &str = "Adopt";
/// Graph series name.
pub(crate) const SER_CLUSTER: &str = "Cluster";
/// Graph node label for Gateway.
pub(crate) const GR_GATEWAY: &str = "Gateway";
/// Graph node label for Auth.
pub(crate) const GR_AUTH: &str = "Auth";
/// Graph node label for Cache.
pub(crate) const GR_CACHE: &str = "Cache";
/// Graph node label for Worker.
pub(crate) const GR_WORKER: &str = "Worker";
/// Graph node label for Storage.
pub(crate) const GR_STORAGE: &str = "Storage";
/// Graph node label for Metrics.
pub(crate) const GR_METRICS: &str = "Metrics";
/// Tree series name.
pub(crate) const SER_SCENE_TREE: &str = "SceneTree";
/// Tree root label.
pub(crate) const TR_ROOT: &str = "AppRoot";
/// Tree branch label for Shell.
pub(crate) const TR_SHELL: &str = "AppShell";
/// Tree leaf label for Sidebar.
pub(crate) const TR_SIDEBAR: &str = "Sidebar";
/// Tree leaf label for Outlet.
pub(crate) const TR_OUTLET: &str = "Outlet";
/// Tree branch label for Portal.
pub(crate) const TR_PORTAL: &str = "Portals";
/// Tree leaf label for Fullscreen.
pub(crate) const TR_FULLSCREEN: &str = "Fullscreen";
/// Tree leaf label for Tooltip.
pub(crate) const TR_TOOLTIP: &str = "Tooltip";
/// GeoMap series name.
pub(crate) const SER_USERS: &str = "Active Users";
/// GeoMap region name for China.
pub(crate) const GEO_CHINA: &str = "China";
/// GeoMap region name for United States.
pub(crate) const GEO_USA: &str = "United States";
/// GeoMap region name for Europe.
pub(crate) const GEO_EUROPE: &str = "Europe";
/// GeoMap region name for Brazil.
pub(crate) const GEO_BRAZIL: &str = "Brazil";
/// GeoMap region name for Australia.
pub(crate) const GEO_AUSTRALIA: &str = "Australia";
/// GeoMap region name for India.
pub(crate) const GEO_INDIA: &str = "India";
/// Geo Lines series name.
pub(crate) const SER_EDGE_ROUTES: &str = "Edge Routes";
/// Geo EffectScatter series name.
pub(crate) const SER_EDGE_HUBS: &str = "Edge Hubs";
/// Parallel series name.
pub(crate) const SER_BENCHMARKS: &str = "Benchmarks";
/// Parallel axis label for TTFF.
pub(crate) const AX_TTFF: &str = "TTFF ms";
/// Parallel axis label for RAM.
pub(crate) const AX_RAM: &str = "RAM MB";
/// Parallel axis label for Score.
pub(crate) const AX_SCORE: &str = "Score";
/// Parallel row label for Flutter.
pub(crate) const ITEM_FLUTTER: &str = "Flutter";
/// ThemeRiver series name.
pub(crate) const SER_STREAMS: &str = "Streams";
/// ThemeRiver stream name for Rust.
pub(crate) const STREAM_RUST: &str = "Rust";
/// ThemeRiver stream name for WASM.
pub(crate) const STREAM_WASM: &str = "WASM";
/// ThemeRiver stream name for Mobile.
pub(crate) const STREAM_MOBILE: &str = "Mobile";
/// PictorialBar series name.
pub(crate) const SER_CAPSULES: &str = "Capsules";
/// Custom series name.
pub(crate) const SER_CUSTOM_TARGET: &str = "Custom Target";
/// Default chart canvas height in logical pixels.
pub(crate) const CHART_CARD_HEIGHT: f32 = 290.0;
/// Tall chart canvas height for complex geo/hierarchy diagrams.
pub(crate) const CHART_CARD_TALL_HEIGHT: f32 = 315.0;
