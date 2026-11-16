use super::*;

pub(crate) type DropdownParentBinding = (Signal<bool>, Option<Signal<bool>>, Box<dyn FnOnce()>);
pub(crate) type DropdownSubParentBinding = (Signal<bool>, Signal<bool>, usize, usize);
