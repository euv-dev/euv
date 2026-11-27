use super::*;

/// Standalone lifecycle hook contract implemented by components and routed pages.
///
/// Business code overrides only the hooks it needs; every method provides a zero-cost no-op default.
pub trait Lifecycle {
    /// Declares how the framework retains this page or component when navigating away.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component or page instance reference.
    ///
    /// # Returns
    ///
    /// - `PageRetention` - Retention strategy (`Destroy`, `Freeze`, or `Alive`).
    fn retention_policy(&self) -> PageRetention {
        PageRetention::Freeze
    }

    /// Invoked once immediately before the component's initial `render` pass executes.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component or page instance reference.
    fn on_create(&self) {}

    /// Invoked once immediately after `render` attaches the root `NodeId` to the scene tree.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component or page instance reference.
    /// - `NodeId` - Mounted root scene node handle.
    fn on_mount(&self, _node: NodeId) {}

    /// Invoked when external props, theme tokens, or cross-page state trigger a component update.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component or page instance reference.
    /// - `NodeId` - Active root scene node handle.
    fn on_update(&self, _node: NodeId) {}

    /// Invoked whenever this component or routed page becomes the active visible view.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component or page instance reference.
    /// - `NodeId` - Active root scene node handle.
    fn on_activate(&self, _node: NodeId) {}

    /// Invoked when the router or parent suspends this subtree under `PageRetention::Freeze`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component or page instance reference.
    /// - `NodeId` - Frozen root scene node handle.
    fn on_freeze(&self, _node: NodeId) {}

    /// Invoked when a previously frozen subtree resumes and coalesced cross-page updates flush.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component or page instance reference.
    /// - `NodeId` - Resumed root scene node handle.
    fn on_resume(&self, _node: NodeId) {}

    /// Invoked immediately before the subtree is detached and its reactive scope is disposed.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component or page instance reference.
    /// - `NodeId` - Unmounting root scene node handle.
    fn on_unmount(&self, _node: NodeId) {}

    /// Invoked after the component's reactive scope and child nodes have been fully destroyed.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component or page instance reference.
    fn on_destroy(&self) {}

    /// Invoked when an error boundary or runtime fault surfaces inside this component subtree.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component or page instance reference.
    /// - `&str` - Diagnostic error message.
    fn on_error(&self, _error: &str) {}
}

/// Core declarative component trait unifying `Lifecycle` hooks with framework-managed `Context` rendering.
pub trait Component: Lifecycle + Sized + 'static {
    /// Renders this component into the ambient framework-managed `Context` and returns its root `NodeId`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Immutable component instance reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Root scene node handle produced by rendering.
    fn render(&self) -> NodeId;

    /// Mounts this component through the complete framework lifecycle (`on_create` -> `render` -> `on_mount`) and registers its `Lifecycle` observer on the mounted node.
    ///
    /// # Arguments
    ///
    /// - `Self` - Owned component instance.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted root scene node handle.
    fn mount(self) -> NodeId {
        let instance: Rc<Self> = Rc::new(self);
        instance.on_create();
        let node: NodeId = instance.render();
        let hooks: Rc<dyn Lifecycle> = Rc::clone(&instance) as Rc<dyn Lifecycle>;
        let ctx: Context = current_context();
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                tree.attach_lifecycle(node, hooks);
            },
            (),
        );
        instance.on_mount(node);
        node
    }
}

/// Initialization contract enabling declarative named-attribute component construction inside `view! { MyComp { prop: value } }`.
pub trait ComponentProps: Sized {
    /// Constructs a component descriptor with default property values ready for fluent named-prop chaining.
    ///
    /// # Returns
    ///
    /// - `Self` - Component instance initialized with its declared prop defaults.
    fn init_props() -> Self;
}
