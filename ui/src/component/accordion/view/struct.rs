use super::*;

/// One section of the [`euv_accordion`].
///
/// The `value` is the stable identity of the section; it is what the
/// open-sections signal holds. The `trigger` is the header text and the
/// `content` is the body shown when the section is open.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvAccordionItem {
    /// The stable identity of this section.
    #[get(type(copy))]
    pub value: &'static str,
    /// The header text of this section.
    #[get(type(copy))]
    pub trigger: &'static str,
    /// The body text of this section.
    #[get(type(copy))]
    pub content: &'static str,
    /// Whether this section cannot be opened.
    #[get(type(copy))]
    pub disabled: bool,
}

/// Props for the [`euv_accordion`] component.
///
/// The open sections live in a caller-owned signal holding every open
/// value, so a `Vec<String>` is used rather than a single key: the
/// multiple mode needs the whole set and the single mode is the same
/// signal narrowed to at most one entry.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvAccordionProps {
    /// The accordion sections, in display order.
    pub items: Vec<EuvAccordionItem>,
    /// The values of the currently-open sections (empty means all closed).
    pub open_values: Signal<Vec<String>>,
    /// Whether one section or many may stay open.
    pub accordion_type: EuvAccordionType,
    /// Whether the open section may be closed again.
    pub collapsible: bool,
    /// Whether every section of the accordion is disabled.
    pub disabled: bool,
    /// Optional callback receiving the open values after a change.
    #[debug(skip)]
    pub on_change: Option<Rc<dyn Fn(Vec<String>)>>,
}
