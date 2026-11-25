use super::*;

/// Trait implemented by declarative UI components and primitives that mount into a `SceneTree`.
pub trait Mountable {
    /// Mounts this view or component into `ctx` and returns its root `NodeId`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Owned view or component instance.
    /// - `&mut BuildContext` - Mutable build context referencing the active scene tree.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Root scene node handle produced by mounting.
    fn mount(self, ctx: &mut BuildContext) -> NodeId;

    /// Mounts this view or component into the active ambient `BuildContext` managed by `App::mount` or `view!`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Owned view or component instance.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Root scene node handle produced by mounting.
    fn mount_current(self) -> NodeId
    where
        Self: Sized,
    {
        let mut ctx: BuildContext = current_build_context();
        self.mount(&mut ctx)
    }
}
