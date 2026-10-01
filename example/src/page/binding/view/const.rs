/// The HTML id for the child input text element.
pub(crate) const CHILD_INPUT_TEXT_ID: &str = "child-input-text";

/// The HTML id for the celsius input element.
pub(crate) const TEMPERATURE_CELSIUS_ID: &str = "temperature-celsius";

/// The HTML id for the fahrenheit input element.
pub(crate) const TEMPERATURE_FAHRENHEIT_ID: &str = "temperature-fahrenheit";

/// The HTML id for the red slider element.
pub(crate) const COLOR_MIXER_RED_ID: &str = "color-mixer-red";

/// The HTML id for the green slider element.
pub(crate) const COLOR_MIXER_GREEN_ID: &str = "color-mixer-green";

/// The HTML id for the blue slider element.
pub(crate) const COLOR_MIXER_BLUE_ID: &str = "color-mixer-blue";

/// The HTML id for the parent message input element.
pub(crate) const BINDING_PARENT_MESSAGE_ID: &str = "binding-parent-message";

/// The HTML name attribute for the child input text element.
pub(crate) const CHILD_INPUT_TEXT_NAME: &str = "shared_text";

/// The HTML name attribute for the celsius input element.
pub(crate) const TEMPERATURE_CELSIUS_NAME: &str = "celsius";

/// The HTML name attribute for the fahrenheit input element.
pub(crate) const TEMPERATURE_FAHRENHEIT_NAME: &str = "fahrenheit";

/// The HTML name attribute for the red slider element.
pub(crate) const COLOR_MIXER_RED_NAME: &str = "red";

/// The HTML name attribute for the green slider element.
pub(crate) const COLOR_MIXER_GREEN_NAME: &str = "green";

/// The HTML name attribute for the blue slider element.
pub(crate) const COLOR_MIXER_BLUE_NAME: &str = "blue";

/// The HTML name attribute for the parent message input element.
pub(crate) const BINDING_PARENT_MESSAGE_NAME: &str = "parent_message";

/// The HTML input type for text.
pub(crate) const BINDING_TEXT_TYPE: &str = "text";

/// The HTML input type for number.
pub(crate) const BINDING_NUMBER_TYPE: &str = "number";

/// The HTML input type for range.
pub(crate) const BINDING_RANGE_TYPE: &str = "range";

/// The HTML autocomplete attribute value for off.
pub(crate) const BINDING_AUTOCOMPLETE_OFF: &str = "off";

/// The HTML min attribute for the color mixer slider.
pub(crate) const COLOR_MIXER_MIN: &str = "0";

/// The HTML max attribute for the color mixer slider.
pub(crate) const COLOR_MIXER_MAX: &str = "255";

/// The hint below the color mixer heading.
pub(crate) const BINDING_COLOR_MIXER_HINT: &str =
    "Adjust the RGB sliders — the hex color and preview update reactively via watch!";

/// The heading of the color mixer demo.
pub(crate) const BINDING_COLOR_MIXER_TITLE: &str = "Color Mixer";

/// The hint below the temperature converter heading.
pub(crate) const BINDING_TEMPERATURE_CONVERTER_HINT: &str =
    "Edit either temperature field — the other updates reactively via watch!";

/// The heading of the temperature converter demo.
pub(crate) const BINDING_TEMPERATURE_CONVERTER_TITLE: &str = "Temperature Converter";

/// The description of the cross-component watch demo.
pub(crate) const BINDING_CROSS_CARD_DESC: &str = "Signals are linked across components using the watch! macro. Changing one Signal automatically updates the other through a reactive side effect.";

/// The card title for the cross-component watch demo.
pub(crate) const BINDING_CROSS_CARD_TITLE: &str = "Cross-Component Reactive Binding (watch!)";

/// The prefix shown before the typed props count value.
pub(crate) const BINDING_COUNT_PREFIX: &str = "Count: ";

/// The prefix shown before the shared text value.
pub(crate) const BINDING_SHARED_TEXT_PREFIX: &str = "Text: ";

/// The heading of the parent component box.
pub(crate) const BINDING_PARENT_COMPONENT_TITLE: &str = "Parent Component";

/// The description of the two-way shared signal demo.
pub(crate) const BINDING_TWO_WAY_CARD_DESC: &str = "Both parent and child components share the same Signal instances. Any mutation in either component is immediately reflected in the other — no callbacks or event listeners needed.";

/// The card title for the two-way shared signal demo.
pub(crate) const BINDING_TWO_WAY_CARD_TITLE: &str = "Two-Way Binding (Shared Signal)";

/// The prefix shown before the typed props max count value.
pub(crate) const BINDING_MAX_PREFIX: &str = "Max: ";

/// The label of the typed props toggle button.
pub(crate) const BINDING_TOGGLE_BUTTON_LABEL: &str = "Toggle";

/// The heading of the typed props controls box.
pub(crate) const BINDING_TYPED_PROPS_TITLE: &str = "Typed Props Controls";

/// The prefix shown before the echoed parent message.
pub(crate) const BINDING_PARENT_MESSAGE_ECHO_PREFIX: &str = "Message: ";

/// The label of the parent message input.
pub(crate) const BINDING_PARENT_MESSAGE_INPUT_LABEL: &str = "Parent message: ";

/// The description of the props and callbacks demo.
pub(crate) const BINDING_PROPS_CARD_DESC: &str = "The parent component passes a string message to the child via props. The child communicates back to the parent through callback functions triggered on user interaction.";

/// The card title for the props and callbacks demo.
pub(crate) const BINDING_PROPS_CARDBACK_CARD_TITLE: &str = "Props & Callbacks";

/// The page header subtitle of the component binding demo.
pub(crate) const BINDING_PAGE_SUBTITLE: &str = "Props passing with callbacks, two-way binding via shared Signals, and cross-component reactive binding using watch!.";

/// The page header title of the component binding demo.
pub(crate) const BINDING_PAGE_TITLE: &str = "Component Binding";

/// The label of the fahrenheit temperature field.
pub(crate) const BINDING_FAHRENHEIT_LABEL: &str = "Fahrenheit";

/// The label of the celsius temperature field.
pub(crate) const BINDING_CELSIUS_LABEL: &str = "Celsius";

/// The prefix shown before the shared count value.
pub(crate) const BINDING_SHARED_COUNT_PREFIX: &str = "Shared count: ";

/// The label of the shared text input.
pub(crate) const BINDING_SHARED_TEXT_INPUT_LABEL: &str = "Edit shared text:";

/// The heading of the shared-signal child component box.
pub(crate) const BINDING_CHILD_COMPONENT_TITLE: &str = "Child Component";

/// The warning text shown when the limited counter is disabled.
pub(crate) const BINDING_DISABLED_WARNING: &str = "Counter is disabled!";

/// The label of the limited counter reset button.
pub(crate) const BINDING_RESET_BUTTON_LABEL: &str = "Reset";

/// The props readout infix joining the disabled and max_count values.
pub(crate) const BINDING_PROPS_MAX_COUNT_INFIX: &str = ", max_count=";

/// The props readout prefix for the disabled flag.
pub(crate) const BINDING_PROPS_DISABLED_PREFIX: &str = "Props received: disabled=";

/// The heading of the limited counter card.
pub(crate) const BINDING_LIMITED_COUNTER_TITLE: &str = "Limited Counter";
