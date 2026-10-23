use super::*;

/// One slide of the [`euv_carousel`].
///
/// Carries the headline of the slide and the supporting line under it.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvCarouselSlide {
    /// The headline of the slide.
    #[get(type(copy))]
    pub title: &'static str,
    /// The supporting line under the headline.
    #[get(type(copy))]
    pub desc: &'static str,
}

/// Props for the [`euv_carousel`] component.
///
/// Defines the strongly-typed interface for a slideshow: the slides it
/// cycles through and the slide that is currently showing.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvCarouselProps {
    /// The slides, in display order.
    pub slides: Vec<EuvCarouselSlide>,
    /// The index of the slide that is currently showing.
    pub active: Signal<u32>,
    /// The callback fired when the shown slide changes.
    #[debug(skip)]
    pub on_change: Option<Rc<dyn Fn(u32)>>,
}
