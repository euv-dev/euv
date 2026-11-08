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
