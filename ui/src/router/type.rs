use super::*;

/// Declarative page builder function receiving the matched route metadata and parameters.
pub type EuvRouteBuilder = Rc<dyn Fn(&EuvRouteMatch) -> NodeId>;
/// Resolved route tuple carrying match metadata, optional builder, and optional retention override.
pub type EuvRouteResolvePolicy = (
    EuvRouteMatch,
    Option<EuvRouteBuilder>,
    Option<PageRetention>,
);
/// Collected subtree lifecycle observer handles dispatched outside scene write borrows.
pub(crate) type LifecycleObserverList = Vec<(NodeId, Rc<dyn Lifecycle>)>;
