/// Semantic tone for `EuvBadge` and `EuvAlert`.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EuvTone {
    /// Neutral default tone.
    #[default]
    Neutral,
    /// Brand primary emphasis tone.
    Primary,
    /// Positive success tone.
    Success,
    /// Caution warning tone.
    Warning,
    /// Error / destructive tone.
    Destructive,
}
