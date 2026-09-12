use super::*;

/// Inner storage for a dynamic node render closure.
///
/// Boxes a `dyn FnMut(&mut HookContext) -> VirtualNode` so it can be stored behind `Rc<UnsafeCell<>>`.
#[derive(CustomDebug, Data, New)]
pub(crate) struct RenderFnInner {
    /// The boxed render closure.
    #[debug(skip)]
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) render_fn: Box<dyn FnMut(&mut HookContext) -> VirtualNode>,
}

/// Represents a text node in the virtual DOM.
///
/// Text nodes may optionally be bound to a reactive signal for automatic updates.
///
/// OPT 29: `content` is `Cow<'static, str>` instead of `String` so the
/// `html!` macro can emit `Cow::Borrowed("...")` for literal text
/// without allocating a `String` per text node per render. Runtime
/// text (signals, `format!`, `to_string()`) still falls through to
/// `Cow::Owned`. The renderer's `set_text_content` and
/// `create_text_node` calls take `&str`, which is what `Cow<'static, str>`
/// derefs to, so call sites use `.as_ref()` (yields `&str`).
#[derive(Clone, CustomDebug, Data, New)]
pub struct TextNode {
    /// The text content.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) content: Cow<'static, str>,
    /// An optional binder that wires a freshly created DOM `Text` node to
    /// its backing signal. The binder is invoked once per DOM text node at
    /// materialization time (`create_dom_with_doc`); it subscribes the
    /// source signal directly to that node, so no intermediate bridge
    /// signal is allocated per binding. Kept text nodes are patched in
    /// place without re-invoking the binder, so re-renders of a bound
    /// position never accumulate subscriptions.
    #[debug(skip)]
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) binder: Option<Rc<dyn Fn(&Text)>>,
}

/// A closure-based dynamic node that re-renders when its dependency signals change.
///
/// Holds a shared reference to a heap-allocated render closure that produces a fresh
/// `VirtualNode` on each evaluation. The renderer subscribes to the closure's
/// signals and patches the DOM automatically.
/// Contains a `HookContext` that persists hook state (like `use_signal`) across
/// re-renders, ensuring that signal values are not reset when the render function
/// is called again.
///
/// Uses `Rc<UnsafeCell<>>` instead of `Rc<RefCell<>>` to avoid runtime borrow
/// checking overhead. Safety is guaranteed by the single-threaded WASM context.
/// The `Rc` provides automatic memory management — the render closure is freed
/// when the last reference (either in the VirtualNode tree or the signal update
/// callback) is dropped.
#[derive(Clone, CustomDebug, Data, New)]
pub struct DynamicNode {
    /// Shared reference to the heap-allocated render closure inner state.
    /// `Rc` ensures automatic deallocation; `UnsafeCell` allows mutable access
    /// without RefCell overhead.
    #[debug(skip)]
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) render_fn: Rc<UnsafeCell<RenderFnInner>>,
    /// Persistent hook context for this dynamic node, storing signal
    /// state and other hook values across render cycles.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) hook_context: HookContext,
}
