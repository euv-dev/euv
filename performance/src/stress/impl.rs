use super::*;

impl StressMatrixReport {
    /// Returns the optional interleaved A/B partial-layout damage summary if that scenario ran.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Stress-matrix report instance.
    ///
    /// # Returns
    ///
    /// - `Option<&PartialDamageAbSummary>` - Borrowed A/B comparison summary when present.
    #[must_use]
    pub fn partial_damage_ab(&self) -> Option<&PartialDamageAbSummary> {
        let Self {
            partial_damage_ab, ..
        } = self;
        partial_damage_ab.as_ref()
    }
}
