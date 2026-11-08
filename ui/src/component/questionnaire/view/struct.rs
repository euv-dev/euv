use super::*;

/// Props for the [`euv_questionnaire`] component.
///
/// The questionnaire takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvQuestionnaireProps {}

/// Props for the [`euv_questionnaire_progress`] component.
///
/// The progress takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvQuestionnaireProgressProps {}

/// Props for the [`euv_questionnaire_item`] component.
///
/// The item takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvQuestionnaireItemProps {}

/// Props for the [`euv_questionnaire_title`] component.
///
/// The title takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvQuestionnaireTitleProps {}

/// Props for the [`euv_questionnaire_description`] component.
///
/// The description takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvQuestionnaireDescriptionProps {}

/// Props for the [`euv_questionnaire_choices`] component.
///
/// The choices carry the kind of answer they take.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvQuestionnaireChoicesProps {
    /// The kind of answer the choices take.
    pub choice_type: EuvQuestionnaireChoiceType,
}

/// Props for the [`euv_questionnaire_choice`] component.
///
/// The choice carries the kind of answer it takes, whether it is
/// ticked, and whether it refuses to be picked.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvQuestionnaireChoiceProps {
    /// The kind of answer the choice takes.
    pub choice_type: EuvQuestionnaireChoiceType,
    /// Whether the choice is ticked.
    pub checked: bool,
    /// Whether the choice refuses to be picked.
    pub disabled: bool,
}

/// Props for the [`euv_questionnaire_choice_label`] component.
///
/// The label takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvQuestionnaireChoiceLabelProps {}

/// Props for the [`euv_questionnaire_choice_description`] component.
///
/// The description takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvQuestionnaireChoiceDescriptionProps {}

/// Props for the [`euv_questionnaire_choice_indicator`] component.
///
/// The indicator carries the kind of answer it stands for and whether
/// the choice is ticked.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvQuestionnaireChoiceIndicatorProps {
    /// The kind of answer the indicator stands for.
    pub choice_type: EuvQuestionnaireChoiceType,
    /// Whether the choice is ticked.
    pub checked: bool,
}

/// Props for the [`euv_questionnaire_input`] component.
///
/// The input takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvQuestionnaireInputProps {}

/// Props for the [`euv_questionnaire_error`] component.
///
/// The error takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvQuestionnaireErrorProps {}

/// Props for the [`euv_questionnaire_actions`] component.
///
/// The actions take no settings of their own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvQuestionnaireActionsProps {}

/// Props for the [`euv_questionnaire_previous`] component.
///
/// The button takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvQuestionnairePreviousProps {}

/// Props for the [`euv_questionnaire_skip`] component.
///
/// The button takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvQuestionnaireSkipProps {}

/// Props for the [`euv_questionnaire_next`] component.
///
/// The button takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvQuestionnaireNextProps {}

/// Props for the [`euv_questionnaire_submit`] component.
///
/// The button takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvQuestionnaireSubmitProps {}
