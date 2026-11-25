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
/// Application brand title displayed in the shell chrome.
pub const SHOWCASE_BRAND: &str = "euv Native Showcase";

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
pub(crate) const NAV_LABEL_OVERLAYS: &str = "Overlays";
pub(crate) const NAV_LABEL_DYNAMIC: &str = "Dynamic OTA";
pub(crate) const DEFAULT_USERNAME: &str = "euv-developer";
pub(crate) const GAME_HUD_CONTROLS_HINT: &str =
    "WASD/Arrows Move | Space/Click Fire | E Dash | R Wave";
pub(crate) const SAMPLE_OBJ_MONUMENT: &str = "o CyberObelisk\nv -1.4 0.0 -1.4\nv 1.4 0.0 -1.4\nv 1.4 0.0 1.4\nv -1.4 0.0 1.4\nv 0.0 7.5 0.0\nf 1 2 5\nf 2 3 5\nf 3 4 5\nf 4 1 5\nf 1 4 3 2\n";
pub(crate) const SAMPLE_GLTF_TOWER: &str = r#"{"asset":{"version":"2.0","generator":"euv-gltf-tower"},"materials":[{"pbrMetallicRoughness":{"baseColorFactor":[0.12,0.86,0.98,1.0],"metallicFactor":0.85,"roughnessFactor":0.12}}],"meshes":[{"name":"PrismBeacon","primitives":[{"extras":{"positions":[-1.2,0.0,-1.2,1.2,0.0,-1.2,1.2,0.0,1.2,-1.2,0.0,1.2,-0.8,5.8,-0.8,0.8,5.8,-0.8,0.8,5.8,0.8,-0.8,5.8,0.8],"indices":[0,1,5,0,5,4,1,2,6,1,6,5,2,3,7,2,7,6,3,0,4,3,4,7,4,5,6,4,6,7]}}]}]}"#;
