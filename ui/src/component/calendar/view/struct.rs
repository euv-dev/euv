use super::*;

/// One cell of the [`euv_calendar`] day grid.
///
/// The calendar is pure presentation: the caller computes the month, the
/// leading/trailing spill-over cells and the today flag, then hands the
/// finished grid in. This keeps the component free of date arithmetic.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvCalendarDay {
    /// The day-of-month number rendered inside the cell.
    #[get(type(copy))]
    pub day: u32,
    /// Whether the cell belongs to the previous or next month.
    ///
    /// Drives the `_muted` modifier so spill-over cells read as secondary
    /// without changing the grid's shape.
    #[get(type(copy))]
    pub muted: bool,
    /// Whether the cell is today's date.
    #[get(type(copy))]
    pub today: bool,
}

/// Props for the [`euv_calendar`] component.
///
/// The component renders only what it is given: `weekdays` drives the header
/// row and `days` the grid. No date is computed here.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvCalendarProps {
    /// The month caption rendered in the header row.
    pub title: &'static str,
    /// The column headings, in display order.
    pub weekdays: Vec<EuvCalendarWeekday>,
    /// The day cells, in display order.
    pub days: Vec<EuvCalendarDay>,
    /// The currently selected day number.
    pub selected: Signal<u32>,
}
