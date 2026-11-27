use super::*;

/// Elevated surface card container with optional title, subtitle, and child nodes.
#[derive(Clone, Data, Debug, New, Props)]
pub struct EuvCard {
    /// Optional card heading.
    pub(crate) title: String,
    /// Optional muted subtitle.
    #[new(skip)]
    #[prop(custom)]
    pub(crate) subtitle: String,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
    /// Pre-mounted child nodes appended inside the card body.
    #[new(skip)]
    pub(crate) children: Vec<NodeId>,
}

/// Page header banner component displaying title and subtitle consistently across all pages.
#[derive(Clone, Data, Debug, New, Props)]
pub struct EuvHeader {
    /// Page title text.
    pub(crate) title: String,
    /// Page description subtitle.
    pub(crate) subtitle: String,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Compact KPI / telemetry metric card.
#[derive(Clone, Data, Debug, New, Props)]
pub struct EuvStatCard {
    /// Metric label.
    pub(crate) label: String,
    /// Primary formatted metric value.
    pub(crate) value: String,
    /// Supporting caption or delta text.
    pub(crate) caption: String,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}
