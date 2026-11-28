use super::*;

thread_local! {
    /// Reusable no-op rejection callback attached to browser Fullscreen API Promises so denied gestures never emit unhandled rejections.
    pub(crate) static WEB_PROMISE_NOOP_REJECT: Closure<dyn FnMut(JsValue)> =
        Closure::wrap(Box::new(|_: JsValue| {}) as Box<dyn FnMut(JsValue)>);
}
