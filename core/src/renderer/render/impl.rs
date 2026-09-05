use super::*;

/// Implementation of owned pointer for heap-allocated renderer state.
///
/// Wraps a raw pointer to ensure the heap allocation is properly freed
/// when the `OwnedPtr` is dropped. Used for renderer sub-trees and
/// arm-tracking state in dynamic nodes.
impl<T> OwnedPtr<T> {
    /// Creates a new `OwnedPtr` from a `Box::into_raw` pointer.
    ///
    /// # Arguments
    ///
    /// - `*mut T` - The raw pointer to wrap, obtained from `Box::into_raw`.
    ///
    /// # Returns
    ///
    /// - `Self` - A new `OwnedPtr` wrapping the given pointer.
    pub(crate) fn new(pointer: *mut T) -> Self {
        Self { ptr: pointer }
    }

    /// Returns the raw pointer for direct access.
    ///
    /// # Returns
    ///
    /// - `*mut T` - The wrapped raw pointer.
    pub(crate) fn get(&self) -> *mut T {
        self.ptr
    }
}

/// Implementation of `Drop` for `OwnedPtr`.
///
/// Ensures the heap-allocated memory is properly freed when the `OwnedPtr`
/// goes out of scope. This prevents memory leaks for renderer sub-trees
/// and other dynamically allocated state.
impl<T> Drop for OwnedPtr<T> {
    /// Drops the owned pointer, freeing the heap allocation.
    ///
    /// # Safety
    ///
    /// This is safe because `OwnedPtr` is only used in single-threaded WASM
    /// contexts, and the pointer is always valid as long as the `OwnedPtr` exists.
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                let _: Box<T> = Box::from_raw(self.ptr);
            }
        }
    }
}

/// Implementation of the virtual DOM renderer.
impl Renderer {
    /// Renders the given virtual DOM tree into the real DOM.
    ///
    /// If a previous tree exists, patches the existing DOM to match the new tree.
    /// Otherwise, creates new DOM nodes from scratch and appends them to the root.
    ///
    /// OPT 1 (zero-copy VDOM): the owned `VirtualNode` is moved into
    /// `unwrap_component_owned`, which first performs a cheap
    /// zero-allocation pre-scan via [`Self::subtree_has_component`] to
    /// detect any `Tag::Component` markers. Component-free trees (the
    /// common case for `html!` closures that use only native elements)
    /// are returned by move without touching a single `Vec` — the deep
    /// tree's `Vec` allocations, `Tag` strings, and attribute vectors
    /// are all reused by the renderer. Trees that actually contain
    /// components pay the rebuild cost via
    /// [`Self::unwrap_component_owned_slow`], where the edge clone
    /// happens only along the descent path that touches a component
    /// wrapper, not over the whole tree.
    ///
    /// # Arguments
    ///
    /// - `VirtualNode` - The new virtual DOM tree to render.
    pub fn render(&mut self, vnode: VirtualNode) {
        let new_unwrapped: VirtualNode = Self::unwrap_component_owned(vnode);
        let old_tree: Option<VirtualNode> = take(self.get_mut_current_tree());
        if let Some(old_vnode) = old_tree.as_ref() {
            self.patch_root(old_vnode, &new_unwrapped);
        } else {
            while let Some(child) = self.get_root().first_child() {
                if let Some(element) = child.dyn_ref::<Element>() {
                    Self::cleanup_subtree(element);
                }
                let _: Result<Node, JsValue> = self.get_root().remove_child(&child);
            }
            let dom_node: Node = self.create_dom_node(&new_unwrapped);
            let _: Result<Node, JsValue> = self.get_root().append_child(&dom_node);
        }
        self.set_current_tree(Some(new_unwrapped));
    }

    /// Renders the given virtual DOM tree into the real DOM by fully replacing
    /// all existing content. Used when a match arm switch occurs (e.g. route
    /// change) where incremental patching would incorrectly align unrelated
    /// child nodes from the previous arm.
    ///
    /// # Arguments
    ///
    /// - `VirtualNode` - The new virtual DOM tree to render.
    pub fn render_full_replace(&mut self, vnode: VirtualNode) {
        let new_unwrapped: VirtualNode = Self::unwrap_component_owned(vnode);
        while let Some(child) = self.get_root().first_child() {
            if let Some(element) = child.dyn_ref::<Element>() {
                Self::cleanup_subtree(element);
            }
            let _: Result<Node, JsValue> = self.get_root().remove_child(&child);
        }
        let dom_node: Node = self.create_dom_node(&new_unwrapped);
        let _: Result<Node, JsValue> = self.get_root().append_child(&dom_node);
        self.set_current_tree(Some(new_unwrapped));
    }

    /// Patches the root DOM tree by replacing the single child of `self.root`.
    ///
    /// # Arguments
    ///
    /// - `&VirtualNode` - The old virtual node to patch from.
    /// - `&VirtualNode` - The new virtual node to patch to.
    fn patch_root(&mut self, old_node: &VirtualNode, new_node: &VirtualNode) {
        let dom_child: Option<Node> = self.get_root().first_child();
        let is_element: bool = if let Some(ref dom_child) = dom_child {
            dom_child.dyn_ref::<Element>().is_some()
        } else {
            false
        };
        if is_element {
            if let Some(dom_child) = dom_child
                && let Ok(element) = dom_child.dyn_into::<Element>()
            {
                self.patch_node(old_node, new_node, &element);
            }
        } else if let Some(dom_child) = dom_child {
            if let Some(element) = dom_child.dyn_ref::<Element>() {
                Self::cleanup_subtree(element);
            }
            let new_dom_node: Node = self.create_dom_node(new_node);
            let _: Result<Node, JsValue> = self.get_root().replace_child(&new_dom_node, &dom_child);
        } else {
            let new_dom_node: Node = self.create_dom_node(new_node);
            let _: Result<Node, JsValue> = self.get_root().append_child(&new_dom_node);
        }
    }

