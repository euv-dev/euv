use super::*;

/// The kind of answer a choice accepts.
///
/// The reference publishes two: the choice that takes one answer out of
/// a list, and the choice that can be ticked on its own.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum EuvQuestionnaireChoiceType {
    /// The choice takes one answer out of the list.
    #[default]
    Radio,
    /// The choice can be ticked on its own.
    Checkbox,
}
