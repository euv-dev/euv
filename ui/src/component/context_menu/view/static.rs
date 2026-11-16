use super::*;

thread_local! {
    /// Live binding identities, never numeric Signal reconstruction.
    pub(crate) static CONTEXT_MENU_SUB_PARENTS: RefCell<HashMap<usize, Weak<ContextMenuHoverOwner>>> = RefCell::new(HashMap::new());
}
