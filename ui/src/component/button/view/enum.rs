use super::*;

/// The visual variant of the `euv_button` component.
///
/// The variants are the ones the registry publishes: a filled default, a
/// quiet secondary, a destructive fill, an outlined box, a borderless
/// ghost and a text link.
///
/// `Primary` is the historical name of the filled default and is kept
/// because callers across the workspace name it; `Default` is the
/// registry's own name for the same treatment.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EuvButtonVariant {
    /// The filled default button.
    #[default]
    Primary,
    /// The registry's name for the filled default button.
    Default,
    /// A quiet button on the secondary surface.
    Secondary,
    /// A button that destroys what it acts on.
    Destructive,
    /// An outlined button on the background surface.
    Outline,
    /// A borderless button that fills on hover.
    Ghost,
    /// A text button that reads as a link.
    Link,
}

/// The size of the `euv_button` component.
///
/// The four sizes are the ones the registry publishes: the default box, a
/// smaller one, a larger one and a square icon box.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EuvButtonSize {
    /// The default control size.
    #[default]
    Default,
    /// The small control size.
    Small,
    /// The large control size.
    Large,
    /// The square icon size.
    Icon,
}
