use super::*;

/// One entry of the [`euv_descriptions`] grid.
///
/// A description is a label and the value it describes. The pair is
/// rendered as a label line over a value line, so a grid cell reads the
/// same way whether it sits alone or beside its neighbours.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvDescriptionsItem {
    /// The field name rendered in `c_euv_descriptions_label`.
    #[get(type(copy))]
    pub label: &'static str,
    /// The field value rendered in `c_euv_descriptions_value`.
    #[get(type(copy))]
    pub value: &'static str,
}

/// Props for the [`euv_descriptions`] component.
///
/// Defines the strongly-typed interface for a key/value detail grid. The
/// entries are laid out in source order and flow across the grid row by
/// row, so the column count decides how many entries share a row.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvDescriptionsProps {
    /// The label/value pairs to render; an empty list renders an empty grid.
    pub items: Vec<EuvDescriptionsItem>,
    /// The number of grid columns; a count of zero falls back to one column.
    pub columns: usize,
}
