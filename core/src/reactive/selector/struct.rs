use super::*;

/// O(1) keyed reactive selector notifying only the previously selected and newly selected keys.
#[derive(CustomDebug)]
pub struct Selector<K>
where
    K: Clone + Eq + Hash + 'static,
{
    /// Active key value mirror.
    #[debug(skip)]
    pub(crate) active: Rc<RefCell<Option<K>>>,
    /// Per-key boolean selection signals.
    #[debug(skip)]
    pub(crate) subscribers: Rc<RefCell<HashMap<K, Signal<bool>>>>,
    /// Effect watching the source key producer.
    pub(crate) watcher: Effect,
}
