use super::*;

/// Props for [`docs_feature_card`].
#[derive(Clone, Copy, Default)]
pub struct DocsFeatureProps {
    /// The feature to render.
    pub feature: crate::data::DocsFeature,
}

/// Props for [`docs_feature_grid`].
#[derive(Clone, Copy, Default)]
pub struct DocsFeatureGridProps {
    /// The feature cards (empty grid renders nothing).
    pub features: &'static [crate::data::DocsFeature],
}