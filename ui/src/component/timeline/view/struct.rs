use super::*;

/// One entry of the [`euv_timeline`] event log.
///
/// A timeline item is a headline, an optional body, and an optional
/// timestamp. The empty fields are skipped at render time so a terse
/// one-line entry does not leave an empty gap in the column.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvTimelineItem {
    /// The entry headline rendered in `c_euv_timeline_title`.
    #[get(type(copy))]
    pub title: &'static str,
    /// The optional entry body rendered in `c_euv_timeline_desc`.
    #[get(type(copy))]
    pub description: &'static str,
    /// The optional timestamp rendered in the monospace
    /// `c_euv_timeline_time` style (skipped when empty).
    #[get(type(copy))]
    pub time: &'static str,
}

/// Props for the [`euv_timeline`] component.
///
/// Defines the strongly-typed interface for a vertical event log. The
/// entries are ordered oldest-first, matching the reading direction of the
/// rail that connects them.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvTimelineProps {
    /// The ordered event entries; an empty list renders an empty column.
    pub items: Vec<EuvTimelineItem>,
}
