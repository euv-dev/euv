use super::*;

/// One item of the footer row.
///
/// The text is the notice itself — a filing number, plain and
/// selectable — while the optional icon is the official badge the
/// filing authority publishes, served by the site itself.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvFooterItem {
    /// The link text.
    #[get(type(copy))]
    pub text: &'static str,
    /// The destination the link points at.
    #[get(type(copy))]
    pub link: &'static str,
    /// An optional image rendered before the text; empty renders no image.
    #[get(type(copy))]
    pub icon: &'static str,
    /// The accessible name of the icon image.
    #[get(type(copy))]
    pub alt: &'static str,
}

/// Props for the [`euv_footer`] component.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvFooterProps {
    /// The plain text of the footer (for example the license line).
    ///
    /// An empty text renders no text row, which is how a footer that
    /// carries nothing but its items is written.
    #[get(type(copy))]
    pub text: &'static str,
    /// The destination the text points at.
    ///
    /// An empty link renders the text as plain, unlinked text.
    #[get(type(copy))]
    pub link: &'static str,
    /// The items of the footer row, in display order.
    #[get(type(copy))]
    pub items: &'static [EuvFooterItem],
}
