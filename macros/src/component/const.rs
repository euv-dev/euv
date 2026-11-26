/// Attribute argument key for configuring `PageRetention` in `#[component(retention = Freeze)]`.
pub(crate) const COMP_ATTR_RETENTION: &str = "retention";
/// Method names routed to `impl Lifecycle for T` by `#[component]`.
pub(crate) const LIFECYCLE_METHOD_NAMES: &[&str] = &[
    "retention_policy",
    "on_create",
    "on_mount",
    "on_update",
    "on_activate",
    "on_freeze",
    "on_resume",
    "on_unmount",
    "on_destroy",
    "on_error",
];
/// Component render method name routed to `impl Component for T`.
pub(crate) const COMPONENT_RENDER_FN: &str = "render";
/// Error message when `#[component]` impl block does not define `fn render(&self) -> NodeId`.
pub(crate) const ERR_COMPONENT_MISSING_RENDER: &str =
    "`#[component]` requires `fn render(&self) -> NodeId` inside the annotated `impl` block";
/// Error message for unknown `#[component(...)]` argument keys.
pub(crate) const ERR_COMPONENT_UNKNOWN_ATTR: &str =
    "unknown `#[component]` attribute — expected `retention = Destroy | Freeze | Alive`";
