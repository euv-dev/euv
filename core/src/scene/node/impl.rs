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
            hover_style: None,
            layout_box: LayoutBox::default(),
            last_constraint: None,
            dirty: DirtyFlags::all(),
            parent: None,
            children: Vec::new(),
            key: None,
            interaction: InteractionState::new(focusable, false, false, false),
            handlers: EventHandlers::default(),
            scope: None,
            canvas_painter: None,
            a11y: None,
            virtual_scroll: None,
            lifecycles: Vec::new(),
        }
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
    pub fn parent_id(&self) -> Option<NodeId> {
        let Self { parent, .. } = self;
        *parent
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
    pub fn hover_paint(&self) -> Option<PaintStyle> {
        let Self { hover_style, .. } = self;
        *hover_style
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
    pub fn owned_scope(&self) -> Option<Scope> {
        let Self { scope, .. } = self;
        *scope
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
    pub fn canvas_callback(&self) -> Option<CanvasDrawCallback> {
        let Self { canvas_painter, .. } = self;
        canvas_painter.clone()
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
    pub fn a11y_props(&self) -> Option<A11yProps> {
        let Self { a11y, .. } = self;
        a11y.clone()
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
    pub fn virtual_scroll_metrics(&self) -> Option<(f32, f32)> {
        let Self { virtual_scroll, .. } = self;
        *virtual_scroll
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
    pub fn add_lifecycle(&mut self, hook: Rc<dyn Lifecycle>) {
        self.get_mut_lifecycles().push(hook);
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
    pub fn lifecycle_observers(&self) -> Vec<Rc<dyn Lifecycle>> {
        let Self { lifecycles, .. } = self;
        lifecycles.clone()
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
    pub(crate) fn take_lifecycles(&mut self) -> Vec<Rc<dyn Lifecycle>> {
        take(self.get_mut_lifecycles())
    }
}
