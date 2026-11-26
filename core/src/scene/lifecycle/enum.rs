/// Retention policy controlling how a routed page behaves when the user navigates away.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum PageRetention {
    /// Dispose the page subtree, reactive scope, and lifecycle resources on leave; recreate fresh on next visit.
    Destroy,
    /// Retain the scene subtree hidden (`Display::None`), freeze all reactive effects and animations while tracking cross-page dirty state, and coalesce updates atomically on resume.
    #[default]
    Freeze,
    /// Hide the page visually (`Display::None`) while keeping its reactive effects running in the background.
    Alive,
}

/// Lifecycle phase of a mounted `Component` or routed page managed by the framework.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum LifecyclePhase {
    /// Component instance created, not yet mounted into the scene tree.
    #[default]
    Created,
    /// Component rendered and attached to the retained scene graph.
    Mounted,
    /// Component or page is actively visible and executing reactive logic.
    Active,
    /// Component or page is hidden with reactive effects suspended (`PageRetention::Freeze`).
    Frozen,
    /// Component detached from the scene graph prior to final disposal.
    Unmounted,
    /// Component scope disposed and resources released.
    Destroyed,
}
