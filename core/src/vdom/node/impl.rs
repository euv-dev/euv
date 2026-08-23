use super::*;

/// Visual equality comparison for text nodes.
///
/// Only compares the text content; the backing signal is not considered
/// because it does not affect visual output.
impl PartialEq for TextNode {
    fn eq(&self, other: &Self) -> bool {
        self.get_content() == other.get_content()
    }
}

/// Clones a `VirtualNode<T>` by deep-copying all fields.
impl<T: Clone> Clone for VirtualNode<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Element {
                tag,
                attributes,
                children,
                key,
                props,
            } => Self::Element {
                tag: tag.clone(),
                attributes: attributes.clone(),
                children: children.clone(),
                key: key.clone(),
                props: props.clone(),
            },
            Self::Text(text_node) => Self::Text(text_node.clone()),
            Self::Fragment(children) => Self::Fragment(children.clone()),
            Self::Dynamic(dynamic_node) => Self::Dynamic(dynamic_node.clone()),
            Self::Empty => Self::Empty,
        }
    }
}

/// Debug formatting for `VirtualNode<T>`.
///
/// Skips `Dynamic` inner details and `props` for brevity.
impl<T: std::fmt::Debug> std::fmt::Debug for VirtualNode<T> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Element {
                tag,
                attributes,
                children,
                key,
                props,
            } => formatter
                .debug_struct("Element")
                .field("tag", tag)
                .field("attributes", attributes)
                .field("children", children)
                .field("key", key)
                .field("props", props)
                .finish(),
            Self::Text(text_node) => formatter.debug_tuple("Text").field(text_node).finish(),
            Self::Fragment(children) => formatter.debug_tuple("Fragment").field(children).finish(),
            Self::Dynamic(_) => formatter.debug_tuple("Dynamic").finish(),
            Self::Empty => formatter.debug_tuple("Empty").finish(),
        }
    }
}

/// Default implementation returns `VirtualNode::Empty`.
impl<T> Default for VirtualNode<T> {
    fn default() -> Self {
        Self::Empty
    }
}

/// Visual equality comparison for virtual DOM nodes.
///
/// Used by DynamicNode re-rendering to skip unnecessary DOM patches when
/// the rendered output has not changed. Event attributes are always
/// considered equal because re-binding event listeners is handled
/// separately by the handler registry and does not affect visual output.
/// Dynamic nodes manage their own subtree re-rendering, so two Dynamic
/// variants are always considered equal — the inner renderer handles
/// patching when the dynamic content actually changes.
impl<T: PartialEq> PartialEq for VirtualNode<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (VirtualNode::Text(old_text), VirtualNode::Text(new_text)) => old_text == new_text,
            (
                VirtualNode::Element {
                    tag: old_tag,
                    attributes: old_attrs,
                    children: old_children,
                    props: old_props,
                    ..
                },
                VirtualNode::Element {
                    tag: new_tag,
                    attributes: new_attrs,
                    children: new_children,
                    props: new_props,
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
                            old_child == new_child
                        },
                    )
                    && old_props == new_props
            }
            (VirtualNode::Fragment(old_children), VirtualNode::Fragment(new_children)) => {
                old_children.len() == new_children.len()
                    && old_children.iter().zip(new_children.iter()).all(
                        |(old_child, new_child): (&VirtualNode, &VirtualNode)| {
                            old_child == new_child
                        },
                    )
            }
            (VirtualNode::Dynamic(_), VirtualNode::Dynamic(_)) => false,
            (VirtualNode::Empty, VirtualNode::Empty) => true,
            _ => false,
        }
    }
}

/// Provides a default empty dynamic node with a no-op render function.
impl Default for DynamicNode {
    fn default() -> Self {
        let render_fn_inner: Rc<UnsafeCell<RenderFnInner>> = Rc::new(UnsafeCell::new(
            RenderFnInner::new(Box::new(|_: &mut HookContext| VirtualNode::Empty)),
        ));
        Self::new(render_fn_inner, HookContext::default())
    }
}

/// Implementation of dynamic node accessor methods.
impl DynamicNode {
    /// Invokes the render closure and returns the produced virtual node.
    ///
    /// # Safety
    ///
    /// Must only be called from the main thread. Guaranteed in WASM
    /// single-threaded context. No concurrent access is possible.
    ///
    /// # Arguments
    ///
    /// - `&mut HookContext` - The hook context to pass to the render closure.
    ///
    /// # Returns
    ///
    /// - `VirtualNode` - The virtual node produced by the render closure.
    pub fn render(&self, hook_context: &mut HookContext) -> VirtualNode {
        let inner: &mut RenderFnInner = unsafe { &mut *self.get_render_fn().get() };
        (inner.get_mut_render_fn())(hook_context)
    }
}

