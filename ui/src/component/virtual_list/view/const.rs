/// The DOM id given to a virtual list whose caller supplies no id.
///
/// The id is rendered onto the scroll container element, so it is
/// visible to CSS selectors and to test queries; give each instance an
/// explicit id when more than one virtual list is mounted.
pub(crate) const VIRTUAL_LIST_DEFAULT_ID: &str = "virtual-list-default";
