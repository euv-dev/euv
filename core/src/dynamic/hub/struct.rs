use super::*;

/// Serializable style overrides attached to a `DynamicNodeSpec`.
#[derive(Clone, Copy, Data, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct DynamicStyleSpec {
    /// Optional explicit pixel width (when `Some`, overrides default auto width).
    #[get(skip)]
    pub(crate) width: Option<f32>,
    /// Optional explicit pixel height (when `Some`, overrides default auto height).
    #[get(skip)]
    pub(crate) height: Option<f32>,
    /// Whether this node should stretch to `100%` of its parent width.
    #[get(type(copy))]
    pub(crate) full_width: bool,
    /// Optional uniform inner padding in logical pixels.
    #[get(skip)]
    pub(crate) padding: Option<f32>,
    /// Optional inter-child gap in logical pixels.
    #[get(skip)]
    pub(crate) gap: Option<f32>,
    /// Optional uniform corner radius in logical pixels.
    #[get(skip)]
    pub(crate) radius: Option<f32>,
    /// Optional text font size in logical pixels.
    #[get(skip)]
    pub(crate) font_size: Option<f32>,
    /// Whether typography renders in bold weight.
    #[get(type(copy))]
    pub(crate) bold: bool,
    /// Optional 24-bit RGB hex background fill color (e.g. `0x2563EB`).
    #[get(skip)]
    pub(crate) background_hex: Option<u32>,
    /// Optional 24-bit RGB hex foreground text color (e.g. `0xFFFFFF`).
    #[get(skip)]
    pub(crate) color_hex: Option<u32>,
    /// Optional border stroke width in logical pixels.
    #[get(skip)]
    pub(crate) border_width: Option<f32>,
    /// Optional 24-bit RGB hex border stroke color.
    #[get(skip)]
    pub(crate) border_hex: Option<u32>,
    /// Grid column count when `kind == DynamicNodeKind::Grid`.
    #[get(type(copy))]
    pub(crate) columns: u16,
    /// Flex grow factor inside parent flex containers.
    #[get(type(copy))]
    pub(crate) flex_grow: f32,
}

/// Serializable declarative UI node tree compiled directly into native `SceneTree` nodes at runtime.
#[derive(Clone, Data, Debug, Default, Deserialize, New, PartialEq, Serialize)]
#[serde(default)]
pub struct DynamicNodeSpec {
    /// Semantic native primitive kind.
    #[get(type(copy))]
    pub(crate) kind: DynamicNodeKind,
    /// Optional UTF-8 text content for `Text`, `Button`, `Badge`, or `Card` title.
    #[new(skip)]
    pub(crate) text: String,
    /// Optional action identifier emitted as `DynamicActionEvent` when clicked.
    #[new(skip)]
    #[get(skip)]
    pub(crate) action: Option<String>,
    /// Optional opaque string payload carried by `DynamicActionEvent`.
    #[new(skip)]
    pub(crate) payload: String,
    /// Optional declarative visual and layout style overrides.
    #[new(skip)]
    #[get(type(copy))]
    pub(crate) style: DynamicStyleSpec,
    /// Ordered nested child specifications.
    #[new(skip)]
    pub(crate) children: Vec<DynamicNodeSpec>,
}

/// Versioned, checksum-verified component-level dynamic update payload.
#[derive(Clone, Data, Debug, Deserialize, New, PartialEq, Serialize)]
pub struct DynamicComponentUpdate {
    /// Unique slot identifier targeted by this component update (e.g. `"promo_banner"`).
    pub(crate) id: String,
    /// Monotonic component revision number (`>= 1`).
    #[get(type(copy))]
    pub(crate) version: u64,
    /// Declarative UI subtree replacing the slot contents.
    pub(crate) spec: DynamicNodeSpec,
    /// Optional expected deterministic FNV-1a checksum (`0` skips checksum verification).
    #[new(skip)]
    #[serde(default)]
    #[get(type(copy))]
    pub(crate) checksum: u64,
    /// Whether this update may apply a version lower than or equal to the currently active version.
    #[new(skip)]
    #[serde(default)]
    #[get(type(copy))]
    pub(crate) allow_downgrade: bool,
}

