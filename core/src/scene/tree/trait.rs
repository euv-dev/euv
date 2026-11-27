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

/// Polymorphic text payload trait mounted by `view!` `text { { expr } }` supporting both static strings and reactive closures without emitting consumer-scope imports.
pub trait TextNodeContent {
    /// Mounts this static or reactive text payload into `ctx` with `style`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Text value or reactive producer closure.
    /// - `&mut Context` - Active scene build context.
    /// - `Style` - Resolved text style.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted text scene node handle.
    fn mount_text_content(self, ctx: &mut Context, style: Style) -> NodeId;
}
