use super::*;

/// Safe optional accessors for `SceneNode`.
impl SceneNode {
    /// Creates an initialized live `SceneNode` with `generation` and `kind`.
    ///
    /// # Arguments
    ///
    /// - `u32` - Slot generation tag.
    /// - `NodeKind` - Semantic node kind.
    /// - `Style` - Initial compiled style.
    ///
    /// # Returns
    ///
    /// - `Self` - Constructed scene node marked dirty for initial layout and paint.
    #[must_use]
    pub fn new(generation: u32, kind: NodeKind, style: Style) -> Self {
        let focusable: bool = matches!(kind, NodeKind::Input(_));
        Self {
            generation,
            alive: true,
            kind,
            style,
            layout_box: LayoutBox::default(),
            last_constraint: None,
            dirty: DirtyFlags::all(),
            parent: None,
            children: Vec::new(),
            interaction: InteractionState::new(focusable, false, false, false),
            extras: None,
        }
    }

    /// Returns whether this node has lazily allocated cold optional metadata (`NodeExtras`).
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scene node reference.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when cold metadata has been allocated.
    #[must_use]
    #[inline(always)]
    pub(crate) fn has_extras(&self) -> bool {
        let Self { extras, .. } = self;
        extras.is_some()
    }

    /// Lazily allocates and returns a mutable reference to `NodeExtras`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene node.
    ///
    /// # Returns
    ///
    /// - `&mut NodeExtras` - Mutable reference to boxed cold metadata.
    #[inline]
    fn extras_mut(&mut self) -> &mut NodeExtras {
        let Self { extras, .. } = self;
        extras.get_or_insert_with(Box::default)
    }

    /// Drops any lazily allocated cold metadata on node disposal.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene node.
    #[inline(always)]
    pub(crate) fn clear_extras(&mut self) {
        let Self { extras, .. } = self;
        *extras = None;
    }

    /// Returns a shared reference to the compiled `Style` without copying by value.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scene node reference.
    ///
    /// # Returns
    ///
    /// - `&Style` - Borrowed compiled style reference.
    #[must_use]
    #[inline(always)]
    pub fn style_ref(&self) -> &Style {
        let Self { style, .. } = self;
        style
    }

    /// Returns the optional parent `NodeId`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scene node reference.
    ///
    /// # Returns
    ///
    /// - `Option<NodeId>` - Parent node handle if attached.
    #[must_use]
    #[inline(always)]
    pub fn parent_id(&self) -> Option<NodeId> {
        let Self { parent, .. } = self;
        *parent
    }

    /// Returns a shared reference to the node's `EventHandlers` without allocating `NodeExtras`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scene node reference.
    ///
    /// # Returns
    ///
    /// - `&EventHandlers` - Borrowed event handler table (shared empty table when unallocated).
    #[must_use]
    #[inline(always)]
    pub fn get_handlers(&self) -> &EventHandlers {
        let Self { extras, .. } = self;
        match extras {
            Some(ext) => ext.get_handlers(),
            None => EMPTY_EVENT_HANDLERS_REF,
        }
    }

    /// Returns a mutable reference to the node's `EventHandlers`, allocating `NodeExtras` lazily.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene node.
    ///
    /// # Returns
    ///
    /// - `&mut EventHandlers` - Mutable event handler table.
    #[inline]
    pub fn get_mut_handlers(&mut self) -> &mut EventHandlers {
        self.extras_mut().get_mut_handlers()
    }

    /// Replaces the node's `EventHandlers`, avoiding allocation when setting an empty table on a plain node.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene node.
    /// - `EventHandlers` - Replacement event handler collection.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - Mutable scene node for chaining.
    #[inline]
    pub fn set_handlers(&mut self, value: EventHandlers) -> &mut Self {
        if !self.has_extras() && value.is_empty() {
            return self;
        }
        let _: &mut NodeExtras = self.extras_mut().set_handlers(value);
        self
    }

    /// Returns the optional hover paint style override.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scene node reference.
    ///
    /// # Returns
    ///
    /// - `Option<PaintStyle>` - Hover paint style if registered.
    #[must_use]
    #[inline(always)]
    pub fn hover_paint(&self) -> Option<PaintStyle> {
        let Self { extras, .. } = self;
        extras
            .as_deref()
            .and_then(|ext: &NodeExtras| ext.hover_style)
    }

    /// Sets the optional hover paint style override.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene node.
    /// - `Option<PaintStyle>` - Hover paint override or `None` to clear.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - Mutable scene node for chaining.
    #[inline]
    pub fn set_hover_style(&mut self, value: Option<PaintStyle>) -> &mut Self {
        if !self.has_extras() && value.is_none() {
            return self;
        }
        let _: &mut NodeExtras = self.extras_mut().set_hover_style(value);
        self
    }

