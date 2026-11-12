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

/// Props of the [`euv_carousel_content`] component.
///
/// The content is the viewport: it clips the track and lays the slides
/// out along the orientation.
#[derive(Clone, Copy, CustomDebug, Default)]
pub struct EuvCarouselContentProps {
    /// The direction the slides travel in.
    pub orientation: EuvCarouselOrientation,
}

/// Props of the [`euv_carousel_item`] component.
///
/// The item is one slide of the track.
#[derive(Clone, Copy, CustomDebug, Default)]
pub struct EuvCarouselItemProps {
    /// The direction the slides travel in.
    pub orientation: EuvCarouselOrientation,
}

/// Props of the [`euv_carousel_next`] component.
///
/// The button steps the track forward.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvCarouselNextProps {
    /// The direction the slides travel in.
    pub orientation: EuvCarouselOrientation,
    /// The accessible name of the button.
    pub label: &'static str,
    /// Whether the button refuses to be pressed.
    pub disabled: bool,
    /// Optional callback fired when the button is pressed.
    #[debug(skip)]
    pub on_press: Option<Rc<dyn Fn()>>,
}

/// Props of the [`euv_carousel_previous`] component.
///
/// The button steps the track back.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvCarouselPreviousProps {
    /// The direction the slides travel in.
    pub orientation: EuvCarouselOrientation,
    /// The accessible name of the button.
    pub label: &'static str,
    /// Whether the button refuses to be pressed.
    pub disabled: bool,
    /// Optional callback fired when the button is pressed.
    #[debug(skip)]
    pub on_press: Option<Rc<dyn Fn()>>,
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
