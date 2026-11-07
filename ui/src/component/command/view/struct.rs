use super::*;

/// Props of the [`euv_command`] component.
///
/// The command holds the query the reader typed, so a caller can filter
/// the rows it puts inside the command.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvCommandProps {
    /// The query the reader typed.
    pub value: Signal<String>,
    /// The accessible name of the command.
    pub label: &'static str,
}

/// Props of the [`euv_command_input`] component.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvCommandInputProps {
    /// The query the reader typed.
    pub value: Signal<String>,
    /// The text shown while the field is empty.
    pub placeholder: &'static str,
    /// Whether the field is disabled.
    pub disabled: bool,
}

/// Props of the [`euv_command_list`] component.
#[derive(Clone, Copy, CustomDebug, Default)]
pub struct EuvCommandListProps {
    /// The accessible name of the list.
    pub label: &'static str,
}

/// Props of the [`euv_command_empty`] component.
#[derive(Clone, Copy, CustomDebug, Default)]
pub struct EuvCommandEmptyProps {
    /// The text shown when nothing matched the query.
    pub text: &'static str,
}

/// Props of the [`euv_command_group`] component.
#[derive(Clone, Copy, CustomDebug, Default)]
pub struct EuvCommandGroupProps {
    /// The heading that names the group.
    pub heading: &'static str,
}

/// Props of the [`euv_command_item`] component.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvCommandItemProps {
    /// The value the row stands for.
    pub value: &'static str,
    /// The label shown inside the row.
    pub label: &'static str,
    /// Whether the row is the selected one.
    pub selected: bool,
    /// Whether the row is disabled.
    pub disabled: bool,
    /// Optional callback receiving the chosen value.
    #[debug(skip)]
    pub on_select: Option<Rc<dyn Fn(&'static str)>>,
}

/// Props of the [`euv_command_separator`] component.
#[derive(Clone, Copy, CustomDebug, Default)]
pub struct EuvCommandSeparatorProps {
    /// The label reported to assistive technology, if any.
    pub label: &'static str,
}

/// Props of the [`euv_command_shortcut`] component.
#[derive(Clone, Copy, CustomDebug, Default)]
pub struct EuvCommandShortcutProps {
    /// The key hint text.
    pub text: &'static str,
}
