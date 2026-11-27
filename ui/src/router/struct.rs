use super::*;

/// Resolved route match containing normalized path, dynamic `:param` captures, and `?key=value` query entries.
#[derive(Clone, Data, Debug, Default, Eq, PartialEq)]
pub struct EuvRouteMatch {
    /// Clean route path without query string (e.g. `"/users/42"`).
    pub(crate) path: String,
    /// Matched route pattern (e.g. `"/users/:id"`).
    pub(crate) pattern: String,
    /// Human-readable page title associated with the route.
    pub(crate) title: String,
    /// Extracted dynamic path parameters.
    pub(crate) params: HashMap<String, String>,
    /// Extracted URL query parameters.
    pub(crate) query: HashMap<String, String>,
}

/// Declarative route entry stored inside `EuvRouter`.
#[derive(Clone, CustomDebug, Data)]
pub struct EuvRouteDef {
    /// Route path pattern supporting static segments and `:param` captures.
    pub(crate) pattern: String,
    /// Human-readable route title.
    pub(crate) title: String,
    /// Optional explicit retention override (`None` delegates to the mounted page's `Lifecycle::retention_policy`).
    #[get(skip)]
    pub(crate) retention: Option<PageRetention>,
    /// Page builder invoked lazily when this route first becomes active.
    #[debug(skip)]
    #[get(skip)]
    pub(crate) builder: EuvRouteBuilder,
}

/// Snapshot describing a cached route's retention policy and live `LifecyclePhase`.
#[derive(Clone, Data, Debug, Eq, PartialEq)]
pub struct EuvRouteLifecycleEntry {
    /// Normalized route path.
    pub(crate) path: String,
    /// Active retention policy (`Destroy`, `Freeze`, or `Alive`).
    #[get(type(copy))]
    pub(crate) retention: PageRetention,
    /// Current lifecycle phase (`Active`, `Frozen`, `Mounted`).
    #[get(type(copy))]
    pub(crate) phase: LifecyclePhase,
    /// Cumulative number of activations for this route.
    #[get(type(copy))]
    pub(crate) activations: u32,
}

/// Internal cached page record managed lazily by `EuvRouterOutlet`.
#[derive(Clone, CustomDebug)]
pub(crate) struct EuvCachedPage {
    /// Per-page `ScrollView` wrapper toggled between `Display::Flex` and `Display::None`, preserving exact scroll offsets across freezes.
    pub(crate) wrapper: NodeId,
    /// Mounted page root node returned by the route builder or `Component::mount`.
    pub(crate) page_node: NodeId,
    /// Dedicated reactive ownership scope frozen/resumed/disposed on route transitions.
    pub(crate) scope: Scope,
    /// Resolved retention strategy for this page.
    pub(crate) retention: PageRetention,
    /// Current lifecycle phase of this cached page.
    pub(crate) phase: LifecyclePhase,
    /// Theme tokens currently applied to this page subtree for zero-teardown in-place theme remapping.
    pub(crate) applied_tokens: ThemeTokens,
    /// Whether `applied_tokens` holds an initialized theme snapshot.
    pub(crate) has_applied_tokens: bool,
    /// Dynamic OTA page revision active when this page was rendered.
    pub(crate) dynamic_rev: u64,
    /// Cumulative activation counter.
    pub(crate) activations: u32,
}

/// Internal snapshot of an active or just-released interactive horizontal page drag preview.
#[derive(Clone, Debug)]
pub(crate) struct EuvDragPreviewState {
    /// Adjacent destination route path currently positioned beside the active page.
    pub(crate) target_path: String,
    /// Wrapper `NodeId` of the adjacent preview page.
    pub(crate) target_wrapper: NodeId,
    /// Direction (`+1.0` forward/next on left drag, `-1.0` backward/prev on right drag).
    pub(crate) direction: f32,
    /// Current horizontal translation offset `dx` of the active page in logical pixels.
    pub(crate) offset_x: f32,
    /// Outlet width captured during the drag.
    pub(crate) width: f32,
}

