use super::*;

/// A handler registered on a chart.
///
/// The handler is shared: registering the same closure twice runs it
/// twice, which is what a user expects from two separate registrations.
pub type EventHandler = Rc<dyn Fn(&ChartEvent)>;
