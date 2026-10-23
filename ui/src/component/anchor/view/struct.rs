use super::*;

/// One entry of the [`euv_anchor`] list.
///
/// Carries the machine value the anchor points at and the label the
/// reader sees.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvAnchorItem {
    /// The machine value of the anchor target.
    #[get(type(copy))]
    pub value: &'static str,
    /// The display label of the anchor entry.
    #[get(type(copy))]
    pub label: &'static str,
}

/// Props for the [`euv_anchor`] component.
///
/// Defines the strongly-typed interface for an anchor list: the entries
/// it holds and the entry that is currently the active one.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvAnchorProps {
    /// The anchor entries, in display order.
    pub items: Vec<EuvAnchorItem>,
    /// The heading above the entries (skipped when empty).
    pub title: &'static str,
    /// The value of the entry that is currently active.
    pub active: Signal<String>,
    /// The callback fired when an entry is taken.
    #[debug(skip)]
    pub on_select: Option<Rc<dyn Fn(&'static str)>>,
}
