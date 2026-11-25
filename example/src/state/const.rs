/// Route path for the overview dashboard page.
pub const ROUTE_OVERVIEW: &str = "/";
/// Route path for the fine-grained reactivity demo page.
pub const ROUTE_REACTIVE: &str = "/reactive";
/// Route path for the multi-mode rendering and vector canvas page.
pub const ROUTE_RENDER: &str = "/render";
/// Route path for the native window and platform inspector page.
pub const ROUTE_WINDOW: &str = "/window";
/// Route path for the interactive forms showcase page.
pub const ROUTE_FORMS: &str = "/forms";
/// Route path for the 17-scheme live theme gallery page.
pub const ROUTE_THEMES: &str = "/themes";
/// Route path for the virtualized list and table showcase page.
pub const ROUTE_DATA: &str = "/data";
/// Route path for the embedded game engine & HUD showcase page.
pub const ROUTE_GAME: &str = "/game";
/// Route path for the AAA 3D open-world PBR & ray-tracing engine showcase page.
pub const ROUTE_GAME_3D: &str = "/game3d";
/// Route path for the full cross-platform media studio (Image, SVG, GIF, LIVE, Video, Audio, PDF, HTML).
pub const ROUTE_MEDIA: &str = "/media";
/// Route path for the complete shadcn/ui component gallery and edge-case regression lab.
pub const ROUTE_LAB: &str = "/lab";
/// Route path for the modal, toast, and alert overlay showcase page.
pub const ROUTE_OVERLAYS: &str = "/overlays";
/// Route path for the component-level and page-level dynamic OTA updates showcase page.
pub const ROUTE_DYNAMIC: &str = "/dynamic";
/// Route path for the embedded page-level dynamic update preview slot.
pub const ROUTE_DYNAMIC_PANEL: &str = "/dynamic/panel";
/// Runtime-injected dynamic route path created on demand without recompiling.
pub const ROUTE_PROMO_LIVE: &str = "/promo-live";
/// Component slot identifier for the live-updatable promotional banner.
pub const SLOT_PROMO_BANNER: &str = "promo_banner";
/// Component slot identifier for the independent live-updatable metrics widget.
pub const SLOT_METRICS_WIDGET: &str = "metrics_widget";
/// Scoped partial slot identifier for the live downloaded `.wasm` native widget.
pub const SLOT_WASM_WIDGET: &str = "wasm_live_widget";
/// Display title for the downloaded partial WASM v1 widget fixture.
pub const TITLE_WASM_PARTIAL_V1: &str = "Downloaded Partial WASM Widget v1";
/// Shared verification key used to sign showcase dynamic `.wasm` bundles.
pub const WASM_SHOWCASE_SIGNING_KEY: &str = "euv-prod-signing-key-2026";
/// CDN URL for healthy partial `.wasm` module v1.
pub const URL_WASM_PARTIAL_V1: &str = "euv-cdn://bundles/partial-widget-v1.wasm";
/// CDN URL for healthy partial `.wasm` module v2.
pub const URL_WASM_PARTIAL_V2: &str = "euv-cdn://bundles/partial-widget-v2.wasm";
/// CDN URL for healthy global full-app `.wasm` bundle v1.
pub const URL_WASM_GLOBAL_V1: &str = "euv-cdn://bundles/global-app-v1.wasm";
/// CDN URL for healthy global full-app `.wasm` bundle v2.
pub const URL_WASM_GLOBAL_V2: &str = "euv-cdn://bundles/global-app-v2.wasm";
/// CDN URL simulating first-attempt timeout followed by automatic retry success.
pub const URL_WASM_TIMEOUT_RETRY: &str = "euv-cdn://faults/timeout-retry.wasm";
/// CDN URL simulating persistent HTTP 500 server error.
pub const URL_WASM_HTTP_500: &str = "euv-cdn://faults/http-500.wasm";
/// CDN URL serving corrupted non-WASM magic header bytes.
pub const URL_WASM_CORRUPTED: &str = "euv-cdn://faults/corrupted.wasm";
/// CDN URL serving checksum-tampered `.wasm` bytes.
pub const URL_WASM_TAMPERED: &str = "euv-cdn://faults/tampered.wasm";
/// CDN URL serving a `.wasm` with mismatched ABI version.
pub const URL_WASM_BAD_ABI: &str = "euv-cdn://faults/bad-abi.wasm";
/// CDN URL serving a `.wasm` missing `euv_render` export.
pub const URL_WASM_MISSING_EXPORT: &str = "euv-cdn://faults/missing-export.wasm";
/// CDN URL serving an infinite-loop `.wasm` terminated by fuel metering.
pub const URL_WASM_FUEL_BOMB: &str = "euv-cdn://faults/fuel-bomb.wasm";
/// CDN URL serving a trapping `unreachable` `.wasm`.
pub const URL_WASM_TRAP: &str = "euv-cdn://faults/trap.wasm";
/// CDN URL serving an oversized binary exceeding sandbox limit.
pub const URL_WASM_OVERSIZED: &str = "euv-cdn://faults/oversized.wasm";
/// Application brand title displayed in the shell chrome.
pub const SHOWCASE_BRAND: &str = "euv Studio";

