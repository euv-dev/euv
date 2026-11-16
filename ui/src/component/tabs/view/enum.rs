/// Determines whether keyboard navigation also selects a tab.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EuvTabsActivationMode {
    /// Arrow and edge keys focus and select their destination.
    #[default]
    Automatic,
    /// Navigation moves focus only; Enter, Space or a click selects.
    Manual,
}