    /// Returns a mutable reference to the optional hover paint style slot.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene node.
    ///
    /// # Returns
    ///
    /// - `&mut Option<PaintStyle>` - Mutable hover paint override slot.
    #[inline]
    pub fn get_mut_hover_style(&mut self) -> &mut Option<PaintStyle> {
        self.extras_mut().get_mut_hover_style()
    }

    /// Sets the optional reconciliation key for keyed list items.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene node.
    /// - `Option<String>` - Reconciliation key or `None` to clear.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - Mutable scene node for chaining.
    #[inline]
    pub fn set_key(&mut self, value: Option<String>) -> &mut Self {
        if !self.has_extras() && value.is_none() {
            return self;
        }
        let _: &mut NodeExtras = self.extras_mut().set_key(value);
        self
    }

    /// Returns a mutable reference to the optional reconciliation key slot.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene node.
    ///
    /// # Returns
    ///
    /// - `&mut Option<String>` - Mutable reconciliation key slot.
    #[inline]
    pub fn get_mut_key(&mut self) -> &mut Option<String> {
        self.extras_mut().get_mut_key()
    }

    /// Returns the optional reactive scope owned by this node.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scene node reference.
    ///
    /// # Returns
    ///
    /// - `Option<Scope>` - Attached reactive scope if present.
    #[must_use]
    #[inline(always)]
    pub fn owned_scope(&self) -> Option<Scope> {
        let Self { extras, .. } = self;
        extras.as_deref().and_then(|ext: &NodeExtras| ext.scope)
    }

    /// Sets the optional reactive ownership scope on this node.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene node.
    /// - `Option<Scope>` - Scope handle or `None` to clear.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - Mutable scene node for chaining.
    #[inline]
    pub fn set_scope(&mut self, value: Option<Scope>) -> &mut Self {
        if !self.has_extras() && value.is_none() {
            return self;
        }
        let _: &mut NodeExtras = self.extras_mut().set_scope(value);
        self
    }

    /// Returns a mutable reference to the optional reactive ownership scope slot.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene node.
    ///
    /// # Returns
    ///
    /// - `&mut Option<Scope>` - Mutable scope slot.
    #[inline]
    pub fn get_mut_scope(&mut self) -> &mut Option<Scope> {
        self.extras_mut().get_mut_scope()
    }

    /// Returns a cloned handle to the custom canvas painter if present.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scene node reference.
    ///
    /// # Returns
    ///
    /// - `Option<CanvasDrawCallback>` - Custom canvas callback if set.
    #[must_use]
    #[inline(always)]
    pub fn canvas_callback(&self) -> Option<CanvasDrawCallback> {
        let Self { extras, .. } = self;
        extras
            .as_deref()
            .and_then(|ext: &NodeExtras| ext.canvas_painter.clone())
    }

    /// Sets the optional custom 2D canvas draw callback on this node.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene node.
    /// - `Option<CanvasDrawCallback>` - Custom canvas painter or `None` to clear.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - Mutable scene node for chaining.
    #[inline]
    pub fn set_canvas_painter(&mut self, value: Option<CanvasDrawCallback>) -> &mut Self {
        if !self.has_extras() && value.is_none() {
            return self;
        }
        let _: &mut NodeExtras = self.extras_mut().set_canvas_painter(value);
        self
    }

    /// Returns a mutable reference to the optional custom canvas painter slot.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene node.
    ///
    /// # Returns
    ///
    /// - `&mut Option<CanvasDrawCallback>` - Mutable canvas painter slot.
    #[inline]
    pub fn get_mut_canvas_painter(&mut self) -> &mut Option<CanvasDrawCallback> {
        self.extras_mut().get_mut_canvas_painter()
    }

    /// Borrows the optional explicit semantic accessibility metadata attached to this node without cloning.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scene node reference.
    ///
    /// # Returns
    ///
    /// - `Option<&A11yProps>` - Borrowed accessibility metadata if explicitly configured (`None` in O(1) when `extras` is absent).
    #[must_use]
    #[inline(always)]
    pub fn a11y_props_ref(&self) -> Option<&A11yProps> {
        let Self { extras, .. } = self;
        extras
            .as_deref()
            .and_then(|ext: &NodeExtras| ext.a11y.as_ref())
    }

    /// Returns the optional explicit semantic accessibility metadata attached to this node.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scene node reference.
    ///
    /// # Returns
    ///
    /// - `Option<A11yProps>` - Cloned accessibility metadata if explicitly configured.
    #[must_use]
    #[inline(always)]
    pub fn a11y_props(&self) -> Option<A11yProps> {
        self.a11y_props_ref().cloned()
    }

    /// Sets the optional semantic accessibility metadata on this node.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene node.
    /// - `Option<A11yProps>` - Accessibility metadata or `None` to clear.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - Mutable scene node for chaining.
    #[inline]
    pub fn set_a11y(&mut self, value: Option<A11yProps>) -> &mut Self {
        if !self.has_extras() && value.is_none() {
            return self;
        }
        let _: &mut NodeExtras = self.extras_mut().set_a11y(value);
        self
    }

