use super::*;

/// The border treatment of the `euv_panel` component.
///
/// The monochrome design system expresses separation with borders rather than
/// shadows or background fills, so the variant only chooses the border style.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EuvPanelVariant {
    /// No border, for panels already separated by surrounding whitespace.
    #[default]
    Plain,
    /// A solid 1px border, for a strongly grouped section.
    Bordered,
    /// A dashed 1px border, for a loosely grouped section.
    Dashed,
}

/// Props for the `euv_panel` component.
///
/// Defines the strongly-typed interface for the bordered section container.
/// A panel is a superset of [`EuvCardProps`]: it adds a subtitle and a border
/// variant. Children are the panel body, so a footer is passed as trailing
/// children rather than as a prop.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvPanelProps {
    /// The panel title rendered in the header.
    pub title: &'static str,
    /// The optional subtitle rendered under the title. Skipped when empty.
    pub subtitle: &'static str,
    /// The border treatment of the panel.
    pub variant: EuvPanelVariant,
}