/// Versioned, checksum-verified page-level dynamic update payload (overrides or injects a route).
#[derive(Clone, Data, Debug, Deserialize, New, PartialEq, Serialize)]
pub struct DynamicPageUpdate {
    /// Normalized route path targeted by this page update (e.g. `"/promo-live"`).
    pub(crate) route: String,
    /// Human-readable page title exposed for navigation menus and headers.
    pub(crate) title: String,
    /// Monotonic page revision number (`>= 1`).
    #[get(type(copy))]
    pub(crate) version: u64,
    /// Declarative UI page tree rendered when this route is active.
    pub(crate) spec: DynamicNodeSpec,
    /// Optional expected deterministic FNV-1a checksum (`0` skips checksum verification).
    #[new(skip)]
    #[serde(default)]
    #[get(type(copy))]
    pub(crate) checksum: u64,
    /// Hint indicating whether external page state should be preserved across hot-swaps.
    #[new(skip)]
    #[serde(default)]
    #[get(type(copy))]
    pub(crate) preserve_state: bool,
    /// Whether this update may apply a version lower than or equal to the currently active version.
    #[new(skip)]
    #[serde(default)]
    #[get(type(copy))]
    pub(crate) allow_downgrade: bool,
}

/// Atomic multi-target update manifest delivering component-level and page-level updates together.
#[derive(Clone, Data, Debug, Default, Deserialize, New, PartialEq, Serialize)]
pub struct DynamicUpdateBundle {
    /// Unique release bundle identifier (e.g. `"ota-2026.11.25-01"`).
    pub(crate) bundle_id: String,
    /// Monotonic bundle release version.
    #[get(type(copy))]
    pub(crate) version: u64,
    /// Ordered component-level updates applied atomically.
    #[new(skip)]
    #[serde(default)]
    pub(crate) components: Vec<DynamicComponentUpdate>,
    /// Ordered page-level updates applied atomically.
    #[new(skip)]
    #[serde(default)]
    pub(crate) pages: Vec<DynamicPageUpdate>,
}

/// Interactive event emitted when a user activates a button inside a dynamically updated component or page.
#[derive(Clone, Data, Debug, Eq, Hash, New, PartialEq)]
pub struct DynamicActionEvent {
    /// Originating component slot ID or page route path.
    pub(crate) source_id: String,
    /// Action identifier declared on the dynamic control.
    pub(crate) action: String,
    /// Optional string payload carried with the action.
    pub(crate) payload: String,
}

/// Cumulative runtime telemetry counters exposed reactively by `DynamicUpdateHub`.
#[derive(Clone, Copy, Data, Debug, Default, Eq, Hash, New, PartialEq)]
pub struct DynamicTelemetry {
    /// Total successful component-level dynamic updates applied.
    #[get(type(copy))]
    pub(crate) components_applied: u64,
    /// Total successful page-level dynamic updates applied.
    #[get(type(copy))]
    pub(crate) pages_applied: u64,
    /// Total component or page rollbacks executed.
    #[get(type(copy))]
    pub(crate) rollbacks: u64,
    /// Total rejected updates (checksum mismatch, version regression, invalid JSON).
    #[get(type(copy))]
    pub(crate) rejected: u64,
    /// Total interactive actions dispatched from dynamically rendered controls.
    #[get(type(copy))]
    pub(crate) actions_dispatched: u64,
}

/// Single revision entry stored on a component or page history stack for instant rollback.
#[derive(Clone, CustomDebug)]
pub(crate) struct DynamicSlotRecord {
    /// Revision version number.
    pub(crate) version: u64,
    /// Optional page title (empty for component slots).
    pub(crate) title: String,
    /// Verified payload checksum (`0` for native Rust closure swaps).
    pub(crate) checksum: u64,
    /// Renderable payload (declarative spec or native Rust builder).
    pub(crate) payload: DynamicPayload,
}