    /// Returns a mutable reference to the optional accessibility metadata slot.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene node.
    ///
    /// # Returns
    ///
    /// - `&mut Option<A11yProps>` - Mutable accessibility metadata slot.
    #[inline]
    pub fn get_mut_a11y(&mut self) -> &mut Option<A11yProps> {
        self.extras_mut().get_mut_a11y()
    }

    /// Returns the optional virtualized scroll metrics `(scroll_y, total_content_height)`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scene node reference.
    ///
    /// # Returns
    ///
    /// - `Option<(f32, f32)>` - Virtual scroll position and full content height when configured.
    #[must_use]
    #[inline(always)]
    pub fn virtual_scroll_metrics(&self) -> Option<(f32, f32)> {
        let Self { extras, .. } = self;
        extras
            .as_deref()
            .and_then(|ext: &NodeExtras| ext.virtual_scroll)
    }

    /// Sets the optional virtualized scroll metrics `(scroll_y, total_content_height)`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene node.
    /// - `Option<(f32, f32)>` - Virtual scroll metrics or `None` to clear.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - Mutable scene node for chaining.
    #[inline]
    pub fn set_virtual_scroll(&mut self, value: Option<(f32, f32)>) -> &mut Self {
        if !self.has_extras() && value.is_none() {
            return self;
        }
        let _: &mut NodeExtras = self.extras_mut().set_virtual_scroll(value);
        self
    }

    /// Returns a mutable reference to the optional virtualized scroll metrics slot.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene node.
    ///
    /// # Returns
    ///
    /// - `&mut Option<(f32, f32)>` - Mutable virtual scroll metrics slot.
    #[inline]
    pub fn get_mut_virtual_scroll(&mut self) -> &mut Option<(f32, f32)> {
        self.extras_mut().get_mut_virtual_scroll()
    }

    /// Returns the cached layout constraint if previously laid out.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scene node reference.
    ///
    /// # Returns
    ///
    /// - `Option<AvailableSpace>` - Last layout constraint.
    #[must_use]
    #[inline(always)]
    pub(crate) fn cached_constraint(&self) -> Option<AvailableSpace> {
        let Self {
            last_constraint, ..
        } = self;
        *last_constraint
    }

    /// Registers a `Lifecycle` observer on this scene node.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene node.
    /// - `Rc<dyn Lifecycle>` - Shared lifecycle hook observer.
    #[inline]
    pub fn add_lifecycle(&mut self, hook: Rc<dyn Lifecycle>) {
        self.extras_mut().get_mut_lifecycles().push(hook);
    }

    /// Returns a mutable reference to the lifecycle observer vector.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene node.
    ///
    /// # Returns
    ///
    /// - `&mut Vec<Rc<dyn Lifecycle>>` - Mutable lifecycle observer list.
    #[inline]
    pub fn get_mut_lifecycles(&mut self) -> &mut Vec<Rc<dyn Lifecycle>> {
        self.extras_mut().get_mut_lifecycles()
    }

    /// Replaces the lifecycle observer vector on this node.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene node.
    /// - `Vec<Rc<dyn Lifecycle>>` - Replacement lifecycle observers.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - Mutable scene node for chaining.
    #[inline]
    pub fn set_lifecycles(&mut self, value: Vec<Rc<dyn Lifecycle>>) -> &mut Self {
        if !self.has_extras() && value.is_empty() {
            return self;
        }
        let _: &mut NodeExtras = self.extras_mut().set_lifecycles(value);
        self
    }

    /// Returns cloned handles to all `Lifecycle` observers attached to this node.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scene node reference.
    ///
    /// # Returns
    ///
    /// - `Vec<Rc<dyn Lifecycle>>` - Attached lifecycle observers.
    #[must_use]
    #[inline]
    pub fn lifecycle_observers(&self) -> Vec<Rc<dyn Lifecycle>> {
        let Self { extras, .. } = self;
        match extras.as_deref() {
            Some(ext) if !ext.lifecycles.is_empty() => ext.lifecycles.clone(),
            _ => Vec::new(),
        }
    }

    /// Removes and returns all `Lifecycle` observers attached to this node during disposal.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene node.
    ///
    /// # Returns
    ///
    /// - `Vec<Rc<dyn Lifecycle>>` - Drained lifecycle observers.
    #[inline]
    pub(crate) fn take_lifecycles(&mut self) -> Vec<Rc<dyn Lifecycle>> {
        let Self { extras, .. } = self;
        match extras.as_deref_mut() {
            Some(ext) if !ext.lifecycles.is_empty() => take(ext.get_mut_lifecycles()),
            _ => Vec::new(),
        }
    }
}
