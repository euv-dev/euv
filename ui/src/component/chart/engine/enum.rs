/// Active drag target tracked during pointer interaction on `EuvEChart`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum ChartDragTarget {
    /// No active drag gesture.
    #[default]
    None,
    /// Dragging the `DataZoom::Slider` left handle.
    SliderLeft,
    /// Dragging the `DataZoom::Slider` right handle.
    SliderRight,
    /// Dragging the `DataZoom::Slider` selection window body.
    SliderWindow,
    /// Panning horizontally inside the plot area via `DataZoom::Inside`.
    InsidePan,
}
