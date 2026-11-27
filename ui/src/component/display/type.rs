use super::*;

/// Type-erased row factory stored by `EuvVirtualListHandle` for targeted single-row refreshes.
pub(crate) type VirtualRowBuilder = Box<dyn FnMut(usize) -> NodeId>;

/// Sliding window storing `(logical_index, row_scope, row_node)` entries in ascending index order.
pub(crate) type VirtualRowWindow = Rc<RefCell<VecDeque<(usize, Scope, NodeId)>>>;
