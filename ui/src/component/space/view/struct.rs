use super::*;

/// Props for the `euv_space` component.
///
/// Defines the strongly-typed interface for the layout gap primitive.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvSpaceProps {
    /// The spacing step taken from the design token scale.
    pub size: EuvSpaceSize,
    /// Whether the gap is applied vertically (`margin-top`) instead of
    /// horizontally (`margin-left`).
    pub vertical: Signal<bool>,
}
