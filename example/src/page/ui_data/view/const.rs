/// The title text for the header on the data demo page.
pub(crate) const DATA_HEADER_TITLE: &str = "Data & Feedback";

/// The subtitle text for the header on the data demo page.
pub(crate) const DATA_HEADER_SUBTITLE: &str = "Lists, description grids, trees, toasts, notifications and confirmation bubbles — the data-reading and feedback half of the component library.";

/// The section heading for the list card.
pub(crate) const DATA_LIST_CARD_TITLE: &str = "List";

/// The hint text explaining the list.
pub(crate) const DATA_LIST_HINT: &str =
    "A row carries a title, an optional description and an optional trailing note.";

/// The title of the first list row.
pub(crate) const DATA_LIST_FIRST_TITLE: &str = "Renderer";

/// The description of the first list row.
pub(crate) const DATA_LIST_FIRST_DESC: &str = "Turns a virtual node tree into DOM.";

/// The trailing note of the first list row.
pub(crate) const DATA_LIST_FIRST_META: &str = "core";

/// The title of the second list row.
pub(crate) const DATA_LIST_SECOND_TITLE: &str = "Scheduler";

/// The description of the second list row.
pub(crate) const DATA_LIST_SECOND_DESC: &str = "Flushes the dirty nodes on a microtask.";

/// The trailing note of the second list row.
pub(crate) const DATA_LIST_SECOND_META: &str = "core";

/// The title of the third list row.
pub(crate) const DATA_LIST_THIRD_TITLE: &str = "Registry";

/// The description of the third list row.
pub(crate) const DATA_LIST_THIRD_DESC: &str = "Keeps the delegated event handlers.";

/// The trailing note of the third list row.
pub(crate) const DATA_LIST_THIRD_META: &str = "core";

/// The section heading for the descriptions card.
pub(crate) const DATA_DESCRIPTIONS_CARD_TITLE: &str = "Descriptions";

/// The hint text explaining the descriptions grid.
pub(crate) const DATA_DESCRIPTIONS_HINT: &str =
    "A grid of label and value pairs. The column count is a custom property on the grid.";

/// The label of the first description pair.
pub(crate) const DATA_DESCRIPTION_FIRST_LABEL: &str = "Crate";

/// The value of the first description pair.
pub(crate) const DATA_DESCRIPTION_FIRST_VALUE: &str = "euv-ui";

/// The label of the second description pair.
pub(crate) const DATA_DESCRIPTION_SECOND_LABEL: &str = "Version";

/// The value of the second description pair.
pub(crate) const DATA_DESCRIPTION_SECOND_VALUE: &str = "0.32.0";

/// The label of the third description pair.
pub(crate) const DATA_DESCRIPTION_THIRD_LABEL: &str = "Target";

/// The value of the third description pair.
pub(crate) const DATA_DESCRIPTION_THIRD_VALUE: &str = "wasm32-unknown-unknown";

/// The label of the fourth description pair.
pub(crate) const DATA_DESCRIPTION_FOURTH_LABEL: &str = "Theme";

/// The value of the fourth description pair.
pub(crate) const DATA_DESCRIPTION_FOURTH_VALUE: &str = "Monochrome";

/// The section heading for the tree card.
pub(crate) const DATA_TREE_CARD_TITLE: &str = "Tree";

/// The hint text explaining the tree.
pub(crate) const DATA_TREE_HINT: &str = "A hierarchy whose branches expand in place. The children stay mounted, so a branch opens with a class swap.";

/// The label of the first tree root.
pub(crate) const DATA_TREE_ROOT_LABEL: &str = "core";

/// The value of the first tree root.
pub(crate) const DATA_TREE_ROOT_VALUE: &str = "core";

/// The label of the first tree leaf.
pub(crate) const DATA_TREE_LEAF_FIRST_LABEL: &str = "signal";

/// The value of the first tree leaf.
pub(crate) const DATA_TREE_LEAF_FIRST_VALUE: &str = "signal";

/// The label of the second tree leaf.
pub(crate) const DATA_TREE_LEAF_SECOND_LABEL: &str = "scheduler";

/// The value of the second tree leaf.
pub(crate) const DATA_TREE_LEAF_SECOND_VALUE: &str = "scheduler";

/// The label of the second tree root.
pub(crate) const DATA_TREE_ROOT_SECOND_LABEL: &str = "ui";

/// The value of the second tree root.
pub(crate) const DATA_TREE_ROOT_SECOND_VALUE: &str = "ui";

/// The label of the third tree leaf.
pub(crate) const DATA_TREE_LEAF_THIRD_LABEL: &str = "component";

/// The value of the third tree leaf.
pub(crate) const DATA_TREE_LEAF_THIRD_VALUE: &str = "component";

/// The section heading for the feedback card.
pub(crate) const DATA_FEEDBACK_CARD_TITLE: &str = "Feedback";

/// The hint text explaining the feedback controls.
pub(crate) const DATA_FEEDBACK_HINT: &str = "A toast floats at the top of the viewport, a notification at the top right, and the confirmation bubble hangs off its trigger.";

/// The label of the button that pushes a toast.
pub(crate) const DATA_MESSAGE_PUSH_LABEL: &str = "Show a toast";

/// The label of the button that dismisses the last toast.
pub(crate) const DATA_MESSAGE_POP_LABEL: &str = "Dismiss the toast";

/// The text of the pushed toast.
pub(crate) const DATA_MESSAGE_TEXT: &str = "The record was saved.";

/// The action label of the pushed toast.
pub(crate) const DATA_MESSAGE_ACTION: &str = "Undo";

/// The label of the button that pushes a notification.
pub(crate) const DATA_NOTIFICATION_PUSH_LABEL: &str = "Show a notification";

/// The label of the button that dismisses the last notification.
pub(crate) const DATA_NOTIFICATION_POP_LABEL: &str = "Dismiss the notification";

/// The title of the pushed notification.
pub(crate) const DATA_NOTIFICATION_TITLE: &str = "Deployment";

/// The text of the pushed notification.
pub(crate) const DATA_NOTIFICATION_TEXT: &str = "The new build is live.";

/// The label of the confirmation bubble's trigger.
pub(crate) const DATA_POPCONFIRM_TRIGGER_LABEL: &str = "Delete the record";

/// The question the confirmation bubble asks.
pub(crate) const DATA_POPCONFIRM_TITLE: &str = "Delete this record? It cannot be undone.";

/// The label of the confirmation bubble's confirm button.
pub(crate) const DATA_POPCONFIRM_OK_LABEL: &str = "Delete";

/// The label of the confirmation bubble's cancel button.
pub(crate) const DATA_POPCONFIRM_CANCEL_LABEL: &str = "Keep it";

// The lines the demo writes into the console panel when a callback runs.
pub(crate) const DATA_POPCONFIRM_OK_LOG: &str = "popconfirm confirmed";
pub(crate) const DATA_POPCONFIRM_CANCEL_LOG: &str = "popconfirm cancelled";
pub(crate) const DATA_MESSAGE_ACTION_LOG: &str = "message action asked for";
pub(crate) const DATA_MESSAGE_CLOSE_LOG: &str = "message closed";
pub(crate) const DATA_NOTIFICATION_CLOSE_LOG: &str = "notification closed";
