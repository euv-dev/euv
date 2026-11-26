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
    /// Page builder invoked whenever this route becomes active.
    #[debug(skip)]
    #[get(skip)]
    pub(crate) builder: EuvRouteBuilder,
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
}

/// Production cross-platform UI router with reactive history stack, `:param`/`?query` parsing, and system-back overlay integration.
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
    /// Shared mutable route table and history stack.
    pub(crate) inner: Rc<RefCell<EuvRouterInner>>,
}

/// Declarative animated router outlet rendering the active `EuvRouter` page and binding system Back handling automatically.
#[derive(Clone, CustomDebug, Data)]
pub struct EuvRouterOutlet {
    /// Bound UI router handle.
    pub(crate) router: EuvRouter,
    /// Fallback page builder invoked when no registered route matches.
    #[debug(skip)]
    #[get(skip)]
    pub(crate) fallback: EuvRouteBuilder,
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
