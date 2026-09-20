#[derive(Clone, Copy, Default)]
pub struct DocsFeatureProps {
    pub feature: crate::data::DocsFeature,
}

#[derive(Clone, Copy, Default)]
pub struct DocsFeatureGridProps {
    pub features: &'static [crate::data::DocsFeature],
}

#[derive(Clone, Copy, Default)]
pub struct DocsStatsRowProps {
    pub stats: &'static [crate::data::DocsStat],
}
