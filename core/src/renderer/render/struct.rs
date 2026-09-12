use super::*;

/// A RAII wrapper around a raw pointer that frees the allocation on drop.
///
/// Used to ensure heap allocations captured by closures are properly freed
/// when the closure is dropped (e.g., when a DynamicNode is cleaned up).
///
/// # Safety
///
/// The pointer must have been allocated via `Box::into_raw`. Only one
/// `OwnedPtr` should exist per allocation (no aliasing ownership).
#[derive(Debug)]
pub(crate) struct OwnedPtr<T> {
    /// The raw pointer owned by this wrapper.
    pub(crate) ptr: *mut T,
}

/// Manages the rendering of virtual DOM nodes to the real DOM.
///
/// Maintains a mapping between virtual nodes and real DOM elements,
/// and handles creation, diffing, and patching of the DOM tree.
#[derive(CustomDebug, Data, New)]
pub(crate) struct Renderer {
    /// The root DOM element.
    #[debug(skip)]
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) root: Element,
    /// The current virtual DOM tree.
    #[debug(skip)]
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    pub(crate) current_tree: Option<VirtualNode>,
}

/// A zero-sized struct providing a static method for mounting
/// virtual DOM trees into the real DOM.
///
/// `Mount::mount()` is the entry point for rendering a virtual DOM tree
/// to a real DOM element selected by a CSS selector.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct Mount;
