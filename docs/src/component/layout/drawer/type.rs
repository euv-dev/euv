use super::*;

/// Nonreactive initial-focus, opener and owned animation-frame state.
pub(crate) type DocsDrawerModalState = Rc<(
    Cell<bool>,
    RefCell<Option<HtmlElement>>,
    Cell<Option<i32>>,
    Cell<Option<i32>>,
)>;
