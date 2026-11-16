use super::*;

thread_local! {
    /// Weak ownership links exist only while submenu lifecycle containers are mounted.
    pub(crate) static DROPDOWN_SUB_PARENTS: RefCell<HashMap<usize, Weak<Cell<Signal<bool>>>>> = RefCell::new(HashMap::new());
}
