use super::*;

/// Props for the [`euv_direction`] component.
///
/// The provider carries the direction and nothing else: the subtree
/// inside it reads the direction from the wrapper.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvDirectionProps {
    /// The direction the content is laid out in.
    pub direction: EuvDirection,
}

/// Props for the [`euv_direction_provider`] component.
///
/// The provider carries the direction and nothing else: the subtree
/// inside it reads the direction from the wrapper. The direction is
/// optional, so a caller that leaves it out gets the left-to-right
/// default instead of having to name it.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvDirectionProviderProps {
    /// The direction the content is laid out in.
    ///
    /// `None` leaves the content laid out from left to right.
    pub direction: Option<EuvDirection>,
}