    /// Patches an existing DOM node to match the new virtual node.
    ///
    /// # Arguments
    ///
    /// - `&VirtualNode` - The old virtual node.
    /// - `&VirtualNode` - The new virtual node.
    /// - `&Element` - The real DOM element to patch.
    fn patch_node(
        &mut self,
        old_node: &VirtualNode,
        new_node: &VirtualNode,
        dom_element: &Element,
    ) {
        match (old_node, new_node) {
            (VirtualNode::Text(old_text), VirtualNode::Text(new_text)) => {
                if old_text != new_text {
                    dom_element.set_text_content(Some(new_text.get_content()));
                }
            }
            (
                VirtualNode::Element {
                    tag: old_tag,
                    attributes: old_attrs,
                    children: old_children,
                    key: _old_key,
                    ..
                },
                VirtualNode::Element {
                    tag: new_tag,
                    attributes: new_attrs,
                    children: new_children,
                    key: _new_key,
                    ..
                },
            ) => {
                if old_tag != new_tag {
                    let new_dom_node: Node = self.create_dom_node(new_node);
                    if let Some(parent) = dom_element.parent_node() {
                        Self::cleanup_subtree(dom_element);
                        let _: Result<Node, JsValue> =
                            parent.replace_child(&new_dom_node, dom_element);
                    }
                    return;
                }
                // Portal markers carry the original `data-euv-portal`
                // attribute set at mount time. The actual children
                // live in a separate DOM subtree (the resolved
                // target), so neither the children nor the attributes
                // contributed by the user should be re-applied to the
                // marker. We deliberately skip both
                // `patch_children` and `patch_attributes` here and
                // rely on `render_full_replace` (the match-arm
                // switcher) to remount portals when their
                // declaration in the tree changes.
                if matches!(old_tag, Tag::Portal(_)) {
                    return;
                }
                self.patch_children(dom_element, old_children, new_children);
                self.patch_attributes(dom_element, old_attrs, new_attrs);
            }
            (VirtualNode::Fragment(old_children), VirtualNode::Fragment(new_children)) => {
                self.patch_children(dom_element, old_children, new_children);
            }
            (VirtualNode::Dynamic(_old_dynamic), VirtualNode::Dynamic(_new_dynamic)) => {}
            (VirtualNode::Dynamic(_), _) => {
                let new_dom_node: Node = self.create_dom_node(new_node);
                if let Some(parent) = dom_element.parent_node() {
                    Self::cleanup_subtree(dom_element);
                    let _: Result<Node, JsValue> = parent.replace_child(&new_dom_node, dom_element);
                }
            }
            (_, VirtualNode::Dynamic(_)) => {
                let new_dom_node: Node = self.create_dom_node(new_node);
                if let Some(parent) = dom_element.parent_node() {
                    Self::cleanup_subtree(dom_element);
                    let _: Result<Node, JsValue> = parent.replace_child(&new_dom_node, dom_element);
                }
            }
            _ => {
                let new_dom_node: Node = self.create_dom_node(new_node);
                if let Some(parent) = dom_element.parent_node() {
                    Self::cleanup_subtree(dom_element);
                    let _: Result<Node, JsValue> = parent.replace_child(&new_dom_node, dom_element);
                }
            }
        }
    }

    /// Patches attributes of an element, adding, removing, or updating as needed.
    ///
    /// # Arguments
    ///
    /// - `&Element` - The DOM element whose attributes to patch.
    /// - `&[AttributeEntry]` - The old attribute list.
    /// - `&[AttributeEntry]` - The new attribute list.
    ///
    /// OPT 5: the `data-euv-id` lookup is hoisted out of the per-attribute
    /// event-handler removal path so it only runs when at least one
    /// removed attribute is an `Event` listener. The fast path
    /// (`non_event` attribute removal) skips the registry entirely.
    /// New-attribute writes use a direct `HashMap<&str, &AttributeValue>`
    /// for O(1) lookup-by-name instead of an `O(N)` linear `find` per
    /// attribute.
    fn patch_attributes(
        &mut self,
        element: &Element,
        old_attrs: &[AttributeEntry],
        new_attrs: &[AttributeEntry],
    ) {
        let old_index: HashMap<&str, &AttributeValue> = old_attrs
            .iter()
            .map(|a| (a.get_name().as_ref(), a.get_value()))
            .collect();
        let new_index: HashMap<&str, &AttributeValue> = new_attrs
            .iter()
            .map(|a| (a.get_name().as_ref(), a.get_value()))
            .collect();
        let mut needs_event_cleanup: bool = false;
        for old_attr in old_attrs {
            let old_name: &str = old_attr.get_name().as_ref();
            if !new_index.contains_key(old_name) {
                if let AttributeValue::Event(_) = old_attr.get_value() {
                    needs_event_cleanup = true;
                }
                element.remove_attribute_or_property(old_attr.get_name());
            }
        }
        let cached_euv_id: usize = if needs_event_cleanup {
            match element.get_attribute(DATA_EUV_ID) {
                Some(id_str) => id_str.parse::<usize>().unwrap_or_else(|_| {
                    let new_id: usize = NEXT_EUV_ID.fetch_add(1, Ordering::Relaxed);
                    let _: Result<(), JsValue> =
                        element.set_attribute(DATA_EUV_ID, &new_id.to_string());
                    new_id
                }),
                None => {
                    let new_id: usize = NEXT_EUV_ID.fetch_add(1, Ordering::Relaxed);
                    let _: Result<(), JsValue> =
                        element.set_attribute(DATA_EUV_ID, &new_id.to_string());
                    new_id
                }
            }
        } else {
            0
        };
        if needs_event_cleanup {
            self.detach_removed_event_handlers(old_attrs, &new_index, cached_euv_id);
        }
        for new_attr in new_attrs {
            match new_attr.get_value() {
                AttributeValue::Event(handler) => {
                    self.attach_event_listener(element, handler);
                }
                _ => {
                    let new_name: &str = new_attr.get_name().as_ref();
                    let old_value: Option<&&AttributeValue> = old_index.get(new_name);
                    let should_set: bool = match old_value {
                        Some(old_val) => *old_val != new_attr.get_value(),
                        None => true,
                    };
                    if should_set {
                        match new_attr.get_value() {
                            AttributeValue::Text(value) => {
                                element.set_attribute_or_property(new_attr.get_name(), value);
                            }
                            AttributeValue::StaticText(value) => {
                                element.set_attribute_or_property(new_attr.get_name(), value);
                            }
                            AttributeValue::Signal(signal) => {
                                let value: String = signal.get();
                                element.set_attribute_or_property(new_attr.get_name(), &value);
                            }
                            AttributeValue::Dynamic(_) => {}
                            AttributeValue::Css(css) => {
                                css.inject_style();
                                element
                                    .set_attribute_or_property(new_attr.get_name(), css.get_name());
                            }
                            // OPT 11: CssRef shares `Css` handling — inject
                            // the style once (idempotent), then use the
                            // class name with no per-element clone.
                            AttributeValue::CssRef(css) => {
                                css.inject_style();
                                element
                                    .set_attribute_or_property(new_attr.get_name(), css.get_name());
                            }
                            AttributeValue::Event(_) => {}
                            AttributeValue::InnerHtml(html) => {
                                element.set_inner_html(html);
                            }
                            AttributeValue::InnerHtmlSignal(signal) => {
                                let value: String = signal.get();
                                element.set_inner_html(&value);
                            }
                            AttributeValue::Ref(node_ref) => {
                                let element_value: JsValue = element.clone().into();
                                node_ref.set(element_value);
                            }
                        }
                    }
                }
            }
        }
    }

