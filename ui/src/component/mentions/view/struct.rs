use super::*;

/// One person or handle the [`euv_mentions`] field can mention.
///
/// Carries the machine value reported to the caller when the mention is
/// picked, and the label written into the field.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvMentionItem {
    /// The machine value of the mention.
    #[get(type(copy))]
    pub value: &'static str,
    /// The label the reader sees.
    #[get(type(copy))]
    pub label: &'static str,
}

/// Props for the [`euv_mentions`] component.
///
/// The text in the field lives in a caller-owned `Signal<String>` so the
/// component takes part in two-way binding; the open state of the
/// suggestion panel is owned by the component itself.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvMentionsProps {
    /// The mentions the field can suggest.
    pub items: Vec<EuvMentionItem>,
    /// The current text of the field.
    pub value: Signal<String>,
    /// The text shown when the field is empty.
    pub placeholder: &'static str,
    /// The group label above the field.
    pub label: &'static str,
    /// The callback fired when a mention is chosen.
    #[debug(skip)]
    pub on_select: Option<Rc<dyn Fn(&'static str)>>,
}