/// Implementation of virtual node construction and property extraction.
impl<T> VirtualNode<T> {
    /// Returns the tag name if this is an element or component node.
    ///
    /// # Returns
    ///
    /// - `Option<String>` - The tag name, or `None` if not an element.
    pub fn try_get_tag_name(&self) -> Option<String> {
        match self {
            Self::Element { tag, .. } => match tag {
                Tag::Element(name) => Some(name.clone()),
                Tag::Component(name) => Some(name.clone()),
                // Portals do not contribute a tag name to the
                // declared position in the DOM tree — their content
                // is rendered into a separate target, and the
                // marker is an internal implementation detail.
                // Returning `None` here keeps callers that use
                // `try_get_tag_name` for "what tag is this?" away
                // from the portal sentinel.
                Tag::Portal(_) => None,
            },
            _ => None,
        }
    }

    /// Returns a reference to the children of this node, if it has any.
    ///
    /// Returns `Some` for `Element` and `Fragment` variants, `None` otherwise.
    ///
    /// # Returns
    ///
    /// - `Option<&Vec<VirtualNode>>` - The children, or `None`.
    pub fn try_get_children(&self) -> Option<&Vec<VirtualNode>> {
        match self {
            Self::Element { children, .. } => Some(children),
            Self::Fragment(children) => Some(children),
            _ => None,
        }
    }

    /// Returns `true` if this node has non-empty children.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether this node has children.
    pub fn has_children(&self) -> bool {
        self.try_get_children()
            .is_some_and(|children: &Vec<VirtualNode>| !children.is_empty())
    }

    /// Clones the props of this node.
    ///
    /// # Returns
    ///
    /// - `Option<T>` - The cloned props, or `None` if this node has no props.
    pub fn try_get_props(&self) -> Option<T>
    where
        T: Clone,
    {
        match self {
            Self::Element { props, .. } => props.as_deref().cloned(),
            _ => None,
        }
    }

    /// Returns the children of this node as a virtual node.
    ///
    /// Returns `VirtualNode::Empty` when there are no children, a single child
    /// when there is exactly one, or `VirtualNode::Fragment` when there are
    /// multiple children.
    ///
    /// # Returns
    ///
    /// - `Option<VirtualNode>` - The children as a virtual node.
    pub fn try_get_child_node(&self) -> Option<VirtualNode> {
        match self.try_get_children() {
            Some(children) => match children.len() {
                0 => None,
                1 => children.first().cloned(),
                _ => Some(VirtualNode::Fragment(children.clone())),
            },
            None => None,
        }
    }

    /// Extends this node's attribute list with the given entries, then
    /// returns the node. If the node is not an `Element` variant, the
    /// entries are dropped and the node is returned unchanged.
    ///
    /// Used by the `html!` macro to splice `class` / `style` / event
    /// handler attributes onto a component-returned node without forcing
    /// a `let mut` binding in the generated code.
    ///
    /// # Arguments
    ///
    /// - `I: IntoIterator<Item = AttributeEntry>` - The extra entries to append.
    ///
    /// # Returns
    ///
    /// - `Self` - The node with extended attributes (or unchanged).
    pub fn extend_attributes<I>(self, extra: I) -> Self
    where
        I: IntoIterator<Item = AttributeEntry>,
    {
        match self {
            Self::Element {
                tag,
                attributes,
                children,
                key,
                props,
            } => {
                let mut attrs: Vec<AttributeEntry> = attributes;
                attrs.extend(extra);
                Self::Element {
                    tag,
                    attributes: attrs,
                    children,
                    key,
                    props,
                }
            }
            other => other,
        }
    }

    /// Returns the children of this node as a virtual node.
    ///
    /// Returns `VirtualNode::Empty` when there are no children, a single child
    /// when there is exactly one, or `VirtualNode::Fragment` when there are
    /// multiple children.
    ///
    /// # Returns
    ///
    /// - `VirtualNode` - The children as a virtual node.
    pub fn get_child_node(&self) -> VirtualNode {
        self.try_get_child_node().unwrap_or_default()
    }
}

/// Implementation of virtual node construction for `VirtualNode<()>`.
impl VirtualNode<()> {
    /// Creates a dynamic node with the given render function.
    ///
    /// # Arguments
    ///
    /// - `F: FnMut(&mut HookContext) -> Self + 'static` - The render function.
    ///
    /// # Returns
    ///
    /// - `Self` - The dynamic node.
    pub fn create_dynamic<F>(render_fn: F) -> Self
    where
        F: FnMut(&mut HookContext) -> Self + 'static,
    {
        let hook_context: HookContext = HookContext::default();
        let inner: Rc<UnsafeCell<RenderFnInner>> =
            Rc::new(UnsafeCell::new(RenderFnInner::new(Box::new(render_fn))));
        Self::Dynamic(DynamicNode::new(inner, hook_context))
    }
}
