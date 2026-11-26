use super::*;

/// Shared string callback type used for Markdown hyperlinks and code-block Copy actions.
pub type MarkdownStrCallback = Rc<dyn Fn(&str)>;

/// Shared task-checkbox toggle callback receiving `(item_index, next_checked)`.
pub type MarkdownTaskCallback = Rc<dyn Fn(usize, bool)>;
