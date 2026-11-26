use super::*;

/// Target storage that receives a created `NodeId` via `view!` `node_ref:`.
pub trait NodeRefTarget {
    /// Stores the mounted `NodeId` handle into this reference target.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable reference target.
    /// - `NodeId` - Newly created scene node handle.
    fn bind_node_ref(&mut self, id: NodeId);
}
