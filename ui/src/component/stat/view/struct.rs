use super::*;

/// Props for the [`euv_stat`] component.
///
/// Defines the strongly-typed interface for a single metric tile: an
/// optional leading icon, the accented value, a muted label, and an
/// optional hint line underneath.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvStatProps {
    /// The metric name rendered under the value in muted styling.
    pub label: ::euv::CowText,
    /// The metric value rendered in the large accented style.
    pub value: ::euv::CowText,
    /// The optional secondary note under the label (skipped when empty).
    pub hint: ::euv::CowText,
    /// The optional leading glyph or icon (skipped when empty).
    pub icon: ::euv::CowText,
}
