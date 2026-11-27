use super::*;

/// Collected scopes and `(NodeId, Rc<dyn Lifecycle>)` observers returned when tearing down a scene subtree.
pub type DisposedSubtreePayload = (Vec<Scope>, Vec<(NodeId, Rc<dyn Lifecycle>)>);

/// Collected `(NodeId, Rc<dyn Lifecycle>)` observers and `ScopeId`s targeted by subtree system event dispatch.
pub type SubtreeLifecycleTargets = (Vec<(NodeId, Rc<dyn Lifecycle>)>, Vec<ScopeId>);
