use super::*;

/// No-op `Lifecycle` implementation allowing raw `NodeId` handles to participate seamlessly in `Component` APIs.
impl Lifecycle for NodeId {}

/// Zero-overhead `Component` implementation returning an already-mounted `NodeId` unchanged.
impl Component for NodeId {
    /// Returns this `NodeId` handle unchanged.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Already-mounted scene node handle.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Same scene node handle.
    fn render(&self) -> NodeId {
        *self
    }

    /// Returns this `NodeId` directly without heap allocation or duplicate lifecycle attachment.
    ///
    /// # Arguments
    ///
    /// - `Self` - Already-mounted scene node handle.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Same scene node handle.
    fn mount(self) -> NodeId {
        self
    }
}
