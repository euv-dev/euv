use super::*;

/// One row of the [`euv_list`] component.
///
/// A row is a title, an optional description line, and optional trailing
/// meta text. The two optional fields are skipped at render time, so a
/// terse one-line row leaves no empty gap in the row body.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvListItem {
    /// The row headline rendered in `c_euv_list_item_title`.
    #[get(type(copy))]
    pub title: &'static str,
    /// The optional row description rendered in `c_euv_list_item_desc`
    /// (skipped when empty).
    #[get(type(copy))]
    pub desc: &'static str,
    /// The optional trailing meta text rendered in `c_euv_list_item_meta`
    /// (skipped when empty).
    #[get(type(copy))]
    pub meta: &'static str,
}

/// Props for the [`euv_list`] component.
///
/// Defines the strongly-typed interface for a list of titled rows. The
/// rows are ordered top to bottom, matching the reading direction of the
/// column they are rendered in.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvListProps {
    /// The rows to render; an empty list renders the empty state instead.
    pub items: Vec<EuvListItem>,
    /// The empty-state text rendered when there are no items (skipped when
    /// empty, leaving the list blank instead).
    pub empty_text: &'static str,
}
