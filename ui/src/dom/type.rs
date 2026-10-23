use super::*;

/// The teardown a browser-backed hook runs when its component unmounts.
///
/// The slot holds the closure that takes the hook's browser
/// subscriptions back off the DOM. It is a cell because the teardown
/// is registered once, on the first render, and run once, when the
/// component goes away.
pub(crate) type Teardown = Rc<RefCell<Option<Box<dyn Fn()>>>>;
