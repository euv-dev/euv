use super::*;

/// Shared native Rust callback invoked when a rendered HTML hyperlink is activated.
pub type HtmlLinkCallback = Rc<RefCell<dyn FnMut(String)>>;
