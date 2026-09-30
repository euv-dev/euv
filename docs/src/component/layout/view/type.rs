/// The scroll-to-anchor callback subscribed to the route signal.
pub(crate) type AnchorScroll = std::sync::Arc<dyn Fn() + Send + Sync>;
