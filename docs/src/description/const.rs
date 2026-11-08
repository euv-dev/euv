/// The reference descriptions of the documentation pages, keyed by the
/// page slug.
///
/// The table is a verbatim copy of the reference site's own page
/// descriptions: every entry is the sentence the reference prints under
/// the page title, and nothing is invented for a page that is not in
/// the table. A page whose slug has no entry simply renders no
/// description line.
pub(crate) const REF_DESCRIPTIONS: &[(&str, &str)] = &[
    (
        "accordion",
        "A vertically stacked set of interactive headings that each reveal a section of content.",
    ),
    ("alert", "Displays a callout for user attention."),
    (
        "alert-dialog",
        "A modal dialog that interrupts the user with important content and expects a response.",
    ),
    ("aspect-ratio", "Displays content within a desired ratio."),
    (
        "attachment",
        "Displays a file or image attachment with media, metadata, upload state, and actions.",
    ),
    (
        "avatar",
        "An image element with a fallback for representing the user.",
    ),
    (
        "badge",
        "Displays a badge or a component that looks like a badge.",
    ),
    (
        "breadcrumb",
        "Displays the path to the current resource using a hierarchy of links.",
    ),
    (
        "bubble",
        "Displays conversational content in a message bubble. Supports variants, alignment, grouping, reactions, and collapsible content.",
    ),
    (
        "button",
        "Displays a button or a component that looks like a button.",
    ),
    (
        "button-group",
        "A container that groups related buttons together with consistent styling.",
    ),
    (
        "calendar",
        "A calendar component that allows users to select a date or a range of dates.",
    ),
    ("card", "Displays a card with header, content, and footer."),
    (
        "carousel",
        "A carousel with motion and swipe built using Embla.",
    ),
    (
        "chart",
        "Beautiful charts. Built using Recharts. Copy and paste into your apps.",
    ),
    (
        "checkbox",
        "A control that allows the user to toggle between checked and not checked.",
    ),
    (
        "code-block",
        "Displays code, terminal output and diffs with an optional header, line numbers and a copy button.",
    ),
    (
        "collapsible",
        "An interactive component which expands/collapses a panel.",
    ),
    ("combobox", "Autocomplete input with a list of suggestions."),
    ("command", "Command menu for search and quick actions."),
    (
        "context-menu",
        "Displays a menu of actions triggered by a right click.",
    ),
    (
        "data-table",
        "Powerful table and datagrids built using TanStack Table.",
    ),
    (
        "date-picker",
        "A date picker component with range and presets.",
    ),
    (
        "dialog",
        "A window overlaid on either the primary window or another dialog window, rendering the content underneath inert.",
    ),
    (
        "direction",
        "A provider component that sets the text direction for your application.",
    ),
    ("drawer", "A drawer component for React."),
    (
        "dropdown-menu",
        "Displays a menu to the user — such as a set of actions or functions — triggered by a button.",
    ),
    (
        "empty",
        "Use the Empty component to display an empty state.",
    ),
    (
        "field",
        "Combine labels, controls, and help text to compose accessible form fields and grouped inputs.",
    ),
    (
        "hover-card",
        "For sighted users to preview content available behind a link.",
    ),
    (
        "input",
        "A text input component for forms and user data entry with built-in styling and accessibility features.",
    ),
    (
        "input-group",
        "Add addons, buttons, and helper content to inputs.",
    ),
    (
        "input-otp",
        "Accessible one-time password component with copy-paste functionality.",
    ),
    (
        "item",
        "A versatile component for displaying content with media, title, description, and actions.",
    ),
    ("kbd", "Used to display textual user input from keyboard."),
    (
        "label",
        "Renders an accessible label associated with controls.",
    ),
    (
        "marker",
        "Displays an inline status, system note, bordered row, or labeled separator in a conversation.",
    ),
    (
        "menubar",
        "A visually persistent menu common in desktop applications that provides quick access to a consistent set of commands.",
    ),
    (
        "message",
        "Displays a message in a conversation, with optional avatar, header, footer, and alignment.",
    ),
    (
        "message-scroller",
        "A chat scroll container that anchors turns, opens saved transcripts, follows streamed responses, loads history without jumping, and jumps to any message.",
    ),
    (
        "native-select",
        "A styled native HTML select element with consistent design system integration.",
    ),
    (
        "navigation-menu",
        "A collection of links for navigating websites.",
    ),
    (
        "pagination",
        "Pagination with page navigation, next and previous links.",
    ),
    (
        "popover",
        "Displays rich content in a portal, triggered by a button.",
    ),
    (
        "progress",
        "Displays an indicator showing the completion progress of a task, typically displayed as a progress bar.",
    ),
    (
        "questionnaire",
        "A multi-step questionnaire with single-choice, multiple-choice, freeform, and skippable questions.",
    ),
    (
        "radio-group",
        "A set of checkable buttons—known as radio buttons—where no more than one of the buttons can be checked at a time.",
    ),
    (
        "resizable",
        "Accessible resizable panel groups and layouts with keyboard support.",
    ),
    (
        "scroll-area",
        "Augments native scroll functionality for custom, cross-browser styling.",
    ),
    (
        "select",
        "Displays a list of options for the user to pick from—triggered by a button.",
    ),
    ("separator", "Visually or semantically separates content."),
    (
        "sheet",
        "Extends the Dialog component to display content that complements the main content of the screen.",
    ),
    (
        "sidebar",
        "A composable, themeable and customizable sidebar component.",
    ),
    (
        "skeleton",
        "Use to show a placeholder while content is loading.",
    ),
    (
        "slider",
        "An input where the user selects a value from within a given range.",
    ),
    (
        "spinner",
        "An indicator that can be used to show a loading state.",
    ),
    (
        "switch",
        "A control that allows the user to toggle between checked and not checked.",
    ),
    ("table", "A responsive table component."),
    (
        "tabs",
        "A set of layered sections of content—known as tab panels—that are displayed one at a time.",
    ),
    (
        "textarea",
        "Displays a form textarea or a component that looks like a textarea.",
    ),
    ("toast", "A succinct message that is displayed temporarily."),
    ("toggle", "A two-state button that can be either on or off."),
    (
        "toggle-group",
        "A set of two-state buttons that can be toggled on or off.",
    ),
    (
        "tooltip",
        "A popup that displays information related to an element when the element receives keyboard focus or the mouse hovers over it.",
    ),
    (
        "trace",
        "The building block for reasoning, tool calls, sources and more.",
    ),
    ("typography", "Styles for headings, paragraphs, lists, etc."),
];
