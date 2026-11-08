use super::*;

/// Props for the [`euv_chart`] component.
///
/// The container carries the identifier that ties the plot to its
/// colours and the configuration that names every series in it. Each
/// entry of the configuration pairs a series with the colour it is
/// drawn in, and the colour is written into a custom property the plot
/// reads.
///
/// The plot itself is passed as children.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvChartProps {
    /// The identifier that ties the plot to its colours.
    pub id: &'static str,
    /// The colour of every series in the chart.
    ///
    /// Each entry pairs the name of a series with the colour it is
    /// drawn in. The colour is a CSS value, so a theme token such as
    /// `var(--primary)` follows the theme.
    pub config: Vec<(&'static str, &'static str)>,
}

/// Props for the [`euv_chart_tooltip_item`] component.
///
/// The row of the tooltip carries the name of a series, the value it
/// holds at the point the reader is looking at, and the colour the
/// series is drawn in.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvChartTooltipItemProps {
    /// The name of the series.
    pub name: &'static str,
    /// The value the series holds.
    pub value: &'static str,
    /// The colour the series is drawn in.
    pub color: &'static str,
}

/// Props for the [`euv_chart_legend_item`] component.
///
/// The legend entry carries the name of a series and the colour it is
/// drawn in.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvChartLegendItemProps {
    /// The name of the series.
    pub label: &'static str,
    /// The colour the series is drawn in.
    pub color: &'static str,
}

/// Props for the [`euv_chart_tooltip`] component.
///
/// The tooltip holds the rows of the point and takes no settings of
/// its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvChartTooltipProps {}

/// Props for the [`euv_chart_tooltip_label`] component.
///
/// The label holds the name of the point and takes no settings.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvChartTooltipLabelProps {}

/// Props for the [`euv_chart_legend`] component.
///
/// The legend holds the entries of the key and takes no settings of
/// its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvChartLegendProps {}