/// Shared immutable context bundle used by `EuvRouterOutlet` when lazily mounting a route wrapper.
#[derive(Clone)]
pub(crate) struct EuvOutletMountEnv {
    /// Bound router handle.
    pub(crate) router: EuvRouter,
    /// Fallback route builder.
    pub(crate) fallback: EuvRouteBuilder,
    /// Optional dynamic OTA update hub.
    pub(crate) hub_opt: Option<DynamicUpdateHub>,
    /// Default retention policy when unspecified by route or component.
    pub(crate) default_ret: PageRetention,
    /// Build context used to mount new pages.
    pub(crate) ctx: Context,
    /// Retained scene handle.
    pub(crate) scene: SceneHandle,
    /// Outlet host node handle.
    pub(crate) host: NodeId,
}

/// Internal mutable history stack shared across cloned `EuvRouter` handles.
#[derive(Clone, CustomDebug, Data)]
pub(crate) struct EuvRouterInner {
    /// Registered declarative routes in match priority order.
    pub(crate) routes: Vec<EuvRouteDef>,
    /// Ordered navigation history stack.
    pub(crate) history: Vec<String>,
    /// Current cursor index inside `history`.
    #[get(type(copy))]
    pub(crate) cursor: usize,
    /// Optional dynamic OTA hub allowing remote hot-pushed page overrides.
    #[debug(skip)]
    #[get(skip)]
    pub(crate) dynamic_hub: Option<DynamicUpdateHub>,
    /// Default retention policy applied when neither route nor component overrides it.
    #[get(type(copy))]
    pub(crate) default_retention: PageRetention,
}

/// Production cross-platform UI router with reactive history stack, lazy page mounting, and `PageRetention` lifecycle management.
#[derive(Clone, CustomDebug, Data)]
pub struct EuvRouter {
    /// Reactive active route path signal.
    #[get(type(copy))]
    pub(crate) current_path: Signal<String>,
    /// Reactive back-navigation availability signal.
    #[get(type(copy))]
    pub(crate) can_go_back: Signal<bool>,
    /// Reactive forward-navigation availability signal.
    #[get(type(copy))]
    pub(crate) can_go_forward: Signal<bool>,
    /// Reactive history depth signal.
    #[get(type(copy))]
    pub(crate) history_len: Signal<usize>,
    /// Reactive count of currently frozen cached pages (`PageRetention::Freeze`).
    #[get(type(copy))]
    pub(crate) frozen_count: Signal<usize>,
    /// Reactive count of currently alive background pages (`PageRetention::Alive`).
    #[get(type(copy))]
    pub(crate) alive_count: Signal<usize>,
    /// Reactive snapshot of all mounted/cached route lifecycle entries.
    #[get(type(copy))]
    pub(crate) lifecycle_entries: Signal<Vec<EuvRouteLifecycleEntry>>,
    /// Shared mutable route table and history stack.
    pub(crate) inner: Rc<RefCell<EuvRouterInner>>,
}

/// Declarative animated router outlet lazily mounting only the active page and enforcing `PageRetention` (`Destroy` / `Freeze` / `Alive`).
#[derive(Clone, CustomDebug, Data)]
pub struct EuvRouterOutlet {
    /// Bound UI router handle.
    pub(crate) router: EuvRouter,
    /// Fallback page builder invoked when no registered route matches.
    #[debug(skip)]
    #[get(skip)]
    pub(crate) fallback: EuvRouteBuilder,
    /// Optional reactive theme state enabling in-place theme updates without page teardown or scroll loss.
    #[debug(skip)]
    #[get(skip)]
    pub(crate) theme: Option<UiThemeState>,
    /// Enables standard app double-back-to-exit at the root route with a bottom snackbar/toast prompt.
    #[get(type(copy))]
    pub(crate) double_back_exit: bool,
    /// Bottom prompt message displayed on the first root Back press before a consecutive second Back exits.
    pub(crate) exit_prompt: String,
    /// Optional ordered route sequence used to compute coupled 1:1 left/right directional slide transitions across tabs and swipes.
    pub(crate) route_order: Vec<String>,
    /// Optional reactive per-page scroll container padding synchronized with the hosting shell.
    #[get(skip)]
    pub(crate) page_padding: Option<Signal<Edges>>,
}

/// Declarative navigation link pushing a target path onto an `EuvRouter` when clicked.
#[derive(Clone, Data, Debug, Props)]
pub struct EuvLink {
    /// Target route path (e.g. `"/themes"`).
    pub(crate) to: String,
    /// Visible link label.
    pub(crate) label: String,
    /// Bound router handle.
    pub(crate) router: EuvRouter,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}
