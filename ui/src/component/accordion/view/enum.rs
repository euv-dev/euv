use super::*;

/// How many sections of an accordion may be open at the same time.
///
/// The reference publishes two: a single section, and any number of
/// sections.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum EuvAccordionType {
    /// At most one section stays open.
    #[default]
    Single,
    /// Any number of sections may stay open.
    Multiple,
}
