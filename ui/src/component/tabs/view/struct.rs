use super::*;

/// One entry of the [`euv_tabs`] tab bar.
///
/// The `key` is the stable identity used for diffing and is written into
/// the caller-owned `active` signal; the `label` is the visible text.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvTabItem {
    /// The stable identity of this tab (written to the `active` signal).
    #[get(type(copy))]
    pub key: &'static str,
    /// The display label of this tab.
    #[get(type(copy))]
    pub label: &'static str,
}

/// Props for the [`euv_tabs`] component.
///
/// The selected tab lives in the caller-owned `active` signal so it survives
/// re-renders and can be shared with page-level state; the panel content is
/// supplied as children and is always mounted.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvTabsProps {
    /// The tab entries rendered in the bar, in display order.
    pub items: Vec<EuvTabItem>,
    /// The key of the selected tab (drives the `_active` tab class).
    pub active: Signal<String>,
}
