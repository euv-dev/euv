/// Typographic scale variants for `EuvTypography`.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum EuvTypographyVariant {
    /// Large display title.
    H1,
    /// Section heading.
    H2,
    /// Subsection heading.
    H3,
    /// Standard body copy.
    #[default]
    Body,
    /// Secondary muted caption.
    Caption,
}
