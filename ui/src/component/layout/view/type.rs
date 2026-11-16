use super::*;

/// Caller-owned content factory; invoked in its semantic region for the current mode.
pub type EuvLayoutSlot = Rc<dyn Fn(EuvLayoutMode) -> VirtualNode>;
