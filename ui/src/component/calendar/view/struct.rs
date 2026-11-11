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

/// Props of the [`euv_calendar_day_button`] component.
///
/// The day button is the clickable cell of the grid. It reports whether
/// the day is the chosen one and whether it can be picked at all.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvCalendarDayButtonProps {
    /// The day-of-month number shown inside the cell.
    pub day: u32,
    /// Whether the day is the chosen one.
    pub selected: bool,
    /// Whether the day refuses to be picked.
    pub disabled: bool,
    /// Optional callback fired when the cell is clicked.
    #[debug(skip)]
    pub on_select: Option<Rc<dyn Fn(u32)>>,
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
    /// The callback a click on a day cell runs.
    ///
    /// The calendar resolves no date of its own, so the cell hands the day
    /// number and whether the cell belongs to a neighbouring month to this
    /// callback and leaves the date arithmetic to the caller. When it is
    /// absent the cell writes its day number into `selected` instead.
    #[debug(skip)]
    pub on_day_click: Option<Rc<dyn Fn(u32, bool)>>,
}