    /// OPT 5: secondary helper that walks only the `AttributeValue::Event`
    /// entries removed from `old_attrs` (i.e. present in `old` but absent
    /// from `new_attrs`) and detaches them. The element's `data-euv-id`
    /// was read once by the caller (`patch_attributes`) and passed in
    /// here so this path does no extra attribute parsing.
    ///
    /// # Arguments
    ///
    /// - `&[AttributeEntry]` - Shared reference to a `[AttributeEntry]`.
    /// - `&HashMap<&str, &AttributeValue>` - Shared reference to a `HashMap<&str, &AttributeValue>`.
    /// - `usize` - A non-negative integer (`usize`).
    fn detach_removed_event_handlers(
        &self,
        old_attrs: &[AttributeEntry],
        new_index: &HashMap<&str, &AttributeValue>,
        euv_id: usize,
    ) {
        for old_attr in old_attrs {
            if let AttributeValue::Event(handler) = old_attr.get_value() {
                let old_name: &str = old_attr.get_name().as_ref();
                if new_index.contains_key(old_name) {
                    continue;
                }
                if let Some(entry) = Registry::get_mut_handler_registry()
                    .get_mut(&euv_id)
                    .and_then(|event_map: &mut HashMap<&'static str, HandlerEntry>| {
                        event_map.remove(&handler.get_event_name())
                    })
                {
                    let slot: &mut HandlerSlot = unsafe { &mut *entry };
                    if let Some(listener_element) = slot.try_get_element().as_ref().cloned()
                        && let Some(listener_function) = slot.get_mut_listener_function().take()
                    {
                        let event_name: &str = handler.get_event_name();
                        let listener: &Function = listener_function.unchecked_ref::<Function>();
                        let _: Result<(), JsValue> = listener_element
                            .remove_event_listener_with_callback(event_name, listener);
                    }
                    slot.set_handler(None);
                    unsafe {
                        let _: Box<HandlerSlot> = Box::from_raw(entry);
                    }
                }
            }
        }
    }

    /// OPT 4: removed `try_get_child_node` — its sole call site in
    /// `patch_children_positional` now reads from the hoisted NodeList
    /// directly.
    /// Patches children of an element using a keyed diff algorithm when keys
    /// are available, falling back to positional diff when no keys exist.
    ///
    /// When all children in both old and new lists have keys, this method
    /// builds a key-to-index map and applies a minimal set of DOM moves,
    /// insertions, and removals. This avoids the O(N) per-child re-patch
    /// that the naive positional algorithm incurs when items are reordered.
    ///
    /// # Arguments
    ///
    /// - `&Element` - The parent DOM element.
    /// - `&[VirtualNode]` - The old children list.
    /// - `&[VirtualNode]` - The new children list.
    ///
    /// OPT 3/4: `parent.child_nodes()` is now hoisted to a single
    /// `NodeList` capture taken once per call, then reused across every
    /// per-child index lookup. The previous implementation called
    /// `parent.child_nodes()` inside the inner loop, which returned a
    /// fresh live `NodeList` view each iteration — each lookup crossed
    /// the JS boundary again. With the hoisted `NodeList`, the cost
    /// per child drops from one JS round-trip to one C-side index
    /// access. The same fast path applies to both keyed and positional
    /// diff (the latter further drops the `try_get_child_node`
    /// convenience wrapper that previously masked this hot path).
    fn patch_children(
        &mut self,
        parent: &Element,
        old_children: &[VirtualNode],
        new_children: &[VirtualNode],
    ) {
        let old_has_keys: bool =
            !old_children.is_empty() && old_children.iter().all(VirtualNode::has_key);
        let new_has_keys: bool =
            !new_children.is_empty() && new_children.iter().all(VirtualNode::has_key);
        if old_has_keys && new_has_keys {
            self.patch_children_keyed(parent, old_children, new_children);
        } else {
            self.patch_children_positional(parent, old_children, new_children);
        }
    }

    /// Keyed diffing algorithm that minimizes DOM operations.
    ///
    /// Builds a mapping from old keys to their DOM indices, then walks the
    /// new children list. For each new child:
    ///
    /// - If its key existed in the old list, patches the existing DOM node.
    /// - Otherwise, creates a new DOM node.
    ///
    /// After processing all new children, removes any old DOM nodes whose
    /// keys are no longer present in the new list.
    ///
    /// # Arguments
    ///
    /// - `&Element` - The parent DOM element.
    /// - `&[VirtualNode]` - The old children list.
    /// - `&[VirtualNode]` - The new children list.
    fn patch_children_keyed(
        &mut self,
        parent: &Element,
        old_children: &[VirtualNode],
        new_children: &[VirtualNode],
    ) {
        // OPT 3: hoist `parent.child_nodes()` to a single live NodeList
        // reference taken once per call. Each subsequent `child_nodes.get(i)`
        // is a C-side index lookup into the existing live view — no extra
        // JS round-trip per child.
        let child_nodes: NodeList = parent.child_nodes();
        let dom_child_count: u32 = child_nodes.length();
        let mut old_key_to_node: HashMap<&str, (usize, Node)> =
            HashMap::with_capacity(old_children.len());
        for (index, old_child) in old_children.iter().enumerate() {
            if let Some(key) = old_child.key() {
                let dom_index: u32 = index as u32;
                if dom_index < dom_child_count
                    && let Some(node) = child_nodes.get(dom_index)
                {
                    old_key_to_node.insert(key, (index, node));
                }
            }
        }
        let mut new_key_set: HashSet<&str> = HashSet::with_capacity(new_children.len());
        for new_child in new_children.iter() {
            if let Some(key) = new_child.key() {
                new_key_set.insert(key);
            }
        }
        for (index, old_child) in old_children.iter().enumerate() {
            if let Some(key) = old_child.key() {
                if !new_key_set.contains(key)
                    && let Some((_old_index, dom_node)) = old_key_to_node.remove(key)
                {
                    if let Some(element) = dom_node.dyn_ref::<Element>() {
                        Self::cleanup_subtree(element);
                    }
                    let _: Result<Node, JsValue> = parent.remove_child(&dom_node);
                }
            } else {
                let dom_index: u32 = index as u32;
                if dom_index < dom_child_count
                    && let Some(dom_node) = child_nodes.get(dom_index)
                {
                    if let Some(element) = dom_node.dyn_ref::<Element>() {
                        Self::cleanup_subtree(element);
                    }
                    let _: Result<Node, JsValue> = parent.remove_child(&dom_node);
                }
            }
        }
        for (new_index, new_child) in new_children.iter().enumerate() {
            let new_key: &str = new_child.key().unwrap_or_default();
            let target_index: u32 = new_index as u32;
            // OPT 3: same hoisted NodeList, no re-fetch.
            let current_at_target: Option<Node> = child_nodes.get(target_index);
            if let Some((old_vnode_index, dom_node)) = old_key_to_node.remove(new_key) {
                let old_child: &VirtualNode = &old_children[old_vnode_index];
                if let Some(element) = dom_node.dyn_ref::<Element>() {
                    self.patch_node(old_child, new_child, element);
                }
                if current_at_target.as_ref() != Some(&dom_node) {
                    if let Some(reference_node) = current_at_target {
                        let _: Result<Node, JsValue> =
                            parent.insert_before(&dom_node, Some(&reference_node));
                    } else {
                        let _: Result<Node, JsValue> = parent.append_child(&dom_node);
                    }
                }
            } else {
                let new_dom_node: Node = self.create_dom_node(new_child);
                if let Some(reference_node) = current_at_target {
                    let _: Result<Node, JsValue> =
                        parent.insert_before(&new_dom_node, Some(&reference_node));
                } else {
                    let _: Result<Node, JsValue> = parent.append_child(&new_dom_node);
                }
            }
        }
    }

    /// Positional diffing algorithm (original behavior).
    ///
    /// Patches children by index position. Used as a fallback when keys
    /// are not available on all children.
    ///
    /// # Arguments
    ///
    /// - `&Element` - The parent DOM element.
    /// - `&[VirtualNode]` - The old children list.
    /// - `&[VirtualNode]` - The new children list.
    ///
    /// OPT 4: hoists `parent.child_nodes()` to a single live NodeList
    /// reference taken once per call. Replaces the previous per-child
    /// `parent.child_nodes().get(index)` (which crossed the JS boundary
    /// each iteration) with a direct index lookup into the captured
    /// NodeList. The `try_get_child_node` helper that previously hid
    /// this hot path is removed.
    fn patch_children_positional(
        &mut self,
        parent: &Element,
        old_children: &[VirtualNode],
        new_children: &[VirtualNode],
    ) {
        let old_len: usize = old_children.len();
        let new_len: usize = new_children.len();
        let common_len: usize = old_len.min(new_len);
        // OPT 4: hoisted NodeList — single JS round-trip per call, not
        // per child. Reused across the whole positional patch loop.
        let child_nodes: NodeList = parent.child_nodes();
        for index in 0..common_len {
            let old_child: &VirtualNode = &old_children[index];
            let new_child: &VirtualNode = &new_children[index];
            let dom_index: u32 = index as u32;
            if let Some(dom_child) = child_nodes.get(dom_index) {
                if let Some(element) = dom_child.dyn_ref::<Element>() {
                    self.patch_node(old_child, new_child, element);
                } else if let (VirtualNode::Text(old_text), VirtualNode::Text(new_text)) =
                    (old_child, new_child)
                {
                    if old_text != new_text {
                        dom_child.set_text_content(Some(new_text.get_content()));
                    }
                } else {
                    let new_dom_node: Node = self.create_dom_node(new_child);
                    if let Some(parent_node) = dom_child.parent_node() {
                        if let Some(child_element) = dom_child.dyn_ref::<Element>() {
                            Self::cleanup_subtree(child_element);
                        }
                        let _: Result<Node, JsValue> =
                            parent_node.replace_child(&new_dom_node, &dom_child);
                    }
                }
            }
        }
        if new_len > old_len {
            // OPT 13: batch the trailing appends via DocumentFragment
            // so a re-render that adds N siblings only triggers one
            // layout invalidation on the parent.
            //
            // We map the borrowed slice directly into a `Vec<Node>` so
            // the virtual nodes themselves stay borrowed — no
            // per-node deep clone like the previous loop paid.
            let appended: Vec<Node> = new_children
                .iter()
                .skip(common_len)
                .map(|new_child| self.create_dom_node(new_child))
                .collect();
            append_nodes(parent, appended);
        } else if old_len > new_len {
            for _ in common_len..old_len {
                if let Some(last_child) = parent.last_child()
                    && let Some(element) = last_child.dyn_ref::<Element>()
                {
                    Self::cleanup_subtree(element);
                }
                if let Some(last_child) = parent.last_child() {
                    let _: Result<Node, JsValue> = parent.remove_child(&last_child);
                }
            }
        }
    }

    /// Creates a real DOM node from a virtual node.
    ///
    /// # Arguments
    ///
    /// - `&VirtualNode` - The virtual node to materialize.
    ///
    /// # Returns
    ///
    /// - `Node` - The created DOM node.
    ///
    fn create_dom_node(&mut self, node: &VirtualNode) -> Node {
        let document: Document = match cached_document() {
            Some(document_instance) => document_instance,
            None => return JsValue::UNDEFINED.into(),
        };
        self.create_dom_with_doc(node, &document)
    }

    /// Creates a real DOM node using a pre-acquired document reference.
    ///
    /// # Arguments
    ///
    /// - `&VirtualNode` - The virtual node to materialize.
    /// - `&Document` - The document reference for creating DOM elements.
    ///
    /// # Returns
    ///
    /// - `Node` - The created DOM node.
    fn create_dom_with_doc(&mut self, node: &VirtualNode, document: &Document) -> Node {
        match node {
            VirtualNode::Element {
                tag,
                attributes,
                children,
                ..
            } => {
                let element: Element = match tag {
                    Tag::Element(name) => match document.create_element(name) {
                        Ok(created_element) => created_element,
                        Err(_err) => return document.create_text_node(EMPTY_STRING).into(),
                    },
                    Tag::Component(_) => {
                        let unwrapped: VirtualNode = Self::unwrap_component_owned(node.clone());
                        return self.create_dom_with_doc(&unwrapped, document);
                    }
                    Tag::Portal(selector) => {
                        // Portal mount protocol:
                        //
                        // 1. Insert a hidden marker `<div
                        //    data-euv-portal="<selector>">` at the
                        //    declared position. The marker is a
                        //    real Element so the parent's
                        //    `patch_children_positional` loop treats
                        //    it as a regular child (no Comment-node
                        //    handling, no special-casing in the
                        //    patch code).
                        //
                        // 2. Resolve the target element via
                        //    `document.query_selector(selector)`,
                        //    falling back to `document.body()` if
                        //    the selector doesn't match anything.
                        //
                        // 3. Append each child node to the target
                        //    element rather than to the marker.
                        //
                        // The marker carries the original selector
                        // in `data-euv-portal`, so future patch
                        // passes can detect "this child is a
                        // portal" by querying the attribute. See
                        // `is_portal_marker` below.
                        // `<div>` is part of the HTML spec on every browser the framework
                        // targets; if `create_element("div")` ever fails we
                        // fall back to a freshly-created empty text node
                        // (`Document::create_text_node` is also total on
                        // every supported browser) so the portal marker
                        // remains a real `Element` and the parent's
                        // positional patch loop keeps treating it as a
                        // regular child. The marker itself is
                        // `display:none` so a fallback that loses its
                        // marker attribute is invisible anyway.
                        let marker: Element =
                            document.create_element("div").unwrap_or_else(|_err| {
                                let fallback: Text = document.create_text_node(EMPTY_STRING);
                                let element_value: JsValue = fallback.into();
                                element_value.unchecked_into::<Element>()
                            });
                        let _: Result<(), JsValue> =
                            marker.set_attribute("data-euv-portal", selector);
                        let _: Result<(), JsValue> = marker.set_attribute("style", "display:none");
                        let target: Element = document
                            .query_selector(selector)
                            .ok()
                            .flatten()
                            .or_else(|| document.body().map(HtmlElement::into))
                            .unwrap_or_else(|| marker.clone());
                        // OPT 13: batch portal-target appends via a single
                        // DocumentFragment so the host element only sees
                        // one DOM mutation per portal mount, regardless
                        // of how many children the portal ships.
                        let child_nodes: Vec<Node> = children
                            .iter()
                            .map(|child| self.create_dom_with_doc(child, document))
                            .collect();
                        append_nodes(&target, child_nodes);
                        return marker.into();
                    }
                };
                let inner_html_payload: Option<String> =
                    attributes
                        .iter()
                        .find_map(|attr: &AttributeEntry| match attr.get_value() {
                            AttributeValue::InnerHtml(html) => Some(html.clone()),
                            _ => None,
                        });
                if let Some(html) = inner_html_payload.as_deref() {
                    element.set_inner_html(html);
                } else {
                    // OPT 13: batch child appends via DocumentFragment
                    // so a tree with N siblings triggers a single
                    // layout invalidation rather than N.
                    let child_nodes: Vec<Node> = children
                        .iter()
                        .map(|child| self.create_dom_with_doc(child, document))
                        .collect();
                    append_nodes(&element, child_nodes);
                }
                for attr in attributes {
                    match attr.get_value() {
                        AttributeValue::Text(value) => {
                            element.set_attribute_or_property(attr.get_name(), value);
                        }
                        // OPT 10: StaticText behaves identically to Text but
                        // skips the heap allocation entirely.
                        AttributeValue::StaticText(value) => {
                            element.set_attribute_or_property(attr.get_name(), value);
                        }
                        AttributeValue::Signal(signal) => {
                            let signal: Signal<String> = *signal;
                            let initial_value: String = signal.get();
                            element.set_attribute_or_property(attr.get_name(), &initial_value);
                            let bridge_signal: Signal<String> = Signal::create(initial_value);
                            element.track_signal_addr(bridge_signal.get_inner());
                            let attr_name: String = attr.get_name().to_string();
                            let element_clone: Element = element.clone();
                            bridge_signal.replace_listener(move || {
                                if !Renderer::is_node_connected(&element_clone) {
                                    return;
                                }
                                let new_value: String = bridge_signal.get();
                                element_clone.set_attribute_or_property(&attr_name, &new_value);
                            });
                            signal.subscribe(move || {
                                bridge_signal.set(signal.get());
                            });
                            // The closure above captures `bridge_signal`, so
                            // `signal` (the source) now transitively keeps the
                            // bridge alive. Register that dependency so the
                            // bridge's heap allocation can be reclaimed once
                            // `signal` is deactivated.
                            BridgeRefsCell::track(bridge_signal.get_inner(), signal.get_inner());
                        }
                        AttributeValue::Event(handler) => {
                            self.attach_event_listener(&element, handler);
                        }
                        AttributeValue::Dynamic(_) => {}
                        AttributeValue::Css(css) => {
                            css.inject_style();
                            element.set_attribute_or_property(attr.get_name(), css.get_name());
                        }
                        // OPT 11: CssRef — zero-copy path for static
                        // classes produced by `class!`. The shared
                        // `OnceLock<Css>` keeps the storage alive for the
                        // program's lifetime so the borrow is `'static`.
                        AttributeValue::CssRef(css) => {
                            css.inject_style();
                            element.set_attribute_or_property(attr.get_name(), css.get_name());
                        }
                        AttributeValue::InnerHtml(_) => {
                            // Already applied above before the children
                            // loop ran; nothing more to do here.
                        }
                        AttributeValue::InnerHtmlSignal(signal) => {
                            let signal: Signal<String> = *signal;
                            let initial_value: String = signal.get();
                            element.set_inner_html(&initial_value);
                            element.track_signal_addr(signal.get_inner());
                            let element_clone: Element = element.clone();
                            signal.subscribe(move || {
                                if !Renderer::is_node_connected(&element_clone) {
                                    return;
                                }
                                let new_value: String = signal.get();
                                element_clone.set_inner_html(&new_value);
                            });
                        }
                        AttributeValue::Ref(node_ref) => {
                            let element_value: JsValue = element.clone().into();
                            node_ref.set(element_value);
                        }
                    }
                }
                element.into()
            }
            VirtualNode::Text(text_node) => {
                let text: Text = document.create_text_node(text_node.get_content());
                if let Some(signal) = text_node.try_get_signal() {
                    let signal: Signal<String> = *signal;
                    let bridge_signal: Signal<String> =
                        Signal::create(text_node.get_content().clone());
                    let text_clone: Text = text.clone();
                    bridge_signal.replace_listener(move || {
                        if !Renderer::is_node_connected(&text_clone) {
                            return;
                        }
                        let new_value: String = bridge_signal.get();
                        text_clone.set_text_content(Some(&new_value));
                    });
                    signal.subscribe(move || {
                        bridge_signal.set(signal.get());
                    });
                    BridgeRefsCell::track(bridge_signal.get_inner(), signal.get_inner());
                }
                text.into()
            }
            VirtualNode::Fragment(children) => {
                let fragment: Element = match document.create_element(FRAGMENT_TAG) {
                    Ok(created_element) => created_element,
                    Err(_err) => return document.create_text_node(EMPTY_STRING).into(),
                };
                let _: Result<(), JsValue> = fragment.set_attribute(ATTR_STYLE, FRAGMENT_STYLE);
                for child in children {
                    let child_node: Node = self.create_dom_with_doc(child, document);
                    let _: Result<Node, JsValue> = fragment.append_child(&child_node);
                }
                fragment.into()
            }
            VirtualNode::Dynamic(dynamic_node) => {
                let placeholder: Element = match document.create_element(DYNAMIC_PLACEHOLDER_TAG) {
                    Ok(created_element) => created_element,
                    Err(_err) => return document.create_text_node(EMPTY_STRING).into(),
                };
                let _: Result<(), JsValue> =
                    placeholder.set_attribute(ATTR_STYLE, DISPLAY_CONTENTS_STYLE);
                let dynamic_id: usize = Self::assign_dynamic_id(&placeholder);
                let initial_dom: Node =
                    self.setup_dynamic_node(dynamic_node, dynamic_id, &placeholder, true);
                let _: Result<Node, JsValue> = placeholder.append_child(&initial_dom);
                placeholder.into()
            }
            VirtualNode::Empty => document.create_text_node(EMPTY_STRING).into(),
        }
    }

    /// Initializes a DynamicNode: runs the initial render, creates a sub-renderer,
    /// and registers the re-render callback in the signal update registry.
    ///
    /// Sets up dependency tracking so that signals accessed during the render
    /// function automatically register this dynamic node as a dependent,
    /// enabling precise dirty marking on subsequent signal changes.
    ///
    /// # Arguments
    ///
    /// - `&DynamicNode` - The dynamic node to set up.
    /// - `usize` - The unique dynamic ID assigned to the placeholder.
    /// - `&Element` - The placeholder DOM element.
    /// - `bool` - Whether to skip rendering if the output is unchanged.
    ///
    /// # Returns
    ///
    /// - `Node` - The initial rendered DOM node.
    fn setup_dynamic_node(
        &mut self,
        dynamic_node: &DynamicNode,
        dynamic_id: usize,
        placeholder: &Element,
        skip_equal: bool,
    ) -> Node {
        let mut hook_context: HookContext = dynamic_node.get_hook_context().clone();
        hook_context.reset_index();
        CURRENT_TRACKING_DYNAMIC_ID.store(dynamic_id, Ordering::Relaxed);
        let initial_vnode: VirtualNode = HookContext::with(hook_context.clone(), || {
            dynamic_node.render(&mut hook_context)
        });
        let initial_unwrapped: VirtualNode = Self::unwrap_component_owned(initial_vnode);
        CURRENT_TRACKING_DYNAMIC_ID.store(usize::MAX, Ordering::Relaxed);
        let initial_dom: Node = self.create_dom_node(&initial_unwrapped);
        let render_fn_rc: Rc<UnsafeCell<RenderFnInner>> = dynamic_node.get_render_fn().clone();
        let placeholder_clone: Element = placeholder.clone();
        let mut renderer_for_sub: Self = Self::new(placeholder_clone.clone());
        renderer_for_sub.set_current_tree(Some(initial_unwrapped));
        // Wrap heap allocations in OwnedPtr so they are freed when the closure drops.
        let renderer_owned: OwnedPtr<Renderer> =
            OwnedPtr::new(Box::into_raw(Box::new(renderer_for_sub)));
        let initial_arm: usize = hook_context
            .get_inner()
            .try_borrow()
            .map(|inner: Ref<HookContextInner>| inner.get_arm_changed())
            .unwrap_or_default();
        let last_arm_owned: OwnedPtr<usize> = OwnedPtr::new(Box::into_raw(Box::new(initial_arm)));
        let callback: Box<dyn FnMut()> = Box::new(move || {
            if placeholder_clone.parent_node().is_none() {
                return;
            }
            hook_context.reset_index();
            let prev_arm: usize = unsafe { *last_arm_owned.get() };
            CURRENT_TRACKING_DYNAMIC_ID.store(dynamic_id, Ordering::Relaxed);
            let new_vnode: VirtualNode = HookContext::with(hook_context.clone(), || {
                let inner: &mut RenderFnInner = unsafe { &mut *render_fn_rc.get() };
                (inner.get_mut_render_fn())(&mut hook_context)
            });
            let current_arm: usize = hook_context
                .get_inner()
                .try_borrow()
                .map(|inner: Ref<HookContextInner>| inner.get_arm_changed())
                .unwrap_or_default();
            let arm_switched: bool = prev_arm != current_arm;
            unsafe {
                *last_arm_owned.get() = current_arm;
            }
            if skip_equal && !arm_switched {
                let renderer_ref: &Renderer = unsafe { &*renderer_owned.get() };
                if let Some(old_vnode) = renderer_ref.try_get_current_tree() {
                    let new_unwrapped: VirtualNode = Self::unwrap_component_owned(new_vnode);
                    if Self::visual_eq(old_vnode, &new_unwrapped) {
                        CURRENT_TRACKING_DYNAMIC_ID.store(usize::MAX, Ordering::Relaxed);
                        return;
                    }
                    let renderer_mut: &mut Renderer = unsafe { &mut *renderer_owned.get() };
                    renderer_mut.render(new_unwrapped);
                    CURRENT_TRACKING_DYNAMIC_ID.store(usize::MAX, Ordering::Relaxed);
                    return;
                }
            }
            let renderer_mut: &mut Renderer = unsafe { &mut *renderer_owned.get() };
            if arm_switched {
                renderer_mut.render_full_replace(new_vnode);
            } else {
                renderer_mut.render(new_vnode);
            }
            CURRENT_TRACKING_DYNAMIC_ID.store(usize::MAX, Ordering::Relaxed);
        });
        Registry::register_dynamic(dynamic_id, callback);
        initial_dom
    }

    /// Unwraps an owned virtual node, expanding any `Tag::Component` nodes
    /// into their rendered output.
    ///
    /// OPT 1: this is the single fused expansion entry point. Two
    /// notable performance characteristics:
    ///
    /// 1. **Zero-allocation pre-scan.** `subtree_has_component` walks
    ///    the tree by shared reference first; component-free trees
    ///    (the common case for `html!` closures that use only native
    ///    elements) are returned by move without touching a single
    ///    `Vec`. The previous "fused" implementation rebuilt every
    ///    `children` / `Fragment` vector via `into_iter().map().collect()`
    ///    on every render — one fresh `Vec` allocation per element per
    ///    render even when no `Tag::Component` existed anywhere in the
    ///    subtree.
    ///
    /// 2. **Component-free fast path.** When the pre-scan finds no
    ///    `Tag::Component`, the owned `VirtualNode` is returned
    ///    untouched: the deep tree's `Vec` allocations, `Tag` strings,
    ///    and attribute vectors are all reused by the renderer. Only
    ///    trees that actually contain components pay the rebuild cost,
    ///    and the pre-scan short-circuits at the first component found.
    ///
    /// # Arguments
    ///
    /// - `VirtualNode` - The owned virtual node to unwrap.
    ///
    /// # Returns
    ///
    /// - `VirtualNode` - The unwrapped virtual node with all components expanded.
    fn unwrap_component_owned(node: VirtualNode) -> VirtualNode {
        if !Self::subtree_has_component(&node) {
            return node;
        }
        Self::unwrap_component_owned_slow(node)
    }

    /// Read-only walk that answers "does this subtree contain any
    /// `Tag::Component` marker?" without allocating.
    ///
    /// `VirtualNode::Dynamic` is treated as a leaf: dynamic closures
    /// produce a fresh tree on every invocation, and that tree passes
    /// through `unwrap_component_owned` again inside the dynamic
    /// render path, so descending into the (not yet rendered) closure
    /// payload here is both impossible and unnecessary.
    ///
    /// # Arguments
    ///
    /// - `&VirtualNode` - The node to inspect.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when a component marker exists in the subtree.
    fn subtree_has_component(node: &VirtualNode) -> bool {
        match node {
            VirtualNode::Element {
                tag: Tag::Component(_),
                ..
            } => true,
            VirtualNode::Element { children, .. } => {
                children.iter().any(Self::subtree_has_component)
            }
            VirtualNode::Fragment(children) => children.iter().any(Self::subtree_has_component),
            _ => false,
        }
    }

    /// Allocating expansion pass, entered only when
    /// [`Self::subtree_has_component`] proved a component exists.
    ///
    /// Each `Tag::Component` wrapper is replaced by its single child
    /// (moved) or by a `Fragment` of its children; native-element
    /// children vectors are rebuilt exactly once along the descent.
    ///
    /// # Arguments
    ///
    /// - `VirtualNode` - The owned virtual node to expand.
    ///
    /// # Returns
    ///
    /// - `VirtualNode` - The expanded virtual node.
    fn unwrap_component_owned_slow(node: VirtualNode) -> VirtualNode {
        match node {
            VirtualNode::Element {
                tag: Tag::Component(_),
                children,
                ..
            } => {
                if children.len() == 1 {
                    match children.into_iter().next() {
                        Some(child) => Self::unwrap_component_owned(child),
                        None => VirtualNode::Fragment(Vec::new()),
                    }
                } else {
                    VirtualNode::Fragment(
                        children
                            .into_iter()
                            .map(Self::unwrap_component_owned)
                            .collect(),
                    )
                }
            }
            VirtualNode::Element {
                tag,
                attributes,
                children,
                key,
                props,
            } => VirtualNode::Element {
                tag,
                attributes,
                children: children
                    .into_iter()
                    .map(Self::unwrap_component_owned)
                    .collect(),
                key,
                props,
            },
            VirtualNode::Fragment(children) => VirtualNode::Fragment(
                children
                    .into_iter()
                    .map(Self::unwrap_component_owned)
                    .collect(),
            ),
            other => other,
        }
    }

    /// Performs a visual equality comparison between two virtual node trees.
    ///
    /// Unlike `PartialEq`, this method recursively unwraps `VirtualNode::Dynamic`
    /// nodes by rendering their inner content and comparing the visual output.
    /// This is used by the `skip_equal` optimization in `setup_dynamic_node`
    /// to avoid unnecessary DOM patches when the rendered output is unchanged.
    ///
    /// # Arguments
    ///
    /// - `&VirtualNode` - The old virtual node.
    /// - `&VirtualNode` - The new virtual node.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the two nodes produce the same visual output.
    fn visual_eq(old_node: &VirtualNode, new_node: &VirtualNode) -> bool {
        match (old_node, new_node) {
            (VirtualNode::Text(old_text), VirtualNode::Text(new_text)) => old_text == new_text,
            (
                VirtualNode::Element {
                    tag: old_tag,
                    attributes: old_attrs,
                    children: old_children,
                    ..
                },
                VirtualNode::Element {
                    tag: new_tag,
                    attributes: new_attrs,
                    children: new_children,
                    ..
                },
            ) => {
                old_tag == new_tag
                    && old_attrs.len() == new_attrs.len()
                    && old_attrs.iter().zip(new_attrs.iter()).all(
                        |(old_attr, new_attr): (&AttributeEntry, &AttributeEntry)| {
                            old_attr == new_attr
                        },
                    )
                    && old_children.len() == new_children.len()
                    && old_children.iter().zip(new_children.iter()).all(
                        |(old_child, new_child): (&VirtualNode, &VirtualNode)| {
                            Self::visual_eq(old_child, new_child)
                        },
                    )
            }
            (VirtualNode::Fragment(old_children), VirtualNode::Fragment(new_children)) => {
                old_children.len() == new_children.len()
                    && old_children.iter().zip(new_children.iter()).all(
                        |(old_child, new_child): (&VirtualNode, &VirtualNode)| {
                            Self::visual_eq(old_child, new_child)
                        },
                    )
            }
            (VirtualNode::Dynamic(_), VirtualNode::Dynamic(_)) => true,
            (VirtualNode::Empty, VirtualNode::Empty) => true,
            _ => false,
        }
    }

    /// Assigns a new `data-euv-dynamic-id` to a newly created DynamicNode placeholder.
    ///
    /// # Arguments
    ///
    /// - `&Element` - The placeholder DOM element.
    ///
    /// # Returns
    ///
    /// - `usize` - The assigned dynamic ID.
    fn assign_dynamic_id(placeholder: &Element) -> usize {
        let dynamic_id: usize = NEXT_EUV_DYNAMIC_ID.fetch_add(1, Ordering::Relaxed);
        let _: Result<(), JsValue> =
            placeholder.set_attribute(DATA_EUV_DYNAMIC_ID, &dynamic_id.to_string());
        dynamic_id
    }

    /// Recursively cleans up framework resources associated with a DOM subtree.
    ///
    /// Removes event handlers, dynamic node listeners, and signal listeners
    /// for the given element and all of its descendants.
    ///
    /// # Arguments
    ///
    /// - `&Element` - The DOM element to clean up.
    fn cleanup_subtree(element: &Element) {
        if let Some(euv_id_str) = element.get_attribute(DATA_EUV_ID)
            && let Ok(euv_id) = euv_id_str.parse::<usize>()
        {
            Registry::cleanup_element(euv_id);
        }
        if let Some(dynamic_id_str) = element.get_attribute(DATA_EUV_DYNAMIC_ID)
            && let Ok(dynamic_id) = dynamic_id_str.parse::<usize>()
        {
            Registry::cleanup_dynamic_node(dynamic_id);
        }
        if let Some(signal_addrs_str) = element.get_attribute(DATA_EUV_SIGNAL_ADDRS) {
            signal_addrs_str
                .split(CHAR_SIGNAL_ADDRS_SEPARATOR)
                .filter_map(|addr_str: &str| addr_str.parse::<usize>().ok())
                .for_each(Signal::<String>::clear_listeners);
        }
        let child_nodes: NodeList = element.child_nodes();
        let length: u32 = child_nodes.length();
        for child_index in 0..length {
            if let Some(child) = child_nodes.get(child_index)
                && let Some(child_element) = child.dyn_ref::<Element>()
            {
                Self::cleanup_subtree(child_element);
            }
        }
    }

    /// Registers an event handler for a DOM element.
    ///
    /// For non-bubbling events (load, error, loadstart, etc.), attaches the
    /// listener directly on the element since global delegation on `window`
    /// cannot capture these events. For all other events, uses global event
    /// delegation via `Registry::delegation`.
    ///
    /// # Arguments
    ///
    /// - `&Element` - The DOM element to attach the handler to.
    /// - `&NativeEventHandler` - The event handler to register.
    fn attach_event_listener(&self, element: &Element, handler: &NativeEventHandler) {
        let euv_id: usize = match element.get_attribute(DATA_EUV_ID) {
            Some(id_str) => id_str
                .parse::<usize>()
                .unwrap_or_else(|_error: ParseIntError| {
                    let new_id: usize = NEXT_EUV_ID.fetch_add(1, Ordering::Relaxed);
                    let _: Result<(), JsValue> =
                        element.set_attribute(DATA_EUV_ID, &new_id.to_string());
                    new_id
                }),
            None => {
                let new_id: usize = NEXT_EUV_ID.fetch_add(1, Ordering::Relaxed);
                let _: Result<(), JsValue> =
                    element.set_attribute(DATA_EUV_ID, &new_id.to_string());
                new_id
            }
        };
        let event_name: &'static str = handler.get_event_name();
        if Registry::is_non_bubbling(event_name) {
            let registry_ref: &mut HandlerRegistryMap = Registry::get_mut_handler_registry();
            if let Some(existing_entry) = registry_ref.get(&euv_id).and_then(
                |event_map: &HashMap<&'static str, HandlerEntry>| event_map.get(&event_name),
            ) {
                let slot: &mut HandlerSlot = unsafe { &mut **existing_entry };
                slot.set_handler(Some(handler.clone()));
            } else {
                let closure: Closure<dyn FnMut(Event)> =
                    Closure::wrap(Box::new(move |event: Event| {
                        if let Some(entry) = Registry::get_handler_registry().get(&euv_id).and_then(
                            |event_map: &HashMap<&'static str, HandlerEntry>| {
                                event_map.get(&event_name)
                            },
                        ) {
                            let slot: &HandlerSlot = unsafe { &**entry };
                            if let Some(active_handler) = slot.try_get_handler().as_ref().cloned() {
                                active_handler.handle(event);
                            }
                        }
                    }));
                let _: Result<(), JsValue> = element
                    .add_event_listener_with_callback(event_name, closure.as_ref().unchecked_ref());
                let listener_function: JsValue = closure.as_ref().clone();
                closure.forget();
                let handler_slot: HandlerEntry = Box::into_raw(Box::new(HandlerSlot::new(
                    Some(handler.clone()),
                    Some(listener_function),
                    Some(element.clone()),
                )));
                registry_ref
                    .entry(euv_id)
                    .or_default()
                    .insert(event_name, handler_slot);
            }
        } else {
            Registry::delegation(event_name);
            let registry_ref: &mut HandlerRegistryMap = Registry::get_mut_handler_registry();
            if let Some(existing_entry) = registry_ref.get(&euv_id).and_then(
                |event_map: &HashMap<&'static str, HandlerEntry>| event_map.get(&event_name),
            ) {
                let slot: &mut HandlerSlot = unsafe { &mut **existing_entry };
                slot.set_handler(Some(handler.clone()));
            } else {
                let handler_slot: HandlerEntry = Box::into_raw(Box::new(HandlerSlot::new(
                    Some(handler.clone()),
                    None,
                    None,
                )));
                registry_ref
                    .entry(euv_id)
                    .or_default()
                    .insert(event_name, handler_slot);
            }
        }
    }

    /// Checks whether a DOM node is currently connected to the document.
    ///
    /// Uses the `isConnected` JavaScript property to determine if the node
    /// is still attached to the live DOM tree.
    ///
    /// # Arguments
    ///
    /// - `&T` - A reference to any type that can be converted to `&Node`.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the node is connected to the document, `false` otherwise.
    fn is_node_connected<T>(node: &T) -> bool
    where
        T: AsRef<Node>,
    {
        node.as_ref().is_connected()
    }
}

