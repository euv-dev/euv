/// The alert variant determining the visual style.
///
/// The registry publishes two variants — the quiet default and the
/// destructive one — and the design system keeps its own success and
/// error treatments beside them.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum AlertVariant {
    /// A success alert with green styling.
    #[default]
    Success,
    /// An error alert with red styling.
    Error,
    /// The registry's quiet alert on the background surface.
    Default,
    /// The registry's destructive alert.
    Destructive,
}