pub(crate) const NAV_SEC_WORKSPACE: &str = "WORKSPACE";
pub(crate) const NAV_SEC_DESIGN: &str = "DESIGN SYSTEM";
pub(crate) const NAV_SEC_ENGINE: &str = "ENGINE & MEDIA";
pub(crate) const NAV_SEC_RUNTIME: &str = "RUNTIME & OTA";
pub(crate) const NAV_LABEL_OVERVIEW: &str = "Overview";
pub(crate) const NAV_LABEL_REACTIVE: &str = "Reactivity";
pub(crate) const NAV_LABEL_RENDER: &str = "Rendering";
pub(crate) const NAV_LABEL_WINDOW: &str = "Window & OS";
pub(crate) const NAV_LABEL_FORMS: &str = "Forms";
pub(crate) const NAV_LABEL_THEMES: &str = "Themes (17)";
pub(crate) const NAV_LABEL_DATA: &str = "Scroll List";
pub(crate) const NAV_LABEL_GAME: &str = "Game Engine";
pub(crate) const NAV_LABEL_GAME_3D: &str = "3D Engine (RT)";
pub(crate) const NAV_LABEL_MEDIA: &str = "Media Studio";
pub(crate) const NAV_LABEL_LAB: &str = "UI & Edge Lab";
pub(crate) const NAV_LABEL_OVERLAYS: &str = "Overlays";
pub(crate) const NAV_LABEL_DYNAMIC: &str = "Dynamic WASM";
pub(crate) const DEFAULT_USERNAME: &str = "euv-developer";
pub(crate) const GAME_HUD_CONTROLS_HINT: &str =
    "WASD/Arrows Move | Space/Click Fire | E Dash | R Wave";
pub(crate) const SAMPLE_OBJ_MONUMENT: &str = "o CyberObelisk\nv -1.4 0.0 -1.4\nv 1.4 0.0 -1.4\nv 1.4 0.0 1.4\nv -1.4 0.0 1.4\nv 0.0 7.5 0.0\nf 1 2 5\nf 2 3 5\nf 3 4 5\nf 4 1 5\nf 1 4 3 2\n";
pub(crate) const SAMPLE_GLTF_TOWER: &str = r#"{"asset":{"version":"2.0","generator":"euv-gltf-tower"},"materials":[{"pbrMetallicRoughness":{"baseColorFactor":[0.12,0.86,0.98,1.0],"metallicFactor":0.85,"roughnessFactor":0.12}}],"meshes":[{"name":"PrismBeacon","primitives":[{"extras":{"positions":[-1.2,0.0,-1.2,1.2,0.0,-1.2,1.2,0.0,1.2,-1.2,0.0,1.2,-0.8,5.8,-0.8,0.8,5.8,-0.8,0.8,5.8,0.8,-0.8,5.8,0.8],"indices":[0,1,5,0,5,4,1,2,6,1,6,5,2,3,7,2,7,6,3,0,4,3,4,7,4,5,6,4,6,7]}}]}]}"#;
