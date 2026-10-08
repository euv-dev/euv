#[derive(Clone, Copy, Default)]
pub(crate) struct DocsFeatureProps {
    pub(crate) feature: crate::data::DocsFeature,
}

#[derive(Clone, Copy, Default)]
pub(crate) struct DocsFeatureGridProps {
    pub(crate) features: &'static [crate::data::DocsFeature],
}

#[derive(Clone, Copy, Default)]
pub(crate) struct DocsStatsRowProps {
    pub(crate) stats: &'static [crate::data::DocsStat],
}
