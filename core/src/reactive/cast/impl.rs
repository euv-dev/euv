use super::*;

/// Converts a static `String` into a text attribute value.
impl From<String> for AttributeValue {
    /// Converts this string into an `AttributeValue::Text`.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - A text attribute value containing this string.
    ///
    /// # Arguments
    ///
    /// - `String` - Input value to convert from.
    fn from(value: String) -> Self {
        AttributeValue::Text(value)
    }
}

/// Converts a string slice into a text attribute value.
impl From<&str> for AttributeValue {
    /// Converts this string slice into an `AttributeValue::Text`.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - A text attribute value containing the owned string.
    ///
    /// # Arguments
    ///
    /// - `&str` - Input value to convert from.
    fn from(value: &str) -> Self {
        AttributeValue::Text(value.to_string())
    }
}

/// Converts a string signal into a reactive attribute value.
impl From<Signal<String>> for AttributeValue {
    /// Converts this string signal into an `AttributeValue::Signal`.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - A signal-backed attribute value.
    ///
    /// # Arguments
    ///
    /// - `Signal<String>` - Input value to convert from.
    fn from(signal: Signal<String>) -> Self {
        AttributeValue::Signal(signal)
    }
}

/// Converts a mutable bool signal into a reactive attribute value.
///
/// The signal is mapped to a `Signal<String>` that yields `"true"` or `"false"`,
/// enabling boolean attributes like `checked` to reactively update the DOM.
impl From<Signal<bool>> for AttributeValue {
    /// Converts this bool signal into an `AttributeValue` via string mapping.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - A signal-backed attribute value yielding `"true"` or `"false"`.
    ///
    /// # Arguments
    ///
    /// - `Signal<bool>` - Input value to convert from.
    fn from(signal: Signal<bool>) -> Self {
        AttributeValue::bool_to_attr(signal)
    }
}

/// Converts a `bool` into a dynamic attribute value.
///
/// Stored as `AttributeValue::Dynamic("true"/"false")` so components can
/// extract the original boolean via `try_get_typed_prop`.
impl From<bool> for AttributeValue {
    /// Converts this boolean into an `AttributeValue::Dynamic`.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - A dynamic attribute value containing the boolean string.
    ///
    /// # Arguments
    ///
    /// - `bool` - Input value to convert from.
    fn from(value: bool) -> Self {
        AttributeValue::Dynamic(value.to_string())
    }
}

/// Converts an `i32` into a dynamic attribute value.
///
/// Stored as `AttributeValue::Dynamic` so components can extract the
/// original integer via `try_get_typed_prop`.
impl From<i32> for AttributeValue {
    /// Converts this integer into an `AttributeValue::Dynamic`.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - A dynamic attribute value containing the integer string.
    ///
    /// # Arguments
    ///
    /// - `i32` - Input value to convert from.
    fn from(value: i32) -> Self {
        AttributeValue::Dynamic(value.to_string())
    }
}

/// Converts an `f64` into a dynamic attribute value.
///
/// Stored as `AttributeValue::Dynamic` so components can extract the
/// original float via `try_get_typed_prop`.
impl From<f64> for AttributeValue {
    /// Converts this float into an `AttributeValue::Dynamic`.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - A dynamic attribute value containing the float string.
    ///
    /// # Arguments
    ///
    /// - `f64` - Input value to convert from.
    fn from(value: f64) -> Self {
        AttributeValue::Dynamic(value.to_string())
    }
}

/// Converts a CSS class reference into an attribute value.
impl From<Css> for AttributeValue {
    /// Converts this CSS class into an `AttributeValue::Css`.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - A CSS class attribute value.
    ///
    /// # Arguments
    ///
    /// - `Css` - Input value to convert from.
    fn from(css: Css) -> Self {
        AttributeValue::Css(css)
    }
}

