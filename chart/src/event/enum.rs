/// The kind of an event a chart emits.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum ChartEventType {
    /// The pointer was pressed and released over the chart.
    #[default]
    Click,
    /// The pointer was pressed twice over the chart.
    DoubleClick,
    /// The pointer moved over the chart.
    MouseMove,
    /// The pointer entered a data point.
    MouseOver,
    /// The pointer left a data point.
    MouseOut,
    /// The pointer left the chart.
    GlobalOut,
    /// A legend entry was selected.
    LegendSelect,
    /// A legend entry was deselected.
    LegendUnSelect,
    /// A legend entry was toggled.
    LegendToggle,
    /// The zoom window changed.
    DataZoom,
    /// The zoom window settled.
    DataZoomEnd,
    /// The tooltip became visible.
    TooltipShow,
    /// The tooltip was hidden.
    TooltipHide,
}

/// An action a chart can be told to perform.
///
/// Actions are the counterpart of events: a chart dispatches events when
/// the user does something, and the application dispatches actions to
/// make the chart do something.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum ChartAction {
    /// Highlights a data point.
    Highlight {
        /// The index of the series.
        series_index: usize,
        /// The index of the data point, or `None` for the whole series.
        data_index: Option<usize>,
    },
    /// Removes the highlight from a data point.
    Downplay {
        /// The index of the series.
        series_index: usize,
        /// The index of the data point, or `None` for the whole series.
        data_index: Option<usize>,
    },
    /// Shows the tooltip at a position.
    ShowTip {
        /// The index of the series the tooltip describes.
        series_index: usize,
        /// The index of the data point the tooltip describes.
        data_index: usize,
        /// The horizontal position of the tooltip, in canvas pixels.
        x: f64,
        /// The vertical position of the tooltip, in canvas pixels.
        y: f64,
    },
    /// Hides the tooltip.
    #[default]
    HideTip,
    /// Zooms the chart to a window of the data.
    ///
    /// `start` and `end` are percentages of the whole range.
    DataZoom {
        /// The start of the window, as a percentage.
        start: f64,
        /// The end of the window, as a percentage.
        end: f64,
    },
    /// Toggles a legend entry.
    LegendToggle {
        /// The name of the series.
        name: String,
    },
    /// Restores the chart to its initial state.
    Restore,
}
