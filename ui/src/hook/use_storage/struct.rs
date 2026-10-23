use super::*;

/// Reactive state for one stored value.
///
/// The handle carries the key it watches, the current value, and the
/// kind of storage it lives in.
///
/// The value is a `Signal<String>`: a render closure that reads it
/// re-renders when the value changes, and nothing else in the tree is
/// disturbed.
#[derive(Clone, CustomDebug, Data)]
pub struct UseStorage {
    /// The storage key this handle watches.
    #[get(type(copy))]
    pub key: &'static str,
    /// The current stored value.
    #[get(type(copy))]
    #[set(skip)]
    pub value: Signal<String>,
    /// Which storage the value lives in.
    #[get(type(copy))]
    pub kind: UseStorageKind,
}
