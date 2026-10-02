use super::*;

/// The scroll-to-anchor callback subscribed to the route signal.
pub(crate) type AnchorScroll = Arc<dyn Fn() + Send + Sync>;