/// Static method for mounting a virtual DOM tree into a real DOM element.
impl Mount {
    /// Mounts the given virtual DOM tree to a specific element matched by a CSS selector.
    ///
    /// Supported selector syntax:
    /// - `"#id"` — select by element ID
    /// - `".class"` — select by class name (uses the first match)
    /// - `"tag"` — select by tag name (uses the first match)
    ///
    /// # Arguments
    ///
    /// - `S: AsRef<str>` - A CSS selector string to locate the target element.
    /// - `FnOnce() -> VirtualNode + 'static` - A closure that returns the virtual DOM tree.
    pub(crate) fn setup<S, F>(selector: S, render_fn: F)
    where
        S: AsRef<str>,
        F: FnOnce() -> VirtualNode,
    {
        let selector: &str = selector.as_ref();
        let window: Window = match window() {
            Some(window_instance) => window_instance,
            None => return,
        };
        let document: Document = match window.document() {
            Some(document_instance) => document_instance,
            None => return,
        };
        let target: Element = if selector == BODY_TAG {
            match document.body() {
                Some(body) => body.into(),
                None => return,
            }
        } else if let Some(id) = selector.strip_prefix(ID_SELECTOR_PREFIX) {
            match document.get_element_by_id(id) {
                Some(element) => element,
                None => return,
            }
        } else if let Some(class) = selector.strip_prefix(CLASS_SELECTOR_PREFIX) {
            match document.get_elements_by_class_name(class).item(0) {
                Some(element) => element,
                None => return,
            }
        } else {
            match document.get_elements_by_tag_name(selector).item(0) {
                Some(element) => element,
                None => return,
            }
        };
        Renderer::new(target).render(render_fn());
    }
}
