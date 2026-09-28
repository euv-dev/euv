use super::*;

/// One card of the [`euv_feature_grid`] component.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvFeature {
    /// The feature icon (emoji, skipped when empty).
    #[get(type(copy))]
    pub icon: &'static str,
    /// The feature title.
    #[get(type(copy))]
    pub title: &'static str,
    /// The feature details.
    #[get(type(copy))]
    pub details: &'static str,
}

/// Props for the [`euv_feature_grid`] component.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvFeatureGridProps {
    /// The feature cards (skipped when empty).
    #[get(type(copy))]
    pub features: &'static [EuvFeature],
}
