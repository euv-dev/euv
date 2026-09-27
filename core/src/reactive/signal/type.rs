/// A `(subscription_id, callback)` pair stored in a signal's listener list.
pub(crate) type ListenerEntry = (usize, Box<dyn FnMut()>);
