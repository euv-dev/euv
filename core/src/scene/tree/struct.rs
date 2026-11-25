use super::*;

/// Flat generation-indexed arena holding the entire retained UI scene graph.
#[derive(CustomDebug, Data)]
pub struct SceneTree {
    /// Contiguous node storage vector.
    pub(crate) nodes: Vec<SceneNode>,
    /// Recycled slot indices for O(1) node allocation.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) free_list: Vec<u32>,
    /// Root scene node handle.
    #[get(type(copy))]
    pub(crate) root: NodeId,
    /// Focus, hover, and pointer-press tracking state.
    #[get(type(copy))]
    pub(crate) focus: FocusManager,
    /// Monotonic structural/visual mutation counter.
    #[get(type(copy))]
    pub(crate) revision: u64,
}

/// Shared handle wrapping a `SceneTree` so reactive `Effect` closures can patch nodes directly.
#[derive(Clone, CustomDebug)]
pub struct SceneHandle {
    /// Shared interior-mutable reference to the thread-local scene tree.
    pub(crate) inner: Rc<RefCell<SceneTree>>,
}

/// Mounting context passed to declarative components and `view!` macro expansions.
#[derive(Clone, CustomDebug, Data)]
pub struct BuildContext {
    /// Shared scene tree handle for creating and binding nodes.
    pub(crate) scene: SceneHandle,
    /// Current viewport logical breakpoint signal for responsive layouts.
    #[get(type(copy))]
    pub(crate) breakpoint: Signal<Breakpoint>,
    /// Current OS/app theme mode signal (`Light` / `Dark`).
    #[get(type(copy))]
    pub(crate) theme_mode: Signal<ThemeMode>,
    /// Active platform kind for capability-aware adaptation.
    #[get(type(copy))]
    pub(crate) platform: PlatformKind,
    /// Active safe-area insets signal.
    #[get(type(copy))]
    pub(crate) safe_area: Signal<SafeAreaInsets>,
}

/// RAII scope guard restoring the previous ambient `BuildContext` on drop.
#[derive(CustomDebug, New)]
pub(crate) struct BuildContextGuard {
    /// Previously bound ambient context to restore when leaving scope.
    pub(crate) previous: Option<BuildContext>,
}
