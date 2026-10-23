use super::*;

/// An event a chart emits.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ChartEvent {
    /// The kind of the event.
    pub event_type: ChartEventType,
    /// The index of the series the event happened on.
    pub series_index: Option<usize>,
    /// The index of the data point the event happened on.
    pub data_index: Option<usize>,
    /// The name of the series the event happened on.
    pub series_name: String,
    /// The name of the data point the event happened on.
    pub name: String,
    /// The value of the data point the event happened on.
    pub value: Option<f64>,
    /// The horizontal position of the pointer, in canvas pixels.
    pub x: f64,
    /// The vertical position of the pointer, in canvas pixels.
    pub y: f64,
    /// The horizontal position of the pointer, relative to the canvas.
    pub offset_x: f64,
    /// The vertical position of the pointer, relative to the canvas.
    pub offset_y: f64,
}

/// A registered handler, together with the kind of event it listens for.
#[derive(Clone)]
pub(crate) struct EventRegistration {
    /// The kind of event the handler listens for.
    pub(crate) event_type: ChartEventType,
    /// The handler itself.
    pub(crate) handler: EventHandler,
}