/// Internal interior-mutable registry storing component/page revision stacks and surgical signals.
#[derive(CustomDebug)]
pub(crate) struct DynamicHubState {
    /// Revision stacks keyed by component slot ID.
    pub(crate) components: HashMap<String, Vec<DynamicSlotRecord>>,
    /// Dedicated per-component reactive revision signals for O(1) surgical re-renders.
    pub(crate) component_signals: HashMap<String, Signal<u64>>,
    /// Revision stacks keyed by normalized page route path.
    pub(crate) pages: HashMap<String, Vec<DynamicSlotRecord>>,
    /// Dedicated per-page reactive revision signals for O(1) surgical page re-renders.
    pub(crate) page_signals: HashMap<String, Signal<u64>>,
    /// Reactive counter bumped whenever the set of registered dynamic routes changes.
    pub(crate) routes_signal: Signal<u64>,
    /// Reactive telemetry snapshot signal.
    pub(crate) telemetry_signal: Signal<DynamicTelemetry>,
    /// Reactive signal carrying the most recent `DynamicActionEvent`.
    pub(crate) last_action: Signal<Option<DynamicActionEvent>>,
    /// Registered imperative action callbacks.
    #[debug(skip)]
    pub(crate) action_handlers: Vec<DynamicActionCallback>,
}

/// Shared cross-platform hub orchestrating surgical component-level and page-level dynamic updates.
#[derive(Clone, CustomDebug)]
pub struct DynamicUpdateHub {
    /// Interior-mutable registry state shared across `App`, `BuildContext`, and live UI slots.
    pub(crate) inner: Rc<RefCell<DynamicHubState>>,
}

/// Declarative `Mountable` slot supporting surgical component-level dynamic updates and rollbacks.
#[derive(CustomDebug, Data, New)]
pub struct DynamicComponentSlot {
    /// Unique component slot identifier targeted by `DynamicUpdateHub::apply_component`.
    pub(crate) id: String,
    /// Compiled fallback UI builder rendered when no dynamic update is active for `id`.
    #[debug(skip)]
    #[get(skip)]
    pub(crate) fallback: DynamicBuilderFn,
    /// Optional explicit `DynamicUpdateHub` override (defaults to the ambient `BuildContext` hub).
    #[new(skip)]
    #[get(skip)]
    pub(crate) hub: Option<DynamicUpdateHub>,
    /// Optional container `Style` applied to the slot host wrapper node.
    #[new(skip)]
    #[get(type(copy))]
    pub(crate) style: Style,
}

/// Declarative `Mountable` slot supporting surgical page-level dynamic updates for a single route.
#[derive(CustomDebug, Data, New)]
pub struct DynamicPageSlot {
    /// Route path targeted by `DynamicUpdateHub::apply_page` (e.g. `"/dynamic/live"`).
    pub(crate) route: String,
    /// Compiled fallback page builder rendered when no dynamic page update is active.
    #[debug(skip)]
    #[get(skip)]
    pub(crate) fallback: DynamicBuilderFn,
    /// Optional explicit `DynamicUpdateHub` override (defaults to the ambient `BuildContext` hub).
    #[new(skip)]
    #[get(skip)]
    pub(crate) hub: Option<DynamicUpdateHub>,
    /// Optional container `Style` applied to the page slot host node.
    #[new(skip)]
    #[get(type(copy))]
    pub(crate) style: Style,
}

/// Reactive route-driven page outlet seamlessly rendering compiled routes, hot-updated pages, and runtime-injected routes.
#[derive(CustomDebug, Data, New)]
pub struct DynamicPageOutlet {
    /// Reactive route path signal controlling which page is currently active.
    #[get(type(copy))]
    pub(crate) route: Signal<String>,
    /// Fallback router closure invoked when the active route has no dynamic page override.
    #[debug(skip)]
    #[get(skip)]
    pub(crate) fallback: DynamicRouteFallbackFn,
    /// Optional explicit `DynamicUpdateHub` override (defaults to the ambient `BuildContext` hub).
    #[new(skip)]
    #[get(skip)]
    pub(crate) hub: Option<DynamicUpdateHub>,
    /// Optional container `Style` applied to the outlet host wrapper node.
    #[new(skip)]
    #[get(type(copy))]
    pub(crate) style: Style,
}
