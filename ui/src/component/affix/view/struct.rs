use super::*;

/// Props for the [`euv_affix`] component.
///
/// Defines the strongly-typed interface for a sticky block: how far from the
/// top of its scroll container the block sticks.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvAffixProps {
    /// How far from the top of the scroll container the block sticks, in pixels.
    pub offset: u32,
}
