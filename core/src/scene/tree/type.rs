use super::*;

/// Collected scopes and `(NodeId, Rc<dyn Lifecycle>)` observers returned when tearing down a scene subtree.
pub type DisposedSubtreePayload = (Vec<Scope>, Vec<(NodeId, Rc<dyn Lifecycle>)>);
