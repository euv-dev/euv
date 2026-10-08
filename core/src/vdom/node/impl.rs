use super::*;

/// Visual equality comparison for text nodes.
///
/// Only compares the text content; the backing signal is not considered
/// because it does not affect visual output.
///
/// OPT 29: `content` is `Cow<'static, str>`; compare the dereffed
/// string views (cheap for both `Borrowed` and `Owned`).
impl PartialEq for TextNode {
    /// Returns `true` when `self` and `other` are equivalent by the [`PartialEq`] contract.
    ///
    /// # Arguments
    ///
    /// - `&Self` - The other value to compare against `self`.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when `self` and `other` are equivalent by the trait contract.
    fn eq(&self, other: &Self) -> bool {
        self.get_content().as_ref() == other.get_content().as_ref()
    }
}

/// Clones a `VirtualNode<T>` by deep-copying all fields.
impl<T: Clone> Clone for VirtualNode<T> {
    /// Clones the [`VirtualNode`] by reusing shared, cheap-to-clone state where possible.
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
impl<T: Debug> Debug for VirtualNode<T> {
    /// Formats the [`VirtualNode`] via the supplied formatter.
    ///
    /// # Arguments
    ///
    /// - `&mut Formatter<'_>` - The formatter receiving the formatted output.
    ///
    /// # Returns
    ///
    /// - `fmt::Result` - Result of the formatting operation.
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Element {
                tag,
                attributes,
                children,
                key,
                props,
            } => formatter
                .debug_struct(DEBUG_NAME_ELEMENT)
                .field(DEBUG_FIELD_TAG, tag)
                .field(DEBUG_FIELD_ATTRIBUTES, attributes)
                .field(DEBUG_FIELD_CHILDREN, children)
                .field(DEBUG_FIELD_KEY, key)
                .field(DEBUG_FIELD_PROPS, props)
                .finish(),
            Self::Text(text_node) => formatter
                .debug_tuple(DEBUG_NAME_TEXT)
                .field(text_node)
                .finish(),
            Self::Fragment(children) => formatter
                .debug_tuple(DEBUG_NAME_FRAGMENT)
                .field(children)
                .finish(),
            Self::Dynamic(_) => formatter.debug_tuple(DEBUG_NAME_DYNAMIC).finish(),
            Self::Empty => formatter.debug_tuple(DEBUG_NAME_EMPTY).finish(),
        }
    }
}

/// Default implementation returns `VirtualNode::Empty`.
impl<T> Default for VirtualNode<T> {
    /// Constructs a default [`VirtualNode`] value.
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
    /// Returns `true` when `self` and `other` are equivalent by the [`PartialEq`] contract.
    ///
    /// # Arguments
    ///
    /// - `&Self` - The other value to compare against `self`.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when `self` and `other` are equivalent by the trait contract.
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
    /// Constructs a default [`DynamicNode`] value.
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
    pub(crate) fn render(&self, hook_context: &mut HookContext) -> VirtualNode {
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
                // OPT 2: `Cow::to_string()` allocates only for the
                // `Owned` branch. The common `Borrowed("div")` path
                // performs one string slice clone (no heap).
                Tag::Element(name) => Some(name.to_string()),
                Tag::Component(name) => Some(name.to_string()),
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

    /// Returns the children of this node as a borrowed slice.
    ///
    /// Returns an empty slice for `Empty`, the children of `Element`
    /// and `Fragment` variants, and an empty slice for `Text` /
    /// `Dynamic`. Zero-copy; callers can iterate without cloning.
    ///
    /// # Returns
    ///
    /// - `&[VirtualNode]` - The children, or an empty slice.
    pub fn get_children(&self) -> &[VirtualNode] {
        match self {
            Self::Element { children, .. } => children.as_slice(),
            Self::Fragment(children) => children.as_slice(),
            _ => &[],
        }
    }

    /// Returns the first child of this node as a borrowed reference,
    /// if any.
    ///
    /// Returns `None` when there are no children; otherwise returns
    /// a reference to the first child. Zero-copy; replaces the
    /// previous `get_child_node` helper that cloned the entire
    /// children subtree per render.
    ///
    /// # Returns
    ///
    /// - `Option<&VirtualNode>` - The first child, or `None`.
    pub fn get_first_child(&self) -> Option<&VirtualNode> {
        self.get_children().first()
    }

    /// Returns `true` if this node has non-empty children.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether this node has children.
    pub fn has_children(&self) -> bool {
        !self.get_children().is_empty()
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

    /// Returns the children of this node as a borrowed slice.
    ///
    /// Equivalent to [`Self::get_children`] but exposes the raw
    /// `Option<&[VirtualNode]>` shape for callers that want to
    /// distinguish "no children" from "empty children" (Element
    /// without children vs. Text/Dynamic/Empty).
    ///
    /// # Returns
    ///
    /// - `Option<&[VirtualNode]>` - The children, or `None`.
    pub fn try_get_children(&self) -> Option<&[VirtualNode]> {
        match self {
            Self::Element { children, .. } => Some(children.as_slice()),
            Self::Fragment(children) => Some(children.as_slice()),
            _ => None,
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
    /// - `I` - The extra entries to append.
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

    /// Returns the diffing key of this node, if it has one.
    ///
    /// Recognizes keys on `Element` variants. Other variants
    /// (`Text`, `Fragment`, `Dynamic`, `Empty`) do not have keys.
    /// This matches the renderer's `get_node_key` semantics
    /// in `core/src/renderer/render/impl.rs`.
    ///
    /// # Returns
    ///
    /// - `Option<&str>` - The key, or `None` if this node has no key
    ///   or is not an `Element` variant.
    pub fn key(&self) -> Option<&str> {
        match self {
            Self::Element { key, .. } => key.as_deref(),
            _ => None,
        }
    }

    /// Returns `true` if this node has a non-`None` diffing key.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if `key()` returns `Some`, `false` otherwise.
    pub fn has_key(&self) -> bool {
        self.key().is_some()
    }
}

/// Implementation of virtual node construction for `VirtualNode<()>`.
impl VirtualNode<()> {
    /// Creates a dynamic node with the given render function.
    ///
    /// # Arguments
    ///
    /// - `F` - The render function.
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
