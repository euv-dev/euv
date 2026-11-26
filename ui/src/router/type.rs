use super::*;

/// Declarative page builder function receiving the matched route metadata and parameters.
pub type EuvRouteBuilder = Rc<dyn Fn(&EuvRouteMatch) -> NodeId>;
