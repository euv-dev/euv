use super::*;

/// Renders the frame a chart is drawn in.
///
/// The reference builds the chart container on top of a charting library
/// and hands it the colours of every series through a configuration. The
/// container here plays the same part: it writes the colour of each
/// series into a custom property named after the series, so the plot
/// inside it can read the colour it is drawn in.
///
/// # Arguments
///
/// - `VirtualNode<EuvChartProps>` - The props node carrying the
///   identifier and the colour configuration of the chart. The plot is
///   passed as children.
///
/// # Returns
///
/// - `VirtualNode` - The euv chart virtual DOM tree.
#[component]
pub fn euv_chart(node: VirtualNode<EuvChartProps>) -> VirtualNode {
    let EuvChartProps { id, config }: EuvChartProps = node.try_get_props().unwrap_or_default();
    // Each series of the configuration becomes a custom property named
    // after the series, so the plot inside the container can read the
    // colour it is drawn in without the caller restating it.
    let mut style_text: String = String::new();
    for (key, color) in config.iter() {
        style_text.push_str(CHART_COLOR_VAR_PREFIX);
        style_text.push_str(key);
        style_text.push_str(": ");
        style_text.push_str(color);
        style_text.push(';');
    }
    let style_ref: &str = style_text.as_str();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "chart"
            "data-chart": id
            style: style_ref
            class: c_euv_chart()
            children
        }
    }
}

/// Renders the frame a chart is drawn in.
///
/// The reference builds the chart container on top of a charting library
/// and hands it the colours of every series through a configuration. The
/// container here plays the same part: it writes the colour of each
/// series into a custom property named after the series, so the plot
/// inside it can read the colour it is drawn in.
///
/// # Arguments
///
/// - `VirtualNode<EuvChartContainerProps>` - The props node carrying the
///   identifier and the colour configuration of the chart. The plot is
///   passed as children.
///
/// # Returns
///
/// - `VirtualNode` - The euv chart container virtual DOM tree.
#[component]
pub fn euv_chart_container(node: VirtualNode<EuvChartContainerProps>) -> VirtualNode {
    let EuvChartContainerProps { id, config }: EuvChartContainerProps =
        node.try_get_props().unwrap_or_default();
    // Each series of the configuration becomes a custom property named
    // after the series, so the plot inside the container can read the
    // colour it is drawn in without the caller restating it.
    let mut style_text: String = String::new();
    for (key, color) in config.iter() {
        style_text.push_str(CHART_COLOR_VAR_PREFIX);
        style_text.push_str(key);
        style_text.push_str(": ");
        style_text.push_str(color);
        style_text.push(';');
    }
    let style_ref: &str = style_text.as_str();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "chart"
            "data-chart": id
            style: style_ref
            class: c_euv_chart_container()
            children
        }
    }
}

/// Renders the panel that describes the point the reader is looking at.
///
/// The tooltip holds the label of the point and one row per series.
///
/// # Arguments
///
/// - `VirtualNode<EuvChartTooltipProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv chart tooltip virtual DOM tree.
#[component]
pub fn euv_chart_tooltip(node: VirtualNode<EuvChartTooltipProps>) -> VirtualNode {
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "chart-tooltip"
            class: c_euv_chart_tooltip()
            children
        }
    }
}

/// Holds the name of the point the tooltip describes.
///
/// # Arguments
///
/// - `VirtualNode<EuvChartTooltipLabelProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv chart tooltip label virtual DOM tree.
#[component]
pub fn euv_chart_tooltip_label(node: VirtualNode<EuvChartTooltipLabelProps>) -> VirtualNode {
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "chart-tooltip-label"
            class: c_euv_chart_tooltip_label()
            children
        }
    }
}

/// The floating body of a tooltip.
///
/// The reference separates the tooltip from the body it floats: the
/// tooltip is the anchor the body hangs off, and the body holds the
/// label of the point and one row per series.
///
/// # Arguments
///
/// - `VirtualNode<EuvChartTooltipContentProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv chart tooltip content virtual DOM tree.
#[component]
pub fn euv_chart_tooltip_content(node: VirtualNode<EuvChartTooltipContentProps>) -> VirtualNode {
    let EuvChartTooltipContentProps {}: EuvChartTooltipContentProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "chart-tooltip-content"
            class: c_euv_chart_tooltip_content()
            children
        }
    }
}

/// Holds one series inside a chart tooltip.
///
/// The row carries the colour of the series, so the reader can match the
/// row with the line in the plot.
///
/// # Arguments
///
/// - `VirtualNode<EuvChartTooltipItemProps>` - The props node carrying
///   the name, the value and the colour of the series.
///
/// # Returns
///
/// - `VirtualNode` - The euv chart tooltip item virtual DOM tree.
#[component]
pub fn euv_chart_tooltip_item(node: VirtualNode<EuvChartTooltipItemProps>) -> VirtualNode {
    let EuvChartTooltipItemProps { name, value, color }: EuvChartTooltipItemProps =
        node.try_get_props().unwrap_or_default();
    let indicator_style: String = format!("background: {};", color);
    let indicator_ref: &str = indicator_style.as_str();
    html! {
        div {
            "data-slot": "chart-tooltip-item"
            class: c_euv_chart_tooltip_item()
            span {
                "data-slot": "chart-indicator"
                "aria-hidden": "true"
                style: indicator_ref
                class: c_euv_chart_indicator()
            }
            span {
                "data-slot": "chart-tooltip-name"
                name
            }
            span {
                "data-slot": "chart-tooltip-value"
                value
            }
        }
    }
}

/// Renders the key that names the series of a chart.
///
/// The legend sits below the plot and holds one entry per series.
///
/// # Arguments
///
/// - `VirtualNode<EuvChartLegendProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv chart legend virtual DOM tree.
#[component]
pub fn euv_chart_legend(node: VirtualNode<EuvChartLegendProps>) -> VirtualNode {
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "chart-legend"
            class: c_euv_chart_legend()
            children
        }
    }
}

/// Holds one series inside a chart legend.
///
/// # Arguments
///
/// - `VirtualNode<EuvChartLegendItemProps>` - The props node carrying
///   the name and the colour of the series.
///
/// # Returns
///
/// - `VirtualNode` - The euv chart legend item virtual DOM tree.
#[component]
pub fn euv_chart_legend_item(node: VirtualNode<EuvChartLegendItemProps>) -> VirtualNode {
    let EuvChartLegendItemProps { label, color }: EuvChartLegendItemProps =
        node.try_get_props().unwrap_or_default();
    let indicator_style: String = format!("background: {};", color);
    let indicator_ref: &str = indicator_style.as_str();
    html! {
        div {
            "data-slot": "chart-legend-item"
            class: c_euv_chart_legend_item()
            span {
                "data-slot": "chart-indicator"
                "aria-hidden": "true"
                style: indicator_ref
                class: c_euv_chart_indicator()
            }
            span {
                "data-slot": "chart-legend-label"
                label
            }
        }
    }
}

/// The body of the key that names the series of a chart.
///
/// The reference separates the legend from the body it holds: the legend
/// is the anchor, and the body carries one entry per series.
///
/// # Arguments
///
/// - `VirtualNode<EuvChartLegendContentProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv chart legend content virtual DOM tree.
#[component]
pub fn euv_chart_legend_content(node: VirtualNode<EuvChartLegendContentProps>) -> VirtualNode {
    let EuvChartLegendContentProps {}: EuvChartLegendContentProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "chart-legend-content"
            class: c_euv_chart_legend_content()
            children
        }
    }
}
