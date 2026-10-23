/// The title text for the header on the form demo page.
pub(crate) const FORM_HEADER_TITLE: &str = "Form Components";

/// The subtitle text for the header on the form demo page.
pub(crate) const FORM_HEADER_SUBTITLE: &str = "Selects, checkbox groups, segmented controls, number inputs, auto-complete, date pickers and the form wrapper — the input half of the component library.";

/// The section heading for the choice card.
pub(crate) const FORM_CHOICE_CARD_TITLE: &str = "Choices";

/// The label above the select.
pub(crate) const FORM_SELECT_LABEL: &str = "Fruit";

/// The placeholder shown when no fruit is chosen.
pub(crate) const FORM_SELECT_PLACEHOLDER: &str = "Pick a fruit";

/// The value the select starts on.
pub(crate) const FORM_DEFAULT_FRUIT: &str = "apple";

/// The value of the first fruit option.
pub(crate) const FORM_FRUIT_FIRST_VALUE: &str = "apple";

/// The label of the first fruit option.
pub(crate) const FORM_FRUIT_FIRST_LABEL: &str = "Apple";

/// The value of the second fruit option.
pub(crate) const FORM_FRUIT_SECOND_VALUE: &str = "banana";

/// The label of the second fruit option.
pub(crate) const FORM_FRUIT_SECOND_LABEL: &str = "Banana";

/// The value of the third fruit option.
pub(crate) const FORM_FRUIT_THIRD_VALUE: &str = "cherry";

/// The label of the third fruit option.
pub(crate) const FORM_FRUIT_THIRD_LABEL: &str = "Cherry";

/// The label above the checkbox group.
pub(crate) const FORM_TOPPING_LABEL: &str = "Toppings";

/// The value of the first topping option.
pub(crate) const FORM_TOPPING_FIRST_VALUE: &str = "cheese";

/// The label of the first topping option.
pub(crate) const FORM_TOPPING_FIRST_LABEL: &str = "Cheese";

/// The value of the second topping option.
pub(crate) const FORM_TOPPING_SECOND_VALUE: &str = "olives";

/// The label of the second topping option.
pub(crate) const FORM_TOPPING_SECOND_LABEL: &str = "Olives";

/// The value of the third topping option.
pub(crate) const FORM_TOPPING_THIRD_VALUE: &str = "basil";

/// The label of the third topping option.
pub(crate) const FORM_TOPPING_THIRD_LABEL: &str = "Basil";

/// The topping the checkbox group starts with checked.
pub(crate) const FORM_DEFAULT_TOPPING: &str = "cheese";

/// The value of the first segment.
pub(crate) const FORM_VIEW_FIRST_VALUE: &str = "grid";

/// The label of the first segment.
pub(crate) const FORM_VIEW_FIRST_LABEL: &str = "Grid";

/// The value of the second segment.
pub(crate) const FORM_VIEW_SECOND_VALUE: &str = "list";

/// The label of the second segment.
pub(crate) const FORM_VIEW_SECOND_LABEL: &str = "List";

/// The value of the third segment.
pub(crate) const FORM_VIEW_THIRD_VALUE: &str = "detail";

/// The label of the third segment.
pub(crate) const FORM_VIEW_THIRD_LABEL: &str = "Detail";

/// The segment the segmented control starts on.
pub(crate) const FORM_DEFAULT_VIEW: &str = "grid";

/// The section heading for the input card.
pub(crate) const FORM_INPUT_CARD_TITLE: &str = "Inputs";

/// The label above the number input.
pub(crate) const FORM_QUANTITY_LABEL: &str = "Quantity";

/// The quantity the number input starts on.
pub(crate) const FORM_DEFAULT_QUANTITY: i64 = 2;

/// The smallest quantity the number input allows.
pub(crate) const FORM_QUANTITY_MIN: i64 = 0;

/// The largest quantity the number input allows.
pub(crate) const FORM_QUANTITY_MAX: i64 = 10;

/// The step the number input moves by.
pub(crate) const FORM_QUANTITY_STEP: i64 = 1;

/// The label above the auto-complete.
pub(crate) const FORM_SEARCH_LABEL: &str = "Search";

/// The placeholder shown in the auto-complete.
pub(crate) const FORM_SEARCH_PLACEHOLDER: &str = "Type to filter";

/// The value of the first search option.
pub(crate) const FORM_SEARCH_FIRST_VALUE: &str = "signal";

/// The label of the first search option.
pub(crate) const FORM_SEARCH_FIRST_LABEL: &str = "Signal";

/// The value of the second search option.
pub(crate) const FORM_SEARCH_SECOND_VALUE: &str = "signal_state";

/// The label of the second search option.
pub(crate) const FORM_SEARCH_SECOND_LABEL: &str = "SignalState";

/// The value of the third search option.
pub(crate) const FORM_SEARCH_THIRD_VALUE: &str = "hook_context";

/// The label of the third search option.
pub(crate) const FORM_SEARCH_THIRD_LABEL: &str = "HookContext";

/// The label above the date picker.
pub(crate) const FORM_DATE_LABEL: &str = "Delivery date";

/// The placeholder shown when no date is chosen.
pub(crate) const FORM_DATE_PLACEHOLDER: &str = "Pick a date";

/// The label of the form's submit button.
pub(crate) const FORM_SUBMIT_LABEL: &str = "Submit";

/// The message logged when the form is submitted.
pub(crate) const FORM_SUBMIT_LOG: &str = "form submitted";

/// The label of the fruit row in the summary.
pub(crate) const FORM_SUMMARY_FRUIT: &str = "fruit";

/// The label of the topping row in the summary.
pub(crate) const FORM_SUMMARY_TOPPING: &str = "toppings";

/// The label of the view row in the summary.
pub(crate) const FORM_SUMMARY_VIEW: &str = "view";

/// The label of the quantity row in the summary.
pub(crate) const FORM_SUMMARY_QUANTITY: &str = "quantity";

// The lines the demo writes into the console panel when a callback runs.
pub(crate) const FORM_SELECT_LOG: &str = "select changed to {value}";
pub(crate) const FORM_SEGMENTED_LOG: &str = "view changed to {value}";
