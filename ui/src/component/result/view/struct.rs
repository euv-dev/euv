use super::*;

/// Props for the [`euv_result`] component.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvResultProps {
    /// The big status code (e.g. `"404"`, skipped when empty).
    pub code: ::euv::CowText,
    /// The result title (skipped when empty).
    pub title: ::euv::CowText,
    /// The description rendered under the title (skipped when empty).
    pub description: ::euv::CowText,
    /// The heading level the title is rendered at.
    ///
    /// The level is a setting of the component because the library cannot
    /// know the outline of the page the result sits in: the result is the
    /// whole page on one route and a panel inside another.
    #[get(type(copy))]
    pub level: Option<EuvHeadingLevel>,
}
