use super::*;

/// Mounts any `Component` into the framework-managed ambient `Context` through its full `Lifecycle`.
///
/// # Arguments
///
/// - `C` - Owned component implementing `Component`.
///
/// # Returns
///
/// - `NodeId` - Mounted root scene node handle.
pub fn mount_component<C>(component: C) -> NodeId
where
    C: Component,
{
    component.mount()
}

/// Dispatches `Lifecycle::on_freeze` across all registered observers collected from a subtree.
///
/// # Arguments
///
/// - `&[(NodeId, Rc<dyn Lifecycle>)]` - Collected `(node, lifecycle)` pairs.
pub fn dispatch_lifecycle_freeze(observers: &[(NodeId, Rc<dyn Lifecycle>)]) {
    for (node_id, hook) in observers.iter() {
        hook.on_freeze(*node_id);
    }
}

/// Dispatches `Lifecycle::on_resume` across all registered observers collected from a subtree.
///
/// # Arguments
///
/// - `&[(NodeId, Rc<dyn Lifecycle>)]` - Collected `(node, lifecycle)` pairs.
pub fn dispatch_lifecycle_resume(observers: &[(NodeId, Rc<dyn Lifecycle>)]) {
    for (node_id, hook) in observers.iter() {
        hook.on_resume(*node_id);
    }
}

/// Dispatches `Lifecycle::on_activate` across all registered observers collected from a subtree.
///
/// # Arguments
///
/// - `&[(NodeId, Rc<dyn Lifecycle>)]` - Collected `(node, lifecycle)` pairs.
pub fn dispatch_lifecycle_activate(observers: &[(NodeId, Rc<dyn Lifecycle>)]) {
    for (node_id, hook) in observers.iter() {
        hook.on_activate(*node_id);
    }
}

/// Dispatches `Lifecycle::on_update` across all registered observers collected from a subtree.
///
/// # Arguments
///
/// - `&[(NodeId, Rc<dyn Lifecycle>)]` - Collected `(node, lifecycle)` pairs.
pub fn dispatch_lifecycle_update(observers: &[(NodeId, Rc<dyn Lifecycle>)]) {
    for (node_id, hook) in observers.iter() {
        hook.on_update(*node_id);
    }
}

/// Executes `on_unmount`, disposes reactive `scopes`, and finishes with `on_destroy` outside any scene borrow.
///
/// # Arguments
///
/// - `Vec<Scope>` - Scopes collected from a disposed subtree.
/// - `Vec<(NodeId, Rc<dyn Lifecycle>)>` - Lifecycle observers collected from a disposed subtree.
pub fn finalize_disposed_subtree(scopes: Vec<Scope>, observers: Vec<(NodeId, Rc<dyn Lifecycle>)>) {
    for (node_id, hook) in observers.iter() {
        hook.on_unmount(*node_id);
    }
    for scope in scopes.into_iter() {
        scope.dispose();
    }
    for (_, hook) in observers.into_iter() {
        hook.on_destroy();
    }
}

/// Dispatches a normalized `SystemEvent` to every `Lifecycle` observer, invoking both its typed lifecycle method and `on_system_event`.
///
/// # Arguments
///
/// - `&[(NodeId, Rc<dyn Lifecycle>)]` - Collected `(node, lifecycle)` observer pairs.
/// - `&SystemEvent` - Normalized system event payload.
pub fn dispatch_lifecycle_system_event(
    observers: &[(NodeId, Rc<dyn Lifecycle>)],
    event: &SystemEvent,
) {
    for (_, hook) in observers.iter() {
        match event {
            SystemEvent::Appear => hook.on_appear(),
            SystemEvent::Disappear => hook.on_disappear(),
            SystemEvent::Foreground => hook.on_foreground(),
            SystemEvent::Background => hook.on_background(),
            SystemEvent::Inactive => hook.on_inactive(),
            SystemEvent::Terminate => hook.on_terminate(),
            _ => {}
        }
        hook.on_system_event(event);
    }
}
