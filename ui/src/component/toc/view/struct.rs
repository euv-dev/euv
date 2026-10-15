use super::*;

/// One entry of the [`euv_toc`] anchor list.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvTocItem {
    /// The heading level (levels deeper than 2 render indented).
    #[get(type(copy))]
    pub level: u8,
    /// The display text.
    #[get(type(copy))]
    pub text: &'static str,
    /// The full href (for hash routers typically `#<route>#<anchor>`).
    #[get(type(copy))]
    pub href: &'static str,
}

/// Props for the [`euv_toc`] component.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvTocProps {
    /// The list title (for example `On this page`).
    pub title: &'static str,
    /// The anchor items.
    pub items: &'static [EuvTocItem],
    /// The `href` of the entry the reader is currently at; that entry
    /// renders the `_active` class.
    ///
    /// A signal rather than a plain field so scroll-driven highlighting
    /// is ordinary reactive state the renderer owns: the caller writes
    /// the signal and the TOC repaints from it. A caller with no
    /// scroll spy leaves this empty and every entry renders inactive.
    pub active: Signal<String>,
}
