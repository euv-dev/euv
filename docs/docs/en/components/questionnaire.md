---
title: Questionnaire
order: 45
---

A run of questions and the answers they collect.

## Installation

```shell
cargo add euv-ui
```

## Composition

```text
Questionnaire
├── QuestionnaireProgress
├── QuestionnaireItem
├── QuestionnaireTitle
├── QuestionnaireDescription
├── QuestionnaireChoices
├── QuestionnaireChoice
├── QuestionnaireInput
├── QuestionnaireError
├── QuestionnaireActions
├── QuestionnairePrevious
├── QuestionnaireSkip
├── QuestionnaireNext
├── QuestionnaireSubmit
```

## Parts

| Part | Component | Slot |
| --- | --- | --- |
| Questionnaire | `euv_questionnaire` | `questionnaire` |
| QuestionnaireActions | `euv_questionnaire_actions` | `questionnaire-actions` |
| QuestionnaireChoice | `euv_questionnaire_choice` | `questionnaire-choice` |
| QuestionnaireChoices | `euv_questionnaire_choices` | `questionnaire-choices` |
| QuestionnaireDescription | `euv_questionnaire_description` | `questionnaire-description` |
| QuestionnaireError | `euv_questionnaire_error` | `questionnaire-error` |
| QuestionnaireInput | `euv_questionnaire_input` | `questionnaire-input` |
| QuestionnaireItem | `euv_questionnaire_item` | `questionnaire-item` |
| QuestionnaireNext | `euv_questionnaire_next` | `questionnaire-next` |
| QuestionnairePrevious | `euv_questionnaire_previous` | `questionnaire-previous` |
| QuestionnaireProgress | `euv_questionnaire_progress` | `questionnaire-progress` |
| QuestionnaireSkip | `euv_questionnaire_skip` | `questionnaire-skip` |
| QuestionnaireSubmit | `euv_questionnaire_submit` | `questionnaire-submit` |
| QuestionnaireTitle | `euv_questionnaire_title` | `questionnaire-title` |
