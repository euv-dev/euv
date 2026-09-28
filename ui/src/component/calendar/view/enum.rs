use super::*;

/// The column headings of the [`euv_calendar`] day grid.
///
/// The order of the vector — not the enum order — decides the column order,
/// so a locale that starts the week on Sunday simply passes
/// `[EuvCalendarWeekday::Sun, ..]`.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EuvCalendarWeekday {
    /// Monday.
    #[default]
    Mon,
    /// Tuesday.
    Tue,
    /// Wednesday.
    Wed,
    /// Thursday.
    Thu,
    /// Friday.
    Fri,
    /// Saturday.
    Sat,
    /// Sunday.
    Sun,
}
