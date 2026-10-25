use super::*;

/// The independently measured runtime subsystems and their combined frame.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Suite {
    Core,
    Engine,
    Chart,
    Mixed,
}
