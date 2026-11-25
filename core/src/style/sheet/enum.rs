/// Color appearance mode synchronized across OS and application themes.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ThemeMode {
    /// Light surface palette.
    #[default]
    Light,
    /// Dark surface palette.
    Dark,
}
