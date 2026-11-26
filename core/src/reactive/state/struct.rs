use super::*;

/// Scoped reactive local component state automatically disposed on unmount and frozen/resumed with its owning `Scope`.
#[derive(Debug)]
pub struct LocalState<T>
where
    T: 'static,
{
    /// Underlying scope-owned fine-grained signal.
    pub(crate) signal: Signal<T>,
}

/// App-wide reactive global state registry supporting type-keyed and named singleton stores plus scope-aware `watch` observers.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GlobalState;
