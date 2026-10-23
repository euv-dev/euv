/// The reasons a map cannot be read.
///
/// The errors say what is wrong with the *data*, because the data is
/// the one thing the developer can fix: a map that is not drawn is
/// never a rendering problem.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum GeoError {
    /// The document has no `type` field.
    #[default]
    MissingType,
    /// The document is of a kind this crate does not read.
    UnsupportedType(String),
    /// A feature has no geometry.
    MissingGeometry,
    /// The geometry cannot be read.
    MalformedGeometry(String),
}
