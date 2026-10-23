use super::*;

/// A command for a chart.
///
/// The commands are what a handle sends to a chart. They are the same
/// actions a chart takes on itself — drawing a new option, resizing,
/// performing an action — and they travel through a signal, the way
/// the rest of euv talks to its parts.
///
/// The option is boxed: an option is thousands of bytes, and a queue
/// of commands should not be as big as the options it carries.
///
/// Every variant carries data, and `#[derive(Default)]` needs one
/// without any, so this enum has no default.
#[derive(Clone, Debug, PartialEq)]
pub enum ChartCommand {
    /// Draws the chart with the given option.
    SetOption(Box<ChartOption>),
    /// Draws the chart at the given size, in CSS pixels.
    Resize(f64, f64),
    /// Performs an action on the chart.
    Action(ChartAction),
}
