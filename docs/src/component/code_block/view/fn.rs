use super::*;

/// Renders one code block: a toolbar carrying the language and a copy
/// button, and the highlighted code below it.
///
/// The copy button is a real clipboard write — `navigator.clipboard()`,
/// through euv-ui's clipboard hook — and the button reports the outcome
/// of that write: it reads "Copy" until the browser has accepted the
/// text and "Copied" for the two seconds that follow.
///
/// The button carries one label per outcome and swaps them by class.
/// The swap is made on the button's own element, and never travels
/// through a signal: the page re-renders whenever a signal it read
/// changes, and the re-render rewrites the labels' classes back to
/// their resting values, so a signal-borne confirmation is erased
/// before the reader can see it. The button therefore owns its own
/// outcome, and the page owns nothing here.
///
/// # Arguments
///
/// - `VirtualNode<DocsCodeBlockProps>` - The props node carrying the
///   fence language, the raw code and the page's copy state.
///
/// # Returns
///
/// - `VirtualNode` - The code block virtual DOM tree.
#[component]
pub(crate) fn docs_code_block(node: VirtualNode<DocsCodeBlockProps>) -> VirtualNode {
    let DocsCodeBlockProps { lang, code }: DocsCodeBlockProps =
        node.try_get_props().unwrap_or_default();
    let clipboard: UseClipboard = use_clipboard();
    let copy: Option<Rc<dyn Fn(Event)>> = Some(Rc::new(move |event: Event| {
        // The write is asynchronous, so the browser answers later. What
        // the button can report straight away is whether the write was
        // taken at all: a host without a clipboard cannot take one, and
        // that is worth saying out loud.
        let started: bool = clipboard.copy(code);
        if let Some(button) = event_button(&event) {
            set_copy_label(&button, if started { COPY_STATE } else { FAILED_STATE });
            let button: Element = button.clone();
            schedule_after(COPY_FEEDBACK_MILLIS, move || {
                set_copy_label(&button, RESTING_STATE);
            });
        }
    }));
    let html: String = highlighted_html(lang, code);
    html! {
        figure {
            class: c_docs_code()
            div {
                class: c_docs_code_header()
                span {
                    class: c_docs_code_lang()
                    {
                        lang
                    }
                }
                button {
                    class: c_docs_code_copy()
                    aria-label: COPY_BUTTON_LABEL
                    onclick: copy
                    span {
                        class: c_docs_code_label()
                        {
                            COPY_LABEL
                        }
                    }
                    span {
                        class: c_docs_code_label_hidden()
                        {
                            COPIED_LABEL
                        }
                    }
                    span {
                        class: c_docs_code_label_hidden()
                        {
                            FAILED_LABEL
                        }
                    }
                }
            }
            pre {
                code {
                    inner_html: html
                }
            }
        }
    }
}

/// Returns the button the press landed on.
///
/// The renderer delegates every bubbling event from the window, so the
/// press arrives with the window as its current target. The button is
/// found from the element the press actually landed on, walking up to
/// the button that owns it.
///
/// # Arguments
///
/// - `&Event` - The event of the press.
///
/// # Returns
///
/// - `Option<Element>` - The button, when the press landed on one.
fn event_button(event: &Event) -> Option<Element> {
    let target: EventTarget = event.target()?;
    let target: Element = target.dyn_into::<Element>().ok()?;
    target.closest(COPY_BUTTON_SELECTOR).ok().flatten()
}

/// Shows one of the button's labels, and hides the others.
///
/// The button carries one label per outcome, in the order they are
/// written: the resting one, the copied one and the failed one. The
/// class of each label decides whether it is on screen, so the outcome
/// is a choice of which label stays.
///
/// # Arguments
///
/// - `&Element` - The copy button.
/// - `u8` - The state of the button.
fn set_copy_label(button: &Element, state: u8) {
    let Ok(labels) = button.query_selector_all(LABEL_SELECTOR) else {
        return;
    };
    let count: u32 = labels.length();
    for index in 0..count {
        let Some(label) = labels.item(index) else {
            continue;
        };
        let Ok(label) = label.dyn_into::<Element>() else {
            continue;
        };
        let class: &str = if index as u8 == state {
            COPY_LABEL_CLASS
        } else {
            HIDDEN_LABEL_CLASS
        };
        let _ = label.set_attribute("class", class);
    }
}

/// Schedules a callback to run `delay_ms` milliseconds from now.
///
/// The callback runs as a macrotask, so it is always queued behind the
/// pending microtasks the renderer uses to commit updates. On a host
/// without a browser the callback is dropped.
///
/// # Arguments
///
/// - `u32` - The delay in milliseconds.
/// - `F` - The callback to run.
fn schedule_after<F>(delay_ms: u32, callback: F)
where
    F: FnOnce() + 'static,
{
    #[cfg(target_arch = "wasm32")]
    {
        let Some(window) = window() else {
            return;
        };
        let function: JsValue = Closure::once_into_js(move || {
            callback();
        });
        let Ok(function) = function.dyn_into::<Function>() else {
            return;
        };
        let _: Result<i32, JsValue> = window.set_timeout_with_callback_and_timeout_and_arguments_0(
            function.as_ref(),
            delay_ms as i32,
        );
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _: u32 = delay_ms;
        let _: F = callback;
    }
}
