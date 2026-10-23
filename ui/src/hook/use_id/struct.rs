use super::*;

/// A stable, unique identifier for one mounted component.
///
/// The identifier is generated once, on the first render, and never
/// changes afterwards: every re-render at the same hook index returns
/// the same [`UseId`], and the value it carries is the same string.
///
/// The value is shaped `<prefix>-<counter>`, where `counter` is a
/// process-wide monotonically increasing number. The prefix lets a
/// caller keep the identifiers of different kinds apart in the DOM
/// (`field-1`, `field-2`, `panel-1`), which is what makes them
/// readable in devtools and in accessibility audits.
///
/// # Arguments
///
/// - `&'static str` - The prefix put in front of the counter.
#[derive(Clone, Data, Debug, New)]
pub struct UseId {
    /// The generated identifier.
    ///
    /// Defaults to `Signal::create(String::default())` via
    /// `#[new(skip)]`; the hook fills it with the generated value
    /// immediately afterwards.
    #[new(skip)]
    #[get(type(copy))]
    pub value: Signal<String>,
}
