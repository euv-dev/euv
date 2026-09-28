use super::*;

/// One entry of the [`euv_collapse`] accordion.
///
/// The `key` is the stable identity written into the caller-owned
/// `open_keys` signal; the `title` is the header text. The panel body is
/// supplied by the caller through the `bodies` prop.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvCollapseItem {
    /// The stable identity of this section (written to the `open_keys`
    /// signal).
    #[get(type(copy))]
    pub key: &'static str,
    /// The header text of this section.
    #[get(type(copy))]
    pub title: &'static str,
}

/// Props for the [`euv_collapse`] component.
///
/// `open_keys` holds the currently-open section keys; a `Vec<String>`
/// (rather than a single key) is used so `allow_multiple` can widen or
/// narrow the set without a different prop type. An empty list means every
/// section is closed.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvCollapseProps {
    /// The accordion sections, in display order.
    pub items: Vec<EuvCollapseItem>,
    /// The keys of the currently-open sections (empty means all closed).
    pub open_keys: Signal<Vec<String>>,
    /// Whether more than one section may stay open at the same time.
    pub allow_multiple: bool,
}
