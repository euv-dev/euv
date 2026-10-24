use super::*;

/// One link of the footer link row.
///
/// The canonical use is a filing notice: the text is the filing number
/// itself — selectable, copyable plain text — while the optional icon is
/// the official badge the filing authority publishes. The link points at
/// the authority's own query page, so a reader can verify the number
/// without trusting the site that shows it.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvFooterLink {
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
    /// The footer text (for example the license line).
    #[get(type(copy))]
    pub text: &'static str,
    /// The links rendered under the text; empty renders no link row.
    #[get(type(copy))]
    pub links: &'static [EuvFooterLink],
}
