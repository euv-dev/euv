/// A single validator closure, used by
/// [`crate::FormState::validate`].
///
/// The closure receives the current value of the field and
/// returns `Some(error_message)` if the value is invalid, or
/// `None` if it is valid. Returning `Some("")` and returning
/// `None` are both treated as "no error" downstream — but
/// using `Some("")` is discouraged because it round-trips
/// through `is_empty()` checks in the UI layer.
///
/// `Validator` is intentionally type-erased (`Box<dyn Fn>`)
/// rather than a generic, because the validators map is
/// typically built up once at component mount and stored in
/// a `HashMap<&'static str, Validator>` — generics over a
/// map would either force a single validator type or balloon
/// the API.
pub type Validator = Box<dyn Fn(&str) -> Option<String>>;
