use super::*;

/// Props for the [`euv_stat`] component.
///
/// Defines the strongly-typed interface for a single metric tile: an
/// optional leading icon, the accented value, a muted label, and an
/// optional hint line underneath.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvStatProps {
    /// The metric name rendered under the value in muted styling.
    pub label: &'static str,
    /// The metric value rendered in the large accented style.
    pub value: &'static str,
    /// The optional secondary note under the label (skipped when empty).
    pub hint: &'static str,
    /// The optional leading glyph or icon (skipped when empty).
    pub icon: &'static str,
}
