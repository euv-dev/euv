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
    /// Full-width wrapper node toggled between `Display::Flex` and `Display::None`.
    pub(crate) wrapper: NodeId,
    /// Mounted page root node returned by the route builder or `Component::mount`.
    pub(crate) page_node: NodeId,
    /// Dedicated reactive ownership scope frozen/resumed/disposed on route transitions.
    pub(crate) scope: Scope,
    /// Resolved retention strategy for this page.
    pub(crate) retention: PageRetention,
    /// Current lifecycle phase of this cached page.
    pub(crate) phase: LifecyclePhase,
    /// Optional theme `(scheme, mode)` active when this page was rendered.
    pub(crate) theme_key: Option<(DesignScheme, ThemeMode)>,
    /// Dynamic OTA page revision active when this page was rendered.
    pub(crate) dynamic_rev: u64,
    /// Cumulative activation counter.
    pub(crate) activations: u32,
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
    /// Optional reactive theme state enabling zero-cost frozen pages to refresh cleanly upon resume when theme changes.
    #[debug(skip)]
    #[get(skip)]
    pub(crate) theme: Option<UiThemeState>,
    /// Enables standard app double-back-to-exit at the root route with a bottom snackbar/toast prompt.
    #[get(type(copy))]
    pub(crate) double_back_exit: bool,
    /// Bottom prompt message displayed on the first root Back press before a consecutive second Back exits.
    pub(crate) exit_prompt: String,
}

/// Declarative navigation link pushing a target path onto an `EuvRouter` when clicked.
#[derive(Clone, Data, Debug)]
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