/// Converts a reference to a CSS class into an attribute value without cloning.
///
/// OPT 11: previously this impl deep-cloned the `Css` struct (name, style,
/// pseudo_rules, media_rules) every time a static class was referenced from
/// `html!`. The `class!` macro produces `&'static Css` references backed
/// by `OnceLock`, so storing the reference directly is sound and removes
/// the per-element allocation.
impl From<&'static Css> for AttributeValue {
    /// Converts this CSS class reference into an `AttributeValue::CssRef`
    /// without cloning.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - A CSS class attribute value borrowing the
    ///   `class!` macro's static storage.
    ///
    /// # Arguments
    ///
    /// - `&'static Css` - Input value to convert from.
    fn from(css: &'static Css) -> Self {
        AttributeValue::CssRef(css)
    }
}

/// Converts a closure into a callback attribute value.
impl<F> From<F> for AttributeValue
where
    F: FnMut(Event) + 'static,
{
    /// Wraps this closure into an `AttributeValue::Event` with a generic "callback" event name.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - An event attribute value wrapping this closure.
    ///
    /// # Arguments
    ///
    /// - `F` - Input value to convert from.
    fn from(closure: F) -> Self {
        AttributeValue::Event(NativeEventHandler::create(CALLBACK_EVENT_NAME, closure))
    }
}

/// Converts an owned event handler into a callback attribute value.
///
/// Re-wraps the handler with a generic "callback" event name so that
/// subsequent `EventAdapter::into_attribute` calls can override it with
/// the correct DOM event type.
impl From<NativeEventHandler> for AttributeValue {
    /// Converts this event handler into an `AttributeValue::Event`.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - An event attribute value with a generic callback event name.
    ///
    /// # Arguments
    ///
    /// - `NativeEventHandler` - Input value to convert from.
    ///
    /// This is the base case of the `NativeEventHandler` ->
    /// `AttrValueAdapter<NativeEventHandler>` -> `NativeEventHandler`
    /// conversion cycle. Calling `.into()` on the adapter instead would
    /// land back in this impl through the generic
    /// `From<AttrValueAdapter<T>>` (which unwraps and calls `T::into()`),
    /// recursing forever.
    fn from(handler: NativeEventHandler) -> Self {
        AttrValueAdapter::new(handler).into_callback_named(CALLBACK_EVENT_NAME)
    }
}

/// Converts an optional event handler into a callback attribute value.
///
/// Re-wraps a `Some` handler with a generic "callback" event name so that
/// subsequent `EventAdapter::into_attribute` calls can override it with
/// the correct DOM event type.
impl From<Option<NativeEventHandler>> for AttributeValue {
    /// Converts this optional handler into an `AttributeValue::Event` or `AttributeValue::Text` if `None`.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - An event attribute value if `Some`, otherwise an empty text attribute.
    ///
    /// # Arguments
    ///
    /// - `Option<NativeEventHandler>` - Input value to convert from.
    fn from(handler: Option<NativeEventHandler>) -> Self {
        match handler {
            Some(event_handler) => {
                AttrValueAdapter::new(event_handler).into_callback_named(CALLBACK_EVENT_NAME)
            }
            None => AttributeValue::Text(String::new()),
        }
    }
}

/// Converts an optional string signal into a reactive or empty attribute value.
///
/// `Some(signal)` becomes `AttributeValue::Signal`, `None` becomes
/// `AttributeValue::Text(String::new())`. This supports component props
/// that use `Option<Signal<String>>` for optional reactive attributes.
impl From<Option<Signal<String>>> for AttributeValue {
    /// Converts this optional signal into an `AttributeValue::Signal` or `AttributeValue::Text` if `None`.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - A signal-backed attribute value if `Some`, otherwise an empty text attribute.
    ///
    /// # Arguments
    ///
    /// - `Option<Signal<String>>` - Input value to convert from.
    fn from(signal: Option<Signal<String>>) -> Self {
        match signal {
            Some(string_signal) => AttributeValue::Signal(string_signal),
            None => AttributeValue::Text(String::new()),
        }
    }
}
